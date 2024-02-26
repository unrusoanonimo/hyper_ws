use std::fs;

use crate::{logger, router::PUBLIC_DIR};

pub fn all() {
    required();
    empty_dirs();
}
pub fn required() {
    logger::setup();
}
fn empty_dirs() {
    fs::create_dir_all(PUBLIC_DIR).unwrap();
}
