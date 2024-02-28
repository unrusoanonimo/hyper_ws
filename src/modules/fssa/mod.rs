use std::{
    fs::{self, File},
    io::{Cursor, Write},
    path::{Path, PathBuf},
};

use const_format::formatcp;
use once_cell::sync::Lazy;
use zip::{write::FileOptions, ZipWriter};

use super::Result;
use crate::util::{files::is_filename, zip_utils};

pub struct FssaModule {}
static FILE_OPTS: Lazy<FileOptions> =
    Lazy::new(|| FileOptions::default().compression_level(Some(9)));

impl FssaModule {
    const MODPACK_VERSION: &str = "1.0.0";
    pub const MODPACK_FILENAME: &str =
        formatcp!("fssa-modpack({}).zip", FssaModule::MODPACK_VERSION);

    const RELEASE_CACHE_PATH: &str = "cache/fssa/release.zip";
    const CONFIG_DIR: &str = "data/fssa/config";

    const BOTH_MOD_DIR: &str = "data/fssa/both";
    const SERVER_MOD_DIR: &str = "data/fssa/server";
    const CLIENT_MOD_DIR: &str = "data/fssa/client";
    const MOD_DIRS: [&str; 3] = [
        Self::BOTH_MOD_DIR,
        Self::SERVER_MOD_DIR,
        Self::CLIENT_MOD_DIR,
    ];

    pub fn new() -> Self {
        let s = Self {};
        s.release_init().unwrap();
        s
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
    pub fn release(&self) -> Result<Vec<u8>> {
        Ok(fs::read(Self::RELEASE_CACHE_PATH)?)
    }
    fn release_init(&self) -> Result<()> {
        if PathBuf::from(Self::RELEASE_CACHE_PATH).exists() {
            return Ok(());
        }
        let cursor: Cursor<Vec<u8>> = Cursor::new(vec![]);

        let mut zip = ZipWriter::new(cursor);

        zip_utils::add_dir(&mut zip, Self::BOTH_MOD_DIR, "mods/", FILE_OPTS.clone())?;
        zip_utils::add_dir(&mut zip, Self::SERVER_MOD_DIR, "mods/", FILE_OPTS.clone())?;
        zip_utils::add_dir(&mut zip, Self::CONFIG_DIR, "config/", FILE_OPTS.clone()).unwrap();

        let data = zip.finish()?.into_inner();
        let path = Path::new(Self::RELEASE_CACHE_PATH);
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix)?;
        Ok(File::create(path).unwrap().write_all(&data)?)
    }
}
