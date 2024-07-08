use crate::api::v5::model::TradingBalanceDetail;
use crate::api::v5::Request;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::*;

/// https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-balance
/// ## Get balance
/// Retrieve a list of assets (with non-zero balance), remaining balance, and available amount in the trading account.
///
///  Interest-free quota and discount rates are public data and not displayed on the account interface.
/// Rate Limit: 10 requests per 2 seconds
/// Rate limit rule: UserID
/// ### HTTP Requests
/// **GET** /api/v5/account/balance
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTradingBalances {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Single currency or multiple currencies (no more than 20) separated with comma, e.g. BTC or BTC,ETH.
    pub ccy: Option<String>,
}

impl Request for GetTradingBalances {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/account/balance";
    const AUTH: bool = true;
    type Response = Vec<TradingBalanceDetail>;
}
