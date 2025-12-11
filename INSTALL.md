# Installation Guide for fnm (fuck-node-modules)

## Quick Start

```bash
# Install Rust (if you don't have it)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/yourusername/fuck-node-modules.git
cd fuck-node-modules
cargo build --release

# Install the binary
cp target/release/fuck-node-modules ~/.local/bin/fnm

# Add to PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

## Detailed Installation Options

### Option 1: User-Local Installation (Recommended)

```bash
# Create local bin directory if it doesn't exist
mkdir -p ~/.local/bin

# Copy the binary
cp target/release/fuck-node-modules ~/.local/bin/fnm

# Make it executable
chmod +x ~/.local/bin/fnm

# Add to PATH (choose the right file for your shell)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc  # For Bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc   # For Zsh
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bash_profile

# Reload your shell
source ~/.zshrc  # or source ~/.bashrc
```

### Option 2: System-Wide Installation

```bash
# Copy to system bin directory (requires sudo)
sudo cp target/release/fuck-node-modules /usr/local/bin/fnm

# Make it executable
sudo chmod +x /usr/local/bin/fnm

# Verify it's available to all users
which fnm
```

### Option 3: Manual Installation (No Rust)

If you don't want to install Rust, you can download pre-built binaries:

```bash
# Download the latest release (example - replace with actual URL)
wget https://github.com/yourusername/fuck-node-modules/releases/download/v0.1.0/fnm-macos

# Make it executable
chmod +x fnm-macos

# Move to bin directory
mv fnm-macos ~/.local/bin/fnm
```

## Verify Installation

```bash
# Check if fnm is available
which fnm
# Should output: /usr/local/bin/fnm or ~/.local/bin/fnm

# Check version
fnm --version

# See help
fnm --help
```

## Troubleshooting

### "Command not found" after installation

1. Check if the binary exists:
   ```bash
   ls ~/.local/bin/fnm
   ```

2. Verify PATH includes ~/.local/bin:
   ```bash
   echo $PATH
   ```

3. If not, add it manually:
   ```bash
   export PATH="$HOME/.local/bin:$PATH"
   ```

### Permission denied

```bash
chmod +x ~/.local/bin/fnm
```

### Rust/Cargo not found

Install Rust first:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## Uninstallation

```bash
# Remove the binary
rm ~/.local/bin/fnm

# Or for system-wide installation
sudo rm /usr/local/bin/fnm

# Remove from PATH (optional)
# Edit your shell config file and remove the PATH line
```

## Updating

```bash
cd fuck-node-modules
git pull
cargo build --release
cp target/release/fuck-node-modules ~/.local/bin/fnm
```

## Cross-Platform Support

The tool works on:
- ✅ macOS
- ✅ Linux
- ✅ Windows (WSL or native with Rust)

For Windows native:
```powershell
# Install Rust
winget install Rustlang.Rustup

# Build
cargo build --release

# Add to PATH
[Environment]::SetEnvironmentVariable("Path", "$env:Path;$env:USERPROFILE\.cargo\bin", "User")
```