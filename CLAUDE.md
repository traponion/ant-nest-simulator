# Ant Nest Simulator - Claude Code Project Guide

## Project Overview

A realistic ant colony simulation inspired by SimEarth, featuring simple dot-based graphics and complex emergent behavior. Pure observation experience watching ant colonies develop naturally without any player intervention.

## Confirmed Requirements

### Core Concept
- **Ant farm observation kit simulation** - side cross-section view
- **Ultra-simple pixel art**: brown dots for soil, black 2-pixel dots for ants
- **SimEarth-inspired realism** - complex environmental simulation
- **Pure observation game** - zero player intervention, "watch and wait" only

### Game Mechanics
- **Autonomous ant behavior**: ants automatically forage, build, reproduce
- **Realistic lifecycle**: birth, aging, death, generational turnover
- **Environmental simulation**: soil moisture, nutrition, temperature per pixel
- **NO player interactions** - completely autonomous simulation
- **Based on Camponotus japonicus (クロオオアリ) ecology**:
  - Single queen independent nest founding
  - Year 1: minimal activity, underground development
  - Year 2+: active foraging, complex social behaviors
  - Age-based division of labor (senior workers forage, junior workers tend nest)
  - Seasonal depth migration (shallow in summer, deep in winter)
  - Waste management system with designated dump sites

### Technical Stack
- **Language**: Rust
- **Engine**: Bevy (ECS architecture)
- **Repository**: https://github.com/traponion/ant-nest-simulator
- **Rendering**: Simple 2D pixel-based display
- **CI/CD**: GitHub Actions for automated testing and cross-platform builds

### Development Philosophy
- **Complexity through emergence**: Simple rules creating complex behaviors
- **Scientific accuracy**: Realistic Camponotus japonicus colony dynamics
- **Pure observation focus**: Beauty of natural processes with zero intervention
- **Performance priority**: Handle thousands of individual ants and soil pixels
- **Therapeutic experience**: Calming, meditative ant watching simulation

