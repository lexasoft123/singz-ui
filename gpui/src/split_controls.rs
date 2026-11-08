//! Shared source transport-popover and action styles.
use crate::tokens::*;
use gpui_kit::{prelude::FluentBuilder, *};
pub fn transport_popover() -> Div {
    div()
        .occlude()
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .p(px(14.))
        .border_1()
        .border_color(rgba(LINE_STRONG))
        .rounded(px(12.))
        .bg(rgb(0x1d1915))
        .shadow(vec![BoxShadow {
            color: rgba(0x00000080).into(),
            offset: point(px(0.), px(12.)),
            blur_radius: px(17.),
            spread_radius: px(0.),
            inset: false,
        }])
}
pub fn split_action(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    enabled: bool,
) -> Stateful<Div> {
    div()
        .id(id)
        .w_full()
        .py(px(10.))
        .px(px(8.))
        .rounded(px(8.))
        .text_size(px(14.))
        .line_height(px(18.))
        .text_color(rgb(TEXT))
        .child(label.into())
        .when(enabled, |d| {
            d.cursor_pointer()
                .hover(|d| d.bg(rgba(ACCENT_SOFT)).text_color(rgb(ACCENT)))
        })
        .when(!enabled, |d| d.opacity(0.45).cursor_default())
}

/// Footer separation status (`.sep-pill`), replacing the split actions while a job runs.
/// The host supplies a 1.6-second phase so hidden captures can pin animation time.
pub fn split_progress(label: impl Into<SharedString>, percent: f32, phase: f32) -> Div {
    div()
        .relative()
        .flex()
        .items_center()
        .gap(px(12.))
        .pl(px(18.))
        .pr(px(10.))
        .py(px(9.))
        .rounded(px(999.))
        .border_1()
        .border_color(rgba(0xffa02859))
        .bg(rgba(ACCENT_SOFT))
        .font_family(DISPLAY_600)
        .text_size(px(13.))
        .line_height(px(15.))
        .text_color(rgb(ACCENT))
        .child(
            canvas(
                |b, _, _| b,
                move |b, _, w, _| {
                    // Two-stop native gradients meet at each shimmer peak. Full rounded
                    // quads under rectangular half masks retain the pill's outer caps.
                    let width = f32::from(b.size.width);
                    if width <= 0. {
                        return;
                    }
                    for peak in [
                        -1. + phase.rem_euclid(1.) * 2.,
                        1. + phase.rem_euclid(1.) * 2.,
                    ] {
                        let x = b.left() + px(width * peak);
                        for (left, right, from, to) in [
                            (
                                b.left(),
                                x.min(b.right()),
                                linear_color_stop(rgba(0xffaa3c00), peak - 0.4),
                                linear_color_stop(rgba(0xffaa3c17), peak),
                            ),
                            (
                                x.max(b.left()),
                                b.right(),
                                linear_color_stop(rgba(0xffaa3c17), peak),
                                linear_color_stop(rgba(0xffaa3c00), peak + 0.4),
                            ),
                        ] {
                            if right > left {
                                w.with_content_mask(
                                    Some(ContentMask {
                                        bounds: Bounds::new(
                                            point(left, b.top()),
                                            size(right - left, b.size.height),
                                        ),
                                    }),
                                    |w| {
                                        w.paint_quad(quad(
                                            b,
                                            px(999.),
                                            linear_gradient(90., from, to),
                                            px(0.),
                                            transparent_black(),
                                            BorderStyle::Solid,
                                        ));
                                    },
                                );
                            }
                        }
                    }
                },
            )
            .absolute()
            .top_0()
            .left_0()
            .size_full(),
        )
        .child(div().relative().child(label.into()))
        .child(
            div()
                .relative()
                .min_w(px(38.))
                .text_right()
                .font_family("SingZ Mono 600")
                .text_size(px(12.))
                .line_height(px(14.))
                .child(format!("{}%", percent.clamp(0., 100.).round() as u32)),
        )
}

pub fn split_cancel(id: impl Into<ElementId>) -> Stateful<Div> {
    div()
        .id(id)
        .relative()
        .size(px(24.))
        .flex_shrink_0()
        .rounded(px(999.))
        .border_1()
        .border_color(rgba(0xffa02859))
        .text_color(rgb(ACCENT))
        .font_family(DISPLAY_600)
        .text_size(px(15.))
        .line_height(px(15.))
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .hover(|d| d.bg(rgba(0xffa0282e)))
        .child("×")
}
