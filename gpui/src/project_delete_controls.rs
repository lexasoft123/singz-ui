//! Source DropScreen delete confirmation: presentation only, no filesystem work.
use crate::tokens::*;
use gpui_kit::{prelude::FluentBuilder, *};
pub fn project_delete_card(viewport: Size<Pixels>) -> Div {
    crate::modal_card().w(px(420_f32.min((f32::from(viewport.width) - 60.).max(0.))))
}
pub fn project_delete_title(name: impl AsRef<str>, w: &Window) -> Div {
    crate::project_picker_title(format!("Delete “{}”?", name.as_ref()), w).mb(px(12.))
}
pub fn project_delete_body(text: impl Into<SharedString>) -> Div {
    div()
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(13.5))
        .line_height(px(22.275))
        .text_color(rgb(DIM))
        .mb(px(14.))
        .child(text.into())
}
pub fn project_delete_note(text: impl Into<SharedString>) -> Div {
    div()
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(12.))
        .line_height(px(19.8))
        .text_color(rgb(FAINT))
        .mb(px(14.))
        .child(text.into())
}
pub fn project_delete_actions() -> Div {
    div().flex().flex_wrap().gap(px(8.)).mt(px(18.))
}
pub fn project_delete_action(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    danger: bool,
    enabled: bool,
) -> Stateful<Div> {
    if !danger {
        return crate::lyrics_editor_button(id, label, false, enabled);
    }
    // Build the shared small-pill geometry without its ghost hover. GPUI
    // permits one hover refinement; danger supplies that complete refinement
    // rather than attempting to layer another hover over the ghost variant.
    crate::lyrics_editor_button(id, label, false, false)
        .text_color(rgb(0xff8a7a))
        .border_color(rgba(0xff8a7a66))
        .when(enabled, |d| {
            d.opacity(1.).cursor_pointer().hover(|s| {
                s.top(px(-1.))
                    .bg(rgba(0xff8a7a1f))
                    .border_color(rgb(0xff8a7a))
            })
        })
}
pub fn project_delete_entrance(card: Stateful<Div>, progress: f32) -> Stateful<Div> {
    crate::project_picker_entrance(card, progress)
}

/// Source `.src-row.sub .src-card-value.warn`, also shown for offline errors.
pub fn library_delete_error(message: impl Into<SharedString>, w: &Window) -> Div {
    div()
        .w_full()
        .min_w_0()
        .border_t_1()
        .border_color(rgba(0xffffff0d))
        .px(px(14.))
        .py(px(8.))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(12.))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_400, 12.).0)
        .text_color(rgb(ACCENT))
        .opacity(0.85)
        .child(message.into())
}

pub fn project_delete_title_label(label:impl Into<SharedString>,w:&Window)->Div {
    crate::project_picker_title(label,w).mb(px(12.))
}
