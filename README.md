# macOS Sleep Prevention Utility

A system tray application that prevents macOS from entering sleep mode for specified durations.

## Features

- 🌙 **System Tray Integration** - Runs quietly in your menu bar
- ⏱️ **Flexible Duration Input** - Supports formats like "2h 30m", "150m", "2:30"
- 🔋 **Battery Aware** - 8-hour maximum limit to prevent battery drain
- 🎯 **Dual Sleep Prevention** - Prevents both system and display sleep
- 🔄 **Smart State Management** - Visual feedback with different tray icon states
- 🚫 **Single Instance** - Prevents multiple app launches

## Quick Start

### Using Make (Recommended)

```bash
# Initial setup
make setup

# Start development
make dev

# Build and install
make build-release
make install
```

### Manual Commands

```bash
# Install dependencies
bun install

# Development
bun tauri dev

# Build
bun tauri build
```

## Development

### Available Make Targets

Run `make help` to see all available commands:

```bash
make setup         # Initial project setup
make dev           # Start development server  
make build         # Build for development
make build-release # Build optimized release
make test          # Run tests
make lint          # Run linters
make clean         # Clean build artifacts
make install       # Install to /Applications/
make manual-test   # Show testing guide
```

### Tech Stack

- **Backend**: Rust + Tauri v2 + IOKit (macOS system integration)
- **Frontend**: Vue.js 3 + TypeScript + Pinia
- **Build**: Vite + bun package manager
- **Platform**: macOS 10.15+ (Catalina or newer)

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Vue.js UI     │◄──►│  Tauri Commands  │◄──►│  macOS IOKit    │
│   (TypeScript)  │    │     (Rust)       │    │  Sleep APIs     │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### Project Structure

```
├── src/                  # Vue.js frontend
│   ├── components/       # Vue components
│   ├── stores/          # Pinia state management
│   └── services/        # API services
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── sleep_manager.rs  # IOKit integration
│   │   ├── session.rs        # Session management
│   │   └── commands/         # Tauri commands
│   └── icons/           # Tray icons
├── specs/               # Feature specifications
└── Makefile            # Development commands
```

## Manual Testing

The app uses manual testing based on real user scenarios. See:
- `specs/001-mac-osx-to/quickstart.md` - Comprehensive testing guide
- `make manual-test` - Quick testing checklist

### Key Test Scenarios

1. **Basic Usage**: Start 5-minute prevention, verify sleep is blocked
2. **Input Validation**: Test formats like "2h 30m", "150m", "2:30"
3. **Visual Feedback**: Observe tray icon state changes
4. **System Integration**: Test lid close, manual sleep attempts
5. **Edge Cases**: Multiple launches, user logout, battery warnings

## Building & Distribution

### Development Build
```bash
make build
```

### Release Build
```bash
make build-release
```

### Install Locally
```bash
make install  # Copies to /Applications/
```

## Configuration

The app stores minimal configuration in:
```
~/Library/Preferences/com.macos-sleep-prevention.toml
```

## Troubleshooting

### Common Issues

1. **Permission Errors**: May require accessibility permissions in System Preferences
2. **Build Failures**: Ensure Rust 1.75+ and latest Xcode Command Line Tools
3. **Tray Icon Missing**: Check System Preferences > Dock & Menu Bar

### Debug Commands

```bash
make logs          # Show app logs
make debug-deps    # Check dependencies
make debug-config  # Show configuration
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes following the constitutional principles
4. Test manually using `make manual-test`
5. Submit a pull request

### Development Principles

This project follows Kent Beck's "Tidy First?" philosophy:
- Make small, reversible changes
- Eliminate complexity rather than managing it
- Write code for the next developer
- Test for confidence, not coverage

## License

MIT License - see LICENSE file for details

## Support

- 📋 Issues: Use GitHub Issues for bug reports
- 📖 Documentation: See `specs/` directory for detailed specifications
- 🔧 Development: Use `make help` for available commands
