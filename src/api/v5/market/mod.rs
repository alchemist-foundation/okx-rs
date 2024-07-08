use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::api::v5::model::{Candle, InstrumentType, Ticker};
use crate::api::v5::Request;

pub mod rest {

    use super::*;

    /// https://www.okx.com/docs-v5/en/#order-book-trading-market-data-get-tickers
    /// ## Get Ticker
    /// Retrieve the latest price snapshot, best bid/ask price, and trading volume in the last 24 hours.
    ///
    /// Rate Limit: 20 requests per 2 second <br/>
    /// Rate limit rule: IP
    ///
    /// ### HTTP Request
    /// **GET** /api/v5/market/tickers
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct GetTickers {
        pub inst_type: InstrumentType,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub uly: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub inst_family: Option<String>,
    }

    impl Request for GetTickers {
        const METHOD: Method = Method::GET;
        const PATH: &'static str = "/market/tickers";
        const AUTH: bool = true;

        type Response = Vec<Ticker>;
    }

    /// https://www.okx.com/docs-v5/en/#order-book-trading-market-data-get-ticker
    /// ## Get Ticker
    /// Retrieve the latest price snapshot, best bid/ask price, and trading volume in the last 24 hours.
    ///
    /// Rate Limit: 20 requests per 2 second <br/>
    /// Rate limit rule: IP
    ///
    /// ### HTTP Request
    /// **GET** /api/v5/market/ticker
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct GetTicker {
        pub inst_id: String,
    }

    impl Request for GetTicker {
        const METHOD: Method = Method::GET;
        const PATH: &'static str = "/market/ticker";
        const AUTH: bool = true;

        type Response = Vec<Ticker>;
    }

    /// https://www.okx.com/docs-v5/en/#order-book-trading-market-data-get-candlesticks
    /// ## Get Candlesticks
    /// Retrieve the candlestick charts. This endpoint can retrieve the latest 1,440 data entries. Charts are returned in groups based on the requested bar.
    ///
    /// Rate Limit: 20 requests per 2 second <br/>
    /// Rate limit rule: IP
    ///
    /// ### HTTP Request
    /// **GET** /api/v5/market/ticker
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct GetCandlesticks {
        pub inst_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bar: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub before: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<String>,
    }

    impl Request for GetCandlesticks {
        const METHOD: Method = Method::GET;
        const PATH: &'static str = "/market/candles";
        const AUTH: bool = true;

        type Response = Vec<Candle>;
    }
}
