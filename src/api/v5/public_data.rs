use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::api::v5::Request;

pub mod rest {

    use crate::api::v5::model::{CandleOHLC, Instrument, InstrumentType, OKXSystemTime};

    use super::*;

    /// https://www.okx.com/docs-v5/en/?shell#public-data-rest-api-get-system-time
    /// ## Get System Time
    /// Retrieve API server time.
    ///
    /// Rate Limit: 10 requests per 2 seconds <br/>
    /// Rate limit rule: IP
    ///
    /// ### HTTP Request
    /// **GET** /api/v5/public/time
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
    /// **GET** /api/v5/public/instruments
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

    impl Request for GetInstruments {
        const METHOD: Method = Method::GET;
        const PATH: &'static str = "/public/instruments";
        const AUTH: bool = false;
        type Response = Vec<Instrument>;
    }

    /// https://www.okx.com/docs-v5/en/#public-data-rest-api-get-mark-price-candlesticks
    /// ## Get mark price candlesticks
    /// Retrieve the candlestick charts of mark price. This endpoint can retrieve the latest 1,440 data entries. Charts are returned in groups based on the requested bar.
    ///
    /// Rate Limit: 20 requests per 2 seconds
    /// Rate limit rule: IP
    /// ## HTTP Request
    /// GET /api/v5/market/mark-price-candles
    #[derive(Debug, Clone, Serialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct GetMarkPriceCandles {
        /// Instrument ID, e.g. BTC-USD-SWAP
        pub inst_id: String,
        /// Pagination of data to return records earlier than the requested ts
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<u64>,
        /// Pagination of data to return records newer than the requested ts. The latest data will be returned when using before individually
        #[serde(skip_serializing_if = "Option::is_none")]
        pub before: Option<u64>,
        /// Bar size, the default is 1m
        /// e.g. [1m/3m/5m/15m/30m/1H/2H/4H]
        /// Hong Kong time opening price k-line：[6H/12H/1D/1W/1M/3M]
        /// UTC time opening price k-line：[6Hutc/12Hutc/1Dutc/1Wutc/1Mutc/3Mutc]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bar: Option<String>,
        /// Number of results per request. The maximum is 100; The default is 100
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<usize>,
    }

    impl Request for GetMarkPriceCandles {
        const METHOD: Method = Method::GET;
        const PATH: &'static str = "/market/mark-price-candles";
        const AUTH: bool = false;

        type Response = Vec<CandleOHLC>;
    }
}
