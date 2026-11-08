//! Current DropScreen/ProjectPicker Drive controls. Credential-free presentation.
use crate::tokens::*;
use gpui_kit::{prelude::FluentBuilder, *};
use std::sync::{Arc, OnceLock};

fn drive_image() -> Arc<RenderImage> {
    static IMAGE: OnceLock<Arc<RenderImage>> = OnceLock::new();
    IMAGE
        .get_or_init(|| {
            let mut pixels = image::load_from_memory(include_bytes!("../assets/gdrive.png"))
                .expect("bundled production Google Drive image")
                .to_rgba8();
            for pixel in pixels.pixels_mut() {
                pixel.0.swap(0, 2);
            }
            Arc::new(RenderImage::new(vec![image::Frame::new(pixels)]))
        })
        .clone()
}
pub fn drive_icon() -> Img {
    img(drive_image()).size(px(13.)).flex_none()
}

pub fn drive_storage_panel() -> Div {
    div()
        .w_full()
        .flex_none()
        .rounded(px(13.))
        .border_1()
        .border_color(rgba(0xffffff0d))
        .bg(rgba(0xffffff09))
        .mb(px(14.))
}
pub fn drive_storage_subrow() -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(12.))
        .min_w_0()
        .border_t_1()
        .border_color(rgba(0xffffff0d))
        .px(px(14.))
        .py(px(8.))
}
pub fn drive_status_body() -> Div {
    div().flex_1().min_w_0().flex().flex_col().gap(px(2.))
}
pub fn drive_status_line(
    message: impl Into<SharedString>,
    icon: bool,
    warning: bool,
    dim: bool,
    wrap: bool,
    w: &Window,
) -> Div {
    let text = message.into();
    div()
        .min_w_0()
        .flex()
        .items_center()
        .gap(px(6.))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(12.))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_400, 12.).0)
        .text_color(rgb(if warning { ACCENT } else { TEXT }))
        .opacity(if dim { 0.55 } else { 0.85 })
        .when(icon, |d| d.child(drive_icon()))
        .when(wrap, |d| d.items_start())
        .child(div().min_w_0().when(!wrap, |d| d.truncate()).child(text))
}
pub fn drive_synced_status(last_sync: Option<String>, w: &Window) -> Div {
    let status = match last_sync {
        Some(label) => format!("up to date · {label}"),
        None => "up to date".into(),
    };
    drive_synced_status_labels("A copy also lives in your Google Drive", status, w)
}
pub fn drive_synced_status_labels(
    copy_label: impl Into<SharedString>,
    status_label: impl Into<SharedString>,
    w: &Window,
) -> Div {
    let copy_label: SharedString = copy_label.into();
    let status_label: SharedString = status_label.into();
    div()
        .min_w_0()
        .flex()
        .items_center()
        .gap(px(6.))
        .overflow_hidden()
        .whitespace_nowrap()
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(12.))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_400, 12.).0)
        .text_color(rgb(TEXT))
        .opacity(0.85)
        .child(drive_icon())
        .child(copy_label)
        .child(
            div()
                .size(px(7.))
                .flex_none()
                .rounded_full()
                .bg(rgb(0x58d68a))
                .shadow(vec![BoxShadow {
                    color: rgba(0x58d68a80).into(),
                    offset: point(px(0.), px(0.)),
                    blur_radius: px(6.),
                    spread_radius: px(0.),
                    inset: false,
                }]),
        )
        .child(div().text_color(rgb(DIM)).child(status_label))
}
pub fn drive_library_action(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    enabled: bool,
    w: &Window,
) -> Stateful<Div> {
    div()
        .id(id)
        .flex_none()
        .font_family(DISPLAY_700)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(12.))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_700, 12.).0)
        .text_color(rgb(ACCENT))
        .child(label.into())
        .when(enabled, |d| d.cursor_pointer().hover(|s| s.underline()))
        .when(!enabled, |d| d.opacity(0.4).cursor_default())
}
pub fn drive_picker_action(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    enabled: bool,
) -> Stateful<Div> {
    drive_picker_action_tooltip(
        id,
        label,
        "Push your projects to a SingZ folder in Google Drive — phones stream them from there, no Drive app needed",
        enabled,
    )
}
pub fn drive_picker_action_tooltip(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    tooltip: impl Into<SharedString>,
    enabled: bool,
) -> Stateful<Div> {
    let tooltip: SharedString = tooltip.into();
    // The source icon is an inline13px img with a6px right margin.
    crate::project_picker_action(id, "", enabled)
        .flex()
        .items_center()
        .child(drive_icon().mr(px(6.)))
        .child(label.into())
        .tooltip(move |w, cx| {
            gpui_kit::component::tooltip::Tooltip::new(tooltip.clone()).build(w, cx)
        })
}
#[derive(Clone, Copy)]
pub enum DriveBadge {
    Pending,
    Syncing,
    Synced,
    Blocked,
}
pub fn drive_project_badge(
    id: impl Into<ElementId>,
    state: DriveBadge,
    reason: Option<String>,
    w: &Window,
) -> Stateful<Div> {
    let title: String = match state {
        DriveBadge::Pending => "Waiting to reach Google Drive".into(),
        DriveBadge::Syncing => "Uploading to Google Drive".into(),
        DriveBadge::Synced => "On Google Drive — up to date".into(),
        DriveBadge::Blocked => format!(
            "Not on Google Drive yet — {}",
            reason.unwrap_or_else(|| "sync failed".into())
        ),
    };
    drive_project_badge_title(id, state, title, w)
}
pub fn drive_project_badge_title(
    id: impl Into<ElementId>,
    state: DriveBadge,
    title: impl Into<SharedString>,
    w: &Window,
) -> Stateful<Div> {
    let title: SharedString = title.into();
    let (label, dim) = match state {
        DriveBadge::Pending => ("…", true),
        DriveBadge::Syncing => ("↑", false),
        DriveBadge::Synced => ("✓", false),
        DriveBadge::Blocked => ("!", true),
    };
    div()
        .id(id)
        .ml_auto()
        .flex_none()
        .flex()
        .items_center()
        .gap(px(4.))
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(12.5))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_400, 12.5).0)
        .text_color(rgb(TEXT))
        .opacity(if dim { 0.45 } else { 0.75 })
        .child(drive_icon())
        .child(label)
        .tooltip(move |w, cx| {
            gpui_kit::component::tooltip::Tooltip::new(title.clone()).build(w, cx)
        })
}
