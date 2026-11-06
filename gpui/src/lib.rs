//! Shared native SingZ controls and bundled fonts. `init` registers the
//! process-local faces; this crate never installs system fonts.
pub mod fonts;
pub use fonts::register_fonts;
mod window_controls;
pub use window_controls::{window_buttons, window_resize_edges};
mod controls;
mod icons;
pub use icons::{colored_icon, icon, NativeIcon, TransportIcon};
mod typography;
pub use typography::{css_normal_line_metrics, tracked_label, tracked_text, TrackedText};
pub mod tokens;
pub use controls::*;

/// Apply SingZ colours to native editable controls after `gpui_kit::init`.
/// Reapply after a host theme change, which replaces the component palette.
pub fn init(cx: &mut gpui_kit::App) {
    register_fonts(cx).expect("register bundled SingZ UIKit fonts");
    use gpui_kit::{component::Theme, rgb, rgba};
    use tokens::*;
    let theme = Theme::global_mut(cx);
    theme.colors.background = rgb(BG).into();
    theme.colors.foreground = rgb(TEXT).into();
    theme.colors.input = rgb(PANEL_DEEP).into();
    theme.colors.border = rgba(LINE_STRONG).into();
    theme.colors.muted = rgb(PANEL).into();
    // Chromium's editable input placeholder uses the browser default #757575.
    // Other muted labels select the explicit SingZ DIM token in their builders.
    theme.colors.muted_foreground = rgb(0x757575).into();
    theme.colors.accent = rgba(ACCENT_SOFT).into();
    theme.colors.accent_foreground = rgb(ACCENT).into();
    theme.colors.ring = rgba(FOCUS_RING).into();
    theme.colors.caret = rgb(ACCENT).into();
    theme.colors.selection = rgba(0xffa02840).into();
}
