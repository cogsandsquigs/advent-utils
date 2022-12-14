#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

/// Public API for Point.
impl Point {
    /// Returns a new `Point`.
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Computes the Manhattan distance between two points.
    pub fn manhattan_distance(&self, other: &Self) -> usize {
        (self.x as isize - other.x as isize).unsigned_abs()
            + (self.y as isize - other.y as isize).unsigned_abs()
    }

    /// Computes the set of all integer-valued points between two points.
    pub fn line(&self, other: &Self) -> Vec<Self> {
        let mut points = vec![];

        let x_diff = (other.x as isize - self.x as isize).unsigned_abs();
        let y_diff = (other.y as isize - self.y as isize).unsigned_abs();

        let x_step = if self.x < other.x { 1 } else { -1 };
        let y_step = if self.y < other.y { 1 } else { -1 };

        for i in 0..=x_diff {
            for j in 0..=y_diff {
                points.push(Self::new(
                    (self.x as isize + i as isize * x_step) as usize,
                    (self.y as isize + j as isize * y_step) as usize,
                ));
            }
        }

        points
    }
}
