rCommon Support and Troubleshooting

Thank you for using rCommon (Rust TUI Shared Utility and Widget Library)! If you are experiencing issues, follow these steps to get help.

Library Integration Issues

If you are having compilation or integration errors in your application:
1. Verify that your `Cargo.toml` points to the correct Git repository:
   ```toml
   rcommon = { git = "https://github.com/local76/rCommon.git", branch = "main" }
   ```
2. Run `cargo clean` and rebuild to clear any stale build caches.
3. Check compile-time errors or warnings to ensure matching platforms (e.g. conditional stubs for non-Windows targets).

Open an Issue

If you find a bug in the shared library code, please open an issue in the official repository:
https://github.com/local76/rCommon/issues

What to include:
* Your build target environment (e.g. Windows 11, Fedora, Pop!_OS).
* Rust toolchain version (`rustc --version`).
* The compiler output logs and stack trace if you encountered a library panic.
