# shields

A high-performance badge rendering engine written in Rust, supporting SVG output and font parsing. This project is designed for developers and services that require fast, customizable, and reliable badge generation, similar to [shields.io](https://shields.io/), but with a focus on performance and extensibility.

## Features

- ⚡ **High Performance**: Built with Rust for maximum speed and efficiency.
- 🖼️ **SVG Output**: Generates crisp, standards-compliant SVG badges.
- 🔤 **Font Parsing**: Supports custom font rendering using TTF parsing.
- 🧠 **LRU Caching**: Efficient in-memory caching for repeated badge requests.
- 🛠️ **Extensible API**: Easy to integrate and extend for various use cases.
- 🧪 **Comprehensive Testing**: Includes tests for rendering accuracy and performance.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024 or later)
- Cargo package manager (comes with Rust)

### Build

Clone the repository and build the project:

```bash
git clone https://github.com/Jannchie/shields.git
cd shields
cargo build --release
```

### Run

To run the badge rendering engine:

```bash
cargo run --release
```

## Usage Example

Here is a basic example of how to use the shields library in your Rust project:

```rust
use shields::BadgeRenderer;

fn main() {
    let renderer = BadgeRenderer::new();
    let svg = renderer.render("build", "passing", "#4c1");
    std::fs::write("badge.svg", svg).unwrap();
}
```

This will generate a `badge.svg` file with a "build: passing" badge.

### 链式调用示例

你可以通过链式调用的方式快速构建和渲染徽章：

```rust
use shields::Badge;

fn main() {
    let svg = Badge::new()
        .set_label("build")
        .set_message("passing")
        .render();
    std::fs::write("badge.svg", svg).unwrap();
}
```

## Contributing

Contributions are welcome! To get started:

1. Fork the repository on [GitHub](https://github.com/Jannchie/shields).
2. Create a new branch for your feature or bugfix.
3. Commit your changes with clear messages.
4. Open a pull request describing your changes.
5. For issues, please use the [GitHub Issues](https://github.com/Jannchie/shields/issues) page.

Before submitting a PR, ensure all tests pass:

```bash
cargo test
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Community & Contact

- GitHub: [https://github.com/Jannchie/shields](https://github.com/Jannchie/shields)
- Documentation: [https://docs.rs/shields](https://docs.rs/shields)
- Author: Jannchie (<jannchie@gmail.com>)
