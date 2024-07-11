use model::Platform24Volume;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::api::v5::model::{Candle, InstrumentType, Ticker};
use crate::api::v5::Request;

use super::*;

/// https://www.okx.com/docs-v5/en/#order-book-trading-market-data-get-tickers
/// ## Get Ticker
/// Retrieve the latest price snapshot, best bid/ask price, and trading volume in the last 24 hours.
///
/// Rate Limit: 20 requests per 2 second \
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
/// Rate Limit: 20 requests per 2 second \
/// Rate limit rule: IP
///
/// ### HTTP Request
/// **GET** /api/v5/market/ticker
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTicker {
    /// Instrument ID, e.g. BTC-USD-SWAP
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
/// Rate Limit: 20 requests per 2 second \
/// Rate limit rule: IP
///
/// ### HTTP Request
/// **GET** /api/v5/market/candles
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCandlesticks {
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

impl Request for GetCandlesticks {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/market/candles";
    const AUTH: bool = true;

    type Response = Vec<Candle>;
}

/// https://www.okx.com/docs-v5/en/#order-book-trading-market-data-get-candlesticks-history
/// ## Get History Candlesticks
/// Retrieve history candlestick charts from recent years(It is last 3 months supported for 1s candlestick).
///
/// Rate Limit: 20 requests per 2 second \
/// Rate limit rule: IP
///
/// ### HTTP Request
/// **GET** /api/v5/market/history-candles
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHistoryCandlesticks {
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

impl Request for GetHistoryCandlesticks {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/market/history-candles";
    const AUTH: bool = true;

    type Response = Vec<Candle>;
}

/// https://www.okx.com/docs-v5/en/#order-book-trading-market-data-get-24h-total-volume
/// ## Get Platform 24 hours volume
/// The 24-hour trading volume is calculated on a rolling basis.
///
/// Rate Limit: 20 requests per 2 second \
/// Rate limit rule: IP
///
/// ### HTTP Request
/// **GET** /api/v5/market/platform-24-volume
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPlatform24Volume {}

impl Request for GetPlatform24Volume {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/market/platform-24-volume";
    const AUTH: bool = true;

    type Response = Vec<Platform24Volume>;
}
