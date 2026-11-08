//! Production effective BPM entry, static analysis bar and clock-driven dots.
use crate::tokens::*;
use gpui_kit::{
    component::input::{Input, InputState},
    prelude::FluentBuilder,
    *,
};
pub fn transport_bpm_field(
    id: impl Into<ElementId>,
    state: &Entity<InputState>,
    w: &Window,
) -> Stateful<Div> {
    let input = Input::new(state)
        .focus_bordered(false)
        .w(px(30.))
        .px_0()
        .py_0()
        .border_0()
        .bg(rgba(0))
        .font_family(MONO_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(12.5))
        .line_height(crate::css_normal_line_metrics(w, MONO_400, 12.5).0)
        .text_right()
        .text_color(rgb(TEXT));
    div()
        .id(id)
        .flex()
        .items_center()
        .gap(px(3.))
        .h(px(26.))
        .px(px(7.))
        .border_1()
        .border_color(rgba(LINE))
        .rounded(px(8.))
        .cursor_text()
        .tooltip(|w, cx| {
            gpui_kit::component::tooltip::Tooltip::new("Set the playback tempo in beats per minute")
                .font_family(DISPLAY_400)
                .text_size(px(12.))
                .build(w, cx)
        })
        .child(Styled::h(input, px(17.)))
        .child(
            div()
                .text_size(px(9.))
                .font_family(DISPLAY_400)
                .font_weight(FontWeight::NORMAL)
                .text_color(rgb(FAINT))
                .child("bpm"),
        )
}
pub fn transport_bpm_analysis(
    id: impl Into<ElementId>,
    label: String,
    progress: f32,
) -> Stateful<Div> {
    let percent = (progress.clamp(0., 1.) * 100.).round();
    div()
        .id(id)
        .flex()
        .items_center()
        .gap(px(6.))
        .h(px(26.))
        .px(px(7.))
        .border_1()
        .border_color(rgba(LINE))
        .rounded(px(8.))
        .opacity(0.4)
        .cursor_default()
        .tooltip(move |w, cx| {
            gpui_kit::component::tooltip::Tooltip::new(label.clone())
                .font_family(DISPLAY_400)
                .text_size(px(12.))
                .build(w, cx)
        })
        .child(
            div()
                .w(px(42.))
                .h(px(4.))
                .rounded(px(2.))
                .bg(rgb(FAINT).alpha(0.25))
                .overflow_hidden()
                .child(
                    div()
                        .h_full()
                        .w(relative(percent / 100.))
                        .rounded(px(2.))
                        .bg(rgb(FAINT)),
                ),
        )
        .child(
            div()
                .text_size(px(9.))
                .font_family(DISPLAY_400)
                .font_weight(FontWeight::NORMAL)
                .text_color(rgb(FAINT))
                .child(format!("{percent:.0}%")),
        )
}
pub fn transport_count_in_dots(id: impl Into<ElementId>, text: String) -> Stateful<Div> {
    div()
        .id(id)
        .flex_none()
        .whitespace_nowrap()
        .child(crate::tracked_text(text, DISPLAY_400, 12., 2., ACCENT).normal_line_height())
}
