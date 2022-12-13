/// Reads a matrix of input, specifically, a matrix of characters, into some other type.
pub fn to_matrix<T, E>(
    input: &str,
    mut f: impl FnMut(char) -> Result<T, E>,
) -> Result<Vec<Vec<T>>, E> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(
                    #[allow(clippy::redundant_closure)]
                    |c| f(c),
                )
                .collect()
        })
        .collect()
}
