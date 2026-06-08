# local76 Ecosystem: Visual Branding & Icon Standards

This document establishes the design principles, visual guidelines, and technical specifications for icons in the `local76` suite of applications (`rFetch`, `rIdle`, `rMonitor`, `rWifi`, `rStartup`, `rTemplate`). 

By defining a unified visual identity, we ensure that every utility feels like a first-class, cohesive member of the same family while remaining individually recognizable.

---

## 1. Design Philosophy & Visual Language

The `local76` applications are local-first, lightweight developer utilities and system monitors. The visual language must reflect this purpose: **precise, high-tech, and minimal**, without feeling corporate or bland.

We define a signature style: **Wireframe Glassmorphism**.

```
┌─────────────────────────────────────────┐
│               SQUIRCLE CONTAINER        │
│  ┌───────────────────────────────────┐  │
│  │   GLOSSY / REFLECTIVE TOP GLASS   │  │
│  │  ┌─────────────────────────────┐  │  │
│  │  │   CYAN/GOLD NEON WIREFRAME  │  │  │
│  │  │        VISUAL METAPHOR      │  │  │
│  │  └─────────────────────────────┘  │  │
│  │   CHAMFERED GLOWING BORDER        │  │
│  └───────────────────────────────────┘  │
│               DARK OBSIDIAN BASE        │
└─────────────────────────────────────────┘
```

### Key Design Pillars:
* **Dark Obsidian Base**: Icons use a dark, charcoal-to-deep-blue gradient (`#0B0F19` to `#161C2C`) as the foundation. This mimics the terminal background environment.
* **Reflective Cover (Glassmorphism)**: A subtle diagonal glare across the top half of the icon, giving it a premium, three-dimensional physical glass appearance.
* **Glowing Neon Wireframe (Blueprint Aesthetic)**: Visual symbols are composed of thin, glowing vector lines and connection nodes, symbolizing system configurations, data pipelines, and hardware internals.
* **App-Specific Accent Colors**: Each icon has a primary neon glow matching its terminal accent:
  * **Cyan (`#00F5FF`)**: Core, template, and developer tools.
  * **Amber/Gold (`#FFB900`)**: State, warnings, and hardware monitors.
  * **Green (`#7FBA00`)**: Networks, execution state, and active monitors.

---

## 2. Visual Metaphors for local76 utilities

To make the icon set cohesive, all icons utilize the **"Blueprint Grid"** in the background, with a distinct glowing wireframe metaphor in the foreground:

| Application | Accent Color | Visual Metaphor | Symbolic Meaning |
| :--- | :--- | :--- | :--- |
| **`rFetch`** | **Neon Cyan** | A node-based network grid overlaid with a glowing hardware query symbol (e.g., a system chip or target node). | System polling, hardware query, static gathering. |
| **`rMonitor`** | **Neon Amber / Gold** | A grid overlay with a rising activity chart or a pulse/heartbeat waveform on a coordinate axis. | Live metrics, resource utilization, telemetry. |
| **`rWifi`** | **Neon Green** | Radial concentric wireframe waves radiating outwards from a central vector point. | Signal telemetry, connection state, wireless data. |
| **`rIdle`** | **Violet / Dark Blue** | A crescent moon constructed from connected geometric nodes, resting on a grid layout. | Standby state, screensaver, screen locking. |
| **`rStartup`** | **Neon Orange / Red** | An ascending rocket shape or a glowing toggle switch constructed from blueprint vector lines. | Service launching, startup boot, initialization. |
| **`rTemplate`** | **Neon Cyan / Gold** | A complete, empty blueprint grid with an structural serif letter **"I"** representing structural scaffolding. | Scaffold creation, skeleton template, boilerplate. |

---

## 3. Technical Specifications

To ensure the icons render crisp and sharp at all operating system scales (from desktop shortcuts down to taskbar icons), they must adhere to the following technical details:

### A. File Formats & Resolutions
* **`app_icon.png`**: High-resolution `512x512` or `256x256` 32-bit RGBA PNG with alpha transparency.
* **`app.ico`**: Multi-resolution Windows ICO container containing the following sizes:
  * `256x256` (high-res display)
  * `48x48` (explorer list/grid)
  * `32x32` (desktop shortcut)
  * `16x16` (taskbar tray/details)

### B. Layout & Padding
* **Container Bounds**: The outer squircle (rounded square) container should occupy approximately **85%** of the canvas width and height.
* **Padding**: Leave a **15% padding margin** around the container to allow the outer glow, soft drop-shadow, and light glare effects to fade out naturally without clipping at the edges.
* **Line Widths**: Wireframe vector lines should be designed with varying weights:
  * Major structure lines: `3px` (at `256x256` canvas)
  * Minor layout/grid lines: `1px` (at `256x256` canvas)
  * Glowing nodes (circles): `6px` diameter (at `256x256` canvas)

---

## 4. Integration

Add references to these visual guidelines in project templates and packaging configurations to maintain brand consistency.
