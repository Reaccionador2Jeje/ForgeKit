# ForgeKit

**ForgeKit** is a cross-platform system diagnostics and maintenance toolkit written in Rust.

The project is developed by **Atdev Dev** and aims to provide a clear command-line interface for inspecting system information, memory, storage, and basic health indicators.

## Current status

Version: `0.1.0` — initial development.

### Available commands

| Command | Description |
| --- | --- |
| `forgekit info` | Display operating system and hardware information |
| `forgekit memory` | Display memory usage |
| `forgekit storage` | List storage devices and capacity |
| `forgekit health` | Run basic read-only health checks |
| `forgekit diagnose` | Run all available diagnostics |
| `forgekit version` | Display the application version |

## Build

Requires a recent stable Rust toolchain.

```sh
cargo check
cargo build --release
```

Run a command during development:

```sh
cargo run -- diagnose
```

## Safety

The initial release performs read-only diagnostics. It does not delete files, modify system settings, or run repair operations.

## License

MIT. See [LICENSE](LICENSE) when added.
