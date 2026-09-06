use std::fs;
use std::path::Path;

fn main() {
    println!("cargo::rerun-if-changed=fonts/fonts.toml");
    iced_fontello::build("fonts/fonts.toml").expect("Fonts Missing");
    patch_fontello_output("src/icon.rs");

    iced_lucide::build_all("lucide", "icon_lucide").expect("Lucide Icons");
}

/// Brings `iced_fontello`'s generated module up to the iced master API.
///
/// `iced_fontello` 0.14.1 is the newest release and still emits
/// `Font::with_name`, which master replaced with the const `Font::new`. The
/// generator writes this file on every build, so the fix has to be applied
/// after it runs rather than committed into the file.
fn patch_fontello_output(path: impl AsRef<Path>) {
    let path = path.as_ref();

    let Ok(generated) = fs::read_to_string(path) else {
        return;
    };

    let patched = generated.replace("Font::with_name(", "Font::new(");

    if patched != generated {
        fs::write(path, patched).expect("rewrite generated icon module");
    }
}
