use serde::Serialize;

use crate::parser_source_document::{DeviceParserSource, OsParserSource, UserAgentParserSource};

#[derive(Debug, Serialize)]
pub struct DeviceParserTarget {
    pub regex: String,
    pub device_replacement: String,
    pub brand_replacement: String,
    pub model_replacement: String,
}

impl From<DeviceParserSource> for DeviceParserTarget {
    fn from(value: DeviceParserSource) -> Self {
        Self {
            regex: apply_regex_flag(value.regex_flag, value.regex),
            device_replacement: value.device_replacement.unwrap_or("$1".to_owned()),
            brand_replacement: value.brand_replacement.unwrap_or("$2".to_owned()),
            model_replacement: value.model_replacement.unwrap_or("$1".to_owned()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OsParserTarget {
    pub regex: String,
    pub os_replacement: String,
    pub os_v1_replacement: String,
    pub os_v2_replacement: String,
    pub os_v3_replacement: String,
    pub os_v4_replacement: String,
}

impl From<OsParserSource> for OsParserTarget {
    fn from(value: OsParserSource) -> Self {
        Self {
            regex: apply_regex_flag(value.regex_flag, value.regex),
            os_replacement: value.os_replacement.unwrap_or("$1".to_owned()),
            os_v1_replacement: value.os_v1_replacement.unwrap_or("$2".to_owned()),
            os_v2_replacement: value.os_v2_replacement.unwrap_or("$3".to_owned()),
            os_v3_replacement: value.os_v3_replacement.unwrap_or("$4".to_owned()),
            os_v4_replacement: value.os_v4_replacement.unwrap_or("$5".to_owned()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserAgentParserTarget {
    pub regex: String,
    pub family_replacement: String,
    pub v1_replacement: String,
    pub v2_replacement: String,
}

impl From<UserAgentParserSource> for UserAgentParserTarget {
    fn from(value: UserAgentParserSource) -> Self {
        Self {
            regex: apply_regex_flag(value.regex_flag, value.regex),
            family_replacement: value.family_replacement.unwrap_or("$1".to_owned()),
            v1_replacement: value.v1_replacement.unwrap_or("$2".to_owned()),
            v2_replacement: value.v2_replacement.unwrap_or("$3".to_owned()),
        }
    }
}

fn apply_regex_flag(flag: Option<String>, regex: String) -> String {
    let Some(flg) = flag else {
        return regex.to_owned();
    };
    format!("(?{}:{})", flg, regex)
}
