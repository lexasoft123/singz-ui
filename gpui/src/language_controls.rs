//! Source LanguageSwitcher rows and Flags.tsx art. The host supplies all copy.
use crate::tokens::*;
use gpui_kit::{prelude::FluentBuilder, *};

/// Keep the existing English header primitive byte-for-byte in native paint;
/// the same20x14slot carries the other source flags and language-code marks.
pub fn language_header_flag(tag: impl Into<SharedString>) -> Div {
    let tag=tag.into();
    div().w(px(20.)).h(px(14.)).rounded(px(3.)).overflow_hidden().flex_none()
        .child(canvas(|b,_,_|b,move |_,b,w,cx| {
            let x=b.left();let y=b.top();let scale=b.size.width/px(30.);
            if tag.as_ref()=="en" {
                w.paint_quad(fill(b,rgb(0x012169)));
                let mut stroke=|segments:&[([f32;2],[f32;2])],thickness:f32,color:u32| {
                    let mut path=PathBuilder::stroke(px(thickness*scale));
                    for (a,z) in segments {
                        path.move_to(point(x+px(a[0]*scale),y+px(a[1]*scale)));
                        path.line_to(point(x+px(z[0]*scale),y+px(z[1]*scale)));
                    }
                    if let Ok(path)=path.build(){w.paint_path(path,rgb(color));}
                };
                let diagonals=[([0.,0.],[30.,21.]),([30.,0.],[0.,21.])];
                stroke(&diagonals,4.,0xffffff);stroke(&diagonals,1.6,0xc8102e);
                let cross=[([15.,0.],[15.,21.]),([0.,10.5],[30.,10.5])];
                stroke(&cross,6.,0xffffff);stroke(&cross,3.4,0xc8102e);
            } else if tag.as_ref()=="ru" {
                for (top,color) in [(0.,0xffffff),(7.,0x0039a6),(14.,0xd52b1e)] {
                    w.paint_quad(fill(Bounds::new(point(x,y+px(top*scale)),size(b.size.width,px(7.*scale))),rgb(color)));
                }
            } else if tag.as_ref()=="zh-CN" {
                w.paint_quad(fill(b,rgb(0xee1c25)));
                let points=[[5.,3.5],[5.94,6.21],[8.8,6.26],[6.52,7.99],[7.35,10.74],[5.,9.1],[2.65,10.74],[3.48,7.99],[1.2,6.26],[4.06,6.21]];
                let mut path=PathBuilder::fill();
                path.move_to(point(x+px(points[0][0]*scale),y+px(points[0][1]*scale)));
                for p in points.iter().skip(1){path.line_to(point(x+px(p[0]*scale),y+px(p[1]*scale)));}
                path.close();if let Ok(path)=path.build(){w.paint_path(path,rgb(0xffde00));}
                for (cx,cy) in [(11.,3.2),(13.2,5.4),(13.2,8.4),(11.,10.6)] {
                    w.paint_quad(fill(Bounds::new(point(x+px((cx-0.95)*scale),y+px((cy-0.95)*scale)),size(px(1.9*scale),px(1.9*scale))),rgb(0xffde00)).corner_radii(px(0.95*scale)));
                }
            } else {
                w.paint_quad(fill(b,rgb(0x26221c)).corner_radii(px(2.*scale)));
                let code:SharedString=match tag.as_ref(){"tr"=>"TR","hi"=>"HI","ja"=>"JA","ko"=>"KO","th"=>"TH","id"=>"ID","vi"=>"VI","fil"=>"FIL","ms"=>"MS","zh-TW"=>"繁",_=>""}.into();
                let font=Font::default().bold();let font_size=px(10.*scale);
                let line=w.text_system().shape_line(code.clone(),font_size,&[TextRun {
                    len:code.len(),font,color:rgb(ACCENT).into(),background_color:None,underline:None,strikethrough:None,
                }],None);
                let baseline=w.text_baseline(line.ascent,line.descent,font_size);
                let _=line.paint(point(x+(b.size.width-line.width)/2.,y+px(14.*scale)-baseline),font_size,TextAlign::Left,None,w,cx);
            }
        }).size_full())
}
pub fn language_row_flag(tag: impl Into<SharedString>) -> Div {
    language_header_flag(tag).shadow(vec![BoxShadow {
        color:rgba(LINE).into(),offset:point(px(0.),px(0.)),blur_radius:px(0.),spread_radius:px(1.),inset:false,
    }])
}
fn language_glyph(kind:u8,color:u32,w:&Window)->Div {
    use std::{cell::RefCell,collections::HashMap,sync::Arc};
    thread_local! {static CACHE:RefCell<HashMap<(u8,u32,u32),Arc<RenderImage>>>=RefCell::new(HashMap::new());}
    let scale=w.scale_factor();
    let image=CACHE.with(|cache|cache.borrow_mut().entry((kind,color,scale.to_bits())).or_insert_with(||{
        let body=if kind==0 {r#"<rect x="3" y="4" width="18" height="12" rx="2"/><path d="M8 20h8M12 16v4"/>"#}
            else {r#"<path d="M20 6 9 17l-5-5"/>"#};
        let width=if kind==0 {1.8}else{3.};
        let svg=format!(r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#{color:06x}" stroke-width="{width}" stroke-linecap="round" stroke-linejoin="round">{body}</svg>"##);
        let tree=resvg::usvg::Tree::from_str(&svg,&resvg::usvg::Options::default()).expect("source language SVG");
        let side=(14.*scale).round().max(1.) as u32;
        let mut image=resvg::tiny_skia::Pixmap::new(side,side).expect("language glyph raster");
        resvg::render(&tree,resvg::tiny_skia::Transform::from_scale(14.*scale/24.,14.*scale/24.),&mut image.as_mut());
        let mut pixels=image.take();
        for pixel in pixels.chunks_exact_mut(4) {
            if pixel[3]>0 {for channel in 0..3 {pixel[channel]=((pixel[channel] as u32*255+pixel[3] as u32/2)/pixel[3] as u32).min(255) as u8;}}
            pixel.swap(0,2);
        }
        Arc::new(RenderImage::new(vec![image::Frame::new(image::RgbaImage::from_raw(side,side,pixels).expect("language SVG pixels"))]))
    }).clone());
    div().size(px(14.)).child(img(image).size_full())
}
pub fn language_menu_width(rows:&[(String,Option<String>)],w:&Window)->f32 {
    let mut text_width=0_f32;
    for (label,hint) in rows {
        for (text,family,size) in std::iter::once((label.as_str(),DISPLAY_600,13.))
            .chain(hint.as_deref().map(|s|(s,DISPLAY_400,11.))) {
            let text:SharedString=text.to_owned().into();
            let line=w.text_system().shape_line(text.clone(),px(size),&[TextRun {
                len:text.len(),font:Font{family:family.into(),..Default::default()},color:rgb(TEXT).into(),background_color:None,underline:None,strikethrough:None,
            }],None);
            text_width=text_width.max(f32::from(line.width));
        }
    }
    (text_width+96.).max(236.)
}
pub fn language_menu_natural_height(hints: impl IntoIterator<Item=bool>) -> f32 {
    let rows:Vec<_>=hints.into_iter().collect();
    14.+rows.iter().map(|hint|if *hint {45.55}else{38.}).sum::<f32>()+rows.len().saturating_sub(1) as f32*2.
}
pub fn language_menu_surface(width:f32,height:f32)->Stateful<Div> {
    div().id("language-menu").w(px(width)).h(px(height)).min_h_0().p(px(6.))
        .bg(rgb(SURFACE_RAISED)).border_1().border_color(rgba(LINE_STRONG)).rounded(px(14.))
        .font_family(DISPLAY_400).font_weight(FontWeight::NORMAL).text_color(rgb(TEXT))
        .shadow(vec![BoxShadow{color:rgba(0x00000080).into(),offset:point(px(0.),px(12.)),blur_radius:px(40.),spread_radius:px(0.),inset:false}])
}
pub fn language_menu_rows(scroll:&ScrollHandle)->Stateful<Div> {
    div().id("language-rows").w_full().h_full().min_h_0().overflow_y_scroll().track_scroll(scroll)
        .flex().flex_col().gap(px(2.))
}
pub fn language_menu_row(id:impl Into<ElementId>,tag:Option<&str>,label:String,hint:Option<String>,selected:bool,keyboard_focus:bool,w:&Window)->Stateful<Div> {
    let cell=div().w(px(32.)).h(px(24.)).flex_none().rounded(px(7.)).border_1()
        .border_color(rgba(if selected{ACCENT_LINE}else{LINE}))
        .when(selected,|d|d.bg(rgba(ACCENT_SOFT)))
        .flex().items_center().justify_center()
        .child(match tag {Some(tag)=>language_row_flag(tag.to_owned()),None=>language_glyph(0,if selected {ACCENT}else{DIM},w)});
    let mut text=div().flex_1().min_w_0().flex().flex_col().gap(px(1.))
        .child(div().font_family(DISPLAY_600).text_size(px(13.)).line_height(px(16.25)).child(label));
    if let Some(hint)=hint {text=text.child(div().font_family(DISPLAY_400).text_size(px(11.)).line_height(px(14.3)).text_color(rgb(DIM)).truncate().child(hint));}
    div().id(id).flex_none().w_full().flex().items_center().gap(px(10.)).pl(px(7.)).pr(px(9.)).py(px(7.)).rounded(px(9.))
        .relative().cursor_pointer().when(keyboard_focus,|d|d.bg(rgba(ACCENT_SOFT)).child(
            div().absolute().inset_0().rounded(px(9.)).border_2().border_color(rgba(FOCUS_RING))))
        .hover(|s|s.bg(rgba(ACCENT_SOFT))).child(cell).child(text)
        .child(language_glyph(1,ACCENT,w).flex_none().opacity(if selected {1.}else{0.}))
}
pub fn language_menu_entrance(menu:Stateful<Div>,progress:f32)->Stateful<Div> {
    crate::project_picker_entrance(menu,progress)
}
