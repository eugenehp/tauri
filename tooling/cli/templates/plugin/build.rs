const COMMANDS: &[&str] = &["ping", "execute"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    // .visionos_path("visionos")
    .build();
}
