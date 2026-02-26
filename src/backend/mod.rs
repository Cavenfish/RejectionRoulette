use std::{path::Path, fs};

use dirs::data_dir;

pub mod db;

use db::init_db;

pub fn init_local() {
    let save_dir = data_dir().unwrap().join("applied");

    if !Path::new(&save_dir).exists() {
        fs::create_dir_all(&save_dir).expect("Failed to create data dir");
    }

    let db_file = save_dir.join("main.db");

    if !Path::new(&db_file).exists() {
        init_db().expect("Failed to init db");
    }
}