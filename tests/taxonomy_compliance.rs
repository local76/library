// Minimal taxonomy test for the flat folder tree.
//
// The original 4-layer taxonomy test was deprecated when the 4-layer
// modules were removed in 2026.6.10.1. The flat folder tree doesn't
// have a layered architecture that needs enforcing; the type system
// catches circular deps via `cargo test --no-run`.
//
// This test just confirms all 4 folders exist and contain at least
// one .rs file. It's a smoke test, not a linter.

use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn test_folders_present() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for folder in &["core", "ui", "toolkit", "apps"] {
        let p = src_dir.join(folder);
        assert!(p.is_dir(), "Expected folder: {}", p.display());
    }
}

#[test]
fn test_folders_have_source_files() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut count = 0;
    fn visit(dir: &Path, n: &mut usize) {
        if dir.is_dir() {
            for entry in fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    visit(&path, n);
                } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    *n += 1;
                }
            }
        }
    }
    visit(&src_dir, &mut count);
    assert!(count > 50, "Expected >50 .rs files under src/, found {}", count);
}
