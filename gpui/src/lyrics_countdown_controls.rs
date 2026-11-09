//! Source lyric gap cues. Raster/blur once off-thread; paint cached pulse levels.
use crate::tokens::*;
use gpui_kit::*;
use image::{GrayImage, Luma};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::Arc,
};

const PAD: f32 = 15.;
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Key {
    text: String,
    tracking: u32,
    scale: u32,
}
pub struct LyricCountdownSprite {
    levels: Vec<Arc<RenderImage>>,
    size: Size<Pixels>,
}
#[derive(Default)]
struct Sprites {
    ready: HashMap<Key, Arc<LyricCountdownSprite>>,
    pending: HashSet<Key>,
    failed: HashSet<Key>,
    order: VecDeque<Key>,
}
impl Global for Sprites {}

fn prepare(text_system: Arc<TextSystem>, key: Key) -> anyhow::Result<LyricCountdownSprite> {
    let scale = f32::from_bits(key.scale);
    let tracking = f32::from_bits(key.tracking);
    anyhow::ensure!(
        (0.5..=4.).contains(&scale)
            && (tracking == 1. || tracking == 5.)
            && key.text.len() <= 32
            && !key.text.is_empty()
            && key
                .text
                .chars()
                .all(|c| c.is_ascii_digit() || matches!(c, ' ' | 's' | '●')),
        "Invalid lyric countdown caption"
    );
    let text = WindowTextSystem::new(text_system);
    let line = text.shape_line(
        key.text.clone().into(),
        px(11.),
        &[TextRun {
            len: key.text.len(),
            font: Font {
                family: DISPLAY_650.into(),
                ..Default::default()
            },
            color: white(),
            background_color: None,
            underline: None,
            strikethrough: None,
        }],
        None,
    );
    let mut clusters = line
        .runs
        .iter()
        .flat_map(|r| r.glyphs.iter().map(|g| g.index))
        .collect::<Vec<_>>();
    clusters.sort_unstable();
    clusters.dedup();
    let width = f32::from(line.width) + tracking * clusters.len() as f32;
    let (_, baseline) = crate::typography::css_line_metrics(
        f32::from(line.ascent),
        f32::from(line.descent),
        Some(11.),
        scale,
    );
    let width = ((width + 2. * PAD) * scale).ceil() as u32;
    let height = ((11. + 2. * PAD) * scale).ceil() as u32;
    anyhow::ensure!(
        width > 0 && width <= 1024 && height > 0 && height <= 256,
        "Lyric countdown sprite exceeds its bound"
    );
    let mut alpha = GrayImage::new(width, height);
    for run in &line.runs {
        for glyph in &run.glyphs {
            let params = RenderGlyphParams {
                font_id: run.font_id,
                glyph_id: glyph.id,
                font_size: px(11.),
                subpixel_variant: point(0, 0),
                scale_factor: scale,
                is_emoji: glyph.is_emoji,
                css_rasterization: true,
                subpixel_rendering: false,
                dilation: 0,
            };
            let (bounds, bytes) = text.glyph_image_mask(&params)?;
            let cluster = clusters.binary_search(&glyph.index).unwrap_or(0);
            let left = ((PAD + f32::from(glyph.position.x) + tracking * cluster as f32) * scale)
                .round() as i32
                + bounds.origin.x.0;
            let top = ((PAD + baseline + f32::from(glyph.position.y)) * scale).round() as i32
                + bounds.origin.y.0;
            let glyph_width = bounds.size.width.0.max(0) as u32;
            let glyph_height = bounds.size.height.0.max(0) as u32;
            for y in 0..glyph_height {
                for x in 0..glyph_width {
                    let (dx, dy) = (left + x as i32, top + y as i32);
                    if dx < 0 || dy < 0 || dx >= width as i32 || dy >= height as i32 {
                        continue;
                    }
                    let at = (y * glyph_width + x) as usize;
                    let a = bytes
                        .get(if glyph.is_emoji { at * 4 + 3 } else { at })
                        .copied()
                        .unwrap_or(0);
                    let old = alpha.get_pixel(dx as u32, dy as u32)[0];
                    alpha.put_pixel(dx as u32, dy as u32, Luma([old.saturating_add(a)]));
                }
            }
        }
    }
    anyhow::ensure!(
        alpha.pixels().any(|pixel| pixel[0] > 0),
        "Lyric countdown caption has no glyph pixels"
    );
    // CSS blur-radius 10px uses sigma 5px. Keep text and its same-colour
    // shadow in one sprite so the stepped opacity composites them together.
    let shadow = image::imageops::blur(&alpha, 5. * scale);
    let levels = (0..=if cfg!(target_os = "windows") { 0 } else { 8 })
        .map(|level| {
            let opacity = 1. - 0.55 * level as f32 / 8.;
            let pixels = image::RgbaImage::from_fn(width, height, |x, y| {
                let glyph = alpha.get_pixel(x, y)[0] as f32 / 255.;
                let glow = shadow.get_pixel(x, y)[0] as f32 / 255. * 0.6;
                image::Rgba([
                    40,
                    160,
                    255,
                    ((glyph + glow * (1. - glyph)) * opacity * 255.).round() as u8,
                ])
            });
            Arc::new(RenderImage::new(vec![image::Frame::new(pixels)]))
        })
        .collect();
    Ok(LyricCountdownSprite {
        levels,
        size: size(px(width as f32 / scale), px(height as f32 / scale)),
    })
}

/// A missing caption queues bounded preparation; no glyph rasterization or blur
/// runs on the UI/paint thread. Call while paused too to prewarm an upcoming cue.
pub fn request_lyric_countdown_sprite(
    text: &str,
    tracking: f32,
    w: &mut Window,
    cx: &mut App,
) -> Option<Arc<LyricCountdownSprite>> {
    let key = Key {
        text: text.into(),
        tracking: tracking.to_bits(),
        scale: w.scale_factor().to_bits(),
    };
    let cache = cx.default_global::<Sprites>();
    if let Some(sprite) = cache.ready.get(&key) {
        return Some(sprite.clone());
    }
    if cache.pending.contains(&key) || cache.failed.contains(&key) || cache.pending.len() >= 4 {
        return None;
    }
    cache.pending.insert(key.clone());
    let text_system = cx.text_system().clone();
    let handle = w.window_handle();
    let build_key = key.clone();
    let task = cx
        .background_executor()
        .spawn(async move { prepare(text_system, build_key) });
    cx.spawn(async move |cx| {
        let result = task.await;
        let _ = cx.update(|cx| {
            let cache = cx.default_global::<Sprites>();
            cache.pending.remove(&key);
            let mut evicted = Vec::new();
            while cache.order.len() >= 16 {
                if let Some(old) = cache.order.pop_front() {
                    if let Some(sprite) = cache.ready.remove(&old) {
                        evicted.extend(sprite.levels.iter().cloned());
                    }
                    cache.failed.remove(&old);
                }
            }
            cache.order.push_back(key.clone());
            match result {
                Ok(sprite) => {
                    cache.ready.insert(key, Arc::new(sprite));
                }
                Err(_) => {
                    cache.failed.insert(key);
                }
            }
            // RenderImage has no atlas-removing Drop. Release uploaded pulse
            // textures as well as CPU entries when a caption leaves the cache.
            for image in evicted {
                cx.drop_image(image, None);
            }
            let _ = handle.update(cx, |_, w, _| w.refresh());
        });
    })
    .detach();
    None
}

/// `origin` is the pseudo-element's 11px line-box origin, before its 4px margin.
pub fn paint_lyric_countdown(
    sprite: Arc<LyricCountdownSprite>,
    origin: Point<Pixels>,
    level: u8,
    w: &mut Window,
) -> bool {
    let bounds = Bounds::new(origin - point(px(PAD), px(PAD)), sprite.size);
    if let Some(image) = sprite.levels.get(level.min(8) as usize) {
        return w
            .paint_image(bounds, bounds, Corners::default(), image.clone(), 0, false)
            .is_ok();
    }
    false
}
