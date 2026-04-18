use std::path::PathBuf;

use dirs::data_dir;

pub mod database;
pub mod plots;
mod utils;

pub fn app_data_dir() -> PathBuf {
    let root = data_dir().unwrap();
    root.join("RejectionRoulette")
}
