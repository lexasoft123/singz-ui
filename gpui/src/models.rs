//! Source-backed native model-manager control variants.
use crate::tokens::*;
use gpui_kit::*;
pub fn model_rows() -> Div {
    div().flex().flex_col().gap(px(14.)).mt(px(18.)).mb(px(6.))
}
pub fn model_head() -> Div {
    div().flex().justify_between().items_baseline().mb(px(6.))
}
pub fn model_progress(percent: f32) -> Div {
    div()
        .w_full()
        .h(px(3.))
        .rounded(px(2.))
        .bg(rgba(LINE))
        .overflow_hidden()
        .child(
            div()
                .h_full()
                .w(relative(percent.clamp(0., 100.) / 100.))
                .bg(linear_gradient(
                    90.,
                    linear_color_stop(rgb(ACCENT_DEEP), 0.),
                    linear_color_stop(rgb(0xffd489), 1.),
                ))
                .shadow(vec![crate::glow(0xffaa3c, 0.7, 4.)]),
        )
}

/// Shared `.mode-seg`: clipped pill outline with separators between choices.
pub fn mode_segment(children: impl IntoIterator<Item = Stateful<Div>>) -> Div {
    let children: Vec<_> = children.into_iter().collect();
    let last = children.len().saturating_sub(1);
    div()
        .flex()
        .border_1()
        .border_color(rgba(LINE))
        .rounded_full()
        .overflow_hidden()
        .children(children.into_iter().enumerate().map(|(index, button)| {
            // GPUI overflow clips to a rectangle, so the edge buttons also
            // paint rounded backgrounds to keep active fill inside the rim.
            let button = if index == 0 {
                button.rounded_l(px(999.))
            } else {
                button.border_l_1().border_color(rgba(LINE))
            };
            if index == last { button.rounded_r(px(999.)) } else { button }
        }))
}
pub fn mode_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    active: bool,
) -> Stateful<Div> {
    div()
        .id(id)
        .px(px(14.))
        .py(px(5.))
        .text_size(px(12.))
        .line_height(px(14.5))
        .font_family(DISPLAY_600)
        .text_color(rgb(if active { ACCENT } else { DIM }))
        .bg(rgba(if active { ACCENT_SOFT } else { 0 }))
        .cursor_pointer()
        .child(label.into())
}
