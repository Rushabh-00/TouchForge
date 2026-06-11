TouchForge

TouchForge is an open-source, cross-platform touch control overlay and input mapping system inspired by Winlator-style on-screen controls.

""build" (https://github.com/Rushabh-00/TouchForge/actions/workflows/build.yml/badge.svg)" (https://github.com/Rushabh-00/TouchForge/actions/workflows/build.yml)

""release" (https://github.com/Rushabh-00/TouchForge/actions/workflows/release.yml/badge.svg)" (https://github.com/Rushabh-00/TouchForge/actions/workflows/release.yml)

Vision

TouchForge aims to provide customizable on-screen touch controls for desktop and mobile platforms.

Profiles can be imported, edited, exported, and eventually rendered as floating overlays that map touch input to keyboard, mouse, and virtual gamepad actions.

Features

Current

- ICP profile parsing
- TouchForge Profile (".tfp") format
- Internal profile model
- Basic ICP → Internal Profile conversion
- Profile creation via CLI
- Profile loading via CLI
- GitHub Actions CI/CD
- Windows x64 builds
- Windows ARM64 builds
- Linux x64 builds

Planned

- ICP → TFP conversion
- TFP → ICP export
- Visual profile editor
- Overlay renderer
- Touch input engine
- Keyboard and mouse mapping
- Virtual gamepad support
- Start on boot
- Per-application profiles
- Android support

Supported Platforms

Current

- Windows x64
- Windows ARM64
- Linux x64

Planned

- Android
- Additional Linux architectures
- Additional Windows targets

CLI Commands

Create a new profile:

touchforge-cli create-profile demo

Open a profile:

touchforge-cli open-profile demo.tfp

Import an ICP profile:

touchforge-cli import profile.icp

Display help:

touchforge-cli --help

Project Structure

TouchForge
├── touchforge-cli
├── touchforge-core
├── touchforge-profile
├── GitHub Actions
├── Release Pipeline
└── Sample Profiles

Development Status

Current release: v0.1.5-alpha

TouchForge is in early alpha development. The current focus is profile formats, profile conversion, and building the foundation required for future overlay rendering and input mapping.

Roadmap

v0.1.x

- Profile formats
- ICP import
- TFP support
- Internal profile model

v0.2.x

- ICP ↔ TFP conversion
- Profile management

v0.3.x

- Visual profile editor

v0.4.x

- Overlay renderer

v0.5.x

- Touch input engine

v0.6.x

- Keyboard and mouse mapping

v0.7.x

- Virtual gamepad support

v1.0

- Stable cross-platform touch control system


## License

MIT License- v0.2: Overlay renderer
- v0.3: Touch input handling
- v0.4: Keyboard and mouse mapping
- v0.5: Virtual gamepad support

## Build
```bash
cargo build
