use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::api::v5::{model::OKXSystemTime, Request};

pub mod rest {

    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GetSystemTime;
    impl Request for GetSystemTime {
        const METHOD: Method = Method::GET;
        const PATH: &'static str = "/public/time";
        const AUTH: bool = false;

        type Response = Vec<OKXSystemTime>;
    }
}
