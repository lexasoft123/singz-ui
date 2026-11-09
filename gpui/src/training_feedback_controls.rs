//! Shared native result acknowledgement from Banner and VocalTraining styles.
use gpui_kit::{prelude::*, *};
use crate::tokens::*;

/// Source `.sz-banner.vt-result-ack`, without focusable actions.
/// Copy is resolved by the host so all rendered labels follow its locale.
pub fn training_result_acknowledgement(
    mark: impl Into<SharedString>,
    heading: impl Into<SharedString>,
    detail: impl Into<SharedString>,
    footer: Option<SharedString>,
    compact: bool,
) -> Div {
    let heading_size = if compact { 16. } else { 18. };
    crate::training_banner()
        .flex_none()
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .mb(px(if compact { 4. } else { 8. }))
        .when(compact, |banner| banner.min_h(px(46.)).px(px(12.)).py(px(6.)))
        .child(
            div()
                .min_h(px(34.))
                .flex()
                .flex_wrap()
                .items_center()
                .justify_center()
                .gap_x(px(12.))
                .gap_y(px(4.))
                .child(div().flex_none().text_color(rgb(ACCENT)).text_size(px(22.)).line_height(px(22.)).child(mark.into()))
                .child(div().font_family(DISPLAY_700).text_color(rgb(TEXT)).text_size(px(heading_size)).line_height(px(heading_size * 1.35)).child(heading.into()))
                .child(div().min_w_0().text_color(rgb(DIM)).text_size(px(14.)).line_height(px(21.)).child(detail.into())),
        )
        .children(footer.map(|text| div().mt(px(4.)).text_color(rgb(DIM)).text_size(px(14.)).line_height(px(21.)).child(text)))
}
