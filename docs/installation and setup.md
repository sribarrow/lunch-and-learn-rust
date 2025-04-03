### Install rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Verify installation
```
rustc --version # compiler
cargo --version # package manager and build tool
rustdoc --version # rust's documentation tool
```

### IDE
1. [Install VS Code](https://code.visualstudio.com/)
2. Open VS Code. Add code to open from terminal
```
Cmd+Shift+P → type Shell Command: Install 'code' command in PATH
```

### Add extensions
<!-- Install Dev Containers, rust-analyzer(intellisense, type hints, go-to-def, Crates, CodeLLDB) and Better TOML (syntax highlighting) -->
```
Cmd+Shift+X -> Search for above extensions to install.
```

### devcontainer.json for docker
```
{
	"name": "Rust",
	// Or use a Dockerfile or Docker Compose file. More info: https://containers.dev/guide/dockerfile
	"image": "mcr.microsoft.com/devcontainers/rust:1-1-bullseye",
	// Configure tool-specific properties.
	"customizations": {
		"vscode": {
			"extensions": [
				"rust-lang.rust-analyzer"
			]
		}
	},
	// Use 'postCreateCommand' to run commands after the container is created.
	"postCreateCommand": "rustup component add clippy rustfmt"
	// Uncomment to connect as root instead. More info: https://aka.ms/dev-containers-non-root.
	// "remoteUser": "root"
}
```

### Create a folder to practice rust
```
mkdir -p path/to/your/folder
```

### Appendix I - Documentation
[RUST Install](https://www.rust-lang.org/tools/install)
