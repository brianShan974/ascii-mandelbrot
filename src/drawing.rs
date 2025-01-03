use std::cmp::min;

use num::Complex;

use crate::math::get_iters;
use crate::Float;
use crate::SYMBOLS;

fn get_symbol(iters: usize, max_iter: usize) -> char {
    let num_symbols = SYMBOLS.len();

    let step_width = max_iter / num_symbols;
    let index = iters / step_width;

    SYMBOLS[min(index, num_symbols - 1)]
}

pub fn draw(
    max_iter: usize,
    x_min: Float,
    x_max: Float,
    y_min: Float,
    y_max: Float,
    t_width: usize,
    lines: Option<usize>,
) {
    let lines = if let Some(lines) = lines {
        lines
    } else {
        t_width
    };

    let x_step_length = (x_max - x_min) / (t_width - 1) as Float;
    let y_step_length = (y_max - y_min) / (lines - 1) as Float;

    for y_step in 0..lines {
        for x_step in 0..t_width {
            let z = Complex::new(
                x_min + x_step as Float * x_step_length,
                y_max - y_step as Float * y_step_length,
            );

            let iters = get_iters(z, max_iter);

            print!("{}", get_symbol(iters, max_iter));
        }
        println!();
    }
}
