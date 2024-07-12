use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer, Serializer};

struct Visitor;

impl<'v> serde::de::Visitor<'v> for Visitor {
    type Value = bool;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a boolean value represented as string or integer")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match v {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(E::invalid_value(Unexpected::Unsigned(v), &"either 0 or 1")),
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match str::parse(v) {
            Ok(0) => Ok(false),
            Ok(1) => Ok(true),
            Ok(v) => Err(E::invalid_value(Unexpected::Unsigned(v), &"either 0 or 1")),
            Err(_) => Err(E::invalid_value(
                Unexpected::Str(v),
                &"an integer represented as a string",
            )),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default, serde::Deserialize)]
struct Wrapper(#[serde(deserialize_with = "deserialize_bool_lax")] pub bool);

pub fn deserialize_option_bool_lax<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<Wrapper>::deserialize(deserializer).map(|w| w.map(|w| w.0))
}

pub fn deserialize_bool_lax<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(Visitor)
}

pub fn serialize_bool_as_u64<S>(value: &bool, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u64(if *value { 1 } else { 0 })
}

pub fn serialize_option_bool_as_u64<S>(value: &Option<bool>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(v) = value {
        serialize_bool_as_u64(v, s)
    } else {
        s.serialize_none()
    }
}

#[cfg(test)]
mod tests {
    use super::{deserialize_bool_lax, deserialize_option_bool_lax};
    use serde::de::value::{Error as DeserializeError, StrDeserializer, U64Deserializer};
    use serde::de::IntoDeserializer;
    use serde::Deserialize;

    #[test]
    fn test_deserialize_bool_lax() {
        let d: StrDeserializer<DeserializeError> = "0".into_deserializer();
        assert_eq!(deserialize_bool_lax(d), Ok(false));

        let d: StrDeserializer<DeserializeError> = "1".into_deserializer();
        assert_eq!(deserialize_bool_lax(d), Ok(true));

        let d: StrDeserializer<DeserializeError> = "2".into_deserializer();
        assert!(deserialize_bool_lax(d).is_err());

        let d: StrDeserializer<DeserializeError> = "abc".into_deserializer();
        assert!(deserialize_bool_lax(d).is_err());

        let d: StrDeserializer<DeserializeError> = "".into_deserializer();
        assert!(deserialize_bool_lax(d).is_err());

        let d: U64Deserializer<DeserializeError> = 0u64.into_deserializer();
        assert_eq!(deserialize_bool_lax(d), Ok(false));

        let d: U64Deserializer<DeserializeError> = 1u64.into_deserializer();
        assert_eq!(deserialize_bool_lax(d), Ok(true));

        let d: U64Deserializer<DeserializeError> = 2u64.into_deserializer();
        assert!(deserialize_bool_lax(d).is_err());
    }

    #[test]
    fn test_deserialize_option_bool_lax() {
        #[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_option_bool_lax", default)]
            pub value: Option<bool>,
            pub z: u32,
        }

        assert_eq!(
            serde_json::from_str::<Test>("{\"z\": 2, \"value\": null}").ok(),
            Some(Test { z: 2, value: None })
        );

        assert_eq!(
            serde_json::from_str::<Test>("{\"z\": 2}").ok(),
            Some(Test { z: 2, value: None })
        );
    }
}
