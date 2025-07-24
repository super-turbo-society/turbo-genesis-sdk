use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct TurboProgramMetadata {
    pub name: String,
    pub program_id: String,
    pub owner_id: String,
    pub commands: BTreeSet<TurboProgramCommandMetadata>,
    pub channels: BTreeSet<TurboProgramChannelMetadata>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct TurboProgramCommandMetadata {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct TurboProgramChannelMetadata {
    pub name: String,
}
