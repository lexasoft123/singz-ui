//! Production LyricsPanel source/status/search presentation; no glyph rendering.
use crate::tokens::*;
use gpui_kit::{
    component::input::{Input, InputState},
    prelude::FluentBuilder,
    *,
};

pub fn lyrics_sidebar_source(w: &Window) -> Div {
    div()
        .flex()
        .items_center()
        .flex_wrap()
        .gap_x(px(8.))
        .gap_y(px(4.))
        .px(px(18.))
        .py(px(9.))
        .border_b_1()
        .border_color(rgba(LINE))
        .text_size(px(11.5))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_400, 11.5).0)
        .text_color(rgb(DIM))
        .min_w_0()
        .flex_shrink_0()
}
pub fn lyrics_sidebar_badge(source: &str) -> Div {
    lyrics_sidebar_badge_label(source, match source {"lrclib"=>"Synced","edited"=>"Edited",_=>"AI transcribed"})
}
pub fn lyrics_sidebar_badge_label(source: &str, label: impl Into<SharedString>) -> Div {
    let label:SharedString=label.into();
    let (_, color, alpha, line) = match source {
        "lrclib" => ("Synced", 0x58d68a, 0.12, 0.4),
        "edited" => ("Edited", ACCENT, 0.12, 0.4),
        _ => ("AI transcribed", 0x45d6b5, 0.1, 0.35),
    };
    div()
        .flex_none()
        .px(px(8.))
        .py(px(2.))
        .rounded_full()
        .border_1()
        .border_color(rgb(color).alpha(line))
        .bg(rgb(color).alpha(alpha))
        .font_family(DISPLAY_700)
        .font_weight(FontWeight::NORMAL)
        .child(crate::tracked_text(
            label.to_uppercase(),
            DISPLAY_700,
            10.,
            0.4,
            color,
        ))
}
pub fn lyrics_sidebar_credit(label: impl Into<SharedString>) -> Div {
    div()
        .flex_1()
        .flex_basis(px(140.))
        .min_w(px(140.))
        .overflow_hidden()
        .text_ellipsis()
        .whitespace_nowrap()
        .child(label.into())
}
pub fn lyrics_sidebar_link(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    enabled: bool,
) -> Stateful<Div> {
    crate::lyrics_editor_linkish(id, label, enabled)
}
pub fn lyrics_sidebar_check(label: impl Into<SharedString>, warn: bool, w: &Window) -> Div {
    div()
        .flex_shrink_0()
        .px(px(18.))
        .py(px(7.))
        .border_b_1()
        .border_color(rgba(LINE))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(11.))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_400, 11.).0)
        .opacity(0.9)
        .text_color(rgb(if warn { ACCENT } else { DIM }))
        .when(warn, |d| d.bg(rgba(ACCENT_SOFT)))
        .child(label.into())
}
pub fn lyrics_sidebar_body(id: impl Into<ElementId>, viewport_height: Pixels) -> Stateful<Div> {
    div()
        .id(id)
        .relative()
        .flex_1()
        .min_h_0()
        .overflow_y_scroll()
        .pt(px(18.))
        .px(px(6.))
        .pb(viewport_height * 0.4)
}
pub fn lyrics_sidebar_state() -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(12.))
        .items_start()
        .py(px(10.))
        .px(px(14.))
}
pub fn lyrics_sidebar_paragraph(text: impl Into<SharedString>) -> Div {
    div()
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(13.))
        .line_height(px(20.8))
        .text_color(rgb(DIM))
        .child(text.into())
}
pub fn lyrics_sidebar_rich(parts: Vec<(String, bool)>) -> Div {
    let text = parts.iter().map(|(s, _)| s.as_str()).collect::<String>();
    let runs = parts
        .iter()
        .map(|(s, strong)| TextRun {
            len: s.len(),
            font: Font {
                family: if *strong { DISPLAY_700 } else { DISPLAY_400 }.into(),
                ..Default::default()
            },
            color: rgb(if *strong { TEXT } else { DIM }).into(),
            background_color: None,
            underline: None,
            strikethrough: None,
        })
        .collect::<Vec<_>>();
    div()
        .text_size(px(13.))
        .line_height(px(20.8))
        .child(StyledText::new(text).with_runs(runs))
}
pub fn lyrics_sidebar_fine(text: impl Into<SharedString>, warn: bool) -> Div {
    div()
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(12.))
        .line_height(px(19.2))
        .text_color(rgb(if warn { DANGER } else { FAINT }))
        .when(warn, |d| d.pb(px(4.)))
        .child(text.into())
}
pub fn lyrics_sidebar_loading(
    label: impl Into<SharedString>,
    percent: impl Into<SharedString>,
) -> Div {
    div()
        .flex()
        .items_baseline()
        .text_size(px(13.))
        .line_height(px(20.8))
        .font_family(DISPLAY_600)
        .font_weight(FontWeight::NORMAL)
        .text_color(rgb(TEXT))
        .child(format!("{} ", label.into()))
        .child(
            div()
                .font_family(MONO_400)
                .text_size(px(11.))
                .text_color(rgb(ACCENT))
                .child(percent.into()),
        )
}
pub fn lyrics_sidebar_bar(percent: f32) -> Div {
    div()
        .w_full()
        .h(px(3.))
        .rounded(px(2.))
        .bg(rgba(0xfff0d614))
        .overflow_hidden()
        .child(
            div()
                .h_full()
                .w(relative(percent.clamp(0., 100.) / 100.))
                .bg(linear_gradient(
                    90.,
                    linear_color_stop(rgb(ACCENT_DEEP), 0.),
                    linear_color_stop(rgb(0xffd489), 1.),
                ))
                .shadow(vec![crate::glow(0xffaa3c, 0.7, 8.)]),
        )
}
pub fn lyrics_sidebar_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    primary: bool,
    small: bool,
    enabled: bool,
) -> Stateful<Div> {
    let button = crate::lyrics_editor_button(id, label, primary, enabled);
    if small {
        button
    } else {
        button
            .px(px(16.))
            .py(px(9.))
            .text_size(px(13.))
            .line_height(px(15.5))
    }
}
pub fn lyrics_sidebar_variants() -> Div {
    div().flex().flex_col().gap(px(6.)).py(px(4.)).px(px(12.))
}
pub fn lyrics_sidebar_variants_top() -> Div {
    div().flex().gap(px(6.)).pt(px(2.))
}
pub fn lyrics_sidebar_chip(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .flex_none()
        .h(px(26.))
        .px(px(10.))
        .rounded(px(7.))
        .border_1()
        .border_color(rgba(LINE))
        .font_family(DISPLAY_600)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(11.))
        .text_color(rgb(DIM))
        .cursor_pointer()
        .hover(|s| s.border_color(rgba(LINE_STRONG)).text_color(rgb(TEXT)))
        .child(label.into())
}
pub fn lyrics_sidebar_search_row() -> Div {
    div().flex().gap(px(8.)).mb(px(8.))
}
pub fn lyrics_sidebar_search_input(state: &Entity<InputState>, focused: bool) -> Input {
    let input = Input::new(state)
        .flex_1()
        .min_w_0()
        .bg(rgb(PANEL_DEEP))
        .border_1()
        .border_color(rgba(if focused { 0xffa02880 } else { LINE }))
        .rounded_full()
        .text_color(rgb(TEXT))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(13.))
        .line_height(px(15.5))
        .py(px(8.))
        .px(px(14.));
    Styled::h(input, px(33.5))
}
pub fn lyrics_sidebar_variant(id: impl Into<ElementId>, enabled: bool) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .flex_col()
        .items_start()
        .gap(px(2.))
        .py(px(8.))
        .px(px(12.))
        .rounded(px(10.))
        .border_1()
        .border_color(rgba(0))
        .when(enabled, |d| {
            d.cursor_pointer()
                .hover(|s| s.bg(rgba(0xfff0d60d)).border_color(rgba(LINE)))
        })
        .when(!enabled, |d| d.opacity(0.4).cursor_default())
}
pub fn lyrics_sidebar_variant_main(track: &str, artist: &str, w: &Window) -> Div {
    let text = format!("{track} {artist}");
    let runs = vec![
        TextRun {
            len: track.len() + 1,
            font: Font {
                family: DISPLAY_650.into(),
                ..Default::default()
            },
            color: rgb(TEXT).into(),
            background_color: None,
            underline: None,
            strikethrough: None,
        },
        TextRun {
            len: artist.len(),
            font: Font {
                family: DISPLAY_500.into(),
                ..Default::default()
            },
            color: rgb(DIM).into(),
            background_color: None,
            underline: None,
            strikethrough: None,
        },
    ];
    div()
        .text_size(px(13.5))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_650, 13.5).0)
        .child(StyledText::new(text).with_runs(runs))
}
pub fn lyrics_sidebar_variant_meta(text: impl Into<SharedString>) -> Div {
    div().child(crate::tracked_text(text, MONO_400, 10., 0.3, FAINT))
}

pub fn lyrics_sidebar_variant_empty(w: &Window) -> Div {
    lyrics_sidebar_variant_empty_text("Nothing found — try other words.",w)
}
pub fn lyrics_sidebar_variant_empty_text(label:impl Into<SharedString>,w: &Window) -> Div {
    div()
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(12.))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_400, 12.).0)
        .text_color(rgb(FAINT))
        .child(label.into())
}

/// Source `.lp-bar div` 400ms ease transition, isolated from the rest of the UI.
pub struct LyricsSidebarProgress {
    from: f32,
    target: f32,
    changed_at: std::time::Instant,
    initialized: bool,
    frame_pending: bool,
}
impl LyricsSidebarProgress {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            from: 0.,
            target: 0.,
            changed_at: std::time::Instant::now(),
            initialized: false,
            frame_pending: false,
        }
    }
    fn value(&self) -> f32 {
        let x = (self.changed_at.elapsed().as_secs_f32() / 0.4).clamp(0., 1.);
        let curve = |t: f32, a: f32, b: f32| {
            3. * (1. - t).powi(2) * t * a + 3. * (1. - t) * t * t * b + t * t * t
        };
        let (mut lo, mut hi) = (0., 1.);
        for _ in 0..20 {
            let t = (lo + hi) / 2.;
            if curve(t, 0.25, 0.25) < x {
                lo = t;
            } else {
                hi = t;
            }
        }
        let y = if x == 0. || x == 1. {
            x
        } else {
            curve((lo + hi) / 2., 0.1, 1.)
        };
        self.from + (self.target - self.from) * y
    }
    pub fn set(&mut self, percent: f32, fresh: bool, cx: &mut Context<Self>) {
        let percent = if percent.is_finite() {
            percent.clamp(0., 100.)
        } else {
            0.
        };
        if !self.initialized || fresh {
            self.from = percent;
            self.target = percent;
            self.initialized = true;
            cx.notify();
        } else if self.target != percent {
            self.from = self.value();
            self.target = percent;
            self.changed_at = std::time::Instant::now();
            cx.notify();
        }
    }
    pub fn percent(&self) -> f32 {
        self.value()
    }
}
impl Render for LyricsSidebarProgress {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.from != self.target
            && self.changed_at.elapsed().as_secs_f32() < 0.4
            && !self.frame_pending
        {
            self.frame_pending = true;
            let weak = cx.weak_entity();
            w.on_next_frame(move |_, cx| {
                let _ = weak.update(cx, |s, cx| {
                    s.frame_pending = false;
                    cx.notify();
                });
            });
            w.request_animation_frame();
        }
        lyrics_sidebar_bar(self.value())
    }
}

/// CSS focus-visible outline, using native keyboard modality and the control's
/// actual border/radius. This also works when Root handles Tab as an action.
pub fn lyrics_sidebar_focus(mut control: Stateful<Div>, handle: FocusHandle) -> Stateful<Div> {
    let style = control.style();
    let borders = style.border_widths.clone();
    let radius = style.corner_radii.top_left.clone();
    control.relative().child(
        canvas(
            |b, _, _| b,
            move |b, _, w, _| {
                if handle.is_focused(w) && w.last_input_was_keyboard() {
                    let rem = w.rem_size();
                    let left = borders.left.map(|n| n.to_pixels(rem)).unwrap_or(px(0.));
                    let top = borders.top.map(|n| n.to_pixels(rem)).unwrap_or(px(0.));
                    let right = borders.right.map(|n| n.to_pixels(rem)).unwrap_or(px(0.));
                    let bottom = borders.bottom.map(|n| n.to_pixels(rem)).unwrap_or(px(0.));
                    let rect = Bounds::new(
                        b.origin - point(left + px(4.), top + px(4.)),
                        size(
                            b.size.width + left + right + px(8.),
                            b.size.height + top + bottom + px(8.),
                        ),
                    );
                    w.paint_quad(quad(
                        rect,
                        radius.map(|n| n.to_pixels(rem) + px(4.)).unwrap_or(px(4.)),
                        transparent_black(),
                        px(2.),
                        rgba(FOCUS_RING),
                        BorderStyle::Solid,
                    ));
                }
            },
        )
        .absolute()
        .inset_0(),
    )
}
/// The source AI caveat belongs after the actual lyric lines in their scroll
/// container, never in a fixed footer or a second glyph renderer.
pub fn lyrics_sidebar_note() -> Div {
    div()
        .pt(px(22.))
        .px(px(14.))
        .opacity(0.75)
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(12.))
        .line_height(px(14.5))
        .text_color(rgb(FAINT))
        .child("AI-transcribed from the vocals — not always perfect. ")
}
pub fn lyrics_sidebar_note_link(id: impl Into<ElementId>) -> Stateful<Div> {
    lyrics_sidebar_link(id, "Fix the words", true)
}
