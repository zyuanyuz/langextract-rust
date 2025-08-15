# LangExtract Rust CLI Installer for Windows PowerShell
# This script installs the langextract-rust CLI tool on Windows

param(
    [switch]$FromCrates,
    [switch]$Help
)

# Configuration
$RepoUrl = "https://github.com/modularflow/langextract-rust"
$BinaryName = "lx-rs"

function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

function Write-Step($Message) {
    Write-ColorOutput Cyan "▶ $Message"
}

function Write-Success($Message) {
    Write-ColorOutput Green "✅ $Message"
}

function Write-Warning($Message) {
    Write-ColorOutput Yellow "⚠️  $Message"
}

function Write-Error($Message) {
    Write-ColorOutput Red "❌ $Message"
}

function Show-Banner {
    Write-ColorOutput Cyan @"
╔══════════════════════════════════════════════════════════════╗
║                    🚀 LangExtract Rust                      ║
║            CLI Installer for Text Extraction                ║
╚══════════════════════════════════════════════════════════════╝
"@
}

function Test-Command($Command) {
    try {
        if (Get-Command $Command -ErrorAction Stop) {
            return $true
        }
    }
    catch {
        return $false
    }
}

function Install-Rust {
    Write-Step "Installing Rust toolchain..."
    
    # Download and run rustup installer
    $rustupUrl = "https://win.rustup.rs/x86_64"
    $rustupPath = "$env:TEMP\rustup-init.exe"
    
    try {
        Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath
        Start-Process -FilePath $rustupPath -ArgumentList "-y" -Wait
        
        # Add cargo to PATH for current session
        $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
        
        Write-Success "Rust installed successfully"
    }
    catch {
        Write-Error "Failed to install Rust: $($_.Exception.Message)"
        exit 1
    }
    finally {
        if (Test-Path $rustupPath) {
            Remove-Item $rustupPath
        }
    }
}

function Test-Prerequisites {
    Write-Step "Checking prerequisites..."
    
    # Check for Rust/Cargo
    if (-not (Test-Command "cargo")) {
        Write-Warning "Rust/Cargo not found. Installing Rust..."
        Install-Rust
    }
    else {
        Write-Success "Rust/Cargo found"
    }
    
    # Check for Git
    if (-not (Test-Command "git")) {
        Write-Error "Git is required but not installed. Please install Git for Windows first."
        Write-Output "Download from: https://git-scm.com/download/win"
        exit 1
    }
    else {
        Write-Success "Git found"
    }
}

function Install-FromSource {
    Write-Step "Installing langextract-rust from source..."
    
    # Create temporary directory
    $tempDir = Join-Path $env:TEMP "langextract-rust-$(Get-Random)"
    New-Item -ItemType Directory -Path $tempDir -Force | Out-Null
    
    try {
        Push-Location $tempDir
        
        Write-Step "Cloning repository..."
        git clone $RepoUrl .
        
        Write-Step "Building with CLI features..."
        cargo install --path . --features cli --force
        
        Write-Success "Installation completed!"
    }
    catch {
        Write-Error "Installation failed: $($_.Exception.Message)"
        exit 1
    }
    finally {
        Pop-Location
        if (Test-Path $tempDir) {
            Remove-Item $tempDir -Recurse -Force
        }
    }
}

function Install-FromCratesIo {
    Write-Step "Installing langextract-rust from crates.io..."
    
    try {
        cargo install langextract-rust --features cli --force
        Write-Success "Installation completed!"
    }
    catch {
        Write-Error "Installation failed: $($_.Exception.Message)"
        exit 1
    }
}

function Set-Environment {
    Write-Step "Setting up environment..."
    
    $cargoPath = "$env:USERPROFILE\.cargo\bin"
    
    # Check if cargo bin is in PATH
    if ($env:PATH -notlike "*$cargoPath*") {
        Write-Step "Adding cargo bin directory to PATH..."
        
        # Get current user PATH
        $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
        $newPath = "$cargoPath;$currentPath"
        
        # Set user PATH
        [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
        
        # Update current session
        $env:PATH = "$cargoPath;$env:PATH"
        
        Write-Success "Added to user PATH"
    }
}

function Test-Installation {
    Write-Step "Verifying installation..."
    
    $binaryPath = "$env:USERPROFILE\.cargo\bin\$BinaryName.exe"
    
    if (Test-Path $binaryPath) {
        Write-Success "Binary installed at $binaryPath"
        
        # Test the binary
        try {
            & $binaryPath --version | Out-Null
            Write-Success "Installation verified successfully!"
            
            Write-Output ""
            Write-ColorOutput Green "🎉 LangExtract Rust CLI installed successfully!"
            Write-Output ""
            Write-Output "Usage examples:"
            Write-ColorOutput Cyan "  $BinaryName extract 'John Doe is 30 years old' --prompt 'Extract names and ages'"
            Write-ColorOutput Cyan "  $BinaryName providers"
            Write-ColorOutput Cyan "  $BinaryName init"
            Write-ColorOutput Cyan "  $BinaryName test"
            Write-Output ""
            Write-Output "For more help:"
            Write-ColorOutput Cyan "  $BinaryName --help"
            Write-Output ""
        }
        catch {
            Write-Error "Installation verification failed: $($_.Exception.Message)"
            exit 1
        }
    }
    else {
        Write-Error "Binary not found after installation"
        exit 1
    }
}

function Show-NextSteps {
    Write-Output ""
    Write-ColorOutput Magenta "📚 Next Steps:"
    Write-Output ""
    Write-Output "1. 🔧 Initialize configuration:"
    Write-ColorOutput Cyan "   $BinaryName init"
    Write-Output ""
    Write-Output "2. 🧪 Test your setup:"
    Write-ColorOutput Cyan "   $BinaryName test --provider ollama"
    Write-Output ""
    Write-Output "3. 📖 View examples:"
    Write-ColorOutput Cyan "   $BinaryName examples"
    Write-Output ""
    Write-Output "4. 🚀 Extract from text:"
    Write-ColorOutput Cyan "   $BinaryName extract 'Your text here' --prompt 'What to extract'"
    Write-Output ""
    Write-Output "5. 🔌 Check available providers:"
    Write-ColorOutput Cyan "   $BinaryName providers"
    Write-Output ""
    Write-ColorOutput Yellow "💡 Pro Tips:"
    Write-Output "• Use --verbose for detailed output"
    Write-Output "• Try --export html for rich visualizations"
    Write-Output "• Use --examples file.json for custom extraction patterns"
    Write-Output ""
    Write-Output "• Restart your terminal or run: refreshenv"
    Write-Output ""
    Write-ColorOutput Green "Happy extracting! 🎯"
}

function Show-Help {
    Write-Output "LangExtract Rust Installer for Windows"
    Write-Output ""
    Write-Output "Usage: .\install.ps1 [OPTIONS]"
    Write-Output ""
    Write-Output "Options:"
    Write-Output "  -FromCrates    Install from crates.io instead of source"
    Write-Output "  -Help          Show this help message"
    Write-Output ""
    Write-Output "Examples:"
    Write-Output "  .\install.ps1                # Install from source"
    Write-Output "  .\install.ps1 -FromCrates   # Install from crates.io"
}

function Main {
    if ($Help) {
        Show-Help
        return
    }
    
    Show-Banner
    
    # Check if running as administrator (optional but recommended)
    $isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
    if (-not $isAdmin) {
        Write-Warning "Not running as administrator. Some features may be limited."
    }
    
    Test-Prerequisites
    Set-Environment
    
    if ($FromCrates) {
        Install-FromCratesIo
    }
    else {
        Install-FromSource
    }
    
    Test-Installation
    Show-NextSteps
}

# Handle script interruption
trap {
    Write-Error "Installation interrupted"
    exit 1
}

# Set execution policy for this session (if needed)
if ((Get-ExecutionPolicy) -eq "Restricted") {
    Write-Warning "PowerShell execution policy is restricted. You may need to run:"
    Write-Output "Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser"
    Write-Output ""
}

Main
