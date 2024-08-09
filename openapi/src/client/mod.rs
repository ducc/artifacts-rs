use async_trait::async_trait;
use futures::{
    future, future::BoxFuture, future::FutureExt, future::TryFutureExt, stream, stream::StreamExt,
    Stream,
};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use hyper::{service::Service, Body, Request, Response, Uri};
use percent_encoding::{utf8_percent_encode, AsciiSet};
use std::borrow::Cow;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::future::Future;
use std::io::{ErrorKind, Read};
use std::marker::PhantomData;
use std::path::Path;
use std::str;
use std::str::FromStr;
use std::string::ToString;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{ApiError, AuthData, BodyExt, Connector, DropContextService, Has, XSpanIdString};
use url::form_urlencoded;

use crate::header;
use crate::models;

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
#[allow(dead_code)]
const FRAGMENT_ENCODE_SET: &AsciiSet = &percent_encoding::CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`');

/// This encode set is used for object IDs
///
/// Aside from the special characters defined in the `PATH_SEGMENT_ENCODE_SET`,
/// the vertical bar (|) is encoded.
#[allow(dead_code)]
const ID_ENCODE_SET: &AsciiSet = &FRAGMENT_ENCODE_SET.add(b'|');

use crate::{
    ActionAcceptNewTaskMyNameActionTaskNewPostResponse,
    ActionCompleteTaskMyNameActionTaskCompletePostResponse,
    ActionCraftingMyNameActionCraftingPostResponse, ActionDeleteItemMyNameActionDeletePostResponse,
    ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse,
    ActionDepositBankMyNameActionBankDepositPostResponse,
    ActionEquipItemMyNameActionEquipPostResponse, ActionFightMyNameActionFightPostResponse,
    ActionGatheringMyNameActionGatheringPostResponse, ActionGeBuyItemMyNameActionGeBuyPostResponse,
    ActionGeSellItemMyNameActionGeSellPostResponse, ActionMoveMyNameActionMovePostResponse,
    ActionRecyclingMyNameActionRecyclingPostResponse,
    ActionTaskExchangeMyNameActionTaskExchangePostResponse,
    ActionUnequipItemMyNameActionUnequipPostResponse,
    ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse,
    ActionWithdrawBankMyNameActionBankWithdrawPostResponse, Api,
    ChangePasswordMyChangePasswordPostResponse, CreateAccountAccountsCreatePostResponse,
    CreateCharacterCharactersCreatePostResponse, DeleteCharacterCharactersDeletePostResponse,
    GenerateTokenTokenPostResponse, GetAllCharactersCharactersGetResponse,
    GetAllCharactersLogsMyLogsGetResponse, GetAllEventsEventsGetResponse,
    GetAllGeItemsGeGetResponse, GetAllItemsItemsGetResponse, GetAllMapsMapsGetResponse,
    GetAllMonstersMonstersGetResponse, GetAllResourcesResourcesGetResponse,
    GetBankGoldsMyBankGoldGetResponse, GetBankItemsMyBankItemsGetResponse,
    GetCharacterCharactersNameGetResponse, GetGeItemGeCodeGetResponse, GetItemItemsCodeGetResponse,
    GetMapMapsXyGetResponse, GetMonsterMonstersCodeGetResponse,
    GetMyCharactersMyCharactersGetResponse, GetResourceResourcesCodeGetResponse,
    GetStatusGetResponse,
};

/// Convert input into a base path, e.g. "http://example:123". Also checks the scheme as it goes.
fn into_base_path(
    input: impl TryInto<Uri, Error = hyper::http::uri::InvalidUri>,
    correct_scheme: Option<&'static str>,
) -> Result<String, ClientInitError> {
    // First convert to Uri, since a base path is a subset of Uri.
    let uri = input.try_into()?;

    let scheme = uri.scheme_str().ok_or(ClientInitError::InvalidScheme)?;

    // Check the scheme if necessary
    if let Some(correct_scheme) = correct_scheme {
        if scheme != correct_scheme {
            return Err(ClientInitError::InvalidScheme);
        }
    }

    let host = uri.host().ok_or(ClientInitError::MissingHost)?;
    let port = uri
        .port_u16()
        .map(|x| format!(":{}", x))
        .unwrap_or_default();
    Ok(format!(
        "{}://{}{}{}",
        scheme,
        host,
        port,
        uri.path().trim_end_matches('/')
    ))
}

/// A client that implements the API by making HTTP calls out to a server.
pub struct Client<S, C>
where
    S: Service<(Request<Body>, C), Response = Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static,
{
    /// Inner service
    client_service: S,

    /// Base path of the API
    base_path: String,

    /// Marker
    marker: PhantomData<fn(C)>,
}

impl<S, C> fmt::Debug for Client<S, C>
where
    S: Service<(Request<Body>, C), Response = Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Client {{ base_path: {} }}", self.base_path)
    }
}

impl<S, C> Clone for Client<S, C>
where
    S: Service<(Request<Body>, C), Response = Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            client_service: self.client_service.clone(),
            base_path: self.base_path.clone(),
            marker: PhantomData,
        }
    }
}

impl<Connector, C> Client<DropContextService<hyper::client::Client<Connector, Body>, C>, C>
where
    Connector: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Create a client with a custom implementation of hyper::client::Connect.
    ///
    /// Intended for use with custom implementations of connect for e.g. protocol logging
    /// or similar functionality which requires wrapping the transport layer. When wrapping a TCP connection,
    /// this function should be used in conjunction with `swagger::Connector::builder()`.
    ///
    /// For ordinary tcp connections, prefer the use of `try_new_http`, `try_new_https`
    /// and `try_new_https_mutual`, to avoid introducing a dependency on the underlying transport layer.
    ///
    /// # Arguments
    ///
    /// * `base_path` - base path of the client API, i.e. "http://www.my-api-implementation.com"
    /// * `protocol` - Which protocol to use when constructing the request url, e.g. `Some("http")`
    /// * `connector` - Implementation of `hyper::client::Connect` to use for the client
    pub fn try_new_with_connector(
        base_path: &str,
        protocol: Option<&'static str>,
        connector: Connector,
    ) -> Result<Self, ClientInitError> {
        let client_service = hyper::client::Client::builder().build(connector);
        let client_service = DropContextService::new(client_service);

        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, protocol)?,
            marker: PhantomData,
        })
    }
}

#[derive(Debug, Clone)]
pub enum HyperClient {
    Http(hyper::client::Client<hyper::client::HttpConnector, Body>),
    Https(hyper::client::Client<HttpsConnector, Body>),
}

impl Service<Request<Body>> for HyperClient {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = hyper::client::ResponseFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        match self {
            HyperClient::Http(client) => client.poll_ready(cx),
            HyperClient::Https(client) => client.poll_ready(cx),
        }
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        match self {
            HyperClient::Http(client) => client.call(req),
            HyperClient::Https(client) => client.call(req),
        }
    }
}

impl<C> Client<DropContextService<HyperClient, C>, C>
where
    C: Clone + Send + Sync + 'static,
{
    /// Create an HTTP client.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "http://www.my-api-implementation.com"
    pub fn try_new(base_path: &str) -> Result<Self, ClientInitError> {
        let uri = Uri::from_str(base_path)?;

        let scheme = uri.scheme_str().ok_or(ClientInitError::InvalidScheme)?;
        let scheme = scheme.to_ascii_lowercase();

        let connector = Connector::builder();

        let client_service = match scheme.as_str() {
            "http" => HyperClient::Http(hyper::client::Client::builder().build(connector.build())),
            "https" => {
                let connector = connector
                    .https()
                    .build()
                    .map_err(ClientInitError::SslError)?;
                HyperClient::Https(hyper::client::Client::builder().build(connector))
            }
            _ => {
                return Err(ClientInitError::InvalidScheme);
            }
        };

        let client_service = DropContextService::new(client_service);

        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, None)?,
            marker: PhantomData,
        })
    }
}

impl<C> Client<DropContextService<hyper::client::Client<hyper::client::HttpConnector, Body>, C>, C>
where
    C: Clone + Send + Sync + 'static,
{
    /// Create an HTTP client.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "http://www.my-api-implementation.com"
    pub fn try_new_http(base_path: &str) -> Result<Self, ClientInitError> {
        let http_connector = Connector::builder().build();

        Self::try_new_with_connector(base_path, Some("http"), http_connector)
    }
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
type HttpsConnector = hyper_openssl::HttpsConnector<hyper::client::HttpConnector>;

impl<C> Client<DropContextService<hyper::client::Client<HttpsConnector, Body>, C>, C>
where
    C: Clone + Send + Sync + 'static,
{
    /// Create a client with a TLS connection to the server
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "https://www.my-api-implementation.com"
    pub fn try_new_https(base_path: &str) -> Result<Self, ClientInitError> {
        let https_connector = Connector::builder()
            .https()
            .build()
            .map_err(ClientInitError::SslError)?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }

    /// Create a client with a TLS connection to the server using a pinned certificate
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "https://www.my-api-implementation.com"
    /// * `ca_certificate` - Path to CA certificate used to authenticate the server
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    pub fn try_new_https_pinned<CA>(
        base_path: &str,
        ca_certificate: CA,
    ) -> Result<Self, ClientInitError>
    where
        CA: AsRef<Path>,
    {
        let https_connector = Connector::builder()
            .https()
            .pin_server_certificate(ca_certificate)
            .build()
            .map_err(ClientInitError::SslError)?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }

    /// Create a client with a mutually authenticated TLS connection to the server.
    ///
    /// # Arguments
    /// * `base_path` - base path of the client API, i.e. "https://www.my-api-implementation.com"
    /// * `ca_certificate` - Path to CA certificate used to authenticate the server
    /// * `client_key` - Path to the client private key
    /// * `client_certificate` - Path to the client's public certificate associated with the private key
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    pub fn try_new_https_mutual<CA, K, D>(
        base_path: &str,
        ca_certificate: CA,
        client_key: K,
        client_certificate: D,
    ) -> Result<Self, ClientInitError>
    where
        CA: AsRef<Path>,
        K: AsRef<Path>,
        D: AsRef<Path>,
    {
        let https_connector = Connector::builder()
            .https()
            .pin_server_certificate(ca_certificate)
            .client_authentication(client_key, client_certificate)
            .build()
            .map_err(ClientInitError::SslError)?;
        Self::try_new_with_connector(base_path, Some("https"), https_connector)
    }
}

impl<S, C> Client<S, C>
where
    S: Service<(Request<Body>, C), Response = Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Clone + Send + Sync + 'static,
{
    /// Constructor for creating a `Client` by passing in a pre-made `hyper::service::Service` /
    /// `tower::Service`
    ///
    /// This allows adding custom wrappers around the underlying transport, for example for logging.
    pub fn try_new_with_client_service(
        client_service: S,
        base_path: &str,
    ) -> Result<Self, ClientInitError> {
        Ok(Self {
            client_service,
            base_path: into_base_path(base_path, None)?,
            marker: PhantomData,
        })
    }
}

/// Error type failing to create a Client
#[derive(Debug)]
pub enum ClientInitError {
    /// Invalid URL Scheme
    InvalidScheme,

    /// Invalid URI
    InvalidUri(hyper::http::uri::InvalidUri),

    /// Missing Hostname
    MissingHost,

    /// SSL Connection Error
    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
    SslError(native_tls::Error),

    /// SSL Connection Error
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    SslError(openssl::error::ErrorStack),
}

impl From<hyper::http::uri::InvalidUri> for ClientInitError {
    fn from(err: hyper::http::uri::InvalidUri) -> ClientInitError {
        ClientInitError::InvalidUri(err)
    }
}

impl fmt::Display for ClientInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &dyn fmt::Debug = self;
        s.fmt(f)
    }
}

impl Error for ClientInitError {
    fn description(&self) -> &str {
        "Failed to produce a hyper client."
    }
}

#[async_trait]
impl<S, C> Api<C> for Client<S, C>
where
    S: Service<(Request<Body>, C), Response = Response<Body>> + Clone + Sync + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<crate::ServiceError> + fmt::Display,
    C: Has<XSpanIdString> + Has<Option<AuthData>> + Clone + Send + Sync + 'static,
{
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), crate::ServiceError>> {
        match self.client_service.clone().poll_ready(cx) {
            Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
            Poll::Ready(Ok(o)) => Poll::Ready(Ok(o)),
            Poll::Pending => Poll::Pending,
        }
    }

    async fn create_account_accounts_create_post(
        &self,
        param_add_account_schema: models::AddAccountSchema,
        context: &C,
    ) -> Result<CreateAccountAccountsCreatePostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/accounts/create", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        // Body parameter
        let body = serde_json::to_string(&param_add_account_schema)
            .expect("impossible to fail to serialize");

        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(CreateAccountAccountsCreatePostResponse::AccountCreatedSuccessfully(body))
            }
            456 => Ok(CreateAccountAccountsCreatePostResponse::UsernameAlreadyUsed),
            457 => Ok(CreateAccountAccountsCreatePostResponse::EmailAlreadyUsed),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn create_character_characters_create_post(
        &self,
        param_add_character_schema: models::AddCharacterSchema,
        context: &C,
    ) -> Result<CreateCharacterCharactersCreatePostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/characters/create", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        // Body parameter
        let body = serde_json::to_string(&param_add_character_schema)
            .expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::CharacterResponseSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(CreateCharacterCharactersCreatePostResponse::SuccessfullyCreatedCharacter(body))
            }
            494 => Ok(CreateCharacterCharactersCreatePostResponse::NameAlreadyUsed),
            495 => Ok(
                CreateCharacterCharactersCreatePostResponse::MaximumCharactersReachedOnYourAccount,
            ),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn delete_character_characters_delete_post(
        &self,
        param_delete_character_schema: models::DeleteCharacterSchema,
        context: &C,
    ) -> Result<DeleteCharacterCharactersDeletePostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/characters/delete", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_delete_character_schema)
            .expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::CharacterResponseSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(DeleteCharacterCharactersDeletePostResponse::SuccessfullyDeletedCharacter(body))
            }
            498 => Ok(DeleteCharacterCharactersDeletePostResponse::CharacterNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_characters_characters_get(
        &self,
        param_sort: Option<String>,
        param_page: Option<i32>,
        param_size: Option<i32>,
        context: &C,
    ) -> Result<GetAllCharactersCharactersGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/characters/", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_sort) = param_sort {
                query_string.append_pair("sort", &param_sort);
            }
            if let Some(param_page) = param_page {
                query_string.append_pair("page", &param_page.to_string());
            }
            if let Some(param_size) = param_size {
                query_string.append_pair("size", &param_size.to_string());
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::DataPageCharacterSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(
                    GetAllCharactersCharactersGetResponse::SuccessfullyFetchedCharactersDetails(
                        body,
                    ),
                )
            }
            404 => Ok(GetAllCharactersCharactersGetResponse::CharactersNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_character_characters_name_get(
        &self,
        param_name: String,
        context: &C,
    ) -> Result<GetCharacterCharactersNameGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/characters/{name}",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::CharacterResponseSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GetCharacterCharactersNameGetResponse::SuccessfullyFetchedCharacter(body))
            }
            404 => Ok(GetCharacterCharactersNameGetResponse::CharacterNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_status_get(&self, context: &C) -> Result<GetStatusGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::StatusResponseSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GetStatusGetResponse::SuccessfulResponse(body))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_events_events_get(
        &self,
        param_page: Option<i32>,
        param_size: Option<i32>,
        context: &C,
    ) -> Result<GetAllEventsEventsGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/events/", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_page) = param_page {
                query_string.append_pair("page", &param_page.to_string());
            }
            if let Some(param_size) = param_size {
                query_string.append_pair("size", &param_size.to_string());
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::DataPageActiveEventSchema>(body)
                    .map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GetAllEventsEventsGetResponse::SuccessfullyFetchedEventsDetails(body))
            }
            404 => Ok(GetAllEventsEventsGetResponse::EventsNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_ge_items_ge_get(
        &self,
        param_page: Option<i32>,
        param_size: Option<i32>,
        context: &C,
    ) -> Result<GetAllGeItemsGeGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/ge/", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_page) = param_page {
                query_string.append_pair("page", &param_page.to_string());
            }
            if let Some(param_size) = param_size {
                query_string.append_pair("size", &param_size.to_string());
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::DataPageGeItemSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GetAllGeItemsGeGetResponse::FetchGrandExchangeItemsDetails(
                    body,
                ))
            }
            404 => Ok(GetAllGeItemsGeGetResponse::ItemNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_ge_item_ge_code_get(
        &self,
        param_code: String,
        context: &C,
    ) -> Result<GetGeItemGeCodeGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/ge/{code}",
            self.base_path,
            code = utf8_percent_encode(&param_code.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::GeItemResponseSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GetGeItemGeCodeGetResponse::SuccessfullyFetchedGrandExchangeItem(body))
            }
            404 => Ok(GetGeItemGeCodeGetResponse::ItemNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_items_items_get(
        &self,
        param_min_level: Option<i32>,
        param_max_level: Option<i32>,
        param_name: Option<String>,
        param_type: Option<String>,
        param_craft_skill: Option<String>,
        param_craft_material: Option<String>,
        param_page: Option<i32>,
        param_size: Option<i32>,
        context: &C,
    ) -> Result<GetAllItemsItemsGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/items/", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_min_level) = param_min_level {
                query_string.append_pair("min_level", &param_min_level.to_string());
            }
            if let Some(param_max_level) = param_max_level {
                query_string.append_pair("max_level", &param_max_level.to_string());
            }
            if let Some(param_name) = param_name {
                query_string.append_pair("name", &param_name);
            }
            if let Some(param_type) = param_type {
                query_string.append_pair("type", &param_type);
            }
            if let Some(param_craft_skill) = param_craft_skill {
                query_string.append_pair("craft_skill", &param_craft_skill);
            }
            if let Some(param_craft_material) = param_craft_material {
                query_string.append_pair("craft_material", &param_craft_material);
            }
            if let Some(param_page) = param_page {
                query_string.append_pair("page", &param_page.to_string());
            }
            if let Some(param_size) = param_size {
                query_string.append_pair("size", &param_size.to_string());
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::DataPageItemSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GetAllItemsItemsGetResponse::FetchItemsDetails(body))
            }
            404 => Ok(GetAllItemsItemsGetResponse::ItemsNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_item_items_code_get(
        &self,
        param_code: String,
        context: &C,
    ) -> Result<GetItemItemsCodeGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/items/{code}",
            self.base_path,
            code = utf8_percent_encode(&param_code.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::ItemResponseSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GetItemItemsCodeGetResponse::SuccessfullyFetchedItem(body))
            }
            404 => Ok(GetItemItemsCodeGetResponse::ItemNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_maps_maps_get(
        &self,
        param_content_type: Option<String>,
        param_content_code: Option<String>,
        param_page: Option<i32>,
        param_size: Option<i32>,
        context: &C,
    ) -> Result<GetAllMapsMapsGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/maps/", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_content_type) = param_content_type {
                query_string.append_pair("content_type", &param_content_type);
            }
            if let Some(param_content_code) = param_content_code {
                query_string.append_pair("content_code", &param_content_code);
            }
            if let Some(param_page) = param_page {
                query_string.append_pair("page", &param_page.to_string());
            }
            if let Some(param_size) = param_size {
                query_string.append_pair("size", &param_size.to_string());
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::DataPageMapSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GetAllMapsMapsGetResponse::SuccessfullyFetchedMapsDetails(
                    body,
                ))
            }
            404 => Ok(GetAllMapsMapsGetResponse::MapsNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_map_maps_xy_get(
        &self,
        param_x: i32,
        param_y: i32,
        context: &C,
    ) -> Result<GetMapMapsXyGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/maps/{x}/{y}",
            self.base_path,
            x = utf8_percent_encode(&param_x.to_string(), ID_ENCODE_SET),
            y = utf8_percent_encode(&param_y.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::MapResponseSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GetMapMapsXyGetResponse::SuccessfullyFetchedMap(body))
            }
            404 => Ok(GetMapMapsXyGetResponse::MapNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_monsters_monsters_get(
        &self,
        param_min_level: Option<i32>,
        param_max_level: Option<i32>,
        param_drop: Option<String>,
        param_page: Option<i32>,
        param_size: Option<i32>,
        context: &C,
    ) -> Result<GetAllMonstersMonstersGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/monsters/", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_min_level) = param_min_level {
                query_string.append_pair("min_level", &param_min_level.to_string());
            }
            if let Some(param_max_level) = param_max_level {
                query_string.append_pair("max_level", &param_max_level.to_string());
            }
            if let Some(param_drop) = param_drop {
                query_string.append_pair("drop", &param_drop);
            }
            if let Some(param_page) = param_page {
                query_string.append_pair("page", &param_page.to_string());
            }
            if let Some(param_size) = param_size {
                query_string.append_pair("size", &param_size.to_string());
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::DataPageMonsterSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GetAllMonstersMonstersGetResponse::SuccessfullyFetchedMonstersDetails(body))
            }
            404 => Ok(GetAllMonstersMonstersGetResponse::MonstersNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_monster_monsters_code_get(
        &self,
        param_code: String,
        context: &C,
    ) -> Result<GetMonsterMonstersCodeGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/monsters/{code}",
            self.base_path,
            code = utf8_percent_encode(&param_code.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::MonsterResponseSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GetMonsterMonstersCodeGetResponse::SuccessfullyFetchedMonster(body))
            }
            404 => Ok(GetMonsterMonstersCodeGetResponse::MonsterNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn change_password_my_change_password_post(
        &self,
        param_change_password: models::ChangePassword,
        context: &C,
    ) -> Result<ChangePasswordMyChangePasswordPostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/my/change_password", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        // Body parameter
        let body =
            serde_json::to_string(&param_change_password).expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ChangePasswordMyChangePasswordPostResponse::PasswordChangedSuccessfully(body))
            }
            458 => Ok(ChangePasswordMyChangePasswordPostResponse::UseADifferentPassword),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_bank_golds_my_bank_gold_get(
        &self,
        context: &C,
    ) -> Result<GetBankGoldsMyBankGoldGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/my/bank/gold", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::GoldBankResponseSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GetBankGoldsMyBankGoldGetResponse::SuccessfullyFetchedGolds(
                    body,
                ))
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_bank_items_my_bank_items_get(
        &self,
        param_item_code: Option<String>,
        param_page: Option<i32>,
        param_size: Option<i32>,
        context: &C,
    ) -> Result<GetBankItemsMyBankItemsGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/my/bank/items", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_item_code) = param_item_code {
                query_string.append_pair("item_code", &param_item_code);
            }
            if let Some(param_page) = param_page {
                query_string.append_pair("page", &param_page.to_string());
            }
            if let Some(param_size) = param_size {
                query_string.append_pair("size", &param_size.to_string());
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::DataPageSimpleItemSchema>(body).map_err(
                    |e| ApiError(format!("Response body did not match the schema: {}", e)),
                )?;
                Ok(GetBankItemsMyBankItemsGetResponse::SuccessfullyFetchedItems(body))
            }
            404 => Ok(GetBankItemsMyBankItemsGetResponse::ItemsNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_accept_new_task_my_name_action_task_new_post(
        &self,
        param_name: String,
        context: &C,
    ) -> Result<ActionAcceptNewTaskMyNameActionTaskNewPostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/task/new",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::TaskResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ActionAcceptNewTaskMyNameActionTaskNewPostResponse::NewTaskSuccessfullyAccepted
                    (body)
                )
            }
            498 => {
                Ok(
                    ActionAcceptNewTaskMyNameActionTaskNewPostResponse::CharacterNotFound
                )
            }
            499 => {
                Ok(
                    ActionAcceptNewTaskMyNameActionTaskNewPostResponse::CharacterInCooldown
                )
            }
            486 => {
                Ok(
                    ActionAcceptNewTaskMyNameActionTaskNewPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                )
            }
            598 => {
                Ok(
                    ActionAcceptNewTaskMyNameActionTaskNewPostResponse::TasksMasterNotFoundOnThisMap
                )
            }
            489 => {
                Ok(
                    ActionAcceptNewTaskMyNameActionTaskNewPostResponse::CharacterAlreadyHasATask
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_complete_task_my_name_action_task_complete_post(
        &self,
        param_name: String,
        context: &C,
    ) -> Result<ActionCompleteTaskMyNameActionTaskCompletePostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/task/complete",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::TaskRewardResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ActionCompleteTaskMyNameActionTaskCompletePostResponse::TheTaskHasBeenSuccessfullyCompleted
                    (body)
                )
            }
            498 => {
                Ok(
                    ActionCompleteTaskMyNameActionTaskCompletePostResponse::CharacterNotFound
                )
            }
            499 => {
                Ok(
                    ActionCompleteTaskMyNameActionTaskCompletePostResponse::CharacterInCooldown
                )
            }
            486 => {
                Ok(
                    ActionCompleteTaskMyNameActionTaskCompletePostResponse::AnActionIsAlreadyInProgressByYourCharacter
                )
            }
            598 => {
                Ok(
                    ActionCompleteTaskMyNameActionTaskCompletePostResponse::TasksMasterNotFoundOnThisMap
                )
            }
            488 => {
                Ok(
                    ActionCompleteTaskMyNameActionTaskCompletePostResponse::CharacterHasNotCompletedTheTask
                )
            }
            487 => {
                Ok(
                    ActionCompleteTaskMyNameActionTaskCompletePostResponse::CharacterHasNoTask
                )
            }
            497 => {
                Ok(
                    ActionCompleteTaskMyNameActionTaskCompletePostResponse::CharacterInventoryIsFull
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_crafting_my_name_action_crafting_post(
        &self,
        param_name: String,
        param_crafting_schema: models::CraftingSchema,
        context: &C,
    ) -> Result<ActionCraftingMyNameActionCraftingPostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/crafting",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body =
            serde_json::to_string(&param_crafting_schema).expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::SkillResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ActionCraftingMyNameActionCraftingPostResponse::TheItemWasSuccessfullyCrafted
                    (body)
                )
            }
            404 => {
                Ok(
                    ActionCraftingMyNameActionCraftingPostResponse::CraftNotFound
                )
            }
            598 => {
                Ok(
                    ActionCraftingMyNameActionCraftingPostResponse::WorkshopNotFoundOnThisMap
                )
            }
            498 => {
                Ok(
                    ActionCraftingMyNameActionCraftingPostResponse::CharacterNotFound
                )
            }
            497 => {
                Ok(
                    ActionCraftingMyNameActionCraftingPostResponse::CharacterInventoryIsFull
                )
            }
            499 => {
                Ok(
                    ActionCraftingMyNameActionCraftingPostResponse::CharacterInCooldown
                )
            }
            486 => {
                Ok(
                    ActionCraftingMyNameActionCraftingPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                )
            }
            493 => {
                Ok(
                    ActionCraftingMyNameActionCraftingPostResponse::NotSkillLevelRequired
                )
            }
            478 => {
                Ok(
                    ActionCraftingMyNameActionCraftingPostResponse::MissingItemOrInsufficientQuantityInYourInventory
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_delete_item_my_name_action_delete_post(
        &self,
        param_name: String,
        param_simple_item_schema: models::SimpleItemSchema,
        context: &C,
    ) -> Result<ActionDeleteItemMyNameActionDeletePostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/delete",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_simple_item_schema)
            .expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::DeleteItemResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ActionDeleteItemMyNameActionDeletePostResponse::ItemSuccessfullyDeletedFromYourCharacter
                    (body)
                )
            }
            498 => {
                Ok(
                    ActionDeleteItemMyNameActionDeletePostResponse::CharacterNotFound
                )
            }
            499 => {
                Ok(
                    ActionDeleteItemMyNameActionDeletePostResponse::CharacterInCooldown
                )
            }
            486 => {
                Ok(
                    ActionDeleteItemMyNameActionDeletePostResponse::AnActionIsAlreadyInProgressByYourCharacter
                )
            }
            478 => {
                Ok(
                    ActionDeleteItemMyNameActionDeletePostResponse::MissingItemOrInsufficientQuantityInYourInventory
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_deposit_bank_gold_my_name_action_bank_deposit_gold_post(
        &self,
        param_name: String,
        param_deposit_withdraw_gold_schema: models::DepositWithdrawGoldSchema,
        context: &C,
    ) -> Result<ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/bank/deposit/gold",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_deposit_withdraw_gold_schema)
            .expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GoldResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse::GoldsSuccessfullyDepositedInYourBank
                    (body)
                )
            }
            598 => {
                Ok(
                    ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse::BankNotFoundOnThisMap
                )
            }
            492 => {
                Ok(
                    ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse::InsufficientGoldsOnYourCharacter
                )
            }
            498 => {
                Ok(
                    ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse::CharacterNotFound
                )
            }
            499 => {
                Ok(
                    ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse::CharacterInCooldown
                )
            }
            461 => {
                Ok(
                    ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse::ATransactionIsAlreadyInProgressWithThisItem
                )
            }
            486 => {
                Ok(
                    ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_deposit_bank_my_name_action_bank_deposit_post(
        &self,
        param_name: String,
        param_simple_item_schema: models::SimpleItemSchema,
        context: &C,
    ) -> Result<ActionDepositBankMyNameActionBankDepositPostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/bank/deposit",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_simple_item_schema)
            .expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ActionItemBankResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ActionDepositBankMyNameActionBankDepositPostResponse::ItemSuccessfullyDepositedInYourBank
                    (body)
                )
            }
            598 => {
                Ok(
                    ActionDepositBankMyNameActionBankDepositPostResponse::BankNotFoundOnThisMap
                )
            }
            404 => {
                Ok(
                    ActionDepositBankMyNameActionBankDepositPostResponse::ItemNotFound
                )
            }
            461 => {
                Ok(
                    ActionDepositBankMyNameActionBankDepositPostResponse::ATransactionIsAlreadyInProgressWithThisItem
                )
            }
            498 => {
                Ok(
                    ActionDepositBankMyNameActionBankDepositPostResponse::CharacterNotFound
                )
            }
            499 => {
                Ok(
                    ActionDepositBankMyNameActionBankDepositPostResponse::CharacterInCooldown
                )
            }
            486 => {
                Ok(
                    ActionDepositBankMyNameActionBankDepositPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                )
            }
            478 => {
                Ok(
                    ActionDepositBankMyNameActionBankDepositPostResponse::MissingItemOrInsufficientQuantityInYourInventory
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_equip_item_my_name_action_equip_post(
        &self,
        param_name: String,
        param_equip_schema: models::EquipSchema,
        context: &C,
    ) -> Result<ActionEquipItemMyNameActionEquipPostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/equip",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body =
            serde_json::to_string(&param_equip_schema).expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::EquipmentResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ActionEquipItemMyNameActionEquipPostResponse::TheItemHasBeenSuccessfullyEquippedOnYourCharacter
                    (body)
                )
            }
            404 => {
                Ok(
                    ActionEquipItemMyNameActionEquipPostResponse::ItemNotFound
                )
            }
            498 => {
                Ok(
                    ActionEquipItemMyNameActionEquipPostResponse::CharacterNotFound
                )
            }
            499 => {
                Ok(
                    ActionEquipItemMyNameActionEquipPostResponse::CharacterInCooldown
                )
            }
            486 => {
                Ok(
                    ActionEquipItemMyNameActionEquipPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                )
            }
            478 => {
                Ok(
                    ActionEquipItemMyNameActionEquipPostResponse::MissingItemOrInsufficientQuantityInYourInventory
                )
            }
            496 => {
                Ok(
                    ActionEquipItemMyNameActionEquipPostResponse::CharacterLevelIsInsufficient
                )
            }
            491 => {
                Ok(
                    ActionEquipItemMyNameActionEquipPostResponse::SlotIsNotEmpty
                )
            }
            485 => {
                Ok(
                    ActionEquipItemMyNameActionEquipPostResponse::ThisItemIsAlreadyEquipped
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_fight_my_name_action_fight_post(
        &self,
        param_name: String,
        context: &C,
    ) -> Result<ActionFightMyNameActionFightPostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/fight",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::CharacterFightResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ActionFightMyNameActionFightPostResponse::TheFightEndedSuccessfully
                    (body)
                )
            }
            498 => {
                Ok(
                    ActionFightMyNameActionFightPostResponse::CharacterNotFound
                )
            }
            499 => {
                Ok(
                    ActionFightMyNameActionFightPostResponse::CharacterInCooldown
                )
            }
            598 => {
                Ok(
                    ActionFightMyNameActionFightPostResponse::MonsterNotFoundOnThisMap
                )
            }
            486 => {
                Ok(
                    ActionFightMyNameActionFightPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                )
            }
            497 => {
                Ok(
                    ActionFightMyNameActionFightPostResponse::CharacterInventoryIsFull
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_gathering_my_name_action_gathering_post(
        &self,
        param_name: String,
        context: &C,
    ) -> Result<ActionGatheringMyNameActionGatheringPostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/gathering",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::SkillResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ActionGatheringMyNameActionGatheringPostResponse::TheResourceHasBeenSuccessfullyGathered
                    (body)
                )
            }
            498 => {
                Ok(
                    ActionGatheringMyNameActionGatheringPostResponse::CharacterNotFound
                )
            }
            499 => {
                Ok(
                    ActionGatheringMyNameActionGatheringPostResponse::CharacterInCooldown
                )
            }
            598 => {
                Ok(
                    ActionGatheringMyNameActionGatheringPostResponse::ResourceNotFoundOnThisMap
                )
            }
            486 => {
                Ok(
                    ActionGatheringMyNameActionGatheringPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                )
            }
            493 => {
                Ok(
                    ActionGatheringMyNameActionGatheringPostResponse::NotSkillLevelRequired
                )
            }
            497 => {
                Ok(
                    ActionGatheringMyNameActionGatheringPostResponse::CharacterInventoryIsFull
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_ge_buy_item_my_name_action_ge_buy_post(
        &self,
        param_name: String,
        param_ge_transaction_item_schema: models::GeTransactionItemSchema,
        context: &C,
    ) -> Result<ActionGeBuyItemMyNameActionGeBuyPostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/ge/buy",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_ge_transaction_item_schema)
            .expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GeTransactionResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ActionGeBuyItemMyNameActionGeBuyPostResponse::ItemSuccessfullyBuyFromTheGrandExchange
                    (body)
                )
            }
            598 => {
                Ok(
                    ActionGeBuyItemMyNameActionGeBuyPostResponse::GrandExchangeNotFoundOnThisMap
                )
            }
            498 => {
                Ok(
                    ActionGeBuyItemMyNameActionGeBuyPostResponse::CharacterNotFound
                )
            }
            497 => {
                Ok(
                    ActionGeBuyItemMyNameActionGeBuyPostResponse::CharacterInventoryIsFull
                )
            }
            499 => {
                Ok(
                    ActionGeBuyItemMyNameActionGeBuyPostResponse::CharacterInCooldown
                )
            }
            483 => {
                Ok(
                    ActionGeBuyItemMyNameActionGeBuyPostResponse::ATransactionIsAlreadyInProgressOnThisItemByAAnotherCharacter
                )
            }
            486 => {
                Ok(
                    ActionGeBuyItemMyNameActionGeBuyPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                )
            }
            492 => {
                Ok(
                    ActionGeBuyItemMyNameActionGeBuyPostResponse::InsufficientGoldsOnYourCharacter
                )
            }
            480 => {
                Ok(
                    ActionGeBuyItemMyNameActionGeBuyPostResponse::NoStockForThisItem
                )
            }
            482 => {
                Ok(
                    ActionGeBuyItemMyNameActionGeBuyPostResponse::NoItemAtThisPrice
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_ge_sell_item_my_name_action_ge_sell_post(
        &self,
        param_name: String,
        param_ge_transaction_item_schema: models::GeTransactionItemSchema,
        context: &C,
    ) -> Result<ActionGeSellItemMyNameActionGeSellPostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/ge/sell",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_ge_transaction_item_schema)
            .expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GeTransactionResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ActionGeSellItemMyNameActionGeSellPostResponse::ItemSuccessfullySellAtTheGrandExchange
                    (body)
                )
            }
            498 => {
                Ok(
                    ActionGeSellItemMyNameActionGeSellPostResponse::CharacterNotFound
                )
            }
            499 => {
                Ok(
                    ActionGeSellItemMyNameActionGeSellPostResponse::CharacterInCooldown
                )
            }
            404 => {
                Ok(
                    ActionGeSellItemMyNameActionGeSellPostResponse::ItemNotFound
                )
            }
            483 => {
                Ok(
                    ActionGeSellItemMyNameActionGeSellPostResponse::ATransactionIsAlreadyInProgressOnThisItemByAAnotherCharacter
                )
            }
            486 => {
                Ok(
                    ActionGeSellItemMyNameActionGeSellPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                )
            }
            478 => {
                Ok(
                    ActionGeSellItemMyNameActionGeSellPostResponse::MissingItemOrInsufficientQuantityInYourInventory
                )
            }
            482 => {
                Ok(
                    ActionGeSellItemMyNameActionGeSellPostResponse::NoItemAtThisPrice
                )
            }
            598 => {
                Ok(
                    ActionGeSellItemMyNameActionGeSellPostResponse::GrandExchangeNotFoundOnThisMap
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_move_my_name_action_move_post(
        &self,
        param_name: String,
        param_destination_schema: models::DestinationSchema,
        context: &C,
    ) -> Result<ActionMoveMyNameActionMovePostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/move",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_destination_schema)
            .expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::CharacterMovementResponseSchema>(body)
                    .map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(ActionMoveMyNameActionMovePostResponse::TheCharacterHasMovedSuccessfully(body))
            }
            498 => Ok(ActionMoveMyNameActionMovePostResponse::CharacterNotFound),
            499 => Ok(ActionMoveMyNameActionMovePostResponse::CharacterInCooldown),
            490 => Ok(ActionMoveMyNameActionMovePostResponse::CharacterAlreadyAtDestination),
            404 => Ok(ActionMoveMyNameActionMovePostResponse::MapNotFound),
            486 => Ok(
                ActionMoveMyNameActionMovePostResponse::AnActionIsAlreadyInProgressByYourCharacter,
            ),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_recycling_my_name_action_recycling_post(
        &self,
        param_name: String,
        param_recycling_schema: models::RecyclingSchema,
        context: &C,
    ) -> Result<ActionRecyclingMyNameActionRecyclingPostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/recycling",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_recycling_schema)
            .expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::RecyclingResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ActionRecyclingMyNameActionRecyclingPostResponse::TheItemsWereSuccessfullyRecycled
                    (body)
                )
            }
            404 => {
                Ok(
                    ActionRecyclingMyNameActionRecyclingPostResponse::ItemNotFound
                )
            }
            598 => {
                Ok(
                    ActionRecyclingMyNameActionRecyclingPostResponse::WorkshopNotFoundOnThisMap
                )
            }
            498 => {
                Ok(
                    ActionRecyclingMyNameActionRecyclingPostResponse::CharacterNotFound
                )
            }
            497 => {
                Ok(
                    ActionRecyclingMyNameActionRecyclingPostResponse::CharacterInventoryIsFull
                )
            }
            499 => {
                Ok(
                    ActionRecyclingMyNameActionRecyclingPostResponse::CharacterInCooldown
                )
            }
            486 => {
                Ok(
                    ActionRecyclingMyNameActionRecyclingPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                )
            }
            493 => {
                Ok(
                    ActionRecyclingMyNameActionRecyclingPostResponse::NotSkillLevelRequired
                )
            }
            478 => {
                Ok(
                    ActionRecyclingMyNameActionRecyclingPostResponse::MissingItemOrInsufficientQuantityInYourInventory
                )
            }
            473 => {
                Ok(
                    ActionRecyclingMyNameActionRecyclingPostResponse::ThisItemCannotBeRecycled
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_task_exchange_my_name_action_task_exchange_post(
        &self,
        param_name: String,
        context: &C,
    ) -> Result<ActionTaskExchangeMyNameActionTaskExchangePostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/task/exchange",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::TaskRewardResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ActionTaskExchangeMyNameActionTaskExchangePostResponse::TheTasksCoinsHaveBeenSuccessfullyExchanged
                    (body)
                )
            }
            498 => {
                Ok(
                    ActionTaskExchangeMyNameActionTaskExchangePostResponse::CharacterNotFound
                )
            }
            499 => {
                Ok(
                    ActionTaskExchangeMyNameActionTaskExchangePostResponse::CharacterInCooldown
                )
            }
            486 => {
                Ok(
                    ActionTaskExchangeMyNameActionTaskExchangePostResponse::AnActionIsAlreadyInProgressByYourCharacter
                )
            }
            598 => {
                Ok(
                    ActionTaskExchangeMyNameActionTaskExchangePostResponse::TasksMasterNotFoundOnThisMap
                )
            }
            478 => {
                Ok(
                    ActionTaskExchangeMyNameActionTaskExchangePostResponse::MissingItemOrInsufficientQuantityInYourInventory
                )
            }
            497 => {
                Ok(
                    ActionTaskExchangeMyNameActionTaskExchangePostResponse::CharacterInventoryIsFull
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_unequip_item_my_name_action_unequip_post(
        &self,
        param_name: String,
        param_unequip_schema: models::UnequipSchema,
        context: &C,
    ) -> Result<ActionUnequipItemMyNameActionUnequipPostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/unequip",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body =
            serde_json::to_string(&param_unequip_schema).expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::EquipmentResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ActionUnequipItemMyNameActionUnequipPostResponse::TheItemHasBeenSuccessfullyUnequippedAndAddedInHisInventory
                    (body)
                )
            }
            404 => {
                Ok(
                    ActionUnequipItemMyNameActionUnequipPostResponse::ItemNotFound
                )
            }
            498 => {
                Ok(
                    ActionUnequipItemMyNameActionUnequipPostResponse::CharacterNotFound
                )
            }
            499 => {
                Ok(
                    ActionUnequipItemMyNameActionUnequipPostResponse::CharacterInCooldown
                )
            }
            486 => {
                Ok(
                    ActionUnequipItemMyNameActionUnequipPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                )
            }
            491 => {
                Ok(
                    ActionUnequipItemMyNameActionUnequipPostResponse::SlotIsEmpty
                )
            }
            497 => {
                Ok(
                    ActionUnequipItemMyNameActionUnequipPostResponse::CharacterInventoryIsFull
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_withdraw_bank_gold_my_name_action_bank_withdraw_gold_post(
        &self,
        param_name: String,
        param_deposit_withdraw_gold_schema: models::DepositWithdrawGoldSchema,
        context: &C,
    ) -> Result<ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/bank/withdraw/gold",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_deposit_withdraw_gold_schema)
            .expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::GoldResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse::GoldsSuccessfullyWithdrawFromYourBank
                    (body)
                )
            }
            498 => {
                Ok(
                    ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse::CharacterNotFound
                )
            }
            499 => {
                Ok(
                    ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse::CharacterInCooldown
                )
            }
            461 => {
                Ok(
                    ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse::ATransactionIsAlreadyInProgressWithThisItem
                )
            }
            486 => {
                Ok(
                    ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                )
            }
            598 => {
                Ok(
                    ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse::BankNotFoundOnThisMap
                )
            }
            460 => {
                Ok(
                    ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse::InsufficientGoldsInYourBank
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn action_withdraw_bank_my_name_action_bank_withdraw_post(
        &self,
        param_name: String,
        param_simple_item_schema: models::SimpleItemSchema,
        context: &C,
    ) -> Result<ActionWithdrawBankMyNameActionBankWithdrawPostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/my/{name}/action/bank/withdraw",
            self.base_path,
            name = utf8_percent_encode(&param_name.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let body = serde_json::to_string(&param_simple_item_schema)
            .expect("impossible to fail to serialize");
        *request.body_mut() = Body::from(body);

        let header = "application/json";
        request.headers_mut().insert(
            CONTENT_TYPE,
            match HeaderValue::from_str(header) {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create header: {} - {}",
                        header, e
                    )))
                }
            },
        );
        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                        .into_raw()
                        .map_err(|e| ApiError(format!("Failed to read response: {}", e))).await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body = serde_json::from_str::<models::ActionItemBankResponseSchema>(body).map_err(|e| {
                    ApiError(format!("Response body did not match the schema: {}", e))
                })?;
                Ok(ActionWithdrawBankMyNameActionBankWithdrawPostResponse::ItemSuccessfullyWithdrawFromYourBank
                    (body)
                )
            }
            404 => {
                Ok(
                    ActionWithdrawBankMyNameActionBankWithdrawPostResponse::ItemNotFound
                )
            }
            498 => {
                Ok(
                    ActionWithdrawBankMyNameActionBankWithdrawPostResponse::CharacterNotFound
                )
            }
            499 => {
                Ok(
                    ActionWithdrawBankMyNameActionBankWithdrawPostResponse::CharacterInCooldown
                )
            }
            461 => {
                Ok(
                    ActionWithdrawBankMyNameActionBankWithdrawPostResponse::ATransactionIsAlreadyInProgressWithThisItem
                )
            }
            486 => {
                Ok(
                    ActionWithdrawBankMyNameActionBankWithdrawPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                )
            }
            497 => {
                Ok(
                    ActionWithdrawBankMyNameActionBankWithdrawPostResponse::CharacterInventoryIsFull
                )
            }
            598 => {
                Ok(
                    ActionWithdrawBankMyNameActionBankWithdrawPostResponse::BankNotFoundOnThisMap
                )
            }
            478 => {
                Ok(
                    ActionWithdrawBankMyNameActionBankWithdrawPostResponse::MissingItemOrInsufficientQuantityInYourInventory
                )
            }
            code => {
                let headers = response.headers().clone();
                let body = response.into_body()
                       .take(100)
                       .into_raw().await;
                Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_characters_logs_my_logs_get(
        &self,
        param_page: Option<i32>,
        param_size: Option<i32>,
        context: &C,
    ) -> Result<GetAllCharactersLogsMyLogsGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/my/logs", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_page) = param_page {
                query_string.append_pair("page", &param_page.to_string());
            }
            if let Some(param_size) = param_size {
                query_string.append_pair("size", &param_size.to_string());
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::DataPageLogSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GetAllCharactersLogsMyLogsGetResponse::SuccessfullyFetchedLogs(body))
            }
            404 => Ok(GetAllCharactersLogsMyLogsGetResponse::LogsNotFound),
            498 => Ok(GetAllCharactersLogsMyLogsGetResponse::CharacterNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_my_characters_my_characters_get(
        &self,
        context: &C,
    ) -> Result<GetMyCharactersMyCharactersGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/my/characters", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Bearer(bearer_header) => {
                    let auth = swagger::auth::Header(bearer_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::MyCharactersListSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GetMyCharactersMyCharactersGetResponse::SuccessfullyFetchedCharacters(body))
            }
            404 => Ok(GetMyCharactersMyCharactersGetResponse::CharactersNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_all_resources_resources_get(
        &self,
        param_min_level: Option<i32>,
        param_max_level: Option<i32>,
        param_skill: Option<String>,
        param_drop: Option<String>,
        param_page: Option<i32>,
        param_size: Option<i32>,
        context: &C,
    ) -> Result<GetAllResourcesResourcesGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/resources/", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            if let Some(param_min_level) = param_min_level {
                query_string.append_pair("min_level", &param_min_level.to_string());
            }
            if let Some(param_max_level) = param_max_level {
                query_string.append_pair("max_level", &param_max_level.to_string());
            }
            if let Some(param_skill) = param_skill {
                query_string.append_pair("skill", &param_skill);
            }
            if let Some(param_drop) = param_drop {
                query_string.append_pair("drop", &param_drop);
            }
            if let Some(param_page) = param_page {
                query_string.append_pair("page", &param_page.to_string());
            }
            if let Some(param_size) = param_size {
                query_string.append_pair("size", &param_size.to_string());
            }
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::DataPageResourceSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GetAllResourcesResourcesGetResponse::SuccessfullyFetchedResourcesDetails(body))
            }
            404 => Ok(GetAllResourcesResourcesGetResponse::ResourcesNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn get_resource_resources_code_get(
        &self,
        param_code: String,
        context: &C,
    ) -> Result<GetResourceResourcesCodeGetResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!(
            "{}/resources/{code}",
            self.base_path,
            code = utf8_percent_encode(&param_code.to_string(), ID_ENCODE_SET)
        );

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::ResourceResponseSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GetResourceResourcesCodeGetResponse::SuccessfullyFetchedResource(body))
            }
            404 => Ok(GetResourceResourcesCodeGetResponse::RessourceNotFound),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }

    async fn generate_token_token_post(
        &self,
        context: &C,
    ) -> Result<GenerateTokenTokenPostResponse, ApiError> {
        let mut client_service = self.client_service.clone();
        let mut uri = format!("{}/token/", self.base_path);

        // Query parameters
        let query_string = {
            let mut query_string = form_urlencoded::Serializer::new("".to_owned());
            query_string.finish()
        };
        if !query_string.is_empty() {
            uri += "?";
            uri += &query_string;
        }

        let uri = match Uri::from_str(&uri) {
            Ok(uri) => uri,
            Err(err) => return Err(ApiError(format!("Unable to build URI: {}", err))),
        };

        let mut request = match Request::builder()
            .method("POST")
            .uri(uri)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(e) => return Err(ApiError(format!("Unable to create request: {}", e))),
        };

        let header = HeaderValue::from_str(Has::<XSpanIdString>::get(context).0.as_str());
        request.headers_mut().insert(
            HeaderName::from_static("x-span-id"),
            match header {
                Ok(h) => h,
                Err(e) => {
                    return Err(ApiError(format!(
                        "Unable to create X-Span ID header value: {}",
                        e
                    )))
                }
            },
        );

        #[allow(clippy::collapsible_match)]
        if let Some(auth_data) = Has::<Option<AuthData>>::get(context).as_ref() {
            // Currently only authentication with Basic and Bearer are supported
            #[allow(clippy::single_match, clippy::match_single_binding)]
            match auth_data {
                AuthData::Basic(basic_header) => {
                    let auth = swagger::auth::Header(basic_header.clone());
                    let header = match HeaderValue::from_str(&format!("{}", auth)) {
                        Ok(h) => h,
                        Err(e) => {
                            return Err(ApiError(format!(
                                "Unable to create Authorization header: {}",
                                e
                            )))
                        }
                    };
                    request
                        .headers_mut()
                        .insert(hyper::header::AUTHORIZATION, header);
                }
                _ => {}
            }
        }

        let response = client_service
            .call((request, context.clone()))
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .await?;

        match response.status().as_u16() {
            200 => {
                let body = response.into_body();
                let body = body
                    .into_raw()
                    .map_err(|e| ApiError(format!("Failed to read response: {}", e)))
                    .await?;
                let body = str::from_utf8(&body)
                    .map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                let body =
                    serde_json::from_str::<models::TokenResponseSchema>(body).map_err(|e| {
                        ApiError(format!("Response body did not match the schema: {}", e))
                    })?;
                Ok(GenerateTokenTokenPostResponse::TokenGeneratedSuccessfully(
                    body,
                ))
            }
            455 => Ok(GenerateTokenTokenPostResponse::TokenGenerationFailed),
            code => {
                let headers = response.headers().clone();
                let body = response.into_body().take(100).into_raw().await;
                Err(ApiError(format!(
                    "Unexpected response code {}:\n{:?}\n\n{}",
                    code,
                    headers,
                    match body {
                        Ok(body) => match String::from_utf8(body) {
                            Ok(body) => body,
                            Err(e) => format!("<Body was not UTF8: {:?}>", e),
                        },
                        Err(e) => format!("<Failed to read body: {}>", e),
                    }
                )))
            }
        }
    }
}
