# Ant Nest Simulator - Claude Code Project Guide

## Project Overview

A realistic ant colony simulation inspired by SimEarth, featuring simple dot-based graphics and complex emergent behavior. Players observe ant colonies developing naturally with minimal intervention.

## Confirmed Requirements

### Core Concept
- **Ant farm observation kit simulation** - side cross-section view
- **Ultra-simple pixel art**: brown dots for soil, black 2-pixel dots for ants
- **SimEarth-inspired realism** - complex environmental simulation
- **Idle game mechanics** - minimal player intervention, "watch and wait"

### Game Mechanics
- **Autonomous ant behavior**: ants automatically forage, build, reproduce
- **Realistic lifecycle**: birth, aging, death, generational turnover
- **Environmental simulation**: soil moisture, nutrition, temperature per pixel
- **Player interactions limited to**:
  - Time acceleration (1x to 100x speed)
  - Natural disaster summoning (rain, drought, cold snaps, invasive species)

### Technical Stack
- **Language**: Rust
- **Engine**: Bevy (ECS architecture)
- **Repository**: https://github.com/traponion/ant-nest-simulator
- **Rendering**: Simple 2D pixel-based display
- **CI/CD**: GitHub Actions for automated testing and cross-platform builds

### Development Philosophy
- **Complexity through emergence**: Simple rules creating complex behaviors
- **Scientific accuracy**: Realistic ant colony dynamics
- **Observation focus**: Beauty of natural processes without excessive intervention
- **Performance priority**: Handle thousands of individual ants and soil pixels

