# library — Architecture

> The flat folder tree, the design system, the Screensaver trait, and
> the platform/runtime split that form the foundation of every tool in
> the local76 ecosystem.

`library` is the shared Cargo crate that every other local76 tool
depends on. This document explains how the code is organized, why it
is organized that way, and how to add new code without breaking the
contract.

---

## 1. Design principles

- **One crate, one foundation.** No other local76 repo re-implements
  sysinfo reads, registry helpers, Screensaver trait implementations,
  or the design-system widgets. If a piece of code will be needed by
  more than one consumer, it lives in `library`.
- **Classification by what.** Code is organized by what each module
  *does*, not by which architectural axis it nominally belongs to.
- **Per-app isolation.** Each local76 app must run independently and
  must not interfere with the others. Three concrete rules:
  - **No global port binding.** No HTTP server on `127.0.0.1:8080`,
    no shared TCP port, no shared UDP port. Use local IPC: Unix
    domain sockets on Linux, Named Pipes on Windows.
  - **Per-app config storage.** Each app's config file lives at
    `%APPDATA%\local76\app\<app>\config.yaml` (Windows) or
    `~/.config/local76/app/<app>/config.yaml` (Linux). Registry
    access uses `Software\local76\<app>\*` paths. The app name is
    the namespace.
  - **Executable-scoped guards.** Single-instance guards and mutex
    locks are scoped to the executable's name, not the library's.

---

## 2. The flat folder layout

```
                      ┌─────────────────────────────────┐
                      │   core                          │
                      │   (neutral foundation)          │
                      └────────────────┬────────────────┘
                                       │
                ┌──────────────────────┴──────────────────────┐
                ▼                                             ▼
      ┌───────────────────┐                         ┌───────────────────┐
      │   ui              │                         │   toolkit         │
      │   (presentation)  │                         │   (platform APIs) │
      └─────────┬─────────┘                         └─────────┬─────────┘
                │                                             │
                └──────────────────────┬──────────────────────┘
                                       ▼
                            ┌───────────────────┐
                            │   apps            │
                            │   (lifecycle/run) │
                            └───────────────────┘
```

### Module responsibilities

#### 2.1 `core` — neutral foundation
- `Screensaver` trait (backend-agnostic, depended on by every scene)
- `ScreensaverState` sub-trait (active/focused flags)
- `TerminalCell` (a single grid cell, ratatui-free)
- `ScreenPalette` (the cross-renderer color story, 8 RGB tuples)
- `LcgRng` (deterministic RNG for reproducible effects)
- `hsl_to_rgb`, `hsv_to_rgb`
- `render_logo_block`
- `formatting`, `error`, `rc_split` (ICO splitter for the build pipeline)

#### 2.2 `ui` — presentation
- Ratatui widgets (theme, status bar, toast, markdown viewer, layout
  guard, title banner, effect preview, mouse selection, scrollbar,
  text box, tabs)
- `ScreensaverRenderer` (the buffer-management helper for
  `[TerminalCell]` grids)
- The 12 in-app effects (`FallingGlyphs`, `FlowingParticles`, …) in
  `ui::effects::*`

#### 2.3 `toolkit` — platform utilities
- `sys_info` (Windows + Linux)
- `monitors`, `gpu`, `wlan`
- `config`, `registry`
- `ipc`, `clipboard`, `packages`
- `rgb_controller`, `rgb_protocol`
- Per-platform splits (`platforms::native`, `platforms::embedded`,
  `platforms::web`, `platforms::mobile`)

#### 2.4 `apps` — lifecycle & run control
- Console window state (`window::*`, `bootstrap`, `console`)
- Process isolation (`guard::SingleInstanceGuard`)
- Panic hook setup (`panic::set_panic_hook`)
- File log writing (`file_log`) and Windows Event logger
  (`event_log`)
- Background daemon loop runners (`daemon`) and service controls
  (`service`)
- Notifications, clipboard, identity
- Cross-app chrome helpers (`chrome`) — F1–F7 doc routing,
  keyboard/mouse predicates, title-bar drag

#### 2.5 (removed) — `screensavers` was here
The 10 screensaver scenes were moved out of `library` into their
own sibling repos (`screensaver-beams`, `screensaver-bounce`, etc.)
in the 2026.6.9 release. The scenes are still implemented in Rust
(they implement `library::core::screensaver::Screensaver`), but each
scene's source lives in its own repo, not in `library`. The
`library::screensaver_runner::run_main` host loop is the runtime
that drives the scenes.

---

## 3. The design system

The design system in `library::ui::*` is a flat set of widgets that
every local76 app uses. There is no single `prelude` — import the
symbols you need.

### 3.1 Color story

`library::core::screen_palette::ScreenPalette` is the canonical
color story:

```rust
pub struct ScreenPalette {
    pub bg:     (u8, u8, u8),
    pub fg:     (u8, u8, u8),
    pub accent: (u8, u8, u8),
    pub dim:    (u8, u8, u8),  // 35% of accent
    pub hot:    (u8, u8, u8),  // accent hue +30°
    pub cool:   (u8, u8, u8),  // accent hue -120°
    pub mid:    (u8, u8, u8),  // neutral chrome
    pub peak:   (u8, u8, u8),  // white-hot peaks
}
```

The same RGB tuples drive both ratatui chrome and GDI pixel
renderers. `query_current_palette()` is the cross-platform helper
that returns one.

### 3.2 The `Screensaver` trait

```rust
pub trait Screensaver {
    fn init(&mut self, _cols: usize, _rows: usize) {}
    fn update(&mut self, dt: Duration, cols: usize, rows: usize);
    fn draw(&self, grid: &mut [TerminalCell], cols: usize, rows: usize);
    fn has_scanlines(&self) -> bool { false }
}
```

The trait lives in `core` because it depends only on `TerminalCell`
(also in `core`) and `std::time::Duration`. Both in-app effects and
screensaver scenes can implement it. `ScreensaverRenderer` is the
ratatui-side buffer manager.

---

## 4. Adding new code

1. **Classify by topic.** Which of the 4 folders does this code
   belong to? If it is neutral and reusable, it goes in `core`. If
   it depends on a presentation layer, an OS, a service, or a task,
   it goes in the matching submodule.
2. **Place in the matching module.** Use `core` only for truly
   neutral data.
3. **Gate behind the appropriate Cargo feature.** `widgets` for
   ratatui, `sys-info` for system info, `service` for background
   services, etc.
4. **Update this `ARCHITECTURE.md` if you add a new pattern.**
5. **Provide cross-platform stubs where possible.** A
   `toolkit::monitors::get_monitors_summary` on Linux should return
   the Xinerama / XRandR result, not a Windows-only stub.
6. **Avoid putting presentation / lifecycle / platform code into
   `core`.** Cross-folder dependencies flow from less-specific to
   more-specific.

---

## 5. Windows / Linux / GitHub

- **Windows / Linux**: Per-platform splits via
  `#[cfg(target_os = "...")]` and `platforms::native::windows` /
  `platforms::native::linux` modules. Cross-platform helpers expose
  the unified API.
- **GitHub**: The org is at `github.com/local76`. The
  `[patch."https://github.com/local76/library.git"]` redirect in
  every consumer's `Cargo.toml` makes local dev work without a
  network round-trip.

---

## 6. Consumers

- The 10 `screensaver-<scene>` repos — each is a 1-line
  `library::screensaver_runner::run_main(scene, name)` wrapper.
- The 5 apps — helm, pulse, scout, trance, ignite. Each depends on
  `library` for widgets, sys_info, config, and chrome helpers.

---

## 7. Future

- Drop the `scenes` and `effects` no-op features (kept for one cycle
  for backward compat).
- `apps::window` split into per-platform modules
  (`window_win`, `window_linux`, `window_macos`).
- `wgpu` renderer as an alternative to ratatui.
- eBPF (extended Berkeley Packet Filter) integration.
