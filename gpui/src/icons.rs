//! Exact production SVG transport paths, rasterized at the current device scale.
use gpui_kit::*;
use std::{cell::RefCell, collections::HashMap, sync::Arc};
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum TransportIcon {
    Restart,
    Play,
    Pause,
    Loop,
    Metronome,
    CarryLine,
    Mic,
    Speaker,
    Chevron,
}
pub struct NativeIcon {
    kind: TransportIcon,
    color: u32,
    edge: Option<u32>,
}
pub fn icon(kind: TransportIcon) -> NativeIcon {
    colored_icon(kind, crate::tokens::DIM)
}
pub fn colored_icon(kind: TransportIcon, color: u32) -> NativeIcon {
    NativeIcon {
        kind,
        color,
        edge: None,
    }
}
impl NativeIcon {
    /// Rerasterize the original SVG at a caller's measured logical edge size.
    pub fn with_size(mut self, edge: u32) -> Self {
        self.edge = Some(edge.max(1));
        self
    }
}
impl IntoElement for NativeIcon {
    type Element = ViewElement<Self>;
    fn into_element(self) -> Self::Element {
        ViewElement::new(self)
    }
}
thread_local! {static CACHE:RefCell<HashMap<(TransportIcon,u32,u32,u32),Arc<RenderImage>>>=RefCell::new(HashMap::new());}
impl RenderOnce for NativeIcon {
    fn render(self, w: &mut Window, _: &mut App) -> impl IntoElement {
        let (default_edge, view, body) = match self.kind {
            TransportIcon::Restart => (
                13,
                13,
                r#"<rect x="1" y="1.5" width="2" height="10" rx=".75"/><path d="M12 2.4v8.2a.9.9 0 0 1-1.4.75L4.5 7.25a.9.9 0 0 1 0-1.5l6.1-4.1A.9.9 0 0 1 12 2.4Z"/>"#,
            ),
            TransportIcon::Play => (
                16,
                16,
                r#"<path d="M4 2.8v10.4a1 1 0 0 0 1.53.85l8.24-5.2a1 1 0 0 0 0-1.7L5.53 1.95A1 1 0 0 0 4 2.8Z"/>"#,
            ),
            TransportIcon::Pause => (
                16,
                16,
                r#"<rect x="2.5" y="2" width="4" height="12" rx="1.2"/><rect x="9.5" y="2" width="4" height="12" rx="1.2"/>"#,
            ),
            TransportIcon::Loop => (
                14,
                16,
                r#"<path fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" d="M3.5 6.5v-1a2 2 0 0 1 2-2h7l-1.8-1.8M12.5 9.5v1a2 2 0 0 1-2 2h-7l1.8 1.8"/>"#,
            ),
            TransportIcon::Metronome => (
                14,
                16,
                r#"<g fill="none" stroke="currentColor" stroke-width="1.3"><path d="M6.1 2h3.8l2.3 11.2H3.8Z" stroke-linejoin="round"/><path d="M8 10.2 11.3 4.2" stroke-linecap="round"/></g>"#,
            ),
            TransportIcon::CarryLine => (
                14,
                16,
                r#"<rect x="2" y="4" width="5" height="8" rx="1.5"/><rect x="9.7" y="4.65" width="3.7" height="6.7" rx="1.2" fill="none" stroke="currentColor" stroke-width="1.3"/>"#,
            ),
            TransportIcon::Mic => (
                13,
                14,
                r#"<path d="M7 1a2.6 2.6 0 0 0-2.6 2.6v3a2.6 2.6 0 1 0 5.2 0v-3A2.6 2.6 0 0 0 7 1Z"/><path d="M2.7 6.4a.65.65 0 0 1 1.3.13v.07a3 3 0 0 0 6 0v-.07a.65.65 0 0 1 1.3-.13v.2a4.3 4.3 0 0 1-3.65 4.25v1.3h1.7a.65.65 0 1 1 0 1.3H4.65a.65.65 0 1 1 0-1.3h1.7v-1.3A4.3 4.3 0 0 1 2.7 6.6v-.2Z"/>"#,
            ),
            TransportIcon::Speaker => (
                16,
                20,
                r#"<g fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 7h3l4-3v12l-4-3H3z"/><path d="M13 7a4 4 0 0 1 0 6"/><path d="M16 4a8 8 0 0 1 0 12"/></g>"#,
            ),
            TransportIcon::Chevron => (8, 8, r#"<path d="M1 2h6L4 6Z"/>"#),
        };
        let edge = self.edge.unwrap_or(default_edge);
        let scale = w.scale_factor();
        let image=CACHE.with(|cache|cache.borrow_mut().entry((self.kind,self.color,scale.to_bits(),edge)).or_insert_with(||{
            let source=format!(r##"<svg xmlns="http://www.w3.org/2000/svg" width="{edge}" height="{edge}" viewBox="0 0 {view} {view}" fill="#{color:06x}" color="#{color:06x}">{body}</svg>"##,color=self.color);
            let tree=resvg::usvg::Tree::from_str(&source,&resvg::usvg::Options::default()).expect("production transport SVG");
            let side=(edge as f32*scale).round().max(1.) as u32;
            let mut pixmap=resvg::tiny_skia::Pixmap::new(side,side).expect("icon raster");
            resvg::render(&tree,resvg::tiny_skia::Transform::from_scale(scale,scale),&mut pixmap.as_mut());
            let mut bytes=pixmap.take();
            for pixel in bytes.chunks_exact_mut(4){if pixel[3]>0 {for c in 0..3{pixel[c]=((pixel[c] as u32*255+pixel[3] as u32/2)/pixel[3] as u32).min(255) as u8;}}}
            for pixel in bytes.chunks_exact_mut(4) { pixel.swap(0, 2); }
                    Arc::new(RenderImage::new(vec![image::Frame::new(image::RgbaImage::from_raw(side,side,bytes).expect("icon pixels"))]))
        }).clone());
        img(image).w(px(edge as f32)).h(px(edge as f32))
    }
}
