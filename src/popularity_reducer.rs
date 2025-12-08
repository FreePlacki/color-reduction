use std::collections::HashMap;

use crate::reducer::Reducer;

pub struct PopularityReducer {
    n: usize,
}

impl PopularityReducer {
    pub fn new(num_colors: usize) -> Self {
        Self { n: num_colors }
    }
}

impl Reducer for PopularityReducer {
    fn reduce(&self, rgba: &[u8], width: usize, height: usize) -> Vec<u8> {
        let img: Vec<_> = rgba
            .chunks_exact(4)
            .map(|p| [p[0], p[1], p[2], p[3]])
            .collect();
        let palette = Palette::from_image_rgba(&img, self.n);

        let mut buf: Vec<[f32; 3]> = rgba
            .chunks_exact(4)
            .map(|px| [px[0] as f32, px[1] as f32, px[2] as f32])
            .collect();

        // No error diffusion version (pure popularity quantization)
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;

                let old = buf[idx];
                let new = palette.nearest_color(old);

                buf[idx] = [new[0] as f32, new[1] as f32, new[2] as f32];
            }
        }

        // Convert to RGBA
        let mut out = vec![0u8; width * height * 4];
        for (i, px) in buf.iter().enumerate() {
            out[i * 4 + 0] = px[0].clamp(0.0, 255.0) as u8;
            out[i * 4 + 1] = px[1].clamp(0.0, 255.0) as u8;
            out[i * 4 + 2] = px[2].clamp(0.0, 255.0) as u8;
            out[i * 4 + 3] = 255;
        }

        out
    }
}

struct Palette {
    pub colors: Vec<[u8; 3]>,
}

impl Palette {
    pub fn from_image_rgba(image: &[[u8; 4]], k: usize) -> Self {
        let mut freq: HashMap<[u8; 3], u32> = HashMap::new();

        for px in image {
            let rgb = [px[0], px[1], px[2]];
            *freq.entry(rgb).or_insert(0) += 1;
        }

        let mut v: Vec<([u8; 3], u32)> = freq.into_iter().collect();
        v.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        let colors = v.into_iter().take(k).map(|x| x.0).collect();

        Self { colors }
    }

    pub fn nearest_color(&self, px: [f32; 3]) -> [u8; 3] {
        let mut best = self.colors[0];
        let mut best_d = u32::MAX;

        for &c in &self.colors {
            let dr = c[0] as i32 - px[0] as i32;
            let dg = c[1] as i32 - px[1] as i32;
            let db = c[2] as i32 - px[2] as i32;
            let d = (dr * dr + dg * dg + db * db) as u32;
            if d < best_d {
                best_d = d;
                best = c;
            }
        }
        best
    }
}
