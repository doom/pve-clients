use std::collections::HashMap;
use std::marker::PhantomData;

use serde::de::{DeserializeOwned, Error, Unexpected};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

struct LaxBoolVisitor;

impl<'v> serde::de::Visitor<'v> for LaxBoolVisitor {
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
    deserializer.deserialize_any(LaxBoolVisitor)
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

struct RepeatedItemsVisitor<'a, V, D> {
    pub prefix: &'a str,
    pub phantom: PhantomData<(V, D)>, // required for types to be correctly inferred
}

impl<'de, V, D> serde::de::Visitor<'de> for RepeatedItemsVisitor<'_, V, D>
where
    V: DeserializeOwned,
    D: Deserializer<'de>,
{
    type Value = HashMap<u32, V>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "repeated items with prefix {}", self.prefix)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut output = HashMap::new();

        while let Some(prefixed_key) = map.next_key::<String>()? {
            let key = prefixed_key
                .strip_prefix(self.prefix)
                .and_then(|key| str::parse::<u32>(key).ok());

            if let Some(key) = key {
                let value = map.next_value::<V>()?;
                output.insert(key, value);
            }
        }

        Ok(output)
    }
}

pub fn deserialize_repeated_with_prefix<'de, D, V>(
    prefix: &str,
    deserializer: D,
) -> Result<HashMap<u32, V>, D::Error>
where
    D: Deserializer<'de>,
    V: DeserializeOwned,
{
    deserializer.deserialize_map(RepeatedItemsVisitor::<V, D> {
        prefix,
        phantom: Default::default(),
    })
}

pub fn serialize_repeated_with_prefix<V, S>(
    value: &HashMap<u32, V>,
    prefix: &str,
    s: S,
) -> Result<S::Ok, S::Error>
where
    V: Serialize,
    S: Serializer,
{
    let mut map = s.serialize_map(Some(value.len()))?;

    for (key, value) in value.iter() {
        map.serialize_entry(&format!("{}{}", prefix, key), value)?;
    }

    map.end()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{deserialize_bool_lax, deserialize_option_bool_lax};
    use super::{deserialize_repeated_with_prefix, serialize_repeated_with_prefix};
    use serde::de::value::{Error as DeserializeError, StrDeserializer, U64Deserializer};
    use serde::de::{DeserializeOwned, IntoDeserializer};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde_json::json;

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

    fn serialize_value<V, S>(value: &HashMap<u32, V>, s: S) -> Result<S::Ok, S::Error>
    where
        V: Serialize,
        S: Serializer,
    {
        serialize_repeated_with_prefix(value, "value", s)
    }

    pub fn deserialize_value<'de, D, V>(deserializer: D) -> Result<HashMap<u32, V>, D::Error>
    where
        D: Deserializer<'de>,
        V: DeserializeOwned,
    {
        deserialize_repeated_with_prefix("value", deserializer)
    }

    #[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
    struct TestRepeated {
        #[serde(
            serialize_with = "serialize_value",
            deserialize_with = "deserialize_value",
            flatten,
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub value: HashMap<u32, String>,
        pub z: u32,
    }

    #[test]
    fn test_deserialize_repeated_with_prefix() {
        let deserialized: TestRepeated = serde_json::from_value(json!({
            "value1": "abc",
            "value2": "def",
            "z": 3,
        }))
        .expect("expected deserialization to succeed");
        let expected = TestRepeated {
            value: HashMap::from([(1, "abc".to_string()), (2, "def".to_string())]),
            z: 3,
        };
        assert_eq!(deserialized, expected);

        let deserialized: TestRepeated = serde_json::from_value(json!({
            "z": 3,
        }))
        .expect("expected deserialization to succeed");
        let expected = TestRepeated {
            value: HashMap::new(),
            z: 3,
        };
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_serialize_repeated_with_prefix() {
        let serialized = serde_json::to_value(&TestRepeated {
            value: HashMap::from([(1, "abc".to_string()), (2, "def".to_string())]),
            z: 3,
        })
        .expect("expected serialization to succeed");
        let expected = serde_json::json!({
            "value1": "abc",
            "value2": "def",
            "z": 3,
        });
        assert_eq!(serialized, expected);

        let serialized = serde_json::to_value(&TestRepeated {
            value: HashMap::new(),
            z: 3,
        })
        .expect("expected serialization to succeed");
        let expected = serde_json::json!({
            "z": 3,
        });
        assert_eq!(serialized, expected);
    }
}
