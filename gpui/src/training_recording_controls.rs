//! Source `.vt-recording`: deliberately bare buttons, not pills.
use crate::tokens::*;
use gpui_kit::*;
pub fn training_recording_bar() -> Div {
    div()
        .text_left()
        .flex()
        .flex_wrap()
        .flex_none()
        .items_center()
        .justify_center()
        .gap(px(8.))
        .py(px(8.))
        .px(px(16.))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(14.))
        .text_color(rgb(TEXT))
}
pub fn training_recording_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    focus: Option<FocusHandle>,
    w: &Window,
) -> Stateful<Div> {
    let mut button = div()
        .id(id)
        .relative()
        .flex_none()
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(14.))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_400, 14.).0)
        .text_color(rgb(TEXT))
        .cursor_pointer()
        .child(label.into());
    if let Some(focus) = focus {
        button = button.track_focus(&focus).child(
            canvas(
                |b, _, _| b,
                move |b, _, w, _| {
                    if focus.is_focused(w) && w.last_input_was_keyboard() {
                        w.paint_quad(quad(
                            Bounds::new(
                                b.origin - point(px(4.), px(4.)),
                                b.size + size(px(8.), px(8.)),
                            ),
                            px(0.),
                            transparent_black(),
                            px(2.),
                            rgb(ACCENT),
                            BorderStyle::Solid,
                        ));
                    }
                },
            )
            .absolute()
            .inset_0(),
        );
    }
    button
}
pub fn training_recording_status(message: impl Into<SharedString>, w: &Window) -> Stateful<Div> {
    div()
        .id("training-recording-status")
        .w_full()
        .min_w_0()
        .flex_none()
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(12.))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_400, 12.).0)
        .text_color(rgb(TEXT))
        .child(message.into())
}

/// The session's bare text uses CSS `normal`, unlike descriptive copy's 1.5.
pub fn training_session_text(
    text: impl Into<SharedString>,
    family: &str,
    size: f32,
    color: u32,
    w: &Window,
) -> Div {
    div()
        .font_family(family.to_owned())
        .font_weight(FontWeight::NORMAL)
        .text_size(px(size))
        .line_height(crate::css_normal_line_metrics(w, family, size).0)
        .text_color(rgb(color))
        .child(text.into())
}
pub fn training_session_responsive_head(viewport: Size<Pixels>) -> Div {
    let compact = f32::from(viewport.height) <= 680.;
    crate::training_session_head()
        .flex_none()
        .w(px(1040_f32.min((f32::from(viewport.width) - 48.).max(0.))))
        .px(px(if compact { 18. } else { 12. }))
        .rounded(px(if compact { 18. } else { 22. }))
}
pub fn training_session_responsive_dock(compact: bool) -> Div {
    crate::training_transport_dock()
        .flex_none()
        .w(px(if compact { 340. } else { 360. }))
        .min_h(px(if compact { 72. } else { 88. }))
        .px(px(if compact { 8. } else { 10. }))
        .py(px(if compact { 4. } else { 8. }))
        .rounded(px(if compact { 24. } else { 28. }))
}
pub fn training_session_transport_action(id: impl Into<ElementId>, compact: bool) -> Stateful<Div> {
    div()
        .id(id)
        .w(px(82.))
        .min_h(px(if compact { 62. } else { 68. }))
        .flex_none()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(4.))
        .rounded(px(20.))
        .text_color(rgb(DIM))
        .cursor_pointer()
        .hover(|s| s.text_color(rgb(TEXT)).bg(rgba(ACCENT_SOFT)))
}
pub fn training_session_transport_status(compact: bool) -> Div {
    div()
        .w(px(104.))
        .min_h(px(if compact { 62. } else { 72. }))
        .flex_none()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(4.))
}
pub fn training_session_transport_label(label: impl Into<SharedString>, color: u32) -> Div {
    div()
        .font_family(DISPLAY_650)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(12.))
        .line_height(px(12.))
        .text_color(rgb(color))
        .child(label.into())
}
