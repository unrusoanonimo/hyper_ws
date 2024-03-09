use std::{
    fs,
    io::{Seek, Write},
    path::{Path, PathBuf},
};

use zip::{write::FileOptions, ZipWriter};

pub fn add_dir(
    zip: &mut ZipWriter<impl Write + Seek>,
    dir_path: impl Into<PathBuf>,
    zip_prefix: impl Into<PathBuf>,
    options: FileOptions,
) -> Result<(), std::io::Error> {
    let dir_path: PathBuf = dir_path.into();
    let prefix: PathBuf = zip_prefix.into();

    for dir_entry in fs::read_dir(dir_path)? {
        let dir_entry = dir_entry?;
        let name = dir_entry.file_name();
        let path = dir_entry.path();
        let mut inner_name = prefix.clone();
        inner_name.push(name);

        if dir_entry.path().is_file() {
            add_file(zip, path, inner_name.to_string_lossy(), options)?;
        } else {
            add_dir(zip, path, inner_name, options)?;
        }
    }
    Ok(())
}

pub fn add_file(
    zip: &mut ZipWriter<impl Write + Seek>,
    file_path: impl AsRef<Path>,
    inner_name: impl Into<String>,
    options: FileOptions,
) -> Result<(), std::io::Error> {
    zip.start_file(inner_name, options)?;
    let data = fs::read(file_path)?;
    zip.write_all(&data)?;
    Ok(())
}
