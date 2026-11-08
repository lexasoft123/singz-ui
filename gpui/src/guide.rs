//! Shared anchored-tour measurement and paint. Hosts own copy and navigation.
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};
#[derive(Clone)]
pub struct GuideAnchors {
    bounds: Rc<RefCell<HashMap<&'static str, Bounds<Pixels>>>>,
    active: Rc<Cell<bool>>,
    changed: Rc<dyn Fn(&mut Window, &mut App)>,
}
impl Default for GuideAnchors {
    fn default() -> Self {
        Self::new(|_, _| {})
    }
}
impl GuideAnchors {
    pub fn new(changed: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        Self {
            bounds: Default::default(),
            active: Default::default(),
            changed: Rc::new(changed),
        }
    }
    pub fn set_active(&self, value: bool) {
        self.active.set(value);
    }
    pub fn get(&self, id: &str) -> Option<Bounds<Pixels>> {
        self.bounds.borrow().get(id).copied()
    }
    pub fn track<E: Styled + ParentElement>(&self, element: E, id: &'static str) -> E {
        self.track_box(element, id, [0.; 4])
    }
    /// Absolute measurement canvases exclude borders. Include left/top/right/bottom
    /// widths to report the same border box as DOM getBoundingClientRect().
    pub fn track_box<E: Styled + ParentElement>(
        &self,
        element: E,
        id: &'static str,
        border: [f32; 4],
    ) -> E {
        let anchors = self.clone();
        element.relative().child(
            canvas(
                move |b, w, cx| {
                    let bounds = Bounds::new(
                        b.origin - point(px(border[0]), px(border[1])),
                        size(
                            b.size.width + px(border[0] + border[2]),
                            b.size.height + px(border[1] + border[3]),
                        ),
                    );
                    let changed = anchors.bounds.borrow_mut().insert(id, bounds) != Some(bounds);
                    if changed && anchors.active.get() {
                        (anchors.changed)(w, cx);
                    }
                    b
                },
                |_, _, _, _| {},
            )
            .absolute()
            .top_0()
            .left_0()
            .size_full(),
        )
    }
}
fn rounded_path(p: &mut PathBuilder, b: Bounds<Pixels>, radius: f32) {
    let r = px(radius
        .min(f32::from(b.size.width) / 2.)
        .min(f32::from(b.size.height) / 2.));
    let k = r * 0.5522848;
    p.move_to(point(b.left() + r, b.top()));
    p.line_to(point(b.right() - r, b.top()));
    p.cubic_bezier_to(
        point(b.right(), b.top() + r),
        point(b.right() - r + k, b.top()),
        point(b.right(), b.top() + r - k),
    );
    p.line_to(point(b.right(), b.bottom() - r));
    p.cubic_bezier_to(
        point(b.right() - r, b.bottom()),
        point(b.right(), b.bottom() - r + k),
        point(b.right() - r + k, b.bottom()),
    );
    p.line_to(point(b.left() + r, b.bottom()));
    p.cubic_bezier_to(
        point(b.left(), b.bottom() - r),
        point(b.left() + r - k, b.bottom()),
        point(b.left(), b.bottom() - r + k),
    );
    p.line_to(point(b.left(), b.top() + r));
    p.cubic_bezier_to(
        point(b.left() + r, b.top()),
        point(b.left(), b.top() + r - k),
        point(b.left() + r - k, b.top()),
    );
    p.close();
}
pub fn guide_shade(hole: Option<Bounds<Pixels>>) -> impl IntoElement {
    canvas(
        |b, _, _| b,
        move |b, _, w, _| {
            let mut p = PathBuilder::fill().with_style(PathStyle::Fill(
                FillOptions::default().with_fill_rule(FillRule::EvenOdd),
            ));
            rounded_path(&mut p, b, 0.);
            if let Some(hole) = hole {
                rounded_path(&mut p, hole, 8.);
            }
            if let Ok(path) = p.build() {
                w.paint_path(path, rgb(0).alpha(0.7));
            }
            if let Some(h) = hole {
                let b = Bounds::new(
                    h.origin - point(px(1.), px(1.)),
                    h.size + size(px(2.), px(2.)),
                );
                w.paint_quad(quad(
                    b,
                    px(9.),
                    transparent_black(),
                    px(2.),
                    rgb(0xffa028),
                    BorderStyle::Solid,
                ));
            }
        },
    )
    .absolute()
    .inset_0()
}
pub fn guide_balloon() -> Div {
    div()
        .relative()
        .rounded(px(14.))
        .border_1()
        .border_color(rgb(0x6e5030))
        .bg(rgb(0x272019))
        .text_color(rgb(crate::tokens::TEXT))
        .font_family(crate::tokens::DISPLAY_400)
        .shadow(vec![BoxShadow {
            color: rgba(0x00000099).into(),
            offset: point(px(0.), px(12.)),
            blur_radius: px(24.),
            spread_radius: px(0.),
            inset: false,
        }])
}
/// CSS's rotated 14px square, with only its two outward border edges.
pub fn guide_arrow(side: &'static str, offset: f32) -> Div {
    let d = 9.899495;
    let at = match side {
        "top" => point(px(offset - d), px(-d)),
        "bottom" => point(px(offset - d), px(-d)),
        "left" => point(px(-d), px(offset - d)),
        _ => point(px(-d), px(offset - d)),
    };
    div()
        .absolute()
        .when(side == "top", |x| x.top(at.y).left(at.x))
        .when(side == "bottom", |x| x.top_full().mt(at.y).left(at.x))
        .when(side == "left", |x| x.left(at.x).top(at.y))
        .when(side == "right", |x| x.left_full().ml(at.x).top(at.y))
        .size(px(d * 2.))
        .child(
            canvas(
                |b, _, _| b,
                move |b, _, w, _| {
                    let center = b.center();
                    let points = [
                        point(center.x, b.top()),
                        point(b.right(), center.y),
                        point(center.x, b.bottom()),
                        point(b.left(), center.y),
                    ];
                    let mut fill = PathBuilder::fill();
                    fill.move_to(points[0]);
                    for p in &points[1..] {
                        fill.line_to(*p);
                    }
                    fill.close();
                    if let Ok(p) = fill.build() {
                        w.paint_path(p, rgb(0x272019));
                    }
                    let indexes = match side {
                        "top" => [3, 0, 1],
                        "bottom" => [1, 2, 3],
                        "left" => [2, 3, 0],
                        _ => [0, 1, 2],
                    };
                    let mut stroke = PathBuilder::stroke(px(1.));
                    stroke.move_to(points[indexes[0]]);
                    stroke.line_to(points[indexes[1]]);
                    stroke.line_to(points[indexes[2]]);
                    if let Ok(p) = stroke.build() {
                        w.paint_path(p, rgb(0x6e5030));
                    }
                },
            )
            .size_full(),
        )
}

/// Source `.pill.small` with primary or ghost treatment.
pub fn guide_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    primary: bool,
) -> Stateful<Div> {
    let label: SharedString = label.into();
    div()
        .id(id)
        .relative()
        .flex()
        .items_center()
        .justify_center()
        .gap(px(7.))
        .px(px(12.))
        .py(px(6.))
        .rounded_full()
        .font_family(crate::tokens::DISPLAY_600)
        .text_size(px(12.))
        .line_height(px(14.5))
        .cursor_pointer()
        .when(!primary, |d| {
            d.border_1()
                .border_color(rgba(crate::tokens::LINE))
                .text_color(rgb(crate::tokens::TEXT))
                .hover(|s| {
                    s.border_color(rgba(crate::tokens::LINE_STRONG))
                        .top(px(-1.))
                })
        })
        .when(primary, |d| {
            d.text_color(rgb(crate::tokens::ACCENT_INK))
                .bg(linear_gradient(
                    180.,
                    linear_color_stop(rgb(crate::tokens::ACCENT_LIFT), 0.),
                    linear_color_stop(rgb(crate::tokens::ACCENT_DEEP), 1.),
                ))
                .shadow(vec![
                    BoxShadow {
                        color: rgb(0xffffff).alpha(0.25).into(),
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
                .hover(|s| {
                    s.top(px(-1.)).shadow(vec![
                        BoxShadow {
                            color: rgb(0xffffff).alpha(0.25).into(),
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
        .child(crate::tracked_label(label, 0.01))
}
/// A keyboard ring at CSS outline-offset:2px, without changing button layout.
pub fn guide_focus_ring(
    handle: FocusHandle,
    keyboard: Rc<Cell<bool>>,
    border: f32,
) -> impl IntoElement {
    canvas(
        |b, _, _| b,
        move |b, _, w, _| {
            if keyboard.get() && handle.is_focused(w) {
                let b = Bounds::new(
                    b.origin - point(px(4. + border), px(4. + border)),
                    size(
                        b.size.width + px(8. + border * 2.),
                        b.size.height + px(8. + border * 2.),
                    ),
                );
                w.paint_quad(quad(
                    b,
                    px(999.),
                    transparent_black(),
                    px(2.),
                    rgba(crate::tokens::FOCUS_RING),
                    BorderStyle::Solid,
                ));
            }
        },
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
}
