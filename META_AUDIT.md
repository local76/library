# library Consolidation Audit (post-restructure)

**Date**: 2026-06-10
**Scope**: `C:/Users/jeryd/Synology/Home/Projects/local76/library` and
the 15 consumer crates (5 apps + 10 screensaver shims).

## Status

The 4-layer taxonomy (interface / lifecycle / platform / role) has
been collapsed into a flat folder tree. The 4-layer module files
(`interface.rs`, `lifecycle.rs`, `platform.rs`, `role.rs`) have
been deleted. The `lib.rs` no longer carries the 4-layer `pub mod`
lines; the `Screensaver` trait is now re-exported from
`ui::screensaver_renderer`.

The 10 screensaver scenes have been moved out of `library` into
their own sibling repos. The chrome module (`apps::chrome`) is in
place. The 5 apps and 10 screensaver-* shims compile against the
new tree.

## What changed in 2026.6.10.1

- **4-layer shim files removed**: `interface.rs`, `lifecycle.rs`,
  `platform.rs`, `role.rs` deleted from `library/src/`.
- **`lib.rs` slimmed**: 110 lines → 45 lines. The 4-layer
  `pub mod` lines are gone. The `Screensaver` / `ScreensaverRenderer`
  re-exports point at `ui::screensaver_renderer` (the canonical
  path).
- **Cargo features re-shaped**: dropped the 4-layer prefix names
  (`interface-app`, `lifecycle-foreground`, `platform-native`,
  `role-system`, `role-application`). The new feature set is
  granular: `widgets`, `sys-info`, `window`, `service`, `event-log`,
  `notification`, `clipboard`, `reg`, `winget`, `gpu`, `gui`,
  `effects`, plus composites `chrome` and `screensaver-runtime`.
  The 4-layer names that downstream consumers used to depend on
  (`scenes`, `effects`) are kept as no-ops for one cycle.
- **`tui_bootstrap` module renamed to `bootstrap`**: the
  `TuiBootstrapConfig` struct is now `Config`. The `bootstrap_tui`
  function is now `init`. The `shutdown_tui` function is now
  `shutdown`. The `set_tui_panic_hook` function is now
  `set_panic_hook`. The `TuiEffect` trait in `ui::effects` is now
  `Effect`.
- **Doc sweep**: `local76/README.md`, `local76/index.md`,
  `library/README.md`, `library/ARCHITECTURE.md`,
  `library/CONTRIBUTING.md`, `library/docs/DESIGN_SYSTEM.md`,
  `library/docs/EMBEDDED_DOCS.md`, `library/docs/VISUAL_STANDARDS.md`,
  and `library/docs/ICON_TROUBLESHOOTING.md` rewritten. The
  3.x → 4.x path migration table and the "TUI" qualifier
  references are gone.
- **Daily release automation added**: `toolkit/scripts/daily-release.ps1`
  (and 3 helper scripts) runs at 04:00 PT, gated on new commits.
  See `toolkit/README.md` for the new flow.

## What still needs work

These are tracked in `local76/README.md` §Roadmap:

- Drop the `scenes` and `effects` no-op features after one cycle.
- `trance`: real Windows screensaver control panel.
- `screensaver-*`: package the 10 binaries as a single
  `apt install local76-screensavers` deb.
- Library: split `apps::window` into per-platform modules.
- Library: `wgpu` renderer as an alternative to ratatui.
- Library: add eBPF integration.

## Consumers (15 crates)

- 5 apps: `app-helm`, `app-pulse`, `app-scout`, `app-trance`,
  `app-ignite`
- 10 screensaver shims: `screensaver-beams`, `screensaver-bounce`,
  `screensaver-bursts`, `screensaver-chaos`, `screensaver-cosmos`,
  `screensaver-disco`, `screensaver-flame`, `screensaver-glyphs`,
  `screensaver-gnats`, `screensaver-storm`

All 15 depend on `library` via
`[patch."https://github.com/local76/library.git"] library = { path = "../library" }`.
