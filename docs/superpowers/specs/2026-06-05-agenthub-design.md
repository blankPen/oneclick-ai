# AgentHub — AI 工具一键安装器

## 1. 项目概述

**项目名称**：AgentHub
**项目类型**：桌面应用（Tauri 2.x）
**核心功能**：通过友好的向导界面，一键安装 Claude Code 等 AI 工具，并自动修复缺失的运行时环境。
**目标用户**：非程序员背景的普通用户，想使用 AI 编程助手但被复杂的安装流程阻碍。

---

## 2. MVP 范围

**本阶段只实现 Claude Code 的安装、卸载、更新。** 其他工具（Codex、OpenCode、OpenClaw、HermesAgent）先预留定义，暂不实现。

---

## 3. 技术架构

### 3.1 技术选型

- **框架**：Tauri 2.x（Rust + Web）
- **前端**：纯 HTML/CSS/JS，单文件
- **后端**：Rust + 平台脚本（bash / PowerShell）
- **目标平台**：macOS + Windows（MVP 阶段）

### 3.2 项目结构

```
agenthub/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands.rs     # Tauri IPC 命令
│   │   ├── tool.rs        # 工具定义
│   │   └── runtime.rs     # 脚本执行器
│   └── scripts/
│       ├── darwin/
│       │   └── claude-code.sh
│       └── windows/
│           └── claude-code.ps1
└── frontend/
    └── index.html
```

---

## 4. Claude Code 详解

### 4.1 安装方式

**推荐：原生安装脚本**（最简单，支持自动更新）

| 平台 | 命令 |
|------|------|
| macOS / Linux | `curl -fsSL https://claude.ai/install.sh \| bash` |
| Windows PowerShell | `irm https://claude.ai/install.ps1 \| iex` |
| Windows CMD | `curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd` |

**备选：包管理器**

| 平台 | 命令 |
|------|------|
| Homebrew | `brew install --cask claude-code` |
| WinGet | `winget install Anthropic.ClaudeCode` |
| npm | `npm install -g @anthropic-ai/claude-code`（需 Node.js 18+） |

### 4.2 环境要求

- **操作系统**：macOS 13.0+ / Windows 10 1809+ / Ubuntu 20.04+
- **内存**：4GB+
- **网络**：需要互联网连接
- **Shell**：Bash, Zsh, PowerShell, CMD

### 4.3 验证安装

```bash
claude --version
# 或
claude doctor
```

### 4.4 卸载

| 安装方式 | 卸载命令 |
|---------|---------|
| 原生安装 | `rm -f ~/.local/bin/claude && rm -rf ~/.local/share/claude`（macOS/Linux）|
| 原生安装 | Windows: 删除 `$env:USERPROFILE\.local\bin\claude.exe` 和 `$env:USERPROFILE\.local\share\claude` |
| Homebrew | `brew uninstall --cask claude-code` |
| WinGet | `winget uninstall Anthropic.ClaudeCode` |
| npm | `npm uninstall -g @anthropic-ai/claude-code` |

### 4.5 更新

- **原生安装**：自动后台更新
- **其他方式**：`claude update` 或对应包管理器更新命令

---

## 5. 平台脚本设计

### 5.1 脚本接口

每个脚本通过 stdout/ stderr 输出日志，退出码表示成功/失败：

```bash
# 检测是否已安装
./check.sh
# 输出: {"installed": true, "version": "2.1.89"}

# 安装
./install.sh
# 输出: 实时日志
# 退出码: 0 成功, 1 失败

# 卸载
./uninstall.sh
# 退出码: 0 成功, 1 失败

# 更新
./update.sh
# 退出码: 0 成功, 1 失败
```

### 5.2 macOS 脚本 (claude-code.sh)

```bash
#!/usr/bin/env bash
set -e

CMD="$1"
CLAUDE_BIN="$HOME/.local/bin/claude"
CLAUDE_DIR="$HOME/.local/share/claude"

case "$CMD" in
  check)
    if command -v claude &>/dev/null; then
      VERSION=$(claude --version 2>/dev/null | head -n1)
      echo "{\"installed\": true, \"version\": \"$VERSION\"}"
    else
      echo "{\"installed\": false}"
    fi
    ;;
  install)
    echo "Installing Claude Code..."
    curl -fsSL https://claude.ai/install.sh | bash
    ;;
  uninstall)
    echo "Uninstalling Claude Code..."
    rm -f "$CLAUDE_BIN"
    rm -rf "$CLAUDE_DIR"
    ;;
  update)
    claude update
    ;;
esac
```

### 5.3 Windows 脚本 (claude-code.ps1)

```powershell
param([string]$Cmd)

$ClaudeBin = "$env:USERPROFILE\.local\bin\claude.exe"
$ClaudeDir = "$env:USERPROFILE\.local\share\claude"

switch ($Cmd) {
  "check" {
    if (Get-Command claude -ErrorAction SilentlyContinue) {
      $Version = claude --version 2>$null | Select-Object -First 1
      Write-Output "{`"installed`": true, `"version`": `"$Version`"}"
    } else {
      Write-Output "{`"installed`": false}"
    }
  }
  "install" {
    Write-Output "Installing Claude Code..."
    Invoke-RestMethod https://claude.ai/install.ps1 | Invoke-Expression
  }
  "uninstall" {
    Write-Output "Uninstalling Claude Code..."
    Remove-Item -Path $ClaudeBin -Force -ErrorAction SilentlyContinue
    Remove-Item -Path $ClaudeDir -Recurse -Force -ErrorAction SilentlyContinue
  }
  "update" {
    claude update
  }
}
```

---

## 6. Rust 后端

### 6.1 Tauri IPC 命令

```rust
#[tauri::command]
fn check_claude_code() -> Result<InstallState, String>;

#[tauri::command]
fn install_claude_code(window: Window) -> Result<(), InstallError>;

#[tauri::command]
fn uninstall_claude_code() -> Result<(), UninstallError>;

#[tauri::command]
fn update_claude_code() -> Result<(), UpdateError>;
```

### 6.2 InstallState 结构

```rust
struct InstallState {
    installed: bool,
    version: Option<String>,
    install_method: Option<String>, // "native" | "homebrew" | "winget" | "npm"
}
```

---

## 7. 前端交互流程

沿用原型图设计，4 步向导：

1. **欢迎页**：介绍功能
2. **选择工具**：目前只有 Claude Code 一个选项（后续扩展）
3. **环境检测**：检测 Claude Code 是否已安装，显示版本
4. **安装进度**：显示安装/卸载/更新进度和日志

---

## 8. 错误处理

| 错误类型 | 处理方式 |
|---------|---------|
| 网络超时 | 重试 3 次，间隔 5s |
| 权限不足 | 提示用户以管理员/sudo 运行 |
| 安装失败 | 显示错误日志 |
| 验证失败 | 提示重新尝试 |
