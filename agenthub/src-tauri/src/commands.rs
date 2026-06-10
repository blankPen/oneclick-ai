use crate::runtime;
use crate::tool::{EnvStatus, InstallState, Tool};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvCheckResult {
    pub node: EnvStatus,
    pub npm: EnvStatus,
    pub python: EnvStatus,
    pub git: EnvStatus,
    pub disk: EnvStatus,
}

#[tauri::command]
pub async fn get_tools() -> Result<Vec<Tool>, String> {
    Ok(Tool::all())
}

#[tauri::command]
pub async fn check_claude_code_cmd() -> Result<InstallState, String> {
    let output = runtime::check_claude_code().await
        .map_err(|e| e.to_string())?;
    serde_json::from_str(&output).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn check_env_cmd() -> Result<EnvCheckResult, String> {
    let (node, npm, python, git, disk) = tokio::join!(
        runtime::check_node_version(),
        runtime::check_npm_version(),
        runtime::check_python_version(),
        runtime::check_git_version(),
        runtime::check_disk_space(),
    );

    Ok(EnvCheckResult {
        node: match node {
            Ok(v) => EnvStatus { found: true, version: Some(v) },
            Err(_) => EnvStatus { found: false, version: None },
        },
        npm: match npm {
            Ok(v) => EnvStatus { found: true, version: Some(v) },
            Err(_) => EnvStatus { found: false, version: None },
        },
        python: match python {
            Ok(v) => EnvStatus { found: true, version: Some(v) },
            Err(_) => EnvStatus { found: false, version: None },
        },
        git: match git {
            Ok(v) => EnvStatus { found: true, version: Some(v) },
            Err(_) => EnvStatus { found: false, version: None },
        },
        disk: match disk {
            Ok(v) => EnvStatus { found: true, version: Some(v) },
            Err(_) => EnvStatus { found: false, version: None },
        },
    })
}

#[tauri::command]
pub async fn install_claude_code_cmd() -> Result<String, String> {
    runtime::install_claude_code().await
}

#[tauri::command]
pub async fn uninstall_claude_code_cmd() -> Result<String, String> {
    runtime::uninstall_claude_code().await
}

#[tauri::command]
pub async fn update_claude_code_cmd() -> Result<String, String> {
    runtime::update_claude_code().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_tools_returns_at_least_claude_code() {
        let tools = get_tools().await.unwrap();
        assert!(!tools.is_empty());
        assert_eq!(tools[0].id, "claude-code");
    }
}
