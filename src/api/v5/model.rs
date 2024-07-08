use crate::serde_util::*;
use serde::{Deserialize, Serialize};

use crate::serde_util::MaybeU64;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OKXSystemTime {
    #[serde(default, with = "str_opt")]
    pub ts: MaybeU64,
}
