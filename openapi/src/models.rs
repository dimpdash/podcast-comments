#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

/// A pet for sale in the pet store
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
#[serde(rename = "Comment")]
pub struct Comment {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    /// comment's body
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}

impl Comment {
    pub fn new() -> Comment {
        Comment {
            id: None,
            body: None,
        }
    }
}

/// Converts the Comment value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Comment {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }


        if let Some(ref body) = self.body {
            params.push("body".to_string());
            params.push(body.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Comment value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Comment {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<i64>,
            pub body: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Comment".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "body" => intermediate_rep.body.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Comment".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Comment {
            id: intermediate_rep.id.into_iter().next(),
            body: intermediate_rep.body.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Comment> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Comment>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Comment>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Comment - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Comment> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Comment as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Comment - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


impl Comment {
    /// Helper function to allow us to convert this model to an XML string.
    /// Will panic if serialisation fails.
    #[allow(dead_code)]
    pub(crate) fn to_xml(&self) -> String {
        serde_xml_rs::to_string(&self).expect("impossible to fail to serialize")
    }
}
