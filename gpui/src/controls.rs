use crate::tokens::*;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::*;

/// Represent an isolated CSS Canvas fill pixel after its premultiplied RGBA8
/// backing-store conversion. Use only for source Canvas fills; DOM colours keep
/// their ordinary float blend semantics. Alpha is quantized before RGB premul,
/// then RGB is unpremultiplied for GPUI's straight-colour paint interface.
/// Verified against 99 real Chromium150 Canvas colour/alpha/backdrop cases.
pub fn canvas_color(color: Hsla) -> Hsla {
    let color: Rgba = color.into();
    let alpha_byte = (color.a.clamp(0., 1.) * 255.).round();
    if alpha_byte == 0. {
        return rgba(0x00000000).into();
    }
    let channel = |value: f32| (value.clamp(0., 1.) * alpha_byte).round() / alpha_byte;
    Rgba {
        r: channel(color.r),
        g: channel(color.g),
        b: channel(color.b),
        a: alpha_byte / 255.,
    }
    .into()
}

/// A CSS box-shadow blur radius maps to Gaussian sigma = radius / 2.
/// GPUI's Metal shadow kernel consumes sigma directly (not CSS blur radius).
/// https://www.w3.org/TR/css-backgrounds-3/#shadow-blur
pub fn glow(color: u32, alpha: f32, radius: f32) -> BoxShadow {
    BoxShadow {
        color: rgb(color).alpha(alpha).into(),
        offset: point(px(0.), px(0.)),
        blur_radius: px(radius * 0.5),
        spread_radius: px(0.),
        inset: false,
    }
}

/// The original native player's one-pixel inset highlight.
pub fn rim() -> BoxShadow {
    BoxShadow {
        color: rgba(0xfff0dc20).into(),
        offset: point(px(0.), px(1.)),
        blur_radius: px(0.),
        spread_radius: px(0.),
        inset: true,
    }
}

/// Compact native pill, preserving the player prototype's 32px geometry.
/// Add application handlers with `.on_click(...)`; event propagation is
/// stopped so the enclosing frameless titlebar cannot begin a drag.
pub fn pill(id: impl Into<ElementId>, label: impl Into<SharedString>, on: bool) -> Stateful<Div> {
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_center()
        .justify_center()
        .px_3()
        .h(px(32.))
        .rounded_full()
        .border_1()
        .border_color(rgba(if on { ACCENT_LINE } else { CONTROL_LINE }))
        .bg(rgba(if on { ACCENT_SOFT } else { CONTROL_FILL }))
        .text_color(rgb(if on { ACCENT } else { DIM }))
        .text_size(px(12.))
        .font_family(DISPLAY_600)
        .font_weight(FontWeight::NORMAL)
        .shadow(vec![rim()])
        .cursor_pointer()
        .hover(|s| s.border_color(rgba(0xfff0dc55)).text_color(rgb(TEXT)))
        .active(|s| s.bg(rgba(0xffa02830)))
        .child(label.into())
}

/// Padded raised content group. Children determine its size.
pub fn card() -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(12.))
        .p(px(18.))
        .rounded(px(14.))
        .border_1()
        .border_color(rgba(LINE))
        .bg(rgb(PANEL))
        .text_color(rgb(TEXT))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .shadow(vec![rim()])
}

/// The shared eyebrow type style (10px / 700 / 0.14em tracking).
pub fn section_heading(label: impl Into<SharedString>) -> Div {
    let label: SharedString = label.into();
    div()
        .text_size(px(10.))
        .font_family(DISPLAY_700)
        .font_weight(FontWeight::NORMAL)
        .text_color(rgb(DIM))
        .child(crate::tracked_text(
            label.to_uppercase(),
            DISPLAY_700,
            10.,
            1.4,
            DIM,
        ))
}

/// Binary switch; state remains owned by the host, just like a controlled pill.
pub fn toggle(id: impl Into<ElementId>, on: bool) -> Stateful<Div> {
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_center()
        .w(px(34.))
        .h(px(20.))
        .px(px(2.))
        .rounded_full()
        .border_1()
        .border_color(rgba(if on { ACCENT_LINE } else { LINE_STRONG }))
        .bg(rgba(if on { ACCENT_SOFT } else { CONTROL_FILL }))
        .cursor_pointer()
        .shadow(vec![rim()])
        .child(
            div()
                .ml(px(if on { 14. } else { 0. }))
                .w(px(14.))
                .h(px(14.))
                .rounded_full()
                .bg(rgb(if on { ACCENT } else { DIM })),
        )
}

/// Read-only uppercase status badge, matching the CSS 9.5px / 700 style.
pub fn badge(label: impl Into<SharedString>, on: bool) -> Div {
    let label: SharedString = label.into();
    div()
        .flex()
        .items_center()
        .px(px(7.))
        .py(px(2.))
        .rounded_full()
        .border_1()
        .border_color(rgba(if on { ACCENT_LINE } else { LINE_STRONG }))
        .text_color(rgb(if on { ACCENT } else { DIM }))
        .text_size(px(9.5))
        .font_family(DISPLAY_700)
        .font_weight(FontWeight::NORMAL)
        .child(crate::tracked_text(
            label.to_uppercase(),
            DISPLAY_700,
            9.5,
            0.76,
            if on { ACCENT } else { DIM },
        ))
}

/// Shared editable search field. The host owns InputState and subscriptions.
pub fn search_input(state: &Entity<InputState>) -> Input {
    Input::new(state)
        .h(px(36.))
        .rounded(px(8.))
        .border_1()
        .border_color(rgba(LINE_STRONG))
        .bg(rgb(PANEL_DEEP))
        .text_color(rgb(TEXT))
        .text_size(px(13.))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
}

/// Settings navigation preserves the production modal's 7px × 10px hit area.
pub fn nav_tab(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    on: bool,
) -> Stateful<Div> {
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .px(px(10.))
        .py(px(7.))
        .rounded(px(8.))
        .text_size(px(13.))
        .line_height(px(15.5))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_color(rgb(if on { TEXT } else { DIM }))
        .bg(rgba(if on { ACCENT_SOFT } else { 0x00000000 }))
        .cursor_pointer()
        .hover(|s| s.text_color(rgb(TEXT)))
        .child(label.into())
}

/// Display and hit target for a host-owned settings option picker.
/// The host supplies the menu behaviour in an attached click callback.
pub fn select_field(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    div()
        .relative()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_center()
        .justify_between()
        .w_full()
        .h(px(31.5))
        .line_height(px(15.5))
        .pl(px(12.))
        .pr(px(36.))
        .py(px(7.))
        .rounded(px(8.))
        .border_1()
        .border_color(rgba(LINE_STRONG))
        .bg(rgb(PANEL_DEEP))
        .text_color(rgb(TEXT))
        .text_size(px(13.))
        .whitespace_nowrap()
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .cursor_pointer()
        .hover(|s| s.border_color(rgb(ACCENT)))
        .child(label.into())
        .child(select_arrow())
}

/// Full-viewport modal material. GPUI 0.9 has no backdrop-filter API;
/// the host must provide a blurred snapshot behind this scrim when required.
pub fn modal_scrim() -> Div {
    div()
        .absolute()
        .inset_0()
        .flex()
        .items_center()
        .justify_center()
        .bg(rgba(0x0806039e))
        .occlude()
}

pub fn modal_card() -> Div {
    div()
        .flex()
        .flex_col()
        .gap_0()
        .p(px(28.))
        .rounded(px(18.))
        .border_1()
        .border_color(rgba(LINE_STRONG))
        .bg(rgb(SURFACE_RAISED))
        .text_color(rgb(TEXT))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .shadow(vec![BoxShadow {
            color: rgba(0x00000099).into(),
            offset: point(px(0.), px(30.)),
            blur_radius: px(40.), // CSS modal shadow blur 80px => sigma 40px
            spread_radius: px(0.),
            inset: false,
        }])
}

fn select_arrow() -> impl IntoElement {
    canvas(
        |b, _, _| b,
        |_, b, w, _| {
            let x = b.origin.x;
            let y = b.origin.y;
            let mut p = PathBuilder::fill();
            p.move_to(point(x, y));
            p.line_to(point(x + px(12.), y));
            p.line_to(point(x + px(6.), y + px(6.)));
            p.close();
            if let Ok(p) = p.build() {
                w.paint_path(p, rgb(TEXT));
            }
        },
    )
    .w(px(12.))
    .h(px(6.))
    // CSS background-position puts the 12×6 triangle one pixel above
    // centre, 12px from the outer edge (padding-box includes its border).
    .absolute()
    .right(px(11.))
    .top(relative(0.5))
    .mt(px(-4.))
}

pub fn checkbox(id: impl Into<ElementId>, on: bool) -> Stateful<Div> {
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .relative()
        .flex_none()
        .w(px(15.))
        .h(px(15.))
        .rounded(px(2.))
        .border_1()
        .border_color(rgb(if on { ACCENT } else { DIM }))
        .bg(if on { rgb(ACCENT) } else { rgb(PANEL_DEEP) })
        .cursor_pointer()
        .child(
            canvas(
                |b, _, _| b,
                move |_, b, w, _| {
                    if on {
                        // Chromium NativeThemeBase::PaintCheckbox: the mark is
                        // defined against the entire square, not its inner border.
                        // Main-source proportions match the reference mark bounds.
                        let mut p = PathBuilder::stroke(b.size.height * 0.16);
                        p.move_to(point(
                            b.left() + b.size.width * 0.2,
                            b.top() + b.size.height * 0.5,
                        ));
                        p.line_to(point(
                            b.left() + b.size.width * 0.4,
                            b.top() + b.size.height * 0.7,
                        ));
                        p.line_to(point(
                            b.left() + b.size.width * 0.8,
                            b.top() + b.size.height * 0.2,
                        ));
                        if let Ok(p) = p.build() {
                            w.paint_path(p, rgb(0x3b3b3b));
                        }
                    }
                },
            )
            .absolute()
            .inset(px(-1.)), // Include the fixed one-pixel checkbox border.
        )
}

/// Render above surrounding flow: parent should be positioned and not clip overflow.
pub fn select_popup() -> Div {
    div()
        .absolute()
        .top_full()
        .left_0()
        .w_full()
        .mt(px(2.))
        .p(px(4.))
        .flex()
        .flex_col()
        .rounded(px(8.))
        .border_1()
        .border_color(rgba(LINE_STRONG))
        .bg(rgb(SURFACE_RAISED))
        .shadow(vec![glow(0, 0.5, 18.)])
        .occlude()
}

/// Native device/channel inventories may contain dozens of options.
/// Keep row metrics intact and scroll the menu instead of compressing rows.
pub fn scrolling_select_popup(id: impl Into<ElementId>) -> Stateful<Div> {
    select_popup()
        .id(id)
        .max_h(px(280.))
        .overflow_y_scroll()
        .restrict_scroll_to_axis()
}

pub fn select_option(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    on: bool,
) -> Stateful<Div> {
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .px(px(8.))
        .py(px(6.))
        .rounded(px(4.))
        .text_size(px(13.))
        .line_height(px(15.5))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_color(rgb(TEXT))
        .bg(rgba(if on { ACCENT_SOFT } else { 0 }))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(ACCENT_SOFT)))
        .child(label.into())
}

pub fn catalog_search(state: &Entity<InputState>) -> Input {
    let input = Input::new(state)
        .w(px(200.))
        .px(px(12.))
        .py(px(6.))
        .rounded(px(10.))
        .border_1()
        .border_color(rgba(LINE))
        .bg(rgba(0xffffff0f))
        .text_color(rgb(TEXT))
        .text_size(px(12.5))
        .line_height(px(15.5))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL);
    // Input::h is an inherent multiline-only setter. Set the actual outer
    // style height, which Input applies after its default single-line sizing.
    Styled::h(input, px(29.))
}

pub fn text_link(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    let label: SharedString = label.into();
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .text_size(px(12.))
        .font_family(DISPLAY_700)
        .font_weight(FontWeight::NORMAL)
        .text_color(rgb(ACCENT))
        .cursor_pointer()
        .hover(|s| s.underline())
        .children((!label.is_empty()).then_some(label))
}

fn ghost_pill_base(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    let label: SharedString = label.into();
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_center()
        .justify_center()
        .gap(px(7.))
        .px(px(16.))
        .py(px(9.))
        .rounded_full()
        .border_1()
        .border_color(rgba(LINE))
        .text_color(rgb(TEXT))
        .text_size(px(13.))
        .line_height(px(15.5))
        .font_family(DISPLAY_600)
        .font_weight(FontWeight::NORMAL)
        .cursor_pointer()
        .children((!label.is_empty()).then(|| crate::tracked_label(label, 0.01)))
}

pub fn ghost_pill(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    ghost_pill_base(id, label).hover(|s| s.border_color(rgba(LINE_STRONG)))
}

pub fn small_ghost_pill(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    ghost_pill(id, label)
        .px(px(12.))
        .py(px(6.))
        .text_size(px(12.))
}

/// Controlled small ghost pill. Disabled presentation follows primitives.css;
/// event/focus admission stays with the owning native control.
pub fn small_ghost_pill_enabled(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    enabled: bool,
) -> Stateful<Div> {
    if enabled {
        return small_ghost_pill(id, label);
    }
    ghost_pill_base(id, label)
        .px(px(12.))
        .py(px(6.))
        .text_size(px(12.))
        .opacity(0.45)
        .cursor_default()
}

#[derive(Clone)]
struct SliderDrag(ElementId);

/// Controlled range input with pointer-down, continuous drag and clamped values.
/// Its measured track bounds live only for the lifetime of this rendered element.
pub fn range_slider(
    id: impl Into<ElementId>,
    value: f32,
    min: f32,
    max: f32,
    on_change: impl Fn(f32, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    range_slider_style(id, value, min, max, on_change, false)
}
pub(crate) fn range_slider_style(
    id: impl Into<ElementId>,
    value: f32,
    min: f32,
    max: f32,
    on_change: impl Fn(f32, &mut Window, &mut App) + 'static,
    native_track: bool,
) -> Stateful<Div> {
    use std::{cell::Cell, rc::Rc};
    let bounds = Rc::new(Cell::new(Bounds::<Pixels>::default()));
    let callback: Rc<dyn Fn(f32, &mut Window, &mut App)> = Rc::new(on_change);
    let fraction = ((value - min) / (max - min)).clamp(0., 1.);
    let down_bounds = bounds.clone();
    let down_callback = callback.clone();
    let id: ElementId = id.into();
    let drag_id = id.clone();
    div()
        .id(id)
        .relative()
        .w_full()
        .h(px(16.))
        .cursor_pointer()
        .on_mouse_down(MouseButton::Left, move |e, w, cx| {
            let b = down_bounds.get();
            let x = f32::from(e.position.x - b.left());
            let (x, width) = if native_track {
                (x - 8., (f32::from(b.size.width) - 16.).max(1.))
            } else {
                (x, f32::from(b.size.width).max(1.))
            };
            down_callback(min + (max - min) * (x / width).clamp(0., 1.), w, cx);
        })
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_drag(SliderDrag(drag_id.clone()), |_, _, _, cx| {
            cx.new(|_| EmptyView)
        })
        .on_drag_move(move |e: &DragMoveEvent<SliderDrag>, w, cx| {
            if e.drag(cx).0 != drag_id {
                return;
            }
            let b = e.bounds;
            let x = f32::from(e.event.position.x - b.left());
            let (x, width) = if native_track {
                (x - 8., (f32::from(b.size.width) - 16.).max(1.))
            } else {
                (x, f32::from(b.size.width).max(1.))
            };
            callback(min + (max - min) * (x / width).clamp(0., 1.), w, cx);
        })
        .on_click(|_, _, cx| cx.stop_propagation())
        .child(
            canvas(
                move |b, _, _| {
                    bounds.set(b);
                    b
                },
                move |_, b, w, _| {
                    let width = b.size.width;
                    if native_track {
                        let center = b.left() + px(8.) + (width - px(16.)).max(px(0.)) * fraction;
                        let track = Bounds::new(
                            point(b.left() + px(1.), b.top() + px(4.5)),
                            size((width - px(2.)).max(px(0.)), px(7.)),
                        );
                        w.paint_quad(quad(
                            track,
                            px(3.5),
                            rgb(0x3b3b3b),
                            px(1.),
                            rgb(0x898989),
                            BorderStyle::Solid,
                        ));
                        let fill_bounds = Bounds::new(
                            track.origin + point(px(1.), px(1.)),
                            size((center - track.left()).max(px(0.)), px(5.)),
                        );
                        w.paint_quad(quad(
                            fill_bounds,
                            px(2.5),
                            rgb(ACCENT),
                            px(0.),
                            rgb(ACCENT),
                            BorderStyle::Solid,
                        ));
                        w.paint_quad(quad(
                            Bounds::new(point(center - px(8.), b.top()), size(px(16.), px(16.))),
                            px(8.),
                            rgb(ACCENT),
                            px(0.),
                            rgb(ACCENT),
                            BorderStyle::Solid,
                        ));
                        return;
                    }
                    let y = b.top() + px(7.);
                    w.paint_quad(fill(
                        Bounds::new(point(b.left(), y), size(width, px(4.))),
                        rgba(LINE_STRONG),
                    ));
                    w.paint_quad(fill(
                        Bounds::new(point(b.left(), y), size(width * fraction, px(4.))),
                        rgb(ACCENT),
                    ));
                    let center = b.left() + width * fraction;
                    w.paint_quad(quad(
                        Bounds::new(
                            point(center - px(7.), b.top() + px(2.)),
                            size(px(14.), px(14.)),
                        ),
                        px(7.),
                        rgb(ACCENT),
                        px(0.),
                        rgb(ACCENT),
                        BorderStyle::Solid,
                    ));
                },
            )
            .absolute()
            .inset_0(),
        )
}

/// Disabled monitor action keeps the production button's full opacity.
pub fn disabled_monitor_pill(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
) -> Stateful<Div> {
    ghost_pill_base(id, label)
        .bg(rgb(PANEL))
        .border_color(rgba(LINE_STRONG))
        .text_color(rgb(FAINT))
        .cursor_default()
}

/// Production header navigation group.
pub fn segmented_nav() -> Div {
    div()
        .flex()
        .items_center()
        .h(px(30.))
        .p(px(2.))
        .rounded(px(9.))
        .border_1()
        .border_color(rgba(LINE))
        .bg(rgba(0x0f0d0ab8))
}

pub fn segmented_tab(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    on: bool,
) -> Stateful<Div> {
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_center()
        .justify_center()
        .h_full()
        .px(px(11.))
        .rounded(px(6.))
        .text_size(px(12.))
        .line_height(px(14.5))
        .font_family(DISPLAY_650)
        .font_weight(FontWeight::NORMAL)
        .text_color(rgb(if on { TEXT } else { DIM }))
        .bg(rgba(if on { 0xfff0d61b } else { 0 }))
        .shadow(if on {
            vec![BoxShadow {
                color: rgba(0x12100d9e).into(),
                offset: point(px(0.), px(1.)),
                blur_radius: px(2.5), // CSS segmented-tab shadow blur 5px
                spread_radius: px(0.),
                inset: false,
            }]
        } else {
            vec![]
        })
        .cursor_pointer()
        .hover(|s| s.text_color(rgb(TEXT)))
        .child(crate::tracked_label(label, 0.01))
}

/// Status pill shell; prepend a status dot with `.child` when needed.
pub fn status_chip(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    let label: SharedString = label.into();
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_center()
        .gap(px(7.))
        .px(px(11.))
        .py(px(6.))
        .rounded_full()
        .border_1()
        .border_color(rgba(LINE))
        .text_size(px(11.5))
        .line_height(px(13.5))
        .font_family(DISPLAY_500)
        .font_weight(FontWeight::NORMAL)
        .text_color(rgb(DIM))
        .children((!label.is_empty()).then(|| crate::tracked_label(label, 0.02)))
}

/// Catalog card delete action, preserving the production hit target and glyph.
pub fn catalog_delete(id: impl Into<ElementId>) -> Stateful<Div> {
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_center()
        .justify_center()
        .w(px(23.))
        .h(px(23.))
        .rounded_full()
        .text_size(px(12.))
        .line_height(px(12.))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_color(rgb(DIM).alpha(0.35))
        .cursor_pointer()
        .hover(|s| s.text_color(rgb(0xff8a7a)).bg(rgba(0xff8a7a24)))
        .child("✕")
}

pub(crate) fn round_button_surface(id: impl Into<ElementId>, on: bool) -> Stateful<Div> {
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_center()
        .justify_center()
        .flex_none()
        .w(px(34.))
        .h(px(34.))
        .rounded_full()
        .border_1()
        .border_color(rgba(if on { ACCENT_LINE } else { LINE }))
        .text_color(rgb(if on { ACCENT } else { DIM }))
        .bg(rgba(if on { ACCENT_SOFT } else { 0 }))
        .cursor_pointer()
}

pub fn round_button(id: impl Into<ElementId>, on: bool) -> Stateful<Div> {
    round_button_surface(id, on)
        .hover(|s| s.text_color(rgb(TEXT)).border_color(rgba(LINE_STRONG)))
}

/// Production `.round-ghost.met.active`: its accent survives hover and has a
/// 14px CSS outer glow. Other round controls keep their ordinary hover treatment.
pub fn metronome_button(id: impl Into<ElementId>, on: bool) -> Stateful<Div> {
    if on {
        round_button_surface(id, true)
            .shadow(vec![glow(ACCENT, 0.15, 14.)])
            .hover(|s| s.text_color(rgb(ACCENT)).border_color(rgba(ACCENT_LINE)))
    } else {
        round_button(id, false)
    }
}

pub fn play_button(id: impl Into<ElementId>, playing: bool) -> Stateful<Div> {
    let mut shadows = vec![BoxShadow {
        color: rgba(0xffffff4d).into(),
        offset: point(px(0.), px(1.)),
        blur_radius: px(0.),
        spread_radius: px(0.),
        inset: true,
    }];
    shadows.push(BoxShadow {
        color: rgba(if playing { 0xffa02880 } else { 0xff962840 }).into(),
        offset: point(px(0.), px(if playing { 0. } else { 6. })),
        blur_radius: px(if playing { 15. } else { 11. }), // CSS 30px / 22px
        spread_radius: px(0.),
        inset: false,
    });
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_center()
        .justify_center()
        .flex_none()
        .w(px(48.))
        .h(px(48.))
        .rounded_full()
        .bg(linear_gradient(
            180.,
            linear_color_stop(rgb(ACCENT_LIFT), 0.),
            linear_color_stop(rgb(ACCENT_DEEP), 1.),
        ))
        .text_color(rgb(ACCENT_INK))
        .shadow(shadows)
        .cursor_pointer()
}

pub(crate) fn transport_chip_surface(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_center()
        .justify_center()
        .w(px(26.))
        .h(px(26.))
        .rounded(px(7.))
        .border_1()
        .border_color(rgba(LINE))
        .text_color(rgb(DIM))
        .text_size(px(14.))
        .line_height(px(16.))
        .font_family(DISPLAY_700)
        .font_weight(FontWeight::NORMAL)
        .cursor_pointer()
        .child(label.into())
}

pub fn transport_chip(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    transport_chip_surface(id,label)
        .hover(|s|s.border_color(rgba(LINE_STRONG)).text_color(rgb(TEXT)))
}

pub fn transport_badge(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    unit: impl Into<SharedString>,
    on: bool,
) -> Stateful<Div> {
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_center()
        .justify_center()
        .gap(px(2.))
        .min_w(px(44.))
        .h(px(26.))
        .rounded(px(8.))
        .border_1()
        .border_color(rgba(if on { ACCENT_LINE } else { LINE }))
        .text_color(rgb(if on { ACCENT } else { DIM }))
        .bg(rgba(if on { ACCENT_SOFT } else { 0 }))
        .text_size(px(12.5))
        .font_family(MONO_500)
        .font_weight(FontWeight::NORMAL)
        .child(label.into())
        .child(
            div()
                .text_size(px(9.))
                .text_color(rgb(FAINT))
                .child(unit.into()),
        )
}

pub fn karaoke_pill(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    on: bool,
) -> Stateful<Div> {
    ghost_pill(id, label)
        .border_color(rgba(if on { ACCENT_LINE } else { LINE }))
        .text_color(rgb(if on { ACCENT } else { DIM }))
        .bg(rgba(if on { ACCENT_SOFT } else { 0 }))
        .shadow(if on {
            vec![glow(ACCENT, 0.15, 18.)]
        } else {
            vec![]
        })
}

pub fn split_pill(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    ghost_pill(id, label)
        .text_color(rgb(0xe2a53e))
        .border_color(rgba(0xe2a53e8c))
        .bg(rgba(0xe2a53e1f))
}

pub fn zoom_chip(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    on: bool,
) -> Stateful<Div> {
    let label: SharedString = label.into();
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_center()
        .justify_center()
        .text_center()
        .min_w(px(26.))
        .px(px(8.))
        .h(px(24.))
        .rounded(px(7.))
        .border_1()
        .border_color(rgba(if on { 0xffa02880 } else { LINE }))
        .text_color(rgb(if on { ACCENT } else { DIM }))
        .bg(rgba(if on { ACCENT_SOFT } else { 0 }))
        .text_size(px(12.))
        .font_family(DISPLAY_700)
        .font_weight(FontWeight::NORMAL)
        .cursor_pointer()
        .child(crate::tracked_label(label, 0.).normal_line_height())
}

pub fn source_badge(label: impl Into<SharedString>, on: bool) -> Div {
    let label: SharedString = label.into();
    div()
        .flex_none()
        .px(px(8.))
        .py(px(2.))
        .rounded_full()
        .border_1()
        .border_color(rgba(if on { 0xffa02866 } else { LINE_STRONG }))
        .bg(rgba(if on { 0xffa0281f } else { 0 }))
        .child(crate::tracked_text(
            label.to_uppercase(),
            DISPLAY_700,
            10.,
            0.4,
            if on { ACCENT } else { DIM },
        ))
}

pub fn lyrics_action(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex_none()
        .px(px(4.))
        .py(px(2.))
        .rounded(px(6.))
        .text_color(rgb(ACCENT))
        .text_size(px(12.))
        .line_height(px(14.))
        .font_family(DISPLAY_600)
        .font_weight(FontWeight::NORMAL)
        .cursor_pointer()
        .hover(|s| s.bg(rgba(ACCENT_SOFT)))
        .child(label.into())
}

pub fn song_output_trigger(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    compact: bool,
) -> Stateful<Div> {
    let button = ghost_pill(id, "")
        .gap(px(8.))
        .h(px(30.))
        .py_0()
        .px(px(10.))
        .rounded(px(8.))
        .child(crate::colored_icon(crate::TransportIcon::Speaker, TEXT))
        .child(
            div()
                .min_w(px(56.))
                .text_center()
                .text_size(px(10.))
                .font_family(MONO_400)
                .font_weight(FontWeight::NORMAL)
                // The source inherits the small pill's 0.12px tracking even
                // after its value span selects a 10px monospace font.
                .child(crate::tracked_label(label, 0.012).normal_line_height()),
        );
    let button = if compact {
        button
    } else {
        button.child(
            div()
                .w(px(28.))
                .h(px(12.))
                .rounded(px(2.))
                .bg(rgb(PANEL_DEEP)),
        )
    };
    button.child(
        div()
            .w(px(4.))
            .h(px(4.))
            .rounded_full()
            .bg(rgba(LINE_STRONG)),
    )
}

pub fn disabled_bpm_field(id: impl Into<ElementId>) -> Stateful<Div> {
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
        .opacity(0.4)
        .cursor_default()
        .child(
            div()
                .w(px(30.))
                .text_right()
                .text_size(px(12.5))
                .font_family(MONO_400)
                .font_weight(FontWeight::NORMAL)
                .text_color(rgb(DIM))
                .child("—"),
        )
        .child(
            div()
                .text_size(px(9.))
                .font_family(DISPLAY_400)
                .font_weight(FontWeight::NORMAL)
                .text_color(rgb(FAINT))
                .child("bpm"),
        )
}

/// Source scrollbar track is transparent. Only the thumb has a BG border.
/// Keep the component's real interaction machinery, painting the CSS thumb
/// separately because its public style projection has no border property.
pub fn production_scrollbar(handle: &ScrollHandle) -> Div {
    source_scrollbar(handle, false)
}
pub fn production_scrollbar_horizontal(handle: &ScrollHandle) -> Div {
    source_scrollbar(handle, true)
}
fn source_scrollbar(handle: &ScrollHandle, horizontal: bool) -> Div {
    use gpui_kit::component::scroll::{Scrollbar, ScrollbarMode};
    let native = if horizontal {
        Scrollbar::horizontal(handle)
    } else {
        Scrollbar::vertical(handle)
    };
    let native = native.mode(ScrollbarMode::Always).styles(|s| {
        s.track(|s| {
            s.width(px(10.))
                .bg(rgba(0).into())
                .border_color(rgba(0).into())
        })
        .track_hover(|s| {
            s.width(px(10.))
                .bg(rgba(0).into())
                .border_color(rgba(0).into())
        })
        .track_active(|s| {
            s.width(px(10.))
                .bg(rgba(0).into())
                .border_color(rgba(0).into())
        })
        .thumb(|s| {
            s.width(px(10.))
                .inset(px(0.))
                .min_length(px(0.))
                .radius(px(5.))
                .bg(rgba(0))
        })
        .thumb_hover(|s| {
            s.width(px(10.))
                .inset(px(0.))
                .min_length(px(0.))
                .radius(px(5.))
                .bg(rgba(0))
        })
        .thumb_active(|s| {
            s.width(px(10.))
                .inset(px(0.))
                .min_length(px(0.))
                .radius(px(5.))
                .bg(rgba(0))
        })
    });
    div().absolute().inset_0().child(SourceScrollbar {
        native: Some(native),
        handle: handle.clone(),
        horizontal,
    })
}

type NativeScrollbar = gpui_kit::component::scroll::Scrollbar;
struct SourceScrollbar {
    native: Option<NativeScrollbar>,
    handle: ScrollHandle,
    horizontal: bool,
}
impl IntoElement for SourceScrollbar {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}
impl Element for SourceScrollbar {
    type RequestLayoutState = <NativeScrollbar as Element>::RequestLayoutState;
    type PrepaintState = (<NativeScrollbar as Element>::PrepaintState, Bounds<Pixels>);
    fn id(&self) -> Option<ElementId> {
        self.native.as_ref().and_then(|native| native.id())
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        self.native
            .as_mut()
            .expect("native scrollbar")
            .request_layout(id, inspector, window, cx)
    }
    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        // ScrollHandle bounds exclude the reserved CSS scrollbar gutter. Keep
        // its client dimensions for ratio/drag calculations, but anchor the
        // cross-axis edge to the wrapper containing that gutter.
        let viewport = scrollbar_viewport(bounds, self.handle.bounds(), self.horizontal);
        let native = self
            .native
            .take()
            .expect("native scrollbar")
            .viewport_bounds(viewport);
        self.native = Some(native);
        let state = self
            .native
            .as_mut()
            .unwrap()
            .prepaint(id, inspector, bounds, request, window, cx);
        (state, viewport)
    }
    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request: &mut Self::RequestLayoutState,
        state: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.native.as_mut().expect("native scrollbar").paint(
            id,
            inspector,
            bounds,
            request,
            &mut state.0,
            window,
            cx,
        );
        if let Some(thumb) = scrollbar_thumb_bounds(
            state.1,
            self.handle.offset(),
            self.handle.max_offset(),
            self.horizontal,
        ) {
            let hovered = thumb.contains(&window.mouse_position());
            window.paint_quad(quad(
                thumb,
                px(5.),
                rgba(if hovered { 0xfff0d638 } else { 0xfff0d61f }),
                px(3.),
                rgb(BG),
                BorderStyle::Solid,
            ));
        }
    }
}
fn scrollbar_viewport(
    wrapper: Bounds<Pixels>,
    client: Bounds<Pixels>,
    horizontal: bool,
) -> Bounds<Pixels> {
    let origin = if horizontal {
        point(wrapper.left(), wrapper.bottom() - client.size.height)
    } else {
        point(wrapper.right() - client.size.width, wrapper.top())
    };
    Bounds::new(origin, client.size)
}

fn scrollbar_thumb_bounds(
    viewport: Bounds<Pixels>,
    offset: Point<Pixels>,
    extent: Point<Pixels>,
    horizontal: bool,
) -> Option<Bounds<Pixels>> {
    let (track_size, overflow, position) = if horizontal {
        (viewport.size.width, extent.x, offset.x)
    } else {
        (viewport.size.height, extent.y, offset.y)
    };
    if track_size <= px(0.) || overflow <= px(0.) {
        return None;
    }
    let length = track_size * (track_size / (track_size + overflow));
    let start = (-position / overflow).clamp(0., 1.) * (track_size - length);
    Some(if horizontal {
        Bounds::new(
            point(viewport.left() + start, viewport.bottom() - px(10.)),
            size(length, px(10.)),
        )
    } else {
        Bounds::new(
            point(viewport.right() - px(10.), viewport.top() + start),
            size(px(10.), length),
        )
    })
}

pub fn stem_chip(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    active: bool,
    color: u32,
) -> Stateful<Div> {
    let label: SharedString = label.into();
    let ink = if label.as_ref() == "M" {
        0x171310
    } else {
        ACCENT_INK
    };
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_center()
        .justify_center()
        .w(px(26.))
        .h(px(24.))
        .rounded(px(7.))
        .border_1()
        .border_color(rgba(if active { 0 } else { LINE }))
        .bg(rgba(if active { (color << 8) | 255 } else { 0 }))
        .text_color(rgb(if active { ink } else { DIM }))
        .font_family(DISPLAY_800)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(11.))
        .line_height(px(13.5))
        .cursor_pointer()
        .hover(|s| s.border_color(rgba(LINE_STRONG)))
        .child(label)
}

/// Source `.vol` rail and thumb, with HTML range-input thumb travel geometry.
pub fn stem_volume(
    id: impl Into<ElementId>,
    value: f32,
    color: u32,
    on_change: impl Fn(f32, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    stem_volume_impl(id, value, color, None, on_change)
}

/// The same source rail/thumb with persistent host-owned range-input focus.
/// Relative keys use a live value so events arriving before repaint accumulate.
pub fn stem_volume_with_focus(
    id: impl Into<ElementId>,
    value: f32,
    color: u32,
    focus: FocusHandle,
    on_change: impl Fn(f32, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    stem_volume_impl(id, value, color, Some(focus), on_change)
}

fn stem_range_key(value: &std::cell::Cell<f32>, key: &str) -> Option<f32> {
    let current = (value.get().clamp(0., 1.) * 100.).round() as i16;
    let next = match key {
        "left" | "down" => current - 1,
        "right" | "up" => current + 1,
        "home" => 0,
        "end" => 100,
        _ => return None,
    }.clamp(0, 100) as f32 / 100.;
    value.set(next);
    Some(next)
}

fn stem_volume_impl(
    id: impl Into<ElementId>,
    value: f32,
    color: u32,
    focus: Option<FocusHandle>,
    on_change: impl Fn(f32, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    use std::{cell::Cell, rc::Rc};
    let bounds = Rc::new(Cell::new(Bounds::<Pixels>::default()));
    let callback: Rc<dyn Fn(f32, &mut Window, &mut App)> = Rc::new(on_change);
    let down_bounds = bounds.clone();
    let down_callback = callback.clone();
    let id: ElementId = id.into();
    let drag_id = id.clone();
    let value = value.clamp(0., 1.);
    let live = Rc::new(Cell::new(value));
    let down_live = live.clone();
    let key_live = live.clone();
    let key_callback = callback.clone();
    let down_focus = focus.clone();
    let keyboard = focus.is_some();
    let element = div()
        .id(id)
        .relative()
        .flex_1()
        .min_w(px(40.))
        .h(px(11.))
        .cursor_pointer()
        .on_mouse_down(MouseButton::Left, move |e, w, cx| {
            let b = down_bounds.get();
            let ratio = (f32::from(e.position.x - b.left()) - 5.5)
                / (f32::from(b.size.width) - 11.).max(1.);
            if let Some(focus) = &down_focus { focus.focus(w, cx); w.prevent_default(); }
            let next = if keyboard { (ratio.clamp(0., 1.) * 100.).round() / 100. } else { ratio.clamp(0., 1.) };
            down_live.set(next);
            down_callback(next, w, cx);
        })
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_drag(SliderDrag(drag_id.clone()), |_, _, _, cx| {
            cx.new(|_| EmptyView)
        })
        .on_drag_move(move |e: &DragMoveEvent<SliderDrag>, w, cx| {
            if e.drag(cx).0 != drag_id {
                return;
            }
            let b = e.bounds;
            let ratio = (f32::from(e.event.position.x - b.left()) - 5.5)
                / (f32::from(b.size.width) - 11.).max(1.);
            let next = if keyboard { (ratio.clamp(0., 1.) * 100.).round() / 100. } else { ratio.clamp(0., 1.) };
            live.set(next);
            callback(next, w, cx);
        })
        .on_key_down(move |event, w, cx| {
            if !keyboard { return; }
            let before = key_live.get();
            if let Some(next) = stem_range_key(&key_live, &event.keystroke.key) {
                if next != before { key_callback(next, w, cx); }
                cx.stop_propagation();
            }
        })
        .on_click(|_, _, cx| cx.stop_propagation())
        .child(
            canvas(
                move |b, _, _| {
                    bounds.set(b);
                    b
                },
                move |_, b, w, _| {
                    let mix = |shift: u32| {
                        (((color >> shift) & 255) as f32 * 0.3
                            + ((PANEL >> shift) & 255) as f32 * 0.7)
                            .round() as u32
                    };
                    let track = (mix(16) << 16) | (mix(8) << 8) | mix(0);
                    w.paint_quad(quad(
                        Bounds::new(
                            point(b.left(), b.top() + px(4.)),
                            size(b.size.width, px(3.)),
                        ),
                        px(2.),
                        rgb(track),
                        px(0.),
                        rgba(0),
                        BorderStyle::Solid,
                    ));
                    let center = b.left() + px(5.5) + (b.size.width - px(11.)) * value;
                    let thumb =
                        Bounds::new(point(center - px(5.5), b.top()), size(px(11.), px(11.)));
                    w.paint_drop_shadows(thumb, px(5.5).into(), &[glow(color, 0.6, 6.)]);
                    w.paint_quad(quad(
                        thumb,
                        px(5.5),
                        rgb(color),
                        px(2.),
                        rgb(BG),
                        BorderStyle::Solid,
                    ));
                },
            )
            .absolute()
            .inset_0(),
        );
    if let Some(focus) = focus { element.track_focus(&focus).tab_index(0) } else { element }
}

#[cfg(test)]
mod stem_volume_keyboard_tests {
    use super::stem_range_key;
    use std::cell::Cell;
    #[test]
    fn stem_range_keys_accumulate_before_repaint_and_clamp_like_html() {
        let value = Cell::new(0.5);
        for expected in [0.51, 0.52, 0.53] {
            assert!((stem_range_key(&value, "right").unwrap() - expected).abs() < 0.000001);
        }
        assert_eq!(stem_range_key(&value, "end"), Some(1.));
        assert_eq!(stem_range_key(&value, "up"), Some(1.));
        assert_eq!(stem_range_key(&value, "home"), Some(0.));
        assert_eq!(stem_range_key(&value, "down"), Some(0.));
        assert_eq!(stem_range_key(&value, "space"), None);
        assert_eq!(stem_range_key(&value, "enter"), None);
    }
}

/// Production monitoring feature label: 8px bold mono / 8px line / .72px tracking.
pub fn monitor_badge(label: impl Into<SharedString>) -> Div {
    div()
        .flex_none()
        .px(px(7.))
        .py(px(4.))
        .rounded_full()
        .border_1()
        .border_color(rgba(0xffa02861))
        .bg(rgba(ACCENT_SOFT))
        .child(crate::tracked_text(label, MONO_700, 8., 0.72, ACCENT).line_height(8.))
}

/// Track-stack ruler action. Disabled semantics remain host-owned: attach
/// callbacks only for enabled actions, matching the native button contract.
pub fn ruler_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    compact: bool,
    disabled: bool,
) -> Stateful<Div> {
    let button = div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_center()
        .justify_center()
        .h(px(22.))
        .px(px(if compact { 7. } else { 11. }))
        .rounded_full()
        .border_1()
        .border_dashed()
        .border_color(rgba(LINE))
        .bg(rgba(0))
        .font_family(DISPLAY_700)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(10.5))
        .line_height(px(13.))
        .text_color(rgb(DIM))
        .child(crate::tracked_label(label, 0.02));
    if disabled {
        button.opacity(0.35).cursor_default()
    } else {
        button.cursor_pointer().hover(|s| {
            s.text_color(rgb(TEXT))
                .border_color(rgba(LINE_STRONG))
                .bg(rgba(0xfff0d60d))
        })
    }
}

/// A single clipped segmented group; sibling separators belong to the group,
/// so buttons never become separate rounded pills.
pub fn zoom_segment(children: impl IntoIterator<Item = Stateful<Div>>) -> Div {
    div()
        .flex()
        .items_stretch()
        .h(px(22.))
        .border_1()
        .border_color(rgba(LINE))
        .rounded_full()
        .overflow_hidden()
        .bg(rgba(0x1b1814a6))
        .children(children.into_iter().enumerate().map(|(index, button)| {
            if index == 0 {
                button
            } else {
                button.border_l_1().border_color(rgba(LINE))
            }
        }))
}

pub fn zoom_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    disabled: bool,
) -> Stateful<Div> {
    let button = div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_center()
        .justify_center()
        .h_full()
        .px(px(11.))
        .font_family(MONO_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(11.))
        .line_height(px(11.))
        .text_color(rgb(DIM))
        .child(label.into());
    if disabled {
        button.opacity(0.35).cursor_default()
    } else {
        button
            .cursor_pointer()
            .hover(|s| s.text_color(rgb(TEXT)).bg(rgba(0xfff0d60d)))
    }
}

#[cfg(test)]
mod scrollbar_tests {
    use super::*;
    #[::core::prelude::v1::test]
    fn gutter_anchor_preserves_client_ratio_and_native_hit_bounds() {
        let wrapper = Bounds::new(point(px(317.), px(130.)), size(px(774.), px(560.)));
        let client = Bounds::new(point(px(317.), px(130.)), size(px(764.), px(550.)));
        let vertical = scrollbar_viewport(wrapper, client, false);
        assert_eq!(vertical.right(), wrapper.right());
        assert_eq!(vertical.size, client.size);
        assert_eq!(vertical.bottom(), wrapper.bottom() - px(10.));
        let horizontal = scrollbar_viewport(wrapper, client, true);
        assert_eq!(horizontal.bottom(), wrapper.bottom());
        assert_eq!(horizontal.size, client.size);
        assert_eq!(horizontal.right(), wrapper.right() - px(10.));
    }
    #[::core::prelude::v1::test]
    fn vertical_thumb_tracks_start_middle_and_end() {
        let viewport = Bounds::new(point(px(10.), px(20.)), size(px(100.), px(80.)));
        let extent = point(px(0.), px(80.));
        for (offset, top) in [(0., 20.), (-40., 40.), (-80., 60.)] {
            let thumb =
                scrollbar_thumb_bounds(viewport, point(px(0.), px(offset)), extent, false).unwrap();
            assert_eq!(
                thumb,
                Bounds::new(point(px(100.), px(top)), size(px(10.), px(40.)))
            );
        }
    }
    #[::core::prelude::v1::test]
    fn horizontal_thumb_and_non_overflowing_view() {
        let viewport = Bounds::new(point(px(10.), px(20.)), size(px(100.), px(80.)));
        assert_eq!(
            scrollbar_thumb_bounds(
                viewport,
                point(px(-100.), px(0.)),
                point(px(100.), px(0.)),
                true
            ),
            Some(Bounds::new(point(px(60.), px(90.)), size(px(50.), px(10.))))
        );
        assert!(
            scrollbar_thumb_bounds(viewport, point(px(0.), px(0.)), point(px(0.), px(0.)), true)
                .is_none()
        );
    }
}

/// Header monitoring status without a secondary stop action. The frame remains
/// stable on hover; only the inner status surface receives the amber wash.
pub fn persistent_monitor_status(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    attention: bool,
) -> Stateful<Div> {
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_stretch()
        .flex_shrink_1()
        .min_w_0()
        .min_h(px(28.))
        .rounded_full()
        .overflow_hidden()
        .border_1()
        .border_color(if attention {
            rgb(0xef695a).alpha(0.65)
        } else {
            rgb(ACCENT).alpha(0.5)
        })
        .bg(rgb(ACCENT).alpha(0.1))
        .shadow(vec![glow(ACCENT, 0.08, 18.)])
        .text_color(rgb(TEXT))
        .text_size(px(11.))
        .line_height(px(13.5))
        .font_family(DISPLAY_650)
        .font_weight(FontWeight::NORMAL)
        .cursor_pointer()
        .child(
            div()
                .id("persistent-monitor-status")
                .flex()
                .items_center()
                .flex_shrink_0()
                .whitespace_nowrap()
                .gap(px(7.))
                .pt(px(4.))
                .pb(px(4.))
                .pl(px(10.))
                .pr(px(9.))
                .bg(rgba(0))
                .hover(|s| s.bg(rgb(ACCENT).alpha(0.12)))
                .child(
                    div()
                        .size(px(7.))
                        .flex_none()
                        .rounded_full()
                        .bg(rgb(if attention { 0xef695a } else { ACCENT }))
                        .shadow(vec![BoxShadow {
                            color: rgb(ACCENT).alpha(0.13).into(),
                            offset: point(px(0.), px(0.)),
                            blur_radius: px(0.),
                            spread_radius: px(3.),
                            inset: false,
                        }]),
                )
                .child(crate::tracked_text(label, DISPLAY_650, 11., 0., TEXT)),
        )
}

/// Cached pitch-match sprite. One image pipeline paints both the dot and halo;
/// a live trail must not alternate shadow/quad pipelines hundreds of times.
pub fn paint_pitch_match_dot(
    window: &mut Window,
    center: Point<Pixels>,
    radius: f32,
    matched: bool,
) -> anyhow::Result<()> {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex, OnceLock},
    };
    type Key = (u16, u16, bool);
    static CACHE: OnceLock<Mutex<HashMap<Key, Arc<RenderImage>>>> = OnceLock::new();
    let scale = window.scale_factor();
    let key = (
        (radius.clamp(2.5, 4.5) * 64.).round() as u16,
        (scale * 64.).round() as u16,
        matched,
    );
    let mut cache = CACHE
        .get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let image = if let Some(image) = cache.get(&key) {
        image.clone()
    } else {
        if cache.len() >= 16 {
            cache.clear();
        }
        let side = (40. * scale).ceil() as u32;
        let mut pixels = image::RgbaImage::new(side, side);
        let radius = key.0 as f32 / 64.;
        let mut core = image::GrayImage::new(side, side);
        for (x, y, pixel) in core.enumerate_pixels_mut() {
            let dx = (x as f32 + 0.5) / scale - 20.;
            let dy = (y as f32 + 0.5) / scale - 20.;
            let distance = (dx * dx + dy * dy).sqrt();
            *pixel = image::Luma([
                (((radius - distance) * scale + 0.5).clamp(0., 1.) * 255.).round() as u8,
            ]);
        }
        let halo = if matched {
            Some(image::imageops::blur(&core, 3. * scale))
        } else {
            None
        };
        for (x, y, pixel) in pixels.enumerate_pixels_mut() {
            let coverage = core.get_pixel(x, y)[0] as f32 / 255.;
            let shadow = halo
                .as_ref()
                .map_or(0., |halo| halo.get_pixel(x, y)[0] as f32 / 255.);
            let alpha = if matched {
                coverage + (1. - coverage) * shadow
            } else {
                coverage * 0.7
            };
            let [r, g, b] = if matched {
                [0x58, 0xd6, 0x8a]
            } else {
                [0xff, 0x7a, 0x5c]
            };
            *pixel = image::Rgba([b, g, r, (alpha * 255.).round() as u8]);
        }
        let image = Arc::new(RenderImage::new(vec![image::Frame::new(pixels)]));
        cache.insert(key, image.clone());
        image
    };
    drop(cache);
    let bounds = Bounds::new(center - point(px(20.), px(20.)), size(px(40.), px(40.)));
    window.paint_image(bounds, bounds, Corners::default(), image, 0, false)
}

/// Inline song/project title editor from App.tsx (`.title-input`).
pub fn title_input(state: &Entity<InputState>, viewport_width: f32) -> Input {
    let input = Input::new(state)
        .focus_bordered(false)
        .w(px(360_f32.min(viewport_width * 0.4)))
        .min_w_0()
        .px(px(10.))
        .py(px(5.))
        .rounded(px(9.))
        .border_1()
        .border_color(rgba(LINE_STRONG))
        .bg(rgb(PANEL_DEEP))
        .text_color(rgb(TEXT))
        .font_family(DISPLAY_500)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(13.))
        .line_height(px(15.5));
    Styled::h(input, px(27.5))
}
/// Source monitor group with independently focusable status and Stop buttons.
/// The host attaches handlers/focus handles, then inserts both buttons in shell.
pub struct PersistentMonitorControl {
    pub shell: Stateful<Div>,
    pub status: Stateful<Div>,
    pub stop: Stateful<Div>,
}
pub fn persistent_monitor_control(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    attention: bool,
    stop_enabled: bool,
) -> PersistentMonitorControl {
    let focus_ring = |s: StyleRefinement| {
        s.shadow(vec![BoxShadow {
            color: rgb(ACCENT).into(),
            offset: point(px(0.), px(0.)),
            blur_radius: px(0.),
            spread_radius: px(2.),
            inset: true,
        }])
    };
    let shell = div()
        .id(id)
        .flex()
        .items_stretch()
        .flex_shrink_1()
        .min_w_0()
        .min_h(px(28.))
        .rounded_full()
        .overflow_hidden()
        .border_1()
        .border_color(if attention {
            rgb(0xef695a).alpha(0.65)
        } else {
            rgb(ACCENT).alpha(0.5)
        })
        .bg(rgb(ACCENT).alpha(0.1))
        .shadow(vec![glow(ACCENT, 0.08, 18.)])
        .text_color(rgb(TEXT))
        .text_size(px(11.))
        .line_height(px(13.5))
        .font_family(DISPLAY_650)
        .font_weight(FontWeight::NORMAL);
    let status = div()
        .id("persistent-monitor-status")
        .flex()
        .items_center()
        .flex_shrink_0()
        .whitespace_nowrap()
        .gap(px(7.))
        .py(px(4.))
        .pl(px(10.))
        .pr(px(9.))
        .bg(rgba(0))
        .cursor_pointer()
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .hover(|s| s.bg(rgb(ACCENT).alpha(0.12)))
        .focus_visible(focus_ring)
        .child(
            div()
                .size(px(7.))
                .flex_none()
                .rounded_full()
                .bg(rgb(if attention { 0xef695a } else { ACCENT }))
                .shadow(vec![BoxShadow {
                    color: rgb(ACCENT).alpha(0.13).into(),
                    offset: point(px(0.), px(0.)),
                    blur_radius: px(0.),
                    spread_radius: px(3.),
                    inset: false,
                }]),
        )
        .child(crate::tracked_text(label, DISPLAY_650, 11., 0., TEXT));
    let mut stop = div()
        .id("persistent-monitor-stop")
        .flex()
        .items_center()
        .flex_none()
        .whitespace_nowrap()
        .py(px(4.))
        .pl(px(9.))
        .pr(px(10.))
        .border_l_1()
        .border_color(rgb(ACCENT).alpha(0.28))
        .text_color(rgb(if stop_enabled { ACCENT } else { FAINT }))
        .bg(rgba(0))
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .child(crate::tracked_text(
            "Stop",
            DISPLAY_650,
            11.,
            0.,
            if stop_enabled { ACCENT } else { FAINT },
        ));
    if stop_enabled {
        stop = stop
            .cursor_pointer()
            .hover(|s| s.bg(rgb(ACCENT).alpha(0.12)))
            .focus_visible(focus_ring);
    } else {
        // GPUI's platform cursor enum has no CSS `wait` cursor yet.
        stop = stop.cursor_default();
    }
    PersistentMonitorControl {
        shell,
        status,
        stop,
    }
}
