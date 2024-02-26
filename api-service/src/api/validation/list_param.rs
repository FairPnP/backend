#![allow(dead_code)]
use serde::de::{self};
use std::fmt;

pub fn param_list_i32<'de, D>(deserializer: D) -> std::result::Result<Option<Vec<i32>>, D::Error>
where
    D: de::Deserializer<'de>,
    // I: de::DeserializeOwned,
{
    struct StringVecVisitor(std::marker::PhantomData<i32>);

    impl<'de> de::Visitor<'de> for StringVecVisitor
    // where I: de::DeserializeOwned
    {
        type Value = Option<Vec<i32>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing a list")
        }

        fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            let mut ids = Vec::new();
            for id in v.split(',') {
                // let id = i32::deserialize(id.into_deserializer())?;
                let id = id.parse::<i32>().map_err(de::Error::custom)?;
                ids.push(id);
            }
            Ok(Some(ids))
        }
    }

    deserializer.deserialize_any(StringVecVisitor(std::marker::PhantomData::<i32>))
}
