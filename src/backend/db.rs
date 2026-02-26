use rusqlite::Connection;
use dirs::data_dir;
use anyhow::Result;

pub fn load_db() -> Result<Connection> {
    let db_file = data_dir().unwrap().join("applied/main.db");

    let db = Connection::open(db_file)?;
    
    Ok(db)
}

pub fn init_db() -> Result<()> {
    let db = load_db()?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS applications (
            id      INTEGER PRIMARY KEY,
            company TEXT,
            role    TEXT,
            date    TEXT
        )",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS interviews (
            id      INTEGER PRIMARY KEY,
            company TEXT,
            role    TEXT,
            date    TEXT
        )",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS responses (
            app_id   INTEGER
            response TEXT,
            date     TEXT,
            FOREIGN KEY (app_id) REFERENCES applications (id))
        )",
        (),
    )?;

    Ok(())
}