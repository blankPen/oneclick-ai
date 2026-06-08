# AgentHub Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Tauri desktop app that can install Claude Code on macOS and Windows via official native install scripts.

**Architecture:** Tauri 2.x with Rust backend and pure HTML/JS frontend. Platform scripts (bash/PowerShell) handle the actual install logic. Rust orchestrates via Tauri IPC commands.

**Tech Stack:** Tauri 2.x, Rust, HTML/CSS/JS, bash, PowerShell

---

## File Structure

```
agenthub/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs              # Tauri app entry
│   │   ├── lib.rs               # Module declarations
│   │   ├── commands.rs          # Tauri IPC command handlers
│   │   ├── tool.rs              # Tool definition structs
│   │   └── runtime.rs           # Script executor
│   ├── scripts/
│   │   ├── darwin/
│   │   │   └── claude-code.sh
│   │   └── windows/
│   │       └── claude-code.ps1
│   ├── Cargo.toml
│   └── tauri.conf.json
├── frontend/
│   └── index.html
└── SPEC.md
```

---

## Task 1: Scaffold Tauri Project

**Files:**
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/tauri.conf.json`
- Create: `src-tauri/src/main.rs`
- Create: `src-tauri/src/lib.rs`
- Create: `src-tauri/build.rs`
- Create: `frontend/index.html`
- Create: `SPEC.md`

- [ ] **Step 1: Create project directories**

```bash
mkdir -p agenthub/src-tauri/src
mkdir -p agenthub/src-tauri/scripts/darwin
mkdir -p agenthub/src-tauri/scripts/windows
mkdir -p agenthub/frontend
```

- [ ] **Step 2: Create Cargo.toml**

```toml
[package]
name = "agenthub"
version = "0.1.0"
edition = "2021"

[lib]
name = "agenthub_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = ["devtools"] }
tauri-plugin-shell = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["process", "io-util"] }
log = "0.4"
env_logger = "0.11"
```

- [ ] **Step 3: Create tauri.conf.json**

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "AgentHub",
  "version": "0.1.0",
  "identifier": "com.agenthub.app",
  "build": {
    "frontendDist": "../frontend",
    "devUrl": "http://localhost:1420",
    "beforeDevCommand": "",
    "beforeBuildCommand": ""
  },
  "app": {
    "windows": [
      {
        "title": "AgentHub — AI 工具安装助手",
        "width": 720,
        "height": 600,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

- [ ] **Step 4: Create build.rs**

```rust
fn main() {
    tauri_build::build()
}
```

- [ ] **Step 5: Create main.rs**

```rust
// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    agenthub_lib::run();
}
```

- [ ] **Step 6: Create lib.rs (stub)**

```rust
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 7: Copy prototype to frontend/index.html**

Copy the content from `public/index.html` into `agenthub/frontend/index.html`.

- [ ] **Step 8: Create SPEC.md**

```markdown
# AgentHub Spec

## Overview
Desktop app to install Claude Code via official native install scripts.

## Supported Platforms
- macOS 13.0+
- Windows 10 1809+

## Install Method
- macOS/Linux: `curl -fsSL https://claude.ai/install.sh | bash`
- Windows: `irm https://claude.ai/install.ps1 | iex`

## Commands
- `claude --version` — verify installation
- `claude update` — update to latest

## Unpublish
- macOS: `rm -f ~/.local/bin/claude && rm -rf ~/.local/share/claude`
- Windows: Remove-Item `$env:USERPROFILE\.local\bin\claude.exe` and `claude` dir
```

- [ ] **Step 9: Commit**

```bash
git add -A && git commit -m "feat: scaffold Tauri project"
```

---

## Task 2: Implement Tool Definition and State

**Files:**
- Create: `src-tauri/src/tool.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Write failing test for InstallState**

Create `src-tauri/src/tool.rs`:

```rust
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
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd agenthub/src-tauri && cargo test --lib -- tool --nocapture
Expected: FAIL — tool.rs doesn't exist yet
```

- [ ] **Step 3: Write minimal tool.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallState {
    pub installed: bool,
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
```

- [ ] **Step 4: Run test to verify it passes**

```bash
cd agenthub/src-tauri && cargo test --lib -- tool
Expected: PASS
```

- [ ] **Step 5: Update lib.rs to export tool module**

```rust
mod tool;
pub use tool::*;
```

- [ ] **Step 6: Commit**

```bash
git add -A && git commit -m "feat: add tool definition and InstallState"
```

---

## Task 3: Implement Script Executor (runtime.rs)

**Files:**
- Create: `src-tauri/src/runtime.rs`
- Create: `src-tauri/scripts/darwin/claude-code.sh`
- Create: `src-tauri/scripts/windows/claude-code.ps1`

- [ ] **Step 1: Write failing test for script executor**

Create `src-tauri/src/runtime.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_claude_code_not_installed() {
        let result = check_claude_code().await;
        // Returns JSON like {"installed": false} or {"installed": true, "version": "..."}
        assert!(result.contains("installed"));
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd agenthub/src-tauri && cargo test --lib -- runtime --nocapture
Expected: FAIL — runtime.rs functions not defined
```

- [ ] **Step 3: Create darwin/claude-code.sh**

```bash
#!/usr/bin/env bash
set -e

CMD="${1:-check}"
CLAUDE_BIN="${HOME}/.local/bin/claude"
CLAUDE_DIR="${HOME}/.local/share/claude"

check_installation() {
    if command -v claude &>/dev/null; then
        VERSION=$(claude --version 2>/dev/null | head -n1 | tr -d '\n')
        printf '{"installed":true,"version":"%s"}' "$VERSION"
    else
        printf '{"installed":false}'
    fi
}

case "$CMD" in
    check)
        check_installation
        ;;
    install)
        printf 'Starting Claude Code installation...\n'
        curl -fsSL https://claude.ai/install.sh | bash
        ;;
    uninstall)
        printf 'Removing Claude Code...\n'
        rm -f "$CLAUDE_BIN"
        rm -rf "$CLAUDE_DIR"
        ;;
    update)
        claude update
        ;;
    *)
        printf 'Unknown command: %s\n' "$CMD" >&2
        exit 1
        ;;
esac
```

- [ ] **Step 4: Create windows/claude-code.ps1**

```powershell
param([string]$Cmd = "check")

$ClaudeBin = "$env:USERPROFILE\.local\bin\claude.exe"
$ClaudeDir = "$env:USERPROFILE\.local\share\claude"

function Check-Installation {
    $cmd = Get-Command claude -ErrorAction SilentlyContinue
    if ($cmd) {
        $version = claude --version 2>$null | Select-Object -First 1
        $version = $version.Trim()
        Write-Output "{`"installed`": true, `"version`": `"$version`"}"
    } else {
        Write-Output "{`"installed`": false}"
    }
}

switch ($Cmd) {
    "check" { Check-Installation }
    "install" {
        Write-Output "Starting Claude Code installation..."
        Invoke-RestMethod https://claude.ai/install.ps1 | Invoke-Expression
    }
    "uninstall" {
        Write-Output "Removing Claude Code..."
        Remove-Item -Path $ClaudeBin -Force -ErrorAction SilentlyContinue
        Remove-Item -Path $ClaudeDir -Recurse -Force -ErrorAction SilentlyContinue
    }
    "update" { claude update }
    default {
        Write-Error "Unknown command: $Cmd"
        exit 1
    }
}
```

- [ ] **Step 5: Write runtime.rs with platform detection**

```rust
use std::process::Command;
use tokio::process::Command as AsyncCommand;
use log::{info, error};

#[cfg(target_os = "macos")]
const PLATFORM_SCRIPT_DIR: &str = "scripts/darwin";

#[cfg(target_os = "windows")]
const PLATFORM_SCRIPT_DIR: &str = "scripts/windows";

fn get_script_path(tool_id: &str, _cmd: &str) -> String {
    let ext = if cfg!(target_os = "windows") { "ps1" } else { "sh" };
    format!("{}/{}/{}.{}", env!("CARGO_MANIFEST_DIR"), PLATFORM_SCRIPT_DIR, tool_id, ext)
}

pub async fn check_claude_code() -> String {
    let script = get_script_path("claude-code", "check");
    run_script(&script, "check").await
}

pub async fn install_claude_code() -> Result<String, String> {
    let script = get_script_path("claude-code", "install");
    Ok(run_script(&script, "install").await)
}

pub async fn uninstall_claude_code() -> Result<String, String> {
    let script = get_script_path("claude-code", "uninstall");
    Ok(run_script(&script, "uninstall").await)
}

pub async fn update_claude_code() -> Result<String, String> {
    let script = get_script_path("claude-code", "update");
    Ok(run_script(&script, "update").await)
}

async fn run_script(script_path: &str, _cmd: &str) -> String {
    let output = if cfg!(target_os = "windows") {
        let out = AsyncCommand::new("powershell")
            .args(["-ExecutionPolicy", "Bypass", "-File", script_path])
            .output()
            .await
            .map_err(|e| e.to_string())?;
        out
    } else {
        let out = AsyncCommand::new("bash")
            .arg(script_path)
            .output()
            .await
            .map_err(|e| e.to_string())?;
        out
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        error!("Script failed: {}", stderr);
    }

    stdout.trim().to_string()
}
```

- [ ] **Step 6: Run test to verify it passes**

```bash
cd agenthub/src-tauri && cargo test --lib -- runtime
Expected: PASS
```

- [ ] **Step 7: Make scripts executable in build.rs**

In build.rs:
```rust
fn main() {
    tauri_build::build();
    #[cfg(not(target_os = "windows"))]
    {
        let scripts_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("scripts/darwin");
        if let Ok(entries) = std::fs::read_dir(scripts_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "sh").unwrap_or(false) {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(metadata) = std::fs::metadata(&path) {
                        let mut perms = metadata.permissions();
                        perms.set_mode(0o755);
                        let _ = std::fs::set_permissions(&path, perms);
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 8: Commit**

```bash
git add -A && git commit -m "feat: add script executor and platform scripts"
```

---

## Task 4: Implement Tauri IPC Commands

**Files:**
- Create: `src-tauri/src/commands.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Write failing test for commands**

Create `src-tauri/src/commands.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_returns_valid_json() {
        let result = check_claude_code_cmd().await;
        let state: InstallState = serde_json::from_str(&result).unwrap();
        assert!(state.installed == false || state.installed == true);
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cd agenthub/src-tauri && cargo test --lib -- commands --nocapture
Expected: FAIL — commands.rs doesn't exist
```

- [ ] **Step 3: Write commands.rs**

```rust
use crate::runtime;
use crate::tool::{InstallState, Tool};
use tauri::Window;

#[tauri::command]
pub async fn get_tools() -> Result<Vec<Tool>, String> {
    Ok(Tool::all())
}

#[tauri::command]
pub async fn check_claude_code_cmd() -> Result<InstallState, String> {
    let output = runtime::check_claude_code().await;
    serde_json::from_str(&output).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn install_claude_code_cmd(window: Window) -> Result<String, String> {
    let output = runtime::install_claude_code().await?;
    Ok(output)
}

#[tauri::command]
pub async fn uninstall_claude_code_cmd() -> Result<String, String> {
    let output = runtime::uninstall_claude_code().await?;
    Ok(output)
}

#[tauri::command]
pub async fn update_claude_code_cmd() -> Result<String, String> {
    let output = runtime::update_claude_code().await?;
    Ok(output)
}
```

- [ ] **Step 4: Update lib.rs to wire up commands**

```rust
mod tool;
mod runtime;
mod commands;

pub use tool::*;
pub use commands::*;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_tools,
            check_claude_code_cmd,
            install_claude_code_cmd,
            uninstall_claude_code_cmd,
            update_claude_code_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 5: Run tests to verify they pass**

```bash
cd agenthub/src-tauri && cargo test --lib -- commands
Expected: PASS
```

- [ ] **Step 6: Commit**

```bash
git add -A && git commit -m "feat: add Tauri IPC commands"
```

---

## Task 5: Build and Verify

**Files:**
- Verify: `agenthub/frontend/index.html`
- Verify: `agenthub/src-tauri/scripts/darwin/claude-code.sh`
- Verify: `agenthub/src-tauri/scripts/windows/claude-code.ps1`

- [ ] **Step 1: Verify all files exist**

```bash
find agenthub -type f | sort
```

- [ ] **Step 2: Try to build the Tauri app**

```bash
cd agenthub && cargo build --release 2>&1 | head -50
```

- [ ] **Step 3: Fix any compilation errors**

Iterate on compilation errors until clean.

- [ ] **Step 4: Commit**

```bash
git add -A && git commit -m "feat: build and verify Tauri app compiles"
```

---

## Spec Coverage Check

- [x] Tauri project scaffold — Task 1
- [x] Tool definition structs — Task 2
- [x] Script executor — Task 3
- [x] Platform scripts (bash + PowerShell) — Task 3
- [x] Tauri IPC commands — Task 4
- [x] Build verification — Task 5
