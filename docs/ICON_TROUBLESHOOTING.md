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

In the 2026-06-09 release, we migrated to Microsoft's
`embed-resource` 2.x crate, which parses PNG-compressed ICOs
correctly at the build-script level. The migration fixed the parser
issue but uncovered a second issue: the Windows SDK `rc.exe 10.0+`
compiler that `embed-resource` invokes has its **own** ICONDIR bug.
When given a single `1 ICON "path.ico"` directive referencing a
multi-size ICO, `rc.exe 10.0+` mangles the ICONDIR offset/size fields
for entries 2..N in the final PE resource directory — the same
symptom as the `winres` bug, but caused by a different stage of the
toolchain.

The 4.2 workaround lives in
`library::core::rc_split::split_for_rc`. When the library's
`write_brand_rc` helper generates the `.rc` file, it first calls
`split_for_rc` on the source ICO, which:

1. Parses the multi-size ICO into its 4 constituent sizes
   (16×16, 32×32, 48×48, 256×256) using the `ico` crate.
2. Writes each size as a single-size ICO into
   `assets/<name>_split/<size>.ico`.
3. Returns 4 `(resource_id, path)` pairs to the caller.

The generated `.rc` then declares **4 separate** `IDI_ICON1`...
`IDI_ICON4` resources, one per single-size file. `rc.exe 10.0+`
handles 4 single-size ICOs correctly because the ICONDIR-corruption
bug only triggers when there is more than one size in a single ICO.

This page documents the verifier you should run after any icon-related
change to confirm both stages of the pipeline (parse + compile) worked.

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
| `Valid32bpp` is 1, all other entries are corrupt (pre-2026-06-09 builds) | `winres 0.1.x` parser | Re-build with `embed-resource` 2.x per `VISUAL_STANDARDS.md` § C |
| `Valid32bpp` is 1 on a 2026-06-09-or-later build, but `assets/<name>_split/` exists and has 4 ICOs | `split_for_rc` ran but the generated `.rc` declared them as a single `1 ICON` directive | Re-build; if the issue persists, check that `library::core::build_resources::write_brand_rc` is being called and the `library` dep is on the 4.2 line or later |
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
