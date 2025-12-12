use crate::reducer::Reducer;

pub struct UncertReducer {
    matrix: DiffusionMatrix,
}

impl Reducer for UncertReducer {
    fn reduce(&self, rgba: &[u8], width: usize, height: usize, num_colors: usize) -> Vec<u8> {
        let palette = Palette::new(num_colors);

        let mut buf: Vec<[f32; 3]> = rgba
            .chunks_exact(4)
            .map(|px| [px[0] as f32, px[1] as f32, px[2] as f32])
            .collect();

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;

                let mut old = buf[idx];
                old[0] = old[0].clamp(0.0, 255.0);
                old[1] = old[1].clamp(0.0, 255.0);
                old[2] = old[2].clamp(0.0, 255.0);

                let new = palette.nearest_color(old);

                let err = [old[0] - new[0], old[1] - new[1], old[2] - new[2]];

                buf[idx] = new;

                for &(dx, dy, weight) in self.matrix.matrix() {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                        let i2 = ny as usize * width + nx as usize;
                        buf[i2][0] += err[0] * weight;
                        buf[i2][1] += err[1] * weight;
                        buf[i2][2] += err[2] * weight;
                    }
                }
            }
        }

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

impl UncertReducer {
    pub fn with_uniform_palette(matrix: DiffusionMatrix) -> Self {
        Self { matrix }
    }
}

pub struct Palette {
    div_r: usize,
    div_g: usize,
    div_b: usize,
}

impl Palette {
    pub fn new(n: usize) -> Self {
        let mut div_r = 1;
        let mut div_g = 1;
        let mut div_b = 1;

        let mut k = n;
        while k > 1 {
            if div_r <= div_g && div_r <= div_b {
                div_r += 1;
            } else if div_g <= div_b {
                div_g += 1;
            } else {
                div_b += 1;
            }
            k -= 1;
        }

        Palette {
            div_r,
            div_g,
            div_b,
        }
    }

    fn center_of(&self, ri: usize, gi: usize, bi: usize) -> [f32; 3] {
        [
            (ri as f32 + 0.5) / self.div_r as f32 * 255.0,
            (gi as f32 + 0.5) / self.div_g as f32 * 255.0,
            (bi as f32 + 0.5) / self.div_b as f32 * 255.0,
        ]
    }

    pub fn nearest_color(&self, px: [f32; 3]) -> [f32; 3] {
        let r = px[0].clamp(0.0, 255.0) / 255.0;
        let g = px[1].clamp(0.0, 255.0) / 255.0;
        let b = px[2].clamp(0.0, 255.0) / 255.0;

        let ri = ((r * self.div_r as f32) - 0.5)
            .round()
            .clamp(0.0, (self.div_r - 1) as f32) as usize;

        let gi = ((g * self.div_g as f32) - 0.5)
            .round()
            .clamp(0.0, (self.div_g - 1) as f32) as usize;

        let bi = ((b * self.div_b as f32) - 0.5)
            .round()
            .clamp(0.0, (self.div_b - 1) as f32) as usize;

        self.center_of(ri, gi, bi)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DiffusionMatrix {
    FloydSteinberg,
    Burkes,
    Stucky,
}

impl Default for DiffusionMatrix {
    fn default() -> Self {
        Self::FloydSteinberg
    }
}

impl DiffusionMatrix {
    pub const fn matrix(&self) -> &'static [(i32, i32, f32)] {
        match self {
            DiffusionMatrix::FloydSteinberg => &[
                (1, 0, 7.0 / 16.0),
                (-1, 1, 3.0 / 16.0),
                (0, 1, 5.0 / 16.0),
                (1, 1, 1.0 / 16.0),
            ],
            DiffusionMatrix::Burkes => &[
                (1, 0, 8.0 / 32.0),
                (2, 0, 4.0 / 32.0),
                (-2, 1, 2.0 / 32.0),
                (-1, 1, 4.0 / 32.0),
                (0, 1, 8.0 / 32.0),
                (1, 1, 4.0 / 32.0),
                (2, 1, 2.0 / 32.0),
            ],
            DiffusionMatrix::Stucky => &[
                (1, 0, 8.0 / 42.0),
                (2, 0, 4.0 / 42.0),
                (-2, 1, 2.0 / 42.0),
                (-1, 1, 4.0 / 42.0),
                (0, 1, 8.0 / 42.0),
                (1, 1, 4.0 / 42.0),
                (2, 1, 2.0 / 42.0),
                (-2, 2, 1.0 / 42.0),
                (-1, 2, 2.0 / 42.0),
                (0, 2, 4.0 / 42.0),
                (1, 2, 2.0 / 42.0),
                (2, 2, 1.0 / 42.0),
            ],
        }
    }
}
