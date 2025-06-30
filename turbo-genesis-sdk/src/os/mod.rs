use super::*;
use base64::{
    engine::general_purpose::{STANDARD as b64, URL_SAFE_NO_PAD as b64_url_safe},
    Engine,
};
use borsh::BorshDeserialize;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    any::{Any, TypeId},
    collections::{BTreeSet, HashMap},
    path::{Path, PathBuf},
};

pub mod client;
pub mod server;

#[derive(Debug, Clone)]
pub struct QueryResult<T> {
    pub loading: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> QueryResult<T> {
    pub fn new() -> Self {
        Self {
            loading: false,
            data: None,
            error: None,
        }
    }
}
impl QueryResult<client::fs::ProgramFile> {
    pub fn contents<T: BorshDeserialize /* <- ideally, Borsh + Json */>(&self) -> Option<T> {
        let Some(data) = &self.data else {
            return None;
        };
        T::try_from_slice(&data.contents).ok()
    }
}

fn from_base64<'a, D: Deserializer<'a>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    use serde::de::Error;
    <String as Deserialize>::deserialize(deserializer).and_then(|string| {
        b64.decode(&string)
            .map_err(|err| Error::custom(err.to_string()))
    })
}

fn as_base64<T: AsRef<[u8]>, S: Serializer>(v: &T, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&b64.encode(v.as_ref()))
}
