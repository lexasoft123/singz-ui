//! Source SetupModal typography, command selection and shared modal controls.
use crate::tokens::*;
use gpui_kit::{
    component::text::{TextView, TextViewState, TextViewStyle},
    prelude::FluentBuilder,
    *,
};

pub fn engine_setup_card(viewport: Size<Pixels>) -> Div {
    crate::modal_card().w(px(480_f32.min((f32::from(viewport.width) - 60.).max(0.))))
}
pub fn engine_setup_title(w: &Window) -> Div {
    engine_setup_title_label("Stem splitting needs Demucs", w)
}
pub fn engine_setup_title_label(label: impl Into<SharedString>, w: &Window) -> Div {
    crate::project_picker_title(label, w).mb(px(12.))
}
pub fn engine_setup_intro() -> Div {
    engine_setup_intro_parts("SingZ uses", "— a free, open-source AI model that runs entirely on your machine — to split songs into stems. One-time setup, in Terminal:")
}
pub fn engine_setup_intro_parts(before: &str, after: &str) -> Div {
    let before = format!("{before} ");
    let after = format!(" {after}");
    let parts=[(before.as_str(),false),("Demucs",true),(after.as_str(),false)];
    let text = parts.iter().map(|(s, _)| *s).collect::<String>();
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
        .text_size(px(13.5))
        .line_height(px(22.275))
        .mb(px(14.))
        .child(StyledText::new(text).with_runs(runs))
}
pub fn engine_setup_command_row() -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(10.))
        .bg(rgb(PANEL_DEEP))
        .border_1()
        .border_color(rgba(LINE))
        .rounded(px(10.))
        .py(px(11.))
        .px(px(13.))
        .mb(px(14.))
}
fn advance(w: &Window, text: &str, family: &str, size: f32) -> f32 {
    f32::from(
        w.text_system()
            .shape_line(
                text.to_owned().into(),
                px(size),
                &[TextRun {
                    len: text.len(),
                    font: Font {
                        family: family.to_owned().into(),
                        ..Default::default()
                    },
                    color: rgb(TEXT).into(),
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                }],
                None,
            )
            .width,
    )
}
pub fn engine_setup_command(
    command: &str,
    state: &Entity<TextViewState>,
    copy_label: &str,
    scroll: &ScrollHandle,
    w: &Window,
) -> Stateful<Div> {
    // .cmd's code is the shrinking flex item. Native scrollbar space matches
    // CSS overflow-x:auto only when the original command exceeds that width.
    let card_width = 480_f32.min((f32::from(w.viewport_size().width) - 60.).max(0.));
    let button_width =
        advance(w, copy_label, DISPLAY_600, 12.) + copy_label.chars().count() as f32 * 0.12 + 26.;
    let command_width = advance(w, command, MONO_400, 11.5).ceil();
    let overflow = command_width > card_width - 58. - 28. - 10. - button_width;
    let selection = state.clone();
    let focus = state.clone();
    div()
        .id("engine-setup-command")
        .relative()
        .flex_1()
        .min_w_0()
        .font_family(MONO_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(11.5))
        .line_height(crate::css_normal_line_metrics(w, MONO_400, 11.5).0)
        .text_color(rgb(0xffd9a0))
        .track_scroll(scroll)
        .overflow_x_scroll()
        .whitespace_nowrap()
        .when(overflow, |d| d.pb(px(10.)))
        .child(
            TextView::new(state)
                .style(TextViewStyle::default().paragraph_gap(rems(0.)))
                // TextView otherwise adopts its constrained parent width and
                // clips internally: the outer ScrollHandle sees no overflow.
                // Keep the command's shaped width, as CSS nowrap code does.
                .w(px(command_width))
                .flex_none()
                .font_family(MONO_400)
                .font_weight(FontWeight::NORMAL)
                .text_size(px(11.5))
                .line_height(crate::css_normal_line_metrics(w, MONO_400, 11.5).0)
                .text_color(rgb(0xffd9a0))
                .whitespace_nowrap(),
        )
        .when(overflow, |d| {
            d.child(crate::production_scrollbar_horizontal(scroll))
        })
        .on_mouse_down(MouseButton::Left, move |_, w, cx| {
            focus.read(cx).focus_handle().clone().focus(w, cx);
            cx.stop_propagation();
        })
        .on_click(move |_, _, cx| selection.update(cx, |s, cx| s.select_all(cx)))
}
pub fn engine_setup_copy(id: impl Into<ElementId>, copied: bool) -> Stateful<Div> {
    engine_setup_copy_label(id, if copied { "Copied ✓" } else { "Copy" })
}
pub fn engine_setup_copy_label(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    crate::lyrics_editor_button(id, label, false, true)
}
pub fn engine_setup_note() -> Div {
    engine_setup_note_text("Needs Python 3.10–3.13 and pipx (`brew install pipx`). The first split downloads the model (~80 MB); a typical song takes a few minutes of CPU time.")
}
pub fn engine_setup_note_text(text: impl Into<SharedString>) -> Div {
    let style = TextViewStyle::default()
        .paragraph_gap(rems(0.))
        .inline_code(HighlightStyle {
            color: Some(rgb(DIM).into()),
            background_color: Some(rgba(0).into()),
            ..Default::default()
        });
    let code = StyleRefinement::default()
        .font_family(MONO_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(11.))
        .text_color(rgb(DIM))
        .bg(rgba(0))
        .p(px(0.))
        .border_0()
        .rounded(px(0.));
    div().mb(px(14.)).font_family(DISPLAY_400).font_weight(FontWeight::NORMAL).text_size(px(12.)).line_height(px(19.8)).text_color(rgb(FAINT))
        .child(TextView::markdown("engine-setup-note",text)
            .style(style).inline_code_style(code).selectable(false).font_family(DISPLAY_400).font_weight(FontWeight::NORMAL)
            .text_size(px(12.)).line_height(px(19.8)).text_color(rgb(FAINT)))
}
pub fn engine_setup_error(message: impl Into<SharedString>) -> Div {
    div()
        .mb(px(14.))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(12.))
        .line_height(px(19.8))
        .text_color(rgb(DANGER))
        .child(message.into())
}
pub fn engine_setup_actions() -> Div {
    div().flex().gap(px(10.)).mt(px(20.)).flex_wrap()
}
pub fn engine_setup_action(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    primary: bool,
    enabled: bool,
) -> Stateful<Div> {
    crate::lyrics_editor_button(id, label, primary, enabled)
        .px(px(16.))
        .py(px(9.))
        .text_size(px(13.))
        .line_height(px(15.5))
}
pub fn engine_setup_entrance(card: Stateful<Div>, progress: f32) -> Stateful<Div> {
    crate::project_picker_entrance(card, progress)
}
