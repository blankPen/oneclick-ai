use tokio::process::Command as AsyncCommand;
use log::error;

#[cfg(target_os = "macos")]
const PLATFORM_SCRIPT_DIR: &str = "scripts/darwin";

#[cfg(target_os = "windows")]
const PLATFORM_SCRIPT_DIR: &str = "scripts/windows";

fn get_script_path(tool_id: &str, _cmd: &str) -> String {
    let ext = if cfg!(target_os = "windows") { "ps1" } else { "sh" };
    format!("{}/{}/{}.{}", env!("CARGO_MANIFEST_DIR"), PLATFORM_SCRIPT_DIR, tool_id, ext)
}

pub async fn check_claude_code() -> Result<String, String> {
    let script = get_script_path("claude-code", "check");
    run_script(&script, "check").await
}

pub async fn install_claude_code() -> Result<String, String> {
    let script = get_script_path("claude-code", "install");
    run_script(&script, "install").await
}

pub async fn uninstall_claude_code() -> Result<String, String> {
    let script = get_script_path("claude-code", "uninstall");
    run_script(&script, "uninstall").await
}

pub async fn update_claude_code() -> Result<String, String> {
    let script = get_script_path("claude-code", "update");
    run_script(&script, "update").await
}

async fn run_script(script_path: &str, _cmd: &str) -> Result<String, String> {
    let output = if cfg!(target_os = "windows") {
        AsyncCommand::new("powershell")
            .args(["-ExecutionPolicy", "Bypass", "-File", script_path])
            .output()
            .await
            .map_err(|e| e.to_string())?
    } else {
        AsyncCommand::new("bash")
            .arg(script_path)
            .output()
            .await
            .map_err(|e| e.to_string())?
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        error!("Script failed: {}", stderr);
        return Err(stderr);
    }

    Ok(stdout.trim().to_string())
}

// Environment detection
pub async fn check_node_version() -> Result<String, String> {
    let output = AsyncCommand::new("node")
        .args(["--version"])
        .output()
        .await
        .map_err(|e| e.to_string())?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err("not found".to_string())
    }
}

pub async fn check_npm_version() -> Result<String, String> {
    let output = AsyncCommand::new("npm")
        .args(["--version"])
        .output()
        .await
        .map_err(|e| e.to_string())?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err("not found".to_string())
    }
}

pub async fn check_python_version() -> Result<String, String> {
    // Try python3 first
    let output = AsyncCommand::new("python3")
        .args(["--version"])
        .output()
        .await
        .map_err(|e| e.to_string())?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
    }
    // Try python as fallback
    let output = AsyncCommand::new("python")
        .args(["--version"])
        .output()
        .await
        .map_err(|e| e.to_string())?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err("not found".to_string())
    }
}

pub async fn check_git_version() -> Result<String, String> {
    let output = AsyncCommand::new("git")
        .args(["--version"])
        .output()
        .await
        .map_err(|e| e.to_string())?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err("not found".to_string())
    }
}

pub async fn check_disk_space() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        let output = AsyncCommand::new("df")
            .args(["-h", "/"])
            .output()
            .await
            .map_err(|e| e.to_string())?;
        if output.status.success() {
            let out = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = out.lines().nth(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    return Ok(parts[3].to_string());
                }
            }
            Ok("unknown".to_string())
        } else {
            Err("failed".to_string())
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        Ok("unknown".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_claude_code_returns_valid_json() {
        let result = check_claude_code().await;
        let output = result.expect("check_claude_code should succeed");
        assert!(output.contains("installed"));
    }
}
