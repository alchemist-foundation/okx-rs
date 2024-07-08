use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::api::v5::{model::OKXSystemTime, Request};

pub mod rest {

    use crate::api::v5::model::InstrumentType;

    use super::*;

    /// https://www.okx.com/docs-v5/en/?shell#public-data-rest-api-get-system-time
    /// ## Get System Time
    /// Retrieve API server time.
    ///
    /// Rate Limit: 10 requests per 2 seconds <br/>
    /// Rate limit rule: IP
    ///
    /// ### HTTP Request
    /// [GET] /api/v5/public/time
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GetSystemTime;
    impl Request for GetSystemTime {
        const METHOD: Method = Method::GET;
        const PATH: &'static str = "/public/time";
        const AUTH: bool = false;

        type Response = Vec<OKXSystemTime>;
    }

    /// https://www.okx.com/docs-v5/en/#public-data-rest-api-get-instruments
    /// ## Get instruments
    /// Retrieve a list of instruments with open contracts.
    ///
    /// Rate Limit: 20 requests per 2 seconds <br/>
    /// Rate limit rule: IP + instrumentType
    ///
    /// ### HTTP Request
    /// [GET] /api/v5/public/instruments
    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GetInstruments {
        /// Instrument Type: SPOT, MARGIN, SWAP, FUTURES, OPTION
        pub inst_type: InstrumentType,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub uly: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub inst_family: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub inst_id: Option<String>,
    }
}
