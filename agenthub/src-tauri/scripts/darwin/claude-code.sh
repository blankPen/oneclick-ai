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
