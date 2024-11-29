# Cargo Commands & Features

## Essential Commands
- `cargo new <project_name>` - Create new project
- `cargo init` - Initialize project in existing directory
- `cargo build` - Compile project
- `cargo run` - Compile and run project
- `cargo check` - Check if code compiles without producing binary
- `cargo test` - Run tests
- `cargo bench` - Run benchmarks
- `cargo clean` - Remove target directory
- `cargo update` - Update dependencies
- `cargo doc` - Generate documentation
- `cargo publish` - Publish to crates.io

## Build Flags
- `--release` - Optimize for production
- `--verbose` or `-v` - Show detailed output
- `--quiet` or `-q` - Suppress output
- `--jobs <N>` or `-j <N>` - Number of parallel jobs
- `--target <triple>` - Build for specific target
- `--features <features>` - Enable specific features

## Dependency Management
```toml
[dependencies]
# Latest version
serde = "1.0"

# Exact version
serde = "=1.0.152"

# Version range
serde = ">=1.0.152, <2.0.0"

# Git repository
serde = { git = "https://github.com/serde-rs/serde" }

# Local path
serde = { path = "../serde" }

# Feature flags
serde = { version = "1.0", features = ["derive"] }
```

## Workspace Management
```toml
[workspace]
members = [
    "project1",
    "project2",
    "project3"
]

# Exclude specific packages
exclude = ["excluded_project"]
```

## Profile Customization
```toml
[profile.dev]
opt-level = 0
debug = true

[profile.release]
opt-level = 3
debug = false
lto = true
```

## Environment Variables
- `CARGO_HOME` - Cargo's home directory
- `RUSTUP_HOME` - Rustup's home directory
- `CARGO_TARGET_DIR` - Custom target directory
- `RUSTFLAGS` - Custom compiler flags
- `CARGO_INCREMENTAL` - Enable/disable incremental compilation

## Advanced Features

### Cross Compilation
```bash
# Install target
rustup target add x86_64-unknown-linux-musl

# Build for target
cargo build --target x86_64-unknown-linux-musl
```

### Custom Scripts
```toml
[package]
# ...

[[bin]]
name = "custom_script"
path = "src/scripts/custom.rs"

[package.metadata.scripts]
test-all = "cargo test --all-features"
```

### Workspace Inheritance
```toml
[workspace.package]
version = "1.0.0"
authors = ["Author Name"]
edition = "2021"

[workspace.dependencies]
serde = "1.0"
```

### Feature Flags
```toml
[features]
default = ["feature1"]
feature1 = []
feature2 = ["dep:serde"]
all = ["feature1", "feature2"]

[dependencies]
serde = { version = "1.0", optional = true }
```

## Cargo.toml Sections
```toml
[package]
name = "project_name"
version = "0.1.0"
authors = ["Author Name"]
edition = "2021"
description = "Project description"
license = "MIT"
repository = "https://github.com/user/repo"
documentation = "https://docs.rs/crate"
readme = "README.md"
keywords = ["keyword1", "keyword2"]
categories = ["category1", "category2"]
```

## Common Patterns

### Dev Dependencies
```toml
[dev-dependencies]
criterion = "0.4"
mockall = "0.11"
```

### Build Dependencies
```toml
[build-dependencies]
cc = "1.0"
```

### Target-specific Dependencies
```toml
[target.'cfg(windows)'.dependencies]
winapi = "0.3"
```

## Testing Features
- `cargo test -- --nocapture` - Show println output
- `cargo test test_name` - Run specific test
- `cargo test -- --test-threads=1` - Single-threaded tests
- `cargo test -- --ignored` - Run ignored tests
- `cargo test --doc` - Test documentation examples

## Documentation
- `cargo doc --open` - Generate and open docs
- `cargo doc --document-private-items` - Include private items
- `cargo doc --no-deps` - Exclude dependencies

## Publishing
- `cargo login` - Login to crates.io
- `cargo package` - Create distributable package
- `cargo publish --dry-run` - Test publication
- `cargo yank --version 1.0.0` - Remove version
- `cargo owner --add username` - Add owner