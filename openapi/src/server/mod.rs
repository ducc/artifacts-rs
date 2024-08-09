use futures::{future, future::BoxFuture, future::FutureExt, stream, stream::TryStreamExt, Stream};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use hyper::{Body, HeaderMap, Request, Response, StatusCode};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
use url::form_urlencoded;

#[allow(unused_imports)]
use crate::{header, models, AuthenticationApi};

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

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

mod server_auth;

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/$",
            r"^/accounts/create$",
            r"^/characters/$",
            r"^/characters/create$",
            r"^/characters/delete$",
            r"^/characters/(?P<name>[^/?#]*)$",
            r"^/events/$",
            r"^/ge/$",
            r"^/ge/(?P<code>[^/?#]*)$",
            r"^/items/$",
            r"^/items/(?P<code>[^/?#]*)$",
            r"^/maps/$",
            r"^/maps/(?P<x>[^/?#]*)/(?P<y>[^/?#]*)$",
            r"^/monsters/$",
            r"^/monsters/(?P<code>[^/?#]*)$",
            r"^/my/bank/gold$",
            r"^/my/bank/items$",
            r"^/my/change_password$",
            r"^/my/characters$",
            r"^/my/logs$",
            r"^/my/(?P<name>[^/?#]*)/action/bank/deposit$",
            r"^/my/(?P<name>[^/?#]*)/action/bank/deposit/gold$",
            r"^/my/(?P<name>[^/?#]*)/action/bank/withdraw$",
            r"^/my/(?P<name>[^/?#]*)/action/bank/withdraw/gold$",
            r"^/my/(?P<name>[^/?#]*)/action/crafting$",
            r"^/my/(?P<name>[^/?#]*)/action/delete$",
            r"^/my/(?P<name>[^/?#]*)/action/equip$",
            r"^/my/(?P<name>[^/?#]*)/action/fight$",
            r"^/my/(?P<name>[^/?#]*)/action/gathering$",
            r"^/my/(?P<name>[^/?#]*)/action/ge/buy$",
            r"^/my/(?P<name>[^/?#]*)/action/ge/sell$",
            r"^/my/(?P<name>[^/?#]*)/action/move$",
            r"^/my/(?P<name>[^/?#]*)/action/recycling$",
            r"^/my/(?P<name>[^/?#]*)/action/task/complete$",
            r"^/my/(?P<name>[^/?#]*)/action/task/exchange$",
            r"^/my/(?P<name>[^/?#]*)/action/task/new$",
            r"^/my/(?P<name>[^/?#]*)/action/unequip$",
            r"^/resources/$",
            r"^/resources/(?P<code>[^/?#]*)$",
            r"^/token/$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_: usize = 0;
    pub(crate) static ID_ACCOUNTS_CREATE: usize = 1;
    pub(crate) static ID_CHARACTERS_: usize = 2;
    pub(crate) static ID_CHARACTERS_CREATE: usize = 3;
    pub(crate) static ID_CHARACTERS_DELETE: usize = 4;
    pub(crate) static ID_CHARACTERS_NAME: usize = 5;
    lazy_static! {
        pub static ref REGEX_CHARACTERS_NAME: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/characters/(?P<name>[^/?#]*)$")
                .expect("Unable to create regex for CHARACTERS_NAME");
    }
    pub(crate) static ID_EVENTS_: usize = 6;
    pub(crate) static ID_GE_: usize = 7;
    pub(crate) static ID_GE_CODE: usize = 8;
    lazy_static! {
        pub static ref REGEX_GE_CODE: regex::Regex = #[allow(clippy::invalid_regex)]
        regex::Regex::new(r"^/ge/(?P<code>[^/?#]*)$")
            .expect("Unable to create regex for GE_CODE");
    }
    pub(crate) static ID_ITEMS_: usize = 9;
    pub(crate) static ID_ITEMS_CODE: usize = 10;
    lazy_static! {
        pub static ref REGEX_ITEMS_CODE: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/items/(?P<code>[^/?#]*)$")
                .expect("Unable to create regex for ITEMS_CODE");
    }
    pub(crate) static ID_MAPS_: usize = 11;
    pub(crate) static ID_MAPS_X_Y: usize = 12;
    lazy_static! {
        pub static ref REGEX_MAPS_X_Y: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/maps/(?P<x>[^/?#]*)/(?P<y>[^/?#]*)$")
                .expect("Unable to create regex for MAPS_X_Y");
    }
    pub(crate) static ID_MONSTERS_: usize = 13;
    pub(crate) static ID_MONSTERS_CODE: usize = 14;
    lazy_static! {
        pub static ref REGEX_MONSTERS_CODE: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/monsters/(?P<code>[^/?#]*)$")
                .expect("Unable to create regex for MONSTERS_CODE");
    }
    pub(crate) static ID_MY_BANK_GOLD: usize = 15;
    pub(crate) static ID_MY_BANK_ITEMS: usize = 16;
    pub(crate) static ID_MY_CHANGE_PASSWORD: usize = 17;
    pub(crate) static ID_MY_CHARACTERS: usize = 18;
    pub(crate) static ID_MY_LOGS: usize = 19;
    pub(crate) static ID_MY_NAME_ACTION_BANK_DEPOSIT: usize = 20;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_BANK_DEPOSIT: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/bank/deposit$")
                .expect("Unable to create regex for MY_NAME_ACTION_BANK_DEPOSIT");
    }
    pub(crate) static ID_MY_NAME_ACTION_BANK_DEPOSIT_GOLD: usize = 21;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_BANK_DEPOSIT_GOLD: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/bank/deposit/gold$")
                .expect("Unable to create regex for MY_NAME_ACTION_BANK_DEPOSIT_GOLD");
    }
    pub(crate) static ID_MY_NAME_ACTION_BANK_WITHDRAW: usize = 22;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_BANK_WITHDRAW: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/bank/withdraw$")
                .expect("Unable to create regex for MY_NAME_ACTION_BANK_WITHDRAW");
    }
    pub(crate) static ID_MY_NAME_ACTION_BANK_WITHDRAW_GOLD: usize = 23;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_BANK_WITHDRAW_GOLD: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/bank/withdraw/gold$")
                .expect("Unable to create regex for MY_NAME_ACTION_BANK_WITHDRAW_GOLD");
    }
    pub(crate) static ID_MY_NAME_ACTION_CRAFTING: usize = 24;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_CRAFTING: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/crafting$")
                .expect("Unable to create regex for MY_NAME_ACTION_CRAFTING");
    }
    pub(crate) static ID_MY_NAME_ACTION_DELETE: usize = 25;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_DELETE: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/delete$")
                .expect("Unable to create regex for MY_NAME_ACTION_DELETE");
    }
    pub(crate) static ID_MY_NAME_ACTION_EQUIP: usize = 26;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_EQUIP: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/equip$")
                .expect("Unable to create regex for MY_NAME_ACTION_EQUIP");
    }
    pub(crate) static ID_MY_NAME_ACTION_FIGHT: usize = 27;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_FIGHT: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/fight$")
                .expect("Unable to create regex for MY_NAME_ACTION_FIGHT");
    }
    pub(crate) static ID_MY_NAME_ACTION_GATHERING: usize = 28;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_GATHERING: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/gathering$")
                .expect("Unable to create regex for MY_NAME_ACTION_GATHERING");
    }
    pub(crate) static ID_MY_NAME_ACTION_GE_BUY: usize = 29;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_GE_BUY: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/ge/buy$")
                .expect("Unable to create regex for MY_NAME_ACTION_GE_BUY");
    }
    pub(crate) static ID_MY_NAME_ACTION_GE_SELL: usize = 30;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_GE_SELL: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/ge/sell$")
                .expect("Unable to create regex for MY_NAME_ACTION_GE_SELL");
    }
    pub(crate) static ID_MY_NAME_ACTION_MOVE: usize = 31;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_MOVE: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/move$")
                .expect("Unable to create regex for MY_NAME_ACTION_MOVE");
    }
    pub(crate) static ID_MY_NAME_ACTION_RECYCLING: usize = 32;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_RECYCLING: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/recycling$")
                .expect("Unable to create regex for MY_NAME_ACTION_RECYCLING");
    }
    pub(crate) static ID_MY_NAME_ACTION_TASK_COMPLETE: usize = 33;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_TASK_COMPLETE: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/task/complete$")
                .expect("Unable to create regex for MY_NAME_ACTION_TASK_COMPLETE");
    }
    pub(crate) static ID_MY_NAME_ACTION_TASK_EXCHANGE: usize = 34;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_TASK_EXCHANGE: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/task/exchange$")
                .expect("Unable to create regex for MY_NAME_ACTION_TASK_EXCHANGE");
    }
    pub(crate) static ID_MY_NAME_ACTION_TASK_NEW: usize = 35;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_TASK_NEW: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/task/new$")
                .expect("Unable to create regex for MY_NAME_ACTION_TASK_NEW");
    }
    pub(crate) static ID_MY_NAME_ACTION_UNEQUIP: usize = 36;
    lazy_static! {
        pub static ref REGEX_MY_NAME_ACTION_UNEQUIP: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/my/(?P<name>[^/?#]*)/action/unequip$")
                .expect("Unable to create regex for MY_NAME_ACTION_UNEQUIP");
    }
    pub(crate) static ID_RESOURCES_: usize = 37;
    pub(crate) static ID_RESOURCES_CODE: usize = 38;
    lazy_static! {
        pub static ref REGEX_RESOURCES_CODE: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/resources/(?P<code>[^/?#]*)$")
                .expect("Unable to create regex for RESOURCES_CODE");
    }
    pub(crate) static ID_TOKEN_: usize = 39;
}

pub struct MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData,
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        future::ok(Service::new(self.api_impl.clone()))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Body::empty())
        .expect("Unable to create Method Not Allowed response"))
}

pub struct Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl,
            marker: PhantomData,
        }
    }
}

impl<T, C> Clone for Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker,
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C>
where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future {
        async fn run<T, C>(
            mut api_impl: T,
            req: (Request<Body>, C),
        ) -> Result<Response<Body>, crate::ServiceError>
        where
            T: Api<C> + Clone + Send + 'static,
            C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static,
        {
            let (request, context) = req;
            let (parts, body) = request.into_parts();
            let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
            let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

            match method {
                // CreateAccountAccountsCreatePost - POST /accounts/create
                hyper::Method::POST if path.matched(paths::ID_ACCOUNTS_CREATE) => {
                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_add_account_schema: Option<models::AddAccountSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_add_account_schema) => param_add_account_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter AddAccountSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter AddAccountSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_add_account_schema = match param_add_account_schema {
                                    Some(param_add_account_schema) => param_add_account_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter AddAccountSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter AddAccountSchema")),
                                };

                                let result = api_impl.create_account_accounts_create_post(
                                            param_add_account_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateAccountAccountsCreatePostResponse::AccountCreatedSuccessfully
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_ACCOUNT_ACCOUNTS_CREATE_POST_ACCOUNT_CREATED_SUCCESSFULLY"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateAccountAccountsCreatePostResponse::UsernameAlreadyUsed
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(456).expect("Unable to turn 456 into a StatusCode");
                                                },
                                                CreateAccountAccountsCreatePostResponse::EmailAlreadyUsed
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(457).expect("Unable to turn 457 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter AddAccountSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter AddAccountSchema")),
                        }
                }

                // CreateCharacterCharactersCreatePost - POST /characters/create
                hyper::Method::POST if path.matched(paths::ID_CHARACTERS_CREATE) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_add_character_schema: Option<models::AddCharacterSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_add_character_schema) => param_add_character_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter AddCharacterSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter AddCharacterSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_add_character_schema = match param_add_character_schema {
                                    Some(param_add_character_schema) => param_add_character_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter AddCharacterSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter AddCharacterSchema")),
                                };

                                let result = api_impl.create_character_characters_create_post(
                                            param_add_character_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateCharacterCharactersCreatePostResponse::SuccessfullyCreatedCharacter
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_CHARACTER_CHARACTERS_CREATE_POST_SUCCESSFULLY_CREATED_CHARACTER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateCharacterCharactersCreatePostResponse::NameAlreadyUsed
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(494).expect("Unable to turn 494 into a StatusCode");
                                                },
                                                CreateCharacterCharactersCreatePostResponse::MaximumCharactersReachedOnYourAccount
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(495).expect("Unable to turn 495 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter AddCharacterSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter AddCharacterSchema")),
                        }
                }

                // DeleteCharacterCharactersDeletePost - POST /characters/delete
                hyper::Method::POST if path.matched(paths::ID_CHARACTERS_DELETE) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_delete_character_schema: Option<models::DeleteCharacterSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_delete_character_schema) => param_delete_character_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter DeleteCharacterSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter DeleteCharacterSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_delete_character_schema = match param_delete_character_schema {
                                    Some(param_delete_character_schema) => param_delete_character_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter DeleteCharacterSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter DeleteCharacterSchema")),
                                };

                                let result = api_impl.delete_character_characters_delete_post(
                                            param_delete_character_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteCharacterCharactersDeletePostResponse::SuccessfullyDeletedCharacter
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_CHARACTER_CHARACTERS_DELETE_POST_SUCCESSFULLY_DELETED_CHARACTER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DeleteCharacterCharactersDeletePostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter DeleteCharacterSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter DeleteCharacterSchema")),
                        }
                }

                // GetAllCharactersCharactersGet - GET /characters/
                hyper::Method::GET if path.matched(paths::ID_CHARACTERS_) => {
                    // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                    let query_params =
                        form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes())
                            .collect::<Vec<_>>();
                    let param_sort = query_params
                        .iter()
                        .filter(|e| e.0 == "sort")
                        .map(|e| e.1.clone())
                        .next();
                    let param_sort = match param_sort {
                        Some(param_sort) => {
                            let param_sort = <String as std::str::FromStr>::from_str(&param_sort);
                            match param_sort {
                            Ok(param_sort) => Some(param_sort),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter sort - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter sort")),
                        }
                        }
                        None => None,
                    };
                    let param_page = query_params
                        .iter()
                        .filter(|e| e.0 == "page")
                        .map(|e| e.1.clone())
                        .next();
                    let param_page = match param_page {
                        Some(param_page) => {
                            let param_page = <i32 as std::str::FromStr>::from_str(&param_page);
                            match param_page {
                            Ok(param_page) => Some(param_page),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter page - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter page")),
                        }
                        }
                        None => None,
                    };
                    let param_size = query_params
                        .iter()
                        .filter(|e| e.0 == "size")
                        .map(|e| e.1.clone())
                        .next();
                    let param_size = match param_size {
                        Some(param_size) => {
                            let param_size = <i32 as std::str::FromStr>::from_str(&param_size);
                            match param_size {
                            Ok(param_size) => Some(param_size),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter size - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter size")),
                        }
                        }
                        None => None,
                    };

                    let result = api_impl
                        .get_all_characters_characters_get(
                            param_sort, param_page, param_size, &context,
                        )
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                                            Ok(rsp) => match rsp {
                                                GetAllCharactersCharactersGetResponse::SuccessfullyFetchedCharactersDetails
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_CHARACTERS_CHARACTERS_GET_SUCCESSFULLY_FETCHED_CHARACTERS_DETAILS"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                GetAllCharactersCharactersGetResponse::CharactersNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                    Ok(response)
                }

                // GetCharacterCharactersNameGet - GET /characters/{name}
                hyper::Method::GET if path.matched(paths::ID_CHARACTERS_NAME) => {
                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_CHARACTERS_NAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE CHARACTERS_NAME in set but failed match against \"{}\"", path, paths::REGEX_CHARACTERS_NAME.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    let result = api_impl
                        .get_character_characters_name_get(param_name, &context)
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            GetCharacterCharactersNameGetResponse::SuccessfullyFetchedCharacter(
                                body,
                            ) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CHARACTER_CHARACTERS_NAME_GET_SUCCESSFULLY_FETCHED_CHARACTER"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                            GetCharacterCharactersNameGetResponse::CharacterNotFound => {
                                *response.status_mut() = StatusCode::from_u16(404)
                                    .expect("Unable to turn 404 into a StatusCode");
                            }
                        },
                        Err(e) => {
                            dbg!(e);
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // GetStatusGet - GET /
                hyper::Method::GET if path.matched(paths::ID_) => {
                    let result = api_impl.get_status_get(&context).await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            GetStatusGetResponse::SuccessfulResponse(body) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_STATUS_GET_SUCCESSFUL_RESPONSE"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                        },
                        Err(e) => {
                            dbg!(e);
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // GetAllEventsEventsGet - GET /events/
                hyper::Method::GET if path.matched(paths::ID_EVENTS_) => {
                    // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                    let query_params =
                        form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes())
                            .collect::<Vec<_>>();
                    let param_page = query_params
                        .iter()
                        .filter(|e| e.0 == "page")
                        .map(|e| e.1.clone())
                        .next();
                    let param_page = match param_page {
                        Some(param_page) => {
                            let param_page = <i32 as std::str::FromStr>::from_str(&param_page);
                            match param_page {
                            Ok(param_page) => Some(param_page),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter page - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter page")),
                        }
                        }
                        None => None,
                    };
                    let param_size = query_params
                        .iter()
                        .filter(|e| e.0 == "size")
                        .map(|e| e.1.clone())
                        .next();
                    let param_size = match param_size {
                        Some(param_size) => {
                            let param_size = <i32 as std::str::FromStr>::from_str(&param_size);
                            match param_size {
                            Ok(param_size) => Some(param_size),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter size - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter size")),
                        }
                        }
                        None => None,
                    };

                    let result = api_impl
                        .get_all_events_events_get(param_page, param_size, &context)
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            GetAllEventsEventsGetResponse::SuccessfullyFetchedEventsDetails(
                                body,
                            ) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_EVENTS_EVENTS_GET_SUCCESSFULLY_FETCHED_EVENTS_DETAILS"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                            GetAllEventsEventsGetResponse::EventsNotFound => {
                                *response.status_mut() = StatusCode::from_u16(404)
                                    .expect("Unable to turn 404 into a StatusCode");
                            }
                        },
                        Err(e) => {
                            dbg!(e);
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // GetAllGeItemsGeGet - GET /ge/
                hyper::Method::GET if path.matched(paths::ID_GE_) => {
                    // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                    let query_params =
                        form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes())
                            .collect::<Vec<_>>();
                    let param_page = query_params
                        .iter()
                        .filter(|e| e.0 == "page")
                        .map(|e| e.1.clone())
                        .next();
                    let param_page = match param_page {
                        Some(param_page) => {
                            let param_page = <i32 as std::str::FromStr>::from_str(&param_page);
                            match param_page {
                            Ok(param_page) => Some(param_page),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter page - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter page")),
                        }
                        }
                        None => None,
                    };
                    let param_size = query_params
                        .iter()
                        .filter(|e| e.0 == "size")
                        .map(|e| e.1.clone())
                        .next();
                    let param_size = match param_size {
                        Some(param_size) => {
                            let param_size = <i32 as std::str::FromStr>::from_str(&param_size);
                            match param_size {
                            Ok(param_size) => Some(param_size),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter size - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter size")),
                        }
                        }
                        None => None,
                    };

                    let result = api_impl
                        .get_all_ge_items_ge_get(param_page, param_size, &context)
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            GetAllGeItemsGeGetResponse::FetchGrandExchangeItemsDetails(body) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_GE_ITEMS_GE_GET_FETCH_GRAND_EXCHANGE_ITEMS_DETAILS"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                            GetAllGeItemsGeGetResponse::ItemNotFound => {
                                *response.status_mut() = StatusCode::from_u16(404)
                                    .expect("Unable to turn 404 into a StatusCode");
                            }
                        },
                        Err(e) => {
                            dbg!(e);
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // GetGeItemGeCodeGet - GET /ge/{code}
                hyper::Method::GET if path.matched(paths::ID_GE_CODE) => {
                    // Path parameters
                    let path: &str = uri.path();
                    let path_params = paths::REGEX_GE_CODE.captures(path).unwrap_or_else(|| {
                        panic!(
                            "Path {} matched RE GE_CODE in set but failed match against \"{}\"",
                            path,
                            paths::REGEX_GE_CODE.as_str()
                        )
                    });

                    let param_code = match percent_encoding::percent_decode(path_params["code"].as_bytes()).decode_utf8() {
                    Ok(param_code) => match param_code.parse::<String>() {
                        Ok(param_code) => param_code,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter code: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["code"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    let result = api_impl.get_ge_item_ge_code_get(param_code, &context).await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            GetGeItemGeCodeGetResponse::SuccessfullyFetchedGrandExchangeItem(
                                body,
                            ) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_GE_ITEM_GE_CODE_GET_SUCCESSFULLY_FETCHED_GRAND_EXCHANGE_ITEM"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                            GetGeItemGeCodeGetResponse::ItemNotFound => {
                                *response.status_mut() = StatusCode::from_u16(404)
                                    .expect("Unable to turn 404 into a StatusCode");
                            }
                        },
                        Err(e) => {
                            dbg!(e);
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // GetAllItemsItemsGet - GET /items/
                hyper::Method::GET if path.matched(paths::ID_ITEMS_) => {
                    // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                    let query_params =
                        form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes())
                            .collect::<Vec<_>>();
                    let param_min_level = query_params
                        .iter()
                        .filter(|e| e.0 == "min_level")
                        .map(|e| e.1.clone())
                        .next();
                    let param_min_level = match param_min_level {
                        Some(param_min_level) => {
                            let param_min_level =
                                <i32 as std::str::FromStr>::from_str(&param_min_level);
                            match param_min_level {
                            Ok(param_min_level) => Some(param_min_level),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter min_level - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter min_level")),
                        }
                        }
                        None => None,
                    };
                    let param_max_level = query_params
                        .iter()
                        .filter(|e| e.0 == "max_level")
                        .map(|e| e.1.clone())
                        .next();
                    let param_max_level = match param_max_level {
                        Some(param_max_level) => {
                            let param_max_level =
                                <i32 as std::str::FromStr>::from_str(&param_max_level);
                            match param_max_level {
                            Ok(param_max_level) => Some(param_max_level),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter max_level - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter max_level")),
                        }
                        }
                        None => None,
                    };
                    let param_name = query_params
                        .iter()
                        .filter(|e| e.0 == "name")
                        .map(|e| e.1.clone())
                        .next();
                    let param_name = match param_name {
                        Some(param_name) => {
                            let param_name = <String as std::str::FromStr>::from_str(&param_name);
                            match param_name {
                            Ok(param_name) => Some(param_name),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter name - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter name")),
                        }
                        }
                        None => None,
                    };
                    let param_type = query_params
                        .iter()
                        .filter(|e| e.0 == "type")
                        .map(|e| e.1.clone())
                        .next();
                    let param_type = match param_type {
                        Some(param_type) => {
                            let param_type = <String as std::str::FromStr>::from_str(&param_type);
                            match param_type {
                            Ok(param_type) => Some(param_type),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter type - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter type")),
                        }
                        }
                        None => None,
                    };
                    let param_craft_skill = query_params
                        .iter()
                        .filter(|e| e.0 == "craft_skill")
                        .map(|e| e.1.clone())
                        .next();
                    let param_craft_skill = match param_craft_skill {
                        Some(param_craft_skill) => {
                            let param_craft_skill =
                                <String as std::str::FromStr>::from_str(&param_craft_skill);
                            match param_craft_skill {
                            Ok(param_craft_skill) => Some(param_craft_skill),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter craft_skill - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter craft_skill")),
                        }
                        }
                        None => None,
                    };
                    let param_craft_material = query_params
                        .iter()
                        .filter(|e| e.0 == "craft_material")
                        .map(|e| e.1.clone())
                        .next();
                    let param_craft_material = match param_craft_material {
                        Some(param_craft_material) => {
                            let param_craft_material =
                                <String as std::str::FromStr>::from_str(&param_craft_material);
                            match param_craft_material {
                            Ok(param_craft_material) => Some(param_craft_material),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter craft_material - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter craft_material")),
                        }
                        }
                        None => None,
                    };
                    let param_page = query_params
                        .iter()
                        .filter(|e| e.0 == "page")
                        .map(|e| e.1.clone())
                        .next();
                    let param_page = match param_page {
                        Some(param_page) => {
                            let param_page = <i32 as std::str::FromStr>::from_str(&param_page);
                            match param_page {
                            Ok(param_page) => Some(param_page),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter page - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter page")),
                        }
                        }
                        None => None,
                    };
                    let param_size = query_params
                        .iter()
                        .filter(|e| e.0 == "size")
                        .map(|e| e.1.clone())
                        .next();
                    let param_size = match param_size {
                        Some(param_size) => {
                            let param_size = <i32 as std::str::FromStr>::from_str(&param_size);
                            match param_size {
                            Ok(param_size) => Some(param_size),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter size - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter size")),
                        }
                        }
                        None => None,
                    };

                    let result = api_impl
                        .get_all_items_items_get(
                            param_min_level,
                            param_max_level,
                            param_name,
                            param_type,
                            param_craft_skill,
                            param_craft_material,
                            param_page,
                            param_size,
                            &context,
                        )
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            GetAllItemsItemsGetResponse::FetchItemsDetails(body) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_ITEMS_ITEMS_GET_FETCH_ITEMS_DETAILS"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                            GetAllItemsItemsGetResponse::ItemsNotFound => {
                                *response.status_mut() = StatusCode::from_u16(404)
                                    .expect("Unable to turn 404 into a StatusCode");
                            }
                        },
                        Err(e) => {
                            dbg!(e);
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // GetItemItemsCodeGet - GET /items/{code}
                hyper::Method::GET if path.matched(paths::ID_ITEMS_CODE) => {
                    // Path parameters
                    let path: &str = uri.path();
                    let path_params = paths::REGEX_ITEMS_CODE.captures(path).unwrap_or_else(|| {
                        panic!(
                            "Path {} matched RE ITEMS_CODE in set but failed match against \"{}\"",
                            path,
                            paths::REGEX_ITEMS_CODE.as_str()
                        )
                    });

                    let param_code = match percent_encoding::percent_decode(path_params["code"].as_bytes()).decode_utf8() {
                    Ok(param_code) => match param_code.parse::<String>() {
                        Ok(param_code) => param_code,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter code: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["code"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    let result = api_impl.get_item_items_code_get(param_code, &context).await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            GetItemItemsCodeGetResponse::SuccessfullyFetchedItem(body) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ITEM_ITEMS_CODE_GET_SUCCESSFULLY_FETCHED_ITEM"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                            GetItemItemsCodeGetResponse::ItemNotFound => {
                                *response.status_mut() = StatusCode::from_u16(404)
                                    .expect("Unable to turn 404 into a StatusCode");
                            }
                        },
                        Err(e) => {
                            dbg!(e);
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // GetAllMapsMapsGet - GET /maps/
                hyper::Method::GET if path.matched(paths::ID_MAPS_) => {
                    // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                    let query_params =
                        form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes())
                            .collect::<Vec<_>>();
                    let param_content_type = query_params
                        .iter()
                        .filter(|e| e.0 == "content_type")
                        .map(|e| e.1.clone())
                        .next();
                    let param_content_type = match param_content_type {
                        Some(param_content_type) => {
                            let param_content_type =
                                <String as std::str::FromStr>::from_str(&param_content_type);
                            match param_content_type {
                            Ok(param_content_type) => Some(param_content_type),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter content_type - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter content_type")),
                        }
                        }
                        None => None,
                    };
                    let param_content_code = query_params
                        .iter()
                        .filter(|e| e.0 == "content_code")
                        .map(|e| e.1.clone())
                        .next();
                    let param_content_code = match param_content_code {
                        Some(param_content_code) => {
                            let param_content_code =
                                <String as std::str::FromStr>::from_str(&param_content_code);
                            match param_content_code {
                            Ok(param_content_code) => Some(param_content_code),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter content_code - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter content_code")),
                        }
                        }
                        None => None,
                    };
                    let param_page = query_params
                        .iter()
                        .filter(|e| e.0 == "page")
                        .map(|e| e.1.clone())
                        .next();
                    let param_page = match param_page {
                        Some(param_page) => {
                            let param_page = <i32 as std::str::FromStr>::from_str(&param_page);
                            match param_page {
                            Ok(param_page) => Some(param_page),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter page - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter page")),
                        }
                        }
                        None => None,
                    };
                    let param_size = query_params
                        .iter()
                        .filter(|e| e.0 == "size")
                        .map(|e| e.1.clone())
                        .next();
                    let param_size = match param_size {
                        Some(param_size) => {
                            let param_size = <i32 as std::str::FromStr>::from_str(&param_size);
                            match param_size {
                            Ok(param_size) => Some(param_size),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter size - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter size")),
                        }
                        }
                        None => None,
                    };

                    let result = api_impl
                        .get_all_maps_maps_get(
                            param_content_type,
                            param_content_code,
                            param_page,
                            param_size,
                            &context,
                        )
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            GetAllMapsMapsGetResponse::SuccessfullyFetchedMapsDetails(body) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_MAPS_MAPS_GET_SUCCESSFULLY_FETCHED_MAPS_DETAILS"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                            GetAllMapsMapsGetResponse::MapsNotFound => {
                                *response.status_mut() = StatusCode::from_u16(404)
                                    .expect("Unable to turn 404 into a StatusCode");
                            }
                        },
                        Err(e) => {
                            dbg!(e);
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // GetMapMapsXyGet - GET /maps/{x}/{y}
                hyper::Method::GET if path.matched(paths::ID_MAPS_X_Y) => {
                    // Path parameters
                    let path: &str = uri.path();
                    let path_params = paths::REGEX_MAPS_X_Y.captures(path).unwrap_or_else(|| {
                        panic!(
                            "Path {} matched RE MAPS_X_Y in set but failed match against \"{}\"",
                            path,
                            paths::REGEX_MAPS_X_Y.as_str()
                        )
                    });

                    let param_x = match percent_encoding::percent_decode(path_params["x"].as_bytes()).decode_utf8() {
                    Ok(param_x) => match param_x.parse::<i32>() {
                        Ok(param_x) => param_x,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter x: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["x"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    let param_y = match percent_encoding::percent_decode(path_params["y"].as_bytes()).decode_utf8() {
                    Ok(param_y) => match param_y.parse::<i32>() {
                        Ok(param_y) => param_y,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter y: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["y"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    let result = api_impl
                        .get_map_maps_xy_get(param_x, param_y, &context)
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            GetMapMapsXyGetResponse::SuccessfullyFetchedMap(body) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_MAP_MAPS_XY_GET_SUCCESSFULLY_FETCHED_MAP"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                            GetMapMapsXyGetResponse::MapNotFound => {
                                *response.status_mut() = StatusCode::from_u16(404)
                                    .expect("Unable to turn 404 into a StatusCode");
                            }
                        },
                        Err(e) => {
                            dbg!(e);
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // GetAllMonstersMonstersGet - GET /monsters/
                hyper::Method::GET if path.matched(paths::ID_MONSTERS_) => {
                    // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                    let query_params =
                        form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes())
                            .collect::<Vec<_>>();
                    let param_min_level = query_params
                        .iter()
                        .filter(|e| e.0 == "min_level")
                        .map(|e| e.1.clone())
                        .next();
                    let param_min_level = match param_min_level {
                        Some(param_min_level) => {
                            let param_min_level =
                                <i32 as std::str::FromStr>::from_str(&param_min_level);
                            match param_min_level {
                            Ok(param_min_level) => Some(param_min_level),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter min_level - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter min_level")),
                        }
                        }
                        None => None,
                    };
                    let param_max_level = query_params
                        .iter()
                        .filter(|e| e.0 == "max_level")
                        .map(|e| e.1.clone())
                        .next();
                    let param_max_level = match param_max_level {
                        Some(param_max_level) => {
                            let param_max_level =
                                <i32 as std::str::FromStr>::from_str(&param_max_level);
                            match param_max_level {
                            Ok(param_max_level) => Some(param_max_level),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter max_level - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter max_level")),
                        }
                        }
                        None => None,
                    };
                    let param_drop = query_params
                        .iter()
                        .filter(|e| e.0 == "drop")
                        .map(|e| e.1.clone())
                        .next();
                    let param_drop = match param_drop {
                        Some(param_drop) => {
                            let param_drop = <String as std::str::FromStr>::from_str(&param_drop);
                            match param_drop {
                            Ok(param_drop) => Some(param_drop),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter drop - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter drop")),
                        }
                        }
                        None => None,
                    };
                    let param_page = query_params
                        .iter()
                        .filter(|e| e.0 == "page")
                        .map(|e| e.1.clone())
                        .next();
                    let param_page = match param_page {
                        Some(param_page) => {
                            let param_page = <i32 as std::str::FromStr>::from_str(&param_page);
                            match param_page {
                            Ok(param_page) => Some(param_page),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter page - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter page")),
                        }
                        }
                        None => None,
                    };
                    let param_size = query_params
                        .iter()
                        .filter(|e| e.0 == "size")
                        .map(|e| e.1.clone())
                        .next();
                    let param_size = match param_size {
                        Some(param_size) => {
                            let param_size = <i32 as std::str::FromStr>::from_str(&param_size);
                            match param_size {
                            Ok(param_size) => Some(param_size),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter size - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter size")),
                        }
                        }
                        None => None,
                    };

                    let result = api_impl
                        .get_all_monsters_monsters_get(
                            param_min_level,
                            param_max_level,
                            param_drop,
                            param_page,
                            param_size,
                            &context,
                        )
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                                            Ok(rsp) => match rsp {
                                                GetAllMonstersMonstersGetResponse::SuccessfullyFetchedMonstersDetails
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_MONSTERS_MONSTERS_GET_SUCCESSFULLY_FETCHED_MONSTERS_DETAILS"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                GetAllMonstersMonstersGetResponse::MonstersNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                    Ok(response)
                }

                // GetMonsterMonstersCodeGet - GET /monsters/{code}
                hyper::Method::GET if path.matched(paths::ID_MONSTERS_CODE) => {
                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MONSTERS_CODE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MONSTERS_CODE in set but failed match against \"{}\"", path, paths::REGEX_MONSTERS_CODE.as_str())
                    );

                    let param_code = match percent_encoding::percent_decode(path_params["code"].as_bytes()).decode_utf8() {
                    Ok(param_code) => match param_code.parse::<String>() {
                        Ok(param_code) => param_code,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter code: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["code"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    let result = api_impl
                        .get_monster_monsters_code_get(param_code, &context)
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            GetMonsterMonstersCodeGetResponse::SuccessfullyFetchedMonster(body) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_MONSTER_MONSTERS_CODE_GET_SUCCESSFULLY_FETCHED_MONSTER"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                            GetMonsterMonstersCodeGetResponse::MonsterNotFound => {
                                *response.status_mut() = StatusCode::from_u16(404)
                                    .expect("Unable to turn 404 into a StatusCode");
                            }
                        },
                        Err(e) => {
                            dbg!(e);
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // ChangePasswordMyChangePasswordPost - POST /my/change_password
                hyper::Method::POST if path.matched(paths::ID_MY_CHANGE_PASSWORD) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_change_password: Option<models::ChangePassword> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_change_password) => param_change_password,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter ChangePassword - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter ChangePassword due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_change_password = match param_change_password {
                                    Some(param_change_password) => param_change_password,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter ChangePassword"))
                                                        .expect("Unable to create Bad Request response for missing body parameter ChangePassword")),
                                };

                                let result = api_impl.change_password_my_change_password_post(
                                            param_change_password,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ChangePasswordMyChangePasswordPostResponse::PasswordChangedSuccessfully
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CHANGE_PASSWORD_MY_CHANGE_PASSWORD_POST_PASSWORD_CHANGED_SUCCESSFULLY"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ChangePasswordMyChangePasswordPostResponse::UseADifferentPassword
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(458).expect("Unable to turn 458 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter ChangePassword: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter ChangePassword")),
                        }
                }

                // GetBankGoldsMyBankGoldGet - GET /my/bank/gold
                hyper::Method::GET if path.matched(paths::ID_MY_BANK_GOLD) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    let result = api_impl.get_bank_golds_my_bank_gold_get(&context).await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            GetBankGoldsMyBankGoldGetResponse::SuccessfullyFetchedGolds(body) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_BANK_GOLDS_MY_BANK_GOLD_GET_SUCCESSFULLY_FETCHED_GOLDS"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                        },
                        Err(e) => {
                            dbg!(e);
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // GetBankItemsMyBankItemsGet - GET /my/bank/items
                hyper::Method::GET if path.matched(paths::ID_MY_BANK_ITEMS) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                    let query_params =
                        form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes())
                            .collect::<Vec<_>>();
                    let param_item_code = query_params
                        .iter()
                        .filter(|e| e.0 == "item_code")
                        .map(|e| e.1.clone())
                        .next();
                    let param_item_code = match param_item_code {
                        Some(param_item_code) => {
                            let param_item_code =
                                <String as std::str::FromStr>::from_str(&param_item_code);
                            match param_item_code {
                            Ok(param_item_code) => Some(param_item_code),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter item_code - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter item_code")),
                        }
                        }
                        None => None,
                    };
                    let param_page = query_params
                        .iter()
                        .filter(|e| e.0 == "page")
                        .map(|e| e.1.clone())
                        .next();
                    let param_page = match param_page {
                        Some(param_page) => {
                            let param_page = <i32 as std::str::FromStr>::from_str(&param_page);
                            match param_page {
                            Ok(param_page) => Some(param_page),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter page - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter page")),
                        }
                        }
                        None => None,
                    };
                    let param_size = query_params
                        .iter()
                        .filter(|e| e.0 == "size")
                        .map(|e| e.1.clone())
                        .next();
                    let param_size = match param_size {
                        Some(param_size) => {
                            let param_size = <i32 as std::str::FromStr>::from_str(&param_size);
                            match param_size {
                            Ok(param_size) => Some(param_size),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter size - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter size")),
                        }
                        }
                        None => None,
                    };

                    let result = api_impl
                        .get_bank_items_my_bank_items_get(
                            param_item_code,
                            param_page,
                            param_size,
                            &context,
                        )
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            GetBankItemsMyBankItemsGetResponse::SuccessfullyFetchedItems(body) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_BANK_ITEMS_MY_BANK_ITEMS_GET_SUCCESSFULLY_FETCHED_ITEMS"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                            GetBankItemsMyBankItemsGetResponse::ItemsNotFound => {
                                *response.status_mut() = StatusCode::from_u16(404)
                                    .expect("Unable to turn 404 into a StatusCode");
                            }
                        },
                        Err(e) => {
                            dbg!(e);
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // ActionAcceptNewTaskMyNameActionTaskNewPost - POST /my/{name}/action/task/new
                hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_TASK_NEW) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_TASK_NEW
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_TASK_NEW in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_TASK_NEW.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    let result = api_impl
                        .action_accept_new_task_my_name_action_task_new_post(param_name, &context)
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                                            Ok(rsp) => match rsp {
                                                ActionAcceptNewTaskMyNameActionTaskNewPostResponse::NewTaskSuccessfullyAccepted
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_ACCEPT_NEW_TASK_MY_NAME_ACTION_TASK_NEW_POST_NEW_TASK_SUCCESSFULLY_ACCEPTED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionAcceptNewTaskMyNameActionTaskNewPostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionAcceptNewTaskMyNameActionTaskNewPostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionAcceptNewTaskMyNameActionTaskNewPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                                ActionAcceptNewTaskMyNameActionTaskNewPostResponse::TasksMasterNotFoundOnThisMap
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(598).expect("Unable to turn 598 into a StatusCode");
                                                },
                                                ActionAcceptNewTaskMyNameActionTaskNewPostResponse::CharacterAlreadyHasATask
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(489).expect("Unable to turn 489 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                    Ok(response)
                }

                // ActionCompleteTaskMyNameActionTaskCompletePost - POST /my/{name}/action/task/complete
                hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_TASK_COMPLETE) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_TASK_COMPLETE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_TASK_COMPLETE in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_TASK_COMPLETE.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    let result = api_impl
                        .action_complete_task_my_name_action_task_complete_post(
                            param_name, &context,
                        )
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                                            Ok(rsp) => match rsp {
                                                ActionCompleteTaskMyNameActionTaskCompletePostResponse::TheTaskHasBeenSuccessfullyCompleted
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_COMPLETE_TASK_MY_NAME_ACTION_TASK_COMPLETE_POST_THE_TASK_HAS_BEEN_SUCCESSFULLY_COMPLETED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionCompleteTaskMyNameActionTaskCompletePostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionCompleteTaskMyNameActionTaskCompletePostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionCompleteTaskMyNameActionTaskCompletePostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                                ActionCompleteTaskMyNameActionTaskCompletePostResponse::TasksMasterNotFoundOnThisMap
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(598).expect("Unable to turn 598 into a StatusCode");
                                                },
                                                ActionCompleteTaskMyNameActionTaskCompletePostResponse::CharacterHasNotCompletedTheTask
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(488).expect("Unable to turn 488 into a StatusCode");
                                                },
                                                ActionCompleteTaskMyNameActionTaskCompletePostResponse::CharacterHasNoTask
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(487).expect("Unable to turn 487 into a StatusCode");
                                                },
                                                ActionCompleteTaskMyNameActionTaskCompletePostResponse::CharacterInventoryIsFull
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(497).expect("Unable to turn 497 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                    Ok(response)
                }

                // ActionCraftingMyNameActionCraftingPost - POST /my/{name}/action/crafting
                hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_CRAFTING) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_CRAFTING
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_CRAFTING in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_CRAFTING.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_crafting_schema: Option<models::CraftingSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_crafting_schema) => param_crafting_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter CraftingSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter CraftingSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_crafting_schema = match param_crafting_schema {
                                    Some(param_crafting_schema) => param_crafting_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter CraftingSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter CraftingSchema")),
                                };

                                let result = api_impl.action_crafting_my_name_action_crafting_post(
                                            param_name,
                                            param_crafting_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ActionCraftingMyNameActionCraftingPostResponse::TheItemWasSuccessfullyCrafted
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_CRAFTING_MY_NAME_ACTION_CRAFTING_POST_THE_ITEM_WAS_SUCCESSFULLY_CRAFTED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionCraftingMyNameActionCraftingPostResponse::CraftNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                ActionCraftingMyNameActionCraftingPostResponse::WorkshopNotFoundOnThisMap
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(598).expect("Unable to turn 598 into a StatusCode");
                                                },
                                                ActionCraftingMyNameActionCraftingPostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionCraftingMyNameActionCraftingPostResponse::CharacterInventoryIsFull
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(497).expect("Unable to turn 497 into a StatusCode");
                                                },
                                                ActionCraftingMyNameActionCraftingPostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionCraftingMyNameActionCraftingPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                                ActionCraftingMyNameActionCraftingPostResponse::NotSkillLevelRequired
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(493).expect("Unable to turn 493 into a StatusCode");
                                                },
                                                ActionCraftingMyNameActionCraftingPostResponse::MissingItemOrInsufficientQuantityInYourInventory
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(478).expect("Unable to turn 478 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter CraftingSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter CraftingSchema")),
                        }
                }

                // ActionDeleteItemMyNameActionDeletePost - POST /my/{name}/action/delete
                hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_DELETE) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_DELETE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_DELETE in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_DELETE.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_simple_item_schema: Option<models::SimpleItemSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_simple_item_schema) => param_simple_item_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter SimpleItemSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter SimpleItemSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_simple_item_schema = match param_simple_item_schema {
                                    Some(param_simple_item_schema) => param_simple_item_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter SimpleItemSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter SimpleItemSchema")),
                                };

                                let result = api_impl.action_delete_item_my_name_action_delete_post(
                                            param_name,
                                            param_simple_item_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ActionDeleteItemMyNameActionDeletePostResponse::ItemSuccessfullyDeletedFromYourCharacter
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_DELETE_ITEM_MY_NAME_ACTION_DELETE_POST_ITEM_SUCCESSFULLY_DELETED_FROM_YOUR_CHARACTER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionDeleteItemMyNameActionDeletePostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionDeleteItemMyNameActionDeletePostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionDeleteItemMyNameActionDeletePostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                                ActionDeleteItemMyNameActionDeletePostResponse::MissingItemOrInsufficientQuantityInYourInventory
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(478).expect("Unable to turn 478 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter SimpleItemSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter SimpleItemSchema")),
                        }
                }

                // ActionDepositBankGoldMyNameActionBankDepositGoldPost - POST /my/{name}/action/bank/deposit/gold
                hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_BANK_DEPOSIT_GOLD) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_BANK_DEPOSIT_GOLD
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_BANK_DEPOSIT_GOLD in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_BANK_DEPOSIT_GOLD.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_deposit_withdraw_gold_schema: Option<models::DepositWithdrawGoldSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_deposit_withdraw_gold_schema) => param_deposit_withdraw_gold_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter DepositWithdrawGoldSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter DepositWithdrawGoldSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_deposit_withdraw_gold_schema = match param_deposit_withdraw_gold_schema {
                                    Some(param_deposit_withdraw_gold_schema) => param_deposit_withdraw_gold_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter DepositWithdrawGoldSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter DepositWithdrawGoldSchema")),
                                };

                                let result = api_impl.action_deposit_bank_gold_my_name_action_bank_deposit_gold_post(
                                            param_name,
                                            param_deposit_withdraw_gold_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse::GoldsSuccessfullyDepositedInYourBank
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_DEPOSIT_BANK_GOLD_MY_NAME_ACTION_BANK_DEPOSIT_GOLD_POST_GOLDS_SUCCESSFULLY_DEPOSITED_IN_YOUR_BANK"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse::BankNotFoundOnThisMap
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(598).expect("Unable to turn 598 into a StatusCode");
                                                },
                                                ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse::InsufficientGoldsOnYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(492).expect("Unable to turn 492 into a StatusCode");
                                                },
                                                ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse::ATransactionIsAlreadyInProgressWithThisItem
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(461).expect("Unable to turn 461 into a StatusCode");
                                                },
                                                ActionDepositBankGoldMyNameActionBankDepositGoldPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter DepositWithdrawGoldSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter DepositWithdrawGoldSchema")),
                        }
                }

                // ActionDepositBankMyNameActionBankDepositPost - POST /my/{name}/action/bank/deposit
                hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_BANK_DEPOSIT) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_BANK_DEPOSIT
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_BANK_DEPOSIT in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_BANK_DEPOSIT.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_simple_item_schema: Option<models::SimpleItemSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_simple_item_schema) => param_simple_item_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter SimpleItemSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter SimpleItemSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_simple_item_schema = match param_simple_item_schema {
                                    Some(param_simple_item_schema) => param_simple_item_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter SimpleItemSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter SimpleItemSchema")),
                                };

                                let result = api_impl.action_deposit_bank_my_name_action_bank_deposit_post(
                                            param_name,
                                            param_simple_item_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ActionDepositBankMyNameActionBankDepositPostResponse::ItemSuccessfullyDepositedInYourBank
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_DEPOSIT_BANK_MY_NAME_ACTION_BANK_DEPOSIT_POST_ITEM_SUCCESSFULLY_DEPOSITED_IN_YOUR_BANK"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionDepositBankMyNameActionBankDepositPostResponse::BankNotFoundOnThisMap
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(598).expect("Unable to turn 598 into a StatusCode");
                                                },
                                                ActionDepositBankMyNameActionBankDepositPostResponse::ItemNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                ActionDepositBankMyNameActionBankDepositPostResponse::ATransactionIsAlreadyInProgressWithThisItem
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(461).expect("Unable to turn 461 into a StatusCode");
                                                },
                                                ActionDepositBankMyNameActionBankDepositPostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionDepositBankMyNameActionBankDepositPostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionDepositBankMyNameActionBankDepositPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                                ActionDepositBankMyNameActionBankDepositPostResponse::MissingItemOrInsufficientQuantityInYourInventory
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(478).expect("Unable to turn 478 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter SimpleItemSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter SimpleItemSchema")),
                        }
                }

                // ActionEquipItemMyNameActionEquipPost - POST /my/{name}/action/equip
                hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_EQUIP) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_EQUIP
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_EQUIP in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_EQUIP.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_equip_schema: Option<models::EquipSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_equip_schema) => param_equip_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter EquipSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter EquipSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_equip_schema = match param_equip_schema {
                                    Some(param_equip_schema) => param_equip_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter EquipSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter EquipSchema")),
                                };

                                let result = api_impl.action_equip_item_my_name_action_equip_post(
                                            param_name,
                                            param_equip_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ActionEquipItemMyNameActionEquipPostResponse::TheItemHasBeenSuccessfullyEquippedOnYourCharacter
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_EQUIP_ITEM_MY_NAME_ACTION_EQUIP_POST_THE_ITEM_HAS_BEEN_SUCCESSFULLY_EQUIPPED_ON_YOUR_CHARACTER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionEquipItemMyNameActionEquipPostResponse::ItemNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                ActionEquipItemMyNameActionEquipPostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionEquipItemMyNameActionEquipPostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionEquipItemMyNameActionEquipPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                                ActionEquipItemMyNameActionEquipPostResponse::MissingItemOrInsufficientQuantityInYourInventory
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(478).expect("Unable to turn 478 into a StatusCode");
                                                },
                                                ActionEquipItemMyNameActionEquipPostResponse::CharacterLevelIsInsufficient
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(496).expect("Unable to turn 496 into a StatusCode");
                                                },
                                                ActionEquipItemMyNameActionEquipPostResponse::SlotIsNotEmpty
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(491).expect("Unable to turn 491 into a StatusCode");
                                                },
                                                ActionEquipItemMyNameActionEquipPostResponse::ThisItemIsAlreadyEquipped
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(485).expect("Unable to turn 485 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter EquipSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter EquipSchema")),
                        }
                }

                // ActionFightMyNameActionFightPost - POST /my/{name}/action/fight
                hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_FIGHT) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_FIGHT
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_FIGHT in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_FIGHT.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    let result = api_impl
                        .action_fight_my_name_action_fight_post(param_name, &context)
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                                            Ok(rsp) => match rsp {
                                                ActionFightMyNameActionFightPostResponse::TheFightEndedSuccessfully
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_FIGHT_MY_NAME_ACTION_FIGHT_POST_THE_FIGHT_ENDED_SUCCESSFULLY"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionFightMyNameActionFightPostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionFightMyNameActionFightPostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionFightMyNameActionFightPostResponse::MonsterNotFoundOnThisMap
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(598).expect("Unable to turn 598 into a StatusCode");
                                                },
                                                ActionFightMyNameActionFightPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                                ActionFightMyNameActionFightPostResponse::CharacterInventoryIsFull
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(497).expect("Unable to turn 497 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                    Ok(response)
                }

                // ActionGatheringMyNameActionGatheringPost - POST /my/{name}/action/gathering
                hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_GATHERING) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_GATHERING
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_GATHERING in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_GATHERING.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    let result = api_impl
                        .action_gathering_my_name_action_gathering_post(param_name, &context)
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                                            Ok(rsp) => match rsp {
                                                ActionGatheringMyNameActionGatheringPostResponse::TheResourceHasBeenSuccessfullyGathered
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_GATHERING_MY_NAME_ACTION_GATHERING_POST_THE_RESOURCE_HAS_BEEN_SUCCESSFULLY_GATHERED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionGatheringMyNameActionGatheringPostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionGatheringMyNameActionGatheringPostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionGatheringMyNameActionGatheringPostResponse::ResourceNotFoundOnThisMap
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(598).expect("Unable to turn 598 into a StatusCode");
                                                },
                                                ActionGatheringMyNameActionGatheringPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                                ActionGatheringMyNameActionGatheringPostResponse::NotSkillLevelRequired
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(493).expect("Unable to turn 493 into a StatusCode");
                                                },
                                                ActionGatheringMyNameActionGatheringPostResponse::CharacterInventoryIsFull
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(497).expect("Unable to turn 497 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                    Ok(response)
                }

                // ActionGeBuyItemMyNameActionGeBuyPost - POST /my/{name}/action/ge/buy
                hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_GE_BUY) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_GE_BUY
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_GE_BUY in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_GE_BUY.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_ge_transaction_item_schema: Option<models::GeTransactionItemSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_ge_transaction_item_schema) => param_ge_transaction_item_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter GeTransactionItemSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter GeTransactionItemSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_ge_transaction_item_schema = match param_ge_transaction_item_schema {
                                    Some(param_ge_transaction_item_schema) => param_ge_transaction_item_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter GeTransactionItemSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter GeTransactionItemSchema")),
                                };

                                let result = api_impl.action_ge_buy_item_my_name_action_ge_buy_post(
                                            param_name,
                                            param_ge_transaction_item_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ActionGeBuyItemMyNameActionGeBuyPostResponse::ItemSuccessfullyBuyFromTheGrandExchange
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_GE_BUY_ITEM_MY_NAME_ACTION_GE_BUY_POST_ITEM_SUCCESSFULLY_BUY_FROM_THE_GRAND_EXCHANGE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionGeBuyItemMyNameActionGeBuyPostResponse::GrandExchangeNotFoundOnThisMap
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(598).expect("Unable to turn 598 into a StatusCode");
                                                },
                                                ActionGeBuyItemMyNameActionGeBuyPostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionGeBuyItemMyNameActionGeBuyPostResponse::CharacterInventoryIsFull
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(497).expect("Unable to turn 497 into a StatusCode");
                                                },
                                                ActionGeBuyItemMyNameActionGeBuyPostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionGeBuyItemMyNameActionGeBuyPostResponse::ATransactionIsAlreadyInProgressOnThisItemByAAnotherCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(483).expect("Unable to turn 483 into a StatusCode");
                                                },
                                                ActionGeBuyItemMyNameActionGeBuyPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                                ActionGeBuyItemMyNameActionGeBuyPostResponse::InsufficientGoldsOnYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(492).expect("Unable to turn 492 into a StatusCode");
                                                },
                                                ActionGeBuyItemMyNameActionGeBuyPostResponse::NoStockForThisItem
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(480).expect("Unable to turn 480 into a StatusCode");
                                                },
                                                ActionGeBuyItemMyNameActionGeBuyPostResponse::NoItemAtThisPrice
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(482).expect("Unable to turn 482 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter GeTransactionItemSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter GeTransactionItemSchema")),
                        }
                }

                // ActionGeSellItemMyNameActionGeSellPost - POST /my/{name}/action/ge/sell
                hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_GE_SELL) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_GE_SELL
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_GE_SELL in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_GE_SELL.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_ge_transaction_item_schema: Option<models::GeTransactionItemSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_ge_transaction_item_schema) => param_ge_transaction_item_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter GeTransactionItemSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter GeTransactionItemSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_ge_transaction_item_schema = match param_ge_transaction_item_schema {
                                    Some(param_ge_transaction_item_schema) => param_ge_transaction_item_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter GeTransactionItemSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter GeTransactionItemSchema")),
                                };

                                let result = api_impl.action_ge_sell_item_my_name_action_ge_sell_post(
                                            param_name,
                                            param_ge_transaction_item_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ActionGeSellItemMyNameActionGeSellPostResponse::ItemSuccessfullySellAtTheGrandExchange
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_GE_SELL_ITEM_MY_NAME_ACTION_GE_SELL_POST_ITEM_SUCCESSFULLY_SELL_AT_THE_GRAND_EXCHANGE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionGeSellItemMyNameActionGeSellPostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionGeSellItemMyNameActionGeSellPostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionGeSellItemMyNameActionGeSellPostResponse::ItemNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                ActionGeSellItemMyNameActionGeSellPostResponse::ATransactionIsAlreadyInProgressOnThisItemByAAnotherCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(483).expect("Unable to turn 483 into a StatusCode");
                                                },
                                                ActionGeSellItemMyNameActionGeSellPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                                ActionGeSellItemMyNameActionGeSellPostResponse::MissingItemOrInsufficientQuantityInYourInventory
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(478).expect("Unable to turn 478 into a StatusCode");
                                                },
                                                ActionGeSellItemMyNameActionGeSellPostResponse::NoItemAtThisPrice
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(482).expect("Unable to turn 482 into a StatusCode");
                                                },
                                                ActionGeSellItemMyNameActionGeSellPostResponse::GrandExchangeNotFoundOnThisMap
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(598).expect("Unable to turn 598 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter GeTransactionItemSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter GeTransactionItemSchema")),
                        }
                }

                // ActionMoveMyNameActionMovePost - POST /my/{name}/action/move
                hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_MOVE) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_MOVE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_MOVE in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_MOVE.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_destination_schema: Option<models::DestinationSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_destination_schema) => param_destination_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter DestinationSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter DestinationSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_destination_schema = match param_destination_schema {
                                    Some(param_destination_schema) => param_destination_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter DestinationSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter DestinationSchema")),
                                };

                                let result = api_impl.action_move_my_name_action_move_post(
                                            param_name,
                                            param_destination_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ActionMoveMyNameActionMovePostResponse::TheCharacterHasMovedSuccessfully
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_MOVE_MY_NAME_ACTION_MOVE_POST_THE_CHARACTER_HAS_MOVED_SUCCESSFULLY"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionMoveMyNameActionMovePostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionMoveMyNameActionMovePostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionMoveMyNameActionMovePostResponse::CharacterAlreadyAtDestination
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(490).expect("Unable to turn 490 into a StatusCode");
                                                },
                                                ActionMoveMyNameActionMovePostResponse::MapNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                ActionMoveMyNameActionMovePostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter DestinationSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter DestinationSchema")),
                        }
                }

                // ActionRecyclingMyNameActionRecyclingPost - POST /my/{name}/action/recycling
                hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_RECYCLING) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_RECYCLING
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_RECYCLING in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_RECYCLING.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_recycling_schema: Option<models::RecyclingSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_recycling_schema) => param_recycling_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter RecyclingSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter RecyclingSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_recycling_schema = match param_recycling_schema {
                                    Some(param_recycling_schema) => param_recycling_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter RecyclingSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter RecyclingSchema")),
                                };

                                let result = api_impl.action_recycling_my_name_action_recycling_post(
                                            param_name,
                                            param_recycling_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ActionRecyclingMyNameActionRecyclingPostResponse::TheItemsWereSuccessfullyRecycled
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_RECYCLING_MY_NAME_ACTION_RECYCLING_POST_THE_ITEMS_WERE_SUCCESSFULLY_RECYCLED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionRecyclingMyNameActionRecyclingPostResponse::ItemNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                ActionRecyclingMyNameActionRecyclingPostResponse::WorkshopNotFoundOnThisMap
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(598).expect("Unable to turn 598 into a StatusCode");
                                                },
                                                ActionRecyclingMyNameActionRecyclingPostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionRecyclingMyNameActionRecyclingPostResponse::CharacterInventoryIsFull
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(497).expect("Unable to turn 497 into a StatusCode");
                                                },
                                                ActionRecyclingMyNameActionRecyclingPostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionRecyclingMyNameActionRecyclingPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                                ActionRecyclingMyNameActionRecyclingPostResponse::NotSkillLevelRequired
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(493).expect("Unable to turn 493 into a StatusCode");
                                                },
                                                ActionRecyclingMyNameActionRecyclingPostResponse::MissingItemOrInsufficientQuantityInYourInventory
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(478).expect("Unable to turn 478 into a StatusCode");
                                                },
                                                ActionRecyclingMyNameActionRecyclingPostResponse::ThisItemCannotBeRecycled
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(473).expect("Unable to turn 473 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter RecyclingSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter RecyclingSchema")),
                        }
                }

                // ActionTaskExchangeMyNameActionTaskExchangePost - POST /my/{name}/action/task/exchange
                hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_TASK_EXCHANGE) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_TASK_EXCHANGE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_TASK_EXCHANGE in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_TASK_EXCHANGE.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    let result = api_impl
                        .action_task_exchange_my_name_action_task_exchange_post(
                            param_name, &context,
                        )
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                                            Ok(rsp) => match rsp {
                                                ActionTaskExchangeMyNameActionTaskExchangePostResponse::TheTasksCoinsHaveBeenSuccessfullyExchanged
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_TASK_EXCHANGE_MY_NAME_ACTION_TASK_EXCHANGE_POST_THE_TASKS_COINS_HAVE_BEEN_SUCCESSFULLY_EXCHANGED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionTaskExchangeMyNameActionTaskExchangePostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionTaskExchangeMyNameActionTaskExchangePostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionTaskExchangeMyNameActionTaskExchangePostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                                ActionTaskExchangeMyNameActionTaskExchangePostResponse::TasksMasterNotFoundOnThisMap
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(598).expect("Unable to turn 598 into a StatusCode");
                                                },
                                                ActionTaskExchangeMyNameActionTaskExchangePostResponse::MissingItemOrInsufficientQuantityInYourInventory
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(478).expect("Unable to turn 478 into a StatusCode");
                                                },
                                                ActionTaskExchangeMyNameActionTaskExchangePostResponse::CharacterInventoryIsFull
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(497).expect("Unable to turn 497 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                    Ok(response)
                }

                // ActionUnequipItemMyNameActionUnequipPost - POST /my/{name}/action/unequip
                hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_UNEQUIP) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_UNEQUIP
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_UNEQUIP in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_UNEQUIP.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_unequip_schema: Option<models::UnequipSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_unequip_schema) => param_unequip_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter UnequipSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter UnequipSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_unequip_schema = match param_unequip_schema {
                                    Some(param_unequip_schema) => param_unequip_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter UnequipSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter UnequipSchema")),
                                };

                                let result = api_impl.action_unequip_item_my_name_action_unequip_post(
                                            param_name,
                                            param_unequip_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ActionUnequipItemMyNameActionUnequipPostResponse::TheItemHasBeenSuccessfullyUnequippedAndAddedInHisInventory
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_UNEQUIP_ITEM_MY_NAME_ACTION_UNEQUIP_POST_THE_ITEM_HAS_BEEN_SUCCESSFULLY_UNEQUIPPED_AND_ADDED_IN_HIS_INVENTORY"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionUnequipItemMyNameActionUnequipPostResponse::ItemNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                ActionUnequipItemMyNameActionUnequipPostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionUnequipItemMyNameActionUnequipPostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionUnequipItemMyNameActionUnequipPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                                ActionUnequipItemMyNameActionUnequipPostResponse::SlotIsEmpty
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(491).expect("Unable to turn 491 into a StatusCode");
                                                },
                                                ActionUnequipItemMyNameActionUnequipPostResponse::CharacterInventoryIsFull
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(497).expect("Unable to turn 497 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter UnequipSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter UnequipSchema")),
                        }
                }

                // ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPost - POST /my/{name}/action/bank/withdraw/gold
                hyper::Method::POST
                    if path.matched(paths::ID_MY_NAME_ACTION_BANK_WITHDRAW_GOLD) =>
                {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_BANK_WITHDRAW_GOLD
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_BANK_WITHDRAW_GOLD in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_BANK_WITHDRAW_GOLD.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_deposit_withdraw_gold_schema: Option<models::DepositWithdrawGoldSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_deposit_withdraw_gold_schema) => param_deposit_withdraw_gold_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter DepositWithdrawGoldSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter DepositWithdrawGoldSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_deposit_withdraw_gold_schema = match param_deposit_withdraw_gold_schema {
                                    Some(param_deposit_withdraw_gold_schema) => param_deposit_withdraw_gold_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter DepositWithdrawGoldSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter DepositWithdrawGoldSchema")),
                                };

                                let result = api_impl.action_withdraw_bank_gold_my_name_action_bank_withdraw_gold_post(
                                            param_name,
                                            param_deposit_withdraw_gold_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse::GoldsSuccessfullyWithdrawFromYourBank
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_WITHDRAW_BANK_GOLD_MY_NAME_ACTION_BANK_WITHDRAW_GOLD_POST_GOLDS_SUCCESSFULLY_WITHDRAW_FROM_YOUR_BANK"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse::ATransactionIsAlreadyInProgressWithThisItem
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(461).expect("Unable to turn 461 into a StatusCode");
                                                },
                                                ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                                ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse::BankNotFoundOnThisMap
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(598).expect("Unable to turn 598 into a StatusCode");
                                                },
                                                ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPostResponse::InsufficientGoldsInYourBank
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(460).expect("Unable to turn 460 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter DepositWithdrawGoldSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter DepositWithdrawGoldSchema")),
                        }
                }

                // ActionWithdrawBankMyNameActionBankWithdrawPost - POST /my/{name}/action/bank/withdraw
                hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_BANK_WITHDRAW) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_MY_NAME_ACTION_BANK_WITHDRAW
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MY_NAME_ACTION_BANK_WITHDRAW in set but failed match against \"{}\"", path, paths::REGEX_MY_NAME_ACTION_BANK_WITHDRAW.as_str())
                    );

                    let param_name = match percent_encoding::percent_decode(path_params["name"].as_bytes()).decode_utf8() {
                    Ok(param_name) => match param_name.parse::<String>() {
                        Ok(param_name) => param_name,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter name: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["name"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_simple_item_schema: Option<models::SimpleItemSchema> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_simple_item_schema) => param_simple_item_schema,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter SimpleItemSchema - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter SimpleItemSchema due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_simple_item_schema = match param_simple_item_schema {
                                    Some(param_simple_item_schema) => param_simple_item_schema,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter SimpleItemSchema"))
                                                        .expect("Unable to create Bad Request response for missing body parameter SimpleItemSchema")),
                                };

                                let result = api_impl.action_withdraw_bank_my_name_action_bank_withdraw_post(
                                            param_name,
                                            param_simple_item_schema,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ActionWithdrawBankMyNameActionBankWithdrawPostResponse::ItemSuccessfullyWithdrawFromYourBank
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ACTION_WITHDRAW_BANK_MY_NAME_ACTION_BANK_WITHDRAW_POST_ITEM_SUCCESSFULLY_WITHDRAW_FROM_YOUR_BANK"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ActionWithdrawBankMyNameActionBankWithdrawPostResponse::ItemNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                ActionWithdrawBankMyNameActionBankWithdrawPostResponse::CharacterNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(498).expect("Unable to turn 498 into a StatusCode");
                                                },
                                                ActionWithdrawBankMyNameActionBankWithdrawPostResponse::CharacterInCooldown
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(499).expect("Unable to turn 499 into a StatusCode");
                                                },
                                                ActionWithdrawBankMyNameActionBankWithdrawPostResponse::ATransactionIsAlreadyInProgressWithThisItem
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(461).expect("Unable to turn 461 into a StatusCode");
                                                },
                                                ActionWithdrawBankMyNameActionBankWithdrawPostResponse::AnActionIsAlreadyInProgressByYourCharacter
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(486).expect("Unable to turn 486 into a StatusCode");
                                                },
                                                ActionWithdrawBankMyNameActionBankWithdrawPostResponse::CharacterInventoryIsFull
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(497).expect("Unable to turn 497 into a StatusCode");
                                                },
                                                ActionWithdrawBankMyNameActionBankWithdrawPostResponse::BankNotFoundOnThisMap
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(598).expect("Unable to turn 598 into a StatusCode");
                                                },
                                                ActionWithdrawBankMyNameActionBankWithdrawPostResponse::MissingItemOrInsufficientQuantityInYourInventory
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(478).expect("Unable to turn 478 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter SimpleItemSchema: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter SimpleItemSchema")),
                        }
                }

                // GetAllCharactersLogsMyLogsGet - GET /my/logs
                hyper::Method::GET if path.matched(paths::ID_MY_LOGS) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                    let query_params =
                        form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes())
                            .collect::<Vec<_>>();
                    let param_page = query_params
                        .iter()
                        .filter(|e| e.0 == "page")
                        .map(|e| e.1.clone())
                        .next();
                    let param_page = match param_page {
                        Some(param_page) => {
                            let param_page = <i32 as std::str::FromStr>::from_str(&param_page);
                            match param_page {
                            Ok(param_page) => Some(param_page),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter page - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter page")),
                        }
                        }
                        None => None,
                    };
                    let param_size = query_params
                        .iter()
                        .filter(|e| e.0 == "size")
                        .map(|e| e.1.clone())
                        .next();
                    let param_size = match param_size {
                        Some(param_size) => {
                            let param_size = <i32 as std::str::FromStr>::from_str(&param_size);
                            match param_size {
                            Ok(param_size) => Some(param_size),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter size - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter size")),
                        }
                        }
                        None => None,
                    };

                    let result = api_impl
                        .get_all_characters_logs_my_logs_get(param_page, param_size, &context)
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            GetAllCharactersLogsMyLogsGetResponse::SuccessfullyFetchedLogs(
                                body,
                            ) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_CHARACTERS_LOGS_MY_LOGS_GET_SUCCESSFULLY_FETCHED_LOGS"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                            GetAllCharactersLogsMyLogsGetResponse::LogsNotFound => {
                                *response.status_mut() = StatusCode::from_u16(404)
                                    .expect("Unable to turn 404 into a StatusCode");
                            }
                            GetAllCharactersLogsMyLogsGetResponse::CharacterNotFound => {
                                *response.status_mut() = StatusCode::from_u16(498)
                                    .expect("Unable to turn 498 into a StatusCode");
                            }
                        },
                        Err(e) => {
                            dbg!(e);
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // GetMyCharactersMyCharactersGet - GET /my/characters
                hyper::Method::GET if path.matched(paths::ID_MY_CHARACTERS) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    let result = api_impl.get_my_characters_my_characters_get(&context).await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                                            Ok(rsp) => match rsp {
                                                GetMyCharactersMyCharactersGetResponse::SuccessfullyFetchedCharacters
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_MY_CHARACTERS_MY_CHARACTERS_GET_SUCCESSFULLY_FETCHED_CHARACTERS"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                GetMyCharactersMyCharactersGetResponse::CharactersNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                    Ok(response)
                }

                // GetAllResourcesResourcesGet - GET /resources/
                hyper::Method::GET if path.matched(paths::ID_RESOURCES_) => {
                    // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                    let query_params =
                        form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes())
                            .collect::<Vec<_>>();
                    let param_min_level = query_params
                        .iter()
                        .filter(|e| e.0 == "min_level")
                        .map(|e| e.1.clone())
                        .next();
                    let param_min_level = match param_min_level {
                        Some(param_min_level) => {
                            let param_min_level =
                                <i32 as std::str::FromStr>::from_str(&param_min_level);
                            match param_min_level {
                            Ok(param_min_level) => Some(param_min_level),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter min_level - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter min_level")),
                        }
                        }
                        None => None,
                    };
                    let param_max_level = query_params
                        .iter()
                        .filter(|e| e.0 == "max_level")
                        .map(|e| e.1.clone())
                        .next();
                    let param_max_level = match param_max_level {
                        Some(param_max_level) => {
                            let param_max_level =
                                <i32 as std::str::FromStr>::from_str(&param_max_level);
                            match param_max_level {
                            Ok(param_max_level) => Some(param_max_level),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter max_level - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter max_level")),
                        }
                        }
                        None => None,
                    };
                    let param_skill = query_params
                        .iter()
                        .filter(|e| e.0 == "skill")
                        .map(|e| e.1.clone())
                        .next();
                    let param_skill = match param_skill {
                        Some(param_skill) => {
                            let param_skill = <String as std::str::FromStr>::from_str(&param_skill);
                            match param_skill {
                            Ok(param_skill) => Some(param_skill),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter skill - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter skill")),
                        }
                        }
                        None => None,
                    };
                    let param_drop = query_params
                        .iter()
                        .filter(|e| e.0 == "drop")
                        .map(|e| e.1.clone())
                        .next();
                    let param_drop = match param_drop {
                        Some(param_drop) => {
                            let param_drop = <String as std::str::FromStr>::from_str(&param_drop);
                            match param_drop {
                            Ok(param_drop) => Some(param_drop),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter drop - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter drop")),
                        }
                        }
                        None => None,
                    };
                    let param_page = query_params
                        .iter()
                        .filter(|e| e.0 == "page")
                        .map(|e| e.1.clone())
                        .next();
                    let param_page = match param_page {
                        Some(param_page) => {
                            let param_page = <i32 as std::str::FromStr>::from_str(&param_page);
                            match param_page {
                            Ok(param_page) => Some(param_page),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter page - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter page")),
                        }
                        }
                        None => None,
                    };
                    let param_size = query_params
                        .iter()
                        .filter(|e| e.0 == "size")
                        .map(|e| e.1.clone())
                        .next();
                    let param_size = match param_size {
                        Some(param_size) => {
                            let param_size = <i32 as std::str::FromStr>::from_str(&param_size);
                            match param_size {
                            Ok(param_size) => Some(param_size),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter size - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter size")),
                        }
                        }
                        None => None,
                    };

                    let result = api_impl
                        .get_all_resources_resources_get(
                            param_min_level,
                            param_max_level,
                            param_skill,
                            param_drop,
                            param_page,
                            param_size,
                            &context,
                        )
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                                            Ok(rsp) => match rsp {
                                                GetAllResourcesResourcesGetResponse::SuccessfullyFetchedResourcesDetails
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_ALL_RESOURCES_RESOURCES_GET_SUCCESSFULLY_FETCHED_RESOURCES_DETAILS"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                GetAllResourcesResourcesGetResponse::ResourcesNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(e) => {
                                                dbg!(e);
// Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                    Ok(response)
                }

                // GetResourceResourcesCodeGet - GET /resources/{code}
                hyper::Method::GET if path.matched(paths::ID_RESOURCES_CODE) => {
                    // Path parameters
                    let path: &str = uri.path();
                    let path_params =
                    paths::REGEX_RESOURCES_CODE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE RESOURCES_CODE in set but failed match against \"{}\"", path, paths::REGEX_RESOURCES_CODE.as_str())
                    );

                    let param_code = match percent_encoding::percent_decode(path_params["code"].as_bytes()).decode_utf8() {
                    Ok(param_code) => match param_code.parse::<String>() {
                        Ok(param_code) => param_code,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter code: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["code"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    let result = api_impl
                        .get_resource_resources_code_get(param_code, &context)
                        .await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            GetResourceResourcesCodeGetResponse::SuccessfullyFetchedResource(
                                body,
                            ) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_RESOURCE_RESOURCES_CODE_GET_SUCCESSFULLY_FETCHED_RESOURCE"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                            GetResourceResourcesCodeGetResponse::RessourceNotFound => {
                                *response.status_mut() = StatusCode::from_u16(404)
                                    .expect("Unable to turn 404 into a StatusCode");
                            }
                        },
                        Err(e) => {
                            dbg!(e);
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // GenerateTokenTokenPost - POST /token/
                hyper::Method::POST if path.matched(paths::ID_TOKEN_) => {
                    {
                        let authorization = match *(&context as &dyn Has<Option<Authorization>>)
                            .get()
                        {
                            Some(ref authorization) => authorization,
                            None => {
                                return Ok(Response::builder()
                                    .status(StatusCode::FORBIDDEN)
                                    .body(Body::from("Unauthenticated"))
                                    .expect("Unable to create Authentication Forbidden response"))
                            }
                        };
                    }

                    let result = api_impl.generate_token_token_post(&context).await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            GenerateTokenTokenPostResponse::TokenGeneratedSuccessfully(body) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GENERATE_TOKEN_TOKEN_POST_TOKEN_GENERATED_SUCCESSFULLY"));
                                let body_content = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body_content);
                            }
                            GenerateTokenTokenPostResponse::TokenGenerationFailed => {
                                *response.status_mut() = StatusCode::from_u16(455)
                                    .expect("Unable to turn 455 into a StatusCode");
                            }
                        },
                        Err(e) => {
                            dbg!(e);
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                _ if path.matched(paths::ID_) => method_not_allowed(),
                _ if path.matched(paths::ID_ACCOUNTS_CREATE) => method_not_allowed(),
                _ if path.matched(paths::ID_CHARACTERS_) => method_not_allowed(),
                _ if path.matched(paths::ID_CHARACTERS_CREATE) => method_not_allowed(),
                _ if path.matched(paths::ID_CHARACTERS_DELETE) => method_not_allowed(),
                _ if path.matched(paths::ID_CHARACTERS_NAME) => method_not_allowed(),
                _ if path.matched(paths::ID_EVENTS_) => method_not_allowed(),
                _ if path.matched(paths::ID_GE_) => method_not_allowed(),
                _ if path.matched(paths::ID_GE_CODE) => method_not_allowed(),
                _ if path.matched(paths::ID_ITEMS_) => method_not_allowed(),
                _ if path.matched(paths::ID_ITEMS_CODE) => method_not_allowed(),
                _ if path.matched(paths::ID_MAPS_) => method_not_allowed(),
                _ if path.matched(paths::ID_MAPS_X_Y) => method_not_allowed(),
                _ if path.matched(paths::ID_MONSTERS_) => method_not_allowed(),
                _ if path.matched(paths::ID_MONSTERS_CODE) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_BANK_GOLD) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_BANK_ITEMS) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_CHANGE_PASSWORD) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_CHARACTERS) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_LOGS) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_NAME_ACTION_BANK_DEPOSIT) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_NAME_ACTION_BANK_DEPOSIT_GOLD) => {
                    method_not_allowed()
                }
                _ if path.matched(paths::ID_MY_NAME_ACTION_BANK_WITHDRAW) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_NAME_ACTION_BANK_WITHDRAW_GOLD) => {
                    method_not_allowed()
                }
                _ if path.matched(paths::ID_MY_NAME_ACTION_CRAFTING) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_NAME_ACTION_DELETE) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_NAME_ACTION_EQUIP) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_NAME_ACTION_FIGHT) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_NAME_ACTION_GATHERING) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_NAME_ACTION_GE_BUY) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_NAME_ACTION_GE_SELL) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_NAME_ACTION_MOVE) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_NAME_ACTION_RECYCLING) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_NAME_ACTION_TASK_COMPLETE) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_NAME_ACTION_TASK_EXCHANGE) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_NAME_ACTION_TASK_NEW) => method_not_allowed(),
                _ if path.matched(paths::ID_MY_NAME_ACTION_UNEQUIP) => method_not_allowed(),
                _ if path.matched(paths::ID_RESOURCES_) => method_not_allowed(),
                _ if path.matched(paths::ID_RESOURCES_CODE) => method_not_allowed(),
                _ if path.matched(paths::ID_TOKEN_) => method_not_allowed(),
                _ => Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response")),
            }
        }
        Box::pin(run(self.api_impl.clone(), req))
    }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match *request.method() {
            // CreateAccountAccountsCreatePost - POST /accounts/create
            hyper::Method::POST if path.matched(paths::ID_ACCOUNTS_CREATE) => {
                Some("CreateAccountAccountsCreatePost")
            }
            // CreateCharacterCharactersCreatePost - POST /characters/create
            hyper::Method::POST if path.matched(paths::ID_CHARACTERS_CREATE) => {
                Some("CreateCharacterCharactersCreatePost")
            }
            // DeleteCharacterCharactersDeletePost - POST /characters/delete
            hyper::Method::POST if path.matched(paths::ID_CHARACTERS_DELETE) => {
                Some("DeleteCharacterCharactersDeletePost")
            }
            // GetAllCharactersCharactersGet - GET /characters/
            hyper::Method::GET if path.matched(paths::ID_CHARACTERS_) => {
                Some("GetAllCharactersCharactersGet")
            }
            // GetCharacterCharactersNameGet - GET /characters/{name}
            hyper::Method::GET if path.matched(paths::ID_CHARACTERS_NAME) => {
                Some("GetCharacterCharactersNameGet")
            }
            // GetStatusGet - GET /
            hyper::Method::GET if path.matched(paths::ID_) => Some("GetStatusGet"),
            // GetAllEventsEventsGet - GET /events/
            hyper::Method::GET if path.matched(paths::ID_EVENTS_) => Some("GetAllEventsEventsGet"),
            // GetAllGeItemsGeGet - GET /ge/
            hyper::Method::GET if path.matched(paths::ID_GE_) => Some("GetAllGeItemsGeGet"),
            // GetGeItemGeCodeGet - GET /ge/{code}
            hyper::Method::GET if path.matched(paths::ID_GE_CODE) => Some("GetGeItemGeCodeGet"),
            // GetAllItemsItemsGet - GET /items/
            hyper::Method::GET if path.matched(paths::ID_ITEMS_) => Some("GetAllItemsItemsGet"),
            // GetItemItemsCodeGet - GET /items/{code}
            hyper::Method::GET if path.matched(paths::ID_ITEMS_CODE) => Some("GetItemItemsCodeGet"),
            // GetAllMapsMapsGet - GET /maps/
            hyper::Method::GET if path.matched(paths::ID_MAPS_) => Some("GetAllMapsMapsGet"),
            // GetMapMapsXyGet - GET /maps/{x}/{y}
            hyper::Method::GET if path.matched(paths::ID_MAPS_X_Y) => Some("GetMapMapsXyGet"),
            // GetAllMonstersMonstersGet - GET /monsters/
            hyper::Method::GET if path.matched(paths::ID_MONSTERS_) => {
                Some("GetAllMonstersMonstersGet")
            }
            // GetMonsterMonstersCodeGet - GET /monsters/{code}
            hyper::Method::GET if path.matched(paths::ID_MONSTERS_CODE) => {
                Some("GetMonsterMonstersCodeGet")
            }
            // ChangePasswordMyChangePasswordPost - POST /my/change_password
            hyper::Method::POST if path.matched(paths::ID_MY_CHANGE_PASSWORD) => {
                Some("ChangePasswordMyChangePasswordPost")
            }
            // GetBankGoldsMyBankGoldGet - GET /my/bank/gold
            hyper::Method::GET if path.matched(paths::ID_MY_BANK_GOLD) => {
                Some("GetBankGoldsMyBankGoldGet")
            }
            // GetBankItemsMyBankItemsGet - GET /my/bank/items
            hyper::Method::GET if path.matched(paths::ID_MY_BANK_ITEMS) => {
                Some("GetBankItemsMyBankItemsGet")
            }
            // ActionAcceptNewTaskMyNameActionTaskNewPost - POST /my/{name}/action/task/new
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_TASK_NEW) => {
                Some("ActionAcceptNewTaskMyNameActionTaskNewPost")
            }
            // ActionCompleteTaskMyNameActionTaskCompletePost - POST /my/{name}/action/task/complete
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_TASK_COMPLETE) => {
                Some("ActionCompleteTaskMyNameActionTaskCompletePost")
            }
            // ActionCraftingMyNameActionCraftingPost - POST /my/{name}/action/crafting
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_CRAFTING) => {
                Some("ActionCraftingMyNameActionCraftingPost")
            }
            // ActionDeleteItemMyNameActionDeletePost - POST /my/{name}/action/delete
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_DELETE) => {
                Some("ActionDeleteItemMyNameActionDeletePost")
            }
            // ActionDepositBankGoldMyNameActionBankDepositGoldPost - POST /my/{name}/action/bank/deposit/gold
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_BANK_DEPOSIT_GOLD) => {
                Some("ActionDepositBankGoldMyNameActionBankDepositGoldPost")
            }
            // ActionDepositBankMyNameActionBankDepositPost - POST /my/{name}/action/bank/deposit
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_BANK_DEPOSIT) => {
                Some("ActionDepositBankMyNameActionBankDepositPost")
            }
            // ActionEquipItemMyNameActionEquipPost - POST /my/{name}/action/equip
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_EQUIP) => {
                Some("ActionEquipItemMyNameActionEquipPost")
            }
            // ActionFightMyNameActionFightPost - POST /my/{name}/action/fight
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_FIGHT) => {
                Some("ActionFightMyNameActionFightPost")
            }
            // ActionGatheringMyNameActionGatheringPost - POST /my/{name}/action/gathering
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_GATHERING) => {
                Some("ActionGatheringMyNameActionGatheringPost")
            }
            // ActionGeBuyItemMyNameActionGeBuyPost - POST /my/{name}/action/ge/buy
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_GE_BUY) => {
                Some("ActionGeBuyItemMyNameActionGeBuyPost")
            }
            // ActionGeSellItemMyNameActionGeSellPost - POST /my/{name}/action/ge/sell
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_GE_SELL) => {
                Some("ActionGeSellItemMyNameActionGeSellPost")
            }
            // ActionMoveMyNameActionMovePost - POST /my/{name}/action/move
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_MOVE) => {
                Some("ActionMoveMyNameActionMovePost")
            }
            // ActionRecyclingMyNameActionRecyclingPost - POST /my/{name}/action/recycling
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_RECYCLING) => {
                Some("ActionRecyclingMyNameActionRecyclingPost")
            }
            // ActionTaskExchangeMyNameActionTaskExchangePost - POST /my/{name}/action/task/exchange
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_TASK_EXCHANGE) => {
                Some("ActionTaskExchangeMyNameActionTaskExchangePost")
            }
            // ActionUnequipItemMyNameActionUnequipPost - POST /my/{name}/action/unequip
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_UNEQUIP) => {
                Some("ActionUnequipItemMyNameActionUnequipPost")
            }
            // ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPost - POST /my/{name}/action/bank/withdraw/gold
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_BANK_WITHDRAW_GOLD) => {
                Some("ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPost")
            }
            // ActionWithdrawBankMyNameActionBankWithdrawPost - POST /my/{name}/action/bank/withdraw
            hyper::Method::POST if path.matched(paths::ID_MY_NAME_ACTION_BANK_WITHDRAW) => {
                Some("ActionWithdrawBankMyNameActionBankWithdrawPost")
            }
            // GetAllCharactersLogsMyLogsGet - GET /my/logs
            hyper::Method::GET if path.matched(paths::ID_MY_LOGS) => {
                Some("GetAllCharactersLogsMyLogsGet")
            }
            // GetMyCharactersMyCharactersGet - GET /my/characters
            hyper::Method::GET if path.matched(paths::ID_MY_CHARACTERS) => {
                Some("GetMyCharactersMyCharactersGet")
            }
            // GetAllResourcesResourcesGet - GET /resources/
            hyper::Method::GET if path.matched(paths::ID_RESOURCES_) => {
                Some("GetAllResourcesResourcesGet")
            }
            // GetResourceResourcesCodeGet - GET /resources/{code}
            hyper::Method::GET if path.matched(paths::ID_RESOURCES_CODE) => {
                Some("GetResourceResourcesCodeGet")
            }
            // GenerateTokenTokenPost - POST /token/
            hyper::Method::POST if path.matched(paths::ID_TOKEN_) => Some("GenerateTokenTokenPost"),
            _ => None,
        }
    }
}
