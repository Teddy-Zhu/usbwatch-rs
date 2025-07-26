# 📦 Installation Guide

This guide provides detailed installation instructions for USBWatch across different platforms and deployment scenarios.

## 🚀 Quick Installation

### Option 1: Using Built-in Installer (Recommended)

```bash
# Build the project
cargo build --release

# Install to system PATH
sudo ./target/release/usbwatch install

# Verify installation
usbwatch --version
```

### Option 2: Manual Installation

```bash
# Build the project
cargo build --release

# Copy to system directory
sudo cp ./target/release/usbwatch /usr/local/bin/

# Set permissions
sudo chmod +x /usr/local/bin/usbwatch
```

## 🖥️ Platform-Specific Instructions

### Linux (Ubuntu, Debian, CentOS, etc.)

#### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.bashrc

# Install build dependencies
sudo apt update
sudo apt install build-essential
```

#### Installation Steps

```bash
# Clone the repository
git clone https://github.com/NotKeira/usbwatch-rs.git
cd usbwatch-rs

# Build the project
cargo build --release

# Install using built-in installer
sudo ./target/release/usbwatch install

# Test the installation
usbwatch --help
```

#### Installation Paths

- **System-wide**: `/usr/local/bin/usbwatch`
- **User-specific**: `~/.local/bin/usbwatch` (manual installation)

### Windows

#### Prerequisites

- Install Rust from [rustup.rs](https://rustup.rs/)
- Install Visual Studio Build Tools or MinGW-w64

#### Installation Steps (PowerShell as Administrator)

```powershell
# Clone the repository
git clone https://github.com/NotKeira/usbwatch-rs.git
cd usbwatch-rs

# Build the project
cargo build --release

# Install using built-in installer
.\target\release\usbwatch.exe install

# Test the installation
usbwatch --help
```

#### Installation Paths

- **System-wide**: `C:\Program Files\usbwatch\usbwatch.exe`
- **User-specific**: `%USERPROFILE%\.local\bin\usbwatch.exe` (manual)

**Note**: You may need to add the installation directory to your PATH environment variable manually.

### macOS

Currently, macOS support is not implemented, but the project structure allows for future extension.

## 🔧 Cross-Compilation

### Building Windows Binaries on Linux

```bash
# Install the Windows target
rustup target add x86_64-pc-windows-gnu

# Install MinGW-w64 cross-compiler
sudo apt install gcc-mingw-w64-x86-64

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu

# The Windows binary will be at:
# target/x86_64-pc-windows-gnu/release/usbwatch.exe
```

### Building Linux Binaries on Windows (WSL)

```bash
# Install WSL2 and Ubuntu
wsl --install -d Ubuntu

# Inside WSL, follow the Linux installation instructions
wsl
git clone https://github.com/NotKeira/usbwatch-rs.git
cd usbwatch-rs
cargo build --release
```

## 🐳 Container Deployment

### Docker

Create a `Dockerfile`:

```dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/usbwatch /usr/local/bin/usbwatch

# Note: USB monitoring in containers requires privileged access
# and volume mounting of /sys and /dev
CMD ["usbwatch", "--json"]
```

Build and run:

```bash
# Build the image
docker build -t usbwatch .

# Run with USB access (requires privileged mode)
docker run --privileged -v /sys:/sys:ro -v /dev:/dev usbwatch
```

## 📋 System Requirements

### Minimum Requirements

- **RAM**: 10MB
- **Storage**: 5MB for binary
- **CPU**: Any modern x86_64 processor

### Operating System Support

- **Linux**: Kernel 2.6+ with sysfs support
- **Windows**: Windows 7/Server 2008 R2 or later
- **Architecture**: x86_64 (64-bit)

### Runtime Dependencies

- **Linux**: None (uses system calls and sysfs)
- **Windows**: None (uses Win32 APIs)

## 🔐 Permissions and Security

### Linux Permissions

```bash
# USBWatch requires read access to:
ls -la /sys/bus/usb/devices/

# Standard users typically have this access by default
# No special permissions needed for monitoring
```

### Windows Permissions

- Standard user permissions for basic monitoring
- Administrator privileges may be required for detailed device information
- Installation requires administrator privileges

### Security Considerations

- USBWatch only reads device information, never modifies anything
- No network connections are made
- All data remains local unless explicitly logged to a file
- File logging respects system file permissions

## 🗑️ Uninstallation

### Using Built-in Uninstaller

```bash
# Linux/macOS
sudo usbwatch uninstall

# Windows (as Administrator)
usbwatch uninstall
```

### Manual Uninstallation

#### Linux

```bash
# Remove the binary
sudo rm /usr/local/bin/usbwatch

# Remove any configuration files (if created)
rm -rf ~/.config/usbwatch
```

#### Windows

```powershell
# Remove the binary
Remove-Item "C:\Program Files\usbwatch\usbwatch.exe"

# Remove the directory if empty
Remove-Item "C:\Program Files\usbwatch" -Force -Recurse

# Remove from PATH if manually added
# (Edit System Environment Variables)
```

## 🔄 Updating

### Using Git

```bash
# Pull latest changes
git pull origin main

# Rebuild and reinstall
cargo build --release
sudo ./target/release/usbwatch uninstall
sudo ./target/release/usbwatch install
```

### Manual Update

1. Download the new binary
2. Uninstall the old version: `sudo usbwatch uninstall`
3. Install the new version: `sudo ./new-usbwatch install`

## 🧪 Development Installation

For developers working on USBWatch:

```bash
# Clone with development dependencies
git clone https://github.com/NotKeira/usbwatch-rs.git
cd usbwatch-rs

# Install development tools
cargo install cargo-watch cargo-audit

# Run in development mode
cargo run -- --help

# Run with hot reload
cargo watch -x run

# Run tests
cargo test

# Check code quality
cargo clippy
cargo audit
```

## 🆘 Troubleshooting

### Common Installation Issues

**"Permission denied" errors**

```bash
# Ensure you're using sudo for system installation
sudo ./target/release/usbwatch install

# Or install to user directory
mkdir -p ~/.local/bin
cp ./target/release/usbwatch ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"
```

**"Command not found" after installation**

```bash
# Check if the binary exists
ls -la /usr/local/bin/usbwatch

# Check if /usr/local/bin is in PATH
echo $PATH | grep -o /usr/local/bin

# Add to PATH if missing (add to ~/.bashrc)
export PATH="/usr/local/bin:$PATH"
```

**Build failures on Windows**

```powershell
# Install Visual Studio Build Tools
# Or install MinGW-w64 via MSYS2
```

**Cross-compilation issues**

```bash
# Ensure target is installed
rustup target list --installed

# Install missing target
rustup target add x86_64-pc-windows-gnu

# Install cross-compiler
sudo apt install gcc-mingw-w64-x86-64
```

### Getting Help

1. Check the [README.md](README.md) for usage information
2. Search existing [GitHub Issues](https://github.com/NotKeira/usbwatch-rs/issues)
3. Create a new issue with:
   - Your operating system and version
   - Rust version (`rustc --version`)
   - Complete error message
   - Steps to reproduce

### Logging and Debugging

```bash
# Enable verbose output (if implemented)
RUST_LOG=debug usbwatch

# Test with specific output
usbwatch --json | jq '.'

# Check system USB devices
# Linux:
ls /sys/bus/usb/devices/
# Windows:
Get-PnpDevice -Class USB
```

---

For additional support, please visit the [GitHub repository](https://github.com/NotKeira/usbwatch-rs) or check the project documentation.
