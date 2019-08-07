use http;
use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;
use std::str::FromStr;

struct UriVisitor;

impl<'de> Visitor<'de> for UriVisitor {
    type Value = Uri;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid URI")
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<Self::Value, E> {
        match http::Uri::from_str(value.as_ref()) {
            Err(_) => Err(E::custom("failed to parse URI".to_string())),
            Ok(uri) => Ok(Uri(uri)),
        }
    }
}

pub struct Uri(pub http::Uri);

impl<'de> Deserialize<'de> for Uri {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_string(UriVisitor)
    }
}
