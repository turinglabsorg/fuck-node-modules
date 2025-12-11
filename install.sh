#!/bin/bash

# fnm (fuck-node-modules) installation script
# Automates building and installing the Rust CLI tool

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'  # No Color

# Version
VERSION="0.1.0"

# Function to print colored messages
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Rust is installed
check_rust() {
    if ! command -v cargo &> /dev/null; then
        print_error "Rust is not installed. Please install Rust first:"
        print_info "Install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    print_info "Rust is installed: $(rustc --version)"
}

# Build the project
build_project() {
    print_info "Building fnm v$VERSION..."
    
    if [ -f "Cargo.toml" ]; then
        cargo build --release
        print_success "Build completed successfully!"
    else
        print_error "Cargo.toml not found. Are you in the project directory?"
        exit 1
    fi
}

# Install the binary
install_binary() {
    local install_dir="$HOME/.local/bin"
    local binary_name="fnm"
    local source_path="target/release/fuck-node-modules"
    local target_path="$install_dir/$binary_name"
    
    # Create local bin directory if it doesn't exist
    if [ ! -d "$install_dir" ]; then
        print_info "Creating local bin directory: $install_dir"
        mkdir -p "$install_dir"
    fi
    
    # Copy the binary
    print_info "Installing $binary_name to $target_path"
    cp "$source_path" "$target_path"
    chmod +x "$target_path"
    
    # Create symlink for original name
    ln -sf "$target_path" "${install_dir}/fuck-node-modules"
    
    print_success "Binary installed successfully!"
}

# Add to PATH if needed
setup_path() {
    local shell_config="$HOME/.zshrc"
    
    # Try to detect shell
    if [ -f "$HOME/.bashrc" ] && [ -z "$ZSH_VERSION" ]; then
        shell_config="$HOME/.bashrc"
    elif [ -f "$HOME/.bash_profile" ]; then
        shell_config="$HOME/.bash_profile"
    fi
    
    # Check if PATH is already set
    if ! grep -q 'export PATH="$HOME/.local/bin:$PATH"' "$shell_config"; then
        print_info "Adding ~/.local/bin to PATH in $shell_config"
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$shell_config"
        print_warning "Please restart your terminal or run: source $shell_config"
    else
        print_info "PATH is already configured in $shell_config"
    fi
}

# Verify installation
verify_installation() {
    if command -v fnm &> /dev/null; then
        print_success "✓ fnm is now available globally"
        print_info "Version: $(fnm --version)"
        print_info "Location: $(which fnm)"
        
        echo
        print_success "Installation complete! 🎉"
        print_info "Usage: fnm [OPTIONS] [PATH]"
        print_info "Try: fnm --help"
    else
        print_warning "fnm is installed but not in PATH"
        print_info "Add this to your shell config: export PATH="$HOME/.local/bin:\$PATH""
    fi
}

# Main installation process
main() {
    echo
    print_info "🚀 fnm (fuck-node-modules) v$VERSION Installation"
    echo
    
    check_rust
    build_project
    install_binary
    setup_path
    verify_installation
    
    echo
}

# Run main function
main