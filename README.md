# TouchForge

TouchForge is an open-source, cross-platform touch control overlay and input mapping system inspired by Winlator-style on-screen controls.

[![build](https://github.com/Rushabh-00/TouchForge/actions/workflows/build.yml/badge.svg)](https://github.com/Rushabh-00/TouchForge/actions/workflows/build.yml)

[![release](https://github.com/Rushabh-00/TouchForge/actions/workflows/release.yml/badge.svg)](https://github.com/Rushabh-00/TouchForge/actions/workflows/release.yml)

## Features

- Import Winlator ICP profiles
- TouchForge Profile (`.tfp`) format
- Internal profile conversion system
- Profile creation and loading from CLI
- Windows x64 support
- Windows ARM64 support
- Linux x64 support
- Automated GitHub builds and releases

## Goals

- Import and export touch control profiles
- Create and edit custom control layouts
- Render floating on-screen controls
- Map touches to keyboard, mouse, and virtual gamepad inputs
- Support Windows, Windows ARM, Linux, and Android
- Support startup on boot
- Provide a profile editor UI
- Support Winlator profile compatibility

## Current Status

Early Alpha Development

Current working features:

- ICP profile parsing
- TFP profile format
- Internal profile model
- Basic ICP → Internal Profile conversion
- CLI profile creation
- CLI profile loading
- Automated multi-platform releases

## CLI Commands

Create a profile:

```bash
touchforge-cli create-profile demo
```

Open a profile:

```bash
touchforge-cli open-profile demo.tfp
```

Import an ICP profile:

```bash
touchforge-cli import profile.icp
```

Show help:

```bash
touchforge-cli --help
```

## Supported Platforms

| Platform | Status |
|----------|---------|
| Windows x64 | ✅ |
| Windows ARM64 | ✅ |
| Linux x64 | ✅ |
| Android | 🚧 Planned |
| Linux ARM64 | 🚧 Planned |

## Roadmap

### v0.1.x
- ICP Import
- TFP Profile Format
- Internal Profile Model
- CLI Tools

### v0.2.x
- ICP ↔ TFP Conversion
- Import / Export Commands
- Profile Management

### v0.3.x
- Overlay Renderer
- Button Rendering
- Joystick Rendering

### v0.4.x
- Touch Input Handling
- Keyboard Mapping
- Mouse Mapping

### v0.5.x
- Virtual Gamepad Support
- XInput Backend
- Advanced Control Types

### v0.6.x
- Graphical Profile Editor
- Drag-and-Drop Controls
- Import / Export UI

### v1.0
- Stable Cross-Platform Release
- Windows
- Windows ARM
- Linux
- Android

## Build

```bash
cargo build --workspace
```

## Run

```bash
cargo run --bin touchforge-cli -- --help
```

## License

MIT License
