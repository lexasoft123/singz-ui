//! Nonblocking source `.toast.ok`, including its entrance and six-second expiry.
use crate::tokens::DISPLAY_400;
use gpui_kit::{prelude::FluentBuilder, *};
use std::{
    cell::Cell,
    rc::Rc,
    time::{Duration, Instant},
};

#[derive(Clone, Debug)]
pub struct ToastState {
    pub visible: bool,
    pub text: Option<String>,
    pub sequence: u64,
    pub age_seconds: f32,
    pub animation_seconds: f32,
    pub opacity: f32,
}

pub struct Toast {
    text: Option<SharedString>,
    sequence: u64,
    shown_at: Instant,
    animation_time: Option<f32>,
    expiry: Option<Task<()>>,
    frame_pending: bool,
    bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
}

fn entrance(progress: f32) -> f32 {
    let x = progress.clamp(0., 1.);
    if x == 0. || x == 1. {
        return x;
    }
    let curve = |t: f32, a: f32, b: f32| {
        3. * (1. - t).powi(2) * t * a + 3. * (1. - t) * t * t * b + t * t * t
    };
    let (mut lo, mut hi) = (0., 1.);
    for _ in 0..20 {
        let t = (lo + hi) / 2.;
        if curve(t, 0.2, 0.2) < x {
            lo = t;
        } else {
            hi = t;
        }
    }
    curve((lo + hi) / 2., 0.7, 1.)
}

impl Toast {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            text: None,
            sequence: 0,
            shown_at: Instant::now(),
            animation_time: None,
            expiry: None,
            frame_pending: false,
            bounds: Default::default(),
        }
    }
    pub fn show(&mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.text = Some(text.into());
        self.sequence = self.sequence.wrapping_add(1);
        self.shown_at = Instant::now();
        self.frame_pending = false;
        self.bounds.set(None);
        let sequence = self.sequence;
        // Replacing the task cancels its previous timer. The generation guard
        // also protects against an earlier completion already queued in App.
        self.expiry = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(Duration::from_secs(6)).await;
            let _ = this.update(cx, |this, cx| {
                if this.sequence == sequence {
                    this.text = None;
                    this.bounds.set(None);
                    cx.notify();
                }
            });
        }));
        cx.notify();
    }
    /// Freeze only entrance time for hidden captures; expiry keeps real time.
    pub fn set_animation_time(&mut self, seconds: Option<f32>, cx: &mut Context<Self>) {
        self.animation_time = seconds.map(|s| s.max(0.));
        cx.notify();
    }
    pub fn bounds(&self) -> Option<Bounds<Pixels>> {
        self.bounds.get()
    }
    pub fn state(&self) -> ToastState {
        let age = self.shown_at.elapsed().as_secs_f32();
        let animation = self.animation_time.unwrap_or(age);
        ToastState {
            visible: self.text.is_some(),
            text: self.text.as_ref().map(ToString::to_string),
            sequence: self.sequence,
            age_seconds: age,
            animation_seconds: animation,
            opacity: if self.text.is_some() {
                entrance(animation / 0.25)
            } else {
                0.
            },
        }
    }
}

impl Render for Toast {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let animation = self
            .animation_time
            .unwrap_or_else(|| self.shown_at.elapsed().as_secs_f32());
        if self.text.is_some()
            && self.animation_time.is_none()
            && animation < 0.25
            && !self.frame_pending
        {
            self.frame_pending = true;
            let sequence = self.sequence;
            let weak = cx.weak_entity();
            w.on_next_frame(move |_, cx| {
                let _ = weak.update(cx, |s, cx| {
                    if s.sequence == sequence {
                        s.frame_pending = false;
                        cx.notify();
                    }
                });
            });
            w.request_animation_frame();
        }
        let progress = entrance(animation / 0.25);
        let bounds = self.bounds.clone();
        // No hitbox occlusion, event handler or focus handle: player controls
        // remain usable, and the host inserts modal layers after this entity.
        div()
            .absolute()
            .left_0()
            .right_0()
            .bottom(px(104. - 10. * (1. - progress)))
            .flex()
            .justify_center()
            .opacity(progress)
            .when_some(self.text.clone(), |root, text| {
                root.child(
                    div()
                        .relative()
                        .min_w_0()
                        .max_w(relative(0.72))
                        .bg(rgb(0x241f16))
                        .border_1()
                        .border_color(rgba(0xffa02866))
                        .text_color(rgb(0xf0d9ae))
                        .px(px(18.))
                        .py(px(11.))
                        .rounded(px(12.))
                        .font_family(DISPLAY_400)
                        .font_weight(FontWeight::NORMAL)
                        .text_size(px(13.))
                        .line_height(px(19.5))
                        .shadow(vec![BoxShadow {
                            color: rgba(0x00000080).into(),
                            offset: point(px(0.), px(12.)),
                            blur_radius: px(20.),
                            spread_radius: px(0.),
                            inset: false,
                        }])
                        .child(text)
                        .child(
                            canvas(
                                move |b, _, _| {
                                    bounds.set(Some(b));
                                    b
                                },
                                |_, _, _, _| {},
                            )
                            .absolute()
                            .inset_0(),
                        ),
                )
            })
    }
}
