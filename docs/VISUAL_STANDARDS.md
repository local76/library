# local76 Ecosystem — Visual Branding & Icon Standards

> Design principles, visual guidelines, and technical specifications
> for icons in the `local76` suite of applications (`helm`, `pulse`,
> `scout`, `trance`, `ignite`) and the 10 screensaver scenes
> (`beams`, `bounce`, `bursts`, `chaos`, `cosmos`, `disco`, `flame`,
> `glyphs`, `gnats`, `storm`).

By defining a unified visual identity, we ensure that every utility
feels like a first-class, cohesive member of the same family while
remaining individually recognizable.

---

## 1. Design philosophy

The `local76` applications are local-first, lightweight developer
utilities and system monitors. The visual language must reflect this
purpose: **precise, high-tech, and minimal**, without feeling
corporate or bland.

We define a signature style: **High-Contrast Monogram** (inspired by
minimalist, clean, highly readable brand identities).

```
┌─────────────────────────────────────────┐
│               SQUIRCLE CONTAINER        │
│  ┌───────────────────────────────────┐  │
│  │     THIN BLACK OUTLINE BORDER     │  │
│  │  ┌─────────────────────────────┐  │  │
│  │  │   BOLD BLACK MONOGRAM       │  │  │
│  │  │   (e.g., hl)                │  │  │
│  │  └─────────────────────────────┘  │  │
│  │     SOLID WHITE BACKGROUND        │  │
│  └───────────────────────────────────┘  │
│               SOLID BLACK BASE          │
└─────────────────────────────────────────┘
```

### Design pillars

- **Solid white squircle base.** Icons use a solid white rounded
  square container as their base, providing maximum contrast and
  readability at all sizes.
- **Bold black monogram.** The foreground uses a clean, modern, bold
  black sans-serif monogram (representing the lowercase app name)
  centered perfectly.
- **Thin black outline.** A crisp, thin black border details the
  perimeter of the squircle to maintain structure.
- **Ecosystem uniformity.** Rather than using varied glowing accent
  colors, the core application icons are uniformly styled in
  high-contrast black-and-white. This establishes a clean, unified
  brand that looks professional on any wallpaper or taskbar.

---

## 2. Visual monograms for local76 utilities

To make the icon set cohesive, all icons use a bold black monogram
on a solid white squircle container:

| Application | Monogram | Meaning |
| :--- | :--- | :--- |
| **`helm`** | **`hl`** | System polling, hardware query, static gathering. |
| **`pulse`** | **`pl`** | Live metrics, resource utilization, telemetry. |
| **`scout`** | **`sc`** | Signal telemetry, connection state, wireless data. |
| **`trance`** | **`tr`** | Standby state, screensaver, screen locking. |
| **`ignite`** | **`ig`** | Service launching, startup boot, initialization. |
| **`screensavers`** (10 scenes) | full scene name lowercase (`beams`, `bounce`, …) | Per-scene effect monogram. |

---

## 3. Technical specifications

To ensure the icons render crisp and sharp at all operating system
scales (from desktop shortcuts down to taskbar icons), they must
adhere to the following technical details.

### A. Grid & stroke scaling

- **Detail grid**. Vector assets must be designed on a baseline
  **`24x24` grid** using a **`1.5px` stroke width**.
- **Vector scaling**. When scaling the master icon canvas (e.g.
  `256x256`), the stroke weight must scale proportionally:
  - At `24x24` canvas: `1.5px` stroke
  - At `256x256` canvas: `16px` stroke
  - At `512x512` canvas: `32px` stroke
- **Line caps & joins**. All strokes must use `stroke-linecap: round`
  and `stroke-linejoin: round` to maintain smooth, soft endpoints.

### B. File formats & resolutions

- **`app_icon.png`**. High-resolution `512x512` or `256x256` 32-bit
  RGBA PNG with alpha transparency.
- **`app.ico`**. Multi-resolution Windows ICO container containing
  exactly **`256x256` (PNG-compressed)**, **`48x48`**, **`32x32`**,
  and **`16x16`** sizes at 32-bit RGBA depth. Having all four sizes
  prevents blurry scaling and ensures Windows Explorer does not fall
  back to standard console icons in list, details, or taskbar views.
- **Padding**. Leave a **15% padding margin** around the container
  bounds to allow glowing neon offsets and glares to fade out
  naturally without edge clipping.

### C. Windows Explorer resource metadata

To ensure the applications look polished in Windows Explorer (e.g.
when right-clicking the binary and viewing **Properties → Details**),
every utility must compile the following PE metadata into its
resources via a build script (`build.rs`) that calls the library's
`core::build_resources::write_brand_rc` helper plus Microsoft's
`embed-resource` 2.x crate:

- **File Description**. A clean, descriptive name of the utility
  (e.g., `helm - System Info Utility`). Sourced from `CARGO_PKG_NAME`
  by default; override with a literal string in your `build.rs`.
- **Product Name**. Grouped under the suite name: `local76 Suite`.
  Exposed as `library::core::build_resources::DEFAULT_PRODUCT_NAME`.
- **Company Name**. Set as `local76`. Exposed as
  `library::core::build_resources::DEFAULT_COMPANY_NAME`.
- **Legal Copyright**. Set as `Copyright © 2026 local76`. Exposed
  as `library::core::build_resources::DEFAULT_LEGAL_COPYRIGHT`.
- **Version Information**. Auto-synchronized with the crate's
  `Cargo.toml` version by the `write_brand_rc` helper.

The Windows SDK `rc.exe 10.0+` ICONDIR-corruption bug is worked
around transparently by `library::core::rc_split::split_for_rc`,
which splits the multi-size ICO into 4 single-size files and
declares 4 separate ICON resources in the generated `.rc`. See
[ICON_TROUBLESHOOTING.md](./ICON_TROUBLESHOOTING.md) for the ICONDIR
verification recipe used to catch regressions.

#### Standard `build.rs` template

In the consuming crate's `Cargo.toml`:

```toml
[target.'cfg(windows)'.build-dependencies]
embed-resource = "2"
```

In its `build.rs` (canonical pattern; used by all 10
`screensaver-*` shim binaries):

```rust
use std::path::Path;
use library::core::build_resources::{
    write_brand_rc, DEFAULT_COMPANY_NAME, DEFAULT_LEGAL_COPYRIGHT, DEFAULT_PRODUCT_NAME,
};

fn main() {
    let ico = "assets/scene-<scene>.ico";
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") { return; }
    if !Path::new(ico).exists() { return; }

    let pkg = std::env::var("CARGO_PKG_NAME").unwrap_or_default();
    let rc = write_brand_rc(
        "build/windows_resource.rc",
        ico,
        &pkg,
        DEFAULT_PRODUCT_NAME,
        DEFAULT_COMPANY_NAME,
        DEFAULT_LEGAL_COPYRIGHT,
    );
    embed_resource::compile(&rc, embed_resource::NONE);
}
```

The `write_brand_rc` function returns the absolute path to the
generated `.rc` file. `embed_resource::compile(&rc, embed_resource::NONE)`
invokes the `embed-resource` 2.x build script under the hood, which
will select `windres` on `*-pc-windows-gnu` and `rc.exe` on
`*-pc-windows-msvc`. The ICONDIR-corruption workaround in
`split_for_rc` is applied transparently — you don't need to know
about it.

If your app's icon is at a different path (e.g.
`assets/brand/app.ico` instead of `assets/scene-<scene>.ico`), just
change the `ico` string in the `build.rs`. The library handles the
rest.

---

## 4. Integration

Add references to these visual guidelines in project templates and
packaging configurations to maintain brand consistency. Every new
tool that joins the local76 ecosystem adopts the same monogram style,
the same `build.rs` template, and the same `.desktop` / `.ico` asset
layout.
