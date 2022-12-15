/// A point in a 2D grid.
/// TODO: Change points to be i32 instead of usize.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Public API for Point.
impl Point {
    /// Returns a new `Point`.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Computes the Manhattan distance between two points.
    pub fn manhattan_distance(&self, other: &Self) -> u32 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
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
                    (self.x + i as i32 * x_step),
                    (self.y + j as i32 * y_step),
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
        let mut neighbors: Vec<Self> = vec![];

        neighbors.push(Self::new(self.x - 1, self.y));

        neighbors.push(Self::new(self.x + 1, self.y));

        neighbors.push(Self::new(self.x, self.y - 1));

        neighbors.push(Self::new(self.x, self.y + 1));

        neighbors
    }

    /// Returns the set of all neighbors diagonal to the given coordinates.
    /// The neighbors are returned in the order of top-left, top-right, bottom-left, bottom-right.
    pub fn diagonal_neighbors(&self) -> Vec<Self> {
        let mut neighbors: Vec<Self> = vec![];

        neighbors.push(Self::new(self.x - 1, self.y - 1));

        neighbors.push(Self::new(self.x + 1, self.y - 1));

        neighbors.push(Self::new(self.x - 1, self.y + 1));

        neighbors.push(Self::new(self.x + 1, self.y + 1));

        neighbors
    }
}
