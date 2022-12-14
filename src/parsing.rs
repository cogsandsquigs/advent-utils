/// The general parsing type. Is implemented for `String`s.
pub trait Parseable {
    /// Parses a string into a matrix of type `T`.
    fn to_matrix<T, E>(&self, f: impl FnMut(char) -> Result<T, E>) -> Result<Vec<Vec<T>>, E>;

    /// Parses a string into a vector of type `T`.
    fn to_vec<T, E>(&self, f: impl FnMut(&str) -> Result<T, E>) -> Result<Vec<T>, E>;
}

impl Parseable for &str {
    /// Reads a matrix of input, specifically, a matrix of characters, into some other type.
    fn to_matrix<T, E>(&self, mut f: impl FnMut(char) -> Result<T, E>) -> Result<Vec<Vec<T>>, E> {
        self.lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.chars()
                    .map(
                        // Have to put this here because otherwise clippy complains about the
                        // closure being redundant. However, if we remove the closure, then we
                        // end up with a compiler error saying we can't move `f` into the closure.
                        #[allow(clippy::redundant_closure)]
                        |c| f(c),
                    )
                    .collect()
            })
            .collect()
    }

    /// Reads a series of lines as input into a vector of some type.
    fn to_vec<T, E>(&self, f: impl FnMut(&str) -> Result<T, E>) -> Result<Vec<T>, E> {
        self.lines().filter(|l| !l.is_empty()).map(f).collect()
    }
}
