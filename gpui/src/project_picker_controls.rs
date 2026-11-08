//! Production `.picker-*` presentation. The host owns library IO and focus.
use crate::tokens::*;
use gpui_kit::{prelude::FluentBuilder, *};

pub fn project_picker_card(viewport: Size<Pixels>) -> Div {
    crate::modal_card()
        .w(px(560_f32.min((f32::from(viewport.width) - 60.).max(0.))))
        .max_h(px((f32::from(viewport.height) - 120.).max(0.)))
        .min_h_0()
}
pub fn project_picker_head() -> Div {
    div()
        .flex()
        .items_baseline()
        .gap(px(12.))
        .mb(px(14.))
        .flex_shrink_0()
}
pub fn project_picker_title(label: impl Into<SharedString>, w: &Window) -> Div {
    div()
        .font_family(DISPLAY_800)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(21.))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_800, 21.).0)
        .text_color(rgb(TEXT))
        .child(label.into())
}
pub fn project_picker_head_actions() -> Div {
    div().ml_auto().flex().gap(px(8.)).flex_shrink_0()
}
/// Source `.pill.ghost.small`, sharing the installed native pill implementation.
pub fn project_picker_action(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    enabled: bool,
) -> Stateful<Div> {
    crate::lyrics_editor_button(id, label, false, enabled)
}
pub fn project_picker_rows(id: impl Into<ElementId>) -> Stateful<Div> {
    div()
        .id(id)
        .relative()
        .flex()
        .flex_col()
        .gap(px(6.))
        .min_h_0()
        .overflow_y_scroll()
}
pub fn project_picker_row(id: impl Into<ElementId>, w: &Window) -> Stateful<Div> {
    div()
        .id(id)
        .relative()
        .flex()
        .items_center()
        .gap(px(12.))
        .flex_shrink_0()
        .bg(rgb(PANEL_DEEP))
        .border_1()
        .border_color(rgba(LINE))
        .rounded(px(11.))
        .text_color(rgb(TEXT))
        .font_family(DISPLAY_600)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(13.5))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_600, 13.5).0)
        .py(px(12.))
        .px(px(15.))
        .cursor_pointer()
        .hover(|s| s.border_color(rgb(ACCENT)).bg(rgba(ACCENT_SOFT)))
}
pub fn project_picker_name(label: impl Into<SharedString>) -> Div {
    div()
        .min_w_0()
        .overflow_hidden()
        .text_ellipsis()
        .whitespace_nowrap()
        .child(label.into())
}
pub fn project_picker_meta() -> Div {
    div()
        .ml_auto()
        .flex_none()
        .flex()
        .items_center()
        .gap(px(6.))
}
pub fn project_picker_date(label: impl Into<SharedString>, w: &Window) -> Div {
    div()
        .font_family(DISPLAY_500)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(11.5))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_500, 11.5).0)
        .text_color(rgb(FAINT))
        .min_w(px(44.))
        .text_align(TextAlign::Right)
        .child(label.into())
}
pub fn project_picker_paragraph(label: impl Into<SharedString>) -> Div {
    div()
        .text_color(rgb(DIM))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(13.5))
        .line_height(px(22.275))
        .mb(px(14.))
        .child(label.into())
}
pub fn project_picker_empty(root: &str) -> Div {
    let parts = [
        ("Nothing saved yet. Load a song and press ", false),
        ("Save project", true),
        (" — it lands in ", false),
        (root, true),
        (" with its stems, lyrics and settings.", false),
    ];
    let text = parts.iter().map(|(s, _)| *s).collect::<String>();
    let runs = parts
        .iter()
        .map(|(s, bold)| TextRun {
            len: s.len(),
            font: Font {
                family: if *bold { DISPLAY_700 } else { DISPLAY_400 }.into(),
                ..Default::default()
            },
            color: rgb(if *bold { TEXT } else { DIM }).into(),
            background_color: None,
            underline: None,
            strikethrough: None,
        })
        .collect::<Vec<_>>();
    div()
        .text_size(px(13.5))
        .line_height(px(22.275))
        .mb(px(14.))
        .child(StyledText::new(text).with_runs(runs))
}
pub fn project_picker_storage() -> Div {
    div()
        .mt(px(14.))
        .pt(px(12.))
        .border_t_1()
        .border_color(rgba(LINE))
        .flex_shrink_0()
}
pub fn project_picker_root(label: impl Into<SharedString>) -> Div {
    project_picker_fine(label)
        .mb(px(8.))
        .overflow_hidden()
        .text_ellipsis()
        .whitespace_nowrap()
}
pub fn project_picker_storage_actions() -> Div {
    div().flex().flex_wrap().gap(px(8.))
}
pub fn project_picker_fine(label: impl Into<SharedString>) -> Div {
    div()
        .text_color(rgb(FAINT))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(12.))
        .line_height(px(19.8))
        .mb(px(14.))
        .child(label.into())
}

/// Source `rise 250ms cubic-bezier(.2,.7,.2,1)`: opacity and 16px translation.
pub fn project_picker_entrance(card: Stateful<Div>, progress: f32) -> Stateful<Div> {
    let x = progress.clamp(0., 1.);
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
    let y = if x == 0. || x == 1. {
        x
    } else {
        curve((lo + hi) / 2., 0.7, 1.)
    };
    card.relative().top(px(16. * (1. - y))).opacity(y)
}

pub fn project_picker_empty_text(template:impl Into<SharedString>,vars:&[(&str,String)])->Div {
    let template=template.into();
    div().text_size(px(13.5)).line_height(px(22.275)).mb(px(14.))
        .child(crate::library_import_controls::rich_template_text(&template,vars))
}
