//! Generates `src/icon_lucide.rs`: a browsable index of every icon family
//! `iced_lucide` carries.
//!
//! `build_index` is the right call rather than `build_all` because this is a UI
//! builder — the icon pickers walk `ALL_ICONS` and filter by family, and naming
//! eighty thousand icons as Rust functions would help nobody. The app's own
//! chrome icons are hand-written aliases over that index in `src/icon.rs`.
//!
//! One subset `.ttf` per family lands in `fonts/`. Those are build output, not
//! source, and are gitignored.
//!
//! It also hands the code generator the `iced` revision this builder resolved,
//! so generated projects can pin the same one.
use std::fs;

pub fn main() {
    // An empty slice means every family the enabled features provide, which is
    // all of them: `iced_lucide` is asked for with `features = ["all"]`.
    iced_lucide::build_index(&[], "icon_lucide").expect("Build icon index");

    println!("cargo::rerun-if-changed=Cargo.lock");
    println!("cargo::rustc-env=ICED_REVISION={}", iced_revision());
}

/// The `iced` commit `Cargo.lock` resolved.
///
/// Generated projects pin this rather than tracking `branch = "master"`, which
/// is not reproducible: master moves, and when it does a generated project
/// stops compiling against the same `widgets`, `iced_lucide` and
/// `iced-native-frame` this builder works with. Moving the builder forward
/// moves what it generates forward with it.
fn iced_revision() -> String {
    const FALLBACK: &str = "master";

    let Ok(lock) = fs::read_to_string("Cargo.lock") else {
        return FALLBACK.to_string();
    };

    // The entry looks like:
    //   name = "iced"
    //   version = "0.15.0-dev"
    //   source = "git+https://github.com/iced-rs/iced.git?branch=master#<sha>"
    lock.split("[[package]]")
        .find(|package| package.contains("name = \"iced\"\n"))
        .and_then(|package| package.lines().find(|line| line.starts_with("source = ")))
        .and_then(|source| source.rsplit_once('#'))
        .map(|(_, sha)| sha.trim_end_matches('"').to_string())
        .unwrap_or_else(|| FALLBACK.to_string())
}
