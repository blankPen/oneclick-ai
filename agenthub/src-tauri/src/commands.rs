use crate::runtime;
use crate::tool::{InstallState, Tool};
use tauri::Window;

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
pub async fn install_claude_code_cmd(_window: Window) -> Result<String, String> {
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
