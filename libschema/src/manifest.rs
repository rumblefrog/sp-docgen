use serde::{Deserialize, Serialize};

use super::meta::Meta;

#[derive(Deserialize)]
pub struct Manifest {
    /// Meta descriptor of manifest content
    pub meta: Meta,

    /// Meta content source
    pub source: Source,
}

#[derive(Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    /// Git SSH URL schema
    /// Repository field must be populated
    Git,

    /// Direct HTTP accessor
    /// Endpoints should be list of URL to directly access those files
    Direct,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct Source {
    /// Type of source or method of access
    pub r#type: SourceType,

    /// If true, all sources will be merged into a single namespace when consumed by the UI
    pub merge: Option<bool>,

    /// Mandatory if Git is selected as the type
    pub repository: Option<String>,

    /// Mandatory if Direct is selected as the type
    pub endpoints: Option<Vec<String>>,

    /// Used as regex glob pattern when Git is selected
    pub patterns: Option<Vec<String>>,
}
