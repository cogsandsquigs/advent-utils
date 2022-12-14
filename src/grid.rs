use crate::point::Point;

/// A matrix of values. The matrix is stored in row-major order.
pub struct Grid<T> {
    /// The width of the matrix.
    width: usize,
    /// The height of the matrix.
    height: usize,
    /// The values of the matrix.
    values: Vec<T>,
}

/// Public API for Grid.
impl<T> Grid<T> {
    /// Returns the value at the given coordinates.
    pub fn get(&self, point: Point) -> &T {
        &self.values[point.y * self.width + point.x]
    }

    /// Returns a mutable reference to the value at the given coordinates.
    pub fn get_mut(&mut self, point: Point) -> &mut T {
        &mut self.values[point.y * self.width + point.x]
    }

    /// Returns the value at the given coordinates.
    pub fn set(&mut self, point: Point, value: T) {
        self.values[point.y * self.width + point.x] = value;
    }

    /// Returns the set of all neighbors of the given coordinates. The neighbors are returned in the
    /// order of left, right, up, down, top-left, top-right, bottom-left, bottom-right.
    pub fn neighbors(&self, point: Point) -> Vec<Point> {
        [
            self.orthogonal_neighbors(point),
            self.diagonal_neighbors(point),
        ]
        .concat()
    }

    /// Returns the set of all neighbors orthogonal to the given coordinates.
    /// The neighbors are returned in the order of left, right, up, down.
    pub fn orthogonal_neighbors(&self, point: Point) -> Vec<Point> {
        let mut neighbors: Vec<Point> = vec![];

        if point.x > 0 {
            neighbors.push(Point::new(point.x - 1, point.y));
        }

        if point.x < self.width - 1 {
            neighbors.push(Point::new(point.x + 1, point.y));
        }

        if point.y > 0 {
            neighbors.push(Point::new(point.x, point.y - 1));
        }

        if point.y < self.height - 1 {
            neighbors.push(Point::new(point.x, point.y + 1));
        }

        neighbors
    }

    /// Returns the set of all neighbors diagonal to the given coordinates.
    /// The neighbors are returned in the order of top-left, top-right, bottom-left, bottom-right.
    pub fn diagonal_neighbors(&self, point: Point) -> Vec<Point> {
        let mut neighbors: Vec<Point> = vec![];

        if point.x > 0 && point.y > 0 {
            neighbors.push(Point::new(point.x - 1, point.y - 1));
        }

        if point.x < self.width - 1 && point.y > 0 {
            neighbors.push(Point::new(point.x + 1, point.y - 1));
        }

        if point.x > 0 && point.y < self.height - 1 {
            neighbors.push(Point::new(point.x - 1, point.y + 1));
        }

        if point.x < self.width - 1 && point.y < self.height - 1 {
            neighbors.push(Point::new(point.x + 1, point.y + 1));
        }

        neighbors
    }
}
