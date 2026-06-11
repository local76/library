# library Design System

> The single source of truth for the visual identity of every app in
> the local76 ecosystem.

The design system in `library::ui::*` is a flat set of widgets that
every local76 app (helm, pulse, scout, trance, ignite) and every
screensaver shim binary (beams, bounce, …) uses. This document
describes the design system from an app author's point of view. For
the underlying architectural rationale, see
[`ARCHITECTURE.md`](../ARCHITECTURE.md).

---

## Widget surface

There is no single `use library::ui::*;` prelude. Import the widgets
you need from `library::ui::<widget>`. The full set:

```rust
use library::ui::theme::{ThemeColors, get_theme, accent_color_from_hex, current_theme};
use library::ui::layout::centered_rect;
use library::ui::layout_guard::{is_too_small, render_too_small_warning, MIN_TERMINAL_WIDTH, MIN_TERMINAL_HEIGHT};
use library::ui::text::{wrap_text, align_line, char_width, visible_len, visible_split, TextAlignment};
use library::ui::status_bar::StatusBar;
use library::ui::toast::{ToastBox, ToastKind};
use library::ui::markdown::{parse_markdown_to_lines, draw_markdown_modal, embedded_docs};
use library::ui::markdown_viewer::MarkdownViewerState;
use library::ui::title_banner::{draw_title_banner, ButtonRect};
use library::ui::mouse_selection::MouseSelection;
use library::ui::effect_preview::draw_effect_preview;
use library::ui::screensaver_renderer::{Screensaver, ScreensaverRenderer, ScreensaverState};
use library::ui::effects::{FallingGlyphs, FlowingParticles, /* ... 10 more ... */ Effect};
```

The 12 in-app effects (`FallingGlyphs`, `FlowingParticles`, etc.) are
gated on the `effects` Cargo feature. Everything else in `ui` is
gated on `widgets`.

---

## Color story

The color story is centered on the **system accent** + a derived
`ScreenPalette`. Every local76 app pulls the same canonical palette
and gets a visually consistent identity out of the box.

```rust
use library::core::screen_palette::{query_current_palette, ScreenPalette};

let palette: ScreenPalette = query_current_palette();

let bg      = palette.bg;      // (0,0,0) in dark mode
let fg      = palette.fg;      // (248,248,242) in dark mode
let accent  = palette.accent;  // System DWM accent on Windows
let dim     = palette.dim;     // 35% of accent
let hot     = palette.hot;     // accent hue +30°, lightness 0.55
let cool    = palette.cool;    // accent hue -120°, lightness 0.45
let mid     = palette.mid;     // (128,128,128) neutral chrome
let peak    = palette.peak;    // (255,255,255) hot peaks
```

`ScreenPalette` uses plain RGB tuples, so it works in both ratatui
`Color::Rgb` and GDI pixel renderers. The same palette is used by
every app's chrome and every screensaver scene.

### Typed palette (for the 12 in-app effects)

For the 12 in-app effects, `library::ui::effects::dimensions::Palette`
exposes the same color story in a ratatui-friendly enum:

```rust
use library::ui::effects::dimensions::Palette;

let p = Palette::Accent;       // system accent
let p = Palette::AccentDim;    // 35%-dimmed accent
let p = Palette::AccentHot;    // +30° hue
let p = Palette::AccentCool;   // -120° hue
let p = Palette::Heat;         // cold-to-hot ramp
```

---

## Onboarding: `helm`

`helm` is the reference consumer. Its `main.rs` and `ui/mod.rs` show
the canonical pattern:

```rust
use library::apps::bootstrap::{Config, init, shutdown, is_app_shutting_down};
use library::apps::panic::set_panic_hook;
use library::apps::chrome::{is_quit_key, is_help_toggle_key, scroll_for_key};
use library::apps::file_log;

fn main() -> io::Result<()> {
    file_log::set_log_app_name("app/helm");
    set_panic_hook();
    let config = Config::new("helm");
    let (mut terminal, _guards) = init(config)?;
    let theme = library::ui::theme::get_theme(
        library::toolkit::sys_info::query_dark_mode(),
        library::toolkit::sys_info::query_accent_color(),
    );
    // ...
    loop {
        terminal.draw(|f| ui::draw_ui(f, &mut app, &theme))?;
        if !event::poll(Duration::from_millis(100))? { continue; }
        let key = event::read()?;
        if is_quit_key(key.code, key.modifiers) { break; }
        if is_help_toggle_key(key.code) { app.show_help = !app.show_help; }
        // ... app-specific handling
        if is_app_shutting_down() { break; }
    }
    shutdown(&mut terminal)?;
    Ok(())
}
```

`Config` (from `library::apps::bootstrap`) is the configuration for
`init()`. `init()` returns a `(Terminal, Guards)` pair — hold onto
`Guards` until shutdown so the Drop impls restore terminal state.

`app.show_help` triggers a `MarkdownViewerState` (for F1–F7 help
docs). The `library::apps::chrome::open_embedded_markdown` helper
maps a key code to a doc name and returns the embedded content.

---

## Onboarding: a screensaver shim binary

The 10 `screensaver-<scene>` shim binaries share a single template:

```rust
// screensaver-beams/src/main.rs
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod beams;

fn main() {
    let effect = beams::Beams::new();
    library::screensaver_runner::run_main(effect, "beams");
}
```

The scene module (`screensaver-beams/src/beams/`) implements
`library::core::screensaver::Screensaver`. `run_main` parses CLI
args, dispatches to the platform-specific render loop, and exits when
the user closes the preview window.

---

## Effect naming: Verb × Noun × Style × Palette

The 12 in-app effects follow a 4-dimension naming system. The type
name is always `Verb` + `Noun` (PascalCase); the file name is the
snake_case of the same; the display name is `"Verb Noun"`.

| Dimension | Values | Purpose |
|---|---|---|
| **Verb** | `Falling`, `Rising`, `Flowing`, `Pulled`, `Pulsing` | Motion model |
| **Noun** | `Glyphs`, `Particles`, `Droplets`, `Comets`, `Blocks`, `Waves` | Visual unit |
| **Style** | `Solid`, `Trailing`, `Flared` | Render treatment |
| **Palette** | `Monochrome(r,g,b)`, `Accent`, `Heat`, `AccentDim`, `AccentHot`, `AccentCool` | Color source |

- **Style** lives in `library::ui::effects::dimensions::Style` and is
  exposed as a field on every effect.
- **Palette** lives in `library::ui::effects::dimensions::Palette` and
  is exposed as a field on every effect.
- All effects expose `with_style(Style)` and `with_palette(Palette)`
  builder methods.

### Current catalog (12 effects)

| Type | File | Default Style | Default Palette |
|---|---|---|---|
| `FallingGlyphs` | `falling_glyphs.rs` | `Trailing` | `Monochrome(Green)` |
| `FlowingParticles` | `flowing_particles.rs` | `Solid` | `Monochrome(White)` |
| `PulledParticles` | `pulled_particles.rs` | `Solid` | `Monochrome(Blue)` |
| `FallingDroplets` | `falling_droplets.rs` | `Solid` | `Monochrome(Blue)` |
| `RisingFlames` | `rising_flames.rs` | `Solid` | `Heat` |
| `FallingComets` | `falling_comets.rs` | `Trailing` | `Monochrome(White)` |
| `PulsingGlyphs` | `pulsing_glyphs.rs` | `Solid` | `Accent` |
| `PulsingWaves` | `pulsing_waves.rs` | `Solid` | `Heat` |
| `FlowingBlocks` | `flowing_blocks.rs` | `Solid` | `Accent` |
| `PulledBlocks` | `pulled_blocks.rs` | `Solid` | `Monochrome(Blue)` |
| `RisingGlyphs` | `rising_glyphs.rs` | `Solid` | `Heat` |
| `PulsingParticles` | `pulsing_particles.rs` | `Solid` | `Accent` |

---

## Testing

The design system ships with `tests/design_facade.rs` in `library` —
a comprehensive set of tests that exercise the public surface
end-to-end (theme, accent, status bar, toast, layout guard, markdown
viewer, text wrap, render-logo-block, all 12 effects, etc.). Add
similar tests in each app to lock in the contract.
