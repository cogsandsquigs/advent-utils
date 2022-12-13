use core::fmt::Debug;
use std::{
    fs::File,
    io::{Read, Result},
    path::Path,
};

pub fn read<P>(path: P) -> Result<String>
where
    P: AsRef<Path> + Debug + Clone,
{
    let mut contents = String::new();

    File::open(path)?.read_to_string(&mut contents)?;

    Ok(contents)
}
