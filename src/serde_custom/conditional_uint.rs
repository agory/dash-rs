use either::*;
use serde::{de, Deserialize, Deserializer, Serializer};

pub fn deserialize<'de, D>(deserializer: D) -> Result<Either<u64, bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let str_sequence = String::deserialize(deserializer)?;
    match str_sequence.as_str() {
        "true" => Ok(Right(true)),
        "false" => Ok(Right(false)),
        value => {
            let value = value.parse::<u64>().map_err(de::Error::custom)?;
            Ok(Left(value))
        }
    }
}

pub fn serialize<S>(conditional_uint: &Either<u64, bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match conditional_uint {
        Left(value) => serializer.serialize_u64(*value),
        Right(value) => serializer.serialize_bool(*value),
    }
}
