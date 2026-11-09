//! Shared controls for the source DriveProjects picker dialog.
use crate::tokens::*;
use gpui_kit::{
    component::{
        input::{Input, InputState},
        progress::Progress,
    },
    prelude::*,
    *,
};

pub fn drive_projects_search(state: &Entity<InputState>, busy: bool) -> Input {
    crate::catalog_search(state).w_full().disabled(busy)
}

pub fn drive_projects_row(
    id: impl Into<ElementId>,
    name: impl Into<SharedString>,
    action: impl Into<SharedString>,
    window: &Window,
) -> Stateful<Div> {
    crate::project_picker_row(id, window)
        .child(crate::project_picker_name(name))
        .child(crate::project_picker_meta().child(action.into()))
}

/// Native determinate progress for the source's unstyled `<progress>` element.
/// The host supplies real download progress; the native platform painter is
/// not a claim of browser-native progress raster parity.
pub fn drive_projects_progress(
    id: impl Into<ElementId>,
    fraction: f32,
    label: impl Into<SharedString>,
) -> Div {
    div()
        .w_full()
        .h(px(16.))
        .flex_none()
        .flex()
        .items_center()
        .child(
            Progress::new(id)
                .w_full()
                .value(fraction.clamp(0., 1.) * 100.)
                .color(rgb(ACCENT))
                .accessibility_label(label),
        )
}
