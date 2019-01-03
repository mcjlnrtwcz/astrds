use ggez::*;

pub fn rects_touching_horizontally(first: graphics::Rect, second: graphics::Rect) -> bool {
    let first_middle = first.x + first.w / 2.0;
    let second_middle = second.x + second.w / 2.0;
    let diff = (first_middle - second_middle).abs();
    let max_diff = first.w / 2.0 + second.w / 2.0;
    diff <= max_diff
}
