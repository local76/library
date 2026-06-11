# Embedding markdown docs in your app (F1–F7 in-app help)

`library::ui::markdown::embedded_docs!` is a compile-time macro that
bakes a set of markdown files (your README, LICENSE, CONTRIBUTING,
etc.) directly into your binary, so your app can show the help text
without reading the filesystem at runtime (which would break in a
single-file `.scr` Windows screensaver install).

## The macro

`library::ui::markdown::embedded_docs!(folder, [file1, file2, ...])`
— declared as `#[macro_export]`, available from any consumer as
`library::embedded_docs!(...)`.

Internally it is a thin wrapper over `include_str!` that returns a
`HashMap<&'static str, &'static str>` mapping file names to contents.
The compiler reads the files at build time; nothing reads the
filesystem at runtime.

## Canonical example

```rust
use std::collections::HashMap;
use std::sync::LazyLock;

/// Standardized embedded documents map.
pub static EMBEDDED_DOCS: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        library::embedded_docs!("..", [
            "README.md",
            "SUPPORT.md",
            "LICENSE.md",
            "COPYRIGHT.md",
            "PRIVACY.md",
            "SECURITY.md",
            "CONTRIBUTING.md",
        ])
    });
```

`".."` is relative to the manifest dir of the consuming crate. For
example, for `helm` at `local76/helm/src/`, `".."` would be
`local76/helm/`. Adjust to your layout.

## Wiring it into the chrome

The cross-app `chrome` module provides F1–F7 routing via
`open_embedded_markdown`. The standard pattern:

```rust
use crossterm::event::{KeyCode, KeyEvent};
use library::apps::chrome::{
    DOC_FILES, is_doc_f_key, open_embedded_markdown,
};

fn handle_key(app: &mut App, key: KeyEvent) {
    if is_doc_f_key(key.code) {
        if let Some(text) = open_embedded_markdown(key.code) {
            app.viewer = Some(MarkdownViewerState::new(text));
        }
    }
    // ... app-specific key handling
}
```

The chrome module handles the F1–F7 → filename mapping and the
`include_str!` lookup. The actual `include_str!` of the 7 docs lives
in each app crate because the files live in each app's repo root.

## Notes

- The macro uses `include_str!` under the hood, so the path is
  resolved at compile time. A typo in the file name = a compile error,
  not a runtime fallback. That's intentional: you want to catch a
  missing file at `cargo build` time, not at first launch.
- The macro is intentionally simple — no glob, no directory walk.
  Explicit file lists are preferred so the compiler knows exactly
  what's in the binary.
- The macro lives in `library::ui::markdown` (not in any deprecated
  path). Use the canonical path.

## Why this exists

Prior to library consolidating this pattern, every app in the suite
had its own F1–F7 help panel implementation, with different
in-memory string tables and different render paths. The cross-app
`chrome` module moved all of them to a single
`library::apps::chrome::open_embedded_markdown` helper + a single
`MarkdownViewer` renderer.

## See also

- `library::ui::markdown::MarkdownViewer` (the renderer that consumes
  the `&'static str` content)
- `library::ui::markdown::parse_markdown_to_lines` (lower-level; used
  internally by `MarkdownViewer`)
- `library::apps::chrome::DOC_FILES` (the 7-doc filename list)
- Each app's `event_handler.rs` for the F1–F7 binding pattern
