use rust_embed::Embed;

/// Embedded assets baked into the binary at compile time.
#[derive(Embed)]
#[folder = "assets/"]
pub struct Assets;

impl Assets {
    /// Get the bytes of an embedded asset by path (relative to `assets/`).
    pub fn get_str(path: &str) -> Option<String> {
        Self::get(path).map(|f| String::from_utf8_lossy(f.data.as_ref()).into_owned())
    }
}
