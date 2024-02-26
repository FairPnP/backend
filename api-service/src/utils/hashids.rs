#![allow(dead_code)]

use harsh::Harsh;
use lazy_static::lazy_static;

lazy_static! {
    static ref HASHIDS: Harsh = {
        let salt = std::env::var("HASHIDS_SALT").expect("HASHIDS_SALT must be set");
        Harsh::builder()
            .length(10)
            .salt(salt)
            .build()
            .expect("Failed to build Harsh")
    };
}

pub fn encode_id(id: i32) -> String {
    HASHIDS.encode(&[id as u64])
}

pub fn encode_id_option(id: Option<i32>) -> Option<String> {
    id.map(encode_id)
}

pub fn decode_id(hash: &str) -> Result<i32, harsh::Error> {
    match HASHIDS.decode(hash) {
        Ok(decoded) => Ok(decoded[0] as i32),
        Err(e) => Err(e),
    }
}

pub fn decode_id_option(hash: &Option<String>) -> Result<Option<i32>, harsh::Error> {
    match hash {
        Some(h) => match decode_id(h) {
            Ok(id) => Ok(Some(id)),
            Err(e) => Err(e),
        },
        None => Ok(None),
    }
}
