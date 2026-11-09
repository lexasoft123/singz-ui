//! Source-backed training control variants, shared by native desktop screens.
use crate::tokens::*;
use gpui_kit::*;
use gpui_kit::prelude::FluentBuilder;
pub fn training_card() -> Div {
    div()
        .border_1()
        .border_color(rgba(0xfff0d62a))
        .rounded(px(18.))
        .bg(rgba(0x1b1814b8))
}
pub fn training_program_card() -> Div {
    div()
        .p(px(24.))
        .border_1()
        .border_color(rgba(LINE))
        .rounded(px(20.))
        .bg(rgb(PANEL))
}
pub fn training_segment() -> Div {
    div()
        .flex()
        .border_1()
        .border_color(rgba(LINE_STRONG))
        .rounded(px(8.))
        .overflow_hidden()
}
pub fn training_choice(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    active: bool,
) -> Stateful<Div> {
    let label: SharedString = label.into();
    let id: ElementId = id.into();
    div()
        .id(id.clone())
        .role(Role::Button).accessibility_id(id.to_string()).aria_label(label.clone()).aria_toggled(if active {Toggled::True} else {Toggled::False})
        .flex_1()
        .min_w_0()
        .min_h(px(34.))
        .px(px(9.))
        .py(px(5.))
        .flex()
        .items_center()
        .justify_center()
        .text_size(px(12.))
        .line_height(px(18.))
        .font_family(DISPLAY_650)
        .text_color(rgb(if active { ACCENT } else { DIM }))
        .bg(rgba(if active { ACCENT_SOFT } else { 0 }))
        .cursor_pointer()
        .child(label)
}
pub fn training_select(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    let label: SharedString = label.into();
    let id: ElementId = id.into();
    div()
        .id(id.clone())
        .role(Role::Button).accessibility_id(id.to_string()).aria_label(label.clone())
        .min_h(px(34.))
        .px(px(10.))
        .border_1()
        .border_color(rgba(LINE_STRONG))
        .rounded(px(8.))
        .bg(rgb(PANEL_DEEP))
        .flex()
        .items_center()
        .justify_between()
        .text_size(px(12.))
        .line_height(px(16.))
        .font_family(DISPLAY_600)
        .text_color(rgb(TEXT))
        .cursor_pointer()
        .child(label)
        .child(crate::colored_icon(crate::TransportIcon::Chevron, TEXT))
}
pub fn training_back(id: impl Into<ElementId>) -> Stateful<Div> {
    training_back_label(id, "← Training")
}
pub fn training_back_label(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    let label: SharedString = label.into();
    let id: ElementId = id.into();
    div()
        .id(id.clone())
        .role(Role::Button).accessibility_id(id.to_string()).aria_label(label.clone())
        .text_size(px(12.))
        .line_height(px(14.5))
        .text_color(rgb(DIM))
        .cursor_pointer()
        .hover(|s| s.text_color(rgb(TEXT)))
        .child(label)
}
pub fn training_eyebrow(text: impl Into<SharedString>) -> Div {
    div()
        .text_size(px(12.))
        .line_height(px(14.5))
        .font_family("SingZ Mono 600")
        .text_color(rgb(ACCENT))
        .child(crate::tracked_label(text, 0.1))
}
/// Native counterpart of the shared Banner primitive.
pub fn training_banner() -> Div {
    div()
        .w_full()
        .max_w(px(720.))
        .mx_auto()
        .min_w_0()
        .px(px(14.))
        .py(px(10.))
        .border_1()
        .border_color(rgba(LINE_STRONG))
        .rounded(px(14.))
        .bg(Rgba {
            r: (1.0 * (0x21 as f32 / 255.) * 0.52 + (27. / 255.) * 0.48)
                / ((0x21 as f32 / 255.) * 0.52 + 0.48),
            g: ((160. / 255.) * (0x21 as f32 / 255.) * 0.52 + (24. / 255.) * 0.48)
                / ((0x21 as f32 / 255.) * 0.52 + 0.48),
            b: ((40. / 255.) * (0x21 as f32 / 255.) * 0.52 + (20. / 255.) * 0.48)
                / ((0x21 as f32 / 255.) * 0.52 + 0.48),
            a: (0x21 as f32 / 255.) * 0.52 + 0.48,
        })
        .text_center()
}

/// Unframed default web pill used by program-level choices.
pub fn training_program_choice(
    id: impl Into<ElementId>,
    text: impl Into<SharedString>,
) -> Stateful<Div> {
    let text: SharedString = text.into();
    let id: ElementId = id.into();
    div()
        .id(id.clone())
        .role(Role::Button).accessibility_id(id.to_string()).aria_label(text.clone())
        .px(px(16.))
        .py(px(9.))
        .rounded_full()
        .font_family(DISPLAY_600)
        .text_size(px(13.))
        .line_height(px(15.5))
        .cursor_pointer()
        .child(crate::tracked_label(text, 0.01))
}
/// Source primary action; disabled state stays separate from the button geometry.
pub fn training_primary(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Stateful<Div> {
    training_program_choice(id, text)
        .flex()
        .items_center()
        .justify_center()
        .text_color(rgb(ACCENT_INK))
        .bg(linear_gradient(
            180.,
            linear_color_stop(rgb(ACCENT_LIFT), 0.),
            linear_color_stop(rgb(ACCENT_DEEP), 1.),
        ))
        .shadow(vec![crate::glow(0xff9628, 0.28, 22.)])
}

/// Controlled primary action; event admission remains with the owning view.
pub fn training_primary_enabled(id: impl Into<ElementId>, text: impl Into<SharedString>, enabled: bool) -> Stateful<Div> {
    crate::disabled_control(training_primary(id, text), enabled).when(!enabled, |button| button.opacity(0.45).cursor_default())
}

pub fn training_range(
    id: impl Into<ElementId>,
    value: f32,
    min: f32,
    max: f32,
    on_change: impl Fn(f32, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    crate::controls::range_slider_style(id, value, min, max, on_change, true)
}
/// Training's existing range painter with persistent focus and stepped keys.
/// The live value accumulates multiple inputs before the next render.
pub fn training_range_with_focus(
    id: impl Into<ElementId>, value: f32, min: f32, max: f32, step: f32,
    focus: FocusHandle, enabled: bool,
    on_change: impl Fn(f32, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    use std::{cell::Cell, rc::Rc};
    let live = Rc::new(Cell::new(value));
    let pointer_live = live.clone();
    let callback = Rc::new(on_change);
    let pointer_callback = callback.clone();
    let element = crate::disabled_control(crate::controls::range_slider_step_with_live(id, value, min, max, step, live.clone(), move |next, w, cx| {
        if enabled {
            let next = crate::controls::settings_range_value(next, min, max, step);
            pointer_live.set(next);
            pointer_callback(next, w, cx);
        }
    }, true), enabled);
    if !enabled { return element; }
    let pointer_focus = focus.clone();
    let key_focus = focus.clone();
    crate::lyrics_sidebar_focus(element, focus.clone())
        .track_focus(&focus).tab_index(0)
        .on_mouse_down(MouseButton::Left, move |_, w, cx| {
            pointer_focus.focus(w, cx);
            w.prevent_default();
        })
        .capture_key_down(move |event, w, cx| {
            if !key_focus.is_focused(w) || event.keystroke.modifiers.modified() { return; }
            let before = live.get();
            if let Some(next) = crate::controls::settings_range_key(before, &event.keystroke.key, min, max, step) {
                live.set(next);
                if next != before { callback(next, w, cx); }
                w.prevent_default();
                cx.stop_propagation();
            }
        })
}
pub fn training_round_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
) -> Stateful<Div> {
    let label: SharedString = label.into();
    let id: ElementId = id.into();
    div()
        .id(id.clone())
        .role(Role::Button).accessibility_id(id.to_string()).aria_label(label.clone())
        .size(px(38.))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .border_1()
        .border_color(rgba(LINE_STRONG))
        .bg(rgba(0x0f0d0ac2))
        .text_color(rgb(TEXT))
        .text_size(px(22.))
        .line_height(px(26.5))
        .cursor_pointer()
        .child(label)
}

pub fn training_toggle(
    id: impl Into<ElementId>,
    short: impl Into<SharedString>,
    label: impl Into<SharedString>,
    on: bool,
) -> Stateful<Div> {
    let label: SharedString = label.into();
    let id: ElementId = id.into();
    div()
        .id(id.clone())
        .role(Role::Button).accessibility_id(id.to_string()).aria_label(label.clone()).aria_toggled(if on {Toggled::True} else {Toggled::False})
        .flex_1()
        .min_w(px(74.))
        .min_h(px(58.))
        .px(px(8.))
        .py(px(9.))
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(4.))
        .border_1()
        .border_color(rgba(if on { ACCENT_LINE } else { LINE }))
        .rounded(px(8.))
        .bg(rgba(if on { ACCENT_SOFT } else { 0 }))
        .cursor_pointer()
        .child(
            div()
                .font_family("SingZ Mono 650")
                .text_size(px(13.))
                .line_height(px(15.5))
                .text_color(rgb(if on { ACCENT } else { DIM }))
                .child(short.into()),
        )
        .child(
            div()
                .text_size(px(12.))
                .line_height(px(14.5))
                .text_color(rgb(DIM))
                .truncate()
                .child(label),
        )
}

/// Source training session and transport surfaces, shared native controls.
pub fn training_session_head() -> Div {
 div().w_full().max_w(px(1040.)).mx_auto().px(px(12.)).py(px(8.)).flex().items_center().justify_between().border_1().border_color(rgba(LINE_STRONG)).rounded(px(22.)).bg(rgba(0x1b1814b8))
}
pub fn training_runway(cents: Option<f32>, tolerance: f32, height: f32) -> Div {
 let zone=(tolerance/200.).max(0.05);
 div().relative().w_full().h(px(height)).overflow_hidden().border_1().border_color(rgba(LINE_STRONG)).rounded(px(26.)).bg(rgba(0x1b1814c2))
 .child(div().absolute().top_0().bottom_0().left(relative(0.5-zone)).w(relative(zone*2.)).bg(rgba(0xffa0281b)))
 .child(div().absolute().top_0().bottom_0().left(relative(0.5)).w(px(2.)).bg(rgb(ACCENT)))
 .when_some(cents,|d,cents|d.child(div().absolute().top(relative(0.5)).left(relative(0.5+cents.clamp(-100.,100.)/200.)).ml(px(-22.)).mt(px(-22.)).size(px(44.)).border(px(5.)).border_color(rgb(BG)).rounded_full().bg(rgb(ACCENT))))
}
pub fn training_hold_meter(progress:f32)->Div {
 div().h(px(8.)).w_full().rounded_full().overflow_hidden().bg(rgba(LINE_STRONG)).child(div().h_full().w(relative(progress.clamp(0.,1.))).rounded_full().bg(rgb(ACCENT)))
}
pub fn training_answer(id:impl Into<ElementId>)->Stateful<Div>{
 div().id(id).role(Role::Button).min_h(px(52.)).p(px(8.)).flex().flex_col().justify_center().gap(px(3.)).text_center().rounded(px(8.)).border_1().border_color(rgba(LINE_STRONG)).bg(rgb(PANEL)).cursor_pointer().hover(|s|s.border_color(rgb(ACCENT)).bg(rgba(ACCENT_SOFT)))
}
pub fn training_transport_dock()->Div {
 div().w(px(360.)).max_w_full().min_h(px(88.)).mx_auto().px(px(10.)).py(px(8.)).flex().items_center().justify_center().gap(px(4.)).border_1().border_color(rgba(LINE_STRONG)).rounded(px(28.)).bg(rgba(0x1b1814c2))
}

/// Shared source summary/progress strip: five equally sized metric columns.
pub fn training_summary_strip() -> Div {
    div().flex().border_1().border_color(rgba(0xfff0d62a)).bg(rgba(0x1b1814b8))
}
pub fn training_metric(label:impl Into<SharedString>,value:impl Into<SharedString>,first:bool) -> Div {
    div().flex_1().min_w_0().px(px(14.)).py(px(18.)).flex().flex_col().gap(px(7.))
        .when(!first,|d|d.border_l_1().border_color(rgba(LINE)))
        .child(div().font_family(MONO_400).text_size(px(12.)).line_height(px(18.)).text_color(rgb(DIM)).child(label.into()))
        .child(div().font_family("SingZ Mono 650").text_size(px(20.)).line_height(px(30.)).text_color(rgb(TEXT)).child(value.into()))
}
pub fn training_history_section() -> Div {
    div().mt(px(24.)).border_1().border_color(rgba(0xfff0d62a)).bg(rgba(0x1b1814b8))
}

/// Source loaded-song preparation panel and four recipe buttons.
pub fn training_song_card() -> Div {
    div().mb(px(34.)).py(px(20.)).px(px(22.)).border_1().border_l(px(3.))
        .border_color(rgba(0xffab3f64)).relative().overflow_hidden()
        .child(div().absolute().left_0().top_0().bottom_0().w(px(3.)).bg(rgb(ACCENT)))
        .rounded_l(px(4.)).rounded_r(px(12.))
        .bg(linear_gradient(110.,linear_color_stop(rgba(0xffa02814),0.),linear_color_stop(rgba(0x0f0d0ab8),1.)))
}
pub fn training_song_choice(id:impl Into<ElementId>,label:impl Into<SharedString>,enabled:bool) -> Stateful<Div> {
    let label: SharedString = label.into();
    let id: ElementId = id.into();
    crate::disabled_control(div().id(id.clone()).role(Role::Button).accessibility_id(id.to_string()).aria_label(label.clone()), enabled).flex_1().min_w_0().min_h(px(36.)).py(px(6.)).px(px(8.)).border_1()
        .border_color(rgba(LINE_STRONG)).rounded(px(8.)).bg(rgba(0x1b1814a6))
        .font_family("SingZ Display 680").text_size(px(12.)).line_height(px(18.)).text_color(rgb(TEXT))
        .flex().items_center().justify_center().cursor_pointer()
        .when(!enabled,|d|d.opacity(0.5).cursor_default())
        .when(enabled,|d|d.hover(|s|s.border_color(rgba(ACCENT_LINE)).bg(rgba(ACCENT_SOFT))))
        .child(label)
}
