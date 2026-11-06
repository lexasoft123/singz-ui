//! Bundled UIKit font distribution; every named family has one normal style.
use gpui_kit::{App, Global};
use std::borrow::Cow;
struct FontsRegistered;
impl Global for FontsRegistered {}
macro_rules! font_bytes {
    ($file:literal) => {{
        #[cfg(target_os = "windows")]
        const DATA: &[u8] = include_bytes!(concat!("../assets/fonts/windows/", $file));
        #[cfg(not(target_os = "windows"))]
        const DATA: &[u8] = include_bytes!(concat!("../assets/fonts/", $file));
        DATA
    }};
}
pub const DISPLAY_400: &[u8] = font_bytes!("Bricolage400.ttf");
pub const DISPLAY_500: &[u8] = font_bytes!("Bricolage500.ttf");
pub const DISPLAY_600: &[u8] = font_bytes!("Bricolage600.ttf");
pub const DISPLAY_650: &[u8] = font_bytes!("Bricolage650.ttf");
pub const DISPLAY_700: &[u8] = font_bytes!("Bricolage700.ttf");
pub const DISPLAY_800: &[u8] = font_bytes!("Bricolage800.ttf");
pub const MONO_400: &[u8] = font_bytes!("MartianMono400.ttf");
pub const MONO_500: &[u8] = font_bytes!("MartianMono500.ttf");
pub const MONO_700: &[u8] = font_bytes!("MartianMono700.ttf");

/// Register bundled faces once per App. No system installation or host assets required.
pub fn register_fonts(cx: &mut App) -> anyhow::Result<()> {
    if cx.has_global::<FontsRegistered>() { return Ok(()); }
    cx.text_system().add_fonts(vec![
        Cow::Borrowed(DISPLAY_400),
        Cow::Borrowed(DISPLAY_500),
        Cow::Borrowed(DISPLAY_600),
        Cow::Borrowed(DISPLAY_650),
        Cow::Borrowed(DISPLAY_700),
        Cow::Borrowed(DISPLAY_800),
        Cow::Borrowed(MONO_400),
        Cow::Borrowed(MONO_500),
        Cow::Borrowed(MONO_700),
    ])?;
    cx.set_global(FontsRegistered);
    Ok(())
}
