# rCommon

`rcommon` is the unified utility, integration, and widget library shared across the `rApps` ecosystem, supporting both Windows and Linux environments. By consolidating platform-specific APIs, terminal handling, custom widgets, and utility routines into a single reusable library, it ensures visual and functional consistency across all terminal applications while enabling seamless cross-platform compilation.

## Purpose

Each application in the `rApps` suite (such as `rFetch`, `rMonitor`, `rSaver`, `rStartup`, and `rWifi`) is designed to run as a standalone TUI tool on both Windows and Linux. 

Instead of copying and pasting code across repositories, `rCommon` is imported directly via Git in each application's `Cargo.toml`. This allows:
1. **Unified Design System**: A single source of truth for accent-color calculations and custom widgets.
2. **Simplified Maintenance**: Bug fixes or improvements to native console setups or OS event logging are made once and automatically rolled out to all apps upon compilation.
3. **Cross-Platform Compilation**: Clean separation of native OS APIs (via conditional compilation) so the entire suite compiles out-of-the-box on Linux as well.

---

## Core Modules

### 1. `win32` (OS Integration Layer)
Provides platform-specific system calls, console emulator tracking, and layout state. On Windows, it binds to native Win32/WinRT APIs. On Linux, a portable stub fallback is automatically compiled to maintain cross-platform build compatibility:
* **Terminal Initialization & Size Guarding**: DPI-aware console resizing on Windows (standardized fallback to `100x35`), layout constraints monitoring, and clean alternate screen state switching.
* **Console Emulator Detection**: Traverses the parent process tree (detecting if running in legacy `conhost.exe` vs. modern `Windows Terminal` or `VS Code`) to fall back to ASCII characters instead of rich Unicode glyphs where appropriate.
* **Event Logging**: Direct FFI integration with Windows Event Log (`advapi32` APIs like `RegisterEventSourceW` and `ReportEventW`) on Windows.
* **Theme & Accent Color Adaptation**: Queries DWM registry settings on Windows to match active system accent coloring in real-time, falling back to standard ANSI palette defaults on Linux.
* **Clipboard Integration**: Clipboard copy-on-release handling allowing text highlighted via terminal mouse actions to be copied to the OS clipboard.
* **Desktop Notifications**: Lightweight, asynchronous WinRT-based toast notifications (using XML DOM and notification manager) on Windows.
* **Power telemetry**: Battery-level polling to dynamically double the TUI poll interval when on battery power to conserve system resources.
* **Panic Hooks**: Registers system handlers to restore terminal state and prevent screen corruption in case of unexpected crashes.

### 2. `reg` (Registry Configuration)
Helper utilities for working with the Windows Registry, wrapping `winreg` to safely retrieve configuration keys and theme settings on Windows, with a mock stub module compiled on Linux to preserve compilation compatibility.

### 3. `widgets`
Custom widgets designed specifically for the `ratatui` ecosystem that adapt to the target system's design style, rendering styled lists and progress bars on all supported platforms:
* **`AccentList`**: A custom list widget that highlights the selected item in the active system accent color.
* **`AccentGauge`**: A progress bar/gauge matching the system accent styling.

---

## Integration in rApps

To consume `rCommon` in any `rApp` or when starting from `rTemplate`, add it as a Git dependency in your project's `Cargo.toml`:

```toml
[dependencies]
rcommon = { git = "https://github.com/local76/rCommon.git", branch = "main" }
```

In your application code, import modules directly from `rcommon`:

```rust
use rcommon::win32;
use rcommon::widgets::{AccentList, AccentGauge};
```
