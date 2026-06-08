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
