use num::{Complex, Zero};

use crate::Float;

pub fn get_iters(c: Complex<Float>, max_iter: usize) -> usize {
    let mut z: Complex<Float> = Complex::zero();

    for i in 0..max_iter {
        if z.norm() >= 2.0 {
            return i;
        }

        z = z * z + c;
    }

    max_iter
}
