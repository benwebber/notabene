use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize)]
pub enum Format {
    #[default]
    Short,
    Full,
    Json,
    JsonLines,
}

impl<'de> Deserialize<'de> for Format {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormatVisitor;

        impl<'de> Visitor<'de> for FormatVisitor {
            type Value = Format;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an output format")
            }

            fn visit_str<E>(self, value: &str) -> Result<Format, E>
            where
                E: de::Error,
            {
                match value {
                    "short" => Ok(Format::Short),
                    "full" => Ok(Format::Full),
                    "json" => Ok(Format::Json),
                    "jsonl" => Ok(Format::JsonLines),
                    _ => Err(de::Error::unknown_variant(
                        value,
                        &["short", "full", "json", "jsonl"],
                    )),
                }
            }
        }

        deserializer.deserialize_str(FormatVisitor)
    }
}
