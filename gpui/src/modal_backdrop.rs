//! Modal-free backdrop caching. The host supplies its paint-prefix capture API;
//! this crate does not depend on a patched renderer or present any window.
use anyhow::{Result, ensure};
use gpui_kit::{App, Bounds, Corners, Pixels, RenderImage, Size, Window};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModalBackdropKey {
    width: u32,
    height: u32,
    scale: u32,
    content: u64,
}
impl ModalBackdropKey {
    pub fn new(viewport: Size<Pixels>, scale: f32, content_revision: u64) -> Self {
        Self { width: f32::from(viewport.width).to_bits(),
            height: f32::from(viewport.height).to_bits(), scale: scale.to_bits(),
            content: content_revision }
    }
    fn valid(self) -> bool {
        [self.width, self.height, self.scale].into_iter()
            .all(|value| f32::from_bits(value).is_finite() && f32::from_bits(value) > 0.)
    }
}

#[derive(Default)]
pub struct ModalBackdropCache {
    key: Option<ModalBackdropKey>,
    failed_key: Option<ModalBackdropKey>,
    image: Option<Arc<RenderImage>>,
    captures: u64,
    last_error: Option<String>,
    raster_size: Option<[u32; 2]>,
}
impl ModalBackdropCache {
    pub fn captures(&self) -> u64 { self.captures }
    pub fn image(&self) -> Option<Arc<RenderImage>> { self.image.clone() }
    pub fn raster_size(&self) -> Option<[u32; 2]> { self.raster_size }
    pub fn last_error(&self) -> Option<&str> { self.last_error.as_deref() }

    /// Retire the atlas allocation, including the window currently being updated.
    /// Call on close and before unmounting a cache while its window remains alive.
    pub fn clear(&mut self, window: &mut Window, cx: &mut App) {
        if let Some(image) = self.image.take() { cx.drop_image(image, Some(window)); }
        self.key = None;
        self.failed_key = None;
        self.last_error = None;
        self.raster_size = None;
    }

    /// Capture and paint in the same paint phase, before modal children paint.
    /// Modal input and animation ticks must not be part of `key`.
    pub fn paint(
        &mut self, key: ModalBackdropKey, bounds: Bounds<Pixels>,
        window: &mut Window, cx: &mut App,
        capture: impl FnOnce(&mut Window) -> Result<image::RgbaImage>,
    ) -> Result<()> {
        ensure!(key.valid(), "Invalid modal backdrop dimensions or scale");
        if self.key != Some(key) && self.failed_key != Some(key) {
            match capture(window).and_then(|image| prepare(image, window.scale_factor())) {
                Ok((image, dimensions)) => {
                    // Prepare first: a failed readback must not destroy the last
                    // usable backdrop. Retire the old atlas entry on replacement.
                    if let Some(old) = self.image.replace(image) { cx.drop_image(old, Some(window)); }
                    self.key = Some(key);
                    self.failed_key = None;
                    self.last_error = None;
                    self.raster_size = Some(dimensions);
                    self.captures = self.captures.wrapping_add(1);
                }
                Err(error) => {
                    // A lost/unsupported renderer is not retried on every modal
                    // keystroke. A new geometry/content key allows a fresh attempt.
                    self.failed_key = Some(key);
                    self.last_error = Some(format!("{error:#}"));
                    return Err(error);
                }
            }
        }
        if let Some(image) = self.image.clone() {
            window.paint_image(bounds, bounds, Corners::default(), image, 0, false)?;
        }
        Ok(())
    }
}

fn prepare(image: image::RgbaImage, scale: f32) -> Result<(Arc<RenderImage>, [u32; 2])> {
    ensure!(image.width() > 0 && image.height() > 0, "Empty modal backdrop capture");
    ensure!(scale.is_finite() && scale > 0., "Invalid modal backdrop scale");
    let dimensions = [image.width(), image.height()];
    let mut blurred = image::imageops::blur(&image, 8. * scale);
    // Preserve the existing native readback -> RenderImage BGRA convention.
    for pixel in blurred.pixels_mut() { pixel.0.swap(0, 2); }
    Ok((Arc::new(RenderImage::new(vec![image::Frame::new(blurred)])), dimensions))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{px, size};
    #[test]
    fn geometry_scale_and_structural_content_are_distinct_keys() {
        let initial = ModalBackdropKey::new(size(px(1280.), px(800.)), 2., 9);
        assert_eq!(initial, ModalBackdropKey::new(size(px(1280.), px(800.)), 2., 9));
        assert_ne!(initial, ModalBackdropKey::new(size(px(1060.), px(640.)), 2., 9));
        assert_ne!(initial, ModalBackdropKey::new(size(px(1280.), px(800.)), 1., 9));
        assert_ne!(initial, ModalBackdropKey::new(size(px(1280.), px(800.)), 2., 10));
        assert!(!ModalBackdropKey::new(size(px(1280.), px(800.)), f32::NAN, 9).valid());
    }
    #[test]
    fn capture_preparation_preserves_dimensions_and_existing_channel_conversion() {
        let raw = image::RgbaImage::from_pixel(12, 8, image::Rgba([200, 50, 20, 255]));
        let (image, dimensions) = prepare(raw, 2.).unwrap();
        assert_eq!(dimensions, [12, 8]);
        assert_eq!(&image.as_bytes(0).unwrap()[..4], &[20, 50, 200, 255]);
        assert!(prepare(image::RgbaImage::new(0, 1), 2.).is_err());
    }
}
