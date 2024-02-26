use std::{
    fs,
    io::{Cursor, Write},
};

use once_cell::sync::Lazy;
use zip::{result::ZipResult, write::FileOptions, ZipWriter};

use crate::util::files::is_filename;

pub struct FssaModule {}
static FILE_OPTS: Lazy<FileOptions> =
    Lazy::new(|| FileOptions::default().compression_level(Some(9)));

impl FssaModule {
    const MOD_DIRS: [&str; 3] = ["data/fssa/both", "data/fssa/server", "data/fssa/client"];

    pub fn new() -> Self {
        Self {}
    }
    pub fn get_mod(&self, filename: &str) -> Option<Vec<u8>> {
        if is_filename(filename) {
            for dir in Self::MOD_DIRS {
                if let Ok(a) = fs::read(format!("{dir}/{filename}")) {
                    return Some(a);
                }
            }
        }
        None
    }
    pub fn release(&self) -> ZipResult<Vec<u8>> {
        let cursor: Cursor<Vec<u8>> = Cursor::new(vec![]);

        let mut zip = ZipWriter::new(cursor);

        zip.start_file("dir/readme.txt", FILE_OPTS.clone())?;
        zip.write_all(b"Hello, World!\n")?;
        zip.write_all(b"Hello, World!\n")?;
        zip.start_file("./readme.txt", FILE_OPTS.clone())?;
        zip.write_all(b"Hello, Worldasdasd!\n")?;

        self.client(&mut zip, "zip_prefix");

        zip.finish().map(|v| v.into_inner())
    }
    fn client(&self, zip: &mut ZipWriter<Cursor<Vec<u8>>>, zip_prefix: impl Into<String>) {
        let prefix = zip_prefix.into();
        let d = fs::read_dir("data/fssa/both");
        d.unwrap().for_each(|v| {
            dbg!(v);
        });
    }
}
