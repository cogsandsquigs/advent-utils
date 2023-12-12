use crate::point::Point;
use std::{
    fmt::{Display, Formatter},
    iter::IntoIterator,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, PartialEq, Eq)]
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

    /// Returns a new `Grid` from a string. The string is split into lines, and each line is split
    /// into characters. The values of the matrix are precomputed with the given function.
    pub fn from_str_precomp<F>(s: &str, f: F) -> Self
    where
        F: Fn(char) -> T,
    {
        let mut values = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for line in s.lines() {
            width = 0; // Reset from prev. iteration
            height += 1;

            for c in line.chars() {
                width += 1;
                values.push(f(c));
            }
        }

        Self {
            width,
            height,
            values,
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
    pub fn get(&self, point: Point<usize>) -> &T {
        &self.values[point.y as usize * self.width + point.x as usize]
    }

    /// Returns a mutable reference to the value at the given coordinates.
    pub fn get_mut(&mut self, point: Point<usize>) -> &mut T {
        &mut self.values[point.y as usize * self.width + point.x as usize]
    }

    /// Returns the value at the given coordinates.
    pub fn set(&mut self, point: Point<usize>, value: T) {
        self.values[point.y as usize * self.width + point.x as usize] = value;
    }

    /// Returns the set of all neighbors of the given coordinates. The neighbors are returned in the
    /// order of left, right, up, down, top-left, top-right, bottom-left, bottom-right.
    pub fn neighbors(&self, point: Point<usize>) -> Vec<Point<usize>> {
        [
            self.orthogonal_neighbors(point),
            self.diagonal_neighbors(point),
        ]
        .concat()
    }

    /// Returns the set of all neighbors orthogonal to the given coordinates.
    /// The neighbors are returned in the order of left, right, up, down.
    pub fn orthogonal_neighbors(&self, point: Point<usize>) -> Vec<Point<usize>> {
        point
            .orthogonal_neighbors()
            .into_iter()
            .filter(|p| p.x < self.width && p.y < self.height)
            .collect()
    }

    /// Returns the set of all neighbors diagonal to the given coordinates.
    /// The neighbors are returned in the order of top-left, top-right, bottom-left, bottom-right.
    pub fn diagonal_neighbors(&self, point: Point<usize>) -> Vec<Point<usize>> {
        point
            .diagonal_neighbors()
            .into_iter()
            .filter(|p| p.x < self.width && p.y < self.height)
            .collect()
    }

    /// Replaces every value that matches `predicate` with the new value `value`
    pub fn replace(&mut self, mut predicate: impl FnMut(&T) -> bool, value: T)
    where
        T: Clone,
    {
        self.values.iter_mut().for_each(|x| {
            let change = predicate(x);

            if change {
                *x = value.clone()
            }
        })
    }
}

/// Display implementation for Grid.
impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(Point::new(x, y)))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

/// Public API for Grid over `char`.
impl Grid<char> {
    /// Returns a new `Grid` from a string. The string is split into lines, and each line is split
    /// into characters. The values of the matrix are characters.
    pub fn from_str(s: &str) -> Self {
        Grid::from_str_precomp(s, |c| c)
    }
}

/// Public API for Grid over `bool`.
impl Grid<bool> {
    /// Returns a list of all `Point`s where the value is `true`.
    /// The points are returned in row-major order.
    pub fn true_points(&self) -> impl Iterator<Item = Point<usize>> + '_ {
        self.values
            .iter()
            .enumerate()
            .filter(|(_, x)| **x)
            .map(|(i, _)| Point::new(i % self.width, i / self.width))
    }

    /// Print out true points in a grid.
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!(
                    "{}",
                    if *self.get(Point::new(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                );
            }
            println!();
        }
    }
}

/// Indexing implementation for Grid.
impl<T> Index<Point<usize>> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point<usize>) -> &Self::Output {
        self.get(point)
    }
}

/// Mutable indexing implementation for Grid.
impl<T> IndexMut<Point<usize>> for Grid<T> {
    fn index_mut(&mut self, point: Point<usize>) -> &mut Self::Output {
        self.get_mut(point)
    }
}

/// Iterates over all the elements of the matrix, in row-major order.
/// Returns an element and its associated coordinates.
impl<T> IntoIterator for Grid<T>
where
    T: Clone,
{
    type Item = (Point<usize>, T);
    type IntoIter = std::vec::IntoIter<(Point<usize>, T)>;

    fn into_iter(self) -> Self::IntoIter {
        self.values
            .into_iter()
            .enumerate()
            .map(|(i, x)| (Point::new(i % self.width, i / self.width), x))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

/// Iterates over all the elements of the matrix, in row-major order.
/// Returns an element and its associated coordinates.
impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = (Point<usize>, &'a T);
    type IntoIter = std::vec::IntoIter<(Point<usize>, &'a T)>;

    fn into_iter(self) -> Self::IntoIter {
        self.values
            .iter()
            .enumerate()
            .map(|(i, x)| (Point::new(i % self.width, i / self.width), x))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T>
where
    T: Clone,
{
    type Error = String;

    fn try_from(v: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        Ok(Grid {
            values: v.concat(),
            width: v[0].len(),
            height: v.len(),
        })
    }
}
