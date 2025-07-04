#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TurboProgramMetadata {
    pub name: String,
    pub program_id: String,
    pub owner_id: String,
    pub commands: Vec<TurboProgramCommandMetadata>,
    pub channels: Vec<TurboProgramChannelMetadata>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TurboProgramCommandMetadata {
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TurboProgramChannelMetadata {
    pub name: String,
}
