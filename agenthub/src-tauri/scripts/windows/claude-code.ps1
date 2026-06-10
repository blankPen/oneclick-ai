param([string]$Cmd = "check")

function Get-ClaudeInstallation {
    $result = @{
        Installed = $false
        Version = $null
        Method = $null
    }
    
    $cmd = Get-Command claude -ErrorAction SilentlyContinue
    if ($cmd) {
        $result.Installed = $true
        $result.Version = (claude --version 2>$null | Select-Object -First 1).Trim()
        
        # Detect installation method
        if (npm list -g @anthropic-ai/claude-code 2>$null | Select-String -Quiet "claude-code") {
            $result.Method = "npm"
        } elseif (Get-Command winget -ErrorAction SilentlyContinue) {
            $wingetList = winget list --accept-source-agreements 2>$null | Select-String -Quiet "Anthropic.ClaudeCode"
            if ($wingetList) {
                $result.Method = "winget"
            }
        } elseif (Test-Path "$env:USERPROFILE\.local\bin\claude.exe") {
            $result.Method = "native"
        } else {
            $result.Method = "unknown"
        }
    }
    
    return $result
}

function Uninstall-Claude {
    $installed = Get-ClaudeInstallation
    
    if (-not $installed.Installed) {
        Write-Output "Claude Code is not installed."
        return
    }
    
    Write-Output "Detected installation method: $($installed.Method)"
    
    switch ($installed.Method) {
        "npm" {
            Write-Output "Uninstalling via npm..."
            npm uninstall -g @anthropic-ai/claude-code
        }
        "winget" {
            Write-Output "Uninstalling via winget..."
            winget uninstall Anthropic.ClaudeCode
        }
        "native" {
            Write-Output "Uninstalling native installation..."
            Remove-Item -Path "$env:USERPROFILE\.local\bin\claude.exe" -Force -ErrorAction SilentlyContinue
            Remove-Item -Path "$env:USERPROFILE\.local\share\claude" -Recurse -Force -ErrorAction SilentlyContinue
        }
        default {
            Write-Output "Could not detect installation method. Please uninstall manually."
        }
    }
}

switch ($Cmd) {
    "check" {
        $result = Get-ClaudeInstallation
        if ($result.Installed) {
            Write-Output "{`"installed`": true, `"version`": `"$($result.Version)`", `"method`": `"$($result.Method)`"}"
        } else {
            Write-Output "{`"installed`": false}"
        }
    }
    "install" {
        Write-Output "Installing Claude Code via npm..."
        npm install -g @anthropic-ai/claude-code
    }
    "uninstall" {
        Write-Output "Uninstalling Claude Code..."
        Uninstall-Claude
    }
    "update" {
        if (Get-Command claude -ErrorAction SilentlyContinue) {
            claude update
        } else {
            Write-Output "Claude Code is not installed."
            exit 1
        }
    }
    default {
        Write-Error "Unknown command: $Cmd"
        exit 1
    }
}
