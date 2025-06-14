# shields.rs

![Crates.io Version](https://img.shields.io/crates/v/shields)
![Deps.rs Crate Dependencies (latest)](https://img.shields.io/deps-rs/shields/latest)
![Crates.io License](https://img.shields.io/crates/l/shields)
![Crates.io Size](https://img.shields.io/crates/size/shields)
![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/shields)
![Crates.io Total Downloads](https://img.shields.io/crates/d/shields)

A high-performance badge rendering engine written in Rust, supporting SVG output and font parsing. This project is designed for developers and services that require fast, customizable, and reliable badge generation.

**🎯 Pixel-Perfect Consistency with shields.io**

Our goal is to achieve pixel-perfect, 100% identical rendering results to [shields.io](https://shields.io/). We utilize precisely the same text length calculation data to ensure full consistency, while delivering significantly improved efficiency.

**🟢 Bitwise-Identical SVG Output**

Not only do we pursue pixel-level similarity, but we also guarantee that the generated SVG string is bitwise-identical to the output returned by shields.io for the same parameters. This ensures absolute compatibility and consistency for all use cases.

**🎨 Supported Styles**

We support all major badge styles: `flat`, `flat-square`, `plastic`, `social` and `for-the-badge`. Each style can be customized with various properties such as label, message, color, logo, and more.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024 or later)
- Cargo package manager (comes with Rust)

## Usage Example

The library provides a chainable API for customizing badges. You can set the label, message, color, and other properties using method chaining:

```rust
use shields::builder::Badge;

fn main() {
    // Simple flat badge
    let badge = Badge::flat().label("test").message("passing").build();
    println!("{}", badge);
    // Flat badge with custom colors
    let badge = Badge::plastic()
        .label("version")
        .message("1.0.0")
        .label_color("#555")
        .message_color("#4c1")
        .build();
    println!("{}", badge);
    // Plastic badge with logo
    let badge = Badge::social()
        .label("github")
        .message("stars")
        .logo("github")
        .link("https://github.com/user/repo")
        .extra_link("https://github.com/user/repo/stargazers")
        .build();
    println!("{}", badge);
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
