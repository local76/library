use std::fs;
use std::path::PathBuf;

/// Re-encode a multi-size ICO into a form that Windows SDK rc.exe 10.0+ handles
/// correctly. The bug: rc.exe mangles the ICONDIR offset/size fields for entries
/// 2..N when given a multi-size ICO via `1 ICON "path.ico"`. Workaround: split
/// the ICO into 4 single-size ICOs and have the caller declare them as 4
/// separate ICON resources in the .rc file.
///
/// We return a Vec of (resource_id, single_size_ico_path) pairs and have the
/// caller write 4 ICON directives instead of 1.
///
/// This is a no-op on non-Windows targets.
pub fn split_for_rc(ico_path: &str) -> Vec<(u16, PathBuf)> {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return Vec::new();
    }

    let bytes = match fs::read(ico_path) {
        Ok(b) => b,
        Err(_) => return Vec::new(),
    };
    if bytes.len() < 6 {
        return Vec::new();
    }
    let count = u16::from_le_bytes([bytes[4], bytes[5]]) as usize;
    if count == 0 || count > 16 {
        return Vec::new();
    }

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let src = PathBuf::from(ico_path);
    let abs_src = if src.is_absolute() {
        src.clone()
    } else {
        PathBuf::from(&manifest_dir).join(&src)
    };
    let stem = abs_src.file_stem().and_then(|s| s.to_str()).unwrap_or("icon");
    let parent = abs_src.parent().unwrap_or_else(|| std::path::Path::new("."));
    let split_dir = parent.join(format!("{stem}_split"));
    let _ = fs::create_dir_all(&split_dir);

    let mut result = Vec::new();
    let mut p = 6;
    for i in 0..count {
        if p + 16 > bytes.len() { break; }
        let w = bytes[p];
        let h = bytes[p + 1];
        let sz = u32::from_le_bytes([bytes[p+8], bytes[p+9], bytes[p+10], bytes[p+11]]) as usize;
        let off = u32::from_le_bytes([bytes[p+12], bytes[p+13], bytes[p+14], bytes[p+15]]) as usize;
        if off + sz > bytes.len() { break; }
        let data = &bytes[off..off+sz];

        // Build a single-size ICO: 22-byte header + image data
        let mut single = Vec::with_capacity(22 + sz);
        single.extend_from_slice(&0u16.to_le_bytes());      // reserved
        single.extend_from_slice(&1u16.to_le_bytes());      // type = icon
        single.extend_from_slice(&1u16.to_le_bytes());      // count = 1
        single.push(w);
        single.push(h);
        single.push(0);                                    // color count
        single.push(0);                                    // reserved
        single.extend_from_slice(&1u16.to_le_bytes());      // planes
        single.extend_from_slice(&32u16.to_le_bytes());     // bpp
        single.extend_from_slice(&(sz as u32).to_le_bytes());  // size
        single.extend_from_slice(&22u32.to_le_bytes());    // data offset
        single.extend_from_slice(data);

        let fname = format!("{}.ico", if w == 0 { 256u32 } else { w as u32 });
        let out = split_dir.join(&fname);
        let _ = fs::write(&out, &single);

        // Resource IDs 1..16. The caller uses these in the .rc file as
        // 1 ICON "16.ico", 2 ICON "32.ico", etc.
        result.push((i as u16 + 1, out));
        p += 16;
    }

    result
}
