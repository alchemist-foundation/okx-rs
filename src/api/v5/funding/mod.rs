use crate::api::v5::model::FundingBalance;
use crate::api::v5::Request;

use reqwest::Method;
use serde::{Deserialize, Serialize};

pub mod rest {

    use super::*;

    /// https://www.okx.com/docs-v5/en/#funding-account-rest-api-get-balance
    /// Get balance
    /// Retrieve the funding account balances of all the assets and the amount that is available or on hold.
    ///
    ///  Only asset information of a currency with a balance greater than 0 will be returned.
    /// Rate Limit: 6 requests per second
    /// Rate limit rule: UserID
    /// HTTP Request
    /// GET /api/v5/asset/balances
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct GetFundingBalances {
        /// Single currency or multiple currencies (no more than 20) separated with comma, e.g. BTC or BTC,ETH.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ccy: Option<String>,
    }

    impl Request for GetFundingBalances {
        const METHOD: Method = Method::GET;
        const PATH: &'static str = "/asset/balances";
        const AUTH: bool = true;
        type Response = Vec<FundingBalance>;
    }
}
