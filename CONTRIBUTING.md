# Contributing to library

Thank you for your interest in contributing to `library`! This
document outlines the guidelines and best practices for developing,
testing, and contributing to the shared foundation of the `local76`
ecosystem.

---

## 🏛️ Architecture

All code added to `library` should be classified by topic. The 4
folders are:

1. **`core/`** — neutral foundation. No heavy UI, platform, or
   lifecycle dependencies. The `Screensaver` trait, `TerminalCell`,
   `ScreenPalette`, `LcgRng`, color conversion, formatting, error
   types, the ICO splitter for the build pipeline, and the logo
   block renderer all live here.

2. **`ui/`** — presentation. Ratatui widgets, the design system
   (`theme`, `layout`, `text`, `status_bar`, `toast`, `markdown`,
   `title_banner`, `mouse_selection`, `scrollbar`, `textbox`,
   `tabs`, `effect_preview`), the `ScreensaverRenderer`, and the
   12 in-app effects in `ui::effects::*`.

3. **`toolkit/`** — platform utilities. `sys_info`, `monitors`,
   `config`, `registry`, `ipc`, `clipboard`, `packages`,
   `rgb_controller`, `gpu`, `wlan`, and per-platform splits under
   `platforms::*`.

4. **`apps/`** — lifecycle & run control. `window`, `guard`,
   `identity`, `panic`, `bootstrap`, `console`, `file_log`,
   `daemon`, `service`, `event_log`, `notification`, `clipboard`,
   and the cross-app `chrome` helpers.

Truly neutral primitives that don't pull in heavy platform or
presentation assumptions go in `core`. Everything else goes in the
folder whose topic it most directly serves.

---

## 📦 Feature gates

To prevent bloat in small CLI daemons, all new functionality must be
gated behind appropriate Cargo features. Features are **granular** —
depend on what you actually need, not on a layered umbrella.

- `widgets` — ratatui widgets (theme, status bar, toast, markdown
  viewer, layout guard, etc.)
- `sys-info` — system info helpers
- `window` — console window management
- `service` — background service runner
- `event-log` — Windows Event Log writer
- `notification` — Windows toast notifications
- `clipboard` — clipboard read/write
- `reg` — Windows registry abstraction
- `winget` — local winget SQLite scanner
- `chrome` — cross-app keyboard/mouse/embedded-docs helpers
  (composite: pulls in `widgets`)
- `gpu` — headless GPU compute
- `gui` — egui/eframe native windowing
- `effects` — the 12 in-app effects
- `screensaver-runtime` — Win32 GDI + raw-termios main loop (the 10
  `screensaver-*` shim binaries enable this)

Place all raw library dependencies (`ratatui`, `crossterm`, `sysinfo`,
`winreg`, `rusqlite`, etc.) under optional dependencies and gate them
with the feature flags above.

---

## 🛠️ Cross-platform standards

`library` supports both **Windows** and **Linux** targets.

- Code that uses raw system APIs (Win32 or Linux `/sys`/`/proc`)
  must have appropriate `#[cfg(target_os = "...")]` guards.
- Provide clean cross-platform fallbacks or stubs for other
  operating systems to prevent compilation failures.

---

## 🧪 Testing guidelines

Every new module or helper must include unit tests.

### Running tests

```bash
# Test with the default feature set
cargo test

# Test with all features enabled
cargo test --all-features

# Verify compilation under all feature combinations
cargo check --all-targets --all-features
```

The library currently has 100+ tests across the `design_facade`,
`taxonomy_compliance`, `sys_info_tests`, and per-module unit tests.
The chrome module has 29 of those.

---

## 📥 Pull request process

1. **Fork & branch**: Create a feature branch from `main`.
2. **Implement**: Write code that follows the 4-folder classification,
   add docstrings, and implement tests.
3. **Audit**: Run `cargo check --all-targets --all-features` and
   `cargo test --all-features`.
4. **Document**: Update the [CHANGELOG.md](CHANGELOG.md) under the
   `[Unreleased]` section for any user-facing API changes.
5. **Submit**: Create a PR targeting `main` at
   `https://github.com/local76/library/pulls`.
