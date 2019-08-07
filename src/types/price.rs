use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

struct PriceVisitor;

impl<'de> Visitor<'de> for PriceVisitor {
    type Value = Price;
    
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid price")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        match value.parse::<f64>() {
            Err(_) => Err(E::custom("failed to parse price".to_string())),
            Ok(price) => Ok(Price(price)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Price(pub f64);

impl<'de> Deserialize<'de> for Price {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(PriceVisitor)
    }
}

mod tests {
    #[test]
    fn test_parse_price() {
        use super::Price;

        let price: Price = serde_json::from_str("\"15.44\"").unwrap();
        assert_eq!(Price(15.44), price);
    }
}
