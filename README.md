# SkyCast

A Rust CLI utility that displays the current weather in the style of **neofetch**.

## Description

SkyCast is a minimal, cross-platform command-line tool written in Rust that fetches and beautifully displays current weather information for a given city — inspired by the aesthetic of `neofetch`.  
This project serves both as a practical utility and a learning exercise in async Rust, HTTP clients, and CLI design.

## Installation and Run

### Ready-made binaries
Pre-built binaries for Linux, macOS, and Windows are available in the [Releases](https://github.com/prosto-chuvak/SkyCast/releases) section. Download the appropriate archive, extract it, and run the executable.

### Building from source code
1. Clone the repository:
```bash
git clone https://github.com/your-username/skycast
cd skycast
```

2. Build the project in release mode:
```bash
cargo build --release
```

3. Run the utility:
```bash
./target/release/skycast -c <city>
# or (on Windows)
.\target\release\skycast.exe -c <city>
```

## Usage

SkyCast supports the following command-line options:

| Flag | Long form      | Description                     |
|------|----------------|---------------------------------|
| `-h` | `--help`       | Show help message               |
| `-c` | `--city <CITY>`| Specify city name (required)    |
| `-V` | `--version`    | Print version information       |

### Examples

```bash
# Get weather for Cairo
skycast -c Cairo

# Get weather for New York
skycast --city "New York"

# Show version
skycast -V
```

> **Note**: The city name is case-insensitive and supports spaces when quoted.

## Requirements

- Rust (installed via [rustup](https://rustup.rs/))
- `cargo` (included with Rust)
- Internet connection (to fetch weather data)

## Dependencies

This project uses the following crates:
- `clap` — for CLI argument parsing
- `serde` — for JSON deserialization
- `tokio` — async runtime
- `reqwest` — HTTP client

## License

Distributed under the **GNU General Public License v3.0**.  
See [LICENSE](LICENSE) for more information.
```
