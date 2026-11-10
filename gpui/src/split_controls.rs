//! Shared source transport-popover and action styles.
use crate::tokens::*;
use gpui_kit::{prelude::FluentBuilder, *};

use std::{collections::VecDeque, sync::Arc};

// CSS `.sep-pill`: a repeated 200% × 100% image with a physical 100°
// three-stop gradient. GPUI's two-stop, aspect-normalized gradient cannot
// represent it directly. Prepare two periods once per geometry, then move the
// cached image under the native rounded mask; no rasterization depends on phase.
const SHIMMER_ANGLE: f32 = 100.;
const SHIMMER_ALPHA: f32 = 0.09;
const SHIMMER_CACHE_ENTRIES: usize = 2;
const SHIMMER_MAX_TILE_WIDTH: u32 = 2048;
const SHIMMER_MAX_HEIGHT: u32 = 128;

#[derive(Clone, Copy, PartialEq, Eq)]
struct ShimmerKey {
    width: u32,
    height: u32,
    scale: u32,
}
impl ShimmerKey {
    fn new(width: f32, height: f32, scale: f32) -> Option<Self> {
        (width.is_finite()
            && height.is_finite()
            && scale.is_finite()
            && width > 0.
            && width <= 4096.
            && height > 0.
            && height <= 256.
            && scale > 0.
            && scale <= 8.)
            .then_some(Self {
                width: width.to_bits(),
                height: height.to_bits(),
                scale: scale.to_bits(),
            })
    }
    fn dimensions(self) -> (f32, f32) {
        (f32::from_bits(self.width), f32::from_bits(self.height))
    }
    fn texture_size(self) -> (u32, u32) {
        let (width, height) = self.dimensions();
        let density = (f32::from_bits(self.scale) * 4.).clamp(1., 16.);
        (
            (2. * width * density)
                .ceil()
                .clamp(2., SHIMMER_MAX_TILE_WIDTH as f32) as u32,
            (height * density)
                .ceil()
                .clamp(1., SHIMMER_MAX_HEIGHT as f32) as u32,
        )
    }
}

/// CSS gradient-line projection in a single 2W × H background image.
struct ShimmerProjection {
    width: f32,
    height: f32,
    dx: f32,
    dy: f32,
    length: f32,
}
impl ShimmerProjection {
    fn new(width: f32, height: f32) -> Self {
        let angle = SHIMMER_ANGLE.to_radians();
        let (dx, dy) = (angle.sin(), -angle.cos());
        Self {
            width,
            height,
            dx,
            dy,
            length: 2. * width * dx.abs() + height * dy.abs(),
        }
    }
    fn alpha(&self, x: f32, y: f32) -> f32 {
        let stop =
            0.5 + ((x - self.width) * self.dx + (y - self.height / 2.) * self.dy) / self.length;
        (1. - (stop - 0.5).abs() / 0.2).clamp(0., 1.) * SHIMMER_ALPHA
    }
}
#[cfg(test)]
fn shimmer_alpha(width: f32, height: f32, x: f32, y: f32) -> f32 {
    ShimmerProjection::new(width, height).alpha(x, y)
}

fn normalized_phase(phase: f32) -> f32 {
    if phase.is_finite() {
        phase.rem_euclid(1.)
    } else {
        0.
    }
}
fn shimmer_image_left(width: f32, phase: f32) -> f32 {
    // Percentage background positioning uses (area - image), not image width:
    // -200% of (W - 2W) travels +2W. The first of two periods starts at -2W.
    -2. * width + 2. * width * normalized_phase(phase)
}

fn prepare_shimmer(key: ShimmerKey) -> image::RgbaImage {
    let (width, height) = key.dimensions();
    let (tile_width, texture_height) = key.texture_size();
    let mut pixels = image::RgbaImage::new(tile_width * 2, texture_height);
    let projection = ShimmerProjection::new(width, height);
    for y in 0..texture_height {
        let logical_y = (y as f32 + 0.5) * height / texture_height as f32;
        for x in 0..tile_width {
            let logical_x = (x as f32 + 0.5) * (2. * width) / tile_width as f32;
            let alpha = (projection.alpha(logical_x, logical_y) * 255.).round() as u8;
            // RenderImage consumes BGRA. Constant RGB with varying alpha is the
            // premultiplied CSS interpolation from transparent to #ffaa3c.
            let pixel = image::Rgba([0x3c, 0xaa, 0xff, alpha]);
            pixels.put_pixel(x, y, pixel);
            pixels.put_pixel(x + tile_width, y, pixel);
        }
    }
    pixels
}

#[derive(Default)]
struct ShimmerImages {
    ready: VecDeque<(ShimmerKey, Arc<RenderImage>)>,
}
impl Global for ShimmerImages {}
impl ShimmerImages {
    fn image(&mut self, key: ShimmerKey) -> (Arc<RenderImage>, Option<Arc<RenderImage>>) {
        if let Some(index) = self.ready.iter().position(|(old, _)| *old == key) {
            let entry = self.ready.remove(index).expect("located split shimmer");
            let image = entry.1.clone();
            self.ready.push_back(entry);
            return (image, None);
        }
        let image = Arc::new(RenderImage::new(vec![image::Frame::new(prepare_shimmer(
            key,
        ))]));
        let evicted = if self.ready.len() == SHIMMER_CACHE_ENTRIES {
            self.ready.pop_front().map(|(_, image)| image)
        } else {
            None
        };
        self.ready.push_back((key, image.clone()));
        (image, evicted)
    }
}

fn paint_split_shimmer(bounds: Bounds<Pixels>, phase: f32, window: &mut Window, cx: &mut App) {
    let (width, height) = (f32::from(bounds.size.width), f32::from(bounds.size.height));
    let Some(key) = ShimmerKey::new(width, height, window.scale_factor()) else {
        return;
    };
    let (image, evicted) = cx.default_global::<ShimmerImages>().image(key);
    if let Some(old) = evicted {
        // RenderImage has no atlas-removing Drop: release CPU AND GPU cache
        // entries, including this currently borrowed window's atlas.
        cx.drop_image(old, Some(window));
    }
    let image_bounds = Bounds::new(
        point(
            bounds.left() + px(shimmer_image_left(width, phase)),
            bounds.top(),
        ),
        size(px(width * 4.), bounds.size.height),
    );
    let _ = window.paint_image(
        bounds,
        image_bounds,
        Corners::all(px(999.)),
        image,
        0,
        false,
    );
}

pub fn transport_popover() -> Div {
    div()
        .occlude()
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .p(px(14.))
        .border_1()
        .border_color(rgba(LINE_STRONG))
        .rounded(px(12.))
        .bg(rgb(0x1d1915))
        .shadow(vec![BoxShadow {
            color: rgba(0x00000080).into(),
            offset: point(px(0.), px(12.)),
            blur_radius: px(17.),
            spread_radius: px(0.),
            inset: false,
        }])
}
pub fn split_action(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    enabled: bool,
) -> Stateful<Div> {
    div()
        .id(id)
        .w_full()
        .py(px(10.))
        .px(px(8.))
        .rounded(px(8.))
        .text_size(px(14.))
        .line_height(px(18.))
        .text_color(rgb(TEXT))
        .child(label.into())
        .when(enabled, |d| {
            d.cursor_pointer()
                .hover(|d| d.bg(rgba(ACCENT_SOFT)).text_color(rgb(ACCENT)))
        })
        .when(!enabled, |d| d.opacity(0.45).cursor_default())
}

/// Footer separation status (`.sep-pill`), replacing the split actions while a job runs.
/// The host supplies a 1.6-second phase so hidden captures can pin animation time.
pub fn split_progress(label: impl Into<SharedString>, percent: f32, phase: f32) -> Div {
    div()
        .relative()
        .flex()
        .items_center()
        .gap(px(12.))
        .pl(px(18.))
        .pr(px(10.))
        .py(px(9.))
        .rounded(px(999.))
        .border_1()
        .border_color(rgba(0xffa02859))
        .bg(rgba(ACCENT_SOFT))
        .font_family(DISPLAY_600)
        .text_size(px(13.))
        .line_height(px(15.))
        .text_color(rgb(ACCENT))
        .child(
            canvas(
                |b, _, _| b,
                move |bounds, _, window, cx| paint_split_shimmer(bounds, phase, window, cx),
            )
            .absolute()
            .top_0()
            .left_0()
            .size_full(),
        )
        .child(div().relative().child(label.into()))
        .child(
            div()
                .relative()
                .min_w(px(38.))
                .text_right()
                .font_family("SingZ Mono 600")
                .text_size(px(12.))
                .line_height(px(14.))
                .child(format!("{}%", percent.clamp(0., 100.).round() as u32)),
        )
}

pub fn split_cancel(id: impl Into<ElementId>) -> Stateful<Div> {
    div()
        .id(id)
        .relative()
        .size(px(24.))
        .flex_shrink_0()
        .rounded(px(999.))
        .border_1()
        .border_color(rgba(0xffa02859))
        .text_color(rgb(ACCENT))
        .font_family(DISPLAY_600)
        .text_size(px(15.))
        .line_height(px(15.))
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .hover(|d| d.bg(rgba(0xffa0282e)))
        .child("×")
}

#[cfg(test)]
mod shimmer_tests {
    use super::{
        normalized_phase, prepare_shimmer, shimmer_alpha, shimmer_image_left, ShimmerImages,
        ShimmerKey, SHIMMER_CACHE_ENTRIES,
    };
    use std::sync::Arc;

    #[test]
    fn css_positioning_travels_right_and_wraps_after_one_linear_period() {
        assert_eq!(shimmer_image_left(200., 0.), -400.);
        assert_eq!(shimmer_image_left(200., 0.25), -300.);
        assert_eq!(shimmer_image_left(200., 0.5), -200.);
        assert_eq!(shimmer_image_left(200., 0.75), -100.);
        assert_eq!(shimmer_image_left(200., 1.), -400.);
        // At 3/4 of 1.6s, the first image's center is at viewport center.
        assert!((shimmer_alpha(200., 35., 200., 17.5) - 0.09).abs() < 0.000001);
        for x in [0., 50., 100., 150., 200.] {
            let image_x = (x - shimmer_image_left(200., 0.25)).rem_euclid(400.);
            assert_eq!(shimmer_alpha(200., 35., image_x, 17.5), 0.);
        }
    }

    #[test]
    fn physical_hundred_degree_peak_tilt_is_independent_of_pill_width() {
        // A 35px-high 100° CSS gradient shifts its peak 6.171444px left
        // between top and bottom; a horizontal/aspect-scaled substitute fails.
        for width in [200., 320.] {
            assert!((shimmer_alpha(width, 35., width + 3.0857228, 0.) - 0.09).abs() < 0.000001);
            assert!((shimmer_alpha(width, 35., width - 3.0857228, 35.) - 0.09).abs() < 0.000001);
        }
        assert!(shimmer_alpha(200., 35., 280., 35.) < shimmer_alpha(200., 35., 280., 0.));
    }

    #[test]
    fn texture_is_two_identical_horizontal_periods_with_source_colour() {
        let key = ShimmerKey::new(200., 35., 2.).unwrap();
        let pixels = prepare_shimmer(key);
        let (tile_width, height) = key.texture_size();
        assert_eq!(pixels.dimensions(), (tile_width * 2, height));
        assert!(pixels.pixels().any(|pixel| pixel[3] == 23));
        for y in 0..height {
            for x in 0..tile_width {
                let pixel = pixels.get_pixel(x, y);
                assert_eq!(pixel, pixels.get_pixel(x + tile_width, y));
                assert_eq!(&pixel.0[..3], &[0x3c, 0xaa, 0xff]);
                assert!(pixel[3] <= 23);
            }
        }
    }

    #[test]
    fn cache_reuses_phase_independent_texture_and_returns_evictions() {
        let mut cache = ShimmerImages::default();
        let key = ShimmerKey::new(20., 10., 1.).unwrap();
        let (first, evicted) = cache.image(key);
        assert!(evicted.is_none());
        let (again, evicted) = cache.image(key);
        assert!(Arc::ptr_eq(&first, &again));
        assert!(evicted.is_none());
        cache.image(ShimmerKey::new(21., 10., 1.).unwrap());
        let (_, evicted) = cache.image(ShimmerKey::new(22., 10., 1.).unwrap());
        assert!(Arc::ptr_eq(&first, &evicted.unwrap()));
        assert_eq!(cache.ready.len(), SHIMMER_CACHE_ENTRIES);
    }

    #[test]
    fn geometry_rejects_invalid_inputs_and_caps_each_texture_at_two_mib() {
        for (width, height, scale) in [
            (0., 35., 2.),
            (f32::NAN, 35., 2.),
            (200., f32::INFINITY, 2.),
            (200., 35., 0.),
        ] {
            assert!(ShimmerKey::new(width, height, scale).is_none());
        }
        let (tile_width, height) = ShimmerKey::new(4096., 256., 8.).unwrap().texture_size();
        assert_eq!(tile_width * 2 * height * 4, 2 * 1024 * 1024);
        assert_eq!(normalized_phase(f32::NAN), 0.);
    }
}
