//! Source-backed custom track title actions and inline editor.
use crate::tokens::{ACCENT, FAINT, LINE_STRONG, TEXT};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::prelude::*;
use gpui_kit::*;

pub fn track_action(
    id: impl Into<ElementId>,
    glyph: &'static str,
    trailing: bool,
) -> Stateful<Div> {
    div()
        .id(id)
        .w(px(18.))
        .h(px(18.))
        .flex_shrink_0()
        .rounded(px(6.))
        .flex()
        .items_center()
        .justify_center()
        .text_size(px(10.))
        .text_color(rgb(FAINT))
        .opacity(0.)
        .cursor_pointer()
        .when(trailing, |d| d.ml(px(2.)))
        .group_hover("custom-track-title", |s| s.opacity(1.))
        .focus(|s| {
            s.opacity(1.)
                .text_color(rgb(ACCENT))
                .bg(rgb(ACCENT).alpha(0.1))
        })
        .hover(|s| s.text_color(rgb(TEXT)).bg(rgba(0xfff0d614)))
        .child(glyph)
}

pub fn track_name_input(state: &Entity<InputState>, color: u32) -> Input {
    let input = Input::new(state)
        .focus_bordered(false)
        .flex_1()
        .min_w_0()
        .px(px(6.))
        .py(px(1.))
        .rounded(px(6.))
        .border_1()
        .border_color(rgba(LINE_STRONG))
        .bg(rgba(0xfff0d60f))
        .text_color(rgb(TEXT))
        .font_family("SingZ Display 700")
        .font_weight(FontWeight::NORMAL)
        .text_size(px(14.5))
        .line_height(px(17.5))
        .border_color(rgb(color));
    Styled::h(input, px(21.5))
}

/// Shared unavailable-audio styling. The host guards callbacks and keyboard input.
pub fn audio_control(control: Stateful<Div>, available: bool) -> Stateful<Div> {
    if available {
        control
    } else {
        control.opacity(0.35).cursor_default()
    }
}

/// Mixer label typography is shared with the inline editor. Preserve ordinary
/// one/two-line names; contain pathological saved labels inside their own row
/// instead of letting unlimited wrapped glyphs paint through neighboring lanes.
pub fn track_name_label(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    color: u32,
) -> Stateful<Div> {
    let label: SharedString = label.into();
    let tooltip = label.clone();
    div()
        .id(id)
        .min_w_0()
        .flex_1()
        .max_h(px(35.))
        .overflow_hidden()
        .text_size(px(14.5))
        .line_height(px(17.5))
        .line_clamp(2)
        .font_family("SingZ Display 700")
        .font_weight(FontWeight::NORMAL)
        .text_color(rgb(color))
        .child(label)
        .tooltip(move |w, cx| {
            gpui_kit::component::tooltip::Tooltip::new(tooltip.clone())
                .max_w(px(420.))
                .font_family(crate::tokens::DISPLAY_400)
                .text_size(px(12.))
                .build(w, cx)
        })
}
