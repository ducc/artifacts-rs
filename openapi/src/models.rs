#![allow(unused_qualifications)]

use validator::Validate;

#[cfg(any(feature = "client", feature = "server"))]
use crate::header;
use crate::models;

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ActionItemBankResponseSchema {
    #[serde(rename = "data")]
    pub data: models::BankItemSchema,
}

impl ActionItemBankResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::BankItemSchema) -> ActionItemBankResponseSchema {
        ActionItemBankResponseSchema { data }
    }
}

/// Converts the ActionItemBankResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ActionItemBankResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ActionItemBankResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ActionItemBankResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::BankItemSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ActionItemBankResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::BankItemSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ActionItemBankResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ActionItemBankResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in ActionItemBankResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ActionItemBankResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ActionItemBankResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ActionItemBankResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ActionItemBankResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<ActionItemBankResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ActionItemBankResponseSchema as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ActionItemBankResponseSchema - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ActiveEventSchema {
    /// Name of the event.
    #[serde(rename = "name")]
    pub name: String,

    /// Map of the event.
    #[serde(rename = "map")]
    pub map: models::MapSchema,

    /// Previous map skin.
    #[serde(rename = "previous_skin")]
    pub previous_skin: String,

    /// Duration in minutes.
    #[serde(rename = "duration")]
    pub duration: i32,

    /// Expiration datetime.
    #[serde(rename = "expiration")]
    pub expiration: chrono::DateTime<chrono::Utc>,

    /// Start datetime.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ActiveEventSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        name: String,
        map: models::MapSchema,
        previous_skin: String,
        duration: i32,
        expiration: chrono::DateTime<chrono::Utc>,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> ActiveEventSchema {
        ActiveEventSchema {
            name,
            map,
            previous_skin,
            duration,
            expiration,
            created_at,
        }
    }
}

/// Converts the ActiveEventSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ActiveEventSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            // Skipping map in query parameter serialization
            Some("previous_skin".to_string()),
            Some(self.previous_skin.to_string()),
            Some("duration".to_string()),
            Some(self.duration.to_string()),
            // Skipping expiration in query parameter serialization

            // Skipping created_at in query parameter serialization
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ActiveEventSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ActiveEventSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub map: Vec<models::MapSchema>,
            pub previous_skin: Vec<String>,
            pub duration: Vec<i32>,
            pub expiration: Vec<chrono::DateTime<chrono::Utc>>,
            pub created_at: Vec<chrono::DateTime<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ActiveEventSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "map" => intermediate_rep.map.push(
                        <models::MapSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "previous_skin" => intermediate_rep.previous_skin.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "duration" => intermediate_rep.duration.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "expiration" => intermediate_rep.expiration.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "created_at" => intermediate_rep.created_at.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ActiveEventSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ActiveEventSchema {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in ActiveEventSchema".to_string())?,
            map: intermediate_rep
                .map
                .into_iter()
                .next()
                .ok_or_else(|| "map missing in ActiveEventSchema".to_string())?,
            previous_skin: intermediate_rep
                .previous_skin
                .into_iter()
                .next()
                .ok_or_else(|| "previous_skin missing in ActiveEventSchema".to_string())?,
            duration: intermediate_rep
                .duration
                .into_iter()
                .next()
                .ok_or_else(|| "duration missing in ActiveEventSchema".to_string())?,
            expiration: intermediate_rep
                .expiration
                .into_iter()
                .next()
                .ok_or_else(|| "expiration missing in ActiveEventSchema".to_string())?,
            created_at: intermediate_rep
                .created_at
                .into_iter()
                .next()
                .ok_or_else(|| "created_at missing in ActiveEventSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ActiveEventSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ActiveEventSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ActiveEventSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ActiveEventSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<ActiveEventSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ActiveEventSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into ActiveEventSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AddAccountSchema {
    /// Your desired username.
    #[serde(rename = "username")]
    #[validate(length(min = 6, max = 32), regex = "RE_ADDACCOUNTSCHEMA_USERNAME")]
    pub username: String,

    /// Your password.
    #[serde(rename = "password")]
    #[validate(length(min = 5, max = 50), regex = "RE_ADDACCOUNTSCHEMA_PASSWORD")]
    pub password: String,

    /// Your email.
    #[serde(rename = "email")]
    pub email: String,
}

lazy_static::lazy_static! {
    static ref RE_ADDACCOUNTSCHEMA_USERNAME: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
}
lazy_static::lazy_static! {
    static ref RE_ADDACCOUNTSCHEMA_PASSWORD: regex::Regex = regex::Regex::new(r"^[^\\s]+$").unwrap();
}

impl AddAccountSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(username: String, password: String, email: String) -> AddAccountSchema {
        AddAccountSchema {
            username,
            password,
            email,
        }
    }
}

/// Converts the AddAccountSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AddAccountSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("username".to_string()),
            Some(self.username.to_string()),
            Some("password".to_string()),
            Some(self.password.to_string()),
            Some("email".to_string()),
            Some(self.email.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AddAccountSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AddAccountSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub username: Vec<String>,
            pub password: Vec<String>,
            pub email: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing AddAccountSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "username" => intermediate_rep.username.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "password" => intermediate_rep.password.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "email" => intermediate_rep.email.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing AddAccountSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AddAccountSchema {
            username: intermediate_rep
                .username
                .into_iter()
                .next()
                .ok_or_else(|| "username missing in AddAccountSchema".to_string())?,
            password: intermediate_rep
                .password
                .into_iter()
                .next()
                .ok_or_else(|| "password missing in AddAccountSchema".to_string())?,
            email: intermediate_rep
                .email
                .into_iter()
                .next()
                .ok_or_else(|| "email missing in AddAccountSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AddAccountSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AddAccountSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<AddAccountSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for AddAccountSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<AddAccountSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <AddAccountSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into AddAccountSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AddCharacterSchema {
    /// Your desired character name. It's unique and all players can see it.
    #[serde(rename = "name")]
    #[validate(length(min = 3, max = 12), regex = "RE_ADDCHARACTERSCHEMA_NAME")]
    pub name: String,

    /// Your desired skin.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "skin")]
    pub skin: String,
}

lazy_static::lazy_static! {
    static ref RE_ADDCHARACTERSCHEMA_NAME: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
}

impl AddCharacterSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(name: String, skin: String) -> AddCharacterSchema {
        AddCharacterSchema { name, skin }
    }
}

/// Converts the AddCharacterSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AddCharacterSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("skin".to_string()),
            Some(self.skin.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AddCharacterSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AddCharacterSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub skin: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing AddCharacterSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "skin" => intermediate_rep.skin.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing AddCharacterSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AddCharacterSchema {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in AddCharacterSchema".to_string())?,
            skin: intermediate_rep
                .skin
                .into_iter()
                .next()
                .ok_or_else(|| "skin missing in AddCharacterSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AddCharacterSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AddCharacterSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<AddCharacterSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for AddCharacterSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<AddCharacterSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <AddCharacterSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into AddCharacterSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AnnouncementSchema {
    /// Announcement text.
    #[serde(rename = "message")]
    pub message: String,

    /// Datetime of the announcement.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl AnnouncementSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(message: String) -> AnnouncementSchema {
        AnnouncementSchema {
            message,
            created_at: None,
        }
    }
}

/// Converts the AnnouncementSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AnnouncementSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("message".to_string()),
            Some(self.message.to_string()),
            // Skipping created_at in query parameter serialization
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AnnouncementSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AnnouncementSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub message: Vec<String>,
            pub created_at: Vec<chrono::DateTime<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing AnnouncementSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "created_at" => intermediate_rep.created_at.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing AnnouncementSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AnnouncementSchema {
            message: intermediate_rep
                .message
                .into_iter()
                .next()
                .ok_or_else(|| "message missing in AnnouncementSchema".to_string())?,
            created_at: intermediate_rep.created_at.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AnnouncementSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AnnouncementSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<AnnouncementSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for AnnouncementSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<AnnouncementSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <AnnouncementSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into AnnouncementSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BankItemSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: models::CooldownSchema,

    /// Item details.
    #[serde(rename = "item")]
    pub item: models::ItemSchema,

    /// Items in your banks.
    #[serde(rename = "bank")]
    pub bank: Vec<models::SimpleItemSchema>,

    /// Player details.
    #[serde(rename = "character")]
    pub character: models::CharacterSchema,
}

impl BankItemSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        cooldown: models::CooldownSchema,
        item: models::ItemSchema,
        bank: Vec<models::SimpleItemSchema>,
        character: models::CharacterSchema,
    ) -> BankItemSchema {
        BankItemSchema {
            cooldown,
            item,
            bank,
            character,
        }
    }
}

/// Converts the BankItemSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for BankItemSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization

            // Skipping item in query parameter serialization

            // Skipping bank in query parameter serialization

            // Skipping character in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a BankItemSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for BankItemSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::CooldownSchema>,
            pub item: Vec<models::ItemSchema>,
            pub bank: Vec<Vec<models::SimpleItemSchema>>,
            pub character: Vec<models::CharacterSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing BankItemSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(
                        <models::CooldownSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "item" => intermediate_rep.item.push(
                        <models::ItemSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    "bank" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in BankItemSchema"
                                .to_string(),
                        )
                    }
                    #[allow(clippy::redundant_clone)]
                    "character" => intermediate_rep.character.push(
                        <models::CharacterSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing BankItemSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(BankItemSchema {
            cooldown: intermediate_rep
                .cooldown
                .into_iter()
                .next()
                .ok_or_else(|| "cooldown missing in BankItemSchema".to_string())?,
            item: intermediate_rep
                .item
                .into_iter()
                .next()
                .ok_or_else(|| "item missing in BankItemSchema".to_string())?,
            bank: intermediate_rep
                .bank
                .into_iter()
                .next()
                .ok_or_else(|| "bank missing in BankItemSchema".to_string())?,
            character: intermediate_rep
                .character
                .into_iter()
                .next()
                .ok_or_else(|| "character missing in BankItemSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<BankItemSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<BankItemSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<BankItemSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for BankItemSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<BankItemSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <BankItemSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into BankItemSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BlockedHitsSchema {
    /// The amount of fire hits blocked.
    #[serde(rename = "fire")]
    pub fire: i32,

    /// The amount of earth hits blocked.
    #[serde(rename = "earth")]
    pub earth: i32,

    /// The amount of water hits blocked.
    #[serde(rename = "water")]
    pub water: i32,

    /// The amount of air hits blocked.
    #[serde(rename = "air")]
    pub air: i32,

    /// The amount of total hits blocked.
    #[serde(rename = "total")]
    pub total: i32,
}

impl BlockedHitsSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(fire: i32, earth: i32, water: i32, air: i32, total: i32) -> BlockedHitsSchema {
        BlockedHitsSchema {
            fire,
            earth,
            water,
            air,
            total,
        }
    }
}

/// Converts the BlockedHitsSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for BlockedHitsSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("fire".to_string()),
            Some(self.fire.to_string()),
            Some("earth".to_string()),
            Some(self.earth.to_string()),
            Some("water".to_string()),
            Some(self.water.to_string()),
            Some("air".to_string()),
            Some(self.air.to_string()),
            Some("total".to_string()),
            Some(self.total.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a BlockedHitsSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for BlockedHitsSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub fire: Vec<i32>,
            pub earth: Vec<i32>,
            pub water: Vec<i32>,
            pub air: Vec<i32>,
            pub total: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing BlockedHitsSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "fire" => intermediate_rep.fire.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "earth" => intermediate_rep.earth.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "water" => intermediate_rep.water.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "air" => intermediate_rep.air.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "total" => intermediate_rep.total.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing BlockedHitsSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(BlockedHitsSchema {
            fire: intermediate_rep
                .fire
                .into_iter()
                .next()
                .ok_or_else(|| "fire missing in BlockedHitsSchema".to_string())?,
            earth: intermediate_rep
                .earth
                .into_iter()
                .next()
                .ok_or_else(|| "earth missing in BlockedHitsSchema".to_string())?,
            water: intermediate_rep
                .water
                .into_iter()
                .next()
                .ok_or_else(|| "water missing in BlockedHitsSchema".to_string())?,
            air: intermediate_rep
                .air
                .into_iter()
                .next()
                .ok_or_else(|| "air missing in BlockedHitsSchema".to_string())?,
            total: intermediate_rep
                .total
                .into_iter()
                .next()
                .ok_or_else(|| "total missing in BlockedHitsSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<BlockedHitsSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<BlockedHitsSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<BlockedHitsSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for BlockedHitsSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<BlockedHitsSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <BlockedHitsSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into BlockedHitsSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ChangePassword {
    /// Your password.
    #[serde(rename = "password")]
    #[validate(length(min = 5, max = 50), regex = "RE_CHANGEPASSWORD_PASSWORD")]
    pub password: String,
}

lazy_static::lazy_static! {
    static ref RE_CHANGEPASSWORD_PASSWORD: regex::Regex = regex::Regex::new(r"^[^\\s]+$").unwrap();
}

impl ChangePassword {
    #[allow(clippy::new_without_default)]
    pub fn new(password: String) -> ChangePassword {
        ChangePassword { password }
    }
}

/// Converts the ChangePassword value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ChangePassword {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("password".to_string()),
            Some(self.password.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ChangePassword value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ChangePassword {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub password: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ChangePassword".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "password" => intermediate_rep.password.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ChangePassword".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ChangePassword {
            password: intermediate_rep
                .password
                .into_iter()
                .next()
                .ok_or_else(|| "password missing in ChangePassword".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ChangePassword> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ChangePassword>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ChangePassword>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ChangePassword - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ChangePassword> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ChangePassword as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into ChangePassword - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CharacterFightDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: models::CooldownSchema,

    /// Fight details.
    #[serde(rename = "fight")]
    pub fight: models::FightSchema,

    /// Player details.
    #[serde(rename = "character")]
    pub character: models::CharacterSchema,
}

impl CharacterFightDataSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        cooldown: models::CooldownSchema,
        fight: models::FightSchema,
        character: models::CharacterSchema,
    ) -> CharacterFightDataSchema {
        CharacterFightDataSchema {
            cooldown,
            fight,
            character,
        }
    }
}

/// Converts the CharacterFightDataSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CharacterFightDataSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization

            // Skipping fight in query parameter serialization

            // Skipping character in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CharacterFightDataSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CharacterFightDataSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::CooldownSchema>,
            pub fight: Vec<models::FightSchema>,
            pub character: Vec<models::CharacterSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CharacterFightDataSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(
                        <models::CooldownSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "fight" => intermediate_rep.fight.push(
                        <models::FightSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "character" => intermediate_rep.character.push(
                        <models::CharacterSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CharacterFightDataSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CharacterFightDataSchema {
            cooldown: intermediate_rep
                .cooldown
                .into_iter()
                .next()
                .ok_or_else(|| "cooldown missing in CharacterFightDataSchema".to_string())?,
            fight: intermediate_rep
                .fight
                .into_iter()
                .next()
                .ok_or_else(|| "fight missing in CharacterFightDataSchema".to_string())?,
            character: intermediate_rep
                .character
                .into_iter()
                .next()
                .ok_or_else(|| "character missing in CharacterFightDataSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CharacterFightDataSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CharacterFightDataSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CharacterFightDataSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for CharacterFightDataSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<CharacterFightDataSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CharacterFightDataSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into CharacterFightDataSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CharacterFightResponseSchema {
    #[serde(rename = "data")]
    pub data: models::CharacterFightDataSchema,
}

impl CharacterFightResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::CharacterFightDataSchema) -> CharacterFightResponseSchema {
        CharacterFightResponseSchema { data }
    }
}

/// Converts the CharacterFightResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CharacterFightResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CharacterFightResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CharacterFightResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::CharacterFightDataSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CharacterFightResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::CharacterFightDataSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CharacterFightResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CharacterFightResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in CharacterFightResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CharacterFightResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CharacterFightResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CharacterFightResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for CharacterFightResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<CharacterFightResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CharacterFightResponseSchema as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CharacterFightResponseSchema - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CharacterMovementDataSchema {
    /// Cooldown details
    #[serde(rename = "cooldown")]
    pub cooldown: models::CooldownSchema,

    /// Destination details.
    #[serde(rename = "destination")]
    pub destination: models::MapSchema,

    /// Character details.
    #[serde(rename = "character")]
    pub character: models::CharacterSchema,
}

impl CharacterMovementDataSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        cooldown: models::CooldownSchema,
        destination: models::MapSchema,
        character: models::CharacterSchema,
    ) -> CharacterMovementDataSchema {
        CharacterMovementDataSchema {
            cooldown,
            destination,
            character,
        }
    }
}

/// Converts the CharacterMovementDataSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CharacterMovementDataSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization

            // Skipping destination in query parameter serialization

            // Skipping character in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CharacterMovementDataSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CharacterMovementDataSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::CooldownSchema>,
            pub destination: Vec<models::MapSchema>,
            pub character: Vec<models::CharacterSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CharacterMovementDataSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(
                        <models::CooldownSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "destination" => intermediate_rep.destination.push(
                        <models::MapSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "character" => intermediate_rep.character.push(
                        <models::CharacterSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CharacterMovementDataSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CharacterMovementDataSchema {
            cooldown: intermediate_rep
                .cooldown
                .into_iter()
                .next()
                .ok_or_else(|| "cooldown missing in CharacterMovementDataSchema".to_string())?,
            destination: intermediate_rep
                .destination
                .into_iter()
                .next()
                .ok_or_else(|| "destination missing in CharacterMovementDataSchema".to_string())?,
            character: intermediate_rep
                .character
                .into_iter()
                .next()
                .ok_or_else(|| "character missing in CharacterMovementDataSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CharacterMovementDataSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CharacterMovementDataSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CharacterMovementDataSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for CharacterMovementDataSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<CharacterMovementDataSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CharacterMovementDataSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into CharacterMovementDataSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CharacterMovementResponseSchema {
    #[serde(rename = "data")]
    pub data: models::CharacterMovementDataSchema,
}

impl CharacterMovementResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::CharacterMovementDataSchema) -> CharacterMovementResponseSchema {
        CharacterMovementResponseSchema { data }
    }
}

/// Converts the CharacterMovementResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CharacterMovementResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CharacterMovementResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CharacterMovementResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::CharacterMovementDataSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CharacterMovementResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::CharacterMovementDataSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CharacterMovementResponseSchema"
                                .to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CharacterMovementResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in CharacterMovementResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CharacterMovementResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CharacterMovementResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CharacterMovementResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CharacterMovementResponseSchema - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<CharacterMovementResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CharacterMovementResponseSchema as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CharacterMovementResponseSchema - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CharacterResponseSchema {
    #[serde(rename = "data")]
    pub data: models::CharacterSchema,
}

impl CharacterResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::CharacterSchema) -> CharacterResponseSchema {
        CharacterResponseSchema { data }
    }
}

/// Converts the CharacterResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CharacterResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CharacterResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CharacterResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::CharacterSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CharacterResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::CharacterSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CharacterResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CharacterResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in CharacterResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CharacterResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CharacterResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CharacterResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for CharacterResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<CharacterResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CharacterResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into CharacterResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CharacterSchema {
    /// Name of the character.
    #[serde(rename = "name")]
    pub name: String,

    /// Character skin code.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "skin")]
    pub skin: String,

    /// Combat level.
    #[serde(rename = "level")]
    pub level: i32,

    /// The current xp level of the combat level.
    #[serde(rename = "xp")]
    pub xp: i32,

    /// XP required to level up the character.
    #[serde(rename = "max_xp")]
    pub max_xp: i32,

    /// Total XP of your character.
    #[serde(rename = "total_xp")]
    pub total_xp: i32,

    /// The numbers of golds on this character.
    #[serde(rename = "gold")]
    pub gold: i32,

    /// *Not available, on the roadmap. Character movement speed.
    #[serde(rename = "speed")]
    pub speed: i32,

    /// Mining level.
    #[serde(rename = "mining_level")]
    pub mining_level: i32,

    /// The current xp level of the Mining skill.
    #[serde(rename = "mining_xp")]
    pub mining_xp: i32,

    /// Mining XP required to level up the skill.
    #[serde(rename = "mining_max_xp")]
    pub mining_max_xp: i32,

    /// Woodcutting level.
    #[serde(rename = "woodcutting_level")]
    pub woodcutting_level: i32,

    /// The current xp level of the Woodcutting skill.
    #[serde(rename = "woodcutting_xp")]
    pub woodcutting_xp: i32,

    /// Woodcutting XP required to level up the skill.
    #[serde(rename = "woodcutting_max_xp")]
    pub woodcutting_max_xp: i32,

    /// Fishing level.
    #[serde(rename = "fishing_level")]
    pub fishing_level: i32,

    /// The current xp level of the Fishing skill.
    #[serde(rename = "fishing_xp")]
    pub fishing_xp: i32,

    /// Fishing XP required to level up the skill.
    #[serde(rename = "fishing_max_xp")]
    pub fishing_max_xp: i32,

    /// Weaponcrafting level.
    #[serde(rename = "weaponcrafting_level")]
    pub weaponcrafting_level: i32,

    /// The current xp level of the Weaponcrafting skill.
    #[serde(rename = "weaponcrafting_xp")]
    pub weaponcrafting_xp: i32,

    /// Weaponcrafting XP required to level up the skill.
    #[serde(rename = "weaponcrafting_max_xp")]
    pub weaponcrafting_max_xp: i32,

    /// Gearcrafting level.
    #[serde(rename = "gearcrafting_level")]
    pub gearcrafting_level: i32,

    /// The current xp level of the Gearcrafting skill.
    #[serde(rename = "gearcrafting_xp")]
    pub gearcrafting_xp: i32,

    /// Gearcrafting XP required to level up the skill.
    #[serde(rename = "gearcrafting_max_xp")]
    pub gearcrafting_max_xp: i32,

    /// Jewelrycrafting level.
    #[serde(rename = "jewelrycrafting_level")]
    pub jewelrycrafting_level: i32,

    /// The current xp level of the Jewelrycrafting skill.
    #[serde(rename = "jewelrycrafting_xp")]
    pub jewelrycrafting_xp: i32,

    /// Jewelrycrafting XP required to level up the skill.
    #[serde(rename = "jewelrycrafting_max_xp")]
    pub jewelrycrafting_max_xp: i32,

    /// The current xp level of the Cooking skill.
    #[serde(rename = "cooking_level")]
    pub cooking_level: i32,

    /// Cooking XP.
    #[serde(rename = "cooking_xp")]
    pub cooking_xp: i32,

    /// Cooking XP required to level up the skill.
    #[serde(rename = "cooking_max_xp")]
    pub cooking_max_xp: i32,

    /// Character HP.
    #[serde(rename = "hp")]
    pub hp: i32,

    /// *Character Haste. Increase speed attack (reduce fight cooldown)
    #[serde(rename = "haste")]
    pub haste: i32,

    /// *Not available, on the roadmap. Character Critical   Strike. Critical strikes increase the attack's damage.
    #[serde(rename = "critical_strike")]
    pub critical_strike: i32,

    /// *Not available, on the roadmap. Regenerates life at the start of each turn.
    #[serde(rename = "stamina")]
    pub stamina: i32,

    /// Fire attack.
    #[serde(rename = "attack_fire")]
    pub attack_fire: i32,

    /// Earth attack.
    #[serde(rename = "attack_earth")]
    pub attack_earth: i32,

    /// Water attack.
    #[serde(rename = "attack_water")]
    pub attack_water: i32,

    /// Air attack.
    #[serde(rename = "attack_air")]
    pub attack_air: i32,

    /// % Fire damage.
    #[serde(rename = "dmg_fire")]
    pub dmg_fire: i32,

    /// % Earth damage.
    #[serde(rename = "dmg_earth")]
    pub dmg_earth: i32,

    /// % Water damage.
    #[serde(rename = "dmg_water")]
    pub dmg_water: i32,

    /// % Air damage.
    #[serde(rename = "dmg_air")]
    pub dmg_air: i32,

    /// % Fire resistance.
    #[serde(rename = "res_fire")]
    pub res_fire: i32,

    /// % Earth resistance.
    #[serde(rename = "res_earth")]
    pub res_earth: i32,

    /// % Water resistance.
    #[serde(rename = "res_water")]
    pub res_water: i32,

    /// % Air resistance.
    #[serde(rename = "res_air")]
    pub res_air: i32,

    /// Character x coordinate.
    #[serde(rename = "x")]
    pub x: i32,

    /// Character y coordinate.
    #[serde(rename = "y")]
    pub y: i32,

    /// Cooldown in seconds.
    #[serde(rename = "cooldown")]
    pub cooldown: i32,

    /// Datetime Cooldown expiration.
    #[serde(rename = "cooldown_expiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown_expiration: Option<chrono::DateTime<chrono::Utc>>,

    /// Weapon slot.
    #[serde(rename = "weapon_slot")]
    pub weapon_slot: String,

    /// Shield slot.
    #[serde(rename = "shield_slot")]
    pub shield_slot: String,

    /// Helmet slot.
    #[serde(rename = "helmet_slot")]
    pub helmet_slot: String,

    /// Body armor slot.
    #[serde(rename = "body_armor_slot")]
    pub body_armor_slot: String,

    /// Leg armor slot.
    #[serde(rename = "leg_armor_slot")]
    pub leg_armor_slot: String,

    /// Boots slot.
    #[serde(rename = "boots_slot")]
    pub boots_slot: String,

    /// Ring 1 slot.
    #[serde(rename = "ring1_slot")]
    pub ring1_slot: String,

    /// Ring 2 slot.
    #[serde(rename = "ring2_slot")]
    pub ring2_slot: String,

    /// Amulet slot.
    #[serde(rename = "amulet_slot")]
    pub amulet_slot: String,

    /// Artifact 1 slot.
    #[serde(rename = "artifact1_slot")]
    pub artifact1_slot: String,

    /// Artifact 2 slot.
    #[serde(rename = "artifact2_slot")]
    pub artifact2_slot: String,

    /// Artifact 3 slot.
    #[serde(rename = "artifact3_slot")]
    pub artifact3_slot: String,

    /// Consumable 1 slot.
    #[serde(rename = "consumable1_slot")]
    pub consumable1_slot: String,

    /// Consumable 1 quantity.
    #[serde(rename = "consumable1_slot_quantity")]
    pub consumable1_slot_quantity: i32,

    /// Consumable 2 slot.
    #[serde(rename = "consumable2_slot")]
    pub consumable2_slot: String,

    /// Consumable 2 quantity.
    #[serde(rename = "consumable2_slot_quantity")]
    pub consumable2_slot_quantity: i32,

    /// Task in progress.
    #[serde(rename = "task")]
    pub task: String,

    /// Task type.
    #[serde(rename = "task_type")]
    pub task_type: String,

    /// Task progression.
    #[serde(rename = "task_progress")]
    pub task_progress: i32,

    /// Task total objective.
    #[serde(rename = "task_total")]
    pub task_total: i32,

    /// Inventory max items.
    #[serde(rename = "inventory_max_items")]
    pub inventory_max_items: i32,

    /// List of inventory slots.
    #[serde(rename = "inventory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory: Option<Vec<models::InventorySlot>>,
}

impl CharacterSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        name: String,
        skin: String,
        level: i32,
        xp: i32,
        max_xp: i32,
        total_xp: i32,
        gold: i32,
        speed: i32,
        mining_level: i32,
        mining_xp: i32,
        mining_max_xp: i32,
        woodcutting_level: i32,
        woodcutting_xp: i32,
        woodcutting_max_xp: i32,
        fishing_level: i32,
        fishing_xp: i32,
        fishing_max_xp: i32,
        weaponcrafting_level: i32,
        weaponcrafting_xp: i32,
        weaponcrafting_max_xp: i32,
        gearcrafting_level: i32,
        gearcrafting_xp: i32,
        gearcrafting_max_xp: i32,
        jewelrycrafting_level: i32,
        jewelrycrafting_xp: i32,
        jewelrycrafting_max_xp: i32,
        cooking_level: i32,
        cooking_xp: i32,
        cooking_max_xp: i32,
        hp: i32,
        haste: i32,
        critical_strike: i32,
        stamina: i32,
        attack_fire: i32,
        attack_earth: i32,
        attack_water: i32,
        attack_air: i32,
        dmg_fire: i32,
        dmg_earth: i32,
        dmg_water: i32,
        dmg_air: i32,
        res_fire: i32,
        res_earth: i32,
        res_water: i32,
        res_air: i32,
        x: i32,
        y: i32,
        cooldown: i32,
        weapon_slot: String,
        shield_slot: String,
        helmet_slot: String,
        body_armor_slot: String,
        leg_armor_slot: String,
        boots_slot: String,
        ring1_slot: String,
        ring2_slot: String,
        amulet_slot: String,
        artifact1_slot: String,
        artifact2_slot: String,
        artifact3_slot: String,
        consumable1_slot: String,
        consumable1_slot_quantity: i32,
        consumable2_slot: String,
        consumable2_slot_quantity: i32,
        task: String,
        task_type: String,
        task_progress: i32,
        task_total: i32,
        inventory_max_items: i32,
    ) -> CharacterSchema {
        CharacterSchema {
            name,
            skin,
            level,
            xp,
            max_xp,
            total_xp,
            gold,
            speed,
            mining_level,
            mining_xp,
            mining_max_xp,
            woodcutting_level,
            woodcutting_xp,
            woodcutting_max_xp,
            fishing_level,
            fishing_xp,
            fishing_max_xp,
            weaponcrafting_level,
            weaponcrafting_xp,
            weaponcrafting_max_xp,
            gearcrafting_level,
            gearcrafting_xp,
            gearcrafting_max_xp,
            jewelrycrafting_level,
            jewelrycrafting_xp,
            jewelrycrafting_max_xp,
            cooking_level,
            cooking_xp,
            cooking_max_xp,
            hp,
            haste,
            critical_strike,
            stamina,
            attack_fire,
            attack_earth,
            attack_water,
            attack_air,
            dmg_fire,
            dmg_earth,
            dmg_water,
            dmg_air,
            res_fire,
            res_earth,
            res_water,
            res_air,
            x,
            y,
            cooldown,
            cooldown_expiration: None,
            weapon_slot,
            shield_slot,
            helmet_slot,
            body_armor_slot,
            leg_armor_slot,
            boots_slot,
            ring1_slot,
            ring2_slot,
            amulet_slot,
            artifact1_slot,
            artifact2_slot,
            artifact3_slot,
            consumable1_slot,
            consumable1_slot_quantity,
            consumable2_slot,
            consumable2_slot_quantity,
            task,
            task_type,
            task_progress,
            task_total,
            inventory_max_items,
            inventory: None,
        }
    }
}

/// Converts the CharacterSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CharacterSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("skin".to_string()),
            Some(self.skin.to_string()),
            Some("level".to_string()),
            Some(self.level.to_string()),
            Some("xp".to_string()),
            Some(self.xp.to_string()),
            Some("max_xp".to_string()),
            Some(self.max_xp.to_string()),
            Some("total_xp".to_string()),
            Some(self.total_xp.to_string()),
            Some("gold".to_string()),
            Some(self.gold.to_string()),
            Some("speed".to_string()),
            Some(self.speed.to_string()),
            Some("mining_level".to_string()),
            Some(self.mining_level.to_string()),
            Some("mining_xp".to_string()),
            Some(self.mining_xp.to_string()),
            Some("mining_max_xp".to_string()),
            Some(self.mining_max_xp.to_string()),
            Some("woodcutting_level".to_string()),
            Some(self.woodcutting_level.to_string()),
            Some("woodcutting_xp".to_string()),
            Some(self.woodcutting_xp.to_string()),
            Some("woodcutting_max_xp".to_string()),
            Some(self.woodcutting_max_xp.to_string()),
            Some("fishing_level".to_string()),
            Some(self.fishing_level.to_string()),
            Some("fishing_xp".to_string()),
            Some(self.fishing_xp.to_string()),
            Some("fishing_max_xp".to_string()),
            Some(self.fishing_max_xp.to_string()),
            Some("weaponcrafting_level".to_string()),
            Some(self.weaponcrafting_level.to_string()),
            Some("weaponcrafting_xp".to_string()),
            Some(self.weaponcrafting_xp.to_string()),
            Some("weaponcrafting_max_xp".to_string()),
            Some(self.weaponcrafting_max_xp.to_string()),
            Some("gearcrafting_level".to_string()),
            Some(self.gearcrafting_level.to_string()),
            Some("gearcrafting_xp".to_string()),
            Some(self.gearcrafting_xp.to_string()),
            Some("gearcrafting_max_xp".to_string()),
            Some(self.gearcrafting_max_xp.to_string()),
            Some("jewelrycrafting_level".to_string()),
            Some(self.jewelrycrafting_level.to_string()),
            Some("jewelrycrafting_xp".to_string()),
            Some(self.jewelrycrafting_xp.to_string()),
            Some("jewelrycrafting_max_xp".to_string()),
            Some(self.jewelrycrafting_max_xp.to_string()),
            Some("cooking_level".to_string()),
            Some(self.cooking_level.to_string()),
            Some("cooking_xp".to_string()),
            Some(self.cooking_xp.to_string()),
            Some("cooking_max_xp".to_string()),
            Some(self.cooking_max_xp.to_string()),
            Some("hp".to_string()),
            Some(self.hp.to_string()),
            Some("haste".to_string()),
            Some(self.haste.to_string()),
            Some("critical_strike".to_string()),
            Some(self.critical_strike.to_string()),
            Some("stamina".to_string()),
            Some(self.stamina.to_string()),
            Some("attack_fire".to_string()),
            Some(self.attack_fire.to_string()),
            Some("attack_earth".to_string()),
            Some(self.attack_earth.to_string()),
            Some("attack_water".to_string()),
            Some(self.attack_water.to_string()),
            Some("attack_air".to_string()),
            Some(self.attack_air.to_string()),
            Some("dmg_fire".to_string()),
            Some(self.dmg_fire.to_string()),
            Some("dmg_earth".to_string()),
            Some(self.dmg_earth.to_string()),
            Some("dmg_water".to_string()),
            Some(self.dmg_water.to_string()),
            Some("dmg_air".to_string()),
            Some(self.dmg_air.to_string()),
            Some("res_fire".to_string()),
            Some(self.res_fire.to_string()),
            Some("res_earth".to_string()),
            Some(self.res_earth.to_string()),
            Some("res_water".to_string()),
            Some(self.res_water.to_string()),
            Some("res_air".to_string()),
            Some(self.res_air.to_string()),
            Some("x".to_string()),
            Some(self.x.to_string()),
            Some("y".to_string()),
            Some(self.y.to_string()),
            Some("cooldown".to_string()),
            Some(self.cooldown.to_string()),
            // Skipping cooldown_expiration in query parameter serialization
            Some("weapon_slot".to_string()),
            Some(self.weapon_slot.to_string()),
            Some("shield_slot".to_string()),
            Some(self.shield_slot.to_string()),
            Some("helmet_slot".to_string()),
            Some(self.helmet_slot.to_string()),
            Some("body_armor_slot".to_string()),
            Some(self.body_armor_slot.to_string()),
            Some("leg_armor_slot".to_string()),
            Some(self.leg_armor_slot.to_string()),
            Some("boots_slot".to_string()),
            Some(self.boots_slot.to_string()),
            Some("ring1_slot".to_string()),
            Some(self.ring1_slot.to_string()),
            Some("ring2_slot".to_string()),
            Some(self.ring2_slot.to_string()),
            Some("amulet_slot".to_string()),
            Some(self.amulet_slot.to_string()),
            Some("artifact1_slot".to_string()),
            Some(self.artifact1_slot.to_string()),
            Some("artifact2_slot".to_string()),
            Some(self.artifact2_slot.to_string()),
            Some("artifact3_slot".to_string()),
            Some(self.artifact3_slot.to_string()),
            Some("consumable1_slot".to_string()),
            Some(self.consumable1_slot.to_string()),
            Some("consumable1_slot_quantity".to_string()),
            Some(self.consumable1_slot_quantity.to_string()),
            Some("consumable2_slot".to_string()),
            Some(self.consumable2_slot.to_string()),
            Some("consumable2_slot_quantity".to_string()),
            Some(self.consumable2_slot_quantity.to_string()),
            Some("task".to_string()),
            Some(self.task.to_string()),
            Some("task_type".to_string()),
            Some(self.task_type.to_string()),
            Some("task_progress".to_string()),
            Some(self.task_progress.to_string()),
            Some("task_total".to_string()),
            Some(self.task_total.to_string()),
            Some("inventory_max_items".to_string()),
            Some(self.inventory_max_items.to_string()),
            // Skipping inventory in query parameter serialization
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CharacterSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CharacterSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub skin: Vec<String>,
            pub level: Vec<i32>,
            pub xp: Vec<i32>,
            pub max_xp: Vec<i32>,
            pub total_xp: Vec<i32>,
            pub gold: Vec<i32>,
            pub speed: Vec<i32>,
            pub mining_level: Vec<i32>,
            pub mining_xp: Vec<i32>,
            pub mining_max_xp: Vec<i32>,
            pub woodcutting_level: Vec<i32>,
            pub woodcutting_xp: Vec<i32>,
            pub woodcutting_max_xp: Vec<i32>,
            pub fishing_level: Vec<i32>,
            pub fishing_xp: Vec<i32>,
            pub fishing_max_xp: Vec<i32>,
            pub weaponcrafting_level: Vec<i32>,
            pub weaponcrafting_xp: Vec<i32>,
            pub weaponcrafting_max_xp: Vec<i32>,
            pub gearcrafting_level: Vec<i32>,
            pub gearcrafting_xp: Vec<i32>,
            pub gearcrafting_max_xp: Vec<i32>,
            pub jewelrycrafting_level: Vec<i32>,
            pub jewelrycrafting_xp: Vec<i32>,
            pub jewelrycrafting_max_xp: Vec<i32>,
            pub cooking_level: Vec<i32>,
            pub cooking_xp: Vec<i32>,
            pub cooking_max_xp: Vec<i32>,
            pub hp: Vec<i32>,
            pub haste: Vec<i32>,
            pub critical_strike: Vec<i32>,
            pub stamina: Vec<i32>,
            pub attack_fire: Vec<i32>,
            pub attack_earth: Vec<i32>,
            pub attack_water: Vec<i32>,
            pub attack_air: Vec<i32>,
            pub dmg_fire: Vec<i32>,
            pub dmg_earth: Vec<i32>,
            pub dmg_water: Vec<i32>,
            pub dmg_air: Vec<i32>,
            pub res_fire: Vec<i32>,
            pub res_earth: Vec<i32>,
            pub res_water: Vec<i32>,
            pub res_air: Vec<i32>,
            pub x: Vec<i32>,
            pub y: Vec<i32>,
            pub cooldown: Vec<i32>,
            pub cooldown_expiration: Vec<chrono::DateTime<chrono::Utc>>,
            pub weapon_slot: Vec<String>,
            pub shield_slot: Vec<String>,
            pub helmet_slot: Vec<String>,
            pub body_armor_slot: Vec<String>,
            pub leg_armor_slot: Vec<String>,
            pub boots_slot: Vec<String>,
            pub ring1_slot: Vec<String>,
            pub ring2_slot: Vec<String>,
            pub amulet_slot: Vec<String>,
            pub artifact1_slot: Vec<String>,
            pub artifact2_slot: Vec<String>,
            pub artifact3_slot: Vec<String>,
            pub consumable1_slot: Vec<String>,
            pub consumable1_slot_quantity: Vec<i32>,
            pub consumable2_slot: Vec<String>,
            pub consumable2_slot_quantity: Vec<i32>,
            pub task: Vec<String>,
            pub task_type: Vec<String>,
            pub task_progress: Vec<i32>,
            pub task_total: Vec<i32>,
            pub inventory_max_items: Vec<i32>,
            pub inventory: Vec<Vec<models::InventorySlot>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CharacterSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "skin" => intermediate_rep.skin.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "level" => intermediate_rep.level.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "xp" => intermediate_rep.xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "max_xp" => intermediate_rep.max_xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "total_xp" => intermediate_rep.total_xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "gold" => intermediate_rep.gold.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "speed" => intermediate_rep.speed.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "mining_level" => intermediate_rep.mining_level.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "mining_xp" => intermediate_rep.mining_xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "mining_max_xp" => intermediate_rep.mining_max_xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "woodcutting_level" => intermediate_rep.woodcutting_level.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "woodcutting_xp" => intermediate_rep.woodcutting_xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "woodcutting_max_xp" => intermediate_rep.woodcutting_max_xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "fishing_level" => intermediate_rep.fishing_level.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "fishing_xp" => intermediate_rep.fishing_xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "fishing_max_xp" => intermediate_rep.fishing_max_xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "weaponcrafting_level" => intermediate_rep.weaponcrafting_level.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "weaponcrafting_xp" => intermediate_rep.weaponcrafting_xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "weaponcrafting_max_xp" => intermediate_rep.weaponcrafting_max_xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "gearcrafting_level" => intermediate_rep.gearcrafting_level.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "gearcrafting_xp" => intermediate_rep.gearcrafting_xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "gearcrafting_max_xp" => intermediate_rep.gearcrafting_max_xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "jewelrycrafting_level" => intermediate_rep.jewelrycrafting_level.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "jewelrycrafting_xp" => intermediate_rep.jewelrycrafting_xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "jewelrycrafting_max_xp" => intermediate_rep.jewelrycrafting_max_xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "cooking_level" => intermediate_rep.cooking_level.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "cooking_xp" => intermediate_rep.cooking_xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "cooking_max_xp" => intermediate_rep.cooking_max_xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "hp" => intermediate_rep.hp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "haste" => intermediate_rep.haste.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "critical_strike" => intermediate_rep.critical_strike.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "stamina" => intermediate_rep.stamina.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "attack_fire" => intermediate_rep.attack_fire.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "attack_earth" => intermediate_rep.attack_earth.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "attack_water" => intermediate_rep.attack_water.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "attack_air" => intermediate_rep.attack_air.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "dmg_fire" => intermediate_rep.dmg_fire.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "dmg_earth" => intermediate_rep.dmg_earth.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "dmg_water" => intermediate_rep.dmg_water.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "dmg_air" => intermediate_rep.dmg_air.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "res_fire" => intermediate_rep.res_fire.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "res_earth" => intermediate_rep.res_earth.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "res_water" => intermediate_rep.res_water.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "res_air" => intermediate_rep.res_air.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "x" => intermediate_rep.x.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "y" => intermediate_rep.y.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "cooldown_expiration" => intermediate_rep.cooldown_expiration.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "weapon_slot" => intermediate_rep.weapon_slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "shield_slot" => intermediate_rep.shield_slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "helmet_slot" => intermediate_rep.helmet_slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "body_armor_slot" => intermediate_rep.body_armor_slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "leg_armor_slot" => intermediate_rep.leg_armor_slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "boots_slot" => intermediate_rep.boots_slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "ring1_slot" => intermediate_rep.ring1_slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "ring2_slot" => intermediate_rep.ring2_slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "amulet_slot" => intermediate_rep.amulet_slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "artifact1_slot" => intermediate_rep.artifact1_slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "artifact2_slot" => intermediate_rep.artifact2_slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "artifact3_slot" => intermediate_rep.artifact3_slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "consumable1_slot" => intermediate_rep.consumable1_slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "consumable1_slot_quantity" => intermediate_rep.consumable1_slot_quantity.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "consumable2_slot" => intermediate_rep.consumable2_slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "consumable2_slot_quantity" => intermediate_rep.consumable2_slot_quantity.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "task" => intermediate_rep.task.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "task_type" => intermediate_rep.task_type.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "task_progress" => intermediate_rep.task_progress.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "task_total" => intermediate_rep.task_total.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "inventory_max_items" => intermediate_rep.inventory_max_items.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    "inventory" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in CharacterSchema"
                                .to_string(),
                        )
                    }
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CharacterSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CharacterSchema {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in CharacterSchema".to_string())?,
            skin: intermediate_rep
                .skin
                .into_iter()
                .next()
                .ok_or_else(|| "skin missing in CharacterSchema".to_string())?,
            level: intermediate_rep
                .level
                .into_iter()
                .next()
                .ok_or_else(|| "level missing in CharacterSchema".to_string())?,
            xp: intermediate_rep
                .xp
                .into_iter()
                .next()
                .ok_or_else(|| "xp missing in CharacterSchema".to_string())?,
            max_xp: intermediate_rep
                .max_xp
                .into_iter()
                .next()
                .ok_or_else(|| "max_xp missing in CharacterSchema".to_string())?,
            total_xp: intermediate_rep
                .total_xp
                .into_iter()
                .next()
                .ok_or_else(|| "total_xp missing in CharacterSchema".to_string())?,
            gold: intermediate_rep
                .gold
                .into_iter()
                .next()
                .ok_or_else(|| "gold missing in CharacterSchema".to_string())?,
            speed: intermediate_rep
                .speed
                .into_iter()
                .next()
                .ok_or_else(|| "speed missing in CharacterSchema".to_string())?,
            mining_level: intermediate_rep
                .mining_level
                .into_iter()
                .next()
                .ok_or_else(|| "mining_level missing in CharacterSchema".to_string())?,
            mining_xp: intermediate_rep
                .mining_xp
                .into_iter()
                .next()
                .ok_or_else(|| "mining_xp missing in CharacterSchema".to_string())?,
            mining_max_xp: intermediate_rep
                .mining_max_xp
                .into_iter()
                .next()
                .ok_or_else(|| "mining_max_xp missing in CharacterSchema".to_string())?,
            woodcutting_level: intermediate_rep
                .woodcutting_level
                .into_iter()
                .next()
                .ok_or_else(|| "woodcutting_level missing in CharacterSchema".to_string())?,
            woodcutting_xp: intermediate_rep
                .woodcutting_xp
                .into_iter()
                .next()
                .ok_or_else(|| "woodcutting_xp missing in CharacterSchema".to_string())?,
            woodcutting_max_xp: intermediate_rep
                .woodcutting_max_xp
                .into_iter()
                .next()
                .ok_or_else(|| "woodcutting_max_xp missing in CharacterSchema".to_string())?,
            fishing_level: intermediate_rep
                .fishing_level
                .into_iter()
                .next()
                .ok_or_else(|| "fishing_level missing in CharacterSchema".to_string())?,
            fishing_xp: intermediate_rep
                .fishing_xp
                .into_iter()
                .next()
                .ok_or_else(|| "fishing_xp missing in CharacterSchema".to_string())?,
            fishing_max_xp: intermediate_rep
                .fishing_max_xp
                .into_iter()
                .next()
                .ok_or_else(|| "fishing_max_xp missing in CharacterSchema".to_string())?,
            weaponcrafting_level: intermediate_rep
                .weaponcrafting_level
                .into_iter()
                .next()
                .ok_or_else(|| "weaponcrafting_level missing in CharacterSchema".to_string())?,
            weaponcrafting_xp: intermediate_rep
                .weaponcrafting_xp
                .into_iter()
                .next()
                .ok_or_else(|| "weaponcrafting_xp missing in CharacterSchema".to_string())?,
            weaponcrafting_max_xp: intermediate_rep
                .weaponcrafting_max_xp
                .into_iter()
                .next()
                .ok_or_else(|| "weaponcrafting_max_xp missing in CharacterSchema".to_string())?,
            gearcrafting_level: intermediate_rep
                .gearcrafting_level
                .into_iter()
                .next()
                .ok_or_else(|| "gearcrafting_level missing in CharacterSchema".to_string())?,
            gearcrafting_xp: intermediate_rep
                .gearcrafting_xp
                .into_iter()
                .next()
                .ok_or_else(|| "gearcrafting_xp missing in CharacterSchema".to_string())?,
            gearcrafting_max_xp: intermediate_rep
                .gearcrafting_max_xp
                .into_iter()
                .next()
                .ok_or_else(|| "gearcrafting_max_xp missing in CharacterSchema".to_string())?,
            jewelrycrafting_level: intermediate_rep
                .jewelrycrafting_level
                .into_iter()
                .next()
                .ok_or_else(|| "jewelrycrafting_level missing in CharacterSchema".to_string())?,
            jewelrycrafting_xp: intermediate_rep
                .jewelrycrafting_xp
                .into_iter()
                .next()
                .ok_or_else(|| "jewelrycrafting_xp missing in CharacterSchema".to_string())?,
            jewelrycrafting_max_xp: intermediate_rep
                .jewelrycrafting_max_xp
                .into_iter()
                .next()
                .ok_or_else(|| "jewelrycrafting_max_xp missing in CharacterSchema".to_string())?,
            cooking_level: intermediate_rep
                .cooking_level
                .into_iter()
                .next()
                .ok_or_else(|| "cooking_level missing in CharacterSchema".to_string())?,
            cooking_xp: intermediate_rep
                .cooking_xp
                .into_iter()
                .next()
                .ok_or_else(|| "cooking_xp missing in CharacterSchema".to_string())?,
            cooking_max_xp: intermediate_rep
                .cooking_max_xp
                .into_iter()
                .next()
                .ok_or_else(|| "cooking_max_xp missing in CharacterSchema".to_string())?,
            hp: intermediate_rep
                .hp
                .into_iter()
                .next()
                .ok_or_else(|| "hp missing in CharacterSchema".to_string())?,
            haste: intermediate_rep
                .haste
                .into_iter()
                .next()
                .ok_or_else(|| "haste missing in CharacterSchema".to_string())?,
            critical_strike: intermediate_rep
                .critical_strike
                .into_iter()
                .next()
                .ok_or_else(|| "critical_strike missing in CharacterSchema".to_string())?,
            stamina: intermediate_rep
                .stamina
                .into_iter()
                .next()
                .ok_or_else(|| "stamina missing in CharacterSchema".to_string())?,
            attack_fire: intermediate_rep
                .attack_fire
                .into_iter()
                .next()
                .ok_or_else(|| "attack_fire missing in CharacterSchema".to_string())?,
            attack_earth: intermediate_rep
                .attack_earth
                .into_iter()
                .next()
                .ok_or_else(|| "attack_earth missing in CharacterSchema".to_string())?,
            attack_water: intermediate_rep
                .attack_water
                .into_iter()
                .next()
                .ok_or_else(|| "attack_water missing in CharacterSchema".to_string())?,
            attack_air: intermediate_rep
                .attack_air
                .into_iter()
                .next()
                .ok_or_else(|| "attack_air missing in CharacterSchema".to_string())?,
            dmg_fire: intermediate_rep
                .dmg_fire
                .into_iter()
                .next()
                .ok_or_else(|| "dmg_fire missing in CharacterSchema".to_string())?,
            dmg_earth: intermediate_rep
                .dmg_earth
                .into_iter()
                .next()
                .ok_or_else(|| "dmg_earth missing in CharacterSchema".to_string())?,
            dmg_water: intermediate_rep
                .dmg_water
                .into_iter()
                .next()
                .ok_or_else(|| "dmg_water missing in CharacterSchema".to_string())?,
            dmg_air: intermediate_rep
                .dmg_air
                .into_iter()
                .next()
                .ok_or_else(|| "dmg_air missing in CharacterSchema".to_string())?,
            res_fire: intermediate_rep
                .res_fire
                .into_iter()
                .next()
                .ok_or_else(|| "res_fire missing in CharacterSchema".to_string())?,
            res_earth: intermediate_rep
                .res_earth
                .into_iter()
                .next()
                .ok_or_else(|| "res_earth missing in CharacterSchema".to_string())?,
            res_water: intermediate_rep
                .res_water
                .into_iter()
                .next()
                .ok_or_else(|| "res_water missing in CharacterSchema".to_string())?,
            res_air: intermediate_rep
                .res_air
                .into_iter()
                .next()
                .ok_or_else(|| "res_air missing in CharacterSchema".to_string())?,
            x: intermediate_rep
                .x
                .into_iter()
                .next()
                .ok_or_else(|| "x missing in CharacterSchema".to_string())?,
            y: intermediate_rep
                .y
                .into_iter()
                .next()
                .ok_or_else(|| "y missing in CharacterSchema".to_string())?,
            cooldown: intermediate_rep
                .cooldown
                .into_iter()
                .next()
                .ok_or_else(|| "cooldown missing in CharacterSchema".to_string())?,
            cooldown_expiration: intermediate_rep.cooldown_expiration.into_iter().next(),
            weapon_slot: intermediate_rep
                .weapon_slot
                .into_iter()
                .next()
                .ok_or_else(|| "weapon_slot missing in CharacterSchema".to_string())?,
            shield_slot: intermediate_rep
                .shield_slot
                .into_iter()
                .next()
                .ok_or_else(|| "shield_slot missing in CharacterSchema".to_string())?,
            helmet_slot: intermediate_rep
                .helmet_slot
                .into_iter()
                .next()
                .ok_or_else(|| "helmet_slot missing in CharacterSchema".to_string())?,
            body_armor_slot: intermediate_rep
                .body_armor_slot
                .into_iter()
                .next()
                .ok_or_else(|| "body_armor_slot missing in CharacterSchema".to_string())?,
            leg_armor_slot: intermediate_rep
                .leg_armor_slot
                .into_iter()
                .next()
                .ok_or_else(|| "leg_armor_slot missing in CharacterSchema".to_string())?,
            boots_slot: intermediate_rep
                .boots_slot
                .into_iter()
                .next()
                .ok_or_else(|| "boots_slot missing in CharacterSchema".to_string())?,
            ring1_slot: intermediate_rep
                .ring1_slot
                .into_iter()
                .next()
                .ok_or_else(|| "ring1_slot missing in CharacterSchema".to_string())?,
            ring2_slot: intermediate_rep
                .ring2_slot
                .into_iter()
                .next()
                .ok_or_else(|| "ring2_slot missing in CharacterSchema".to_string())?,
            amulet_slot: intermediate_rep
                .amulet_slot
                .into_iter()
                .next()
                .ok_or_else(|| "amulet_slot missing in CharacterSchema".to_string())?,
            artifact1_slot: intermediate_rep
                .artifact1_slot
                .into_iter()
                .next()
                .ok_or_else(|| "artifact1_slot missing in CharacterSchema".to_string())?,
            artifact2_slot: intermediate_rep
                .artifact2_slot
                .into_iter()
                .next()
                .ok_or_else(|| "artifact2_slot missing in CharacterSchema".to_string())?,
            artifact3_slot: intermediate_rep
                .artifact3_slot
                .into_iter()
                .next()
                .ok_or_else(|| "artifact3_slot missing in CharacterSchema".to_string())?,
            consumable1_slot: intermediate_rep
                .consumable1_slot
                .into_iter()
                .next()
                .ok_or_else(|| "consumable1_slot missing in CharacterSchema".to_string())?,
            consumable1_slot_quantity: intermediate_rep
                .consumable1_slot_quantity
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "consumable1_slot_quantity missing in CharacterSchema".to_string()
                })?,
            consumable2_slot: intermediate_rep
                .consumable2_slot
                .into_iter()
                .next()
                .ok_or_else(|| "consumable2_slot missing in CharacterSchema".to_string())?,
            consumable2_slot_quantity: intermediate_rep
                .consumable2_slot_quantity
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "consumable2_slot_quantity missing in CharacterSchema".to_string()
                })?,
            task: intermediate_rep
                .task
                .into_iter()
                .next()
                .ok_or_else(|| "task missing in CharacterSchema".to_string())?,
            task_type: intermediate_rep
                .task_type
                .into_iter()
                .next()
                .ok_or_else(|| "task_type missing in CharacterSchema".to_string())?,
            task_progress: intermediate_rep
                .task_progress
                .into_iter()
                .next()
                .ok_or_else(|| "task_progress missing in CharacterSchema".to_string())?,
            task_total: intermediate_rep
                .task_total
                .into_iter()
                .next()
                .ok_or_else(|| "task_total missing in CharacterSchema".to_string())?,
            inventory_max_items: intermediate_rep
                .inventory_max_items
                .into_iter()
                .next()
                .ok_or_else(|| "inventory_max_items missing in CharacterSchema".to_string())?,
            inventory: intermediate_rep.inventory.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CharacterSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CharacterSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CharacterSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for CharacterSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<CharacterSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CharacterSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into CharacterSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CooldownSchema {
    /// The total seconds of the cooldown.
    #[serde(rename = "total_seconds")]
    pub total_seconds: i32,

    /// The remaining seconds of the cooldown.
    #[serde(rename = "remaining_seconds")]
    pub remaining_seconds: i32,

    /// The start of the cooldown.
    #[serde(rename = "started_at")]
    pub started_at: chrono::DateTime<chrono::Utc>,

    /// The expiration of the cooldown.
    #[serde(rename = "expiration")]
    pub expiration: chrono::DateTime<chrono::Utc>,

    /// The expiration of the cooldown.
    #[serde(rename = "cooldown_expiration")]
    pub cooldown_expiration: chrono::DateTime<chrono::Utc>,

    /// The reason of the cooldown.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "reason")]
    pub reason: String,
}

impl CooldownSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        total_seconds: i32,
        remaining_seconds: i32,
        started_at: chrono::DateTime<chrono::Utc>,
        expiration: chrono::DateTime<chrono::Utc>,
        cooldown_expiration: chrono::DateTime<chrono::Utc>,
        reason: String,
    ) -> CooldownSchema {
        CooldownSchema {
            total_seconds,
            remaining_seconds,
            started_at,
            expiration,
            reason,
            cooldown_expiration,
        }
    }
}

/// Converts the CooldownSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CooldownSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("total_seconds".to_string()),
            Some(self.total_seconds.to_string()),
            Some("remaining_seconds".to_string()),
            Some(self.remaining_seconds.to_string()),
            // Skipping started_at in query parameter serialization

            // Skipping expiration in query parameter serialization
            Some("reason".to_string()),
            Some(self.reason.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CooldownSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CooldownSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub total_seconds: Vec<i32>,
            pub remaining_seconds: Vec<i32>,
            pub started_at: Vec<chrono::DateTime<chrono::Utc>>,
            pub expiration: Vec<chrono::DateTime<chrono::Utc>>,
            pub cooldown_expiration: Vec<chrono::DateTime<chrono::Utc>>,
            pub reason: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CooldownSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "total_seconds" => intermediate_rep.total_seconds.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "remaining_seconds" => intermediate_rep.remaining_seconds.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "started_at" => intermediate_rep.started_at.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "expiration" => intermediate_rep.expiration.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "reason" => intermediate_rep.reason.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CooldownSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CooldownSchema {
            total_seconds: intermediate_rep
                .total_seconds
                .into_iter()
                .next()
                .ok_or_else(|| "total_seconds missing in CooldownSchema".to_string())?,
            remaining_seconds: intermediate_rep
                .remaining_seconds
                .into_iter()
                .next()
                .ok_or_else(|| "remaining_seconds missing in CooldownSchema".to_string())?,
            started_at: intermediate_rep
                .started_at
                .into_iter()
                .next()
                .ok_or_else(|| "started_at missing in CooldownSchema".to_string())?,
            expiration: intermediate_rep
                .expiration
                .into_iter()
                .next()
                .ok_or_else(|| "expiration missing in CooldownSchema".to_string())?,
            cooldown_expiration: intermediate_rep
                .cooldown_expiration
                .into_iter()
                .next()
                .ok_or_else(|| "cooldown expiration missing in CooldownSchema".to_string())?,
            reason: intermediate_rep
                .reason
                .into_iter()
                .next()
                .ok_or_else(|| "reason missing in CooldownSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CooldownSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CooldownSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CooldownSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for CooldownSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CooldownSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CooldownSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into CooldownSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CraftSchema {
    /// Skill required to craft the item.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "skill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill: Option<String>,

    /// The skill level required to craft the item.
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,

    /// List of items required to craft the item.
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::SimpleItemSchema>>,

    /// Quantity of items crafted.
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

impl CraftSchema {
    #[allow(clippy::new_without_default)]
    pub fn new() -> CraftSchema {
        CraftSchema {
            skill: None,
            level: None,
            items: None,
            quantity: None,
        }
    }
}

/// Converts the CraftSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CraftSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            self.skill
                .as_ref()
                .map(|skill| ["skill".to_string(), skill.to_string()].join(",")),
            self.level
                .as_ref()
                .map(|level| ["level".to_string(), level.to_string()].join(",")),
            // Skipping items in query parameter serialization
            self.quantity
                .as_ref()
                .map(|quantity| ["quantity".to_string(), quantity.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CraftSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CraftSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub skill: Vec<String>,
            pub level: Vec<i32>,
            pub items: Vec<Vec<models::SimpleItemSchema>>,
            pub quantity: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CraftSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "skill" => intermediate_rep.skill.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "level" => intermediate_rep.level.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    "items" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in CraftSchema"
                                .to_string(),
                        )
                    }
                    #[allow(clippy::redundant_clone)]
                    "quantity" => intermediate_rep.quantity.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CraftSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CraftSchema {
            skill: intermediate_rep.skill.into_iter().next(),
            level: intermediate_rep.level.into_iter().next(),
            items: intermediate_rep.items.into_iter().next(),
            quantity: intermediate_rep.quantity.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CraftSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CraftSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CraftSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for CraftSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CraftSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CraftSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into CraftSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CraftingSchema {
    /// Craft code.
    #[serde(rename = "code")]
    #[validate(regex = "RE_CRAFTINGSCHEMA_CODE")]
    pub code: String,

    /// Quantity of items to craft.
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
}

lazy_static::lazy_static! {
    static ref RE_CRAFTINGSCHEMA_CODE: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
}

impl CraftingSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(code: String) -> CraftingSchema {
        CraftingSchema {
            code,
            quantity: Some(1),
        }
    }
}

/// Converts the CraftingSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CraftingSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("code".to_string()),
            Some(self.code.to_string()),
            self.quantity
                .as_ref()
                .map(|quantity| ["quantity".to_string(), quantity.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CraftingSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CraftingSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<String>,
            pub quantity: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing CraftingSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "quantity" => intermediate_rep.quantity.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing CraftingSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CraftingSchema {
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in CraftingSchema".to_string())?,
            quantity: intermediate_rep.quantity.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CraftingSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CraftingSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<CraftingSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for CraftingSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CraftingSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <CraftingSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into CraftingSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataPageActiveEventSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::ActiveEventSchema>,

    #[serde(rename = "total")]
    pub total: swagger::Nullable<u32>,

    #[serde(rename = "page")]
    pub page: swagger::Nullable<u32>,

    #[serde(rename = "size")]
    pub size: swagger::Nullable<u32>,

    #[serde(rename = "pages")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<swagger::Nullable<u32>>,
}

impl DataPageActiveEventSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        data: Vec<models::ActiveEventSchema>,
        total: swagger::Nullable<u32>,
        page: swagger::Nullable<u32>,
        size: swagger::Nullable<u32>,
    ) -> DataPageActiveEventSchema {
        DataPageActiveEventSchema {
            data,
            total,
            page,
            size,
            pages: None,
        }
    }
}

/// Converts the DataPageActiveEventSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DataPageActiveEventSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization
            Some("total".to_string()),
            Some(
                self.total
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("page".to_string()),
            Some(
                self.page
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("size".to_string()),
            Some(
                self.size
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            self.pages.as_ref().map(|pages| {
                [
                    "pages".to_string(),
                    pages.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DataPageActiveEventSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DataPageActiveEventSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::ActiveEventSchema>>,
            pub total: Vec<u32>,
            pub page: Vec<u32>,
            pub size: Vec<u32>,
            pub pages: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DataPageActiveEventSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in DataPageActiveEventSchema".to_string()),
                    "total" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageActiveEventSchema".to_string()),
                    "page" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageActiveEventSchema".to_string()),
                    "size" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageActiveEventSchema".to_string()),
                    "pages" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageActiveEventSchema".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing DataPageActiveEventSchema".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DataPageActiveEventSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in DataPageActiveEventSchema".to_string())?,
            total: std::result::Result::Err(
                "Nullable types not supported in DataPageActiveEventSchema".to_string(),
            )?,
            page: std::result::Result::Err(
                "Nullable types not supported in DataPageActiveEventSchema".to_string(),
            )?,
            size: std::result::Result::Err(
                "Nullable types not supported in DataPageActiveEventSchema".to_string(),
            )?,
            pages: std::result::Result::Err(
                "Nullable types not supported in DataPageActiveEventSchema".to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DataPageActiveEventSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DataPageActiveEventSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DataPageActiveEventSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DataPageActiveEventSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DataPageActiveEventSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DataPageActiveEventSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DataPageActiveEventSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataPageCharacterSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::CharacterSchema>,

    #[serde(rename = "total")]
    pub total: swagger::Nullable<u32>,

    #[serde(rename = "page")]
    pub page: swagger::Nullable<u32>,

    #[serde(rename = "size")]
    pub size: swagger::Nullable<u32>,

    #[serde(rename = "pages")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<swagger::Nullable<u32>>,
}

impl DataPageCharacterSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        data: Vec<models::CharacterSchema>,
        total: swagger::Nullable<u32>,
        page: swagger::Nullable<u32>,
        size: swagger::Nullable<u32>,
    ) -> DataPageCharacterSchema {
        DataPageCharacterSchema {
            data,
            total,
            page,
            size,
            pages: None,
        }
    }
}

/// Converts the DataPageCharacterSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DataPageCharacterSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization
            Some("total".to_string()),
            Some(
                self.total
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("page".to_string()),
            Some(
                self.page
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("size".to_string()),
            Some(
                self.size
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            self.pages.as_ref().map(|pages| {
                [
                    "pages".to_string(),
                    pages.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DataPageCharacterSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DataPageCharacterSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::CharacterSchema>>,
            pub total: Vec<u32>,
            pub page: Vec<u32>,
            pub size: Vec<u32>,
            pub pages: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DataPageCharacterSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in DataPageCharacterSchema".to_string()),
                    "total" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageCharacterSchema".to_string()),
                    "page" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageCharacterSchema".to_string()),
                    "size" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageCharacterSchema".to_string()),
                    "pages" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageCharacterSchema".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing DataPageCharacterSchema".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DataPageCharacterSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in DataPageCharacterSchema".to_string())?,
            total: std::result::Result::Err(
                "Nullable types not supported in DataPageCharacterSchema".to_string(),
            )?,
            page: std::result::Result::Err(
                "Nullable types not supported in DataPageCharacterSchema".to_string(),
            )?,
            size: std::result::Result::Err(
                "Nullable types not supported in DataPageCharacterSchema".to_string(),
            )?,
            pages: std::result::Result::Err(
                "Nullable types not supported in DataPageCharacterSchema".to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DataPageCharacterSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DataPageCharacterSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DataPageCharacterSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DataPageCharacterSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DataPageCharacterSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DataPageCharacterSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DataPageCharacterSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataPageGeItemSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::GeItemSchema>,

    #[serde(rename = "total")]
    pub total: swagger::Nullable<u32>,

    #[serde(rename = "page")]
    pub page: swagger::Nullable<u32>,

    #[serde(rename = "size")]
    pub size: swagger::Nullable<u32>,

    #[serde(rename = "pages")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<swagger::Nullable<u32>>,
}

impl DataPageGeItemSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        data: Vec<models::GeItemSchema>,
        total: swagger::Nullable<u32>,
        page: swagger::Nullable<u32>,
        size: swagger::Nullable<u32>,
    ) -> DataPageGeItemSchema {
        DataPageGeItemSchema {
            data,
            total,
            page,
            size,
            pages: None,
        }
    }
}

/// Converts the DataPageGeItemSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DataPageGeItemSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization
            Some("total".to_string()),
            Some(
                self.total
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("page".to_string()),
            Some(
                self.page
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("size".to_string()),
            Some(
                self.size
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            self.pages.as_ref().map(|pages| {
                [
                    "pages".to_string(),
                    pages.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DataPageGeItemSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DataPageGeItemSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::GeItemSchema>>,
            pub total: Vec<u32>,
            pub page: Vec<u32>,
            pub size: Vec<u32>,
            pub pages: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DataPageGeItemSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in DataPageGeItemSchema".to_string()),
                    "total" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageGeItemSchema".to_string()),
                    "page" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageGeItemSchema".to_string()),
                    "size" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageGeItemSchema".to_string()),
                    "pages" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageGeItemSchema".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing DataPageGeItemSchema".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DataPageGeItemSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in DataPageGeItemSchema".to_string())?,
            total: std::result::Result::Err(
                "Nullable types not supported in DataPageGeItemSchema".to_string(),
            )?,
            page: std::result::Result::Err(
                "Nullable types not supported in DataPageGeItemSchema".to_string(),
            )?,
            size: std::result::Result::Err(
                "Nullable types not supported in DataPageGeItemSchema".to_string(),
            )?,
            pages: std::result::Result::Err(
                "Nullable types not supported in DataPageGeItemSchema".to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DataPageGeItemSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DataPageGeItemSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DataPageGeItemSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DataPageGeItemSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DataPageGeItemSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DataPageGeItemSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DataPageGeItemSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataPageItemSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::ItemSchema>,

    #[serde(rename = "total")]
    pub total: swagger::Nullable<u32>,

    #[serde(rename = "page")]
    pub page: swagger::Nullable<u32>,

    #[serde(rename = "size")]
    pub size: swagger::Nullable<u32>,

    #[serde(rename = "pages")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<swagger::Nullable<u32>>,
}

impl DataPageItemSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        data: Vec<models::ItemSchema>,
        total: swagger::Nullable<u32>,
        page: swagger::Nullable<u32>,
        size: swagger::Nullable<u32>,
    ) -> DataPageItemSchema {
        DataPageItemSchema {
            data,
            total,
            page,
            size,
            pages: None,
        }
    }
}

/// Converts the DataPageItemSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DataPageItemSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization
            Some("total".to_string()),
            Some(
                self.total
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("page".to_string()),
            Some(
                self.page
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("size".to_string()),
            Some(
                self.size
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            self.pages.as_ref().map(|pages| {
                [
                    "pages".to_string(),
                    pages.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DataPageItemSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DataPageItemSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::ItemSchema>>,
            pub total: Vec<u32>,
            pub page: Vec<u32>,
            pub size: Vec<u32>,
            pub pages: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DataPageItemSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in DataPageItemSchema".to_string()),
                    "total" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageItemSchema".to_string()),
                    "page" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageItemSchema".to_string()),
                    "size" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageItemSchema".to_string()),
                    "pages" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageItemSchema".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing DataPageItemSchema".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DataPageItemSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in DataPageItemSchema".to_string())?,
            total: std::result::Result::Err(
                "Nullable types not supported in DataPageItemSchema".to_string(),
            )?,
            page: std::result::Result::Err(
                "Nullable types not supported in DataPageItemSchema".to_string(),
            )?,
            size: std::result::Result::Err(
                "Nullable types not supported in DataPageItemSchema".to_string(),
            )?,
            pages: std::result::Result::Err(
                "Nullable types not supported in DataPageItemSchema".to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DataPageItemSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DataPageItemSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DataPageItemSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DataPageItemSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DataPageItemSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DataPageItemSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DataPageItemSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataPageLogSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::LogSchema>,

    #[serde(rename = "total")]
    pub total: swagger::Nullable<u32>,

    #[serde(rename = "page")]
    pub page: swagger::Nullable<u32>,

    #[serde(rename = "size")]
    pub size: swagger::Nullable<u32>,

    #[serde(rename = "pages")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<swagger::Nullable<u32>>,
}

impl DataPageLogSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        data: Vec<models::LogSchema>,
        total: swagger::Nullable<u32>,
        page: swagger::Nullable<u32>,
        size: swagger::Nullable<u32>,
    ) -> DataPageLogSchema {
        DataPageLogSchema {
            data,
            total,
            page,
            size,
            pages: None,
        }
    }
}

/// Converts the DataPageLogSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DataPageLogSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization
            Some("total".to_string()),
            Some(
                self.total
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("page".to_string()),
            Some(
                self.page
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("size".to_string()),
            Some(
                self.size
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            self.pages.as_ref().map(|pages| {
                [
                    "pages".to_string(),
                    pages.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DataPageLogSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DataPageLogSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::LogSchema>>,
            pub total: Vec<u32>,
            pub page: Vec<u32>,
            pub size: Vec<u32>,
            pub pages: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DataPageLogSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in DataPageLogSchema".to_string()),
                    "total" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageLogSchema".to_string()),
                    "page" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageLogSchema".to_string()),
                    "size" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageLogSchema".to_string()),
                    "pages" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageLogSchema".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing DataPageLogSchema".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DataPageLogSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in DataPageLogSchema".to_string())?,
            total: std::result::Result::Err(
                "Nullable types not supported in DataPageLogSchema".to_string(),
            )?,
            page: std::result::Result::Err(
                "Nullable types not supported in DataPageLogSchema".to_string(),
            )?,
            size: std::result::Result::Err(
                "Nullable types not supported in DataPageLogSchema".to_string(),
            )?,
            pages: std::result::Result::Err(
                "Nullable types not supported in DataPageLogSchema".to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DataPageLogSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DataPageLogSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DataPageLogSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DataPageLogSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DataPageLogSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DataPageLogSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DataPageLogSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataPageMapSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::MapSchema>,

    #[serde(rename = "total")]
    pub total: swagger::Nullable<u32>,

    #[serde(rename = "page")]
    pub page: swagger::Nullable<u32>,

    #[serde(rename = "size")]
    pub size: swagger::Nullable<u32>,

    #[serde(rename = "pages")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<swagger::Nullable<u32>>,
}

impl DataPageMapSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        data: Vec<models::MapSchema>,
        total: swagger::Nullable<u32>,
        page: swagger::Nullable<u32>,
        size: swagger::Nullable<u32>,
    ) -> DataPageMapSchema {
        DataPageMapSchema {
            data,
            total,
            page,
            size,
            pages: None,
        }
    }
}

/// Converts the DataPageMapSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DataPageMapSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization
            Some("total".to_string()),
            Some(
                self.total
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("page".to_string()),
            Some(
                self.page
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("size".to_string()),
            Some(
                self.size
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            self.pages.as_ref().map(|pages| {
                [
                    "pages".to_string(),
                    pages.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DataPageMapSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DataPageMapSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::MapSchema>>,
            pub total: Vec<u32>,
            pub page: Vec<u32>,
            pub size: Vec<u32>,
            pub pages: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DataPageMapSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in DataPageMapSchema".to_string()),
                    "total" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageMapSchema".to_string()),
                    "page" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageMapSchema".to_string()),
                    "size" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageMapSchema".to_string()),
                    "pages" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageMapSchema".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing DataPageMapSchema".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DataPageMapSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in DataPageMapSchema".to_string())?,
            total: std::result::Result::Err(
                "Nullable types not supported in DataPageMapSchema".to_string(),
            )?,
            page: std::result::Result::Err(
                "Nullable types not supported in DataPageMapSchema".to_string(),
            )?,
            size: std::result::Result::Err(
                "Nullable types not supported in DataPageMapSchema".to_string(),
            )?,
            pages: std::result::Result::Err(
                "Nullable types not supported in DataPageMapSchema".to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DataPageMapSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DataPageMapSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DataPageMapSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DataPageMapSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DataPageMapSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DataPageMapSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DataPageMapSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataPageMonsterSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::MonsterSchema>,

    #[serde(rename = "total")]
    pub total: swagger::Nullable<u32>,

    #[serde(rename = "page")]
    pub page: swagger::Nullable<u32>,

    #[serde(rename = "size")]
    pub size: swagger::Nullable<u32>,

    #[serde(rename = "pages")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<swagger::Nullable<u32>>,
}

impl DataPageMonsterSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        data: Vec<models::MonsterSchema>,
        total: swagger::Nullable<u32>,
        page: swagger::Nullable<u32>,
        size: swagger::Nullable<u32>,
    ) -> DataPageMonsterSchema {
        DataPageMonsterSchema {
            data,
            total,
            page,
            size,
            pages: None,
        }
    }
}

/// Converts the DataPageMonsterSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DataPageMonsterSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization
            Some("total".to_string()),
            Some(
                self.total
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("page".to_string()),
            Some(
                self.page
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("size".to_string()),
            Some(
                self.size
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            self.pages.as_ref().map(|pages| {
                [
                    "pages".to_string(),
                    pages.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DataPageMonsterSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DataPageMonsterSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::MonsterSchema>>,
            pub total: Vec<u32>,
            pub page: Vec<u32>,
            pub size: Vec<u32>,
            pub pages: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DataPageMonsterSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in DataPageMonsterSchema".to_string()),
                    "total" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageMonsterSchema".to_string()),
                    "page" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageMonsterSchema".to_string()),
                    "size" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageMonsterSchema".to_string()),
                    "pages" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageMonsterSchema".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing DataPageMonsterSchema".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DataPageMonsterSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in DataPageMonsterSchema".to_string())?,
            total: std::result::Result::Err(
                "Nullable types not supported in DataPageMonsterSchema".to_string(),
            )?,
            page: std::result::Result::Err(
                "Nullable types not supported in DataPageMonsterSchema".to_string(),
            )?,
            size: std::result::Result::Err(
                "Nullable types not supported in DataPageMonsterSchema".to_string(),
            )?,
            pages: std::result::Result::Err(
                "Nullable types not supported in DataPageMonsterSchema".to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DataPageMonsterSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DataPageMonsterSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DataPageMonsterSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DataPageMonsterSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DataPageMonsterSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DataPageMonsterSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DataPageMonsterSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataPageResourceSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::ResourceSchema>,

    #[serde(rename = "total")]
    pub total: swagger::Nullable<u32>,

    #[serde(rename = "page")]
    pub page: swagger::Nullable<u32>,

    #[serde(rename = "size")]
    pub size: swagger::Nullable<u32>,

    #[serde(rename = "pages")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<swagger::Nullable<u32>>,
}

impl DataPageResourceSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        data: Vec<models::ResourceSchema>,
        total: swagger::Nullable<u32>,
        page: swagger::Nullable<u32>,
        size: swagger::Nullable<u32>,
    ) -> DataPageResourceSchema {
        DataPageResourceSchema {
            data,
            total,
            page,
            size,
            pages: None,
        }
    }
}

/// Converts the DataPageResourceSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DataPageResourceSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization
            Some("total".to_string()),
            Some(
                self.total
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("page".to_string()),
            Some(
                self.page
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("size".to_string()),
            Some(
                self.size
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            self.pages.as_ref().map(|pages| {
                [
                    "pages".to_string(),
                    pages.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DataPageResourceSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DataPageResourceSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::ResourceSchema>>,
            pub total: Vec<u32>,
            pub page: Vec<u32>,
            pub size: Vec<u32>,
            pub pages: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DataPageResourceSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in DataPageResourceSchema".to_string()),
                    "total" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageResourceSchema".to_string()),
                    "page" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageResourceSchema".to_string()),
                    "size" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageResourceSchema".to_string()),
                    "pages" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageResourceSchema".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing DataPageResourceSchema".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DataPageResourceSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in DataPageResourceSchema".to_string())?,
            total: std::result::Result::Err(
                "Nullable types not supported in DataPageResourceSchema".to_string(),
            )?,
            page: std::result::Result::Err(
                "Nullable types not supported in DataPageResourceSchema".to_string(),
            )?,
            size: std::result::Result::Err(
                "Nullable types not supported in DataPageResourceSchema".to_string(),
            )?,
            pages: std::result::Result::Err(
                "Nullable types not supported in DataPageResourceSchema".to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DataPageResourceSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DataPageResourceSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DataPageResourceSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DataPageResourceSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DataPageResourceSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DataPageResourceSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DataPageResourceSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataPageSimpleItemSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::SimpleItemSchema>,

    #[serde(rename = "total")]
    pub total: swagger::Nullable<u32>,

    #[serde(rename = "page")]
    pub page: swagger::Nullable<u32>,

    #[serde(rename = "size")]
    pub size: swagger::Nullable<u32>,

    #[serde(rename = "pages")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<swagger::Nullable<u32>>,
}

impl DataPageSimpleItemSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        data: Vec<models::SimpleItemSchema>,
        total: swagger::Nullable<u32>,
        page: swagger::Nullable<u32>,
        size: swagger::Nullable<u32>,
    ) -> DataPageSimpleItemSchema {
        DataPageSimpleItemSchema {
            data,
            total,
            page,
            size,
            pages: None,
        }
    }
}

/// Converts the DataPageSimpleItemSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DataPageSimpleItemSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization
            Some("total".to_string()),
            Some(
                self.total
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("page".to_string()),
            Some(
                self.page
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            Some("size".to_string()),
            Some(
                self.size
                    .as_ref()
                    .map_or("null".to_string(), |x| x.to_string()),
            ),
            self.pages.as_ref().map(|pages| {
                [
                    "pages".to_string(),
                    pages.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ]
                .join(",")
            }),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DataPageSimpleItemSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DataPageSimpleItemSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::SimpleItemSchema>>,
            pub total: Vec<u32>,
            pub page: Vec<u32>,
            pub size: Vec<u32>,
            pub pages: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DataPageSimpleItemSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in DataPageSimpleItemSchema".to_string()),
                    "total" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageSimpleItemSchema".to_string()),
                    "page" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageSimpleItemSchema".to_string()),
                    "size" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageSimpleItemSchema".to_string()),
                    "pages" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in DataPageSimpleItemSchema".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing DataPageSimpleItemSchema".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DataPageSimpleItemSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in DataPageSimpleItemSchema".to_string())?,
            total: std::result::Result::Err(
                "Nullable types not supported in DataPageSimpleItemSchema".to_string(),
            )?,
            page: std::result::Result::Err(
                "Nullable types not supported in DataPageSimpleItemSchema".to_string(),
            )?,
            size: std::result::Result::Err(
                "Nullable types not supported in DataPageSimpleItemSchema".to_string(),
            )?,
            pages: std::result::Result::Err(
                "Nullable types not supported in DataPageSimpleItemSchema".to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DataPageSimpleItemSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DataPageSimpleItemSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DataPageSimpleItemSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DataPageSimpleItemSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DataPageSimpleItemSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DataPageSimpleItemSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DataPageSimpleItemSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteCharacterSchema {
    /// Character name.
    #[serde(rename = "name")]
    #[validate(length(min = 3, max = 12), regex = "RE_DELETECHARACTERSCHEMA_NAME")]
    pub name: String,
}

lazy_static::lazy_static! {
    static ref RE_DELETECHARACTERSCHEMA_NAME: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
}

impl DeleteCharacterSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(name: String) -> DeleteCharacterSchema {
        DeleteCharacterSchema { name }
    }
}

/// Converts the DeleteCharacterSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DeleteCharacterSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> =
            vec![Some("name".to_string()), Some(self.name.to_string())];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DeleteCharacterSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DeleteCharacterSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DeleteCharacterSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing DeleteCharacterSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DeleteCharacterSchema {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in DeleteCharacterSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DeleteCharacterSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DeleteCharacterSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DeleteCharacterSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DeleteCharacterSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DeleteCharacterSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DeleteCharacterSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DeleteCharacterSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteItemResponseSchema {
    #[serde(rename = "data")]
    pub data: models::DeleteItemSchema,
}

impl DeleteItemResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::DeleteItemSchema) -> DeleteItemResponseSchema {
        DeleteItemResponseSchema { data }
    }
}

/// Converts the DeleteItemResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DeleteItemResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DeleteItemResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DeleteItemResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::DeleteItemSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DeleteItemResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::DeleteItemSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing DeleteItemResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DeleteItemResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in DeleteItemResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DeleteItemResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DeleteItemResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DeleteItemResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DeleteItemResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DeleteItemResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DeleteItemResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DeleteItemResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DeleteItemSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: models::CooldownSchema,

    /// Item details.
    #[serde(rename = "item")]
    pub item: models::SimpleItemSchema,

    /// Player details.
    #[serde(rename = "character")]
    pub character: models::CharacterSchema,
}

impl DeleteItemSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        cooldown: models::CooldownSchema,
        item: models::SimpleItemSchema,
        character: models::CharacterSchema,
    ) -> DeleteItemSchema {
        DeleteItemSchema {
            cooldown,
            item,
            character,
        }
    }
}

/// Converts the DeleteItemSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DeleteItemSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization

            // Skipping item in query parameter serialization

            // Skipping character in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DeleteItemSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DeleteItemSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::CooldownSchema>,
            pub item: Vec<models::SimpleItemSchema>,
            pub character: Vec<models::CharacterSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DeleteItemSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(
                        <models::CooldownSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "item" => intermediate_rep.item.push(
                        <models::SimpleItemSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "character" => intermediate_rep.character.push(
                        <models::CharacterSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing DeleteItemSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DeleteItemSchema {
            cooldown: intermediate_rep
                .cooldown
                .into_iter()
                .next()
                .ok_or_else(|| "cooldown missing in DeleteItemSchema".to_string())?,
            item: intermediate_rep
                .item
                .into_iter()
                .next()
                .ok_or_else(|| "item missing in DeleteItemSchema".to_string())?,
            character: intermediate_rep
                .character
                .into_iter()
                .next()
                .ok_or_else(|| "character missing in DeleteItemSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DeleteItemSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DeleteItemSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DeleteItemSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DeleteItemSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DeleteItemSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DeleteItemSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DeleteItemSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DepositWithdrawGoldSchema {
    /// Quantity of gold.
    #[serde(
        rename = "quantity",
        deserialize_with = "serde_aux::prelude::deserialize_number_from_string"
    )]
    pub quantity: u32,
}

impl DepositWithdrawGoldSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(quantity: u32) -> DepositWithdrawGoldSchema {
        DepositWithdrawGoldSchema { quantity }
    }
}

/// Converts the DepositWithdrawGoldSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DepositWithdrawGoldSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("quantity".to_string()),
            Some(self.quantity.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DepositWithdrawGoldSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DepositWithdrawGoldSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub quantity: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DepositWithdrawGoldSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "quantity" => intermediate_rep.quantity.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing DepositWithdrawGoldSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DepositWithdrawGoldSchema {
            quantity: intermediate_rep
                .quantity
                .into_iter()
                .next()
                .ok_or_else(|| "quantity missing in DepositWithdrawGoldSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DepositWithdrawGoldSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DepositWithdrawGoldSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DepositWithdrawGoldSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DepositWithdrawGoldSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DepositWithdrawGoldSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DepositWithdrawGoldSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DepositWithdrawGoldSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DestinationSchema {
    /// The x coordinate of the destination.
    #[serde(rename = "x")]
    pub x: i32,

    /// The y coordinate of the destination.
    #[serde(rename = "y")]
    pub y: i32,
}

impl DestinationSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(x: i32, y: i32) -> DestinationSchema {
        DestinationSchema { x, y }
    }
}

/// Converts the DestinationSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DestinationSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("x".to_string()),
            Some(self.x.to_string()),
            Some("y".to_string()),
            Some(self.y.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DestinationSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DestinationSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub x: Vec<i32>,
            pub y: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DestinationSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "x" => intermediate_rep.x.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "y" => intermediate_rep.y.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing DestinationSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DestinationSchema {
            x: intermediate_rep
                .x
                .into_iter()
                .next()
                .ok_or_else(|| "x missing in DestinationSchema".to_string())?,
            y: intermediate_rep
                .y
                .into_iter()
                .next()
                .ok_or_else(|| "y missing in DestinationSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DestinationSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DestinationSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DestinationSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DestinationSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<DestinationSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DestinationSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DestinationSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DropRateSchema {
    /// Item code.
    #[serde(rename = "code")]
    #[validate(regex = "RE_DROPRATESCHEMA_CODE")]
    pub code: String,

    /// Chance rate.
    #[serde(rename = "rate")]
    pub rate: u32,

    /// Minimum quantity.
    #[serde(rename = "min_quantity")]
    pub min_quantity: u32,

    /// Maximum quantity.
    #[serde(rename = "max_quantity")]
    pub max_quantity: u32,
}

lazy_static::lazy_static! {
    static ref RE_DROPRATESCHEMA_CODE: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
}

impl DropRateSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(code: String, rate: u32, min_quantity: u32, max_quantity: u32) -> DropRateSchema {
        DropRateSchema {
            code,
            rate,
            min_quantity,
            max_quantity,
        }
    }
}

/// Converts the DropRateSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DropRateSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("code".to_string()),
            Some(self.code.to_string()),
            Some("rate".to_string()),
            Some(self.rate.to_string()),
            Some("min_quantity".to_string()),
            Some(self.min_quantity.to_string()),
            Some("max_quantity".to_string()),
            Some(self.max_quantity.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DropRateSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DropRateSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<String>,
            pub rate: Vec<u32>,
            pub min_quantity: Vec<u32>,
            pub max_quantity: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DropRateSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "rate" => intermediate_rep.rate.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "min_quantity" => intermediate_rep.min_quantity.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "max_quantity" => intermediate_rep.max_quantity.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing DropRateSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DropRateSchema {
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in DropRateSchema".to_string())?,
            rate: intermediate_rep
                .rate
                .into_iter()
                .next()
                .ok_or_else(|| "rate missing in DropRateSchema".to_string())?,
            min_quantity: intermediate_rep
                .min_quantity
                .into_iter()
                .next()
                .ok_or_else(|| "min_quantity missing in DropRateSchema".to_string())?,
            max_quantity: intermediate_rep
                .max_quantity
                .into_iter()
                .next()
                .ok_or_else(|| "max_quantity missing in DropRateSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DropRateSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DropRateSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DropRateSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DropRateSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<DropRateSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DropRateSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DropRateSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DropSchema {
    /// The code of the item.
    #[serde(rename = "code")]
    pub code: String,

    /// The quantity of the item.
    #[serde(rename = "quantity")]
    pub quantity: i32,
}

impl DropSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(code: String, quantity: i32) -> DropSchema {
        DropSchema { code, quantity }
    }
}

/// Converts the DropSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DropSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("code".to_string()),
            Some(self.code.to_string()),
            Some("quantity".to_string()),
            Some(self.quantity.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DropSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DropSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<String>,
            pub quantity: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing DropSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "quantity" => intermediate_rep.quantity.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing DropSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DropSchema {
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in DropSchema".to_string())?,
            quantity: intermediate_rep
                .quantity
                .into_iter()
                .next()
                .ok_or_else(|| "quantity missing in DropSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DropSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DropSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<DropSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for DropSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<DropSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <DropSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into DropSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct EquipRequestSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: models::CooldownSchema,

    /// Item slot.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "slot")]
    pub slot: String,

    /// Item details.
    #[serde(rename = "item")]
    pub item: models::ItemSchema,

    /// Player details.
    #[serde(rename = "character")]
    pub character: models::CharacterSchema,
}

impl EquipRequestSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        cooldown: models::CooldownSchema,
        slot: String,
        item: models::ItemSchema,
        character: models::CharacterSchema,
    ) -> EquipRequestSchema {
        EquipRequestSchema {
            cooldown,
            slot,
            item,
            character,
        }
    }
}

/// Converts the EquipRequestSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for EquipRequestSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization
            Some("slot".to_string()),
            Some(self.slot.to_string()),
            // Skipping item in query parameter serialization

            // Skipping character in query parameter serialization
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a EquipRequestSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for EquipRequestSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::CooldownSchema>,
            pub slot: Vec<String>,
            pub item: Vec<models::ItemSchema>,
            pub character: Vec<models::CharacterSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing EquipRequestSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(
                        <models::CooldownSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "slot" => intermediate_rep.slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "item" => intermediate_rep.item.push(
                        <models::ItemSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "character" => intermediate_rep.character.push(
                        <models::CharacterSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing EquipRequestSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(EquipRequestSchema {
            cooldown: intermediate_rep
                .cooldown
                .into_iter()
                .next()
                .ok_or_else(|| "cooldown missing in EquipRequestSchema".to_string())?,
            slot: intermediate_rep
                .slot
                .into_iter()
                .next()
                .ok_or_else(|| "slot missing in EquipRequestSchema".to_string())?,
            item: intermediate_rep
                .item
                .into_iter()
                .next()
                .ok_or_else(|| "item missing in EquipRequestSchema".to_string())?,
            character: intermediate_rep
                .character
                .into_iter()
                .next()
                .ok_or_else(|| "character missing in EquipRequestSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<EquipRequestSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<EquipRequestSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<EquipRequestSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for EquipRequestSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<EquipRequestSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <EquipRequestSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into EquipRequestSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct EquipSchema {
    /// Item code.
    #[serde(rename = "code")]
    #[validate(regex = "RE_EQUIPSCHEMA_CODE")]
    pub code: String,

    /// Item slot.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "slot")]
    pub slot: String,
}

lazy_static::lazy_static! {
    static ref RE_EQUIPSCHEMA_CODE: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
}

impl EquipSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(code: String, slot: String) -> EquipSchema {
        EquipSchema { code, slot }
    }
}

/// Converts the EquipSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for EquipSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("code".to_string()),
            Some(self.code.to_string()),
            Some("slot".to_string()),
            Some(self.slot.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a EquipSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for EquipSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<String>,
            pub slot: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing EquipSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "slot" => intermediate_rep.slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing EquipSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(EquipSchema {
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in EquipSchema".to_string())?,
            slot: intermediate_rep
                .slot
                .into_iter()
                .next()
                .ok_or_else(|| "slot missing in EquipSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<EquipSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<EquipSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<EquipSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for EquipSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<EquipSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <EquipSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into EquipSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct EquipmentResponseSchema {
    #[serde(rename = "data")]
    pub data: models::EquipRequestSchema,
}

impl EquipmentResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::EquipRequestSchema) -> EquipmentResponseSchema {
        EquipmentResponseSchema { data }
    }
}

/// Converts the EquipmentResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for EquipmentResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a EquipmentResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for EquipmentResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::EquipRequestSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing EquipmentResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::EquipRequestSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing EquipmentResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(EquipmentResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in EquipmentResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<EquipmentResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<EquipmentResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<EquipmentResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for EquipmentResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<EquipmentResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <EquipmentResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into EquipmentResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FightSchema {
    /// The amount of xp gained by the fight.
    #[serde(rename = "xp")]
    pub xp: i32,

    /// The amount of gold gained by the fight.
    #[serde(rename = "gold")]
    pub gold: i32,

    /// The items dropped by the fight.
    #[serde(rename = "drops")]
    pub drops: Vec<models::DropSchema>,

    /// Numbers of the turns of the combat.
    #[serde(rename = "turns")]
    pub turns: i32,

    /// The amount of blocked hits by the monster.
    #[serde(rename = "monster_blocked_hits")]
    pub monster_blocked_hits: models::BlockedHitsSchema,

    /// The amount of blocked hits by the player.
    #[serde(rename = "player_blocked_hits")]
    pub player_blocked_hits: models::BlockedHitsSchema,

    /// The fight logs.
    #[serde(rename = "logs")]
    pub logs: Vec<String>,

    /// The result of the fight.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "result")]
    pub result: String,
}

impl FightSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        xp: i32,
        gold: i32,
        drops: Vec<models::DropSchema>,
        turns: i32,
        monster_blocked_hits: models::BlockedHitsSchema,
        player_blocked_hits: models::BlockedHitsSchema,
        logs: Vec<String>,
        result: String,
    ) -> FightSchema {
        FightSchema {
            xp,
            gold,
            drops,
            turns,
            monster_blocked_hits,
            player_blocked_hits,
            logs,
            result,
        }
    }
}

/// Converts the FightSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for FightSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("xp".to_string()),
            Some(self.xp.to_string()),
            Some("gold".to_string()),
            Some(self.gold.to_string()),
            // Skipping drops in query parameter serialization
            Some("turns".to_string()),
            Some(self.turns.to_string()),
            // Skipping monster_blocked_hits in query parameter serialization

            // Skipping player_blocked_hits in query parameter serialization
            Some("logs".to_string()),
            Some(
                self.logs
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            Some("result".to_string()),
            Some(self.result.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a FightSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for FightSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub xp: Vec<i32>,
            pub gold: Vec<i32>,
            pub drops: Vec<Vec<models::DropSchema>>,
            pub turns: Vec<i32>,
            pub monster_blocked_hits: Vec<models::BlockedHitsSchema>,
            pub player_blocked_hits: Vec<models::BlockedHitsSchema>,
            pub logs: Vec<Vec<String>>,
            pub result: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing FightSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "xp" => intermediate_rep.xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "gold" => intermediate_rep.gold.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    "drops" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in FightSchema"
                                .to_string(),
                        )
                    }
                    #[allow(clippy::redundant_clone)]
                    "turns" => intermediate_rep.turns.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "monster_blocked_hits" => intermediate_rep.monster_blocked_hits.push(
                        <models::BlockedHitsSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "player_blocked_hits" => intermediate_rep.player_blocked_hits.push(
                        <models::BlockedHitsSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    "logs" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in FightSchema"
                                .to_string(),
                        )
                    }
                    #[allow(clippy::redundant_clone)]
                    "result" => intermediate_rep.result.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing FightSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(FightSchema {
            xp: intermediate_rep
                .xp
                .into_iter()
                .next()
                .ok_or_else(|| "xp missing in FightSchema".to_string())?,
            gold: intermediate_rep
                .gold
                .into_iter()
                .next()
                .ok_or_else(|| "gold missing in FightSchema".to_string())?,
            drops: intermediate_rep
                .drops
                .into_iter()
                .next()
                .ok_or_else(|| "drops missing in FightSchema".to_string())?,
            turns: intermediate_rep
                .turns
                .into_iter()
                .next()
                .ok_or_else(|| "turns missing in FightSchema".to_string())?,
            monster_blocked_hits: intermediate_rep
                .monster_blocked_hits
                .into_iter()
                .next()
                .ok_or_else(|| "monster_blocked_hits missing in FightSchema".to_string())?,
            player_blocked_hits: intermediate_rep
                .player_blocked_hits
                .into_iter()
                .next()
                .ok_or_else(|| "player_blocked_hits missing in FightSchema".to_string())?,
            logs: intermediate_rep
                .logs
                .into_iter()
                .next()
                .ok_or_else(|| "logs missing in FightSchema".to_string())?,
            result: intermediate_rep
                .result
                .into_iter()
                .next()
                .ok_or_else(|| "result missing in FightSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<FightSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<FightSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<FightSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for FightSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<FightSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <FightSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into FightSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GeItemResponseSchema {
    #[serde(rename = "data")]
    pub data: models::GeItemSchema,
}

impl GeItemResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::GeItemSchema) -> GeItemResponseSchema {
        GeItemResponseSchema { data }
    }
}

/// Converts the GeItemResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GeItemResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GeItemResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GeItemResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::GeItemSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GeItemResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::GeItemSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing GeItemResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GeItemResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in GeItemResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GeItemResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GeItemResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GeItemResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for GeItemResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<GeItemResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GeItemResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into GeItemResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GeItemSchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,

    /// Item stock.
    #[serde(rename = "stock")]
    pub stock: i32,

    /// The item's selling price.
    #[serde(rename = "sell_price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_price: Option<i32>,

    /// The item's buying price.
    #[serde(rename = "buy_price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_price: Option<i32>,
}

impl GeItemSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(code: String, stock: i32) -> GeItemSchema {
        GeItemSchema {
            code,
            stock,
            sell_price: None,
            buy_price: None,
        }
    }
}

/// Converts the GeItemSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GeItemSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("code".to_string()),
            Some(self.code.to_string()),
            Some("stock".to_string()),
            Some(self.stock.to_string()),
            self.sell_price
                .as_ref()
                .map(|sell_price| ["sell_price".to_string(), sell_price.to_string()].join(",")),
            self.buy_price
                .as_ref()
                .map(|buy_price| ["buy_price".to_string(), buy_price.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GeItemSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GeItemSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<String>,
            pub stock: Vec<i32>,
            pub sell_price: Vec<i32>,
            pub buy_price: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GeItemSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "stock" => intermediate_rep.stock.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "sell_price" => intermediate_rep.sell_price.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "buy_price" => intermediate_rep.buy_price.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing GeItemSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GeItemSchema {
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in GeItemSchema".to_string())?,
            stock: intermediate_rep
                .stock
                .into_iter()
                .next()
                .ok_or_else(|| "stock missing in GeItemSchema".to_string())?,
            sell_price: intermediate_rep.sell_price.into_iter().next(),
            buy_price: intermediate_rep.buy_price.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GeItemSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GeItemSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GeItemSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for GeItemSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GeItemSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GeItemSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into GeItemSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GeTransactionItemSchema {
    /// Item code.
    #[serde(rename = "code")]
    #[validate(regex = "RE_GETRANSACTIONITEMSCHEMA_CODE")]
    pub code: String,

    /// Item quantity.
    #[serde(
        rename = "quantity",
        deserialize_with = "serde_aux::prelude::deserialize_number_from_string"
    )]
    pub quantity: u8,

    /// Item price. Item price validation protects you if the price has changed since you last checked the buy/sale price of an item.
    #[serde(rename = "price")]
    pub price: u32,
}

lazy_static::lazy_static! {
    static ref RE_GETRANSACTIONITEMSCHEMA_CODE: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
}

impl GeTransactionItemSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(code: String, quantity: u8, price: u32) -> GeTransactionItemSchema {
        GeTransactionItemSchema {
            code,
            quantity,
            price,
        }
    }
}

/// Converts the GeTransactionItemSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GeTransactionItemSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("code".to_string()),
            Some(self.code.to_string()),
            Some("quantity".to_string()),
            Some(self.quantity.to_string()),
            Some("price".to_string()),
            Some(self.price.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GeTransactionItemSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GeTransactionItemSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<String>,
            pub quantity: Vec<u8>,
            pub price: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GeTransactionItemSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "quantity" => intermediate_rep
                        .quantity
                        .push(<u8 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "price" => intermediate_rep.price.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing GeTransactionItemSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GeTransactionItemSchema {
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in GeTransactionItemSchema".to_string())?,
            quantity: intermediate_rep
                .quantity
                .into_iter()
                .next()
                .ok_or_else(|| "quantity missing in GeTransactionItemSchema".to_string())?,
            price: intermediate_rep
                .price
                .into_iter()
                .next()
                .ok_or_else(|| "price missing in GeTransactionItemSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GeTransactionItemSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GeTransactionItemSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GeTransactionItemSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for GeTransactionItemSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<GeTransactionItemSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GeTransactionItemSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into GeTransactionItemSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GeTransactionListSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: models::CooldownSchema,

    /// Transaction details.
    #[serde(rename = "transaction")]
    pub transaction: models::GeTransactionSchema,

    /// Character details.
    #[serde(rename = "character")]
    pub character: models::CharacterSchema,
}

impl GeTransactionListSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        cooldown: models::CooldownSchema,
        transaction: models::GeTransactionSchema,
        character: models::CharacterSchema,
    ) -> GeTransactionListSchema {
        GeTransactionListSchema {
            cooldown,
            transaction,
            character,
        }
    }
}

/// Converts the GeTransactionListSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GeTransactionListSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization

            // Skipping transaction in query parameter serialization

            // Skipping character in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GeTransactionListSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GeTransactionListSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::CooldownSchema>,
            pub transaction: Vec<models::GeTransactionSchema>,
            pub character: Vec<models::CharacterSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GeTransactionListSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(
                        <models::CooldownSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "transaction" => intermediate_rep.transaction.push(
                        <models::GeTransactionSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "character" => intermediate_rep.character.push(
                        <models::CharacterSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing GeTransactionListSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GeTransactionListSchema {
            cooldown: intermediate_rep
                .cooldown
                .into_iter()
                .next()
                .ok_or_else(|| "cooldown missing in GeTransactionListSchema".to_string())?,
            transaction: intermediate_rep
                .transaction
                .into_iter()
                .next()
                .ok_or_else(|| "transaction missing in GeTransactionListSchema".to_string())?,
            character: intermediate_rep
                .character
                .into_iter()
                .next()
                .ok_or_else(|| "character missing in GeTransactionListSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GeTransactionListSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GeTransactionListSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GeTransactionListSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for GeTransactionListSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<GeTransactionListSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GeTransactionListSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into GeTransactionListSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GeTransactionResponseSchema {
    #[serde(rename = "data")]
    pub data: models::GeTransactionListSchema,
}

impl GeTransactionResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::GeTransactionListSchema) -> GeTransactionResponseSchema {
        GeTransactionResponseSchema { data }
    }
}

/// Converts the GeTransactionResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GeTransactionResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GeTransactionResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GeTransactionResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::GeTransactionListSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GeTransactionResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::GeTransactionListSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing GeTransactionResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GeTransactionResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in GeTransactionResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GeTransactionResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GeTransactionResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GeTransactionResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for GeTransactionResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<GeTransactionResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GeTransactionResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into GeTransactionResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GeTransactionSchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,

    /// Item quantity.
    #[serde(rename = "quantity")]
    pub quantity: i32,

    /// Item price.
    #[serde(rename = "price")]
    pub price: i32,

    /// Total price of the transaction.
    #[serde(rename = "total_price")]
    pub total_price: i32,
}

impl GeTransactionSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(code: String, quantity: i32, price: i32, total_price: i32) -> GeTransactionSchema {
        GeTransactionSchema {
            code,
            quantity,
            price,
            total_price,
        }
    }
}

/// Converts the GeTransactionSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GeTransactionSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("code".to_string()),
            Some(self.code.to_string()),
            Some("quantity".to_string()),
            Some(self.quantity.to_string()),
            Some("price".to_string()),
            Some(self.price.to_string()),
            Some("total_price".to_string()),
            Some(self.total_price.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GeTransactionSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GeTransactionSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<String>,
            pub quantity: Vec<i32>,
            pub price: Vec<i32>,
            pub total_price: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GeTransactionSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "quantity" => intermediate_rep.quantity.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "price" => intermediate_rep.price.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "total_price" => intermediate_rep.total_price.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing GeTransactionSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GeTransactionSchema {
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in GeTransactionSchema".to_string())?,
            quantity: intermediate_rep
                .quantity
                .into_iter()
                .next()
                .ok_or_else(|| "quantity missing in GeTransactionSchema".to_string())?,
            price: intermediate_rep
                .price
                .into_iter()
                .next()
                .ok_or_else(|| "price missing in GeTransactionSchema".to_string())?,
            total_price: intermediate_rep
                .total_price
                .into_iter()
                .next()
                .ok_or_else(|| "total_price missing in GeTransactionSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GeTransactionSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GeTransactionSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GeTransactionSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for GeTransactionSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<GeTransactionSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GeTransactionSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into GeTransactionSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GoldBankResponseSchema {
    #[serde(rename = "data")]
    pub data: models::GoldSchema,
}

impl GoldBankResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::GoldSchema) -> GoldBankResponseSchema {
        GoldBankResponseSchema { data }
    }
}

/// Converts the GoldBankResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GoldBankResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GoldBankResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GoldBankResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::GoldSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GoldBankResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::GoldSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing GoldBankResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GoldBankResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in GoldBankResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GoldBankResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GoldBankResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GoldBankResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for GoldBankResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<GoldBankResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GoldBankResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into GoldBankResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GoldResponseSchema {
    #[serde(rename = "data")]
    pub data: models::GoldTransactionSchema,
}

impl GoldResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::GoldTransactionSchema) -> GoldResponseSchema {
        GoldResponseSchema { data }
    }
}

/// Converts the GoldResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GoldResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GoldResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GoldResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::GoldTransactionSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GoldResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::GoldTransactionSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing GoldResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GoldResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in GoldResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GoldResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GoldResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GoldResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for GoldResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<GoldResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GoldResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into GoldResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GoldSchema {
    /// Quantity of gold.
    #[serde(
        rename = "quantity",
        deserialize_with = "serde_aux::prelude::deserialize_number_from_string"
    )]
    pub quantity: u32,
}

impl GoldSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(quantity: u32) -> GoldSchema {
        GoldSchema { quantity }
    }
}

/// Converts the GoldSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GoldSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("quantity".to_string()),
            Some(self.quantity.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GoldSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GoldSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub quantity: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GoldSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "quantity" => intermediate_rep.quantity.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing GoldSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GoldSchema {
            quantity: intermediate_rep
                .quantity
                .into_iter()
                .next()
                .ok_or_else(|| "quantity missing in GoldSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GoldSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GoldSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GoldSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for GoldSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GoldSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GoldSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into GoldSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GoldTransactionSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: models::CooldownSchema,

    /// Bank details.
    #[serde(rename = "bank")]
    pub bank: models::GoldSchema,

    /// Player details.
    #[serde(rename = "character")]
    pub character: models::CharacterSchema,
}

impl GoldTransactionSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        cooldown: models::CooldownSchema,
        bank: models::GoldSchema,
        character: models::CharacterSchema,
    ) -> GoldTransactionSchema {
        GoldTransactionSchema {
            cooldown,
            bank,
            character,
        }
    }
}

/// Converts the GoldTransactionSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GoldTransactionSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization

            // Skipping bank in query parameter serialization

            // Skipping character in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GoldTransactionSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GoldTransactionSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::CooldownSchema>,
            pub bank: Vec<models::GoldSchema>,
            pub character: Vec<models::CharacterSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing GoldTransactionSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(
                        <models::CooldownSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "bank" => intermediate_rep.bank.push(
                        <models::GoldSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "character" => intermediate_rep.character.push(
                        <models::CharacterSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing GoldTransactionSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GoldTransactionSchema {
            cooldown: intermediate_rep
                .cooldown
                .into_iter()
                .next()
                .ok_or_else(|| "cooldown missing in GoldTransactionSchema".to_string())?,
            bank: intermediate_rep
                .bank
                .into_iter()
                .next()
                .ok_or_else(|| "bank missing in GoldTransactionSchema".to_string())?,
            character: intermediate_rep
                .character
                .into_iter()
                .next()
                .ok_or_else(|| "character missing in GoldTransactionSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<GoldTransactionSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GoldTransactionSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<GoldTransactionSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for GoldTransactionSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<GoldTransactionSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <GoldTransactionSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into GoldTransactionSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct HttpValidationError {
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Vec<models::ValidationError>>,
}

impl HttpValidationError {
    #[allow(clippy::new_without_default)]
    pub fn new() -> HttpValidationError {
        HttpValidationError { detail: None }
    }
}

/// Converts the HttpValidationError value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for HttpValidationError {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping detail in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a HttpValidationError value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for HttpValidationError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub detail: Vec<Vec<models::ValidationError>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing HttpValidationError".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "detail" => return std::result::Result::Err(
                        "Parsing a container in this style is not supported in HttpValidationError"
                            .to_string(),
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing HttpValidationError".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(HttpValidationError {
            detail: intermediate_rep.detail.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<HttpValidationError> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<HttpValidationError>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<HttpValidationError>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for HttpValidationError - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<HttpValidationError>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <HttpValidationError as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into HttpValidationError - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InventorySlot {
    /// Inventory slot identifier.
    #[serde(rename = "slot")]
    pub slot: i32,

    /// Item code.
    #[serde(rename = "code")]
    pub code: String,

    /// Quantity in the slot.
    #[serde(rename = "quantity")]
    pub quantity: i32,
}

impl InventorySlot {
    #[allow(clippy::new_without_default)]
    pub fn new(slot: i32, code: String, quantity: i32) -> InventorySlot {
        InventorySlot {
            slot,
            code,
            quantity,
        }
    }
}

/// Converts the InventorySlot value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InventorySlot {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("slot".to_string()),
            Some(self.slot.to_string()),
            Some("code".to_string()),
            Some(self.code.to_string()),
            Some("quantity".to_string()),
            Some(self.quantity.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InventorySlot value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InventorySlot {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub slot: Vec<i32>,
            pub code: Vec<String>,
            pub quantity: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing InventorySlot".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "slot" => intermediate_rep.slot.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "quantity" => intermediate_rep.quantity.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing InventorySlot".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InventorySlot {
            slot: intermediate_rep
                .slot
                .into_iter()
                .next()
                .ok_or_else(|| "slot missing in InventorySlot".to_string())?,
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in InventorySlot".to_string())?,
            quantity: intermediate_rep
                .quantity
                .into_iter()
                .next()
                .ok_or_else(|| "quantity missing in InventorySlot".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InventorySlot> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InventorySlot>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<InventorySlot>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for InventorySlot - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<InventorySlot> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <InventorySlot as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into InventorySlot - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ItemEffectSchema {
    /// Effect name.
    #[serde(rename = "name")]
    pub name: String,

    /// Effect value.
    #[serde(rename = "value")]
    pub value: i32,
}

impl ItemEffectSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(name: String, value: i32) -> ItemEffectSchema {
        ItemEffectSchema { name, value }
    }
}

/// Converts the ItemEffectSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ItemEffectSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("value".to_string()),
            Some(self.value.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ItemEffectSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ItemEffectSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub value: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ItemEffectSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ItemEffectSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ItemEffectSchema {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in ItemEffectSchema".to_string())?,
            value: intermediate_rep
                .value
                .into_iter()
                .next()
                .ok_or_else(|| "value missing in ItemEffectSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ItemEffectSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ItemEffectSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ItemEffectSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ItemEffectSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<ItemEffectSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ItemEffectSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into ItemEffectSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ItemResponseSchema {
    #[serde(rename = "data")]
    pub data: models::SingleItemSchema,
}

impl ItemResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::SingleItemSchema) -> ItemResponseSchema {
        ItemResponseSchema { data }
    }
}

/// Converts the ItemResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ItemResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ItemResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ItemResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::SingleItemSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ItemResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::SingleItemSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ItemResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ItemResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in ItemResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ItemResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ItemResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ItemResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ItemResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<ItemResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ItemResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into ItemResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ItemSchema {
    /// Item name.
    #[serde(rename = "name")]
    pub name: String,

    /// Item code. This is the item's unique identifier (ID).
    #[serde(rename = "code")]
    pub code: String,

    /// Item level.
    #[serde(rename = "level")]
    pub level: u32,

    /// Item type.
    #[serde(rename = "type")]
    pub r#type: String,

    /// Item subtype.
    #[serde(rename = "subtype")]
    pub subtype: String,

    /// Item description.
    #[serde(rename = "description")]
    pub description: String,

    /// List of object effects. For equipment, it will include item stats.
    #[serde(rename = "effects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<Vec<models::ItemEffectSchema>>,

    #[serde(rename = "craft")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub craft: Option<swagger::Nullable<models::CraftSchema>>,
}

impl ItemSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        name: String,
        code: String,
        level: u32,
        r#type: String,
        subtype: String,
        description: String,
    ) -> ItemSchema {
        ItemSchema {
            name,
            code,
            level,
            r#type,
            subtype,
            description,
            effects: None,
            craft: None,
        }
    }
}

/// Converts the ItemSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ItemSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("code".to_string()),
            Some(self.code.to_string()),
            Some("level".to_string()),
            Some(self.level.to_string()),
            Some("type".to_string()),
            Some(self.r#type.to_string()),
            Some("subtype".to_string()),
            Some(self.subtype.to_string()),
            Some("description".to_string()),
            Some(self.description.to_string()),
            // Skipping effects in query parameter serialization

            // Skipping craft in query parameter serialization
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ItemSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ItemSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub code: Vec<String>,
            pub level: Vec<u32>,
            pub r#type: Vec<String>,
            pub subtype: Vec<String>,
            pub description: Vec<String>,
            pub effects: Vec<Vec<models::ItemEffectSchema>>,
            pub craft: Vec<models::CraftSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ItemSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "level" => intermediate_rep.level.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "subtype" => intermediate_rep.subtype.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    "effects" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in ItemSchema"
                                .to_string(),
                        )
                    }
                    "craft" => {
                        return std::result::Result::Err(
                            "Parsing a nullable type in this style is not supported in ItemSchema"
                                .to_string(),
                        )
                    }
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ItemSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ItemSchema {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in ItemSchema".to_string())?,
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in ItemSchema".to_string())?,
            level: intermediate_rep
                .level
                .into_iter()
                .next()
                .ok_or_else(|| "level missing in ItemSchema".to_string())?,
            r#type: intermediate_rep
                .r#type
                .into_iter()
                .next()
                .ok_or_else(|| "type missing in ItemSchema".to_string())?,
            subtype: intermediate_rep
                .subtype
                .into_iter()
                .next()
                .ok_or_else(|| "subtype missing in ItemSchema".to_string())?,
            description: intermediate_rep
                .description
                .into_iter()
                .next()
                .ok_or_else(|| "description missing in ItemSchema".to_string())?,
            effects: intermediate_rep.effects.into_iter().next(),
            craft: std::result::Result::Err(
                "Nullable types not supported in ItemSchema".to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ItemSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ItemSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ItemSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ItemSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ItemSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ItemSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into ItemSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LogSchema {
    /// Character name.
    #[serde(rename = "character")]
    pub character: String,

    /// Account character.
    #[serde(rename = "account")]
    pub account: String,

    /// Type of action.
    #[serde(rename = "type")]
    pub r#type: String,

    /// Description of action.
    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "content")]
    pub content: serde_json::Value,

    /// Cooldown in seconds.
    #[serde(rename = "cooldown")]
    pub cooldown: i32,

    /// Datetime of cooldown expiration.
    #[serde(rename = "cooldown_expiration")]
    pub cooldown_expiration: chrono::DateTime<chrono::Utc>,

    /// Datetime of creation.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl LogSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        character: String,
        account: String,
        r#type: String,
        description: String,
        content: serde_json::Value,
        cooldown: i32,
        cooldown_expiration: chrono::DateTime<chrono::Utc>,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> LogSchema {
        LogSchema {
            character,
            account,
            r#type,
            description,
            content,
            cooldown,
            cooldown_expiration,
            created_at,
        }
    }
}

/// Converts the LogSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LogSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("character".to_string()),
            Some(self.character.to_string()),
            Some("account".to_string()),
            Some(self.account.to_string()),
            Some("type".to_string()),
            Some(self.r#type.to_string()),
            Some("description".to_string()),
            Some(self.description.to_string()),
            // Skipping content in query parameter serialization
            Some("cooldown".to_string()),
            Some(self.cooldown.to_string()),
            // Skipping cooldown_expiration in query parameter serialization

            // Skipping created_at in query parameter serialization
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LogSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LogSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub character: Vec<String>,
            pub account: Vec<String>,
            pub r#type: Vec<String>,
            pub description: Vec<String>,
            pub content: Vec<serde_json::Value>,
            pub cooldown: Vec<i32>,
            pub cooldown_expiration: Vec<chrono::DateTime<chrono::Utc>>,
            pub created_at: Vec<chrono::DateTime<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing LogSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "character" => intermediate_rep.character.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "account" => intermediate_rep.account.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "content" => intermediate_rep.content.push(
                        <serde_json::Value as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "cooldown_expiration" => intermediate_rep.cooldown_expiration.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "created_at" => intermediate_rep.created_at.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing LogSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LogSchema {
            character: intermediate_rep
                .character
                .into_iter()
                .next()
                .ok_or_else(|| "character missing in LogSchema".to_string())?,
            account: intermediate_rep
                .account
                .into_iter()
                .next()
                .ok_or_else(|| "account missing in LogSchema".to_string())?,
            r#type: intermediate_rep
                .r#type
                .into_iter()
                .next()
                .ok_or_else(|| "type missing in LogSchema".to_string())?,
            description: intermediate_rep
                .description
                .into_iter()
                .next()
                .ok_or_else(|| "description missing in LogSchema".to_string())?,
            content: intermediate_rep
                .content
                .into_iter()
                .next()
                .ok_or_else(|| "content missing in LogSchema".to_string())?,
            cooldown: intermediate_rep
                .cooldown
                .into_iter()
                .next()
                .ok_or_else(|| "cooldown missing in LogSchema".to_string())?,
            cooldown_expiration: intermediate_rep
                .cooldown_expiration
                .into_iter()
                .next()
                .ok_or_else(|| "cooldown_expiration missing in LogSchema".to_string())?,
            created_at: intermediate_rep
                .created_at
                .into_iter()
                .next()
                .ok_or_else(|| "created_at missing in LogSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LogSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LogSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<LogSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for LogSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LogSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <LogSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into LogSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MapContentSchema {
    /// Type of the content.
    #[serde(rename = "type")]
    pub r#type: String,

    /// Code of the content.
    #[serde(rename = "code")]
    pub code: String,
}

impl MapContentSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, code: String) -> MapContentSchema {
        MapContentSchema { r#type, code }
    }
}

/// Converts the MapContentSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MapContentSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("type".to_string()),
            Some(self.r#type.to_string()),
            Some("code".to_string()),
            Some(self.code.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MapContentSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MapContentSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub code: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing MapContentSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing MapContentSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MapContentSchema {
            r#type: intermediate_rep
                .r#type
                .into_iter()
                .next()
                .ok_or_else(|| "type missing in MapContentSchema".to_string())?,
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in MapContentSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MapContentSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MapContentSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<MapContentSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for MapContentSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<MapContentSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <MapContentSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into MapContentSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MapResponseSchema {
    #[serde(rename = "data")]
    pub data: models::MapSchema,
}

impl MapResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::MapSchema) -> MapResponseSchema {
        MapResponseSchema { data }
    }
}

/// Converts the MapResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MapResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MapResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MapResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::MapSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing MapResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::MapSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing MapResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MapResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in MapResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MapResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MapResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<MapResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for MapResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<MapResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <MapResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into MapResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MapSchema {
    /// Name of the map.
    #[serde(rename = "name")]
    pub name: String,

    /// Skin of the map.
    #[serde(rename = "skin")]
    pub skin: String,

    /// Position X of the map.
    #[serde(rename = "x")]
    pub x: i32,

    /// Position Y of the map.
    #[serde(rename = "y")]
    pub y: i32,

    #[serde(rename = "content")]
    pub content: swagger::Nullable<models::MapContentSchema>,
}

impl MapSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        name: String,
        skin: String,
        x: i32,
        y: i32,
        content: swagger::Nullable<models::MapContentSchema>,
    ) -> MapSchema {
        MapSchema {
            name,
            skin,
            x,
            y,
            content,
        }
    }
}

/// Converts the MapSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MapSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("skin".to_string()),
            Some(self.skin.to_string()),
            Some("x".to_string()),
            Some(self.x.to_string()),
            Some("y".to_string()),
            Some(self.y.to_string()),
            // Skipping content in query parameter serialization
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MapSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MapSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub skin: Vec<String>,
            pub x: Vec<i32>,
            pub y: Vec<i32>,
            pub content: Vec<models::MapContentSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing MapSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "skin" => intermediate_rep.skin.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "x" => intermediate_rep.x.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "y" => intermediate_rep.y.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    "content" => {
                        return std::result::Result::Err(
                            "Parsing a nullable type in this style is not supported in MapSchema"
                                .to_string(),
                        )
                    }
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing MapSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MapSchema {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in MapSchema".to_string())?,
            skin: intermediate_rep
                .skin
                .into_iter()
                .next()
                .ok_or_else(|| "skin missing in MapSchema".to_string())?,
            x: intermediate_rep
                .x
                .into_iter()
                .next()
                .ok_or_else(|| "x missing in MapSchema".to_string())?,
            y: intermediate_rep
                .y
                .into_iter()
                .next()
                .ok_or_else(|| "y missing in MapSchema".to_string())?,
            content: std::result::Result::Err(
                "Nullable types not supported in MapSchema".to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MapSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MapSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<MapSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for MapSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MapSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <MapSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into MapSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MonsterResponseSchema {
    #[serde(rename = "data")]
    pub data: models::MonsterSchema,
}

impl MonsterResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::MonsterSchema) -> MonsterResponseSchema {
        MonsterResponseSchema { data }
    }
}

/// Converts the MonsterResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MonsterResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MonsterResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MonsterResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::MonsterSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing MonsterResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::MonsterSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing MonsterResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MonsterResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in MonsterResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MonsterResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MonsterResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<MonsterResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for MonsterResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<MonsterResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <MonsterResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into MonsterResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MonsterSchema {
    /// Name of the monster.
    #[serde(rename = "name")]
    pub name: String,

    /// The code of the monster. This is the monster's unique identifier (ID).
    #[serde(rename = "code")]
    pub code: String,

    /// Monster level.
    #[serde(rename = "level")]
    pub level: i32,

    /// Monster hit points.
    #[serde(rename = "hp")]
    pub hp: i32,

    /// Monster fire attack.
    #[serde(rename = "attack_fire")]
    pub attack_fire: i32,

    /// Monster earth attack.
    #[serde(rename = "attack_earth")]
    pub attack_earth: i32,

    /// Monster water attack.
    #[serde(rename = "attack_water")]
    pub attack_water: i32,

    /// Monster air attack.
    #[serde(rename = "attack_air")]
    pub attack_air: i32,

    /// Monster % fire resistance.
    #[serde(rename = "res_fire")]
    pub res_fire: i32,

    /// Monster % earth resistance.
    #[serde(rename = "res_earth")]
    pub res_earth: i32,

    /// Monster % water resistance.
    #[serde(rename = "res_water")]
    pub res_water: i32,

    /// Monster % air resistance.
    #[serde(rename = "res_air")]
    pub res_air: i32,

    /// Monster minimum gold drop.
    #[serde(rename = "min_gold")]
    pub min_gold: i32,

    /// Monster maximum gold drop.
    #[serde(rename = "max_gold")]
    pub max_gold: i32,

    /// Monster drops. This is a list of items that the monster drops after killing the monster.
    #[serde(rename = "drops")]
    pub drops: Vec<models::DropRateSchema>,
}

impl MonsterSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        name: String,
        code: String,
        level: i32,
        hp: i32,
        attack_fire: i32,
        attack_earth: i32,
        attack_water: i32,
        attack_air: i32,
        res_fire: i32,
        res_earth: i32,
        res_water: i32,
        res_air: i32,
        min_gold: i32,
        max_gold: i32,
        drops: Vec<models::DropRateSchema>,
    ) -> MonsterSchema {
        MonsterSchema {
            name,
            code,
            level,
            hp,
            attack_fire,
            attack_earth,
            attack_water,
            attack_air,
            res_fire,
            res_earth,
            res_water,
            res_air,
            min_gold,
            max_gold,
            drops,
        }
    }
}

/// Converts the MonsterSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MonsterSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("code".to_string()),
            Some(self.code.to_string()),
            Some("level".to_string()),
            Some(self.level.to_string()),
            Some("hp".to_string()),
            Some(self.hp.to_string()),
            Some("attack_fire".to_string()),
            Some(self.attack_fire.to_string()),
            Some("attack_earth".to_string()),
            Some(self.attack_earth.to_string()),
            Some("attack_water".to_string()),
            Some(self.attack_water.to_string()),
            Some("attack_air".to_string()),
            Some(self.attack_air.to_string()),
            Some("res_fire".to_string()),
            Some(self.res_fire.to_string()),
            Some("res_earth".to_string()),
            Some(self.res_earth.to_string()),
            Some("res_water".to_string()),
            Some(self.res_water.to_string()),
            Some("res_air".to_string()),
            Some(self.res_air.to_string()),
            Some("min_gold".to_string()),
            Some(self.min_gold.to_string()),
            Some("max_gold".to_string()),
            Some(self.max_gold.to_string()),
            // Skipping drops in query parameter serialization
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MonsterSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MonsterSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub code: Vec<String>,
            pub level: Vec<i32>,
            pub hp: Vec<i32>,
            pub attack_fire: Vec<i32>,
            pub attack_earth: Vec<i32>,
            pub attack_water: Vec<i32>,
            pub attack_air: Vec<i32>,
            pub res_fire: Vec<i32>,
            pub res_earth: Vec<i32>,
            pub res_water: Vec<i32>,
            pub res_air: Vec<i32>,
            pub min_gold: Vec<i32>,
            pub max_gold: Vec<i32>,
            pub drops: Vec<Vec<models::DropRateSchema>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing MonsterSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "level" => intermediate_rep.level.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "hp" => intermediate_rep.hp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "attack_fire" => intermediate_rep.attack_fire.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "attack_earth" => intermediate_rep.attack_earth.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "attack_water" => intermediate_rep.attack_water.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "attack_air" => intermediate_rep.attack_air.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "res_fire" => intermediate_rep.res_fire.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "res_earth" => intermediate_rep.res_earth.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "res_water" => intermediate_rep.res_water.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "res_air" => intermediate_rep.res_air.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "min_gold" => intermediate_rep.min_gold.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "max_gold" => intermediate_rep.max_gold.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    "drops" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in MonsterSchema"
                                .to_string(),
                        )
                    }
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing MonsterSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MonsterSchema {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in MonsterSchema".to_string())?,
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in MonsterSchema".to_string())?,
            level: intermediate_rep
                .level
                .into_iter()
                .next()
                .ok_or_else(|| "level missing in MonsterSchema".to_string())?,
            hp: intermediate_rep
                .hp
                .into_iter()
                .next()
                .ok_or_else(|| "hp missing in MonsterSchema".to_string())?,
            attack_fire: intermediate_rep
                .attack_fire
                .into_iter()
                .next()
                .ok_or_else(|| "attack_fire missing in MonsterSchema".to_string())?,
            attack_earth: intermediate_rep
                .attack_earth
                .into_iter()
                .next()
                .ok_or_else(|| "attack_earth missing in MonsterSchema".to_string())?,
            attack_water: intermediate_rep
                .attack_water
                .into_iter()
                .next()
                .ok_or_else(|| "attack_water missing in MonsterSchema".to_string())?,
            attack_air: intermediate_rep
                .attack_air
                .into_iter()
                .next()
                .ok_or_else(|| "attack_air missing in MonsterSchema".to_string())?,
            res_fire: intermediate_rep
                .res_fire
                .into_iter()
                .next()
                .ok_or_else(|| "res_fire missing in MonsterSchema".to_string())?,
            res_earth: intermediate_rep
                .res_earth
                .into_iter()
                .next()
                .ok_or_else(|| "res_earth missing in MonsterSchema".to_string())?,
            res_water: intermediate_rep
                .res_water
                .into_iter()
                .next()
                .ok_or_else(|| "res_water missing in MonsterSchema".to_string())?,
            res_air: intermediate_rep
                .res_air
                .into_iter()
                .next()
                .ok_or_else(|| "res_air missing in MonsterSchema".to_string())?,
            min_gold: intermediate_rep
                .min_gold
                .into_iter()
                .next()
                .ok_or_else(|| "min_gold missing in MonsterSchema".to_string())?,
            max_gold: intermediate_rep
                .max_gold
                .into_iter()
                .next()
                .ok_or_else(|| "max_gold missing in MonsterSchema".to_string())?,
            drops: intermediate_rep
                .drops
                .into_iter()
                .next()
                .ok_or_else(|| "drops missing in MonsterSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MonsterSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MonsterSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<MonsterSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for MonsterSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MonsterSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <MonsterSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into MonsterSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MyCharactersListSchema {
    /// List of your characters.
    #[serde(rename = "data")]
    pub data: Vec<models::CharacterSchema>,
}

impl MyCharactersListSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: Vec<models::CharacterSchema>) -> MyCharactersListSchema {
        MyCharactersListSchema { data }
    }
}

/// Converts the MyCharactersListSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MyCharactersListSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MyCharactersListSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MyCharactersListSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::CharacterSchema>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing MyCharactersListSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in MyCharactersListSchema".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing MyCharactersListSchema".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MyCharactersListSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in MyCharactersListSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MyCharactersListSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MyCharactersListSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<MyCharactersListSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for MyCharactersListSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<MyCharactersListSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <MyCharactersListSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into MyCharactersListSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RecyclingDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: models::CooldownSchema,

    /// Craft details.
    #[serde(rename = "details")]
    pub details: models::RecyclingItemsSchema,

    /// Player details.
    #[serde(rename = "character")]
    pub character: models::CharacterSchema,
}

impl RecyclingDataSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        cooldown: models::CooldownSchema,
        details: models::RecyclingItemsSchema,
        character: models::CharacterSchema,
    ) -> RecyclingDataSchema {
        RecyclingDataSchema {
            cooldown,
            details,
            character,
        }
    }
}

/// Converts the RecyclingDataSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RecyclingDataSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization

            // Skipping details in query parameter serialization

            // Skipping character in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RecyclingDataSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RecyclingDataSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::CooldownSchema>,
            pub details: Vec<models::RecyclingItemsSchema>,
            pub character: Vec<models::CharacterSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing RecyclingDataSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(
                        <models::CooldownSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "details" => intermediate_rep.details.push(
                        <models::RecyclingItemsSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "character" => intermediate_rep.character.push(
                        <models::CharacterSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing RecyclingDataSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RecyclingDataSchema {
            cooldown: intermediate_rep
                .cooldown
                .into_iter()
                .next()
                .ok_or_else(|| "cooldown missing in RecyclingDataSchema".to_string())?,
            details: intermediate_rep
                .details
                .into_iter()
                .next()
                .ok_or_else(|| "details missing in RecyclingDataSchema".to_string())?,
            character: intermediate_rep
                .character
                .into_iter()
                .next()
                .ok_or_else(|| "character missing in RecyclingDataSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RecyclingDataSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RecyclingDataSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<RecyclingDataSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for RecyclingDataSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<RecyclingDataSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <RecyclingDataSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into RecyclingDataSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RecyclingItemsSchema {
    /// Objects received.
    #[serde(rename = "items")]
    pub items: Vec<models::DropSchema>,
}

impl RecyclingItemsSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(items: Vec<models::DropSchema>) -> RecyclingItemsSchema {
        RecyclingItemsSchema { items }
    }
}

/// Converts the RecyclingItemsSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RecyclingItemsSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping items in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RecyclingItemsSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RecyclingItemsSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub items: Vec<Vec<models::DropSchema>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing RecyclingItemsSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "items" => return std::result::Result::Err("Parsing a container in this style is not supported in RecyclingItemsSchema".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing RecyclingItemsSchema".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RecyclingItemsSchema {
            items: intermediate_rep
                .items
                .into_iter()
                .next()
                .ok_or_else(|| "items missing in RecyclingItemsSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RecyclingItemsSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RecyclingItemsSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<RecyclingItemsSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for RecyclingItemsSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<RecyclingItemsSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <RecyclingItemsSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into RecyclingItemsSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RecyclingResponseSchema {
    #[serde(rename = "data")]
    pub data: models::RecyclingDataSchema,
}

impl RecyclingResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::RecyclingDataSchema) -> RecyclingResponseSchema {
        RecyclingResponseSchema { data }
    }
}

/// Converts the RecyclingResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RecyclingResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RecyclingResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RecyclingResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::RecyclingDataSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing RecyclingResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::RecyclingDataSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing RecyclingResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RecyclingResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in RecyclingResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RecyclingResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RecyclingResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<RecyclingResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for RecyclingResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<RecyclingResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <RecyclingResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into RecyclingResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RecyclingSchema {
    /// Item code.
    #[serde(rename = "code")]
    #[validate(regex = "RE_RECYCLINGSCHEMA_CODE")]
    pub code: String,

    /// Quantity of items to recycle.
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
}

lazy_static::lazy_static! {
    static ref RE_RECYCLINGSCHEMA_CODE: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
}

impl RecyclingSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(code: String) -> RecyclingSchema {
        RecyclingSchema {
            code,
            quantity: Some(1),
        }
    }
}

/// Converts the RecyclingSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RecyclingSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("code".to_string()),
            Some(self.code.to_string()),
            self.quantity
                .as_ref()
                .map(|quantity| ["quantity".to_string(), quantity.to_string()].join(",")),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RecyclingSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RecyclingSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<String>,
            pub quantity: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing RecyclingSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "quantity" => intermediate_rep.quantity.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing RecyclingSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RecyclingSchema {
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in RecyclingSchema".to_string())?,
            quantity: intermediate_rep.quantity.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RecyclingSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RecyclingSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<RecyclingSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for RecyclingSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<RecyclingSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <RecyclingSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into RecyclingSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ResourceResponseSchema {
    #[serde(rename = "data")]
    pub data: models::ResourceSchema,
}

impl ResourceResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::ResourceSchema) -> ResourceResponseSchema {
        ResourceResponseSchema { data }
    }
}

/// Converts the ResourceResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ResourceResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ResourceResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ResourceResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::ResourceSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ResourceResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::ResourceSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ResourceResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ResourceResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in ResourceResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ResourceResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ResourceResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ResourceResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ResourceResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<ResourceResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ResourceResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into ResourceResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ResourceSchema {
    /// The name of the resource
    #[serde(rename = "name")]
    pub name: String,

    /// The code of the resource. This is the resource's unique identifier (ID).
    #[serde(rename = "code")]
    pub code: String,

    /// The skill required to gather this resource.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "skill")]
    pub skill: String,

    /// The skill level required to gather this resource.
    #[serde(rename = "level")]
    pub level: i32,

    /// The drops of this resource.
    #[serde(rename = "drops")]
    pub drops: Vec<models::DropRateSchema>,
}

impl ResourceSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        name: String,
        code: String,
        skill: String,
        level: i32,
        drops: Vec<models::DropRateSchema>,
    ) -> ResourceSchema {
        ResourceSchema {
            name,
            code,
            skill,
            level,
            drops,
        }
    }
}

/// Converts the ResourceSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ResourceSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("name".to_string()),
            Some(self.name.to_string()),
            Some("code".to_string()),
            Some(self.code.to_string()),
            Some("skill".to_string()),
            Some(self.skill.to_string()),
            Some("level".to_string()),
            Some(self.level.to_string()),
            // Skipping drops in query parameter serialization
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ResourceSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ResourceSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub code: Vec<String>,
            pub skill: Vec<String>,
            pub level: Vec<i32>,
            pub drops: Vec<Vec<models::DropRateSchema>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ResourceSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "skill" => intermediate_rep.skill.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "level" => intermediate_rep.level.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    "drops" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in ResourceSchema"
                                .to_string(),
                        )
                    }
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ResourceSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ResourceSchema {
            name: intermediate_rep
                .name
                .into_iter()
                .next()
                .ok_or_else(|| "name missing in ResourceSchema".to_string())?,
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in ResourceSchema".to_string())?,
            skill: intermediate_rep
                .skill
                .into_iter()
                .next()
                .ok_or_else(|| "skill missing in ResourceSchema".to_string())?,
            level: intermediate_rep
                .level
                .into_iter()
                .next()
                .ok_or_else(|| "level missing in ResourceSchema".to_string())?,
            drops: intermediate_rep
                .drops
                .into_iter()
                .next()
                .ok_or_else(|| "drops missing in ResourceSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ResourceSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ResourceSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ResourceSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ResourceSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ResourceSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ResourceSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into ResourceSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ResponseSchema {
    #[serde(rename = "message")]
    pub message: String,
}

impl ResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(message: String) -> ResponseSchema {
        ResponseSchema { message }
    }
}

/// Converts the ResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> =
            vec![Some("message".to_string()), Some(self.message.to_string())];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ResponseSchema {
            message: intermediate_rep
                .message
                .into_iter()
                .next()
                .ok_or_else(|| "message missing in ResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ResponseSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ResponseSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into ResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SimpleItemSchema {
    /// Item code.
    #[serde(rename = "code")]
    #[validate(regex = "RE_SIMPLEITEMSCHEMA_CODE")]
    pub code: String,

    /// Item quantity.
    #[serde(
        rename = "quantity",
        deserialize_with = "serde_aux::prelude::deserialize_number_from_string"
    )]
    pub quantity: u32,
}

lazy_static::lazy_static! {
    static ref RE_SIMPLEITEMSCHEMA_CODE: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
}

impl SimpleItemSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(code: String, quantity: u32) -> SimpleItemSchema {
        SimpleItemSchema { code, quantity }
    }
}

/// Converts the SimpleItemSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SimpleItemSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("code".to_string()),
            Some(self.code.to_string()),
            Some("quantity".to_string()),
            Some(self.quantity.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SimpleItemSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SimpleItemSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<String>,
            pub quantity: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing SimpleItemSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "quantity" => intermediate_rep.quantity.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing SimpleItemSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SimpleItemSchema {
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in SimpleItemSchema".to_string())?,
            quantity: intermediate_rep
                .quantity
                .into_iter()
                .next()
                .ok_or_else(|| "quantity missing in SimpleItemSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SimpleItemSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SimpleItemSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<SimpleItemSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for SimpleItemSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<SimpleItemSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <SimpleItemSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into SimpleItemSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SingleItemSchema {
    /// Item information.
    #[serde(rename = "item")]
    pub item: models::ItemSchema,

    #[serde(rename = "ge")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ge: Option<swagger::Nullable<models::GeItemSchema>>,
}

impl SingleItemSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(item: models::ItemSchema) -> SingleItemSchema {
        SingleItemSchema { item, ge: None }
    }
}

/// Converts the SingleItemSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SingleItemSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping item in query parameter serialization

            // Skipping ge in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SingleItemSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SingleItemSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub item: Vec<models::ItemSchema>,
            pub ge: Vec<models::GeItemSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing SingleItemSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "item" => intermediate_rep.item.push(<models::ItemSchema as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "ge" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in SingleItemSchema".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing SingleItemSchema".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SingleItemSchema {
            item: intermediate_rep
                .item
                .into_iter()
                .next()
                .ok_or_else(|| "item missing in SingleItemSchema".to_string())?,
            ge: std::result::Result::Err(
                "Nullable types not supported in SingleItemSchema".to_string(),
            )?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SingleItemSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SingleItemSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<SingleItemSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for SingleItemSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<SingleItemSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <SingleItemSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into SingleItemSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SkillDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: models::CooldownSchema,

    /// Craft details.
    #[serde(rename = "details")]
    pub details: models::SkillInfoSchema,

    /// Player details.
    #[serde(rename = "character")]
    pub character: models::CharacterSchema,
}

impl SkillDataSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        cooldown: models::CooldownSchema,
        details: models::SkillInfoSchema,
        character: models::CharacterSchema,
    ) -> SkillDataSchema {
        SkillDataSchema {
            cooldown,
            details,
            character,
        }
    }
}

/// Converts the SkillDataSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SkillDataSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization

            // Skipping details in query parameter serialization

            // Skipping character in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SkillDataSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SkillDataSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::CooldownSchema>,
            pub details: Vec<models::SkillInfoSchema>,
            pub character: Vec<models::CharacterSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing SkillDataSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(
                        <models::CooldownSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "details" => intermediate_rep.details.push(
                        <models::SkillInfoSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "character" => intermediate_rep.character.push(
                        <models::CharacterSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing SkillDataSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SkillDataSchema {
            cooldown: intermediate_rep
                .cooldown
                .into_iter()
                .next()
                .ok_or_else(|| "cooldown missing in SkillDataSchema".to_string())?,
            details: intermediate_rep
                .details
                .into_iter()
                .next()
                .ok_or_else(|| "details missing in SkillDataSchema".to_string())?,
            character: intermediate_rep
                .character
                .into_iter()
                .next()
                .ok_or_else(|| "character missing in SkillDataSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SkillDataSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SkillDataSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<SkillDataSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for SkillDataSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<SkillDataSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <SkillDataSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into SkillDataSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SkillInfoSchema {
    /// The amount of xp gained.
    #[serde(rename = "xp")]
    pub xp: i32,

    /// Objects received.
    #[serde(rename = "items")]
    pub items: Vec<models::DropSchema>,
}

impl SkillInfoSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(xp: i32, items: Vec<models::DropSchema>) -> SkillInfoSchema {
        SkillInfoSchema { xp, items }
    }
}

/// Converts the SkillInfoSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SkillInfoSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("xp".to_string()),
            Some(self.xp.to_string()),
            // Skipping items in query parameter serialization
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SkillInfoSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SkillInfoSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub xp: Vec<i32>,
            pub items: Vec<Vec<models::DropSchema>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing SkillInfoSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "xp" => intermediate_rep.xp.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    "items" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in SkillInfoSchema"
                                .to_string(),
                        )
                    }
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing SkillInfoSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SkillInfoSchema {
            xp: intermediate_rep
                .xp
                .into_iter()
                .next()
                .ok_or_else(|| "xp missing in SkillInfoSchema".to_string())?,
            items: intermediate_rep
                .items
                .into_iter()
                .next()
                .ok_or_else(|| "items missing in SkillInfoSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SkillInfoSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SkillInfoSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<SkillInfoSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for SkillInfoSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<SkillInfoSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <SkillInfoSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into SkillInfoSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SkillResponseSchema {
    #[serde(rename = "data")]
    pub data: models::SkillDataSchema,
}

impl SkillResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::SkillDataSchema) -> SkillResponseSchema {
        SkillResponseSchema { data }
    }
}

/// Converts the SkillResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SkillResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SkillResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SkillResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::SkillDataSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing SkillResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::SkillDataSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing SkillResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SkillResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in SkillResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SkillResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SkillResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<SkillResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for SkillResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<SkillResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <SkillResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into SkillResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct StatusResponseSchema {
    #[serde(rename = "data")]
    pub data: models::StatusSchema,
}

impl StatusResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::StatusSchema) -> StatusResponseSchema {
        StatusResponseSchema { data }
    }
}

/// Converts the StatusResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for StatusResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a StatusResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for StatusResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::StatusSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing StatusResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::StatusSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing StatusResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(StatusResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in StatusResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<StatusResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<StatusResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<StatusResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for StatusResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<StatusResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <StatusResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into StatusResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct StatusSchema {
    /// Server status
    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "characters_online")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters_online: Option<i32>,

    #[serde(rename = "server_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_time: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "announcements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announcements: Option<Vec<models::AnnouncementSchema>>,

    /// Last server wipe.
    #[serde(rename = "last_wipe")]
    pub last_wipe: String,

    /// Next server wipe.
    #[serde(rename = "next_wipe")]
    pub next_wipe: String,
}

impl StatusSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(status: String, last_wipe: String, next_wipe: String) -> StatusSchema {
        StatusSchema {
            status,
            version: None,
            characters_online: None,
            server_time: None,
            announcements: None,
            last_wipe,
            next_wipe,
        }
    }
}

/// Converts the StatusSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for StatusSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("status".to_string()),
            Some(self.status.to_string()),
            self.version
                .as_ref()
                .map(|version| ["version".to_string(), version.to_string()].join(",")),
            self.characters_online.as_ref().map(|characters_online| {
                [
                    "characters_online".to_string(),
                    characters_online.to_string(),
                ]
                .join(",")
            }),
            // Skipping server_time in query parameter serialization

            // Skipping announcements in query parameter serialization
            Some("last_wipe".to_string()),
            Some(self.last_wipe.to_string()),
            Some("next_wipe".to_string()),
            Some(self.next_wipe.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a StatusSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for StatusSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub status: Vec<String>,
            pub version: Vec<String>,
            pub characters_online: Vec<i32>,
            pub server_time: Vec<chrono::DateTime<chrono::Utc>>,
            pub announcements: Vec<Vec<models::AnnouncementSchema>>,
            pub last_wipe: Vec<String>,
            pub next_wipe: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing StatusSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "version" => intermediate_rep.version.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "characters_online" => intermediate_rep.characters_online.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "server_time" => intermediate_rep.server_time.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    "announcements" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in StatusSchema"
                                .to_string(),
                        )
                    }
                    #[allow(clippy::redundant_clone)]
                    "last_wipe" => intermediate_rep.last_wipe.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "next_wipe" => intermediate_rep.next_wipe.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing StatusSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(StatusSchema {
            status: intermediate_rep
                .status
                .into_iter()
                .next()
                .ok_or_else(|| "status missing in StatusSchema".to_string())?,
            version: intermediate_rep.version.into_iter().next(),
            characters_online: intermediate_rep.characters_online.into_iter().next(),
            server_time: intermediate_rep.server_time.into_iter().next(),
            announcements: intermediate_rep.announcements.into_iter().next(),
            last_wipe: intermediate_rep
                .last_wipe
                .into_iter()
                .next()
                .ok_or_else(|| "last_wipe missing in StatusSchema".to_string())?,
            next_wipe: intermediate_rep
                .next_wipe
                .into_iter()
                .next()
                .ok_or_else(|| "next_wipe missing in StatusSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<StatusSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<StatusSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<StatusSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for StatusSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<StatusSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <StatusSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into StatusSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TaskDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: models::CooldownSchema,

    /// Task details.
    #[serde(rename = "task")]
    pub task: models::TaskSchema,

    /// Player details.
    #[serde(rename = "character")]
    pub character: models::CharacterSchema,
}

impl TaskDataSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        cooldown: models::CooldownSchema,
        task: models::TaskSchema,
        character: models::CharacterSchema,
    ) -> TaskDataSchema {
        TaskDataSchema {
            cooldown,
            task,
            character,
        }
    }
}

/// Converts the TaskDataSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TaskDataSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization

            // Skipping task in query parameter serialization

            // Skipping character in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TaskDataSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TaskDataSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::CooldownSchema>,
            pub task: Vec<models::TaskSchema>,
            pub character: Vec<models::CharacterSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing TaskDataSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(
                        <models::CooldownSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "task" => intermediate_rep.task.push(
                        <models::TaskSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "character" => intermediate_rep.character.push(
                        <models::CharacterSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing TaskDataSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TaskDataSchema {
            cooldown: intermediate_rep
                .cooldown
                .into_iter()
                .next()
                .ok_or_else(|| "cooldown missing in TaskDataSchema".to_string())?,
            task: intermediate_rep
                .task
                .into_iter()
                .next()
                .ok_or_else(|| "task missing in TaskDataSchema".to_string())?,
            character: intermediate_rep
                .character
                .into_iter()
                .next()
                .ok_or_else(|| "character missing in TaskDataSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TaskDataSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TaskDataSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<TaskDataSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for TaskDataSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TaskDataSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <TaskDataSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into TaskDataSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TaskResponseSchema {
    #[serde(rename = "data")]
    pub data: models::TaskDataSchema,
}

impl TaskResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::TaskDataSchema) -> TaskResponseSchema {
        TaskResponseSchema { data }
    }
}

/// Converts the TaskResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TaskResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TaskResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TaskResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::TaskDataSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing TaskResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::TaskDataSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing TaskResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TaskResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in TaskResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TaskResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TaskResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<TaskResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for TaskResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<TaskResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <TaskResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into TaskResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TaskRewardDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: models::CooldownSchema,

    /// Reward details.
    #[serde(rename = "reward")]
    pub reward: models::TaskRewardSchema,

    /// Player details.
    #[serde(rename = "character")]
    pub character: models::CharacterSchema,
}

impl TaskRewardDataSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(
        cooldown: models::CooldownSchema,
        reward: models::TaskRewardSchema,
        character: models::CharacterSchema,
    ) -> TaskRewardDataSchema {
        TaskRewardDataSchema {
            cooldown,
            reward,
            character,
        }
    }
}

/// Converts the TaskRewardDataSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TaskRewardDataSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping cooldown in query parameter serialization

            // Skipping reward in query parameter serialization

            // Skipping character in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TaskRewardDataSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TaskRewardDataSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cooldown: Vec<models::CooldownSchema>,
            pub reward: Vec<models::TaskRewardSchema>,
            pub character: Vec<models::CharacterSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing TaskRewardDataSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cooldown" => intermediate_rep.cooldown.push(
                        <models::CooldownSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "reward" => intermediate_rep.reward.push(
                        <models::TaskRewardSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "character" => intermediate_rep.character.push(
                        <models::CharacterSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing TaskRewardDataSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TaskRewardDataSchema {
            cooldown: intermediate_rep
                .cooldown
                .into_iter()
                .next()
                .ok_or_else(|| "cooldown missing in TaskRewardDataSchema".to_string())?,
            reward: intermediate_rep
                .reward
                .into_iter()
                .next()
                .ok_or_else(|| "reward missing in TaskRewardDataSchema".to_string())?,
            character: intermediate_rep
                .character
                .into_iter()
                .next()
                .ok_or_else(|| "character missing in TaskRewardDataSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TaskRewardDataSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TaskRewardDataSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<TaskRewardDataSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for TaskRewardDataSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<TaskRewardDataSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <TaskRewardDataSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into TaskRewardDataSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TaskRewardResponseSchema {
    #[serde(rename = "data")]
    pub data: models::TaskRewardDataSchema,
}

impl TaskRewardResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(data: models::TaskRewardDataSchema) -> TaskRewardResponseSchema {
        TaskRewardResponseSchema { data }
    }
}

/// Converts the TaskRewardResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TaskRewardResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TaskRewardResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TaskRewardResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<models::TaskRewardDataSchema>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing TaskRewardResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "data" => intermediate_rep.data.push(
                        <models::TaskRewardDataSchema as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing TaskRewardResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TaskRewardResponseSchema {
            data: intermediate_rep
                .data
                .into_iter()
                .next()
                .ok_or_else(|| "data missing in TaskRewardResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TaskRewardResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TaskRewardResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<TaskRewardResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for TaskRewardResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<TaskRewardResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <TaskRewardResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into TaskRewardResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TaskRewardSchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,

    /// Item quantity.
    #[serde(rename = "quantity")]
    pub quantity: i32,
}

impl TaskRewardSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(code: String, quantity: i32) -> TaskRewardSchema {
        TaskRewardSchema { code, quantity }
    }
}

/// Converts the TaskRewardSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TaskRewardSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("code".to_string()),
            Some(self.code.to_string()),
            Some("quantity".to_string()),
            Some(self.quantity.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TaskRewardSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TaskRewardSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<String>,
            pub quantity: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing TaskRewardSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "quantity" => intermediate_rep.quantity.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing TaskRewardSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TaskRewardSchema {
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in TaskRewardSchema".to_string())?,
            quantity: intermediate_rep
                .quantity
                .into_iter()
                .next()
                .ok_or_else(|| "quantity missing in TaskRewardSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TaskRewardSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TaskRewardSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<TaskRewardSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for TaskRewardSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<TaskRewardSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <TaskRewardSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into TaskRewardSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TaskSchema {
    /// Task objective.
    #[serde(rename = "code")]
    pub code: String,

    /// The type of task.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    /// The total required to complete the task.
    #[serde(rename = "total")]
    pub total: i32,
}

impl TaskSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(code: String, r#type: String, total: i32) -> TaskSchema {
        TaskSchema {
            code,
            r#type,
            total,
        }
    }
}

/// Converts the TaskSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TaskSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("code".to_string()),
            Some(self.code.to_string()),
            Some("type".to_string()),
            Some(self.r#type.to_string()),
            Some("total".to_string()),
            Some(self.total.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TaskSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TaskSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<String>,
            pub r#type: Vec<String>,
            pub total: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing TaskSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "total" => intermediate_rep.total.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing TaskSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TaskSchema {
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in TaskSchema".to_string())?,
            r#type: intermediate_rep
                .r#type
                .into_iter()
                .next()
                .ok_or_else(|| "type missing in TaskSchema".to_string())?,
            total: intermediate_rep
                .total
                .into_iter()
                .next()
                .ok_or_else(|| "total missing in TaskSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TaskSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TaskSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<TaskSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for TaskSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TaskSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <TaskSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into TaskSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TokenResponseSchema {
    #[serde(rename = "token")]
    pub token: String,
}

impl TokenResponseSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(token: String) -> TokenResponseSchema {
        TokenResponseSchema { token }
    }
}

/// Converts the TokenResponseSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TokenResponseSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> =
            vec![Some("token".to_string()), Some(self.token.to_string())];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TokenResponseSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TokenResponseSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub token: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing TokenResponseSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "token" => intermediate_rep.token.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing TokenResponseSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TokenResponseSchema {
            token: intermediate_rep
                .token
                .into_iter()
                .next()
                .ok_or_else(|| "token missing in TokenResponseSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TokenResponseSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TokenResponseSchema>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<TokenResponseSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for TokenResponseSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<TokenResponseSchema>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <TokenResponseSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into TokenResponseSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UnequipSchema {
    /// Item slot.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "slot")]
    pub slot: String,
}

impl UnequipSchema {
    #[allow(clippy::new_without_default)]
    pub fn new(slot: String) -> UnequipSchema {
        UnequipSchema { slot }
    }
}

/// Converts the UnequipSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UnequipSchema {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> =
            vec![Some("slot".to_string()), Some(self.slot.to_string())];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UnequipSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UnequipSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub slot: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing UnequipSchema".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "slot" => intermediate_rep.slot.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing UnequipSchema".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UnequipSchema {
            slot: intermediate_rep
                .slot
                .into_iter()
                .next()
                .ok_or_else(|| "slot missing in UnequipSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UnequipSchema> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UnequipSchema>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<UnequipSchema>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for UnequipSchema - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UnequipSchema> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <UnequipSchema as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into UnequipSchema - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ValidationError {
    #[serde(rename = "loc")]
    pub loc: Vec<models::ValidationErrorLocInner>,

    #[serde(rename = "msg")]
    pub msg: String,

    #[serde(rename = "type")]
    pub r#type: String,
}

impl ValidationError {
    #[allow(clippy::new_without_default)]
    pub fn new(
        loc: Vec<models::ValidationErrorLocInner>,
        msg: String,
        r#type: String,
    ) -> ValidationError {
        ValidationError { loc, msg, r#type }
    }
}

/// Converts the ValidationError value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ValidationError {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping loc in query parameter serialization
            Some("msg".to_string()),
            Some(self.msg.to_string()),
            Some("type".to_string()),
            Some(self.r#type.to_string()),
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ValidationError value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ValidationError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub loc: Vec<Vec<models::ValidationErrorLocInner>>,
            pub msg: Vec<String>,
            pub r#type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ValidationError".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "loc" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in ValidationError"
                                .to_string(),
                        )
                    }
                    #[allow(clippy::redundant_clone)]
                    "msg" => intermediate_rep.msg.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ValidationError".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ValidationError {
            loc: intermediate_rep
                .loc
                .into_iter()
                .next()
                .ok_or_else(|| "loc missing in ValidationError".to_string())?,
            msg: intermediate_rep
                .msg
                .into_iter()
                .next()
                .ok_or_else(|| "msg missing in ValidationError".to_string())?,
            r#type: intermediate_rep
                .r#type
                .into_iter()
                .next()
                .ok_or_else(|| "type missing in ValidationError".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ValidationError> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ValidationError>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ValidationError>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ValidationError - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<ValidationError>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ValidationError as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into ValidationError - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(
    Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate,
)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ValidationErrorLocInner {}

impl ValidationErrorLocInner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ValidationErrorLocInner {
        ValidationErrorLocInner {}
    }
}

/// Converts the ValidationErrorLocInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ValidationErrorLocInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ValidationErrorLocInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ValidationErrorLocInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {}

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ValidationErrorLocInner".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ValidationErrorLocInner".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ValidationErrorLocInner {})
    }
}

// Methods for converting between header::IntoHeaderValue<ValidationErrorLocInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ValidationErrorLocInner>>
    for hyper::header::HeaderValue
{
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ValidationErrorLocInner>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for ValidationErrorLocInner - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue>
    for header::IntoHeaderValue<ValidationErrorLocInner>
{
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ValidationErrorLocInner as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into ValidationErrorLocInner - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}
