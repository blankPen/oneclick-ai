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
