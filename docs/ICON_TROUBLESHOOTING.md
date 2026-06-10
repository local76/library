# Icon Troubleshooting

This page covers how to verify that a Windows binary in the `local76`
ecosystem actually has its icon embedded correctly. It is the
companion to [`VISUAL_STANDARDS.md`](./VISUAL_STANDARDS.md) and the
diagnostic recipe that `toolkit/scripts/verify-icon.ps1` automates.

## Why this exists

The historical `winres 0.1.x` build-script path used by every local76
tool silently produces a binary whose ICONDIR resource is **corrupted**:
the 16×16 entry embeds correctly, but the 32×32 / 48×48 / 256×256
entries have their widths, heights, color depths, and sizes all
trashed. Windows Explorer reads the 16×16 frame and uses it everywhere,
or falls back to a generic console icon entirely — the user sees
"all my screensavers have the same icon" even though every `.scr` has
its own brand asset on disk.

Microsoft's `embed-resource` 2.x crate parses PNG-compressed ICOs
correctly, so the fix is to migrate. This page documents the verifier
you should run after any icon-related change to confirm the migration
worked.

## ICONDIR byte layout (what to look for)

A Windows ICO file (and the `RT_GROUP_ICON` resource of a PE binary)
starts with a 6-byte ICONDIR header:

```
[0..1] reserved   = 0x0000
[2..3] type       = 0x0001  (1 = icon)
[4..5] count      = N       (number of sub-icons that follow)
```

Followed by `N` × 16-byte ICONDIRENTRY rows:

```
[0]    width   (0 means 256)
[1]    height  (0 means 256)
[2]    color count (0 if more than 256 colors)
[3]    reserved = 0
[4..5] color planes
[6..7] bits per pixel
[8..11] size in bytes of the image data
[12..15] offset from start of ICO to image data
```

For a 4-size local76 ICO, the **expected** pattern is:

```
count = 4
  16x16   bpp=32
  32x32   bpp=32
  48x48   bpp=32
  256x256 bpp=32
```

## Verifier recipe (PowerShell)

Drop this into `verify-icon.ps1` (the toolkit already has this; see
`toolkit/scripts/verify-icon.ps1`). It scans a `dist/binaries/` folder
and reports PASS/FAIL per binary.

```powershell
param([string]$BinDir = "dist/binaries")

$results = foreach ($bin in Get-ChildItem -LiteralPath $BinDir -File) {
    $bytes = [System.IO.File]::ReadAllBytes($bin.FullName)
    $icondirHits = @()
    for ($i = 0; $i -lt $bytes.Length - 6; $i++) {
        if ($bytes[$i] -eq 0 -and $bytes[$i+1] -eq 0 `
            -and $bytes[$i+2] -eq 1 -and $bytes[$i+3] -eq 0) {
            $count = [BitConverter]::ToUInt16($bytes, $i+4)
            if ($count -ge 1 -and $count -le 16) { $icondirHits += $i }
        }
    }
    $best = $null
    foreach ($h in $icondirHits) {
        $count = [BitConverter]::ToUInt16($bytes, $h+4)
        $p = $h + 6
        $valid32bpp = 0
        for ($i = 0; $i -lt $count; $i++) {
            $bpp = [BitConverter]::ToUInt16($bytes, $p + 6)
            $w = if ($bytes[$p] -eq 0) { 256 } else { $bytes[$p] }
            $h2 = if ($bytes[$p+1] -eq 0) { 256 } else { $bytes[$p+1] }
            if ($bpp -eq 32 -and $w -in 16,32,48,256 -and $h2 -in 16,32,48,256) {
                $valid32bpp++
            }
            $p += 16
        }
        if ($valid32bpp -gt ($best.Valid32bpp -as [int])) {
            $best = [PSCustomObject]@{ Offset=$h; Count=$count; Valid32bpp=$valid32bpp }
        }
    }
    $verdict = if ($best -and $best.Valid32bpp -ge 4) { "PASS" } else { "FAIL" }
    [PSCustomObject]@{
        Binary   = $bin.Name
        Offset   = if ($best) { $best.Offset } else { "-" }
        Count    = if ($best) { $best.Count } else { 0 }
        Valid32bpp = if ($best) { $best.Valid32bpp } else { 0 }
        Verdict  = $verdict
    }
}
$results | Format-Table -AutoSize
```

## Common failure modes

| Symptom | Cause | Fix |
|---|---|---|
| `Valid32bpp` is 1, all other entries are corrupt | `winres 0.1.x` parser | Switch to `embed-resource 2.x` per `VISUAL_STANDARDS.md` § C |
| `Count` is 0 (no ICONDIR found at all) | Build script never ran, or `target_os != "windows"` | Confirm `cargo build --release` on Windows, not cross-compile |
| `Valid32bpp` is 4 but file shows wrong icon in Explorer | Windows icon cache stale | `ie4uinit.exe -show` (or restart `explorer.exe`) |
| `Count` is 4 but sizes are 16, 32, 64, 128 (missing 256) | Source ICO was generated without 256 | Re-export with 16/32/48/256 at 32bpp per `VISUAL_STANDARDS.md` § B |
| Linux `.scr` is fine but `.deb` shows generic icon | Packaging overwrites `.desktop` with `Icon=utilities-terminal` | Use the per-scene `assets/scene-<name>.desktop` instead; install PNG to hicolor |

## Quick one-liner (no script)

```powershell
$bin = "dist\binaries\chaos.scr"
$bytes = [System.IO.File]::ReadAllBytes($bin)
"size = $($bytes.Length) bytes"
# Find the LAST ICONDIR (the one with the most sub-icons is usually the real one)
$best = $null
for ($i = 0; $i -lt $bytes.Length - 6; $i++) {
    if ($bytes[$i] -eq 0 -and $bytes[$i+1] -eq 0 -and $bytes[$i+2] -eq 1 -and $bytes[$i+3] -eq 0) {
        $count = [BitConverter]::ToUInt16($bytes, $i+4)
        if ($count -ge 1 -and $count -le 16 -and (-not $best -or $count -gt $best.Count)) {
            $best = [PSCustomObject]@{Offset=$i; Count=$count}
        }
    }
}
"best ICONDIR at offset $($best.Offset), count = $($best.Count)"
```
