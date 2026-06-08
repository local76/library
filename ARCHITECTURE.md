# rCommon Architecture

rCommon is the shared foundation library for the local76 "r*" ecosystem of local-first terminal and system utilities (rIdle, rFetch, rMonitor, rTemplate, rWifi, rStartup, rIdle-scenes, etc.).

## Design Principles

- **Avoid duplication**: Common utilities, platform abstractions, TUI widgets, effects, and system info live here so individual apps stay small and focused.
- **Classification by Taxonomy**: Code is organized according to the 4-layer taxonomy to prevent accidental coupling between concerns (e.g., a TUI-only effect type being changed in a way that breaks a background service or CLI tool).
- **Absolute Isolation & Non-Interference**: All `rApps` must work independently on their own and not interfere with one another. To guarantee this:
  - **No Global Port Binding**: Do not bind to shared network ports (like HTTP servers on `127.0.0.1:8080`) for communication or single-instance locks. This prevents firewall prompts and port collisions. Use application-scoped Local IPC (Unix domain sockets on Linux and Named Pipes on Windows) instead.
  - **Isolated Configuration Storage**: Ensure configuration schemas and storage access (like Registry paths or configuration files) are strictly namespaced under the application name (`Software\rApps\<AppName>`) so they never overwrite each other.
  - **Executable-Scoped Guards**: Single-instance guards and Mutex locks must be scoped using the executable's name to avoid multi-instance conflicts without locking out other `rApps`.


### 1. Interface (Presentation Layer)
How the software communicates visually (or non-visually) with the user or other software.
- **CLI** (Command Line Interface)
- **TUI** (Text User Interface) — ratatui-based, TerminalCell grids, effects, widgets, logo rendering.
- **GUI: Native/OS** — Standard WIMP using OS toolkit.
- **GUI: Custom/Game Engine** — Continuous-loop canvas (e.g., egui, custom renderers).
- **Headless / API** — No UI; communicates with other software (REST, IPC, libraries, daemons exposing APIs).

### 2. Execution State (Lifecycle)
How the OS manages the application's runtime.
- **Foreground Applications** — Require active user attention, window/terminal focus (console guards, hiding, single-instance, title management).
- **Background Processes** — Run silently, often at startup, no interface (services, event logging, power management for daemons).

### 3. Platform & Architecture (Deployment)
How the software is packaged and where it runs.
- **Native Applications** — Compiled for host OS/hardware (primary focus: Windows + Linux via cfg(target_os)).
- **Web Applications** — Browser (stubs/future).
- **Mobile Applications** — iOS/Android (future).
- **Embedded Software** — Dedicated hardware (routers, etc., future).

Windows and Linux specifics (FFI, services, registry emulation, console behavior, power, monitors, etc.) live primarily here, with platform splits (e.g., `platform/native/windows.rs`).

GitHub fits as the **distribution and collaboration mechanism** for the Native + git-dependency model used across the suite (e.g., `rcommon = { git = "https://github.com/local76/rCommon.git" }` + `[patch]` for local dev). It is not a runtime concern but enables the ecosystem.

### 4. System Role (Purpose)
The software's ultimate objective.
- **System Software** — Infrastructure: manages hardware, provides platform (low-level power, registry, services, event logs, disk enumeration, BIOS, shell detection).
- **Application Software** — Task-oriented for end-users or tools (RGB control, games/effects, package inventory, TUI dashboards, higher-level formatting).

## Current Module Structure (Aligned to Taxonomy)

- **core/** (or core.rs): Neutral, cross-cutting primitives usable by *any* combination of the above. Must remain free of heavy UI, OS, or lifecycle assumptions.
  - LcgRng (deterministic RNG for effects/games)
  - TerminalCell (universal for grid renderers: TUI effects, headless logging, custom GUIs)
  - DashboardInfo / SystemInfo (rich live system context with logo_text, uptime, mem, power, etc.)
  - get_dashboard_info, get_packages_breakdown, get_monitors_summary, render_logo_block, etc.
  - Classification: Core foundation for all layers.

- **interface/**
  - **tui/**: TUI-specific presentation (widgets like AccentGauge, effects like MatrixRain/ParticleSystem, text wrapping/alignment helpers, ObstacleJump game rendering, logo block).
    - Classification: Interface (TUI) + Role (Application) + some Platform.
  - **gui/**: GUI helpers (eframe/egui for custom/game-engine UIs).
    - Classification: Interface (GUI Custom).
  - **api/**: Headless/API surfaces (IPC, service exposure, library interfaces).
    - Classification: Interface (Headless/API).
  - Future: cli/, gui_native/.

- **lifecycle/**
  - **foreground/**: Console/window management for apps that need focus (BorderlessConsole, SingleInstanceGuard, ConsoleTitleGuard, hide_console_at_startup, relaunch_in_conhost).
    - Classification: Lifecycle (Foreground) + Platform (Native).
  - **background/**: Silent running (services, event logging, notifications, clipboard for daemons, thread execution state).
    - Classification: Lifecycle (Background) + Role (System).

- **platform/**
  - **native/**: OS-specific deployment (sys_info with Windows/Linux splits, reg, monitor enumeration, power, dark mode, accent via DWM/registry, console DPI, etc.).
    - Classification: Platform (Native) + Role (System).
  - Future: web/, mobile/, embedded/.

- **role/**
  - **system/**: Infrastructure (low-level registry, power, services, event logs, disk/BIOS queries).
    - Classification: Role (System Software).
  - **application/**: Higher-level/task-oriented (RGB controller, games/effects integration, package inventory, formatting helpers like get_battery_info).
    - Classification: Role (Application Software).

- **Other**:
  - `rgb/`: RGB lighting control (OpenRGB protocol + controller). Classification: Role (Application) + Interface (for effects).
  - `bin/rpack.rs`: Packaging tool (builds, deb/rpm, etc.). Classification: Tooling / Role (Application).
  - Legacy `win32` shim (deprecated, for old consumers).

## Windows OS, Linux OS, and GitHub

- **Windows/Linux**: Primarily **Platform (Native)** with strong influence on **Lifecycle** (services vs daemons, conhost behavior) and **Role (System)** (registry vs config files, DWM, power APIs). Code uses `#[cfg(target_os = "...")]` and splits (e.g., `platform/native/windows.rs`, `sys_info/windows.rs`).
- **GitHub**: External to runtime taxonomy. Supports **Platform (Native)** distribution via git dependencies and releases. Also hosts org profile, workflows, and packaging metadata. The monorepo-like local setup (git_push_all, patches) relies on it.

## Adding New Code

1. Classify using the taxonomy.
2. Place in the matching module (use `core` only for truly neutral data).
3. Add documentation with classification comment.
4. Gate behind appropriate feature (e.g., `effects` for TUI visuals, `sys-info` for platform queries).
5. Update this ARCHITECTURE.md and relevant mod.rs docs.
6. Provide cross-platform stubs where possible.
7. Avoid putting presentation/lifecycle code into `core`.

## Current State & Audit Notes

The structure has been cleaned up and aligned (see list of 10 tasks executed in the refactor session). Old flat files were moved into taxonomy categories. Re-exports preserve compat for r* consumers using git + [patch].

**Audit of Ports from Other Projects** (valuable reusable pieces extracted and classified):
- From rIdle-scenes/ridle-core: SystemInfo/get_system_info (core + platform), render_logo_block + 5x5 (interface/tui), LcgRng enhancements (core), registry/theme helpers (already via core/platform).
- From rIdle-scenes effects (rObstacleJump, rLife, rMatrix, etc.): Particle systems, MatrixRain, ObstacleJump logic (interface/tui + role/application), console typing/dashboard patterns.
- From rFetch: Package counting (role/application/packages.rs), monitor enumeration (platform/native/monitors.rs), accent/dark mode/power formatters (platform + lifecycle).
- From rIdle (saver_win32): Advanced console (high contrast, thread exec state, titles, screensaver control) (lifecycle/foreground/console.rs), power/accent delegation.
- From rMonitor/rStartup/rTemplate/rWifi: Common win32 shims, console hiding, system queries (now centralized in lifecycle/platform/role).
- rpack bin and rgb/game already in rCommon (role/application).

## Migration Guide for Consumers

To ensure long-term architecture sustainability, consumers should move away from the deprecated `rcommon::win32` module (legacy flat shim) and transition to the new 4-layer taxonomy modules.

> [!NOTE]
> **Taxonomy Features (Cargo features)** control what code is compiled in your `Cargo.toml` dependencies, whereas **Taxonomy Paths (module paths)** are the new Rust import locations in your code.

Here is a concrete "Before & After" mapping for imports and usage:

### 1. Presentation Layer (Interface)
* **TUI Effects / Primitives**:
  * *Before*: `use rcommon::win32::{TerminalCell, MatrixRain, SimpleParticles};`
  * *After*: `use rcommon::interface::tui::effects::{TerminalCell, MatrixRain, SimpleParticles};` (or `rcommon::core::TerminalCell` / `rcommon::interface::tui::MatrixRain`)
* **TUI Focus Widgets**:
  * *Before*: `use rcommon::widgets::{AccentList, AccentTabs};`
  * *After*: `use rcommon::interface::tui::widgets::{AccentList, AccentTabs};`
* **Headless IPC**:
  * *Before*: `use rcommon::api::{IpcServer, IpcClient};`
  * *After*: `use rcommon::interface::api::{IpcServer, IpcClient};`

### 2. Execution State Layer (Lifecycle)
* **Console & Window Controls**:
  * *Before*: `use rcommon::win32::{hide_console_at_startup, BorderlessConsole, ConsoleTitleGuard};`
  * *After*: `use rcommon::lifecycle::foreground::window::{hide_console_at_startup, BorderlessConsole, ConsoleTitleGuard};`
* **Single Instance Lock**:
  * *Before*: `use rcommon::win32::SingleInstanceGuard;`
  * *After*: `use rcommon::lifecycle::foreground::guard::SingleInstanceGuard;`
* **Services & Notifications**:
  * *Before*: `use rcommon::win32::{query_windows_service_status, show_toast_notification};`
  * *After*: `use rcommon::lifecycle::background::service::query_service_status;` and `use rcommon::lifecycle::background::notification::show_toast_notification;`

### 3. Platform & Architecture Layer (Deployment)
* **System Theme & Uptime Helpers**:
  * *Before*: `use rcommon::win32::{query_dark_mode, get_system_screen_resolution, get_dwm_accent_color};`
  * *After*: `use rcommon::platform::native::sys_info::{query_dark_mode, get_system_screen_resolution, get_dwm_accent_color};`
* **Monitor / Screen Enumeration**:
  * *Before*: `use rcommon::win32::{get_monitors_summary, get_all_monitors};`
  * *After*: `use rcommon::platform::native::monitors::{get_monitors_summary, get_all_monitors};`

### 4. System Role Layer (Purpose)
* **Application Roles (RGB/Packages)**:
  * *Before*: `use rcommon::win32::{get_packages_breakdown, RgbController};`
  * *After*: `use rcommon::role::application::packages::get_packages_breakdown;` and `use rcommon::role::application::rgb::controller::RgbController;`
* **Registry Access (Infrastructure)**:
  * *Before*: `use rcommon::win32::{read_string, write_string, HKEY_CURRENT_USER};`
  * *After*: `use rcommon::platform::native::reg::{read_string, write_string, HKEY_CURRENT_USER};`

This structure allows multiple crates per section in the future (e.g., rcommon-interface-tui as a separate crate) while keeping the single-crate experience simple for git-based consumption in the r* apps.

## Related Projects (for context)

- rIdle / rIdle-scenes: Heavy use of TUI effects, lifecycle (screensavers as background/foreground), platform (console/windowing).
- rFetch / rMonitor / rStartup: System info, package inventory, monitors, power, dark mode (Role + Platform + Interface TUI).
- rTemplate (incl. window subcrate): GUI custom + TUI, diagnostics.
- rWifi: Platform (WLAN), TUI, lifecycle.
- All use rCommon via git + local [patch] for development.

## Future

- Full port of reusable effects from rIdle-scenes.
- Stronger API/Headless support.
- Better CLI vs TUI separation.
- CI enforcement of taxonomy (no cross-layer imports).
- Potential workspace split per major section.