# Rustup - the Official Toolchain installer. 

It serves as a unified way to install, manage and maintain different versions of the Rust compiler and its associated tools.

### Rust Analyzer
for code analysis
- Code completion
- Error detection
- Code navigation
- doc lookup


### Components of a toolchain
- rustc -> compiler
- cargo -> package manager and build tool
- rustdoc -> Documentation generator
- Optional components
    - rustfmt -> Code Formatter
    - clippy -> linter
    - miri -> undefined behaviour checks
    - Editor support tools

### profiles
Group of components that can be installed as part of the toolchain
- minimal -> installs basic components such as rust-c, rust-std and cargo (recommended for windows)
- default -> All components of minimal plus rust-docs, clippy and rustfmt
- complete -> all components available through rustup
```
rustup set profile minimal
OR
rustup install --profile <name>
```

### Update Rust to latest version
```
rustup update
```

### Install specific version
```
rustup install 1.74.0
```

### Set global default version

```
rustup default 1.74.0
```

### Set a version just for the current project
This creates a rust-toolchain.toml file
```
rustup override set 1.74.0
```

### Switch to nightly or beta
```
rustup install nightly
rustup override set nightly
```

### See installed versions
```
rustup show
```

### Uninstall a version
```
rustup uninstall 1.74.0
```
### Add rust-toolchain.toml to project root to pin the version for a given project (collaboration)
```
[toolchain]
channel = "1.74.0"
```