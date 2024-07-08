use crate::serde_util::str_opt;
use std::{borrow::Cow, fmt::Debug};

use reqwest::Method;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

// export - public data
pub mod funding;
pub mod model;
pub mod public_data;
pub mod trading;

pub trait Request: Serialize {
    const METHOD: Method;
    const PATH: &'static str;
    const AUTH: bool = false;

    type Response: DeserializeOwned + Debug;

    fn path(&self) -> Cow<'_, str> {
        Cow::Borrowed(Self::PATH)
    }
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    #[serde(default, with = "str_opt")]
    pub code: Option<u64>,
    #[serde(default, with = "str_opt")]
    pub msg: Option<String>,
    pub data: Option<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsResponse<'a, A: Debug, T: Debug> {
    pub id: Option<&'a str>,
    pub op: Option<&'a str>,
    pub arg: Option<A>,
    #[serde(default, with = "str_opt")]
    pub code: Option<u64>,
    pub conn_id: Option<&'a str>,
    pub event: Option<&'a str>,
    pub action: Option<&'a str>,
    pub data: Option<T>,
    pub msg: Option<&'a str>,
}
