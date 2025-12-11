# Changelog

All notable changes to `fnm` (fuck-node-modules) will be documented in this file.

## [Unreleased]

## [0.1.0] - 2023-12-11

### Added

- Initial release of `fnm` - a Rust CLI tool for cleaning up `node_modules` folders
- Recursive directory traversal to find all `node_modules` folders
- Safe deletion with interactive confirmation
- Dry-run mode by default
- Detailed statistics showing count and total size
- Colorful terminal output
- Robust error handling for permission issues and broken symlinks
- Age-based filtering with `--older-than` parameter (default: 30 days)
- Multiple installation methods (user-local and system-wide)
- Comprehensive documentation

### Features

- **Basic functionality**: `fnm` to search current directory
- **Force deletion**: `fnm --force` to actually delete folders
- **Auto-confirm**: `fnm --yes` to skip confirmation prompts
- **Age filtering**: `fnm -o 30` to only delete folders older than 30 days
- **Custom paths**: `fnm /path/to/project` to search specific directories
- **Help system**: `fnm --help` for usage information

### Technical Details

- Built with Rust for performance and safety
- Uses `walkdir` for efficient directory traversal
- Uses `clap` for command-line argument parsing
- Uses `console` and `dialoguer` for user-friendly terminal interface
- Uses `humansize` for readable file size formatting
- Cross-platform support (macOS, Linux, Windows)

### Breaking Changes

None - this is the initial release.

### Bug Fixes

- Fixed directory traversal to handle broken symbolic links
- Fixed error handling for inaccessible directories
- Fixed type mismatches in file system operations

### Performance Improvements

- Optimized directory scanning
- Efficient memory usage
- Fast size calculation

### Documentation

- Comprehensive README.md with usage examples
- Detailed INSTALL.md with multiple installation methods
- Clear command-line help

### Known Issues

- None reported yet

### Deprecations

- None

### Security

- Safe by default (dry-run mode)
- Interactive confirmation before deletion
- Age filtering to protect active projects

## Future Plans

### Upcoming Features

- Parallel directory scanning for better performance
- Exclusion patterns to ignore specific projects
- Size-based filtering (delete folders larger than X)
- Interactive selection of which folders to delete
- JSON output format for scripting
- Configuration file support
- Git integration to check project activity

### Potential Enhancements

- GUI version for visual folder selection
- Browser extension to clean up web projects
- Integration with package managers
- Automatic scheduling

---

**Note**: This changelog follows [Keep a Changelog](https://keepachangelog.com/) format.