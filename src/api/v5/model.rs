use crate::impl_string_enum;
use crate::serde_util::*;

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct Unknown;
impl Display for Unknown {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown")
    }
}
impl FromStr for Unknown {
    type Err = ();

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Self)
    }
}

impl From<&str> for Unknown {
    fn from(_: &str) -> Self {
        Self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstrumentType {
    Spot,
    Margin,
    Swap,
    Futures,
    Option,
    Any,
}

impl_string_enum!(InstrumentType,
    Spot => "SPOT",
    Margin => "MARGIN",
    Swap => "SWAP",
    Futures => "FUTURES",
    Option => "OPTION",
    Any => "ANY",
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Buy,
    Sell,
}

impl_string_enum!(Side,
    Buy => "buy",
    Sell => "sell",
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositionSide {
    Long,
    Short,
    Net,
}

impl_string_enum!(PositionSide,
    Long => "long",
    Short => "short",
    Net => "net",
);

#[derive(Debug, Clone, Copy, Hash)]
pub enum MarginMode {
    Cross,
    Isolated,
}

impl_string_enum!(MarginMode,
    Cross => "cross",
    Isolated => "isolated",
);

#[derive(Debug, Clone, Copy, Hash)]
pub enum TradeMode {
    Cross,
    Isolated,
    Cash,
}

impl_string_enum!(TradeMode,
    Cross => "cross",
    Isolated => "isolated",
    Cash => "cash",
);

#[derive(Debug, Clone, Copy, Hash)]
pub enum OrderType {
    Market,
    Limit,
    PostOnly,
    Fok,
    Ioc,
    OptimalLimitIoc,
}

impl_string_enum!(OrderType,
    Market => "market",
    Limit => "limit",
    PostOnly => "post_only",
    Fok => "fok",
    Ioc => "ioc",
    OptimalLimitIoc => "optimal_limit_ioc",
);

#[derive(Debug, Clone, Copy, Hash)]
pub enum QuantityType {
    BaseCcy,
    QuoteCcy,
    Other(Unknown),
}

impl_string_enum!(QuantityType,
    Other,
    BaseCcy => "base_ccy",
    QuoteCcy => "quote_ccy",
);

#[derive(Debug, Clone, Copy, Hash)]
pub enum OrderState {
    Canceled,
    Live,
    PartiallyFilled,
    Filled,
    Other(Unknown),
}

impl_string_enum!(OrderState,
    Other,
    Canceled => "canceled",
    Live => "live",
    PartiallyFilled => "partially_filled",
    Filled => "filled",
);

#[derive(Debug, Clone, Copy, Hash)]
pub enum TakeProfitTriggerPriceType {
    Last,
    Index,
    Mark,
}

impl_string_enum!(TakeProfitTriggerPriceType,
    Last => "last",
    Index => "index",
    Mark => "mark",
);

#[derive(Debug, Clone, Copy, Hash)]
pub enum StopLossTriggerPriceType {
    Last,
    Index,
    Mark,
}

impl_string_enum!(StopLossTriggerPriceType,
    Last => "last",
    Index => "index",
    Mark => "mark",
);

#[derive(Debug, Clone, Copy, Hash)]
pub enum Category {
    Normal,
    Twap,
    Adl,
    FullLiquidation,
    PartialLiquidation,
    Delivery,
    Ddh,
}

impl_string_enum!(Category,
    Normal => "normal",
    Twap => "twap",
    Adl => "adl",
    FullLiquidation => "full_liquidation",
    PartialLiquidation => "partial_liquidation",
    Delivery => "delivery",
    Ddh => "ddh",
);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InstrumentStatus {
    // live
    Live,
    // suspend
    Suspend,
    // preopen. e.g. There will be preopen before the Futures and Options new contracts state is live.
    Preopen,
    // test: Test pairs, can't be traded
    Test,
}

impl_string_enum!(InstrumentStatus,
    Live => "live",
    Suspend => "suspend",
    Preopen => "preopen",
    Test => "test",
);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OptionType {
    Call,
    Put,
}

impl_string_enum!(OptionType,
    Call => "C",
    Put => "P",
);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ContractType {
    Linear,
    Inverse,
}

impl_string_enum!(ContractType,
    Linear => "linear",
    Inverse => "inverse",
);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FutureType {
    ThisWeek,
    NextWeek,
    Quarter,
    NextQuarter,
}

impl_string_enum!(FutureType,
    ThisWeek => "this_week",
    NextWeek => "next_week",
    Quarter => "quarter",
    NextQuarter => "next_quarter",
);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DeliveryExerciseHistoryType {
    /// Delivery
    Delivery,
    /// Exercised
    Exercised,
    /// Expired out of the money
    ExpiredOtm,
}

impl_string_enum!(DeliveryExerciseHistoryType,
    Delivery => "delivery",
    Exercised => "exercised",
    ExpiredOtm => "expired_otm",
);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CandleState {
    Uncompleted,
    Completed,
}

impl_string_enum!(CandleState,
    Uncompleted => "0",
    Completed => "1",
);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SelfTradePreventionMode {
    CancelMaker,
    CancelTaker,
    CancelBoth,
}

impl_string_enum!(SelfTradePreventionMode,
    CancelMaker => "cancel_maker",
    CancelTaker => "cancel_taker",
    CancelBoth => "cancel_both",
);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SubAccountBillType {
    MasterToSubAccount,
    SubAccountToMaster,
}

impl_string_enum!(SubAccountBillType,
    MasterToSubAccount => "0",
    SubAccountToMaster => "1",
);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OKXSystemTime {
    // System time
    pub ts: MaybeU64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "uly", deserialize_with = "deserialize_from_opt_str")]
    pub underlying: Option<String>, // Only applicable to FUTURES/SWAP/OPTION
    pub category: String, // Fee schedule
    #[serde(rename = "baseCcy", deserialize_with = "deserialize_from_opt_str")]
    pub base_currency: Option<String>, // Only applicable to SPOT/MARGIN
    #[serde(rename = "quoteCcy", deserialize_with = "deserialize_from_opt_str")]
    pub quote_currency: Option<String>, // Only applicable to SPOT/MARGIN
    #[serde(rename = "settleCcy", deserialize_with = "deserialize_from_opt_str")]
    pub margin_currency: Option<String>, // Settlement and margin currency; Only applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "ctVal", default, with = "str_opt")]
    pub face_value: MaybeFloat, // Contract value; Only applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "ctMult", default, with = "str_opt")]
    pub contract_multiplier: MaybeFloat, // Only applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "ctValCcy", deserialize_with = "deserialize_from_opt_str")]
    pub contract_value_currency: Option<String>, // Only applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "optType", deserialize_with = "deserialize_from_opt_str")]
    pub option_type: Option<OptionType>, // Only applicable to OPTION
    #[serde(rename = "stk", default, with = "str_opt")]
    pub strike_price: MaybeFloat, // Only applicable to OPTION
    #[serde(rename = "listTime", default, with = "str_opt")]
    pub listing_time: MaybeU64,
    #[serde(rename = "expTime", default, with = "str_opt")]
    pub expiry_time: MaybeU64,
    #[serde(rename = "lever", default, with = "str_opt")]
    pub max_leverage: MaybeFloat, // Only applicable to FUTURES/OPTION; Not applicable to SPOT, OPTION
    #[serde(rename = "tickSz", default, with = "str_opt")]
    pub tick_size: MaybeFloat,
    #[serde(rename = "lotSz", default, with = "str_opt")]
    pub lot_size: MaybeFloat,
    #[serde(rename = "minSz", default, with = "str_opt")]
    pub min_size: MaybeFloat,
    #[serde(rename = "ctType", deserialize_with = "deserialize_from_opt_str")]
    pub contract_type: Option<ContractType>, // Only applicable to FUTURES/SWAP
    #[serde(rename = "alias", deserialize_with = "deserialize_from_opt_str")]
    pub future_type: Option<FutureType>, // Only applicable to FUTURES
    #[serde(rename = "state")]
    pub status: InstrumentStatus,
    #[serde(rename = "maxLmtSz", default, with = "str_opt")]
    pub max_lmt_size: MaybeFloat, // The maximum order quantity of the contract or spot limit order
    #[serde(rename = "maxMktSz", default, with = "str_opt")]
    pub max_mkt_size: MaybeFloat, // The maximum order quantity of the contract or spot market order
    #[serde(rename = "maxTwapSz", default, with = "str_opt")]
    pub max_twap_size: MaybeFloat, // The maximum order quantity of the contract or spot twap order
    #[serde(rename = "maxIcebergSz", default, with = "str_opt")]
    pub max_iceberg_size: MaybeFloat, // The maximum order quantity of the contract or spot iceBerg order
    #[serde(rename = "maxTriggerSz", default, with = "str_opt")]
    pub max_trigger_size: MaybeFloat, // The maximum order quantity of the contract or spot trigger order
    #[serde(rename = "maxStopSz", default, with = "str_opt")]
    pub max_stop_size: MaybeFloat, // The maximum order quantity of the contract or spot stop order
}

// ========== Trading ==========
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradingBalanceDetail {
    /// Update time of account information, millisecond format of Unix timestamp, e.g. 1597026383085
    #[serde(deserialize_with = "deserialize_from_opt_str")]
    pub u_time: Option<u64>,
    /// The total amount of equity in USD
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub total_eq: MaybeFloat,
    /// Isolated margin equity in USD
    // Applicable to Single-currency margin and Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub iso_eq: MaybeFloat,
    /// Adjusted / Effective equity in USD
    /// The net fiat value of the assets in the account that can provide margins for spot, futures, perpetual swap and options under the cross margin mode.
    /// Cause in multi-ccy or PM mode, the asset and margin requirement will all be converted to USD value to process the order check or liquidation.
    /// Due to the volatility of each currency market, our platform calculates the actual USD value of each currency based on discount rates to balance market risks.
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub adj_eq: MaybeFloat,
    /// Cross margin frozen for pending orders in USD
    /// Only applicable to Multi-currency margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub ord_froz: MaybeFloat,
    /// Initial margin requirement in USD
    /// The sum of initial margins of all open positions and pending orders under cross margin mode in USD.
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub imr: MaybeFloat,
    /// Maintenance margin requirement in USD
    /// The sum of maintenance margins of all open positions under cross margin mode in USD.
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub mmr: MaybeFloat,
    /// Potential borrowing IMR of the account in USD
    /// Only applicable to Multi-currency margin and Portfolio margin. It is "" for other margin modes.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub borrow_froz: MaybeFloat,
    /// Margin ratio in USD
    /// The index for measuring the risk of a certain asset in the account.
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub mgn_ratio: MaybeFloat,
    /// Notional value of positions in USD
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub notional_usd: MaybeFloat,
    /// Detailed asset information in all currencies
    pub details: Vec<TradingBalance>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradingBalance {
    /// Cash Balance
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub cash_bal: MaybeFloat,
    /// Equity of the currency
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub eq: MaybeFloat,
    /// Currency
    pub ccy: String,
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub u_time: Option<u64>,
    /// Isolated margin equity of the currency
    /// Applicable to Single-currency margin and Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub iso_eq: MaybeFloat,
    /// Available equity of the currency
    /// The balance that can be used on margin or futures/swap trading.
    /// Applicable to Single-currency margin, Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub avail_eq: MaybeFloat,
    /// Discount equity of the currency in USD.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub dis_eq: MaybeFloat,
    /// Frozen balance
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub fixed_bal: MaybeFloat,
    /// Available balance of the currency
    /// The balance that can be withdrawn or transferred or used on spot trading.
    /// Applicable to Simple, Single-currency margin, Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub avail_bal: MaybeFloat,
    /// Frozen balance of the currency
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub frozen_bal: MaybeFloat,
    /// Margin frozen for open orders
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub ord_frozen: MaybeFloat,
    /// Liabilities of the currency
    /// It is a positive value, e.g."21625.64". Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub liab: MaybeFloat,
    /// The sum of the unrealized profit & loss of all margin and derivatives positions of the currency.
    /// Applicable to Single-currency margin, Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub upl: MaybeFloat,
    /// Liabilities due to Unrealized loss of the currency
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub upl_liab: MaybeFloat,
    /// Cross liabilities of the currency
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub cross_liab: MaybeFloat,
    /// Isolated liabilities of the currency
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub iso_liab: MaybeFloat,
    /// Isolated liabilities of the currency
    /// Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub mgn_ratio: MaybeFloat,
    /// Accrued interest of the currency
    /// It is a positive value, e.g."9.01". Applicable to Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub interest: MaybeFloat,
    /// Risk indicator of auto liability repayment
    /// Divided into multiple levels from 0 to 5, the larger the number, the more likely the auto repayment will be triggered.
    /// Applicable to Multi-currency margin and Portfolio margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub twap: MaybeFloat,
    /// Max loan of the currency
    /// Applicable to cross of Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub max_loan: MaybeFloat,
    /// Equity in USD of the currency
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub eq_usd: MaybeFloat,
    /// Potential borrowing IMR of the currency in USD
    /// Only applicable to Multi-currency margin and Portfolio margin. It is "" for other margin modes.
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub borrow_froz: MaybeFloat,
    /// Leverage of the currency
    /// Applicable to Single-currency margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub notional_level: MaybeFloat,
    /// Strategy equity
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub stgy_eq: MaybeFloat,
    /// Isolated unrealized profit and loss of the currency
    /// Applicable to Single-currency margin and Multi-currency margin and Portfolio margin
    #[serde(default, deserialize_with = "deserialize_from_opt_str")]
    pub iso_upl: MaybeFloat,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundingBalance {
    /// Available balance
    /// The balance that can be withdrawn or transferred or used for spot trading
    #[serde(default, with = "str_opt")]
    pub avail_bal: MaybeFloat,
    /// Balance
    #[serde(default, with = "str_opt")]
    pub bal: MaybeFloat,
    /// Frozen balance
    #[serde(default, with = "str_opt")]
    pub frozen_bal: MaybeFloat,
    /// Currency
    pub ccy: String,
}
