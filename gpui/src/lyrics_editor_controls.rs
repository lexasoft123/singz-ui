//! Native presentation of the production `.lyed-*` lyric editor controls.
//! The host owns draft state, audio envelopes, focus, interaction and backdrop
//! blur. This module owns shared dimensions, typography and control surfaces.
use crate::tokens::*;
use gpui_kit::{
    component::input::{Input, InputState, Textarea, TextareaState},
    prelude::FluentBuilder,
    *,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
};

fn mix(a: Rgba, b: Rgba, fraction: f32) -> Rgba {
    let aa = a.a * fraction;
    let ba = b.a * (1. - fraction);
    let alpha = aa + ba;
    Rgba {
        r: (a.r * aa + b.r * ba) / alpha,
        g: (a.g * aa + b.g * ba) / alpha,
        b: (a.b * aa + b.b * ba) / alpha,
        a: alpha,
    }
}

fn enabled(button: Stateful<Div>, is_enabled: bool) -> Stateful<Div> {
    button
        .when(is_enabled, |b| b.cursor_pointer())
        .when(!is_enabled, |b| b.opacity(0.45).cursor_default())
}

/// The source's three-stop glass gradient and nonanimated radial sheen. Cached
/// at device resolution; no image allocation or pixel work occurs on playback
/// frames after a size/scale combination has been painted once.
fn paint_card_surface(bounds: Bounds<Pixels>, window: &mut Window) {
    type Key = (u32, u32, u32, bool);
    static CACHE: OnceLock<Mutex<HashMap<Key, Arc<RenderImage>>>> = OnceLock::new();
    let scale = window.scale_factor();
    let width = (f32::from(bounds.size.width) * scale).ceil().max(1.) as u32;
    let height = (f32::from(bounds.size.height) * scale).ceil().max(1.) as u32;
    let key = (width, height, scale.to_bits(), cfg!(windows));
    let mut cache = CACHE
        .get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let surface = cache
        .entry(key)
        .or_insert_with(|| {
            let rgb = |color: u32| {
                [
                    (color >> 16 & 255) as f32 / 255.,
                    (color >> 8 & 255) as f32 / 255.,
                    (color & 255) as f32 / 255.,
                ]
            };
            let panel = rgb(PANEL);
            let deep = rgb(PANEL_DEEP);
            let bg = rgb(BG);
            let text = rgb(TEXT);
            let angle = 160_f32.to_radians();
            let (dx, dy) = (angle.sin(), -angle.cos());
            let span = (width as f32 * dx.abs() + height as f32 * dy.abs()).max(1.);
            let mut pixels = image::RgbaImage::new(width, height);
            for (x, y, pixel) in pixels.enumerate_pixels_mut() {
                let t = (((x as f32 + 0.5 - width as f32 / 2.) * dx
                    + (y as f32 + 0.5 - height as f32 / 2.) * dy)
                    / span
                    + 0.5)
                    .clamp(0., 1.);
                let (a, mut premul) = if cfg!(windows) {
                    (
                        1.,
                        std::array::from_fn::<_, 3, _>(|c| panel[c] * 0.94 + bg[c] * 0.06),
                    )
                } else if t <= 0.6 {
                    let f = t / 0.6;
                    let a = 0.62 * (1. - f) + 0.55 * f;
                    (
                        a,
                        std::array::from_fn(|c| panel[c] * 0.62 * (1. - f) + deep[c] * 0.55 * f),
                    )
                } else {
                    let a = 0.55 + (0.62 - 0.55) * ((t - 0.6) / 0.4);
                    (a, std::array::from_fn(|c| deep[c] * a))
                };
                let nx = ((x as f32 + 0.5) / width as f32 - 0.18) / 1.2;
                let ny = ((y as f32 + 0.5) / height as f32 + 0.08) / 0.6;
                let sheen = 0.07 * (1. - (nx * nx + ny * ny).sqrt() / 0.55).clamp(0., 1.);
                let alpha = sheen + a * (1. - sheen);
                for c in 0..3 {
                    premul[c] = text[c] * sheen + premul[c] * (1. - sheen);
                }
                // GPUI RenderImage's upload contract uses BGRA channel ordering.
                *pixel = image::Rgba([
                    (premul[2] / alpha * 255.).round() as u8,
                    (premul[1] / alpha * 255.).round() as u8,
                    (premul[0] / alpha * 255.).round() as u8,
                    (alpha * 255.).round() as u8,
                ]);
            }
            Arc::new(RenderImage::new(vec![image::Frame::new(pixels)]))
        })
        .clone();
    if cache.len() > 8 {
        cache.retain(|k, _| *k == key);
    }
    drop(cache);
    let _ = window.paint_image(bounds, bounds, Corners::all(px(21.)), surface, 0, false);
}

pub fn lyrics_editor_card(viewport: Size<Pixels>) -> Div {
    div()
        .relative()
        .flex()
        .flex_col()
        .min_w_0()
        .min_h_0()
        .w(px(880_f32.min((f32::from(viewport.width) - 64.).max(1.))))
        .h(px(680_f32.min((f32::from(viewport.height) - 88.).max(1.))))
        .pt(px(22.))
        .px(px(26.))
        .pb(px(18.))
        .rounded(px(22.))
        .border_1()
        .border_color(rgb(TEXT).alpha(0.14))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_color(rgb(TEXT))
        .overflow_hidden()
        .occlude()
        .shadow(vec![
            BoxShadow {
                color: rgb(TEXT).alpha(0.16).into(),
                offset: point(px(0.), px(1.)),
                blur_radius: px(0.),
                spread_radius: px(0.),
                inset: true,
            },
            BoxShadow {
                color: rgb(TEXT).alpha(0.05).into(),
                offset: point(px(0.), px(-1.)),
                blur_radius: px(0.),
                spread_radius: px(0.),
                inset: true,
            },
            BoxShadow {
                color: black().alpha(0.65).into(),
                offset: point(px(0.), px(40.)),
                blur_radius: px(50.),
                spread_radius: px(0.),
                inset: false,
            },
        ])
        .child(
            canvas(|b, _, _| b, |_, b, w, _| paint_card_surface(b, w))
                .absolute()
                .inset_0(),
        )
}

pub fn lyrics_editor_head() -> Div {
    div()
        .relative()
        .flex()
        .items_baseline()
        .gap(px(14.))
        .flex_none()
}
pub fn lyrics_editor_title(label: impl Into<SharedString>) -> Div {
    div()
        .min_w_0()
        .font_family(DISPLAY_700)
        .text_size(px(19.))
        .child(crate::tracked_label(label, 0.01))
}
pub fn lyrics_editor_song(label: impl Into<SharedString>) -> Div {
    div()
        .min_w_0()
        .text_size(px(13.))
        .text_color(rgb(DIM))
        .text_ellipsis()
        .child(label.into())
}
pub fn lyrics_editor_clock(label: impl Into<SharedString>) -> Div {
    div()
        .min_w(px(58.))
        .font_family(MONO_400)
        .text_size(px(12.))
        .text_color(rgb(DIM))
        .text_right()
        .whitespace_nowrap()
        .child(label.into())
}
pub fn lyrics_editor_dirty_dot() -> Div {
    div()
        .size(px(7.))
        .rounded_full()
        .bg(rgb(ACCENT))
        .flex_none()
        .self_center()
}
pub fn lyrics_editor_tools() -> Div {
    div()
        .relative()
        .flex()
        .items_center()
        .flex_wrap()
        .gap(px(8.))
        .mt(px(14.))
        .mb(px(10.))
        .flex_none()
}
pub fn lyrics_editor_tool(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    is_enabled: bool,
    ghost: bool,
) -> Stateful<Div> {
    enabled(
        crate::transport_chip(id, label)
            .w_auto()
            .h(px(26.))
            .px(px(11.))
            .font_family(DISPLAY_600)
            .text_size(px(11.))
            .line_height(px(13.))
            .whitespace_nowrap()
            .when(ghost, |b| b.border_dashed()),
        is_enabled,
    )
}
pub fn lyrics_editor_list(id: impl Into<ElementId>) -> Stateful<Div> {
    div()
        .id(id)
        .relative()
        .flex()
        .flex_col()
        .flex_1()
        .min_h_0()
        .gap(px(2.))
        .mx(px(-10.))
        .px(px(10.))
        .py(px(2.))
        .overflow_y_scroll()
        .restrict_scroll_to_axis()
}
/// Place stamp(62), print(54), flexing input, add(26), remove(26) in this row.
pub fn lyrics_editor_row(id: impl Into<ElementId>, current: bool) -> Stateful<Div> {
    div()
        .id(id)
        .relative()
        .flex()
        .items_center()
        .gap(px(10.))
        .rounded(px(12.))
        .px(px(6.))
        .py(px(3.))
        .min_h(px(34.))
        .flex_none()
        .when(current, |r| {
            r.bg(rgb(ACCENT).alpha(0.12)).shadow(vec![BoxShadow {
                color: rgb(ACCENT).into(),
                offset: point(px(2.), px(0.)),
                blur_radius: px(0.),
                spread_radius: px(0.),
                inset: true,
            }])
        })
        .hover(move |r| {
            r.bg(if current {
                rgb(ACCENT).alpha(0.12)
            } else {
                rgb(TEXT).alpha(0.05)
            })
        })
}
pub fn lyrics_editor_stamp(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    focused: bool,
    is_enabled: bool,
) -> Stateful<Div> {
    enabled(
        div()
            .id(id)
            .w(px(62.))
            .flex_none()
            .py(px(4.))
            .border_1()
            .border_color(if focused {
                rgb(ACCENT).into()
            } else {
                transparent_black()
            })
            .rounded(px(8.))
            .font_family(MONO_400)
            .font_weight(FontWeight::NORMAL)
            .text_size(px(11.))
            .line_height(px(13.))
            .text_center()
            .text_color(rgb(if focused { ACCENT } else { DIM }))
            .bg(rgb(TEXT).alpha(0.06))
            .hover(|b| {
                b.text_color(rgb(ACCENT))
                    .border_color(rgb(ACCENT).alpha(0.4))
            })
            .child(label.into()),
        is_enabled,
    )
}
pub fn lyrics_editor_print_button(
    id: impl Into<ElementId>,
    open: bool,
    focused: bool,
    is_enabled: bool,
) -> Stateful<Div> {
    enabled(
        div()
            .id(id)
            .w(px(54.))
            .h(px(23.))
            .flex_none()
            .pt(px(1.))
            .border_1()
            .rounded(px(8.))
            .border_color(if focused {
                rgb(ACCENT).into()
            } else if open {
                rgb(ACCENT).alpha(0.4).into()
            } else {
                transparent_black()
            })
            .when(open, |b| b.bg(rgb(ACCENT).alpha(0.08)))
            .hover(|b| {
                b.border_color(rgb(ACCENT).alpha(0.4))
                    .bg(rgb(ACCENT).alpha(0.08))
            }),
        is_enabled,
    )
}
pub fn lyrics_editor_voiceprint(levels: [f32; 18]) -> Div {
    div().w(px(54.)).h(px(20.)).opacity(0.9).child(
        canvas(
            |b, _, _| b,
            move |_, b, w, _| {
                for (i, value) in levels.iter().enumerate() {
                    let level = if value.is_finite() {
                        value.clamp(0., 1.)
                    } else {
                        0.
                    };
                    let height = (level * 20.).max(1.5);
                    w.paint_quad(fill(
                        Bounds::new(
                            point(
                                b.left() + px(i as f32 * 3. + 0.5),
                                b.top() + px((20. - height) / 2.),
                            ),
                            size(px(1.5), px(height)),
                        ),
                        crate::canvas_color(rgb(ACCENT).alpha(0.6).into()),
                    ));
                }
            },
        )
        .size_full(),
    )
}
pub fn lyrics_editor_row_input(state: &Entity<InputState>, focused: bool, silent: bool) -> Input {
    let input = Input::new(state)
        .appearance(false)
        .focus_bordered(false)
        .border_0()
        .rounded(px(8.))
        .px(px(4.))
        .py(px(5.))
        .min_w_0()
        .flex_1()
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(15.))
        .line_height(px(18.))
        .text_color(rgb(if silent { DIM } else { TEXT }))
        .bg(if focused {
            rgb(PANEL_DEEP).alpha(0.65).into()
        } else {
            transparent_black()
        })
        .when(silent, |i| {
            i.line_through()
                .text_decoration_color(rgb(DIM))
                .text_decoration_1()
        })
        .when(focused, |i| {
            i.shadow(vec![BoxShadow {
                color: rgb(ACCENT).alpha(0.45).into(),
                offset: point(px(0.), px(0.)),
                blur_radius: px(0.),
                spread_radius: px(1.),
                inset: true,
            }])
        });
    Styled::h(input, px(28.))
}
pub fn lyrics_editor_row_action(
    id: impl Into<ElementId>,
    remove: bool,
    visible: bool,
    focused: bool,
    is_enabled: bool,
) -> Stateful<Div> {
    enabled(
        div()
            .id(id)
            .size(px(26.))
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .p(px(4.))
            .rounded(px(6.))
            .font_family(DISPLAY_400)
            .text_size(px(12.))
            .text_color(rgb(DIM))
            .opacity(if visible || focused { 1. } else { 0. })
            .hover(move |b| b.text_color(rgb(if remove { DANGER } else { ACCENT })))
            .child(if remove { "✕" } else { "+" }),
        is_enabled,
    )
}
pub fn lyrics_editor_word_strip() -> Div {
    div()
        .relative()
        .h(px(62.))
        .mx(px(6.))
        .mb(px(6.))
        .flex_none()
        .rounded(px(12.))
        .bg(rgb(PANEL_DEEP).alpha(0.6))
        .border_1()
        .border_color(mix(rgb(ACCENT), rgba(LINE), 0.22))
        .overflow_hidden()
        .cursor_pointer()
}
pub fn lyrics_editor_word(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    odd_row: bool,
    dragging: bool,
    focused: bool,
) -> Stateful<Div> {
    div()
        .id(id)
        .absolute()
        .top(px(if odd_row { 32. } else { 7. }))
        .h(px(24.))
        .rounded(px(8.))
        .border_1()
        .border_color(rgb(ACCENT).alpha(if dragging || focused { 1. } else { 0.45 }))
        .bg(if dragging {
            rgb(PANEL).blend(rgb(ACCENT).alpha(0.2))
        } else {
            rgb(PANEL).alpha(0.82)
        })
        .text_color(rgb(TEXT))
        .font_family(DISPLAY_600)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(11.))
        .line_height(px(22.))
        .px(px(4.))
        .max_w(relative(0.3))
        .whitespace_nowrap()
        .text_ellipsis()
        .cursor(if dragging {
            CursorStyle::ClosedHand
        } else {
            CursorStyle::OpenHand
        })
        .when(focused, |w| {
            w.shadow(vec![BoxShadow {
                color: rgb(ACCENT).into(),
                offset: point(px(0.), px(0.)),
                blur_radius: px(0.),
                spread_radius: px(1.),
                inset: true,
            }])
        })
        .hover(|w| w.border_color(rgb(ACCENT)))
        .child(label.into())
}
pub fn lyrics_editor_word_playhead() -> Div {
    div()
        .absolute()
        .top_0()
        .bottom_0()
        .w(px(1.))
        .bg(rgb(ACCENT))
        .shadow(vec![crate::glow(ACCENT, 0.7, 6.)])
}
pub fn lyrics_editor_replacement_input(state: &Entity<TextareaState>, focused: bool) -> Textarea {
    Textarea::new(state)
        .appearance(false)
        .bordered(false)
        .flex_1()
        .min_h_0()
        .border_1()
        .border_color(if focused {
            rgb(ACCENT).alpha(0.45)
        } else {
            rgba(LINE).into()
        })
        .rounded(px(12.))
        .bg(rgb(PANEL_DEEP).alpha(0.65))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(15.))
        .line_height(px(25.5))
        .py(px(14.))
        .px(px(16.))
}
pub fn lyrics_editor_footer() -> Div {
    div()
        .relative()
        .flex()
        .items_center()
        .gap(px(8.))
        .mt(px(12.))
        .min_h(px(30.))
        .flex_none()
}
pub fn lyrics_editor_status(label: impl Into<SharedString>, warning: bool) -> Div {
    div()
        .min_w_0()
        .flex_shrink_1()
        .font_family(DISPLAY_400)
        .text_size(px(12.))
        .text_color(rgb(if warning { DANGER } else { DIM }))
        .child(div().min_w_0().child(label.into()))
}
pub fn lyrics_editor_busy_bar(progress: f32) -> Div {
    let progress = if progress.is_finite() {
        progress.clamp(0., 1.)
    } else {
        0.
    };
    div()
        .w(px(64.))
        .h(px(4.))
        .rounded(px(2.))
        .bg(rgb(TEXT).alpha(0.1))
        .overflow_hidden()
        .flex_none()
        .child(div().h_full().w(relative(progress)).bg(rgb(ACCENT)))
}
pub fn lyrics_editor_help_button(id: impl Into<ElementId>, focused: bool) -> Stateful<Div> {
    // Match the transport chip's base geometry without inheriting its hover
    // registration: GPUI permits exactly one hover style per element.
    div()
        .id(id)
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .flex()
        .items_center()
        .justify_center()
        .w(px(26.))
        .h(px(26.))
        .rounded_full()
        .border_1()
        .font_family(DISPLAY_800)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(13.))
        .line_height(px(16.))
        .text_color(rgb(ACCENT))
        .border_color(rgb(ACCENT).alpha(if focused { 1. } else { 0.45 }))
        .bg(rgb(ACCENT).alpha(0.1))
        .cursor_pointer()
        .hover(|b| {
            b.text_color(rgb(ACCENT))
                .border_color(rgb(ACCENT))
                .bg(rgb(ACCENT).alpha(0.18))
        })
        .child("?")
}
pub fn lyrics_editor_help_sheet() -> Div {
    div()
        .absolute()
        .inset_0()
        .flex()
        .items_center()
        .justify_center()
        .bg(rgb(PANEL_DEEP).blend(rgb(BG).alpha(0.12)))
        .occlude()
}
pub fn lyrics_editor_help_card(id: impl Into<ElementId>, card: Size<Pixels>) -> Stateful<Div> {
    div()
        .id(id)
        .w(px(620_f32.min((f32::from(card.width) - 48.).max(1.))))
        .max_h((card.height - px(48.)).max(px(1.)))
        .overflow_y_scroll()
        .restrict_scroll_to_axis()
        .px(px(8.))
        .py(px(4.))
}
pub fn lyrics_editor_help_key(label: impl Into<SharedString>, graphic: bool) -> Div {
    div()
        .flex_none()
        .px(px(6.))
        .py(px(2.))
        .rounded(px(6.))
        .border_1()
        .when(!graphic, |d| d.border_b_2())
        .border_color(rgba(LINE))
        .bg(rgb(TEXT).alpha(0.08))
        .text_color(rgb(TEXT))
        .font_family(if graphic { DISPLAY_600 } else { MONO_400 })
        .text_size(px(10.))
        .whitespace_nowrap()
        .child(label.into())
}
pub fn lyrics_editor_help_description(label: impl Into<SharedString>) -> Div {
    div()
        .text_size(px(12.5))
        .line_height(px(18.125))
        .text_color(rgb(DIM))
        .child(label.into())
}

/// `.pill.small`: primary is the source gradient action, never an amber toggle.
pub fn lyrics_editor_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    primary: bool,
    is_enabled: bool,
) -> Stateful<Div> {
    let base = div()
        .id(id)
        .relative()
        .flex()
        .items_center()
        .justify_center()
        .gap(px(7.))
        .px(px(12.))
        .py(px(6.))
        .rounded_full()
        .flex_none()
        .font_family(DISPLAY_600)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(12.))
        .line_height(px(14.))
        .whitespace_nowrap()
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .on_click(|_, _, cx| cx.stop_propagation())
        .child(crate::tracked_label(label, 0.01));
    let base = if primary {
        base.text_color(rgb(ACCENT_INK))
            .bg(linear_gradient(
                180.,
                linear_color_stop(rgb(ACCENT_LIFT), 0.),
                linear_color_stop(rgb(ACCENT_DEEP), 1.),
            ))
            .shadow(vec![
                BoxShadow {
                    color: white().alpha(0.25),
                    offset: point(px(0.), px(1.)),
                    blur_radius: px(0.),
                    spread_radius: px(0.),
                    inset: true,
                },
                BoxShadow {
                    color: rgb(0xff9628).alpha(0.28).into(),
                    offset: point(px(0.), px(6.)),
                    blur_radius: px(11.),
                    spread_radius: px(0.),
                    inset: false,
                },
            ])
            .when(is_enabled, |b| {
                b.hover(|s| {
                    s.top(px(-1.)).shadow(vec![
                        BoxShadow {
                            color: white().alpha(0.25),
                            offset: point(px(0.), px(1.)),
                            blur_radius: px(0.),
                            spread_radius: px(0.),
                            inset: true,
                        },
                        BoxShadow {
                            color: rgb(0xff9628).alpha(0.4).into(),
                            offset: point(px(0.), px(8.)),
                            blur_radius: px(14.),
                            spread_radius: px(0.),
                            inset: false,
                        },
                    ])
                })
            })
    } else {
        base.border_1()
            .border_color(rgba(LINE))
            .text_color(rgb(TEXT))
            .when(is_enabled, |b| {
                b.hover(|s| s.top(px(-1.)).border_color(rgba(LINE_STRONG)))
            })
    };
    enabled(base, is_enabled)
}

pub fn lyrics_editor_play(id: impl Into<ElementId>, playing: bool) -> Stateful<Div> {
    lyrics_editor_button(id, if playing { "❚❚" } else { "▶" }, false, true).min_w(px(44.))
}

pub fn lyrics_editor_linkish(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    is_enabled: bool,
) -> Stateful<Div> {
    enabled(
        div()
            .id(id)
            .flex_none()
            .py(px(2.))
            .px(px(4.))
            .rounded(px(6.))
            .font_family(DISPLAY_600)
            .font_weight(FontWeight::NORMAL)
            .text_size(px(12.))
            .line_height(px(14.))
            .text_color(rgb(ACCENT))
            .whitespace_nowrap()
            .when(is_enabled, |b| b.hover(|s| s.bg(rgba(ACCENT_SOFT))))
            .child(label.into()),
        is_enabled,
    )
}

pub fn lyrics_editor_help_heading(label: impl Into<SharedString>) -> Div {
    div()
        .font_family(DISPLAY_700)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(17.))
        .line_height(px(20.5))
        .mb(px(14.))
        .child(label.into())
}

pub fn lyrics_editor_help_section(label: impl Into<SharedString>) -> Div {
    let label: SharedString = label.into();
    div()
        .font_family(DISPLAY_700)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(11.))
        .line_height(px(13.))
        .text_color(rgb(ACCENT))
        .mb(px(8.))
        .child(crate::tracked_label(label.to_uppercase(), 0.08))
}

pub fn lyrics_editor_help_columns() -> Div {
    div().grid().grid_cols(2).gap_x(px(28.)).gap_y(px(16.))
}

pub fn lyrics_editor_help_row(keys: Div, description: impl Into<SharedString>) -> Div {
    div()
        .flex()
        .items_baseline()
        .gap(px(10.))
        .mb(px(7.))
        .child(
            div()
                .w(px(96.))
                .flex_none()
                .flex()
                .justify_end()
                .child(keys.flex().flex_wrap().justify_end().gap(px(4.))),
        )
        .child(
            lyrics_editor_help_description(description)
                .flex_1()
                .min_w_0(),
        )
}
