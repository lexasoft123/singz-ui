//! Source LibraryImport modal presentation. Filesystem work belongs to the host.
use crate::tokens::*;
use gpui_kit::*;

pub fn library_import_card(viewport: Size<Pixels>) -> Div {
    crate::modal_card().w(px(480_f32.min((f32::from(viewport.width) - 60.).max(0.))))
}
pub fn library_import_head() -> Div {
    crate::project_picker_head()
}
pub fn library_import_title(w: &Window) -> Div {
    crate::project_picker_title("Add to your library", w)
}
pub fn library_import_head_actions() -> Div {
    crate::project_picker_head_actions()
}
pub fn library_import_action(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    enabled: bool,
) -> Stateful<Div> {
    crate::project_picker_action(id, label, enabled)
}
pub fn library_import_actions() -> Div {
    crate::project_picker_storage_actions()
}
pub fn library_import_body(dir: &str, root: &str) -> Div {
    let parts=[("This project lives in ",false),(dir,true),
        (", outside your library. It plays and saves perfectly well there — adding it puts it in ",false),(root,true),
        (", where the Open screen lists it and Drive sync picks it up.",false)];
    let text = parts.iter().map(|(s, _)| *s).collect::<String>();
    let runs = parts
        .iter()
        .map(|(s, bold)| TextRun {
            len: s.len(),
            font: Font {
                family: if *bold { DISPLAY_700 } else { DISPLAY_400 }.into(),
                ..Default::default()
            },
            color: rgb(if *bold { TEXT } else { DIM }).into(),
            background_color: None,
            underline: None,
            strikethrough: None,
        })
        .collect::<Vec<_>>();
    div()
        .min_w_0()
        .text_size(px(13.5))
        .line_height(px(22.275))
        .mb(px(14.))
        .child(StyledText::new(text).with_runs(runs))
}
pub fn library_import_hint(busy: bool) -> Div {
    crate::project_picker_fine(if busy {
        "Working — a project with stems is a few hundred MB, so give it a moment…"
    }else{
        "Copying leaves the original alone, which is what you want for a folder someone else also uses. Moving takes it with you, stems and all."
    }).mt(px(14.))
}
pub fn library_import_entrance(card: Stateful<Div>, progress: f32) -> Stateful<Div> {
    crate::project_picker_entrance(card, progress)
}

/// Source rich-copy spans are identified before expanding untrusted display
/// values: a folder literally containing `**` never becomes translator markup.
fn rich_copy_parts(template: &str, vars: &[(&str, String)]) -> Vec<(String, bool)> {
    fn fill(template: &str, vars: &[(&str, String)]) -> String {
        let mut output=String::new();let mut rest=template;
        while let Some(start)=rest.find('{') {
            output.push_str(&rest[..start]);rest=&rest[start..];
            if let Some(end)=rest.find('}') {
                let key=&rest[1..end];
                if !key.is_empty()&&key.bytes().all(|b|b.is_ascii_alphanumeric()||b==b'_') {
                    match vars.iter().rev().find(|(name,_)|*name==key) {
                        Some((_,value))=>output.push_str(value),None=>output.push_str(&rest[..=end]),
                    }
                    rest=&rest[end+1..];continue;
                }
            }
            output.push('{');rest=&rest[1..];
        }
        output.push_str(rest);output
    }
    let mut parts=vec![];let mut cursor=0;let mut scan=0;
    while let Some(offset)=template[scan..].find("**") {
        let open=scan+offset;let rest=&template[open+2..];
        if let Some(end)=rest.find('*') {
            if end>0&&rest[end..].starts_with("**") {
                if open>cursor {parts.push((fill(&template[cursor..open],vars),false));}
                parts.push((fill(&rest[..end],vars),true));
                cursor=open+2+end+2;scan=cursor;continue;
            }
        }
        scan=open+1;
    }
    if cursor<template.len(){parts.push((fill(&template[cursor..],vars),false));}
    parts
}
pub(crate) fn rich_template_text(template:&str,vars:&[(&str,String)])->StyledText {
    let parts=rich_copy_parts(template,vars);
    let text=parts.iter().map(|(text,_)|text.as_str()).collect::<String>();
    let runs=parts.into_iter().map(|(text,bold)|TextRun {
        len:text.len(),font:Font{family:if bold {DISPLAY_700}else{DISPLAY_400}.into(),..Default::default()},
        color:rgb(if bold {TEXT}else{DIM}).into(),background_color:None,underline:None,strikethrough:None,
    }).collect::<Vec<_>>();
    StyledText::new(text).with_runs(runs)
}
pub fn library_import_title_label(label:impl Into<SharedString>,w:&Window)->Div {
    crate::project_picker_title(label,w)
}
pub fn library_import_body_text(template:impl Into<SharedString>,vars:&[(&str,String)])->Div {
    let template=template.into();
    div().min_w_0().text_size(px(13.5)).line_height(px(22.275)).mb(px(14.))
        .child(rich_template_text(&template,vars))
}
pub fn library_import_hint_text(text:impl Into<SharedString>)->Div {
    crate::project_picker_fine(text).mt(px(14.))
}
#[cfg(test)]
mod rich_copy_tests {
    use super::*;
    #[::core::prelude::v1::test]
    fn literal_path_markers_are_never_parsed_as_source_bold() {
        let parts=rich_copy_parts("Saved in **{root}**. {name}",&[("root","/songs/**mix**/{name}".into()),("name","**literal**".into())]);
        assert_eq!(parts,vec![("Saved in ".into(),false),("/songs/**mix**/{name}".into(),true),(". **literal**".into(),false)]);
    }
    #[::core::prelude::v1::test]
    fn unbalanced_and_nested_stars_follow_the_source_bold_pattern() {
        assert_eq!(rich_copy_parts("A **broken",&[]),vec![("A **broken".into(),false)]);
        assert_eq!(rich_copy_parts("***bold**",&[]),vec![("*".into(),false),("bold".into(),true)]);
    }
}
