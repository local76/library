# local76 Ecosystem: Visual Branding & Icon Standards

This document establishes the design principles, visual guidelines, and technical specifications for icons in the `local76` suite of applications (`rFetch`, `rIdle`, `rMonitor`, `rWifi`, `rStartup`, `rTemplate`). 

By defining a unified visual identity, we ensure that every utility feels like a first-class, cohesive member of the same family while remaining individually recognizable.

---

## 1. Design Philosophy & Visual Language

The `local76` applications are local-first, lightweight developer utilities and system monitors. The visual language must reflect this purpose: **precise, high-tech, and minimal**, without feeling corporate or bland.

We define a signature style: **Stroke Monogram Duotone** (inspired by the Hugeicons stroke-rounded duotone layout).

```
┌─────────────────────────────────────────┐
│               SQUIRCLE CONTAINER        │
│  ┌───────────────────────────────────┐  │
│  │     1.5px STROKE OUTLINE BORDER   │  │
│  │  ┌─────────────────────────────┐  │  │
│  │  │   2-LETTER MONOGRAM (e.g. rF)│  │  │
│  │  │   WITH 1.5px STROKE OUTLINE  │  │  │
│  │  └─────────────────────────────┘  │  │
│  │     SOFT ACCENT COLOR FILL (15%)  │  │
│  └───────────────────────────────────┘  │
│               DARK OBSIDIAN BASE        │
└─────────────────────────────────────────┘
```

### Key Design Pillars:
* **Dark Obsidian Base**: Icons use a dark, charcoal-to-deep-blue gradient (`#0B0F19` to `#161C2C`) as the foundation. This mimics the terminal background environment.
* **Stroke-Rounded Monogram**: The foreground utilizes a clean, modern sans-serif outline monogram (representing the utility prefix, e.g. `rF` for `rFetch`) using a strict **1.5px stroke width on a 24px grid**. Line caps and corner joins are rounded, with no solid fills for the glyph itself.
* **Accent Border & Duotone Fill**: The outer container's border is also drawn with a matching 1.5px stroke, and the container's interior features a semi-transparent filler color (10% to 20% opacity of the accent color), creating a beautiful duotone glassmorphism look.
* **App-Specific Accent Colors**: Each icon has a primary neon glow and duotone fill matching its terminal accent:
  * **Cyan (`#00F5FF`)**: Core, template, and developer tools.
  * **Amber/Gold (`#FFB900`)**: State, warnings, and hardware monitors.
  * **Green (`#7FBA00`)**: Networks, execution state, and active monitors.

---

## 2. Visual Monograms for local76 utilities

To make the icon set cohesive, all icons utilize a foreground 1.5px stroke outline monogram on a dark squircle container with corresponding accent and filler colors:

| Application | Accent Color | Monogram | Symbolic Meaning |
| :--- | :--- | :--- | :--- |
| **`rFetch`** | **Neon Cyan** | **`rF`** | System polling, hardware query, static gathering. |
| **`rMonitor`** | **Neon Amber / Gold** | **`rM`** | Live metrics, resource utilization, telemetry. |
| **`rWifi`** | **Neon Green** | **`rW`** | Signal telemetry, connection state, wireless data. |
| **`rIdle`** | **Violet / Dark Blue** | **`rI`** | Standby state, screensaver, screen locking. |
| **`rStartup`** | **Neon Orange / Red** | **`rS`** | Service launching, startup boot, initialization. |
| **`rTemplate`** | **Neon Cyan / Gold** | **`rT`** | Scaffold creation, skeleton template, boilerplate. |

---

## 3. Technical Specifications

To ensure the icons render crisp and sharp at all operating system scales (from desktop shortcuts down to taskbar icons), they must adhere to the following technical details:

### A. Grid & Stroke Scaling
* **Detail Grid**: Vector assets must be designed on a baseline **`24x24` grid** using a **`1.5px` stroke width**.
* **Vector Scaling**: When scaling the master icon canvas (e.g. `256x256`), the stroke weight must scale proportionally:
  * At `24x24` canvas: `1.5px` stroke
  * At `256x256` canvas: `16px` stroke
  * At `512x512` canvas: `32px` stroke
* **Line Caps & Joins**: All strokes must use `stroke-linecap: round` and `stroke-linejoin: round` to maintain smooth, soft endpoints.

### B. File Formats & Resolutions
* **`app_icon.png`**: High-resolution `512x512` or `256x256` 32-bit RGBA PNG with alpha transparency.
* **`app.ico`**: Multi-resolution Windows ICO container containing `256x256`, `48x48`, `32x32`, and `16x16` sizes.
* **Padding**: Leave a **15% padding margin** around the container bounds to allow glowing neon offsets and glares to fade out naturally without edge clipping.

---

## 4. Integration

Add references to these visual guidelines in project templates and packaging configurations to maintain brand consistency.
