# fuck-node-modules (fnm) 🗑️

A Rust CLI tool to recursively find and remove `node_modules` folders, with smart age-based filtering to protect your active projects.

## Features

✅ **Recursive Search** - Finds all `node_modules` folders in nested directories
✅ **Age-Based Filtering** - Only delete folders older than N days (default: 30 days)
✅ **Safe Deletion** - Interactive confirmation with dry-run mode
✅ **Detailed Reporting** - Shows size, count, and what was skipped
✅ **Robust Error Handling** - Skips permission errors and broken symlinks
✅ **Fast & Efficient** - Built with Rust for performance

## Installation

### Quick Install (macOS/Linux)

```bash
# 1. Clone the repository
git clone https://github.com/yourusername/fuck-node-modules.git
cd fuck-node-modules

# 2. Build the release binary
cargo build --release

# 3. Install globally (choose one method)

# Method A: User-local installation (recommended)
mkdir -p ~/.local/bin
cp target/release/fuck-node-modules ~/.local/bin/fnm

# Method B: System-wide installation (requires sudo)
sudo cp target/release/fuck-node-modules /usr/local/bin/fnm

# 4. Add to PATH if using user-local installation
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc  # or ~/.bashrc
source ~/.zshrc
```

### Verify Installation

```bash
which fnm
# Should show: /usr/local/bin/fnm or ~/.local/bin/fnm

fnm --help
# Should show the help menu
```

## Usage

### Basic Usage

```bash
# Search current directory (dry run - shows what would be deleted)
fnm

# Actually delete with confirmation
fnm --force

# Skip confirmation prompts
fnm --yes

# Search specific path
fnm /path/to/your/projects
```

### Age-Based Filtering

```bash
# Default: Delete node_modules older than 30 days
fnm .

# Custom threshold: Delete folders older than 7 days
fnm . --older-than 7
fnm . -o 7

# Delete folders older than 90 days
fnm . -o 90

# Disable filtering (delete all node_modules)
fnm . -o 0
```

### Combined Options

```bash
# Delete all node_modules older than 14 days, auto-confirm
fnm . -o 14 --yes

# Search specific path, 60-day threshold, force delete
fnm ~/code -o 60 --force
```

## Examples

### Example 1: Clean up old projects

```bash
cd ~/old-projects
fnm -o 30

# Output:
# 🔍 Searching for node_modules folders...
#   Looking for folders older than 30 days
# 
# 📊 Found old node_modules folders:
#   Count: 8
#   Total size: 2.4 GB
#   Skipped 2 recent folders
# 
#   - ~/old-projects/web-app/node_modules
#   - ~/old-projects/api-server/node_modules
#   ...
```

### Example 2: Aggressive cleanup (7 days)

```bash
fnm ~/all-projects -o 7 --yes

# This will automatically delete all node_modules folders
# that haven't been modified in the last 7 days
```

### Example 3: Safe dry-run

```bash
fnm ~/important-code

# Shows what would be deleted without actually deleting
# Gives you a chance to review before using --force or --yes
```

## Command Line Arguments

```
USAGE:
    fnm [OPTIONS] [path]

ARGS:
    <path>    The path to search for node_modules folders [default: .]

OPTIONS:
    -f, --force          Actually delete the folders (dry run by default)
    -o, --older-than     Only delete node_modules folders older than N days [default: 30]
    -y, --yes            Skip confirmation prompts
    -h, --help           Print help information
    -V, --version        Print version information
```

## How It Works

1. **Recursive Search**: Uses `walkdir` to traverse directories efficiently
2. **Age Filtering**: Checks each `node_modules` folder's last modified time
3. **Size Calculation**: Computes total disk space that can be reclaimed
4. **Safe Deletion**: Provides clear confirmation before any deletion
5. **Error Handling**: Gracefully skips permission errors and broken symlinks

## Building from Source

### Prerequisites

- Rust (install via [rustup](https://rustup.rs/))
- Cargo (comes with Rust)

### Build Steps

```bash
# Clone the repository
git clone https://github.com/yourusername/fuck-node-modules.git
cd fuck-node-modules

# Build in release mode (optimized)
cargo build --release

# Run tests (if available)
cargo test

# Install the binary
cp target/release/fuck-node-modules ~/.local/bin/fnm
```

## Configuration

The tool uses sensible defaults:

- **Default path**: Current directory (`.`)
- **Default age threshold**: 30 days
- **Default mode**: Dry-run (shows what would be deleted)

You can override any of these with command-line arguments.

## Safety Features

🔒 **Dry-run by default** - Always shows what will be deleted before doing anything
🔒 **Age filtering** - Protects recently used projects by default
🔒 **Interactive confirmation** - Requires explicit confirmation unless `--yes` is used
🔒 **Error handling** - Won't crash on permission errors or broken symlinks

## Performance

- **Fast scanning**: Rust's performance makes directory traversal quick
- **Efficient memory usage**: Processes folders incrementally
- **Parallel-ready**: Could be enhanced with Rayon for multi-threaded scanning

## Contributing

Contributions are welcome! Please open issues or pull requests for:

- Bug fixes
- Feature requests
- Performance improvements
- Documentation enhancements

## License

MIT License - See [LICENSE](LICENSE) file for details.

## Support

If you find this tool useful, consider:

- ⭐ Star the repository
- 🐦 Share it on social media
- 💬 Provide feedback and suggestions

## Alternatives

- `rm -rf **/node_modules` - Dangerous, no safety checks
- `find . -name "node_modules" -type d` - No age filtering or size reporting
- `npm-prune` - Only works for specific projects, not recursive

`fnm` provides a safe, feature-rich alternative with smart filtering!

---

**Happy cleaning! 🧹** Let `fnm` help you reclaim valuable disk space from those pesky `node_modules` folders!