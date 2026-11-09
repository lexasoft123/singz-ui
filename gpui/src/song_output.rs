//! SongOutput.tsx primitives: gain fader, overload meter and reusable popup.
use crate::{tokens::*, *};
use gpui_kit::*;
use gpui_kit::prelude::FluentBuilder;

pub struct SongOutputCopy {
    pub title:String, pub saved:String, pub per_song:String, pub peak:String, pub caption:String,
}
pub fn song_output_speaker(muted:bool)->Div {
    div().flex_none().child(colored_icon(if muted {TransportIcon::SpeakerMuted}else{TransportIcon::Speaker},if muted {ACCENT}else{TEXT}))
}
pub fn song_output_action(id:impl Into<ElementId>,label:impl Into<SharedString>)->Stateful<Div> {
    ghost_pill(id,label).px(px(9.)).py(px(5.)).text_size(px(11.))
}
pub fn song_output_mute(id:impl Into<ElementId>,muted:bool)->Stateful<Div> {
    ghost_pill(id,"").size(px(30.)).p_0().justify_center()
        .when(muted,|d|d.text_color(rgb(ACCENT)).border_color(rgba(ACCENT_LINE)).bg(rgba(ACCENT_SOFT)))
        .child(song_output_speaker(muted))
}
pub fn song_output_clip(id:impl Into<ElementId>,label:impl Into<SharedString>,clipped:bool)->Stateful<Div> {
    let label:SharedString=label.into();
    let id:ElementId=id.into();
    div().id(id.clone()).role(Role::Button).accessibility_id(id.to_string()).aria_label(label.clone())
        .flex().items_center().gap(px(4.)).px(px(5.)).py(px(3.)).rounded(px(4.))
        .border_1().border_color(rgba(LINE)).text_color(rgb(if clipped {DANGER}else{FAINT}))
        .font_family(MONO_400).text_size(px(9.)).cursor_pointer()
        .when(clipped,|d|d.border_color(rgb(DANGER).alpha(0.45)).bg(rgb(DANGER).alpha(0.08)))
        .child(div().size(px(4.)).rounded(px(1.)).bg(rgb(if clipped {DANGER}else{FAINT})))
        .child(label)
}
fn peak_fraction(peak:f32)->f32 {if peak>0. {((20.*peak.log10()+60.)/60.).clamp(0.,1.)}else{0.}}
fn meter(peak:f32,mini:bool)->Div {
    let fraction=peak_fraction(peak);
    div().relative().overflow_hidden().bg(rgb(PANEL_DEEP)).rounded(px(if mini {2.}else{3.}))
        .h(px(if mini {12.}else{14.})).when(!mini,|d|d.border_1().border_color(rgba(LINE)))
        .when(mini,|d|d.w(px(28.)))
        .child(canvas(|b,_,_|b,move|_,b,w,_|{
            let width=f32::from(b.size.width)*fraction;
            if mini {w.paint_quad(fill(Bounds::new(b.origin,size(px(width),b.size.height)),rgb(SUCCESS)));}
            else {
                let full=f32::from(b.size.width);
                for x in (0..width.ceil() as usize).step_by(7) {
                    let x=x as f32;let color=if x/full<0.8 {SUCCESS}else if x/full<0.95 {ACCENT}else{DANGER};
                    w.paint_quad(fill(Bounds::new(point(b.left()+px(x),b.top()),size(px((width-x).min(5.).max(0.)),b.size.height)),rgb(color)));
                }
            }
        }).absolute().inset_0())
}
pub fn song_output_trigger_meter(id:impl Into<ElementId>,label:impl Into<SharedString>,compact:bool,muted:bool,open:bool,peak:f32,clipped:bool)->Stateful<Div> {
    ghost_pill(id,"").gap(px(8.)).h(px(30.)).py_0().px(px(10.)).rounded(px(8.))
        .when(open,|d|d.bg(rgba(ACCENT_SOFT)).border_color(rgba(ACCENT_LINE)))
        .when(muted,|d|d.text_color(rgb(DIM)))
        .child(song_output_speaker(muted))
        .child(div().min_w(px(56.)).text_center().text_size(px(10.)).font_family(MONO_400).child(tracked_label(label,0.012).normal_line_height()))
        .when(!compact,|d|d.child(meter(peak,true)))
        .child(div().size(px(4.)).rounded_full().bg(rgba(if clipped {(DANGER<<8)|255}else{LINE_STRONG})))
}
fn ticks(labels:&[(&str,f32)])->Div {
    let gain_ticks=labels.get(2).is_some_and(|(label,_)|*label=="−12");
    div().relative().h(px(12.)).mt(px(9.)).font_family(MONO_400).text_size(px(9.)).text_color(rgb(FAINT))
        .children(labels.iter().map(|(label,fraction)| {
            let d=div().absolute().child((*label).to_owned())
                .when(gain_ticks&&*label=="0",|d|d.text_color(rgb(ACCENT)));
            if *fraction==0. {d.left_0()}
            else if *fraction==1. {d.right_0()}
            else {d.left(relative(*fraction)).w(px(24.)).ml(px(-12.)).text_center()}
        }))
}
#[derive(Clone)]
struct SongOutputSliderDrag(ElementId);
fn song_output_db_at(x:f32,width:f32)->f32 {
    -60.+72.*((x-7.5)/(width-15.).max(1.)).clamp(0.,1.)
}
/// SongOutput's CSS range surface is deliberately independent of training's
/// native-looking slider: 6px accent-mixed track and a 15px panel-ring thumb.
pub fn song_output_fader(id:impl Into<ElementId>,db:f32,on_change:impl Fn(f32,&mut Window,&mut App)+'static)->Stateful<Div> {
    use std::{cell::{Cell,RefCell},rc::Rc};
    let bounds=Rc::new(Cell::new(Bounds::<Pixels>::default()));
    let live=Rc::new(Cell::new(db));
    let callback_live=live.clone();
    let callback:Rc<dyn Fn(f32,&mut Window,&mut App)>=Rc::new(move|next,w,cx|{
        // The host accepts half-dB values. Keep the same original pointer
        // callback value, while keyboard and AX share its accepted live step.
        callback_live.set((next.clamp(-60.,12.)*2.).round()/2.);
        on_change(next,w,cx);
    });
    let down_bounds=bounds.clone();let move_bounds=bounds.clone();let down_callback=callback.clone();
    let key_live=live.clone();let key_callback=callback.clone();let semantic_callback=callback.clone();
    let hitbox=Rc::new(RefCell::new(None::<Hitbox>));let paint_hitbox=hitbox.clone();
    let id:ElementId=id.into();let drag_id=id.clone();
    let fraction=((db+60.)/72.).clamp(0.,1.);
    let control=div().id(id).relative().w_full().min_w(px(40.)).h(px(6.)).cursor_pointer()
        .on_key_down(move|event:&KeyDownEvent,w,cx|{
            let current=key_live.get();
            let next=match event.keystroke.key.as_str(){"left"|"down"=>Some(current-0.5),"right"|"up"=>Some(current+0.5),"home"=>Some(-60.),"end"=>Some(12.),_=>None};
            if let Some(next)=next {key_callback(next.clamp(-60.,12.),w,cx);cx.stop_propagation();}
            else if matches!(event.keystroke.key.as_str(),"space"|"enter") {cx.stop_propagation();}
        })
        .child(canvas(move|b,_,_|{bounds.set(b);b},|_,b,w,_|{
            // color-mix(in srgb, var(--accent) 30%, var(--panel)).
            let color=Rgba{r:(255.*0.3+27.*0.7)/255.,g:(160.*0.3+24.*0.7)/255.,b:(40.*0.3+20.*0.7)/255.,a:1.};
            w.paint_quad(quad(b,px(2.),color,px(0.),color,BorderStyle::Solid));
        }).absolute().inset_0())
        // Border-box 15px thumb + 1px outside outline; its 3px panel border
        // leaves the source's 9px accent core. The absolute thumb does not
        // inflate the six-pixel range's layout height.
        .child(div().absolute().top(px(-5.5)).left(relative(fraction)).ml(px(-15.*fraction-1.))
            .size(px(17.)).p(px(1.)).rounded_full().bg(rgb(ACCENT)).cursor_pointer()
            .shadow(vec![BoxShadow{offset:point(px(0.),px(1.)),..crate::glow(0,0.4,5.)}])
            .child(div().size(px(15.)).rounded_full().bg(rgb(PANEL)).flex().items_center().justify_center()
                .child(div().size(px(9.)).rounded_full().bg(rgb(ACCENT)))))
        // Keep layout/track at 6px while the full 17px outlined thumb owns
        // pointer down and drag, including its part outside the track. Read
        // the track's stored bounds for X mapping, not this larger hitbox.
        .child(div().id("song-output-fader-hit").absolute().top(px(-5.5)).left(px(-1.)).right(px(-1.)).h(px(17.)).cursor_pointer()
        .child(canvas(move|b,w,_|{
            *paint_hitbox.borrow_mut()=Some(w.insert_hitbox(b,HitboxBehavior::Normal));b
        },|_,_,_,_|{}).absolute().inset_0())
        .on_mouse_down(MouseButton::Left,move|e,w,cx| {
            let b=down_bounds.get();
            down_callback(song_output_db_at(f32::from(e.position.x-b.left()),f32::from(b.size.width)),w,cx);
        })
        .on_mouse_move(|_,_,cx|cx.stop_propagation())
        .on_drag(SongOutputSliderDrag(drag_id.clone()),|_,_,_,cx|cx.new(|_|EmptyView))
        .on_drag_move(move|e:&DragMoveEvent<SongOutputSliderDrag>,w,cx|{
            if e.drag(cx).0==drag_id {
                callback(song_output_db_at(f32::from(e.event.position.x-move_bounds.get().left()),f32::from(move_bounds.get().size.width)),w,cx);
            }
        })
        .on_click(|_,_,cx|cx.stop_propagation())
);
    crate::controls::range_accessibility(control,live,-60.,12.,0.5,hitbox,semantic_callback)
}
#[cfg(test)]
mod fader_tests {
    use super::song_output_db_at;
    #[test]
    fn song_fader_pointer_travel_matches_fifteen_pixel_thumb_and_silent_endpoint() {
        assert_eq!(song_output_db_at(7.5,286.),-60.);
        assert_eq!(song_output_db_at(278.5,286.),12.);
        assert_eq!(song_output_db_at(143.,286.),-24.);
        assert_eq!(song_output_db_at(-20.,286.),-60.);
        assert_eq!(song_output_db_at(500.,286.),12.);
    }
}
pub fn song_output_panel(copy:SongOutputCopy,gain:String,peak:f32,mute:Stateful<Div>,unity:Stateful<Div>,fader:Stateful<Div>,clip:Stateful<Div>)->Div {
    let peak_label=if peak>0.000001 {format!("{:.1}",20.*peak.log10())}else{"−∞".into()};
    div().occlude().relative().flex().flex_col().w(px(328.)).p(px(20.)).rounded(px(14.))
        .border_1().border_color(rgba(LINE_STRONG)).bg(rgb(PANEL)).shadow(vec![BoxShadow{offset:point(px(0.),px(20.)),..crate::glow(0x000000,0.53,60.)}])
        .child(div().flex().items_start().justify_between().gap(px(12.))
            .child(div().child(div().font_family(DISPLAY_600).text_size(px(16.)).text_color(rgb(TEXT)).child(copy.title))
                .child(div().mt(px(4.)).font_family(DISPLAY_400).text_size(px(11.)).text_color(rgb(DIM)).child(copy.saved)))
            .child(div().px(px(7.)).py(px(4.)).border_1().border_color(rgba(LINE)).rounded(px(5.)).text_size(px(10.)).text_color(rgb(DIM)).whitespace_nowrap().child(copy.per_song)))
        .child(div().flex().items_center().gap(px(12.)).mt(px(24.)).mb(px(20.)).child(mute)
            .child(div().flex().items_baseline().font_family(MONO_400).text_size(px(28.)).text_color(rgb(TEXT)).whitespace_nowrap().child(gain)
                .child(div().ml(px(7.)).text_size(px(12.)).text_color(rgb(DIM)).child("dB")))
            .child(div().ml_auto().child(unity)))
        .child(div().relative().pb(px(19.)).child(fader)
            .child(ticks(&[("−∞",0.),("−36",1./3.),("−12",2./3.),("0",5./6.),("+12",1.)]).mx(px(7.5)))
            .child(div().absolute().top(px(6.)).left(relative(5./6.)).ml(px(-5.)).w(px(1.)).h(px(4.)).bg(rgb(ACCENT).alpha(0.6))))
        .child(div().pt(px(16.)).border_t_1().border_color(rgba(LINE))
            .child(div().flex().items_center().justify_between().gap(px(8.)).mb(px(11.)).text_size(px(11.)).text_color(rgb(DIM))
                .child(copy.peak).child(div().ml_auto().font_family(MONO_400).text_size(px(9.)).text_color(rgb(FAINT)).child(format!("{peak_label} dBFS"))).child(clip))
            .child(meter(peak,false)).child(ticks(&[("−60",0.),("−36",0.4),("−18",0.7),("−6",0.9),("0",1.)])))
        .child(div().mt(px(15.)).font_family(DISPLAY_400).text_size(px(11.)).line_height(px(17.05)).text_color(rgb(DIM)).child(copy.caption))
}
