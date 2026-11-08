//! Shared native SingZ controls and bundled fonts. `init` registers the
//! process-local faces; this crate never installs system fonts.
pub mod fonts;
pub use fonts::register_fonts;
mod window_controls;
pub use window_controls::{window_buttons, window_resize_edges};
mod controls;
mod training;
pub use training::*;
mod icons;
pub use icons::{NativeIcon, TransportIcon, colored_icon, icon};
mod typography;
pub use typography::{TrackedText, css_normal_line_metrics, tracked_label, tracked_text};
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

mod models;
pub use models::*;

mod split_controls;
pub use split_controls::*;

mod guide;
pub use guide::*;

mod metronome_controls;
pub use metronome_controls::*;

mod lyrics_editor_controls;
pub use lyrics_editor_controls::*;

mod project_picker_controls;
pub use project_picker_controls::*;

mod library_import_controls;
pub use library_import_controls::*;

mod toast;
pub use toast::{Toast, ToastState};

mod release_highlights_controls;
pub use release_highlights_controls::*;

mod lyrics_sidebar_controls;
pub use lyrics_sidebar_controls::*;

mod engine_setup_controls;
pub use engine_setup_controls::*;

mod log_controls;
pub use log_controls::*;

mod training_recording_controls;
pub use training_recording_controls::*;

mod project_delete_controls;
pub use project_delete_controls::*;

mod track_controls;
pub use track_controls::{audio_control, track_action, track_name_input, track_name_label};

mod stem_export;
pub use stem_export::*;

mod carry_line_controls;
pub use carry_line_controls::*;

mod transport_timing_controls;
pub use transport_timing_controls::*;

mod drive_controls;
pub use drive_controls::*;

mod language_controls;
pub use language_controls::*;
