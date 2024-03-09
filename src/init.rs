use std::fs;

use crate::{logger, router::PUB_DIR};

pub fn all() {
    required();
    empty_dirs();
}
pub fn required() {
    logger::setup();
}
fn empty_dirs() {
    fs::create_dir_all(&*PUB_DIR).unwrap();
}
