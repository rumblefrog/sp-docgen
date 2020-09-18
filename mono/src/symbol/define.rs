use serde::{Deserialize, Serialize};

use crate::symbol::Declaration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Define {
    #[serde(flatten)]
    pub declaration: Declaration,

    pub value: String,
}