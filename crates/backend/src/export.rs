use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use csv::Writer;
use serde::Serialize;

pub fn export_table<T: Serialize>(table: Vec<T>, filename: &Path) -> Result<()> {
    let mut writer = Writer::from_path(filename)?;

    for row in table.iter() {
        writer.serialize(row)?;
    }

    writer.flush()?;

    Ok(())
}

pub fn export_data<T: Serialize>(data: T, filename: &Path) -> Result<()> {
    let data = toml::to_string_pretty(&data)?;
    let mut file = fs::File::create(&filename)?;

    file.write_all(data.as_bytes())?;
    Ok(())
}
