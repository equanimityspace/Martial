#[derive(Debug)]
pub struct ManifestSummary {
    pub issuer: String,
    pub ai_present: bool,
    pub ai_description: Option<String>,
    pub error: String,
}

impl Default for ManifestSummary {
    fn default() -> Self {
        Self {
            issuer: "None".to_string(),
            ai_present: false,
            ai_description: None,
            error: "".to_string(),
        }
    }
}

impl ManifestSummary {
    pub fn no_credentials() -> Self {
        Self {
            issuer: "None".to_string(),
            ai_present: false,
            ai_description: Some("No Content Credentials Manifests Found".to_string()),
            error: "".to_string(),
        }
    }
}
