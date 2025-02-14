use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeviceParserSource {
    pub regex: String,
    pub regex_flag: Option<String>,
    pub device_replacement: Option<String>,
    pub brand_replacement: Option<String>,
    pub model_replacement: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OsParserSource {
    pub regex: String,
    pub regex_flag: Option<String>,
    pub os_replacement: Option<String>,
    pub os_v1_replacement: Option<String>,
    pub os_v2_replacement: Option<String>,
    pub os_v3_replacement: Option<String>,
    pub os_v4_replacement: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserAgentParserSource {
    pub regex: String,
    pub regex_flag: Option<String>,
    pub family_replacement: Option<String>,
    pub v1_replacement: Option<String>,
    pub v2_replacement: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ParserSourceDocument {
    pub device_parsers: Vec<DeviceParserSource>,
    pub os_parsers: Vec<OsParserSource>,
    pub user_agent_parsers: Vec<UserAgentParserSource>,
}
