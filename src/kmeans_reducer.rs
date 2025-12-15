use crate::reducer::Reducer;

use std::cell::RefCell;

const LUT_RES: usize = 32;
const LUT_SIZE: usize = LUT_RES * LUT_RES * LUT_RES;
const MAX_ITERS: usize = 50;

pub struct KMeansReducer {
    pixels: Vec<[f32; 3]>,
    epsilon: f32,

    centers: RefCell<Vec<[f32; 3]>>,
    lut: RefCell<Vec<[u8; 3]>>,
}

impl KMeansReducer {
    pub fn new(rgba: &[u8], epsilon: f32) -> Self {
        let pixels = rgba
            .chunks_exact(4)
            .map(|p| [p[0] as f32, p[1] as f32, p[2] as f32])
            .collect();

        Self {
            pixels,
            epsilon,
            centers: RefCell::new(Vec::new()),
            lut: RefCell::new(Vec::new()),
        }
    }

    fn run_kmeans(&self, k: usize) {
        let mut centers = self.init_centers(k);
        let mut assignments = vec![0usize; self.pixels.len()];

        let eps2 = self.epsilon * self.epsilon;

        for _ in 0..MAX_ITERS {
            let mut sums = vec![[0.0; 3]; k];
            let mut counts = vec![0u32; k];

            // assignment
            for (i, &px) in self.pixels.iter().enumerate() {
                let mut best = 0;
                let mut best_d = f32::MAX;

                for (ci, c) in centers.iter().enumerate() {
                    let d = dist2(px, *c);
                    if d < best_d {
                        best_d = d;
                        best = ci;
                    }
                }

                assignments[i] = best;
                sums[best][0] += px[0];
                sums[best][1] += px[1];
                sums[best][2] += px[2];
                counts[best] += 1;
            }

            // update + convergence check
            let mut max_move = 0.0f32;

            for i in 0..k {
                if counts[i] == 0 {
                    continue;
                }

                let new = [
                    sums[i][0] / counts[i] as f32,
                    sums[i][1] / counts[i] as f32,
                    sums[i][2] / counts[i] as f32,
                ];

                let d = dist2(new, centers[i]);
                max_move = max_move.max(d);
                centers[i] = new;
            }

            if max_move < eps2 {
                break;
            }
        }

        *self.centers.borrow_mut() = centers;
        self.build_lut();
    }

    fn init_centers(&self, k: usize) -> Vec<[f32; 3]> {
        let step = self.pixels.len() / k.max(1);
        (0..k)
            .map(|i| self.pixels[(i * step) % self.pixels.len()])
            .collect()
    }

    fn build_lut(&self) {
        let centers = self.centers.borrow();
        let mut lut = vec![[0u8; 3]; LUT_SIZE];

        for r in 0..LUT_RES {
            for g in 0..LUT_RES {
                for b in 0..LUT_RES {
                    let px = [
                        (r * 255 / (LUT_RES - 1)) as f32,
                        (g * 255 / (LUT_RES - 1)) as f32,
                        (b * 255 / (LUT_RES - 1)) as f32,
                    ];

                    let mut best = centers[0];
                    let mut best_d = f32::MAX;

                    for &c in centers.iter() {
                        let d = dist2(px, c);
                        if d < best_d {
                            best_d = d;
                            best = c;
                        }
                    }

                    let idx = (r * LUT_RES + g) * LUT_RES + b;
                    lut[idx] = [
                        best[0].clamp(0.0, 255.0) as u8,
                        best[1].clamp(0.0, 255.0) as u8,
                        best[2].clamp(0.0, 255.0) as u8,
                    ];
                }
            }
        }

        *self.lut.borrow_mut() = lut;
    }

    #[inline]
    fn lookup(&self, px: [u8; 3]) -> [u8; 3] {
        let r = px[0] as usize * (LUT_RES - 1) / 255;
        let g = px[1] as usize * (LUT_RES - 1) / 255;
        let b = px[2] as usize * (LUT_RES - 1) / 255;

        self.lut.borrow()[(r * LUT_RES + g) * LUT_RES + b]
    }
}

impl Reducer for KMeansReducer {
    fn reduce(
        &self,
        rgba: &[u8],
        width: usize,
        height: usize,
        num_colors: usize,
    ) -> Vec<u8> {
        self.run_kmeans(num_colors);

        let mut out = vec![0u8; width * height * 4];

        for (i, px) in rgba.chunks_exact(4).enumerate() {
            let q = self.lookup([px[0], px[1], px[2]]);
            out[i * 4] = q[0];
            out[i * 4 + 1] = q[1];
            out[i * 4 + 2] = q[2];
            out[i * 4 + 3] = 255;
        }

        out
    }
}

#[inline]
fn dist2(a: [f32; 3], b: [f32; 3]) -> f32 {
    let dr = a[0] - b[0];
    let dg = a[1] - b[1];
    let db = a[2] - b[2];
    dr * dr + dg * dg + db * db
}

