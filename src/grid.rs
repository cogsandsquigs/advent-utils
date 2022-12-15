use crate::point::Point;
use std::ops::{Index, IndexMut};

/// A matrix of values. The matrix is stored in row-major order.
pub struct Grid<T> {
    /// The width of the matrix.
    pub width: usize,
    /// The height of the matrix.
    pub height: usize,
    /// The values of the matrix.
    values: Vec<T>,
}

/// Public API for Grid.
impl<T> Grid<T> {
    /// Returns a new `Grid` with the given width and height. The values of the matrix are
    /// initialized with the default value of `T`.
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            width,
            height,
            values: vec![T::default(); width * height + 1],
        }
    }

    /// Returns a new `Grid` with the given width and height. The values of the matrix are
    /// initialized with the given value.
    pub fn new_with_value(width: usize, height: usize, value: T) -> Self
    where
        T: Clone,
    {
        Self {
            width,
            height,
            values: vec![value; width * height],
        }
    }

    /// Returns the value at the given coordinates.
    pub fn get(&self, point: Point) -> &T {
        &self.values[point.y as usize * self.width + point.x as usize]
    }

    /// Returns a mutable reference to the value at the given coordinates.
    pub fn get_mut(&mut self, point: Point) -> &mut T {
        &mut self.values[point.y as usize * self.width + point.x as usize]
    }

    /// Returns the value at the given coordinates.
    pub fn set(&mut self, point: Point, value: T) {
        self.values[point.y as usize * self.width + point.x as usize] = value;
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
    /// TODO: Change points to be i32 instead of usize.
    pub fn orthogonal_neighbors(&self, point: Point) -> Vec<Point> {
        #[allow(clippy::absurd_extreme_comparisons)]
        #[allow(unused_comparisons)]
        point
            .orthogonal_neighbors()
            .into_iter()
            .filter(|p| p.x >= 0 && p.x < self.width as i32 && p.y >= 0 && p.y < self.height as i32)
            .collect()
    }

    /// Returns the set of all neighbors diagonal to the given coordinates.
    /// The neighbors are returned in the order of top-left, top-right, bottom-left, bottom-right.
    pub fn diagonal_neighbors(&self, point: Point) -> Vec<Point> {
        #[allow(clippy::absurd_extreme_comparisons)]
        #[allow(unused_comparisons)]
        point
            .diagonal_neighbors()
            .into_iter()
            .filter(|p| p.x >= 0 && p.x < self.width as i32 && p.y >= 0 && p.y < self.height as i32)
            .collect()
    }
}

/// Indexing implementation for Grid.
impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        self.get(point)
    }
}

/// Mutable indexing implementation for Grid.
impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        self.get_mut(point)
    }
}

/// Iterates over all the rows of the matrix.
impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}
