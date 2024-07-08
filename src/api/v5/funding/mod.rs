use crate::api::v5::model::FundingBalance;
use crate::api::v5::model::{AccountType, FundTransferResponse, TransferType};
use crate::api::v5::Request;
use crate::serde_util::MaybeFloat;

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
    /// ### HTTP Request
    /// **GET** /api/v5/asset/balances
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

    /// https://www.okx.com/docs-v5/en/#funding-account-rest-api-funds-transfer
    /// ## Funds transfer
    /// Only API keys with Trade privilege can call this endpoint.
    ///
    /// This endpoint supports the transfer of funds between your funding account and trading account, and from the master account to sub-accounts.
    ///
    /// Sub-account can transfer out to master account by default. Need to call Set permission of transfer out to grant privilege first if you want sub-account transferring to another sub-account (sub-accounts need to belong to same master account.)
    ///
    /// **Failure of the request does not mean the transfer has failed. Recommend to call "Get funds transfer state" to confirm the status.**
    ///
    /// Rate Limit: 1 request per second
    /// Rate limit rule: UserID + Currency
    /// ### HTTP Request
    /// **POST** /api/v5/asset/transfer
    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FundsTransfer {
        /// Transfer type
        pub r#type: TransferType,
        /// Transfer currency, e.g. USDT
        pub ccy: String,
        /// Amount to be transferred
        #[serde(default, deserialize_with = "deserialize_from_opt_str")]
        pub amt: MaybeFloat,
        /// The remitting account
        #[serde(serialize_with = "crate::serde_util::serialize_as_str")]
        pub from: AccountType,
        /// The beneficiary account
        #[serde(serialize_with = "crate::serde_util::serialize_as_str")]
        pub to: AccountType,
        /// Name of the sub-account
        /// When type is 1/2/4, this parameter is required.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sub_acct: Option<String>,
        /// Client-supplied ID
        /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub client_id: Option<String>,
    }

    impl Request for FundsTransfer {
        const METHOD: Method = Method::POST;
        const PATH: &'static str = "/asset/transfer";
        const AUTH: bool = true;

        type Response = Vec<FundTransferResponse>;
    }
}
