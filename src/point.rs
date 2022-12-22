use core::fmt::{Display, Formatter};
use num_traits::{real::Real, Float, Num, Signed};

/// A point in a 2D grid.
/// TODO: Change points to be T instead of usize.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T>
where
    T: Num,
{
    pub x: T,
    pub y: T,
}

/// Public API for Point.
impl<T> Point<T>
where
    T: Num,
{
    /// Returns a new `Point`.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Computes the Manhattan distance between two points.
    pub fn manhattan_distance(&self, other: &Self) -> T
    where
        T: Ord + Copy,
    {
        (self.x.max(other.x) - self.x.min(other.x)) + (self.y.max(other.y) - self.y.min(other.y))
    }

    /// Returns the set of all neighbors of the given coordinates. The neighbors are returned in the
    /// order of left, right, up, down, top-left, top-right, bottom-left, bottom-right.
    pub fn neighbors(&self) -> Vec<Self>
    where
        T: Copy + From<usize>,
    {
        [self.orthogonal_neighbors(), self.diagonal_neighbors()].concat()
    }

    /// Returns the set of all neighbors orthogonal to the given coordinates.
    /// The neighbors are returned in the order of left, right, up, down.
    pub fn orthogonal_neighbors(&self) -> Vec<Self>
    where
        T: From<usize> + Copy,
    {
        vec![
            Self::new(self.x - 1.into(), self.y),
            Self::new(self.x + 1.into(), self.y),
            Self::new(self.x, self.y - 1.into()),
            Self::new(self.x, self.y + 1.into()),
        ]
    }

    /// Returns the set of all neighbors diagonal to the given coordinates.
    /// The neighbors are returned in the order of top-left, top-right, bottom-left, bottom-right.
    pub fn diagonal_neighbors(&self) -> Vec<Self>
    where
        T: From<usize> + Copy,
    {
        vec![
            Self::new(self.x - 1.into(), self.y - 1.into()),
            Self::new(self.x + 1.into(), self.y - 1.into()),
            Self::new(self.x - 1.into(), self.y + 1.into()),
            Self::new(self.x + 1.into(), self.y + 1.into()),
        ]
    }
}

impl<T> From<(T, T)> for Point<T>
where
    T: Num,
{
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T> From<Point<T>> for (T, T)
where
    T: Num,
{
    fn from(point: Point<T>) -> Self {
        (point.x, point.y)
    }
}

impl<T> Display for Point<T>
where
    T: Num + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
