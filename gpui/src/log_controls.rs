//! Source diagnostic log surfaces and typography; host owns data and selection.
use crate::tokens::*;
use gpui_kit::{base::SelectableText, prelude::FluentBuilder, *};
pub fn log_card(viewport: Size<Pixels>, height: f32) -> Stateful<Div> {
    crate::modal_card()
        .id("log-card")
        .w(px((f32::from(viewport.width) - 60.).min(860.)))
        .h(px(height))
        .px(px(24.))
        .py(px(22.))
}
pub fn log_head() -> Div {
    div()
        .flex()
        .items_baseline()
        .gap(px(12.))
        .h(px(29.5))
        .flex_shrink_0()
        .mb(px(12.))
}
pub fn log_title() -> Div {
    log_title_label("Log")
}
pub fn log_title_label(label: impl Into<SharedString>) -> Div {
    div()
        .font_family(DISPLAY_800)
        .text_size(px(21.))
        .line_height(px(25.))
        .child(label.into())
}
pub fn log_count(label: impl Into<SharedString>) -> Div {
    div()
        .text_size(px(12.))
        .line_height(px(14.))
        .text_color(rgb(FAINT))
        .child(label.into())
}
pub fn log_actions() -> Div {
    div().ml_auto().flex().gap(px(8.)).flex_shrink_0()
}
pub fn log_action(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    crate::small_ghost_pill(id, label).line_height(px(14.))
}
pub fn log_session(
    label: impl Into<SharedString>,
    options: &[String],
    w: &Window,
) -> Stateful<Div> {
    let label: SharedString = label.into();
    let widest = options
        .iter()
        .map(|option| {
            let label: SharedString = option.clone().into();
            w.text_system()
                .shape_line(
                    label.clone(),
                    px(13.),
                    &[TextRun {
                        len: label.len(),
                        font: Font {
                            family: DISPLAY_400.into(),
                            ..Default::default()
                        },
                        color: rgb(TEXT).into(),
                        background_color: None,
                        underline: None,
                        strikethrough: None,
                    }],
                    None,
                )
                .width
        })
        .fold(px(0.), |max, width| max.max(width));
    crate::select_field("log-session", label)
        .w(widest + px(50.))
        .h(px(26.))
        .py(px(4.))
}
pub fn log_popup() -> Stateful<Div> {
    crate::scrolling_select_popup("log-session-popup")
        .top(px(29.))
        .left_0()
        .w(px(320.))
}
pub fn log_option(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    selected: bool,
) -> Stateful<Div> {
    crate::select_option(id, label, selected)
}
pub fn log_body_surface() -> Div {
    div()
        .relative()
        .flex_1()
        .min_h_0()
        .rounded(px(10.))
        .border_1()
        .border_color(rgba(LINE))
        .bg(rgb(PANEL_DEEP))
}
pub fn log_body(scroll: &ScrollHandle, row_height: f32) -> Stateful<Div> {
    div()
        .id("log-body")
        .size_full()
        .overflow_y_scroll()
        .track_scroll(scroll)
        .px(px(12.))
        .py(px(10.))
        .font_family(MONO_400)
        .text_size(px(11.))
        .line_height(px(row_height))
        .cursor(CursorStyle::IBeam)
}
pub fn log_empty(loading: bool) -> Div {
    log_empty_label(if loading {"Loading…"} else {"Nothing logged yet."})
}
pub fn log_empty_label(label: impl Into<SharedString>) -> Div {
    div().py(px(8.)).px(px(2.)).text_color(rgb(FAINT)).child(label.into())
}

pub fn log_row(
    index: usize,
    time: String,
    source: String,
    message: String,
    level: &str,
    row_height: f32,
) -> Stateful<Div> {
    div()
        .id(("log-row", index))
        .flex()
        .gap(px(10.))
        .min_h(px(row_height))
        .line_height(px(row_height))
        .child(div().flex_shrink_0().text_color(rgb(FAINT)).child(
            SelectableText::new(("log-time", index), time).document_order((index * 3) as u64),
        ))
        .child(
            div()
                .flex_shrink_0()
                .min_w(px(52.))
                .text_color(rgb(DIM))
                .child(
                    SelectableText::new(("log-source", index), source)
                        .document_order((index * 3 + 1) as u64),
                ),
        )
        .child(
            div()
                .min_w_0()
                .text_color(rgb(match level {
                    "warn" => ACCENT,
                    "error" => DANGER,
                    _ => TEXT,
                }))
                .child(
                    SelectableText::new(("log-message", index), message)
                        .document_order((index * 3 + 2) as u64),
                ),
        )
}
pub fn log_saved(path: impl Into<SharedString>) -> Div {
    let path: SharedString = path.into();
    log_saved_label(format!("Saved to {path}"))
}
pub fn log_saved_label(label: impl Into<SharedString>) -> Div {
    div()
        .mt(px(10.))
        .font_family(DISPLAY_400)
        .text_size(px(12.))
        .line_height(px(19.8))
        .text_color(rgb(FAINT))
        .child(label.into())
}
pub fn log_entrance(card: Stateful<Div>, progress: f32) -> Stateful<Div> {
    crate::project_picker_entrance(card, progress)
}
