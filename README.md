# rCommon

`rcommon` is the unified Windows integration, DWM styling, and utility library shared across the `rApps` ecosystem. By consolidating low-level Windows APIs, console handling, custom widgets, and helper routines into a single reusable library, it ensures visual and functional consistency across all utilities while drastically reducing code duplication.

## Purpose

Each application in the `rApps` suite (such as `rFetch`, `rMonitor`, `rSaver`, `rStartup`, and `rWifi`) is designed to run as a standalone Windows TUI tool. However, they share a common set of design standards and Windows-specific integrations. 

Instead of copying and pasting code across repositories, `rCommon` is imported directly via Git in each application's `Cargo.toml`. This allows:
1. **Unified Design System**: A single source of truth for accent-color calculations and custom widgets.
2. **Simplified Maintenance**: Bug fixes or improvements to native console setups or OS event logging are made once and automatically rolled out to all apps upon compilation.
3. **Standalone Portability**: Apps can still compile independently without requiring a local cargo workspace.

---

## Core Modules

### 1. `win32`
The core integration layer with native Windows APIs and WinRT, providing:
* **Terminal Initialization & Size Guarding**: DPI-aware console resizing (standardized fallback to `100x35`), layout constraints monitoring, and clean alternate screen state switching.
* **Console Emulator Detection**: Traverses the parent process tree (detecting if running in legacy `conhost.exe` vs. modern `Windows Terminal` or `VS Code`) to fall back to ASCII characters instead of rich Unicode glyphs where appropriate.
* **Windows Event Logging**: Direct FFI integration with Windows Event Log (`advapi32` APIs like `RegisterEventSourceW` and `ReportEventW`).
* **Dynamic Accent Color & Theme Mode Polling**: Reads DWM registry keys to match the active Windows theme and accent coloring in real-time.
* **Clipboard Copy-on-Release**: Cross-platform/win32 handling to automatically copy text highlighted via terminal mouse actions to the Windows clipboard.
* **Native Desktop Toast Notifications**: Lightweight, asynchronous WinRT-based toast notifications (using XML DOM and notification manager) replacing the legacy PowerShell-spawning shims.
* **CPU/Power Conservation**: Battery-level polling to dynamically double the TUI poll interval when on battery power.
* **Panic Hooks**: Registers robust system handlers to restore terminal state and prevent screen corruption in case of unexpected crashes.

### 2. `reg`
Helper utilities for working with the Windows Registry, wrapping `winreg` to safely retrieve configuration keys and theme settings.

### 3. `widgets`
Custom widgets designed specifically for the `ratatui` ecosystem that automatically inherit the system's DWM accent theme:
* **`AccentList`**: A custom list widget that highlights the selected item in the active Windows accent color.
* **`AccentGauge`**: A progress bar/gauge matching the Windows accent styling.

---

## Integration in rApps

To consume `rCommon` in any `rApp` or when starting from `rTemplate`, add it as a Git dependency in your project's `Cargo.toml`:

```toml
[dependencies]
rcommon = { git = "https://github.com/tourian-dynamics/rCommon.git", branch = "main" }
```

In your application code, replace local module declarations (`mod win32;`, `mod reg;`, `mod widgets;`) with imports from `rcommon`:

```rust
use rcommon::win32;
use rcommon::widgets::{AccentList, AccentGauge};
```
