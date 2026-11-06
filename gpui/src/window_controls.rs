//! Frameless window controls, matching UIKit's 46×52px desktop buttons.
use gpui_kit::*;
use crate::tokens::{DIM, TEXT};

pub fn window_buttons(maximized: bool) -> Div {
    div().absolute().top_0().right_0().h(px(52.)).flex()
        .child(button("window-minimize", 0, maximized).on_click(|_, w, cx| {cx.stop_propagation(); w.minimize_window();}))
        .child(button("window-maximize", 1, maximized).on_click(|_, w, cx| {cx.stop_propagation(); w.zoom_window(); w.refresh();}))
        .child(button("window-close", 2, maximized).on_click(|_, w, cx| {cx.stop_propagation(); w.remove_window();}))
}
fn button(id: &'static str, kind: u8, maximized: bool) -> Stateful<Div> {
    div().id(id).w(px(46.)).h_full().flex_none().flex().items_center().justify_center()
        .text_color(rgb(DIM)).cursor_pointer()
        .window_control_area(match kind {0=>WindowControlArea::Min,1=>WindowControlArea::Max,_=>WindowControlArea::Close})
        .on_mouse_move(|_,_,cx|cx.stop_propagation())
        .hover(move |style| style.bg(if kind==2 {rgb(0xc42b1c)} else {rgba(0xfff0d612)}).text_color(rgb(if kind==2 {0xffffff} else {TEXT})))
        .child(canvas(|bounds,_,_|bounds, move |_,bounds,w,_| {
            let mut path=PathBuilder::stroke(px(if kind==2 {1.1} else {1.}));
            let mut line=|a:(f32,f32), b:(f32,f32)| {path.move_to(bounds.origin+point(px(a.0),px(a.1)));path.line_to(bounds.origin+point(px(b.0),px(b.1)));};
            match kind {
                0=>line((0.,5.),(10.,5.)),
                1 if maximized=> {for (a,b) in [((2.5,2.5),(2.5,0.5)),((2.5,0.5),(9.5,0.5)),((9.5,0.5),(9.5,7.5)),((9.5,7.5),(7.5,7.5)),((0.5,2.5),(7.5,2.5)),((7.5,2.5),(7.5,9.5)),((7.5,9.5),(0.5,9.5)),((0.5,9.5),(0.5,2.5))] {line(a,b);}},
                1=>{for(a,b)in [((0.5,0.5),(9.5,0.5)),((9.5,0.5),(9.5,9.5)),((9.5,9.5),(0.5,9.5)),((0.5,9.5),(0.5,0.5))]{line(a,b);}},
                _=>{line((0.7,0.7),(9.3,9.3));line((9.3,0.7),(0.7,9.3));}
            }
            if let Ok(path)=path.build(){w.paint_path(path,w.text_style().color);}
        }).size(px(10.)))
}

/// Invisible client-side resize grips for Linux window managers. Add last,
/// above content, only while restored and using client-side decorations.
pub fn window_resize_edges(size: Size<Pixels>) -> Vec<Stateful<Div>> {
    let w=f32::from(size.width);let h=f32::from(size.height);let t=4.;
    use ResizeEdge::*;
    use CursorStyle::*;
    [
        (Top,ResizeUp, t,0.,w-2.*t,t),
        (Bottom,ResizeDown,t,h-t,w-2.*t,t),
        (Left,ResizeLeft,0.,t,t,h-2.*t),
        (Right,ResizeRight,w-t,t,t,h-2.*t),
        (TopLeft,ResizeUpLeftDownRight,0.,0.,t,t),
        (TopRight,ResizeUpRightDownLeft,w-t,0.,t,t),
        (BottomLeft,ResizeUpRightDownLeft,0.,h-t,t,t),
        (BottomRight,ResizeUpLeftDownRight,w-t,h-t,t,t),
    ].into_iter().enumerate().map(|(i,(edge,cursor,x,y,width,height))|
        div().id(("window-resize-edge",i)).absolute().left(px(x)).top(px(y))
            .w(px(width.max(0.))).h(px(height.max(0.))).cursor(cursor)
            .on_mouse_down(MouseButton::Left, move |_,window,cx|{cx.stop_propagation();window.start_window_resize(edge);})
    ).collect()
}
