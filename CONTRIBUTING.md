Contributing to rCommon

We are thrilled that you want to help improve rCommon! Contributions from the community are what make open-source projects so special. Please follow these guidelines to make sure your contribution matches the style and quality standards of the project.

Developer Environment Setup

To build and test rCommon locally:
1. Make sure you have the standard Rust toolchain installed.
2. Clone this repository.
3. Check code formatting using `cargo fmt --check`.
4. Run standard compiler lints using `cargo clippy`.
5. Run unit tests using `cargo test`.
6. Build the library using `cargo build --release`.

Pull Request Process

1. Fork the repository and create a new feature branch.
2. Write clean code and keep your changes focused.
3. Make sure all compile checks, lints, and unit tests pass.
4. Document any new features in the README.md or corresponding module documentation.
5. Open a Pull Request detailing the purpose of your change and any design decisions you made.

Design Principles

If you are modifying the library:
* **Cross-Platform Compatibility**: Ensure all code compiles on both Windows and Linux target architectures. Use conditional compilation (`#[cfg(target_os = "windows")]` and stubs) to keep dependencies clean.
* **Modular Architecture**: Keep modules focused (e.g., `win32`, `reg`, `widgets`).
* **Ratatui Widgets**: Keep widgets generic, reusable, and customizable for any ratatui application.
