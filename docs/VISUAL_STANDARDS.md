# local76 Ecosystem — Visual Branding & Icon Standards

> Design principles, visual guidelines, and technical specifications for icons in the `local76` suite of applications (`helm`, `pulse`, `scout`, `trance`, `ignite`) and the 10 scenes in `screensavers`.

By defining a unified visual identity, we ensure that every utility feels like a first-class, cohesive member of the same family while remaining individually recognizable.

---

## 1. Design philosophy

The `local76` applications are local-first, lightweight developer utilities and system monitors. The visual language must reflect this purpose: **precise, high-tech, and minimal**, without feeling corporate or bland.

We define a signature style: **High-Contrast Monogram** (inspired by minimalist, clean, highly readable brand identities).

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

- **Solid white squircle base.** Icons use a solid white rounded square container as their base, providing maximum contrast and readability at all sizes.
- **Bold black monogram.** The foreground uses a clean, modern, bold black sans-serif monogram (representing the lowercase app name) centered perfectly.
- **Thin black outline.** A crisp, thin black border details the perimeter of the squircle to maintain structure.
- **Ecosystem uniformity.** Rather than using varied glowing accent colors, the core application icons are uniformly styled in high-contrast black-and-white. This establishes a clean, unified brand that looks professional on any wallpaper or taskbar.

---

## 2. Visual monograms for local76 utilities

To make the icon set cohesive, all icons use a bold black monogram on a solid white squircle container:

| Application | Monogram | Meaning |
| :--- | :--- | :--- |
| **`helm`** | **`hl`** | System polling, hardware query, static gathering. |
| **`pulse`** | **`pl`** | Live metrics, resource utilization, telemetry. |
| **`scout`** | **`sc`** | Signal telemetry, connection state, wireless data. |
| **`trance`** | **`tr`** | Standby state, screensaver, screen locking. |
| **`ignite`** | **`ig`** | Service launching, startup boot, initialization. |
| **`screensavers`** (10 scenes) | scene name lowercase (`beams`, `flame`, `cosmos`, ...) | Per-scene effect monogram. |

---

## 3. Technical specifications

To ensure the icons render crisp and sharp at all operating system scales (from desktop shortcuts down to taskbar icons), they must adhere to the following technical details.

### A. Grid & stroke scaling

- **Detail grid**. Vector assets must be designed on a baseline **`24x24` grid** using a **`1.5px` stroke width**.
- **Vector scaling**. When scaling the master icon canvas (e.g. `256x256`), the stroke weight must scale proportionally:
  - At `24x24` canvas: `1.5px` stroke
  - At `256x256` canvas: `16px` stroke
  - At `512x512` canvas: `32px` stroke
- **Line caps & joins**. All strokes must use `stroke-linecap: round` and `stroke-linejoin: round` to maintain smooth, soft endpoints.

### B. File formats & resolutions

- **`app_icon.png`**. High-resolution `512x512` or `256x256` 32-bit RGBA PNG with alpha transparency.
- **`app.ico`**. Multi-resolution Windows ICO container containing exactly **`256x256` (PNG-compressed)**, **`48x48`**, **`32x32`**, and **`16x16`** sizes at 32-bit RGBA depth. Having all four sizes prevents blurry scaling and ensures Windows Explorer does not fall back to standard console icons in list, details, or taskbar views.
- **Padding**. Leave a **15% padding margin** around the container bounds to allow glowing neon offsets and glares to fade out naturally without edge clipping.

### C. Windows Explorer resource metadata

To ensure the applications look polished in Windows Explorer (e.g. when right-clicking the binary and viewing **Properties → Details**), every utility must compile the following PE metadata into its resources using a build script (`build.rs`) and the **`embed-resource` 2.x** crate:

- **File Description**. A clean, descriptive name of the utility (e.g., `helm - System Info Utility`).
- **Product Name**. Grouped under the suite name: `local76 Suite`.
- **Company Name**. Set as `local76`.
- **Legal Copyright**. Set as `Copyright © 2026 local76`.
- **Version Information**. Automatically synchronizes the file and product versions with the crate's `Cargo.toml` version.

> **Migration note (2026-06-09):** the historical `winres 0.1` template has been
> deprecated. The `winres 0.1.x` parser mangles PNG-compressed multi-size ICOs,
> which causes Windows Explorer to fall back to a generic console icon. The new
> template below uses Microsoft's `embed-resource` 2.x crate, which correctly
> preserves all four ICO sizes. The brand defaults
> (`local76 Suite` / `local76` / `Copyright © 2026 local76`) are exposed via
> `library::build_resources::{DEFAULT_PRODUCT_NAME, DEFAULT_COMPANY_NAME, DEFAULT_LEGAL_COPYRIGHT}`
> so they live in one place. See `ICON_TROUBLESHOOTING.md` for the ICONDIR
> verification recipe used to catch regressions.

#### Standard `build.rs` template

In the consuming crate's `Cargo.toml`:

```toml
[target.'cfg(windows)'.build-dependencies]
embed-resource = "2"
```

In its `build.rs`:

```rust
use library::build_resources::{
    DEFAULT_COMPANY_NAME, DEFAULT_LEGAL_COPYRIGHT, DEFAULT_PRODUCT_NAME, prepare_icon,
};

fn main() {
    if let Some((icon_path, meta)) = prepare_icon("assets/brand/app.ico") {
        let mut rc = embed_resource::new();
        rc.set_icon(&icon_path);
        rc.set("FileDescription", &meta.file_description);
        rc.set("ProductName",     DEFAULT_PRODUCT_NAME);
        rc.set("CompanyName",     DEFAULT_COMPANY_NAME);
        rc.set("LegalCopyright",  DEFAULT_LEGAL_COPYRIGHT);
        rc.compile().expect("failed to compile winres resource");
    }
}
```

> The exact `embed_resource::new()` / `set_icon` / `set` / `compile` calls
> match the 2.x API. If a future 3.x release renames the entry point, only
> the call site changes — `library::build_resources` does not need an update.

---

## 4. Integration

Add references to these visual guidelines in project templates and packaging configurations to maintain brand consistency. Every new tool that joins the local76 ecosystem adopts the same monogram style, the same `build.rs` template, and the same `.desktop` / `.ico` asset layout.
