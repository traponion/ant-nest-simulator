# 🐜 Ant Nest Simulator

A realistic ant colony simulation inspired by SimEarth, featuring simple dot-based graphics and complex emergent behavior. Players observe ant colonies developing naturally with minimal intervention.

## ✨ Key Features

### 🌍 Realistic Ecosystem Simulation
- **Authentic ant behavior**: Foraging, nest building, lifecycle management
- **Environmental simulation**: Soil moisture, temperature, and nutrition per pixel
- **Natural disasters**: Rain, drought, cold snaps, and invasive species
- **Colony dynamics**: Queen reproduction, egg hatching, generational turnover

### 🎮 Idle Game Mechanics
- **Autonomous behavior**: Ants act independently with minimal player intervention
- **Observation-focused gameplay**: Watch and learn from natural processes
- **Time control**: Adjust simulation speed from 1x to 100x or pause completely
- **Emergency intervention**: Trigger disasters to test colony resilience

### 🎨 Simple Yet Effective Visuals
- **Ultra-simple pixel art**: Brown dots for soil, black 2-pixel dots for ants
- **Visual effects**: Particle systems for weather and environmental changes
- **Color overlays**: Visual feedback during active disasters
- **Accessibility options**: Toggle visual effects for better accessibility

### 🛠 Technical Excellence
- **Built with Rust and Bevy**: Modern ECS architecture for performance
- **Emergent complexity**: Simple rules creating sophisticated behaviors
- **Performance optimized**: Handle thousands of individual ants and soil pixels
- **Cross-platform**: Runs on Windows, macOS, and Linux

## 🚀 Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)
- Git

### Installation & Running
```bash
# Clone the repository
git clone https://github.com/traponion/ant-nest-simulator.git
cd ant-nest-simulator

# Build and run the simulation
cargo run --release
```

The simulation window will open automatically. Watch as your ant colony begins to develop!

## 🎯 How to Play

### Basic Observation
The simulation starts with a small ant colony. Simply watch as:
- Ants begin foraging for food
- The queen lays eggs that hatch into new ants
- Soil conditions change over time
- The colony naturally grows and develops

### Time Controls
- **Spacebar**: Pause/unpause the simulation
- **1-9 keys**: Set simulation speed (1x to 9x)
- **0 key**: Set maximum speed (100x)

### Disaster Controls
Test your colony's resilience by triggering natural disasters:
- **R**: Rain (increases soil moisture, affects movement)
- **D**: Drought (decreases moisture and food availability)
- **C**: Cold Snap (slows ant movement and metabolism)
- **I**: Invasive Species (introduces competing organisms)

### Visual Effects
- **V**: Toggle all visual effects (particles and overlays)
- **P**: Toggle particle effects only
- **O**: Toggle color overlays only

## 🔧 System Requirements

### Minimum Requirements
- **OS**: Windows 10, macOS 10.14, or Ubuntu 18.04+
- **Memory**: 2 GB RAM
- **Graphics**: Any GPU with OpenGL 3.3 support
- **Storage**: 100 MB available space

### Recommended Requirements
- **OS**: Latest versions of Windows, macOS, or Linux
- **Memory**: 4 GB RAM
- **Graphics**: Dedicated GPU for better performance with large colonies
- **Storage**: 500 MB available space

## 🏗 Architecture Overview

The simulator is built using Bevy's Entity Component System (ECS) architecture:

### Core Components
- **Entities**: Ants, soil cells, food sources, eggs, particles
- **Components**: Position, behavior, lifecycle, environmental properties
- **Systems**: Movement, reproduction, environmental updates, disasters

### Key Systems
- **Lifecycle Management**: Aging, energy, birth, and death
- **Behavioral AI**: Foraging, pathfinding, state machines
- **Environmental Simulation**: Soil properties, weather effects
- **Disaster Management**: Event triggering and environmental impact
- **Rendering**: Efficient sprite-based visualization

## 🛠 Development Setup

### Building from Source
```bash
# Clone the repository
git clone https://github.com/traponion/ant-nest-simulator.git
cd ant-nest-simulator

# Install dependencies and build
cargo build

# Run in development mode
cargo run

# Run with optimizations
cargo run --release

# Run tests
cargo test
```

### Dependencies
- **Bevy**: 0.14 (Game engine and ECS framework)
- **Rand**: 0.8 (Random number generation)

### Development Tools
- **Rust**: Latest stable version recommended
- **Cargo**: Package manager and build tool (included with Rust)
- **Git**: Version control

## 🤝 Contributing

We welcome contributions! Here's how you can help:

### Getting Started
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests if applicable
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Areas for Contribution
- 🐛 Bug fixes and performance improvements
- ✨ New features and gameplay mechanics
- 📚 Documentation and tutorials
- 🎨 Visual improvements and animations
- 🧪 Testing and quality assurance
- 🌍 Localization and accessibility

### Code Guidelines
- Follow Rust conventions and `cargo fmt` formatting
- Write clear, descriptive commit messages
- Include tests for new functionality
- Update documentation as needed
- Ensure compatibility with existing features

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🎯 Project Goals

This simulation aims to demonstrate:
- **Emergent behavior** from simple rule sets
- **Scientific accuracy** in ant colony dynamics
- **Educational value** about ecosystem interactions
- **Technical excellence** in Rust and Bevy development

## 🚧 Future Development

Planned features and improvements:
- Save/load colony states
- Detailed colony statistics and metrics
- Additional disaster types
- Enhanced AI and behavioral complexity
- Performance optimizations for massive colonies
- Sound effects and ambient audio

## 🙋‍♀️ Support

- 📊 [Report Issues](https://github.com/traponion/ant-nest-simulator/issues)
- 💬 [Discussion Forum](https://github.com/traponion/ant-nest-simulator/discussions)
- 📖 [Wiki](https://github.com/traponion/ant-nest-simulator/wiki)

---

**Happy Colony Watching! 🐜🏠**

*Experience the fascinating world of ant colonies - where simple rules create incredible complexity.*