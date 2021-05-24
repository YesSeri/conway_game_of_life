/// Used when keeping track of specific points, for example when keeping track of which values to flip. 
/// The board itself doesn't use this, but instead just uses a nested 2d vector.
#[derive(Debug, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}
