use anyhow::{anyhow, Result};
use core::fmt::Debug;
use std::{fmt::Display, fs::File, io::Read, path::Path};

/// Read the contents of a file into a `String`. Takes in anything that is a valid `Path` and can be
/// cloned, which includes `&str`, `String`, `PathBuf`, etc.
pub fn read<P>(path: P) -> Result<String>
where
    P: AsRef<Path> + Debug + Clone + Display,
{
    let mut contents = String::new();

    let mut file = match File::open(path.clone()) {
        Ok(file) => Ok(file),
        // TODO: only catch file not found errors here and bubble up others
        Err(..) => Err(anyhow!("File not found: {}", path)),
    }?;

    file.read_to_string(&mut contents)?;

    Ok(contents)
}
