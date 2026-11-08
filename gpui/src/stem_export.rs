//! Production StemExport.tsx presentation, composed from shared native controls.
use crate::tokens::*;
use gpui_kit::{prelude::FluentBuilder, *};

/// Fixed card chrome; the inner content owns scrolling so bounds never move
/// with its content offset.
pub fn stem_export_frame(w: &Window) -> Stateful<Div> {
    let viewport = w.viewport_size();
    crate::modal_card()
        .id("export-card")
        .p(px(0.))
        .gap(px(0.))
        .w(px(460_f32.min(f32::from(viewport.width) * 0.9)))
        .max_h(px(f32::from(viewport.height) * 0.85))
        .min_h_0()
        .border_color(rgba(LINE))
        .rounded(px(16.))
        .bg(rgb(BG))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(14.))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_400, 14.).0)
        .text_color(rgb(TEXT))
        .overflow_hidden()
}
pub fn stem_export_card(_w: &Window, scroll: &ScrollHandle) -> Stateful<Div> {
    div()
        .id("export-content")
        .relative()
        .flex()
        .flex_col()
        .gap(px(20.))
        .p(px(24.))
        .w_full()
        .min_h_0()
        .flex_shrink_1()
        .overflow_y_scroll()
        .track_scroll(scroll)
        .child(crate::production_scrollbar(scroll))
}
pub fn stem_export_title(w: &Window) -> Div {
    div()
        .flex_none()
        .font_family(DISPLAY_700)
        .text_size(px(21.))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_700, 21.).0)
        .child("Export tracks")
}
pub fn stem_export_paragraph(text: impl Into<SharedString>, note: bool) -> Div {
    let size = if note { 13. } else { 14. };
    div()
        .flex_none()
        .min_w_0()
        .text_size(px(size))
        .line_height(px(size * 1.5))
        .text_color(rgb(DIM))
        .child(text.into())
}
pub fn stem_export_control(label: impl Into<SharedString>, w: &Window) -> Div {
    div().flex_none().flex().flex_col().gap(px(8.)).child(
        div()
            .font_family(DISPLAY_600)
            .text_size(px(13.))
            .line_height(crate::css_normal_line_metrics(w, DISPLAY_600, 13.).0)
            .child(label.into()),
    )
}
pub fn stem_export_segment(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    selected: bool,
    enabled: bool,
) -> Stateful<Div> {
    crate::mode_button(id, label, selected)
        .flex_1()
        .flex()
        .justify_center()
        .when(!enabled, |d| d.opacity(0.45).cursor_default())
}
pub fn stem_export_selection() -> Div {
    div()
        .flex_none()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(div().child("Choose tracks"))
}
pub fn stem_export_row() -> Div {
    div().flex().gap(px(20.)).items_start()
}
pub fn stem_export_track(
    id: String,
    label: impl Into<SharedString>,
    checkbox: Stateful<Div>,
    enabled: bool,
) -> Stateful<Div> {
    // The label owns the click; the shared checkbox's event stopper is replaced
    // by the host callback on that checkbox as well.
    div()
        .id(id.clone())
        .relative()
        .flex_1()
        .min_w_0()
        .flex()
        .items_center()
        .gap(px(10.))
        .when(enabled, |d| d.cursor_pointer())
        .when(!enabled, |d| d.opacity(0.45))
        .child(checkbox)
        .child(
            div()
                .flex_1()
                .min_w_0()
                .whitespace_normal()
                .child(label.into()),
        )
}
pub fn stem_export_action(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    primary: bool,
    enabled: bool,
    w: &Window,
) -> Stateful<Div> {
    div()
        .id(id)
        .relative()
        .px(px(12.))
        .py(px(8.))
        .border_1()
        .border_color(rgba(LINE))
        .rounded(px(8.))
        .bg(rgb(if primary { ACCENT } else { BG }))
        .text_color(rgb(if primary { 0x20180b } else { TEXT }))
        .font_family(DISPLAY_400)
        .text_size(px(14.))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_400, 14.).0)
        .when(enabled, |d| d.cursor_pointer())
        .when(!enabled, |d| d.opacity(0.45))
        .child(label.into())
}

pub fn stem_export_title_label(label:impl Into<SharedString>,w:&Window)->Div {
    div().flex_none().font_family(DISPLAY_700).text_size(px(21.))
        .line_height(crate::css_normal_line_metrics(w,DISPLAY_700,21.).0).child(label.into())
}
pub fn stem_export_selection_label(label:impl Into<SharedString>)->Div {
    div().flex_none().flex().flex_col().gap(px(12.)).child(div().child(label.into()))
}
