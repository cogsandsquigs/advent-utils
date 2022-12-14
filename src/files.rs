use core::fmt::Debug;
use std::{
    fs::File,
    io::{Read, Result},
    path::Path,
};

/// Read the contents of a file into a `String`. Takes in anything that is a valid `Path` and can be
/// cloned, which includes `&str`, `String`, `PathBuf`, etc.
pub fn read<P>(path: P) -> Result<String>
where
    P: AsRef<Path> + Debug + Clone,
{
    let mut contents = String::new();

    File::open(path)?.read_to_string(&mut contents)?;

    Ok(contents)
}
