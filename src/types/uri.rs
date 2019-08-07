use http;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

struct UriVisitor;

impl<'de> Visitor<'de> for UriVisitor {
    type Value = Uri;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid URI")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        match value.parse::<http::Uri>() {
            Err(_) => Err(E::custom("failed to parse URI".to_string())),
            Ok(uri) => Ok(Uri(uri)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Uri(pub http::Uri);

impl<'de> Deserialize<'de> for Uri {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(UriVisitor)
    }
}

mod tests {
    #[test]
    fn test_build_uri() {
        use super::Uri;
        Uri("https://testuri.com/path/1".parse::<http::Uri>().unwrap());
    }

    #[test]
    fn test_build_uri_from_json() {
        use super::Uri;

        let uri: Uri = serde_json::from_str("\"https://testuri.com/path/1\"").unwrap();
        assert_eq!(uri, Uri("https://testuri.com/path/1".parse::<http::Uri>().unwrap()));
    }
}
