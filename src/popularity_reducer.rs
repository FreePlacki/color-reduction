use std::cell::{Cell, RefCell};

use crate::reducer::Reducer;

const BITS: usize = 5;
const BINS: usize = 1 << BITS;
const BIN_COUNT: usize = BINS * BINS * BINS;

const LUT_RES: usize = 32;
const LUT_SIZE: usize = LUT_RES * LUT_RES * LUT_RES;

#[derive(Clone, Copy, Default)]
struct Bin {
    count: u32,
    sum: [u32; 3],
}

pub struct PopularityReducer {
    bins: Vec<Bin>,

    lut: RefCell<Vec<[u8; 3]>>,
    lut_colors: Cell<usize>,
}

impl PopularityReducer {
    pub fn new(rgba: &[u8]) -> Self {
        let mut bins = vec![Bin::default(); BIN_COUNT];

        for px in rgba.chunks_exact(4) {
            let r = px[0] as usize;
            let g = px[1] as usize;
            let b = px[2] as usize;

            let idx = bin_index(r, g, b);
            let bin = &mut bins[idx];

            bin.count += 1;
            bin.sum[0] += r as u32;
            bin.sum[1] += g as u32;
            bin.sum[2] += b as u32;
        }

        Self {
            bins,
            lut: RefCell::new(Vec::new()),
            lut_colors: Cell::new(0),
        }
    }

    fn build_palette(&self, k: usize) -> Vec<[u8; 3]> {
        let mut used: Vec<&Bin> =
            self.bins.iter().filter(|b| b.count > 0).collect();

        used.sort_unstable_by(|a, b| b.count.cmp(&a.count));

        used.into_iter()
            .take(k)
            .map(|b| [
                (b.sum[0] / b.count) as u8,
                (b.sum[1] / b.count) as u8,
                (b.sum[2] / b.count) as u8,
            ])
            .collect()
    }

    fn ensure_lut(&self, num_colors: usize) {
        if self.lut_colors.get() == num_colors {
            return;
        }

        let palette = self.build_palette(num_colors);
        let mut lut = vec![[0u8; 3]; LUT_SIZE];

        for r in 0..LUT_RES {
            for g in 0..LUT_RES {
                for b in 0..LUT_RES {
                    let px = [
                        (r * 255 / (LUT_RES - 1)) as i32,
                        (g * 255 / (LUT_RES - 1)) as i32,
                        (b * 255 / (LUT_RES - 1)) as i32,
                    ];

                    let mut best = palette[0];
                    let mut best_d = i32::MAX;

                    for &c in &palette {
                        let dr = c[0] as i32 - px[0];
                        let dg = c[1] as i32 - px[1];
                        let db = c[2] as i32 - px[2];
                        let d = dr * dr + dg * dg + db * db;
                        if d < best_d {
                            best_d = d;
                            best = c;
                        }
                    }

                    let idx = (r * LUT_RES + g) * LUT_RES + b;
                    lut[idx] = best;
                }
            }
        }

        *self.lut.borrow_mut() = lut;
        self.lut_colors.set(num_colors);
    }

    #[inline]
    fn lookup(&self, px: [u8; 3]) -> [u8; 3] {
        let r = px[0] as usize * (LUT_RES - 1) / 255;
        let g = px[1] as usize * (LUT_RES - 1) / 255;
        let b = px[2] as usize * (LUT_RES - 1) / 255;

        self.lut.borrow()[(r * LUT_RES + g) * LUT_RES + b]
    }
}

impl Reducer for PopularityReducer {
    fn reduce(
        &self,
        rgba: &[u8],
        width: usize,
        height: usize,
        num_colors: usize,
    ) -> Vec<u8> {
        self.ensure_lut(num_colors);

        let mut out = vec![0u8; width * height * 4];

        for (i, px) in rgba.chunks_exact(4).enumerate() {
            let q = self.lookup([px[0], px[1], px[2]]);
            out[i * 4 + 0] = q[0];
            out[i * 4 + 1] = q[1];
            out[i * 4 + 2] = q[2];
            out[i * 4 + 3] = 255;
        }

        out
    }
}

#[inline]
fn bin_index(r: usize, g: usize, b: usize) -> usize {
    let shift = 8 - BITS;
    let ri = r >> shift;
    let gi = g >> shift;
    let bi = b >> shift;

    (ri * BINS + gi) * BINS + bi
}

