use std::borrow::Cow;

use anyhow::Result;
use rust_embed::RustEmbed;
use gpui_kit::*;

/// Application-specific assets embedded at compile time.
/// Falls back to gpui-kit's bundled icons for anything not found here.
#[derive(RustEmbed)]
#[folder = "assets"]
pub struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }
        // Try custom assets first.
        if let Some(f) = AppAssets::get(path) {
            return Ok(Some(f.data));
        }
        // Fall back to gpui-kit's bundled icons.
        gpui_kit::assets::Assets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut files: Vec<SharedString> = AppAssets::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect();
        files.extend(gpui_kit::assets::Assets.list(path)?);
        Ok(files)
    }
}
