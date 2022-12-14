pub type Point = (usize, usize);

/// A matrix of values. The matrix is stored in row-major order.
pub struct Matrix<T> {
    /// The width of the matrix.
    pub width: usize,
    /// The height of the matrix.
    pub height: usize,
    /// The values of the matrix.
    pub values: Vec<T>,
}

/// Public API for Matrix.
impl<T> Matrix<T> {
    /// Returns the value at the given coordinates.
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.values[y * self.width + x]
    }

    /// Returns a mutable reference to the value at the given coordinates.
    pub fn get_mut(&mut self, (x, y): Point) -> &mut T {
        &mut self.values[y * self.width + x]
    }

    /// Returns the value at the given coordinates.
    pub fn set(&mut self, (x, y): Point, value: T) {
        self.values[y * self.width + x] = value;
    }

    /// Returns the set of all neighbors of the given coordinates. The neighbors are returned in the
    /// order of left, right, up, down, top-left, top-right, bottom-left, bottom-right.
    pub fn neighbors(&self, (x, y): Point) -> Vec<Point> {
        [
            self.orthogonal_neighbors((x, y)),
            self.diagonal_neighbors((x, y)),
        ]
        .concat()
    }

    /// Returns the set of all neighbors orthogonal to the given coordinates.
    /// The neighbors are returned in the order of left, right, up, down.
    pub fn orthogonal_neighbors(&self, (x, y): Point) -> Vec<Point> {
        let mut neighbors: Vec<Point> = vec![];

        if x > 0 {
            neighbors.push((x - 1, y));
        }

        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }

        if y > 0 {
            neighbors.push((x, y - 1));
        }

        if y < self.height - 1 {
            neighbors.push((x, y + 1));
        }

        neighbors
    }

    /// Returns the set of all neighbors diagonal to the given coordinates.
    /// The neighbors are returned in the order of top-left, top-right, bottom-left, bottom-right.
    pub fn diagonal_neighbors(&self, (x, y): Point) -> Vec<Point> {
        let mut neighbors: Vec<Point> = vec![];

        if x > 0 && y > 0 {
            neighbors.push((x - 1, y - 1));
        }

        if x < self.width - 1 && y > 0 {
            neighbors.push((x + 1, y - 1));
        }

        if x > 0 && y < self.height - 1 {
            neighbors.push((x - 1, y + 1));
        }

        if x < self.width - 1 && y < self.height - 1 {
            neighbors.push((x + 1, y + 1));
        }

        neighbors
    }
}
