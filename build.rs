fn main() {
    println!("cargo::rerun-if-changed=fonts/fonts.toml");
    iced_fontello::build("fonts/fonts.toml").expect("Fonts Missing");
    iced_lucide::build_all("icon_lucide").expect("Lucide Icons");
}