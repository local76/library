# library

> The shared foundation library for the local76 ecosystem.

`library` provides the design system, widgets, `Screensaver` trait,
screensaver runtime (Win32 GDI + Linux raw-termios), RGB controller,
system-info helpers, registry abstraction, daemon IPC, and
file-logging. Every other local76 crate depends on this one.

`library` is the only Cargo crate in the local76 ecosystem that ships
a meaningful public API. The 5 apps and the 10 screensaver shims are
consumers; `library` is the producer.

---

## Add as a dependency

For local development (recommended — every consumer in this monorepo
uses this form):

```toml
[patch."https://github.com/local76/library.git"]
library = { path = "../library" }

[dependencies]
library = { git = "https://github.com/local76/library.git", branch = "main", features = [...] }
```

The `[patch]` redirects the git URL to the local sibling directory.
Edits to library source take effect on the next `cargo build` of the
consumer.

For external consumers (CI, release):

```toml
[dependencies]
library = { git = "https://github.com/local76/library.git", tag = "v2026.6.10.1" }
```

The git tag pins to a specific published version. Cargo will not pull
a `main` branch that hasn't been tagged.

---

## Cargo features

Features are granular — depend on what you actually need.

| Feature | What it enables |
|---|---|
| `widgets` | Ratatui widgets (theme, status bar, toast, markdown viewer, layout guard, etc.) |
| `sys-info` | System info helper (CPU, memory, disk, network, hostname) |
| `window` | Console window management (borderless, centering, single-instance) |
| `service` | Background service runner (Windows Service Wrapper) |
| `event-log` | Windows Event Log writer |
| `notification` | Windows toast notifications |
| `clipboard` | Clipboard read/write |
| `reg` | Windows registry abstraction |
| `winget` | Local winget SQLite scanner |
| `chrome` | Cross-app keyboard/mouse/embedded-docs helpers (composite: pulls in `widgets`) |
| `gpu` | Headless GPU compute (wgpu) |
| `gui` | egui/eframe native windowing |
| `effects` | The 12 in-app effects (Verb × Noun × Style × Palette) |
| `screensaver-runtime` | Win32 GDI + raw-termios main loop (host a screensaver process). The 10 `screensaver-*` shim binaries enable this. |

Default features: `widgets`, `sys-info`, `window`, `service`,
`event-log`, `reg`, `winget`, `chrome`.

---

## Usage

### Diagnostic doctor

```rust
use library::apps::doctor::run_doctor;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--doctor") {
        run_doctor();
        return;
    }
}
```

### Host a screensaver process

The 10 `screensaver-*` shim binaries do this in 1 line:

```rust
fn main() {
    let effect = beams::Beams::new();
    library::screensaver_runner::run_main(effect, "beams");
}
```

`library::screensaver_runner::run_main` parses the standard
screensaver CLI args (`/s` run, `/c` configure, `/p HWND` preview,
`-h` help) and dispatches to the platform implementation. On Windows
the runtime is currently a scaffold-only stub (the full Win32 GDI
window loop is future work). On Linux/macOS the runtime runs a real
raw-termios terminal loop at 60 FPS.

### Use the design system

```rust
use library::ui::theme::ThemeColors;
use library::ui::layout::centered_rect;
use library::ui::markdown::MarkdownViewer;
use library::ui::status_bar::StatusBar;
use library::ui::toast::{ToastBox, ToastKind};
```

No single "prelude" — import the symbols you need. The design system
is a flat set of widgets in `library::ui::*`.

### Cross-app chrome helpers

```rust
use library::apps::chrome::{
    DOC_FILES, doc_for_f_key, is_doc_f_key, open_embedded_markdown,
    is_quit_key, is_help_toggle_key, scroll_for_key,
    ChromeLayout, handle_chrome_mouse,
};
```

`chrome` provides F1–F7 doc routing, quit/help key predicates, and
title-bar drag / button hit-test logic. Every app uses the same
`chrome` helpers instead of re-implementing them.

### System info

```rust
use library::toolkit::sys_info::get_system_info;

let info = get_system_info();
println!("hostname: {}", info.hostname);
println!("cpu count: {}", info.cpu_count);
```

### File logging

```rust
use library::apps::file_log;

file_log::set_log_app_name("app/helm");
file_log::log_message("INFO", "starting up");
```

Logs land at `%APPDATA%\local76\app\helm\log.txt` (Windows) or
`~/.local/state/local76/app/helm/log.txt` (Linux).

---

## Layout

```
library/src/
├── core/        # Screensaver trait, TerminalCell, ScreenPalette,
│                # hsl_to_rgb, formatting, rc_split, logo_block, error
├── ui/          # theme, layout, text, status_bar, toast, markdown,
│                # title_banner, mouse_selection, scrollbar, tabs,
│                # textbox, 12 in-app effects, screensaver_renderer
├── toolkit/     # sys_info, monitors, config, registry, ipc,
│                # rgb_controller, gpu, wlan, platforms
├── apps/        # window, guard, identity, panic, power_sync,
│                # bootstrap (raw-mode setup), console, file_log,
│                # daemon, service, event_log, notification,
│                # clipboard, doctor, chrome
└── screensaver_runner.rs   # the run_main host loop (feature-gated)
```

Cross-folder dependencies flow from less-specific to more-specific:
`core` has no dependencies, `ui` and `toolkit` depend on `core`,
`apps` depends on all of them, `screensaver_runner` depends on
`apps`. There is no formal linter enforcing this — the type system
and `cargo test --no-run` catch circular deps.

---

## Visual standards

All applications in the ecosystem share a cohesive UI style. See
[docs/VISUAL_STANDARDS.md](docs/VISUAL_STANDARDS.md) for the icon
container layout, monogram style, and branding asset packaging.

## Embedded markdown docs (F1–F7 in-app help)

The `library::embedded_docs!` macro bakes markdown files (README,
LICENSE, CONTRIBUTING, etc.) directly into your binary at compile time.
The app can show help text without reading the filesystem at runtime
— which would break in a single-file `.scr` Windows screensaver
install. See [docs/EMBEDDED_DOCS.md](docs/EMBEDDED_DOCS.md) for the
canonical example and the F1–F7 wiring pattern.

---

## Build

```pwsh
git clone https://github.com/local76/library.git
cd library
cargo build --release
```

For the full local76 build orchestrator, see
[`toolkit`](https://github.com/local76/toolkit). For a one-shot
build of everything in the monorepo, use
[`run.ps1`](https://github.com/local76/local76) at the monorepo
root.

---

## License

MIT. See [LICENSE.md](LICENSE.md).
