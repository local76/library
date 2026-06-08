//! GUI (Graphical User Interface) components.
//!
//! **Taxonomy Classification**: Interface (GUI).
//!
//! Part of the Interface (Presentation Layer).
//! Includes both Native/OS GUIs and Custom/Game Engine continuous canvas UIs.

#[cfg(feature = "gui")]
pub mod gui;  // The moved gui.rs content (eframe/egui helpers etc.)
pub mod native;  // Native/OS GUI stubs (message boxes, etc.)

// The gui submodule provides the items (e.g. interface::gui::gui::...).
// Re-exports can be added here if a flatter interface::gui::AccentGauge is desired.