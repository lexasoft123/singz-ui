//! Shared project-card surfaces; the application owns project identity/actions.
use gpui_kit::prelude::*;
use gpui_kit::*;
use crate::tokens::ACCENT;

pub fn catalog_project_card(id: impl Into<ElementId>) -> Stateful<Div> {
    div().id(id).size_full().flex().items_center().gap(px(12.))
        .pl(px(11.)).pr(px(38.)).py(px(11.)).rounded(px(15.))
        .border_1().border_color(rgba(0xffffff0d)).bg(rgba(0xffffff0b))
        .cursor_pointer().hover(|s|s.border_color(rgb(ACCENT)))
        .focus_visible(|s|s.border_color(rgb(ACCENT)))
}

pub fn catalog_project_delete(id: impl Into<ElementId>) -> Stateful<Div> {
    crate::catalog_delete(id)
        .focus_visible(|s|s.text_color(rgb(0xff8a7a)).bg(rgba(0xff8a7a24)))
}
