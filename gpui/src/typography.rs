//! Shaped text with explicit tracking. Preserves native shaping and glyph
//! positions, adding spacing at cluster boundaries instead of inserting spaces.
use gpui_kit::*;

pub struct TrackedText {
    text: SharedString,
    family: SharedString,
    size: f32,
    tracking: f32,
    line_height: Option<f32>,
    color: u32,
    inherit_style: bool,
    wrap: bool,
    use_normal_line_height: bool,
}

/// Explicit typography; tracking and optional line-height overrides use pixels.
pub fn tracked_text(
    text: impl Into<SharedString>,
    family: impl Into<SharedString>,
    size: f32,
    tracking: f32,
    color: u32,
) -> TrackedText {
    TrackedText {
        text: text.into(),
        family: family.into(),
        size,
        tracking,
        line_height: None,
        color,
        inherit_style: false,
        wrap: false,
        use_normal_line_height: false,
    }
}

/// A label inheriting the host's resolved text style, with CSS em tracking.
/// Resolve during layout so parent size/family/colour overrides and disabled
/// variants remain effective. The parent's line box is preserved unchanged.
pub fn tracked_label(text: impl Into<SharedString>, tracking_em: f32) -> TrackedText {
    TrackedText {
        text: text.into(),
        family: "".into(),
        size: 0.,
        tracking: tracking_em,
        line_height: None,
        color: 0,
        inherit_style: true,
        wrap: false,
        use_normal_line_height: false,
    }
}

impl TrackedText {
    /// Soft-wrap at Unicode line-break opportunities while retaining complete
    /// shaped clusters. CSS-normal spaces collapse; long words remain intact.
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }
    /// Inherit font and colour while explicitly selecting CSS-normal metrics,
    /// rather than a numeric parent line-height approximation.
    pub fn normal_line_height(mut self) -> Self {
        self.line_height = None;
        self.use_normal_line_height = true;
        self
    }
    /// Explicit CSS line box, independent of glyph ascent/descent.
    pub fn line_height(mut self, height: f32) -> Self {
        self.line_height = Some(height);
        self.use_normal_line_height = false;
        self
    }
}
impl IntoElement for TrackedText {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}
// Opt-in diagnostics for the hidden parity driver. The disabled path reads the
// environment once per process and performs no logging or per-frame allocation.
fn text_diagnostics(label: &str, phase: &str, message: impl FnOnce() -> String) {
    use std::{
        collections::HashSet,
        sync::{Mutex, OnceLock},
    };
    static ENABLED: OnceLock<bool> = OnceLock::new();
    if !*ENABLED.get_or_init(|| std::env::var_os("SINGZ_GPUI_TEXT_DIAGNOSTICS").is_some())
        || !matches!(label, "Sing" | "Z" | "dev")
    {
        return;
    }
    static SEEN: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
    let key = format!("{label}:{phase}");
    if SEEN
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .unwrap()
        .insert(key)
    {
        eprintln!("SingZ text {label} {phase}: {}", message());
    }
}

/// Chromium's measured line box and baseline policy. Font ascent/descent
/// round independently in device pixels; fractional leading is floored after
/// quantizing the physical line height to the CSS 1/64 grid. `None` is CSS
/// normal, not an assumed 1.2 × font-size multiplier.
pub(crate) fn css_line_metrics(ascent: f32, descent: f32, line_height: Option<f32>, scale: f32) -> (f32, f32) {
    let ascent = (ascent * scale).round();
    let descent = (descent * scale).round();
    let height = line_height.unwrap_or((ascent + descent) / scale);
    let physical_height = (height * scale * 64.).round() / 64.;
    let baseline = (ascent + ((physical_height - ascent - descent) / 2.).floor()) / scale;
    (height, baseline)
}

/// CSS-normal line height and baseline for aligning independent native text
/// boxes. Own the family string so callers may pass dynamic or static names.
pub fn css_normal_line_metrics(window: &Window, family: &str, font_size: f32) -> (Pixels, Pixels) {
    let line = window.text_system().shape_line(
        "M".into(),
        px(font_size),
        &[TextRun {
            len: 1,
            font: Font {
                family: family.to_owned().into(),
                ..Default::default()
            },
            color: rgb(0).into(),
            background_color: None,
            underline: None,
            strikethrough: None,
        }],
        None,
    );
    let (height, baseline) = css_line_metrics(
        f32::from(line.ascent),
        f32::from(line.descent),
        None,
        window.scale_factor(),
    );
    (px(height), px(baseline))
}

/// Resolved typography shared by layout, truncation and painting.
pub struct TrackedLayout {
    line: ShapedLine,
    full_line: ShapedLine,
    lines: Vec<ShapedLine>,
    wrap: bool,
    font: Font,
    font_size: Pixels,
    line_height: Pixels,
    spacing: Pixels,
    color: Hsla,
    overflow: Option<TextOverflow>,
    align: TextAlign,
}

fn cluster_boundaries(text: &str, line: &ShapedLine) -> Vec<usize> {
    let mut indices: Vec<_> = line
        .runs
        .iter()
        .flat_map(|r| r.glyphs.iter().map(|g| g.index))
        .filter(|&index| index <= text.len() && text.is_char_boundary(index))
        .collect();
    indices.push(0);
    indices.push(text.len());
    indices.sort_unstable();
    indices.dedup();
    indices
}
fn tracked_width(line: &ShapedLine, spacing: Pixels) -> Pixels {
    let mut indices: Vec<_> = line
        .runs
        .iter()
        .flat_map(|r| r.glyphs.iter().map(|g| g.index))
        .collect();
    indices.sort_unstable();
    indices.dedup();
    line.width + spacing * indices.len() as f32
}
#[derive(Clone, Copy)]
enum Truncation {
    End,
    Start,
    Middle,
}

/// Binary search only complete shaped clusters, including multi-byte UTF-8,
/// combining sequences and shaped emoji clusters. At most log2(n)+2 fits.
fn ellipsized_text(
    text: &str,
    boundaries: &[usize],
    affix: &str,
    mode: Truncation,
    mut fits: impl FnMut(&str) -> bool,
) -> String {
    if !fits(affix) {
        return String::new();
    }
    let count = boundaries.len().saturating_sub(1);
    let candidate = |keep: usize| -> String {
        match mode {
            Truncation::End => format!("{}{}", &text[..boundaries[keep]], affix),
            Truncation::Start => format!("{}{}", affix, &text[boundaries[count - keep]..]),
            Truncation::Middle => {
                let left = (keep + 1) / 2;
                let right = keep / 2;
                format!(
                    "{}{}{}",
                    &text[..boundaries[left]],
                    affix,
                    &text[boundaries[count - right]..]
                )
            }
        }
    };
    let (mut low, mut high) = (0, count);
    while low < high {
        let mid = low + (high - low + 1) / 2;
        if fits(&candidate(mid)) {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    candidate(low)
}

fn css_normal_spaces(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut pending = false;
    for ch in text.chars() {
        if matches!(ch, ' ' | '\t' | '\r' | '\n' | '\u{c}') {
            pending = !result.is_empty();
        } else {
            if pending {
                result.push(' ');
                pending = false;
            }
            result.push(ch);
        }
    }
    result
}

fn wrap_text(text: &str, boundaries: &[usize], mut fits: impl FnMut(&str) -> bool) -> Vec<String> {
    if text.is_empty() {
        return vec![];
    }
    let ends: Vec<_> = unicode_linebreak::linebreaks(text)
        .map(|(index, _)| index)
        .filter(|index| boundaries.binary_search(index).is_ok())
        .collect();
    let mut lines = vec![];
    let mut start = 0;
    while start < text.len() {
        let mut chosen = None;
        for &end in ends.iter().filter(|&&end| end > start) {
            let candidate = text[start..end].trim_end_matches(' ');
            let fits = fits(candidate);
            if fits || chosen.is_none() {
                chosen = Some(end);
            } else {
                break;
            }
            if !fits {
                break;
            } // CSS normal does not split an unbreakable word.
        }
        let end = chosen.unwrap_or(text.len());
        lines.push(text[start..end].trim_end_matches(' ').to_owned());
        start = end;
        while start < text.len() && text.as_bytes()[start] == b' ' {
            start += 1;
        }
    }
    lines
}

fn shape_lines(
    text: &str,
    full: &ShapedLine,
    width: Pixels,
    font: &Font,
    font_size: Pixels,
    spacing: Pixels,
    color: Hsla,
    window: &Window,
) -> Vec<ShapedLine> {
    let boundaries = cluster_boundaries(text, full);
    let shape = |text: &str| {
        window.text_system().shape_line(
            text.to_owned().into(),
            font_size,
            &[TextRun {
                len: text.len(),
                font: font.clone(),
                color,
                background_color: None,
                underline: None,
                strikethrough: None,
            }],
            None,
        )
    };
    wrap_text(text, &boundaries, |text| {
        tracked_width(&shape(text), spacing) <= width
    })
    .into_iter()
    .map(|text| shape(&text))
    .collect()
}

impl Element for TrackedText {
    type RequestLayoutState = TrackedLayout;
    type PrepaintState = ();
    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        w: &mut Window,
        _: &mut App,
    ) -> (LayoutId, TrackedLayout) {
        let style = w.text_style();
        let (font, font_size, spacing, requested_line_height, color) = if self.inherit_style {
            let size = style.font_size.to_pixels(w.rem_size());
            let line = if self.use_normal_line_height {
                None
            } else {
                Some(self.line_height.unwrap_or_else(|| {
                    f32::from(style.line_height.to_pixels(size.into(), w.rem_size()))
                }))
            };
            (style.font(), size, size * self.tracking, line, style.color)
        } else {
            (
                Font {
                    family: self.family.clone(),
                    ..Default::default()
                },
                px(self.size),
                px(self.tracking),
                self.line_height,
                rgb(self.color).into(),
            )
        };
        let source_text: SharedString = if self.wrap {
            css_normal_spaces(&self.text).into()
        } else {
            self.text.clone()
        };
        let line = w.text_system().shape_line(
            source_text.clone(),
            font_size,
            &[TextRun {
                len: source_text.len(),
                font: font.clone(),
                color,
                background_color: None,
                underline: None,
                strikethrough: None,
            }],
            None,
        );
        let (height, _) = css_line_metrics(
            f32::from(line.ascent),
            f32::from(line.descent),
            requested_line_height,
            w.scale_factor(),
        );
        let line_height = px(height);
        let width = tracked_width(&line, spacing);
        let overflow = if self.inherit_style {
            style.text_overflow.clone()
        } else {
            None
        };
        let can_truncate = overflow.is_some();
        let wrap = self.wrap;
        let measured_font = font.clone();
        let measured_line = line.clone();
        let id = w.request_measured_layout(Style::default(), move |known, available, window, _| {
            let min_content =
                known.width.is_none() && matches!(available.width, AvailableSpace::MinContent);
            let limit = known.width.or(match available.width {
                AvailableSpace::Definite(width) => Some(width),
                AvailableSpace::MinContent => Some(px(0.)),
                AvailableSpace::MaxContent => None,
            });
            if wrap {
                if let Some(limit) = limit {
                    let lines = shape_lines(
                        &source_text,
                        &measured_line,
                        limit,
                        &measured_font,
                        font_size,
                        spacing,
                        color,
                        window,
                    );
                    let width = lines
                        .iter()
                        .map(|line| tracked_width(line, spacing))
                        .fold(px(0.), Pixels::max);
                    return size(
                        if min_content {
                            width
                        } else {
                            width.min(limit.max(px(0.)))
                        },
                        line_height * lines.len() as f32,
                    );
                }
            }
            let bounded = if can_truncate {
                limit.map_or(width, |limit| width.min(limit.max(px(0.))))
            } else {
                width
            };
            size(bounded, line_height)
        });
        text_diagnostics(self.text.as_ref(), "layout", || {
            format!("font_size={font_size:?} line_height={line_height:?} tracking={spacing:?} ascent={:?} descent={:?} width={width:?} color={color:?}",line.ascent,line.descent)
        });
        let layout = TrackedLayout {
            full_line: line.clone(),
            lines: vec![line.clone()],
            wrap: self.wrap,
            line,
            font,
            font_size,
            line_height,
            spacing,
            color,
            overflow,
            align: if self.inherit_style {
                style.text_align
            } else {
                TextAlign::Left
            },
        };
        (id, layout)
    }
    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut TrackedLayout,
        w: &mut Window,
        _: &mut App,
    ) {
        layout.line = layout.full_line.clone();
        if layout.wrap {
            layout.lines = shape_lines(
                &layout.full_line.text,
                &layout.full_line,
                bounds.size.width,
                &layout.font,
                layout.font_size,
                layout.spacing,
                layout.color,
                w,
            );
            return;
        }
        layout.lines = vec![layout.line.clone()];
        if tracked_width(&layout.full_line, layout.spacing) <= bounds.size.width {
            return;
        }
        let Some(overflow) = &layout.overflow else {
            return;
        };
        let (affix, mode) = match overflow {
            TextOverflow::Truncate(text) => (text.clone(), Truncation::End),
            TextOverflow::TruncateStart(text) => (text.clone(), Truncation::Start),
            TextOverflow::TruncateMiddle(text) => (text.clone(), Truncation::Middle),
        };
        let boundaries = cluster_boundaries(&self.text, &layout.full_line);
        let shape = |text: &str| {
            w.text_system().shape_line(
                text.to_owned().into(),
                layout.font_size,
                &[TextRun {
                    len: text.len(),
                    font: layout.font.clone(),
                    color: layout.color,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                }],
                None,
            )
        };
        let text = ellipsized_text(&self.text, &boundaries, &affix, mode, |text| {
            tracked_width(&shape(text), layout.spacing) <= bounds.size.width
        });
        layout.line = shape(&text);
        layout.lines = vec![layout.line.clone()];
    }
    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut TrackedLayout,
        _: &mut (),
        w: &mut Window,
        _: &mut App,
    ) {
        for (row, line) in layout.lines.iter().enumerate() {
            let mut clusters: Vec<_> = line
                .runs
                .iter()
                .flat_map(|r| r.glyphs.iter().map(|g| g.index))
                .collect();
            clusters.sort_unstable();
            clusters.dedup();
            let width = tracked_width(line, layout.spacing);
            let free = (bounds.size.width - width).max(px(0.));
            let left = match layout.align {
                TextAlign::Left => px(0.),
                TextAlign::Center => free / 2.,
                TextAlign::Right => free,
            };
            let (_, baseline) = css_line_metrics(
                f32::from(line.ascent),
                f32::from(line.descent),
                Some(f32::from(layout.line_height)),
                w.scale_factor(),
            );
            let baseline = px(baseline) + layout.line_height * row as f32;
            text_diagnostics(self.text.as_ref(), "paint", || {
                format!("bounds={bounds:?} baseline={:?} font_size={:?} tracking={:?} painted_text={:?}",bounds.top()+baseline,layout.font_size,layout.spacing,line.text)
            });
            for run in &line.runs {
                for glyph in &run.glyphs {
                    let cluster = clusters.binary_search(&glyph.index).unwrap_or(0);
                    let origin = bounds.origin
                        + glyph.position
                        + point(left + layout.spacing * cluster as f32, baseline);
                    if glyph.is_emoji {
                        let _ = w.paint_emoji(origin, run.font_id, glyph.id, layout.font_size);
                    } else {
                        let _ = w.paint_glyph(
                            origin,
                            run.font_id,
                            glyph.id,
                            layout.font_size,
                            layout.color,
                        );
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[::core::prelude::v1::test]
    fn css_wrap_keeps_words_nonbreaking_spaces_and_clusters() {
        let text = css_normal_spaces("  one \n two\tthree ");
        assert_eq!(text, "one two three");
        let boundaries: Vec<_> = text
            .char_indices()
            .map(|(i, _)| i)
            .chain([text.len()])
            .collect();
        assert_eq!(
            wrap_text(&text, &boundaries, |s| s.len() <= 7),
            ["one two", "three"]
        );
        let text = "longword end";
        let boundaries: Vec<_> = (0..=text.len()).collect();
        assert_eq!(
            wrap_text(text, &boundaries, |s| s.len() <= 4),
            ["longword", "end"]
        );
        let text = "a\u{a0}b c";
        let boundaries: Vec<_> = text
            .char_indices()
            .map(|(i, _)| i)
            .chain([text.len()])
            .collect();
        assert_eq!(
            wrap_text(text, &boundaries, |s| s.len() <= 2),
            ["a\u{a0}b", "c"]
        );
        let text = "👩\u{200d}🎤 word";
        let boundaries = [0, 11, 12, 16];
        assert_eq!(
            wrap_text(text, &boundaries, |s| s.len() <= 4),
            ["👩\u{200d}🎤", "word"]
        );
    }
    #[::core::prelude::v1::test]
    fn measured_css_font_line_box_fixtures() {
        // Bricolage 930/-270 units/em; these are representative rows from
        // the independent 576-case Chromium/CoreText line-metric probe.
        for (ascent, descent, scale, height, baseline) in [
            (15.809875, 4.59007, 2., 20.5, 16.), // 17px header logo
            (9.765, 2.835, 2., 13., 10.),        // 10.5px version/ruler
            (48.36, 14.04, 2., 62.5, 48.5),      // 52px catalog headline
            (12.09, 3.51, 2., 15.5, 12.),        // 13px pill
            (11.16, 3.24, 2., 14., 11.),         // 12px small pill
            (15.809875, 4.59007, 1., 21., 16.),  // same face, DPR 1
        ] {
            assert_eq!(
                css_line_metrics(ascent, descent, None, scale),
                (height, baseline)
            );
        }
    }
    #[::core::prelude::v1::test]
    fn explicit_numeric_leading_is_preserved_and_floored() {
        assert_eq!(css_line_metrics(48.36, 14.04, Some(62.4), 2.), (62.4, 48.));
        assert_eq!(css_line_metrics(48.36, 14.04, Some(62.5), 2.), (62.5, 48.5));
        assert_eq!(css_line_metrics(9.765, 2.835, Some(12.9), 2.), (12.9, 9.5));
        assert_eq!(css_line_metrics(9.765, 2.835, Some(13.1), 2.), (13.1, 10.));
    }
    #[::core::prelude::v1::test]
    fn ellipsis_preserves_combining_clusters_and_utf8() {
        let text = "Ae\u{301}Z";
        let result = ellipsized_text(text, &[0, 1, 4, 5], "…", Truncation::End, |s| {
            s.len() <= 5
        });
        assert_eq!(result, "A…"); // An incomplete e + combining mark must never fit.
        let result = ellipsized_text("AB🙂Z", &[0, 1, 2, 6, 7], "…", Truncation::Start, |s| {
            s.len() <= 8
        });
        assert_eq!(result, "…🙂Z");
    }
    #[::core::prelude::v1::test]
    fn middle_ellipsis_and_too_narrow_bounds() {
        assert_eq!(
            ellipsized_text(
                "AB🙂CD",
                &[0, 1, 2, 6, 7, 8],
                "…",
                Truncation::Middle,
                |s| s.len() <= 6
            ),
            "AB…D"
        );
        assert_eq!(
            ellipsized_text("abc", &[0, 1, 2, 3], "…", Truncation::End, |s| s.len()
                < 3),
            ""
        );
    }
    #[::core::prelude::v1::test]
    fn long_labels_use_logarithmically_bounded_probes() {
        let text = "x".repeat(65536);
        let boundaries: Vec<_> = (0..=text.len()).collect();
        let mut probes = 0;
        let result = ellipsized_text(&text, &boundaries, "…", Truncation::End, |s| {
            probes += 1;
            s.len() <= 32
        });
        assert_eq!(result.len(), 32);
        assert!(probes <= 18, "unbounded probes: {probes}");
    }
}
