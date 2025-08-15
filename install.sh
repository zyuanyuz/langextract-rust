#!/bin/bash
set -e

# LangExtract Rust CLI Installer
# This script installs the langextract-rust CLI tool

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
REPO_URL="https://github.com/modularflow/langextract-rust"
CARGO_BIN_DIR="$HOME/.cargo/bin"
BINARY_NAME="lx-rs"

print_banner() {
    echo -e "${CYAN}"
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║                     LangExtract Rust                         ║"
    echo "║            CLI Installer for Text Extraction                 ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

print_step() {
    echo -e "${BLUE}▶${NC} $1"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

check_command() {
    if command -v "$1" >/dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

install_rust() {
    print_step "Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    print_success "Rust installed successfully"
}

check_prerequisites() {
    print_step "Checking prerequisites..."
    
    # Check for Rust
    if ! check_command "cargo"; then
        print_warning "Rust/Cargo not found. Installing Rust..."
        install_rust
    else
        print_success "Rust/Cargo found"
    fi
    
    # Check for git
    if ! check_command "git"; then
        print_error "Git is required but not installed. Please install git first."
        exit 1
    else
        print_success "Git found"
    fi
    
    # Check for curl
    if ! check_command "curl"; then
        print_error "curl is required but not installed. Please install curl first."
        exit 1
    else
        print_success "curl found"
    fi
}

install_from_source() {
    print_step "Installing langextract-rust from source..."
    
    # Create temporary directory
    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"
    
    print_step "Cloning repository..."
    git clone "$REPO_URL" .
    
    print_step "Building with CLI features..."
    cargo install --path . --features cli --force
    
    # Cleanup
    cd - > /dev/null
    rm -rf "$TEMP_DIR"
    
    print_success "Installation completed!"
}

install_from_crates_io() {
    print_step "Installing langextract-rust from crates.io..."
    cargo install langextract-rust --features cli --force
    print_success "Installation completed!"
}

install_prebuilt_binary() {
    print_step "Installing pre-built binary from GitHub releases..."
    
    # Detect platform
    OS="$(uname -s)"
    ARCH="$(uname -m)"
    
    case "$OS" in
        Linux*)
            if [ "$ARCH" = "x86_64" ]; then
                PLATFORM="x86_64-unknown-linux-gnu"
            else
                print_error "Unsupported architecture: $ARCH"
                exit 1
            fi
            ;;
        Darwin*)
            if [ "$ARCH" = "x86_64" ]; then
                PLATFORM="x86_64-apple-darwin"
            elif [ "$ARCH" = "arm64" ]; then
                PLATFORM="aarch64-apple-darwin"
            else
                print_error "Unsupported architecture: $ARCH"
                exit 1
            fi
            ;;
        *)
            print_error "Unsupported operating system: $OS"
            exit 1
            ;;
    esac
    
    # Get latest release
    LATEST_VERSION=$(curl -s https://api.github.com/repos/modularflow/langextract-rust/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    
    if [ -z "$LATEST_VERSION" ]; then
        print_error "Could not determine latest version"
        exit 1
    fi
    
    print_step "Downloading version $LATEST_VERSION for $PLATFORM..."
    
    DOWNLOAD_URL="https://github.com/modularflow/langextract-rust/releases/download/$LATEST_VERSION/lx-rs-$PLATFORM.tar.gz"
    
    # Download and extract
    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"
    
    if ! curl -L "$DOWNLOAD_URL" -o "lx-rs.tar.gz"; then
        print_error "Failed to download binary"
        exit 1
    fi
    
    tar -xzf "lx-rs.tar.gz"
    
    # Install to cargo bin directory
    mkdir -p "$CARGO_BIN_DIR"
    mv "lx-rs" "$CARGO_BIN_DIR/"
    chmod +x "$CARGO_BIN_DIR/lx-rs"
    
    # Cleanup
    cd - > /dev/null
    rm -rf "$TEMP_DIR"
    
    print_success "Pre-built binary installation completed!"
}

setup_environment() {
    print_step "Setting up environment..."
    
    # Add cargo bin to PATH if not already there
    if [[ ":$PATH:" != *":$CARGO_BIN_DIR:"* ]]; then
        print_step "Adding cargo bin directory to PATH..."
        
        # Determine shell and add to appropriate config file
        if [[ "$SHELL" == *"zsh"* ]]; then
            echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$HOME/.zshrc"
            print_success "Added to ~/.zshrc"
        elif [[ "$SHELL" == *"bash"* ]]; then
            echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$HOME/.bashrc"
            print_success "Added to ~/.bashrc"
        else
            print_warning "Unknown shell. Please manually add $CARGO_BIN_DIR to your PATH"
        fi
        
        # Update current session
        export PATH="$HOME/.cargo/bin:$PATH"
    fi
}

verify_installation() {
    print_step "Verifying installation..."
    
    if [ -f "$CARGO_BIN_DIR/$BINARY_NAME" ]; then
        print_success "Binary installed at $CARGO_BIN_DIR/$BINARY_NAME"
        
        # Test the binary
        if "$CARGO_BIN_DIR/$BINARY_NAME" --version >/dev/null 2>&1; then
            print_success "Installation verified successfully!"
            
            echo
            echo -e "${GREEN}🎉 LangExtract Rust CLI installed successfully!${NC}"
            echo
            echo "Usage examples:"
            echo -e "  ${CYAN}$BINARY_NAME extract 'John Doe is 30 years old' --prompt 'Extract names and ages'${NC}"
            echo -e "  ${CYAN}$BINARY_NAME providers${NC}"
            echo -e "  ${CYAN}$BINARY_NAME init${NC}"
            echo -e "  ${CYAN}$BINARY_NAME test${NC}"
            echo
            echo "For more help:"
            echo -e "  ${CYAN}$BINARY_NAME --help${NC}"
            echo
        else
            print_error "Installation verification failed"
            exit 1
        fi
    else
        print_error "Binary not found after installation"
        exit 1
    fi
}

create_desktop_entry() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        print_step "Creating desktop entry..."
        
        DESKTOP_DIR="$HOME/.local/share/applications"
        mkdir -p "$DESKTOP_DIR"
        
        cat > "$DESKTOP_DIR/langextract-rust.desktop" << EOF
[Desktop Entry]
Name=LangExtract Rust
Comment=Extract structured information from text using LLMs
Exec=$CARGO_BIN_DIR/$BINARY_NAME
Icon=text-x-generic
Terminal=true
Type=Application
Categories=Development;TextTools;
EOF
        
        print_success "Desktop entry created"
    fi
}

setup_shell_completion() {
    print_step "Setting up shell completion..."
    
    # Create completion directory
    COMPLETION_DIR="$HOME/.local/share/bash-completion/completions"
    mkdir -p "$COMPLETION_DIR"
    
    # Generate completion script
    if "$CARGO_BIN_DIR/$BINARY_NAME" --help >/dev/null 2>&1; then
        # Note: This would need clap_complete to generate actual completions
        # For now, we'll create a basic placeholder
        print_warning "Shell completion setup requires manual configuration"
        echo "To enable tab completion, add this to your shell config:"
        echo "eval \"\$($BINARY_NAME completion bash)\"  # for bash"
        echo "eval \"\$($BINARY_NAME completion zsh)\"   # for zsh"
    fi
}

show_next_steps() {
    echo
    echo -e "${PURPLE}📚 Next Steps:${NC}"
    echo
    echo "1. 🔧 Initialize configuration:"
    echo -e "   ${CYAN}$BINARY_NAME init${NC}"
    echo
    echo "2. 🧪 Test your setup:"
    echo -e "   ${CYAN}$BINARY_NAME test --provider ollama${NC}"
    echo
    echo "3. 📖 View examples:"
    echo -e "   ${CYAN}$BINARY_NAME examples${NC}"
    echo
    echo "4. 🚀 Extract from text:"
    echo -e "   ${CYAN}$BINARY_NAME extract 'Your text here' --prompt 'What to extract'${NC}"
    echo
    echo "5. 🔌 Check available providers:"
    echo -e "   ${CYAN}$BINARY_NAME providers${NC}"
    echo
    echo -e "${YELLOW}💡 Pro Tips:${NC}"
    echo "• Use --verbose for detailed output"
    echo "• Try --export html for rich visualizations"
    echo "• Use --examples file.json for custom extraction patterns"
    echo
    echo -e "${GREEN}Happy extracting! 🎯${NC}"
}

main() {
    print_banner
    
    # Parse arguments
    INSTALL_METHOD="auto"
    while [[ $# -gt 0 ]]; do
        case $1 in
            --from-crates)
                INSTALL_METHOD="crates"
                shift
                ;;
            --from-source)
                INSTALL_METHOD="source"
                shift
                ;;
            --prebuilt)
                INSTALL_METHOD="prebuilt"
                shift
                ;;
            --help|-h)
                echo "LangExtract Rust Installer"
                echo
                echo "Usage: $0 [OPTIONS]"
                echo
                echo "Options:"
                echo "  --from-crates    Install from crates.io (requires Rust)"
                echo "  --from-source    Install from source (requires Rust)"
                echo "  --prebuilt       Install pre-built binary from GitHub releases"
                echo "  --help, -h       Show this help message"
                echo
                echo "Default: Auto-detect best method (prebuilt if available, otherwise crates.io)"
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    # Auto-detect installation method
    if [[ "$INSTALL_METHOD" == "auto" ]]; then
        # Try prebuilt first (fastest), fallback to crates.io
        if command -v curl >/dev/null 2>&1; then
            INSTALL_METHOD="prebuilt"
            print_step "Auto-detected installation method: pre-built binary"
        elif command -v cargo >/dev/null 2>&1; then
            INSTALL_METHOD="crates"
            print_step "Auto-detected installation method: crates.io"
        else
            INSTALL_METHOD="source"
            print_step "Auto-detected installation method: from source (will install Rust)"
        fi
    fi
    
    check_prerequisites
    setup_environment
    
    case "$INSTALL_METHOD" in
        "crates")
            install_from_crates_io
            ;;
        "source")
            install_from_source
            ;;
        "prebuilt")
            if install_prebuilt_binary; then
                true  # Success
            else
                print_warning "Pre-built binary installation failed, falling back to crates.io"
                install_from_crates_io
            fi
            ;;
        *)
            print_error "Unknown installation method: $INSTALL_METHOD"
            exit 1
            ;;
    esac
    
    verify_installation
    create_desktop_entry
    setup_shell_completion
    show_next_steps
}

# Handle script interruption
trap 'echo -e "\n${RED}Installation interrupted${NC}"; exit 1' INT TERM

main "$@"
