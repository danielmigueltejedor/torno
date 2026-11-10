//! Torno app icon, shared by desktop, packaging and CLI: a portrait of Henry Maudslay over the
//! app colour, in a rounded tile. The picture is `torno-portrait-512.rgba`, and every size is
//! resampled from it, so the window, the packages and the CLI always show the same image.

/// Side of the source picture, in pixels.
const SOURCE_SIZE: usize = 512;

/// The source picture: rows from top to bottom, four bytes per pixel (red, green, blue and
/// straight alpha). Pixels outside the rounded tile are fully transparent.
const SOURCE: &[u8] = include_bytes!("torno-portrait-512.rgba");

/// This block is the icon image data
pub fn rgba(size: usize) -> Result<Vec<u8>, String> {
    if !(1..=2048).contains(&size) {
        return Err("icon size must be 1..=2048".into());
    }
    if SOURCE.len() != SOURCE_SIZE * SOURCE_SIZE * 4 {
        return Err("icon source is not 512 x 512 RGBA".into());
    }
    // Each output pixel is the area average of the source pixels it covers, weighted by how
    // much of each it covers and by alpha, so the rounded corners stay clean at small sizes.
    let scale = SOURCE_SIZE as f64 / size as f64;
    let mut rgba = Vec::with_capacity(size * size * 4);
    for y in 0..size {
        let (top, bottom) = (y as f64 * scale, (y + 1) as f64 * scale);
        for x in 0..size {
            let (left, right) = (x as f64 * scale, (x + 1) as f64 * scale);
            let (mut colour, mut alpha, mut area) = ([0f64; 3], 0f64, 0f64);
            for sy in (top.floor() as usize)..(bottom.ceil() as usize).min(SOURCE_SIZE) {
                let height = bottom.min(sy as f64 + 1.0) - top.max(sy as f64);
                for sx in (left.floor() as usize)..(right.ceil() as usize).min(SOURCE_SIZE) {
                    let weight = height * (right.min(sx as f64 + 1.0) - left.max(sx as f64));
                    if weight <= 0.0 {
                        continue;
                    }
                    let at = (sy * SOURCE_SIZE + sx) * 4;
                    let a = f64::from(SOURCE[at + 3]) / 255.0;
                    for k in 0..3 {
                        colour[k] += f64::from(SOURCE[at + k]) * a * weight;
                    }
                    alpha += a * weight;
                    area += weight;
                }
            }
            if alpha <= 0.0 || area <= 0.0 {
                rgba.extend_from_slice(&[0, 0, 0, 0]);
            } else {
                let channel = |k: usize| (colour[k] / alpha).round().clamp(0.0, 255.0) as u8;
                rgba.extend_from_slice(&[channel(0), channel(1), channel(2), (alpha / area * 255.0).round().clamp(0.0, 255.0) as u8]);
            }
        }
    }
    Ok(rgba)
}

#[cfg(test)]
mod tests {
    #[test]
    fn icon_is_a_rounded_tile_at_window_and_package_sizes() {
        for size in [16, 24, 32, 48, 64, 128, 256, 512] {
            let pixels = super::rgba(size).expect("valid size");
            assert_eq!(pixels.len(), size * size * 4);
            assert_eq!(pixels[3], 0, "the corner of the tile is transparent");
            let centre = (size / 2 * size + size / 2) * 4;
            assert_eq!(pixels[centre + 3], 255, "the centre of the tile is opaque");
        }
        assert!(super::rgba(0).is_err());
        assert!(super::rgba(2049).is_err());
    }

    /// At its own size the icon is the source picture, byte for byte.
    #[test]
    fn full_size_icon_is_the_source_picture() {
        assert_eq!(super::rgba(super::SOURCE_SIZE).expect("valid size"), super::SOURCE);
    }
}
