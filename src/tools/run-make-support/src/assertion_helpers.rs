//! Collection of assertions and assertion-related helpers.

use std::panic;
use std::path::Path;

use crate::fs as rfs;

/// Assert that `actual` is equal to `expected`.
#[track_caller]
pub fn assert_equals<A: AsRef<str>, E: AsRef<str>>(actual: A, expected: E) {
    let actual = actual.as_ref();
    let expected = expected.as_ref();
    if actual != expected {
        eprintln!("=== ACTUAL TEXT ===");
        eprintln!("{}", actual);
        eprintln!("=== EXPECTED ===");
        eprintln!("{}", expected);
        panic!("expected text was not found in actual text");
    }
}

/// Assert that `haystack` contains `needle`.
#[track_caller]
pub fn assert_contains<H: AsRef<str>, N: AsRef<str>>(haystack: H, needle: N) {
    let haystack = haystack.as_ref();
    let needle = needle.as_ref();
    if !haystack.contains(needle) {
        eprintln!("=== HAYSTACK ===");
        eprintln!("{}", haystack);
        eprintln!("=== NEEDLE ===");
        eprintln!("{}", needle);
        panic!("needle was not found in haystack");
    }
}

/// Assert that `haystack` does not contain `needle`.
#[track_caller]
pub fn assert_not_contains<H: AsRef<str>, N: AsRef<str>>(haystack: H, needle: N) {
    let haystack = haystack.as_ref();
    let needle = needle.as_ref();
    if haystack.contains(needle) {
        eprintln!("=== HAYSTACK ===");
        eprintln!("{}", haystack);
        eprintln!("=== NEEDLE ===");
        eprintln!("{}", needle);
        panic!("needle was unexpectedly found in haystack");
    }
}

/// Assert that all files in `dir1` exist and have the same content in `dir2`
pub fn assert_dirs_are_equal(dir1: impl AsRef<Path>, dir2: impl AsRef<Path>) {
    let dir2 = dir2.as_ref();
    rfs::read_dir_entries(dir1, |entry_path| {
        let entry_name = entry_path.file_name().unwrap();
        if entry_path.is_dir() {
            assert_dirs_are_equal(&entry_path, &dir2.join(entry_name));
        } else {
            let path2 = dir2.join(entry_name);
            let file1 = rfs::read(&entry_path);
            let file2 = rfs::read(&path2);

            // We don't use `assert_eq!` because they are `Vec<u8>`, so not great for display.
            // Why not using String? Because there might be minified files or even potentially
            // binary ones, so that would display useless output.
            assert!(
                file1 == file2,
                "`{}` and `{}` have different content",
                entry_path.display(),
                path2.display(),
            );
        }
    });
}
