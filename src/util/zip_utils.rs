use std::{
    fs,
    io::{Cursor, Write},
    path::{Path, PathBuf},
};

use zip::{write::FileOptions, ZipWriter};

pub fn add_dir(
    zip: &mut ZipWriter<Cursor<Vec<u8>>>,
    dir_path: impl Into<PathBuf>,
    zip_prefix: impl Into<PathBuf>,
    options: FileOptions,
) -> Result<(), std::io::Error> {
    let dir_path: PathBuf = dir_path.into();
    let prefix: PathBuf = zip_prefix.into();

    for path in fs::read_dir(dir_path)? {
        let file_path = path?;
        let filename = file_path.file_name();
        let mut inner_name = prefix.clone();
        inner_name.push(filename);
        add_file(zip, file_path.path(), inner_name.to_string_lossy(), options)?;
    }
    Ok(())
}

pub fn add_file(
    zip: &mut ZipWriter<Cursor<Vec<u8>>>,
    file_path: impl AsRef<Path>,
    inner_name: impl Into<String>,
    options: FileOptions,
) -> Result<(), std::io::Error> {
    zip.start_file(inner_name, options)?;
    let data = fs::read(file_path)?;
    zip.write_all(&data)?;
    Ok(())
}
