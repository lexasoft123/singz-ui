//! Source WhatsNew presentation, original SVG paths, and bundled Telegram art.
use crate::tokens::*;
use gpui_kit::*;
use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, OnceLock},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ReleaseIcon {
    News,
    Export,
    Mic,
    Controls,
}
fn telegram_image() -> Arc<RenderImage> {
    static IMAGE: OnceLock<Arc<RenderImage>> = OnceLock::new();
    IMAGE
        .get_or_init(|| {
            let mut pixels = image::load_from_memory(include_bytes!("../assets/telegram-logo.png"))
                .expect("bundled production Telegram image")
                .to_rgba8();
            for p in pixels.pixels_mut() {
                p.0.swap(0, 2);
            }
            Arc::new(RenderImage::new(vec![image::Frame::new(pixels)]))
        })
        .clone()
}
thread_local! {static ICONS:RefCell<HashMap<(ReleaseIcon,u32),Arc<RenderImage>>>=RefCell::new(HashMap::new());}
fn highlight_image(icon: ReleaseIcon, w: &Window) -> Arc<RenderImage> {
    let scale = w.scale_factor();
    ICONS.with(|cache|cache.borrow_mut().entry((icon,scale.to_bits())).or_insert_with(||{
        let path=match icon {
            ReleaseIcon::News=>"M4 4h16v16H4z M8 8h8 M8 12h8 M8 16h4",
            ReleaseIcon::Export=>"M12 3v12 M7 10l5 5 5-5 M4 16v5h16v-5",
            ReleaseIcon::Mic=>"M9 4a3 3 0 0 1 6 0v7a3 3 0 0 1-6 0z M5 10v1a7 7 0 0 0 14 0v-1 M12 18v4 M8 22h8",
            ReleaseIcon::Controls=>"M4 7h16 M4 17h16 M8 4v6 M16 14v6",
        };
        let source=format!(r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="#{ACCENT:06x}" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"><path d="{path}"/></svg>"##);
        let tree=resvg::usvg::Tree::from_str(&source,&resvg::usvg::Options::default()).expect("production highlight SVG");
        let side=(24.*scale).round().max(1.) as u32;
        let mut pixels=resvg::tiny_skia::Pixmap::new(side,side).expect("highlight icon raster");
        resvg::render(&tree,resvg::tiny_skia::Transform::from_scale(scale,scale),&mut pixels.as_mut());
        let mut bytes=pixels.take();
        for p in bytes.chunks_exact_mut(4){
            if p[3]>0{for c in 0..3{p[c]=((p[c] as u32*255+p[3] as u32/2)/p[3] as u32).min(255) as u8;}}
            p.swap(0,2);
        }
        Arc::new(RenderImage::new(vec![image::Frame::new(image::RgbaImage::from_raw(side,side,bytes).expect("highlight pixels"))]))
    }).clone())
}
pub fn release_highlights_card(viewport: Size<Pixels>) -> Div {
    crate::modal_card()
        .w(px(580_f32.min((f32::from(viewport.width) - 32.).max(0.))))
        .max_h(px(f32::from(viewport.height) * 0.85))
        .min_h_0()
        .gap(px(18.))
        .text_size(px(14.))
}
pub fn release_highlights_heading() -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .flex_shrink_0()
        .font_family(DISPLAY_800)
        .font_weight(FontWeight::NORMAL)
}
pub fn release_highlights_brand(version: impl Into<SharedString>, w: &Window) -> Div {
    div()
        .flex()
        .items_baseline()
        .text_size(px(14.))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_800, 14.).0)
        .child("SingZ ")
        .child(
            div()
                .ml(px(10.))
                .font_family(DISPLAY_500)
                .text_color(rgb(DIM))
                .text_size(px(14. / 1.2))
                .line_height(crate::css_normal_line_metrics(w, DISPLAY_500, 14. / 1.2).0)
                .child(version.into()),
        )
}
pub fn release_highlights_title(label: impl Into<SharedString>, w: &Window) -> Div {
    div()
        .font_family(DISPLAY_800)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(32.))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_800, 32.).0)
        .flex_shrink_0()
        .child(label.into())
}
pub fn release_highlights_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    primary: bool,
) -> Stateful<Div> {
    crate::lyrics_editor_button(id, label, primary, true)
        .px(px(16.))
        .py(px(9.))
        .text_size(px(13.))
        .line_height(px(15.5))
}
pub fn release_highlights_telegram(
    id: impl Into<ElementId>,
    title: impl Into<SharedString>,
    description: impl Into<SharedString>,
    w: &Window,
) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .items_center()
        .gap(px(14.))
        .p(px(16.))
        .border_1()
        .border_color(rgba(0x279bd344))
        .rounded(px(16.))
        .bg(rgba(0x279bd30c))
        .cursor_pointer()
        .flex_shrink_0()
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(14.))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_400, 14.).0)
        .child(img(telegram_image()).w(px(42.)).h(px(42.)).flex_shrink_0())
        .child(
            div()
                .flex_1()
                .min_w_0()
                .child(div().font_family(DISPLAY_700).child(title.into()))
                .child(
                    div()
                        .mt(px(5.))
                        .text_color(rgb(DIM))
                        .text_size(px(14. / 1.2))
                        .line_height(crate::css_normal_line_metrics(w, DISPLAY_400, 14. / 1.2).0)
                        .child(description.into()),
                ),
        )
        .child("↗")
}
pub fn release_highlights_content(id: impl Into<ElementId>) -> Stateful<Div> {
    div().id(id).relative().min_h_0().overflow_y_scroll()
}
pub fn release_highlights_section() -> Div {
    div()
}
pub fn release_highlights_version(label: impl Into<SharedString>, w: &Window) -> Div {
    div()
        .font_family(DISPLAY_700)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(14. * 1.17))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_700, 14. * 1.17).0)
        .child(label.into())
}
pub fn release_highlights_article(id: impl Into<ElementId>, separated: bool) -> Stateful<Div> {
    let row = div().id(id).flex().gap(px(16.)).py(px(16.));
    if separated {
        row.border_t_1().border_color(rgba(0x88888822))
    } else {
        row
    }
}
pub fn release_highlights_icon(icon: ReleaseIcon, w: &Window) -> Div {
    // Inline SVGs retain the surrounding line's descent below their baseline.
    let (height, baseline) = crate::css_normal_line_metrics(w, DISPLAY_400, 14.);
    div()
        .p(px(10.))
        .rounded(px(12.))
        .bg(rgba(0xffa02812))
        .flex_none()
        .self_start()
        .child(
            div()
                .w(px(24.))
                .h(px(24.) + height - baseline)
                .child(img(highlight_image(icon, w)).size(px(24.))),
        )
}
pub fn release_highlights_article_copy() -> Div {
    div().min_w_0().flex_1()
}
pub fn release_highlights_desktop(label: impl Into<SharedString>, w: &Window) -> Div {
    div()
        .font_family(DISPLAY_400)
        .text_size(px(14. / 1.2))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_400, 14. / 1.2).0)
        .child(label.into())
}
pub fn release_highlights_article_title(label: impl Into<SharedString>, w: &Window) -> Div {
    div()
        .font_family(DISPLAY_700)
        .font_weight(FontWeight::NORMAL)
        .text_size(px(18.))
        .line_height(crate::css_normal_line_metrics(w, DISPLAY_700, 18.).0)
        .mb(px(6.))
        .child(label.into())
}
pub fn release_highlights_description(label: impl Into<SharedString>) -> Div {
    div()
        .font_family(DISPLAY_400)
        .font_weight(FontWeight::NORMAL)
        .text_color(rgb(DIM))
        .text_size(px(13.5))
        .line_height(px(20.25))
        .child(label.into())
}
pub fn release_highlights_footer() -> Div {
    div()
        .flex()
        .flex_wrap()
        .gap(px(10.))
        .mt(px(20.))
        .flex_shrink_0()
}
pub fn release_highlights_entrance(card: Stateful<Div>, progress: f32) -> Stateful<Div> {
    crate::project_picker_entrance(card, progress)
}
