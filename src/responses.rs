#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub success: bool,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub message: String,
}
