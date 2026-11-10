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
pub const DISPLAY_680: &[u8] = font_bytes!("Bricolage680.ttf");
pub const DISPLAY_780: &[u8] = font_bytes!("Bricolage780.ttf");
pub const DISPLAY_700: &[u8] = font_bytes!("Bricolage700.ttf");
pub const DISPLAY_800: &[u8] = font_bytes!("Bricolage800.ttf");
pub const MONO_400: &[u8] = font_bytes!("MartianMono400.ttf");
pub const MONO_500: &[u8] = font_bytes!("MartianMono500.ttf");
pub const MONO_600: &[u8] = font_bytes!("MartianMono600.ttf");
pub const MONO_700: &[u8] = font_bytes!("MartianMono700.ttf");

pub const DISPLAY_720: &[u8] = font_bytes!("Bricolage720.ttf");

pub const MONO_650: &[u8] = font_bytes!("MartianMono650.ttf");

pub const MONO_720: &[u8] = font_bytes!("MartianMono720.ttf");

pub const MONO_760: &[u8] = font_bytes!("MartianMono760.ttf");

// Linux's native text system may have no host emoji font. Keep the source's
// Unicode folder and supply its color bitmap without changing Apple/Segoe fallback.
#[cfg(target_os = "linux")]
const FOLDER_EMOJI: &[u8] = include_bytes!("../assets/fonts/NotoColorEmojiFolder.ttf");

/// Register bundled faces once per App. No system installation or host assets required.
pub fn register_fonts(cx: &mut App) -> anyhow::Result<()> {
    if cx.has_global::<FontsRegistered>() { return Ok(()); }
    cx.text_system().add_fonts(vec![
        Cow::Borrowed(DISPLAY_680),
        Cow::Borrowed(DISPLAY_780),
        Cow::Borrowed(DISPLAY_720),
        Cow::Borrowed(MONO_650),
        Cow::Borrowed(MONO_720),
        Cow::Borrowed(MONO_760),
        Cow::Borrowed(DISPLAY_400),
        Cow::Borrowed(DISPLAY_500),
        Cow::Borrowed(DISPLAY_600),
        Cow::Borrowed(DISPLAY_650),
        Cow::Borrowed(DISPLAY_700),
        Cow::Borrowed(DISPLAY_800),
        Cow::Borrowed(MONO_400),
        Cow::Borrowed(MONO_500),
        Cow::Borrowed(MONO_600),
        Cow::Borrowed(MONO_700),
    ])?;
    #[cfg(target_os = "linux")]
    cx.text_system().add_fonts(vec![Cow::Borrowed(FOLDER_EMOJI)])?;
    cx.set_global(FontsRegistered);
    Ok(())
}
