# 🔧 Service Manager

A powerful command-line tool for managing macOS system services, supporting both **launchd** and **Homebrew** services with an intuitive interactive interface.

## ✨ Features

- 📋 **List Services**: View all system services with their current status
- 🚀 **Start Services**: Interactive service selection and startup
- 🛑 **Stop Services**: Interactive service selection and shutdown
- 📊 **Service Status**: Check detailed status of specific services
- 🍺 **Homebrew Support**: Manage both launchd and brew services
- 🎨 **Colorful Output**: Beautiful, color-coded terminal interface
- ⚡ **Fast & Efficient**: Built with Rust for optimal performance

## 🚀 Installation

### Prerequisites

- macOS (required for launchd support)
- Rust 1.70+ (for building from source)
- Homebrew (optional, for brew service management)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/sw3do/service-manager.git
cd service-manager

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

### Using Cargo

```bash
cargo install --git https://github.com/sw3do/service-manager.git
```

## 📖 Usage

### Basic Commands

```bash
# Show help
service-manager --help

# List all services
service-manager list

# List only running services
service-manager list --running

# Include brew services in listing
service-manager list --brew

# Start a service (interactive)
service-manager start

# Start with brew services included
service-manager start --brew

# Stop a service (interactive)
service-manager stop

# Stop with brew services included
service-manager stop --brew

# Check specific service status
service-manager status <service-name>

# Check brew service status
service-manager status <service-name> --brew
```

### Examples

#### List All Services
```bash
$ service-manager list
🔧 System Services:
────────────────────────────────────────────────────────────────────────────────
🟢 [LAUNCHD] com.apple.WindowServer - running (PID: 123)
🔴 [BREW] nginx - stopped
🟢 [BREW] mysql - started
────────────────────────────────────────────────────────────────────────────────
📊 Total 3 services listed
```

#### Interactive Service Management
```bash
$ service-manager start --brew
🚀 Select the service you want to start:
❯ nginx [BREW]
  postgresql [BREW]
  com.example.service [LAUNCHD]
```

#### Service Status Check
```bash
$ service-manager status nginx --brew
📋 Brew Service: nginx - Status: started
```

## 🛠️ Technical Details

### Architecture

- **Language**: Rust 🦀
- **CLI Framework**: clap 4.0
- **Async Runtime**: tokio
- **Interactive UI**: dialoguer
- **Colorization**: colored
- **Serialization**: serde

### Service Types

1. **Launchd Services**: Native macOS system services managed by `launchctl`
2. **Homebrew Services**: Services installed and managed via `brew services`

### Commands Used

- `launchctl list` - List launchd services
- `launchctl load/unload` - Start/stop launchd services
- `brew services list` - List brew services
- `brew services start/stop` - Start/stop brew services

## 🎨 Output Format

The tool provides color-coded output for easy identification:

- 🟢 **Green**: Running/Started services
- 🔴 **Red**: Stopped services
- 🟡 **Yellow**: Unknown status
- 🔵 **Blue**: Service names and headers
- 🟣 **Magenta**: [BREW] badges
- 🔷 **Cyan**: [LAUNCHD] badges and PID information

## 🔧 Configuration

No configuration file is required. The tool automatically:

- Detects Homebrew availability
- Adapts interface based on available service types
- Provides appropriate warnings when brew is unavailable

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/sw3do/service-manager.git
cd service-manager

# Install dependencies
cargo build

# Run tests
cargo test

# Run with development profile
cargo run -- list
```

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Ensure all tests pass (`cargo test`)
- Run clippy for linting (`cargo clippy`)

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with ❤️ using Rust
- Inspired by the need for better macOS service management
- Thanks to the Rust community for excellent crates

## 📞 Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/sw3do/service-manager/issues) page
2. Create a new issue with detailed information
3. Include your macOS version and error messages

---

**Made with 🦀 Rust by [sw3do](https://github.com/sw3do)**