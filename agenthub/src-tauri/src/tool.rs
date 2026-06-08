use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallState {
    pub installed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub description: String,
}

impl Tool {
    pub fn claude_code() -> Self {
        Tool {
            id: "claude-code".to_string(),
            name: "Claude Code".to_string(),
            icon: "⚡".to_string(),
            description: "Anthropic 官方 AI 编程助手".to_string(),
        }
    }

    pub fn all() -> Vec<Tool> {
        vec![Self::claude_code()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_state_json_serialization() {
        let state = InstallState {
            installed: true,
            version: Some("2.1.0".to_string()),
        };
        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains("\"installed\":true"));
        assert!(json.contains("\"version\":\"2.1.0\""));
    }

    #[test]
    fn test_install_state_not_installed() {
        let state = InstallState {
            installed: false,
            version: None,
        };
        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains("\"installed\":false"));
        assert!(!json.contains("version"));
    }
}
