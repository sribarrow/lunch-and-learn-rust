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