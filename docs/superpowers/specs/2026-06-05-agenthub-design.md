# AgentHub — AI 工具一键安装器

## 1. 项目概述

**项目名称**：AgentHub
**项目类型**：桌面应用（Tauri 2.x）
**核心功能**：通过友好的向导界面，一键安装 Claude Code、Codex、OpenCode、OpenClaw、HermesAgent 等 AI 工具，并自动修复缺失的运行时环境。
**目标用户**：非程序员背景的普通用户，想使用 AI 编程助手但被复杂的安装流程阻碍。

---

## 2. 技术架构

### 2.1 技术选型

- **框架**：Tauri 2.x（Rust + Web）
- **前端**：纯 HTML/CSS/JS，单文件，复用原型图设计
- **后端**：Rust，模块化设计
- **目标平台**：macOS + Windows（MVP 阶段）

### 2.2 Rust 模块设计

```
src-tauri/src/
├── main.rs              # 入口，命令注册
├── env.rs               # 环境检测（Node.js、Python、Git、npm、pip、磁盘空间）
├── installer.rs         # 安装逻辑（调用系统命令）
├── autofix.rs           # 自动修复缺失环境（brew / winget）
├── uninstaller.rs       # 卸载已安装工具
├── updater.rs           # 检测并更新工具
└── log.rs               # 安装日志收集
```

### 2.3 前端结构

- 单 `index.html` 文件，包含完整 CSS 和 JS
- 4 步向导：欢迎 → 选择工具 → 环境检测 → 安装进度
- 与 Rust 后端通过 Tauri IPC（invoke）通信

---

## 3. 安装工具清单

| 工具 | 安装命令 | 依赖环境 |
|------|---------|---------|
| Claude Code | `npm install -g @anthropic/claude-code` | Node.js 18+ |
| Codex | `npm install -g openai-codex` | Node.js 18+ |
| OpenCode | 下载 GitHub release 二进制 + PATH | Go 运行时（可选） |
| OpenClaw | `pip install openclaw` | Python 3.10+ |
| HermesAgent | `pip install hermes-agent` | Python 3.10+ |

**OpenCode 安装方式**：从 GitHub releases 下载对应平台的二进制文件，赋予执行权限后放入 `$PATH` 目录（如 `~/.local/bin`）。

---

## 4. 环境检测与自动修复

### 4.1 检测项目

| 检测项 | macOS 检查方式 | Windows 检查方式 |
|--------|--------------|----------------|
| Node.js | `node --version` | 同 |
| npm | `npm --version` | 同 |
| Python | `python3 --version` | `python --version` |
| pip | `pip3 --version` | `pip --version` |
| Git | `git --version` | 同 |
| 磁盘空间 | `df -h` | `wmic logicaldisk get size,freespace` |

### 4.2 自动修复策略

当检测到环境缺失时，Rust 后端自动调用包管理器安装：

**macOS**（优先使用 Homebrew）：
- Node.js 缺失 → `brew install node@20`
- Python 缺失 → `brew install python@3.11`
- Git 缺失 → `brew install git`

**Windows**（优先使用 Winget）：
- Node.js 缺失 → `winget install OpenJS.NodeJS.LTS`
- Python 缺失 → `winget install Python.Python.3.11`
- Git 缺失 → `winget install Git.Git`

**安装顺序**：先安装环境依赖，再安装工具本身。

### 4.3 重试与错误处理

- 网络请求超时：重试 3 次，每次间隔 5 秒
- 权限不足：提示用户以管理员/sudo 权限重试
- 安装失败：记录错误日志，向前端返回友好错误信息

---

## 5. 卸载与更新

### 5.1 卸载

| 工具 | 卸载命令 |
|------|---------|
| Claude Code | `npm uninstall -g @anthropic/claude-code` |
| Codex | `npm uninstall -g openai-codex` |
| OpenCode | 删除二进制文件 + 从 PATH 移除 |
| OpenClaw | `pip uninstall openclaw` |
| HermesAgent | `pip uninstall hermes-agent` |

### 5.2 更新

- 检测已安装版本：`npm list -g --depth=0` / `pip list`
- 与最新版本对比
- 执行更新：`npm update -g <pkg>` / `pip install --upgrade <pkg>` / 下载新二进制

---

## 6. 前端交互流程

沿用原型图的 4 步向导设计：

1. **欢迎页**：介绍功能，展示可安装工具列表
2. **选择工具**：卡片式多选界面
3. **环境检测**：实时检测，缺失项高亮，支持"自动修复"按钮
4. **安装进度**：每个工具独立进度条 + 实时日志 + 完成后操作指引

---

## 7. 多平台条件编译

```rust
#[cfg(target_os = "macos")]
fn install_nodejs() { /* brew install */ }

#[cfg(target_os = "windows")]
fn install_nodejs() { /* winget install */ }
```

---

## 8. 暂不在 MVP 范围的功能

- API 密钥配置（安装后由用户自行配置）
- Linux 支持
- 工具的深度配置（如自定义模型端点）
