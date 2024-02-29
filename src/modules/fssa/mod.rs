use std::{
    fs::{self, File},
    io::{Cursor, Write},
    path::{Path, PathBuf},
};

use const_format::formatcp;
use once_cell::sync::Lazy;
use zip::{write::FileOptions, ZipWriter};

use super::Result;
use crate::{
    config::CONFIG,
    model::fssa::{ModData, ModSide, ModpackData},
    util::{files::is_filename, zip_utils},
};

pub struct FssaModule {
    api_path: String,
    modpack: ModpackData,
}
static FILE_OPTS: Lazy<FileOptions> =
    Lazy::new(|| FileOptions::default().compression_level(Some(9)));

impl FssaModule {
    const MODPACK_VERSION: &str = "1.0.0";
    pub const MODPACK_FILENAME: &str =
        formatcp!("fssa-modpack({}).zip", FssaModule::MODPACK_VERSION);

    const RELEASE_CACHE_PATH: &str = "cache/fssa/release.zip";
    const CONFIG_CACHE_PATH: &str = "cache/fssa/config.zip";
    const CONFIG_DIR: &str = "data/fssa/config";

    const DATAPACKS_DIR: &str = "data/fssa/datapacks";

    const BOTH_MOD_DIR: &str = "data/fssa/both";
    const SERVER_MOD_DIR: &str = "data/fssa/server";
    const CLIENT_MOD_DIR: &str = "data/fssa/client";
    const MOD_DIRS: [&str; 3] = [
        Self::BOTH_MOD_DIR,
        Self::SERVER_MOD_DIR,
        Self::CLIENT_MOD_DIR,
    ];

    pub fn new() -> Self {
        let api_path = CONFIG.origin.clone() + "/api/fssa";
        let datapacks: Vec<String> = fs::read_dir(Self::DATAPACKS_DIR)
            .unwrap()
            .map(|v| {
                format!(
                    "{}/datapack/{}",
                    api_path,
                    url_escape::encode_path(&v.unwrap().file_name().to_string_lossy())
                )
            })
            .collect();

        let mods = Self::MOD_DIRS
            .iter()
            .map(|&mod_dir| {
                let modside = match mod_dir {
                    Self::BOTH_MOD_DIR => ModSide::BOTH,
                    Self::CLIENT_MOD_DIR => ModSide::CLIENT,
                    Self::SERVER_MOD_DIR => ModSide::SERVER,
                    _ => unreachable!(),
                };
                let a_path = api_path.clone();
                fs::read_dir(mod_dir)
                    .unwrap()
                    .map(move |v| {
                        let name = v.unwrap().file_name().to_string_lossy().to_string();
                        ModData::new(modside, &name, format!("{}/mod/{}", &a_path, url_escape::encode_path(&name)))
                    })
                    .collect::<Box<_>>()
            })
            .collect::<Box<[_]>>()
            .concat();

        let s = Self {
            api_path: api_path.clone(),
            modpack: ModpackData::new(Self::MODPACK_VERSION, mods, api_path + "/config", datapacks),
        };
        s.release_init().unwrap();
        s.config_init().unwrap();
        s
    }
    pub fn list(&self)->&ModpackData {
        &self.modpack
    }
    pub fn config(&self) -> Result<Vec<u8>> {
        Ok(fs::read(Self::CONFIG_CACHE_PATH)?)
    }
    pub fn get_datapack(&self, filename: &str) -> Option<Vec<u8>> {
        if is_filename(filename) {
            return fs::read(format!("{}/{}", Self::DATAPACKS_DIR, filename)).ok();
        }
        None
    }
    pub fn get_mod(&self, filename: &str) -> Option<Vec<u8>> {
        if is_filename(filename) {
            for dir in Self::MOD_DIRS {
                if let Ok(a) = fs::read(format!("{}/{}", dir, filename)) {
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
        zip_utils::add_dir(&mut zip, Self::CLIENT_MOD_DIR, "mods/", FILE_OPTS.clone())?;
        zip_utils::add_dir(&mut zip, Self::CONFIG_DIR, "config/", FILE_OPTS.clone())?;

        let data = zip.finish()?.into_inner();
        let path = Path::new(Self::RELEASE_CACHE_PATH);
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix)?;
        Ok(File::create(path)?.write_all(&data)?)
    }
    fn config_init(&self) -> Result<()> {
        if PathBuf::from(Self::CONFIG_CACHE_PATH).exists() {
            return Ok(());
        }
        let cursor: Cursor<Vec<u8>> = Cursor::new(vec![]);

        let mut zip = ZipWriter::new(cursor);

        zip_utils::add_dir(&mut zip, Self::CONFIG_DIR, "", FILE_OPTS.clone())?;

        let data = zip.finish()?.into_inner();
        let path = Path::new(Self::CONFIG_CACHE_PATH);
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix)?;
        Ok(File::create(path)?.write_all(&data)?)
    }
}
