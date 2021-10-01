//! Serde deserialization code.

use std::fmt;

use bracket_lib::prelude::{to_cp437, RGB};
use serde::{de, Deserializer};

pub fn u16_from_cp437<'de, D: Deserializer<'de>>(d: D) -> Result<u16, D::Error> {
    d.deserialize_char(CP437Visitor)
}

struct CP437Visitor;

impl<'de> de::Visitor<'de> for CP437Visitor {
    type Value = u16;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a char")
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(to_cp437(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.is_empty() {
            return Err(E::invalid_value(de::Unexpected::Str(v), &self));
        }

        let mut chars = v.chars();
        let c = to_cp437(chars.next().unwrap());

        if chars.next().is_some() {
            Err(E::invalid_value(de::Unexpected::Str(v), &self))
        } else {
            Ok(c)
        }
    }
}

pub fn rgb_from_hex<'de, D: Deserializer<'de>>(d: D) -> Result<RGB, D::Error> {
    d.deserialize_str(RGBVisitor)
}

struct RGBVisitor;

impl<'de> de::Visitor<'de> for RGBVisitor {
    type Value = RGB;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a hex color string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        RGB::from_hex(v).map_err(|_| E::custom("invalid hex color"))
    }
}
