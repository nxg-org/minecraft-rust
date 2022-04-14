use serde_json::Value;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct Namespace {
    #[serde(default)]
    pub types: Types,
    #[serde(flatten)]
    pub sub: HashMap<String, Namespace>,
}
pub type Types = HashMap<String, Type>;

#[derive(Debug)]
pub enum Type {
    Native,
    Reference { ref_name: String },
    Call { called_type: String, args: Value },
}

impl<'de> serde::Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct JsonTypeVisitor;
        impl<'de> serde::de::Visitor<'de> for JsonTypeVisitor {
            type Value = Type;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(r#"`"native"` for `JsonType::Native`, `"{type}"` for `JsonType::Reference` and `["{type}", ...]` for `JsonType::Reference`"#)
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(if v == "native" {
                    Self::Value::Native
                } else {
                    Self::Value::Reference {
                        ref_name: v.to_string(),
                    }
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(if v == "native" {
                    Self::Value::Native
                } else {
                    Self::Value::Reference { ref_name: v }
                })
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                if let Ok(Some(called_type)) = seq.next_element::<String>() {
                    if let Ok(Some(args)) = seq.next_element() {
                        Ok(Self::Value::Call { called_type, args })
                    } else {
                        Err(serde::de::Error::custom(
                            "missing arguments in call-like protodef type",
                        ))
                    }
                } else {
                    Err(serde::de::Error::custom(
                        "missing caller type in call-like protodef type",
                    ))
                }
            }
        }
        deserializer.deserialize_any(JsonTypeVisitor)
    }
}
