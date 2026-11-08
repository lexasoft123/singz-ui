//! Production TrainPopover controls; native scheduling belongs to the host.
use crate::tokens::*;
use gpui_kit::{prelude::FluentBuilder, *};
pub fn carry_line_card() -> Stateful<Div> {
    crate::transport_popover()
        .id("carry-card")
        .w(px(294.))
        .p(px(14.))
        .flex()
        .flex_col()
        .min_h_0()
        .overflow_hidden()
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(14.))
        .text_color(rgb(TEXT))
}
/// Padding belongs to the fixed chrome. Keep the real scrolling viewport
/// unpadded: GPUI counts absolute probe/scrollbar bounds as content, so padded
/// scrolling would double-count padding and invent overflow on both axes.
/// Scrolling is a native overflow safeguard; fitting content never scrolls.
pub fn carry_line_content(scroll: &ScrollHandle) -> Stateful<Div> {
    div()
        .id("carry-content")
        .relative()
        .w_full()
        .min_h_0()
        .flex_shrink_1()
        .p(px(0.))
        .overflow_y_scroll()
        .track_scroll(scroll)
        .child(crate::production_scrollbar(scroll))
}
pub fn carry_line_head() -> Div {
    carry_line_head_label("Carry the line")
}
pub fn carry_line_head_label(label: impl Into<SharedString>) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .mb(px(10.))
        .child(
            div()
                .text_size(px(13.))
                .font_family(DISPLAY_700)
                .child(label.into()),
        )
}
pub fn carry_line_mode() -> Div {
    div().flex().w_full().mb(px(10.))
}
pub fn carry_line_row() -> Div {
    div().flex().items_center().gap(px(7.)).mb(px(2.))
}
pub fn carry_line_label(text: impl Into<SharedString>) -> Div {
    div()
        .text_size(px(12.))
        .text_color(rgb(DIM))
        .child(text.into())
}
pub fn carry_line_number(text: impl Into<SharedString>) -> Div {
    div()
        .min_w(px(34.))
        .text_center()
        .text_size(px(13.))
        .font_family(DISPLAY_650)
        .child(text.into())
}
pub fn carry_line_caption(text: impl Into<SharedString>) -> Div {
    div()
        .mt(px(8.))
        .mx(px(2.))
        .mb(px(10.))
        .text_size(px(12.))
        .line_height(px(17.))
        .text_color(rgb(FAINT))
        .child(text.into())
}
pub fn carry_line_lanes() -> Div {
    carry_line_lanes_label("Muted while you sing:")
}
pub fn carry_line_lanes_label(label: impl Into<SharedString>) -> Div {
    div()
        .flex()
        .flex_wrap()
        .items_center()
        .gap(px(6.))
        .child(carry_line_label(label))
}
pub fn carry_line_lane(
    id: impl Into<ElementId>,
    text: impl Into<SharedString>,
    selected: bool,
    enabled: bool,
) -> Stateful<Div> {
    let text: SharedString = text.into();
    let tooltip = text.clone();
    crate::controls::transport_chip_surface(id, "")
        .child(
            div()
                .min_w_0()
                .max_w_full()
                .overflow_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .child(text),
        )
        .tooltip(move |w, cx| {
            gpui_kit::component::tooltip::Tooltip::new(tooltip.clone())
                .max_w(px(420.))
                .font_family(DISPLAY_400)
                .text_size(px(12.))
                .build(w, cx)
        })
        .w_auto()
        .max_w_full()
        .px(px(10.))
        .h(px(26.))
        .font_family(DISPLAY_600)
        .text_size(px(11.))
        .when(selected, |d| {
            d.text_color(rgb(ACCENT))
                .border_color(rgba(ACCENT_LINE))
                .bg(rgba(ACCENT_SOFT))
        })
        .when(enabled, |d| {
            d.hover(move |s| {
                s.text_color(rgb(if selected { ACCENT } else { TEXT }))
                    .border_color(rgba(if selected { ACCENT_LINE } else { LINE_STRONG }))
            })
        })
        .when(!enabled, |d| d.opacity(0.45).cursor_default())
}
pub fn carry_line_step(
    id: impl Into<ElementId>,
    text: impl Into<SharedString>,
    enabled: bool,
) -> Stateful<Div> {
    crate::controls::transport_chip_surface(id, text)
        .h(px(24.))
        .text_size(px(11.))
        .font_family(DISPLAY_800)
        .line_height(px(13.2))
        .when(enabled, |d| {
            d.hover(|s| s.text_color(rgb(TEXT)).border_color(rgba(LINE_STRONG)))
        })
        .when(!enabled, |d| d.opacity(0.45).cursor_default())
}
pub fn carry_line_trigger(id: impl Into<ElementId>, enabled: bool, ducking: bool) -> Stateful<Div> {
    if !enabled {
        return crate::round_button(id, false);
    }
    let button = crate::controls::round_button_surface(id, true)
        .shadow(vec![crate::glow(ACCENT, 0.15, 14.)]);
    if ducking {
        button
            .text_color(rgb(BG))
            .bg(rgb(ACCENT))
            .border_color(rgb(ACCENT_DEEP))
            .hover(|s| {
                s.text_color(rgb(BG))
                    .bg(rgb(ACCENT))
                    .border_color(rgb(ACCENT_DEEP))
            })
    } else {
        button.hover(|s| s.text_color(rgb(ACCENT)).border_color(rgba(ACCENT_LINE)))
    }
}
