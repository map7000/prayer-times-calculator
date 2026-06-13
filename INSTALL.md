# Installation Guide

## Prerequisites
- Rust: Version 1.70 or higher (Install Rust
)
- Cargo: Comes bundled with Rust

## Option 1: Run as API Server

### 1. Clone the Repository
```
git clone https://github.com/map7000/prayer-times-calculator.git
cd prayer-times-calculator
```
### 2. Build the Project
```
cargo build --release
```
### 3. Run the Server
```
# Default port (3000)
cargo run --release

# Custom port
PORT=8080 cargo run --release
```
## Option 2: Use as a Rust Library
Add to your Cargo.toml:
```
[dependencies]
prayer-times-calculator = { git = "https://github.com/map7000/prayer-times-calculator" }
```
Or
```
[dependencies]
prayer-times-calculator = "0.1.0"
```
