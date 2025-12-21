# Reinforcing Mars

A monorepo containing the Terraforming Mars simulation engine and reinforcement
learning training code. The simulation engine is implemented in Rust with Python
bindings, enabling high-performance game simulation for RL agent training with
PyTorch.

## Structure

- `simulation/` - Rust simulation engine with Python bindings (PyO3)
- `training/` - PyTorch training code for RL agents
- `shared/` - Shared game data, card definitions, and configurations

## Overview

This project provides a high-performance simulation engine for the Terraforming Mars
board game, designed for training reinforcement learning agents. The Rust core ensures
fast game state transitions and batch simulation, while Python bindings enable
seamless integration with PyTorch for model training.

## Status

Project in early development.
