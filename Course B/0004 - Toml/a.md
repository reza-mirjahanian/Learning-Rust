# Cargo.toml Structure Reference

## [package] Section
```toml
[package]
name = "project_name"          # Required: Package name
version = "0.1.0"             # Required: Semantic version
edition = "2021"              # Rust edition (2015, 2018, 2021)
authors = ["Name <email>"]    # Package authors
description = "Description"   # Package description
documentation = "https://..."  # Documentation URL
homepage = "https://..."      # Project homepage
repository = "https://..."    # Source repository
license = "MIT"              # License identifier
license-file = "LICENSE"      # Or custom license file
keywords = ["key1", "key2"]   # Max 5 keywords
categories = ["cat1", "cat2"] # crates.io categories
readme = "README.md"         # README file path
exclude = ["*.txt", "docs/*"] # Files to exclude
include = ["src/**/*"]       # Files to include
publish = true               # Whether package can be published
default-run = "main"         # Default binary to run
rust-version = "1.56"        # Minimum Rust version
autobins = true             # Auto-discover binary targets
autoexamples = true         # Auto-discover examples
autotests = true            # Auto-discover tests
autobenches = true          # Auto-discover benchmarks
```

## Dependencies Sections

### Standard Dependencies
```toml
[dependencies]
# Version requirements
serde = "1.0"                # ^1.0.0
serde = "=1.0.0"            # Exact version
serde = ">=1.0.0"           # Version range
serde = "*"                 # Latest version

# Extended configuration
serde = { version = "1.0", features = ["derive"] }
serde = { path = "../serde" }
serde = { git = "https://github.com/serde-rs/serde" }
serde = { git = "...", branch = "master" }
serde = { git = "...", tag = "v1.0.0" }
serde = { git = "...", rev = "a123456" }
```

### Development Dependencies
```toml
[dev-dependencies]
criterion = "0.4"           # For benchmarking
mockall = "0.11"           # For testing
```

### Build Dependencies
```toml
[build-dependencies]
cc = "1.0"                 # For build scripts
```

### Target-specific Dependencies
```toml
[target.'cfg(windows)'.dependencies]
winapi = "0.3"

[target.x86_64-unknown-linux-gnu.dependencies]
openssl = "0.10"
```

## Features Configuration
```toml
[features]
default = ["feature1"]      # Default features
feature1 = []              # Simple feature flag
feature2 = ["dep:serde"]   # Feature with dependency
all = [                    # Feature group
    "feature1",
    "feature2"
]

# Optional dependencies
[dependencies]
serde = { version = "1.0", optional = true }
```

## Profile Settings
```toml
[profile.dev]
opt-level = 0              # Optimization level (0-3)
debug = true              # Include debug info
debug-assertions = true   # Enable debug assertions
overflow-checks = true    # Enable integer overflow checks
lto = false              # Link-time optimization
panic = 'unwind'         # Panic strategy
incremental = true       # Incremental compilation
codegen-units = 16       # Parallel code generation units

[profile.release]
opt-level = 3
debug = false
strip = true             # Strip symbols
lto = true
panic = 'abort'
codegen-units = 1
```

## Workspace Configuration
```toml
[workspace]
members = [
    "package1",
    "package2",
    "packages/*"
]
exclude = ["ignored_package"]
resolver = "2"           # Dependency resolver version

[workspace.package]      # Shared package metadata
version = "1.0.0"
authors = ["Author Name"]
edition = "2021"

[workspace.dependencies] # Shared dependencies
serde = "1.0"
tokio = "1.0"
```

## Binary, Library, and Example Targets
```toml
[[bin]]
name = "custom_binary"
path = "src/bin/custom.rs"
test = true
bench = true
doc = true
doctest = true
harness = true

[lib]
name = "my_lib"
path = "src/lib.rs"
crate-type = ["cdylib", "rlib"]
test = true
doctest = true
bench = true
doc = true

[[example]]
name = "example1"
path = "examples/ex1.rs"
required-features = ["feature1"]
```

## Build Script Configuration
```toml
[package]
build = "build.rs"

[build-dependencies]
cc = "1.0"

[package.metadata.build-script-deps]
cmake = "3.21"
```

## Package Metadata
```toml
[package.metadata]
custom_field = "value"

[package.metadata.docs.rs]
features = ["full"]
rustdoc-args = ["--cfg", "docsrs"]
default-target = "x86_64-unknown-linux-gnu"
```

## Publishing Configuration
```toml
[badges]
maintenance = { status = "actively-developed" }
travis-ci = { repository = "user/repo" }

[package]
publish = ["crates-io"]    # Allowed registries
```

## Package Registry Configuration
```toml
[registries]
custom-registry = { index = "https://custom.registry.org/index" }

[registry]
default = "crates-io"
token = "..."             # Registry authentication token
```