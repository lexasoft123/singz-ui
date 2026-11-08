//! Shared transport settings rows, segmented choices and editable tempo field.
use crate::tokens::*;
use gpui_kit::{
    component::input::{Input, InputState},
    prelude::FluentBuilder,
    *,
};
pub fn transport_setting_row(label: impl Into<SharedString>) -> Div {
    div().flex().items_center().gap(px(7.)).mb(px(8.)).child(
        div()
            .min_w(px(74.))
            .text_size(px(12.))
            .line_height(px(14.5))
            .text_color(rgb(DIM))
            .child(label.into()),
    )
}
pub fn mode_group() -> Div {
    div()
        .flex()
        .border_1()
        .border_color(rgba(LINE))
        .rounded_full()
        .overflow_hidden()
}
pub fn mode_option(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    on: bool,
    enabled: bool,
    first: bool,
    last: bool,
) -> Stateful<Div> {
    div()
        .id(id)
        .px(px(10.))
        .py(px(5.))
        .text_size(px(12.))
        .line_height(px(14.5))
        .font_family(DISPLAY_600)
        .text_color(rgb(if on { ACCENT } else { DIM }))
        .when(on, |d| d.bg(rgba(ACCENT_SOFT)))
        .when(first, |d| d.rounded_l_full())
        .when(last, |d| d.rounded_r_full())
        .when(!first, |d| d.border_l_1().border_color(rgba(LINE)))
        .when(enabled, |d| d.cursor_pointer())
        .when(!enabled, |d| d.opacity(0.4).cursor_default())
        .child(label.into())
}
pub fn metronome_chip(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    enabled: bool,
) -> Stateful<Div> {
    crate::transport_chip(id, label)
        .w_auto()
        .min_w(px(26.))
        .h(px(26.))
        .px(px(7.))
        .when(!enabled, |d| d.opacity(0.45).cursor_default())
}
pub fn metronome_bpm_input(state: &Entity<InputState>) -> Div {
    let input = Input::new(state)
        .focus_bordered(false)
        .w(px(44.))
        .px_0()
        .py_0()
        .border_0()
        .bg(transparent_black())
        .text_color(rgb(TEXT))
        .font_family(MONO_400)
        .text_size(px(12.5))
        .text_right();
    div()
        .flex()
        .items_center()
        .gap(px(3.))
        .h(px(26.))
        .px(px(7.))
        .border_1()
        .border_color(rgba(LINE))
        .rounded(px(8.))
        .child(Styled::h(input, px(24.)))
        .child(div().text_size(px(9.)).text_color(rgb(FAINT)).child("bpm"))
}

/// Source `.ps-note`/`.ps-bar`: static analysis progress avoids animation work
/// on weak integrated GPUs. `progress` is a normalized fraction, rounded to the
/// same whole-percent fill used by the production pitch strip.
pub fn analysis_progress_note(label: impl Into<SharedString>, progress: f32) -> Div {
    let progress = if progress.is_finite() {
        (progress.clamp(0., 1.) * 100.).round() / 100.
    } else {
        0.
    };
    div()
        .flex()
        .items_center()
        .child(crate::tracked_text(label, MONO_400, 10., 0.4, FAINT))
        .child(
            div()
                .ml(px(6.))
                .w(px(64.))
                .h(px(4.))
                .flex_shrink_0()
                .rounded(px(2.))
                .bg(rgb(FAINT).alpha(0.25))
                .overflow_hidden()
                .child(
                    div()
                        .h_full()
                        .w(relative(progress))
                        .rounded(px(2.))
                        .bg(rgb(FAINT)),
                ),
        )
}
