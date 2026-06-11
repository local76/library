//! One-shot bootstrap: enable raw mode, alt screen, mouse capture, console sizing.
//!
//! Encapsulates the raw-mode + alt-screen + size + borderless dance that
//! starts every app's `main()`. Returns a `Terminal` ready for rendering.

use std::sync::atomic::{AtomicBool, Ordering};

static APP_SHUTTING_DOWN: AtomicBool = AtomicBool::new(false);

/// Signal that the application is shutting down.
pub fn set_app_shutting_down(val: bool) {
    APP_SHUTTING_DOWN.store(val, Ordering::Relaxed);
}

/// Check if the application is shutting down (for background threads).
pub fn is_app_shutting_down() -> bool {
    APP_SHUTTING_DOWN.load(Ordering::Relaxed)
}

#[cfg(all(target_os = "windows", feature = "widgets"))]
unsafe extern "system" fn ctrl_handler(ctrl_type: u32) -> windows_sys::Win32::Foundation::BOOL {
    use windows_sys::Win32::Foundation::FALSE;
    use windows_sys::Win32::System::Console::{
        CTRL_C_EVENT, CTRL_BREAK_EVENT, CTRL_CLOSE_EVENT, CTRL_LOGOFF_EVENT, CTRL_SHUTDOWN_EVENT
    };

    match ctrl_type {
        CTRL_C_EVENT | CTRL_BREAK_EVENT | CTRL_CLOSE_EVENT | CTRL_LOGOFF_EVENT | CTRL_SHUTDOWN_EVENT => {
            // SAFETY: the Windows console control handler runs on a thread owned
            // by the OS, in a context where many locks (crossterm internals,
            // notification state, panic hook) are NOT safe to take — if the
            // main thread is currently holding one of them and panics, this
            // handler can deadlock or re-panic. We do the minimum: flip an
            // atomic flag. The normal shutdown path observes the flag and runs
            // all cleanup (raw mode, alt screen, toast clear) on its own
            // thread. (Fix for I12 / B5 panic-during-panic.)
            set_app_shutting_down(true);
            FALSE
        }
        _ => FALSE,
    }
}

#[cfg(feature = "widgets")]
mod imp {
    use std::io;

    use ratatui::{Terminal, backend::TermwizBackend};
    use crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetSize, disable_raw_mode, enable_raw_mode},
        event::{EnableMouseCapture, DisableMouseCapture},
    };

    use crate::apps::panic::set_panic_hook;
    use crate::apps::window::{BorderlessConsole, ConsoleTitleGuard, SingleInstanceGuard, center_console_window};

    /// Configuration for `bootstrap::init`.
    #[derive(Debug, Clone)]
    pub struct Config {
        /// Window title (used for the console tab/title and the `ConsoleTitleGuard`).
        pub title: &'static str,
        /// Whether to enforce a 100x35 minimum via `SetSize`.
        pub size: (u16, u16),
        /// If true, install a `SingleInstanceGuard` and exit on conflict.
        pub enforce_single_instance: bool,
        /// If true, enable the borderless console window and skip centering.
        pub borderless: bool,
        /// Whether to install the `set_panic_hook` (recommended).
        pub install_panic_hook: bool,
    }

    impl Config {
        pub fn new(title: &'static str) -> Self {
            Self {
                title,
                size: (100, 35),
                enforce_single_instance: true,
                borderless: false,
                install_panic_hook: true,
            }
        }
    }

    /// Automatically disables raw mode and restores screen settings on drop.
    pub struct ConsoleGuard {
        active: bool,
    }

    impl ConsoleGuard {
        pub fn new() -> Self {
            Self { active: true }
        }
        pub fn deactivate(&mut self) {
            self.active = false;
        }
    }

    impl Default for ConsoleGuard {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Drop for ConsoleGuard {
        fn drop(&mut self) {
            if self.active {
                let _ = disable_raw_mode();
                let _ = execute!(
                    io::stdout(),
                    LeaveAlternateScreen,
                    DisableMouseCapture
                );
            }
        }
    }

    /// All Drop guards returned by `init` so the caller can keep them alive.
    pub struct Guards {
        /// Set if `enforce_single_instance` is true.
        pub _instance_guard: Option<SingleInstanceGuard>,
        /// Always set while running.
        pub _title_guard: ConsoleTitleGuard,
        /// Set if `borderless` is true.
        pub _borderless: Option<BorderlessConsole>,
        /// Restores terminal configuration automatically on drop.
        pub _console_guard: ConsoleGuard,
    }

    /// Enable raw mode, alt screen, mouse capture, sizing, optional single-instance & borderless.
    /// Returns the Terminal + Drop guards. The caller should hold onto `guards` until shutdown.
    pub fn init(
        config: Config,
    ) -> io::Result<(Terminal<TermwizBackend>, Guards)> {
        super::set_app_shutting_down(false);

        #[cfg(target_os = "windows")]
        unsafe {
            let _ = windows_sys::Win32::System::Console::SetConsoleCtrlHandler(
                Some(super::ctrl_handler),
                windows_sys::Win32::Foundation::TRUE,
            );
        }

        if config.install_panic_hook {
            set_panic_hook();
        }

        let _instance_guard = if config.enforce_single_instance {
            Some(SingleInstanceGuard::try_new_or_exit(config.title))
        } else {
            None
        };

        let _title_guard = ConsoleTitleGuard::new(config.title);

        enable_raw_mode()?;
        let mut stdout = io::stdout();
        let _ = execute!(stdout, SetSize(config.size.0, config.size.1));
        if let Err(e) = execute!(stdout, EnterAlternateScreen, EnableMouseCapture) {
            let _ = disable_raw_mode();
            return Err(e);
        }

        let _borderless = if config.borderless {
            Some(BorderlessConsole::enable())
        } else {
            None
        };
        // Allow console size/style changes to propagate to the buffer
        std::thread::sleep(std::time::Duration::from_millis(50));

        if _borderless.is_none() {
            center_console_window();
        }

        let backend = TermwizBackend::new().map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        let _console_guard = ConsoleGuard::new();

        Ok((
            terminal,
            Guards {
                _instance_guard,
                _title_guard,
                _borderless,
                _console_guard,
            },
        ))
    }

    /// Restore raw-mode terminal state. Call this at the end of `main` (or in a Drop).
    pub fn shutdown(
        _terminal: &mut Terminal<TermwizBackend>,
    ) -> io::Result<()> {
        super::set_app_shutting_down(true);

        #[cfg(feature = "notification")]
        crate::apps::notification::clear_my_toast_notifications();

        let _ = disable_raw_mode();
        let _ = execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        Ok(())
    }
}

#[cfg(feature = "widgets")]
pub use imp::*;
