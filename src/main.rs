mod drawing;
mod math;

use drawing::draw;

type Float = f32;

const SYMBOLS: &[char] = &[' ', '.', '·', '*', '+', 'x', '$', '#', '%'];
const MAX_ITER: usize = 1000;

fn main() {
    let size = termsize::get().unwrap();

    let cols = size.cols as usize;
    let rows = size.rows as usize;

    let x_min = -2.0;
    let x_max = 1.0;

    let y_min = -0.9;
    let y_max = -y_min;

    draw(MAX_ITER, x_min, x_max, y_min, y_max, cols, Some(rows));
}
