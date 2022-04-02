#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FilesList {
    #[serde(rename = "files")]
    pub files: Vec<String>,

}

impl FilesList {
    pub fn new(files: Vec<String>, ) -> FilesList {
        FilesList {
            files: files,
        }
    }
}

/// Converts the FilesList value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for FilesList {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("files".to_string());
        params.push(self.files.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a FilesList value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for FilesList {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub files: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing FilesList".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "files" => return std::result::Result::Err("Parsing a container in this style is not supported in FilesList".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing FilesList".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(FilesList {
            files: intermediate_rep.files.into_iter().next().ok_or("files missing in FilesList".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<FilesList> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<FilesList>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<FilesList>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for FilesList - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<FilesList> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <FilesList as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into FilesList - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse200 {
    #[serde(rename = "role")]
    pub role: models::Role,

    #[serde(rename = "location")]
    pub location: models::MoveTo,

    #[serde(rename = "identity")]
    pub identity: String,

}

impl InlineResponse200 {
    pub fn new(role: models::Role, location: models::MoveTo, identity: String, ) -> InlineResponse200 {
        InlineResponse200 {
            role: role,
            location: location,
            identity: identity,
        }
    }
}

/// Converts the InlineResponse200 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse200 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping role in query parameter serialization

        // Skipping location in query parameter serialization


        params.push("identity".to_string());
        params.push(self.identity.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse200 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse200 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub role: Vec<models::Role>,
            pub location: Vec<models::MoveTo>,
            pub identity: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InlineResponse200".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "role" => intermediate_rep.role.push(<models::Role as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "location" => intermediate_rep.location.push(<models::MoveTo as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "identity" => intermediate_rep.identity.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InlineResponse200".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse200 {
            role: intermediate_rep.role.into_iter().next().ok_or("role missing in InlineResponse200".to_string())?,
            location: intermediate_rep.location.into_iter().next().ok_or("location missing in InlineResponse200".to_string())?,
            identity: intermediate_rep.identity.into_iter().next().ok_or("identity missing in InlineResponse200".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InlineResponse200> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse200>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InlineResponse200>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InlineResponse200 - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<InlineResponse200> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InlineResponse200 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InlineResponse200 - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InlineResponse2001 {
    #[serde(rename = "is_alive")]
    pub is_alive: bool,

}

impl InlineResponse2001 {
    pub fn new(is_alive: bool, ) -> InlineResponse2001 {
        InlineResponse2001 {
            is_alive: is_alive,
        }
    }
}

/// Converts the InlineResponse2001 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InlineResponse2001 {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("is_alive".to_string());
        params.push(self.is_alive.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InlineResponse2001 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InlineResponse2001 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub is_alive: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InlineResponse2001".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "is_alive" => intermediate_rep.is_alive.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InlineResponse2001".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InlineResponse2001 {
            is_alive: intermediate_rep.is_alive.into_iter().next().ok_or("is_alive missing in InlineResponse2001".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InlineResponse2001> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InlineResponse2001>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InlineResponse2001>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InlineResponse2001 - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<InlineResponse2001> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InlineResponse2001 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InlineResponse2001 - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct KillResult {
    #[serde(rename = "killed_users_count")]
    pub killed_users_count: u32,

}

impl KillResult {
    pub fn new(killed_users_count: u32, ) -> KillResult {
        KillResult {
            killed_users_count: killed_users_count,
        }
    }
}

/// Converts the KillResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for KillResult {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("killed_users_count".to_string());
        params.push(self.killed_users_count.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a KillResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for KillResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub killed_users_count: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing KillResult".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "killed_users_count" => intermediate_rep.killed_users_count.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing KillResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(KillResult {
            killed_users_count: intermediate_rep.killed_users_count.into_iter().next().ok_or("killed_users_count missing in KillResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<KillResult> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<KillResult>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<KillResult>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for KillResult - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<KillResult> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <KillResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into KillResult - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MoveTo {
    #[serde(rename = "location")]
    pub location: String,

}

impl MoveTo {
    pub fn new(location: String, ) -> MoveTo {
        MoveTo {
            location: location,
        }
    }
}

/// Converts the MoveTo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MoveTo {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("location".to_string());
        params.push(self.location.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MoveTo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MoveTo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub location: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MoveTo".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "location" => intermediate_rep.location.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MoveTo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MoveTo {
            location: intermediate_rep.location.into_iter().next().ok_or("location missing in MoveTo".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MoveTo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MoveTo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MoveTo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MoveTo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MoveTo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MoveTo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MoveTo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Notification {
    #[serde(rename = "message")]
    pub message: String,

}

impl Notification {
    pub fn new(message: String, ) -> Notification {
        Notification {
            message: message,
        }
    }
}

/// Converts the Notification value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Notification {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("message".to_string());
        params.push(self.message.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Notification value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Notification {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Notification".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Notification".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Notification {
            message: intermediate_rep.message.into_iter().next().ok_or("message missing in Notification".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Notification> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Notification>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Notification>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Notification - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Notification> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Notification as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Notification - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Query {
    #[serde(rename = "arguments")]
    pub arguments: Vec<String>,

}

impl Query {
    pub fn new(arguments: Vec<String>, ) -> Query {
        Query {
            arguments: arguments,
        }
    }
}

/// Converts the Query value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Query {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("arguments".to_string());
        params.push(self.arguments.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Query value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Query {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub arguments: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Query".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "arguments" => return std::result::Result::Err("Parsing a container in this style is not supported in Query".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Query".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Query {
            arguments: intermediate_rep.arguments.into_iter().next().ok_or("arguments missing in Query".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Query> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Query>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Query>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Query - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Query> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Query as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Query - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Role {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "allowed_commands")]
    pub allowed_commands: Vec<String>,

}

impl Role {
    pub fn new(title: String, allowed_commands: Vec<String>, ) -> Role {
        Role {
            title: title,
            allowed_commands: allowed_commands,
        }
    }
}

/// Converts the Role value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Role {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("title".to_string());
        params.push(self.title.to_string());


        params.push("allowed_commands".to_string());
        params.push(self.allowed_commands.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Role value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub title: Vec<String>,
            pub allowed_commands: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Role".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "allowed_commands" => return std::result::Result::Err("Parsing a container in this style is not supported in Role".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Role".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Role {
            title: intermediate_rep.title.into_iter().next().ok_or("title missing in Role".to_string())?,
            allowed_commands: intermediate_rep.allowed_commands.into_iter().next().ok_or("allowed_commands missing in Role".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Role> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Role>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Role>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Role - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Role> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Role as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Role - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UsersList {
    #[serde(rename = "users")]
    pub users: Vec<models::UsersListUsers>,

}

impl UsersList {
    pub fn new(users: Vec<models::UsersListUsers>, ) -> UsersList {
        UsersList {
            users: users,
        }
    }
}

/// Converts the UsersList value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UsersList {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping users in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UsersList value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UsersList {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub users: Vec<Vec<models::UsersListUsers>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UsersList".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "users" => return std::result::Result::Err("Parsing a container in this style is not supported in UsersList".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing UsersList".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UsersList {
            users: intermediate_rep.users.into_iter().next().ok_or("users missing in UsersList".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UsersList> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UsersList>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UsersList>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UsersList - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UsersList> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UsersList as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UsersList - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UsersListUsers {
    #[serde(rename = "user")]
    pub user: String,

    #[serde(rename = "location")]
    pub location: String,

}

impl UsersListUsers {
    pub fn new(user: String, location: String, ) -> UsersListUsers {
        UsersListUsers {
            user: user,
            location: location,
        }
    }
}

/// Converts the UsersListUsers value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UsersListUsers {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("user".to_string());
        params.push(self.user.to_string());


        params.push("location".to_string());
        params.push(self.location.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UsersListUsers value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UsersListUsers {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub user: Vec<String>,
            pub location: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UsersListUsers".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "user" => intermediate_rep.user.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "location" => intermediate_rep.location.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UsersListUsers".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UsersListUsers {
            user: intermediate_rep.user.into_iter().next().ok_or("user missing in UsersListUsers".to_string())?,
            location: intermediate_rep.location.into_iter().next().ok_or("location missing in UsersListUsers".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UsersListUsers> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UsersListUsers>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UsersListUsers>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UsersListUsers - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UsersListUsers> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UsersListUsers as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UsersListUsers - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

