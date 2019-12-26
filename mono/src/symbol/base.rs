use spdcp::Comment;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Documentation {
    #[serde(default)]
    pub doc_start: DocLocation,
    #[serde(default)]
    pub doc_end: DocLocation,

    pub docs: Option<Comment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Declaration {
    pub name: String,

    #[serde(flatten)]
    pub documentation: Documentation,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DocLocation(i64);

impl Default for DocLocation {
    fn default() -> Self {
        Self(0)
    }
}

impl From<usize> for DocLocation {
    fn from(v: usize) -> Self {
        Self(v as i64)
    }
}

impl Into<usize> for DocLocation {
    fn into(self) -> usize {
        self.0 as usize
    }
}