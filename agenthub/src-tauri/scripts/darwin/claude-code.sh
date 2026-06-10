#!/usr/bin/env bash
set -e

CMD="${1:-check}"

# Detect installation method and uninstall accordingly
detect_and_uninstall() {
    echo "Detecting Claude Code installation method..."
    
    # Check if command exists
    if ! command -v claude &>/dev/null; then
        echo "Claude Code is not installed."
        return 0
    fi
    
    # Try npm first (default)
    if npm list -g @anthropic-ai/claude-code &>/dev/null; then
        echo "Detected: npm installation"
        npm uninstall -g @anthropic-ai/claude-code
        echo "Uninstalled via npm."
        return 0
    fi
    
    # Check Homebrew
    if command -v brew &>/dev/null && brew list --cask claude-code &>/dev/null 2>&1; then
        echo "Detected: Homebrew installation"
        brew uninstall --cask claude-code
        echo "Uninstalled via Homebrew."
        return 0
    fi
    
    # Check native install locations
    if [ -f "${HOME}/.local/bin/claude" ]; then
        echo "Detected: Native installation"
        rm -f "${HOME}/.local/bin/claude"
        rm -rf "${HOME}/.local/share/claude"
        echo "Uninstalled native installation."
        return 0
    fi
    
    # Fallback: try to remove via which
    CLAUDE_PATH=$(which claude 2>/dev/null)
    if [ -n "$CLAUDE_PATH" ]; then
        echo "Detected: Installation at ${CLAUDE_PATH}"
        rm -f "$CLAUDE_PATH"
        # Clean up fnm directory if applicable
        if [[ "$CLAUDE_PATH" == *"fnm_multishells"* ]]; then
            rm -rf "$(dirname "$CLAUDE_PATH")"
        fi
        return 0
    fi
    
    echo "Could not detect installation method. Please uninstall manually."
    return 1
}

check_installation() {
    if command -v claude &>/dev/null; then
        VERSION=$(claude --version 2>/dev/null | head -n1)
        # Try to detect install method
        METHOD="unknown"
        if npm list -g @anthropic-ai/claude-code &>/dev/null 2>&1; then
            METHOD="npm"
        elif brew list --cask claude-code &>/dev/null 2>&1; then
            METHOD="brew"
        elif [ -f "${HOME}/.local/bin/claude" ]; then
            METHOD="native"
        fi
        printf '{"installed":true,"version":"%s","method":"%s"}' "$VERSION" "$METHOD"
    else
        printf '{"installed":false}'
    fi
}

case "$CMD" in
    check)
        check_installation
        ;;
    install)
        echo "Installing Claude Code via npm..."
        npm install -g @anthropic-ai/claude-code
        ;;
    uninstall)
        echo "Uninstalling Claude Code..."
        detect_and_uninstall
        ;;
    update)
        if command -v claude &>/dev/null; then
            claude update
        else
            echo "Claude Code is not installed."
            exit 1
        fi
        ;;
    *)
        echo "Unknown command: $CMD" >&2
        exit 1
        ;;
esac
