//! Native painters for the source DSP runtime graph card.
//! Topology, locale and host ownership stay with the application.
use crate::tokens::*;
use gpui_kit::{prelude::*, *};

/// Runtime graph heading, format facts and state from DspGraphVisualization.
pub fn runtime_graph_card(
    eyebrow: impl Into<SharedString>,
    title: impl Into<SharedString>,
    facts: Vec<String>,
    state: impl Into<SharedString>,
    running: bool,
    fault: bool,
) -> Div {
    let color = if fault {
        DANGER
    } else if running {
        ACCENT
    } else {
        FAINT
    };
    div()
        .relative()
        .mt(px(12.))
        .rounded(px(9.))
        .overflow_hidden()
        .border_1()
        .border_color(rgba(LINE_STRONG))
        .bg(rgb(0x110f0c))
        .child(
            div()
                .absolute()
                .top_0()
                .left_0()
                .right_0()
                .h(px(52.))
                .bg(linear_gradient(
                    180.,
                    linear_color_stop(rgba(0x1b181494), 0.),
                    linear_color_stop(rgba(0x1b181400), 1.),
                )),
        )
        .child(
            div()
                .relative()
                .min_h(px(56.))
                .px(px(10.))
                .py(px(8.))
                .border_b_1()
                .border_color(rgba(LINE))
                .child(
                    div()
                        .flex()
                        .gap(px(12.))
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .min_w(px(120.))
                                .flex_1()
                                .child(
                                    div()
                                        .font_family(MONO_400)
                                        .text_color(rgb(FAINT))
                                        .text_size(px(7.))
                                        .line_height(px(7.))
                                        .child(crate::tracked_label(
                                            eyebrow.into().to_string().to_uppercase(),
                                            0.11,
                                        )),
                                )
                                .child(
                                    div()
                                        .mt(px(3.))
                                        .font_family(DISPLAY_700)
                                        .text_color(rgb(TEXT))
                                        .text_size(px(11.))
                                        .line_height(px(13.))
                                        .child(crate::tracked_label(title.into(), 0.01)),
                                ),
                        )
                        .child(
                            div()
                                .flex_none()
                                .flex()
                                .gap(px(6.))
                                .items_center()
                                .font_family(MONO_400)
                                .text_color(rgb(color))
                                .text_size(px(8.))
                                .line_height(px(8.))
                                .child(div().size(px(6.)).rounded_full().bg(rgb(color)).shadow(
                                    vec![BoxShadow {
                                            color: rgb(color)
                                                .alpha(if running {
                                                    0.17
                                                } else if fault {
                                                    0.16
                                                } else {
                                                    0.14
                                                })
                                                .into(),
                                            offset: point(px(0.), px(0.)),
                                            blur_radius: px(0.),
                                            spread_radius: px(2.),
                                            inset: false,
                                        }],
                                ))
                                .child(state.into()),
                        ),
                )
                .child(
                    div()
                        .mt(px(12.))
                        .flex()
                        .gap(px(4.))
                        .items_center()
                        .children(facts.into_iter().map(|text| {
                            div()
                                .font_family(MONO_400)
                                .text_color(rgb(FAINT))
                                .text_size(px(7.))
                                .line_height(px(7.))
                                .px(px(5.))
                                .py(px(3.))
                                .border_1()
                                .border_color(rgba(LINE))
                                .rounded(px(3.))
                                .bg(rgb(PANEL).alpha(0.8))
                                .child(crate::tracked_label(text, 0.02))
                        })),
                ),
        )
}

/// Source playback module face. Each face is the sole child of its list item,
/// so the source `:not(:last-child)` connector does not apply to playback.
/// Actual bus edges are listed separately without implying a linear chain.
pub fn runtime_graph_node(
    kind: impl Into<SharedString>,
    label: impl Into<SharedString>,
    value: impl Into<SharedString>,
    detail: impl Into<SharedString>,
    running: bool,
) -> Div {
    div()
        .relative()
        .flex_1()
        .w(px(0.))
        .min_w(px(104.))
        .min_h(px(108.))
        .px(px(8.))
        .pt(px(10.))
        .pb(px(9.))
        .flex()
        .flex_col()
        .rounded(px(6.))
        .border_1()
        .border_color(if running {
            rgba(0xffa6348a)
        } else {
            rgba(LINE)
        })
        .bg(if running {
            rgba(0x1e1a14e9)
        } else {
            rgb(PANEL).alpha(0.86)
        })
        .shadow(vec![BoxShadow {
            color: rgba(0xffffff06).into(),
            offset: point(px(0.), px(1.)),
            blur_radius: px(0.),
            spread_radius: px(0.),
            inset: true,
        }])
        .child(
            div()
                .font_family(MONO_400)
                .text_color(rgb(FAINT))
                .text_size(px(7.))
                .line_height(px(7.))
                .child(crate::tracked_label(kind.into().to_string().to_uppercase(), 0.1).wrap()),
        )
        .child(
            div()
                .mt(px(5.))
                .font_family(DISPLAY_700)
                .text_color(rgb(if running { ACCENT } else { DIM }))
                .text_size(px(10.))
                .line_height(px(11.5))
                .truncate()
                .child(label.into()),
        )
        .child(
            div()
                .mt(px(5.))
                .font_family(MONO_700)
                .text_color(rgb(if running { ACCENT } else { TEXT }))
                .text_size(px(10.))
                .line_height(px(11.5))
                .truncate()
                .child(value.into()),
        )
        .child(
            div()
                .mt_auto()
                .pt(px(7.))
                .font_family(MONO_400)
                .text_color(rgb(FAINT))
                .text_size(px(7.))
                .line_height(px(9.1))
                .max_h(px(34.3))
                .overflow_hidden()
                .line_clamp(3)
                .child(detail.into()),
        )
}

pub fn runtime_graph_connections() -> Div {
    div()
        .mt(px(12.))
        .pt(px(10.))
        .border_t_1()
        .border_color(rgba(LINE))
}

pub fn runtime_graph_connection(
    route: impl Into<SharedString>,
    latency: impl Into<SharedString>,
    following: bool,
) -> Div {
    div()
        .flex()
        .flex_wrap()
        .gap_y(px(6.))
        .gap_x(px(14.))
        .font_family(MONO_400)
        .text_color(rgb(FAINT))
        .text_size(px(8.))
        .line_height(px(11.6))
        .when(following, |row| row.mt(px(5.)))
        .child(
            div()
                .font_family(MONO_500)
                .text_color(rgb(DIM))
                .child(route.into()),
        )
        .child(latency.into())
}

/// Source viewport focus outline is inset, so focus does not change layout.
/// Apply to a stationary wrapper around the scroller: an overlay inside the
/// scrollable element would translate with its content and lose the viewport.
pub fn runtime_graph_viewport_focus(control: Stateful<Div>, focus: FocusHandle) -> Stateful<Div> {
    let pointer = focus.clone();
    control
        .track_focus(&focus)
        .tab_index(0)
        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
            pointer.focus(window, cx)
        })
        .relative()
        .child(
            canvas(
                |bounds, _, _| bounds,
                move |bounds, _, window, _| {
                    if focus.is_focused(window) && window.last_input_was_keyboard() {
                        let inset = Bounds::new(
                            bounds.origin + point(px(3.), px(3.)),
                            size(
                                (bounds.size.width - px(6.)).max(px(0.)),
                                (bounds.size.height - px(6.)).max(px(0.)),
                            ),
                        );
                        window.paint_quad(quad(
                            inset,
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
        )
}
