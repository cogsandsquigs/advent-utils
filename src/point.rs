use core::fmt::{Display, Formatter};

/// A point in a 2D grid.
/// TODO: Change points to be i64 instead of usize.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

/// Public API for Point.
impl Point {
    /// Returns a new `Point`.
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    /// Computes the Manhattan distance between two points.
    pub fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Computes the set of all integer-valued points between two points.
    pub fn line(&self, other: &Self) -> Vec<Self> {
        let mut points = vec![];

        let x_diff = (other.x - self.x).unsigned_abs();
        let y_diff = (other.y - self.y).unsigned_abs();

        let x_step = if self.x < other.x { 1 } else { -1 };
        let y_step = if self.y < other.y { 1 } else { -1 };

        for i in 0..=x_diff {
            for j in 0..=y_diff {
                points.push(Self::new(
                    self.x + i as i64 * x_step,
                    self.y + j as i64 * y_step,
                ));
            }
        }

        points
    }

    /// Returns the set of all neighbors of the given coordinates. The neighbors are returned in the
    /// order of left, right, up, down, top-left, top-right, bottom-left, bottom-right.
    pub fn neighbors(&self) -> Vec<Self> {
        [self.orthogonal_neighbors(), self.diagonal_neighbors()].concat()
    }

    /// Returns the set of all neighbors orthogonal to the given coordinates.
    /// The neighbors are returned in the order of left, right, up, down.
    pub fn orthogonal_neighbors(&self) -> Vec<Self> {
        vec![
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y - 1),
            Self::new(self.x, self.y + 1),
        ]
    }

    /// Returns the set of all neighbors diagonal to the given coordinates.
    /// The neighbors are returned in the order of top-left, top-right, bottom-left, bottom-right.
    pub fn diagonal_neighbors(&self) -> Vec<Self> {
        vec![
            Self::new(self.x - 1, self.y - 1),
            Self::new(self.x + 1, self.y - 1),
            Self::new(self.x - 1, self.y + 1),
            Self::new(self.x + 1, self.y + 1),
        ]
    }
}

impl From<(i64, i64)> for Point {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

impl From<Point> for (i64, i64) {
    fn from(point: Point) -> Self {
        (point.x, point.y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
