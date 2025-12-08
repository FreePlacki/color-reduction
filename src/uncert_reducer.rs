use crate::reducer::Reducer;

pub struct UncertReducer {
    palette: Palette,
    matrix: DiffusionMatrix,
}

impl Reducer for UncertReducer {
    fn reduce(&self, rgba: &[u8], width: usize, height: usize) -> Vec<u8> {
        let mut buf: Vec<[f32; 3]> = rgba
            .chunks_exact(4)
            .map(|px| [px[0] as f32, px[1] as f32, px[2] as f32])
            .collect();

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;

                let old = buf[idx];
                let new = self.palette.nearest_color(old);
                let err = [
                    old[0] - new[0] as f32,
                    old[1] - new[1] as f32,
                    old[2] - new[2] as f32,
                ];

                buf[idx] = [new[0] as f32, new[1] as f32, new[2] as f32];

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

        // Convert back into RGBA8
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
    pub fn with_uniform_palette(num_colors: usize, matrix: DiffusionMatrix) -> Self {
        Self {
            palette: Palette::Uniform(num_colors),
            matrix,
        }
    }
}

pub enum Palette {
    Uniform(usize),
}

impl Palette {
    pub fn nearest_color(&self, px: [f32; 3]) -> [u8; 3] {
        match *self {
            Palette::Uniform(n) => {
                let levels = (n as f32).cbrt().ceil() as usize;
                let maxv = (levels - 1) as f32;

                let rf = (px[0] / 255.0) * maxv;
                let gf = (px[1] / 255.0) * maxv;
                let bf = (px[2] / 255.0) * maxv;

                let r_i = rf.round().clamp(0.0, maxv);
                let g_i = gf.round().clamp(0.0, maxv);
                let b_i = bf.round().clamp(0.0, maxv);

                let r = (r_i / maxv * 255.0) as u8;
                let g = (g_i / maxv * 255.0) as u8;
                let b = (b_i / maxv * 255.0) as u8;

                [r, g, b]
            }
        }
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
