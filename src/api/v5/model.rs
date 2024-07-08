use crate::{impl_string_enum, serde_util::*};
use serde::{Deserialize, Serialize};

use crate::serde_util::MaybeU64;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OKXSystemTime {
    #[serde(default, with = "str_opt")]
    pub ts: MaybeU64,
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
