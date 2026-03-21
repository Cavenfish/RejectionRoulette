use std::{path::PathBuf, fs};

use dirs::data_dir;
use rusqlite::{params, Connection};
use anyhow::Result;
use chrono::Local;

#[derive(Debug, Clone)]
pub struct Application {
    pub id: Option<i64>,
    pub company: String,
    pub role: String,
    pub date: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Interview {
    pub id: Option<i64>,
    pub company: String,
    pub role: String,
    pub date: String,
    pub status: String,
}

pub trait Database<T> {
    fn insert(self, item: T) -> Result<()>;
    fn delete(self, id: i64) -> Result<()>;
    fn edit(self, item: T, id: i64) -> Result<()>;
    // fn search(self) -> Result<Vec<T>>;
    fn pull_all(self) -> Result<Vec<T>>;
}

#[derive(Debug)]
pub struct AppDB {
    pub filename: PathBuf,
    pub connection: Connection
}

impl AppDB {
    pub fn new() -> Self {
        let filename = data_dir().unwrap().join("RejectionRoulette/main.db");
        let save_dir = filename.parent().unwrap();

        if !save_dir.exists() {
            fs::create_dir_all(&save_dir).expect("Failed to create data dir");
        }

        let connection = Connection::open(&filename).unwrap();

        init_db(&connection).expect("Failed to init db");

        Self { filename, connection }
    }
}

impl Database<Application> for AppDB {
    fn insert(self, item: Application) -> Result<()> {

        let date = match item.date.as_str() {
            // Handle default value
            "today" => {
                let tmp = Local::now();

                &tmp.format("%Y-%m-%d").to_string()
            }

            // Handle user input
            _ => &item.date,
        };

        self.connection.execute(
            "INSERT INTO applications (
            company, role, date, status) VALUES (
            ?1, ?2, ?3, ?4)
            ",
            params![item.company, item.role, date, item.status]
        )?;

        Ok(())
    }

    fn delete(self, id: i64) -> Result<()> {
        self.connection.execute("DELETE FROM applications WHERE id=?1", params![id])?;

        Ok(())
    }

    fn edit(self, item: Application, id: i64) -> Result<()> {
        self.connection.execute(
            "UPDATE applications
            SET company=?1, role=?2, date=?3, status=?4
            WHERE id=?5
            ",
            params![item.company, item.role, item.date, item.status, id]
        )?;

        Ok(())
    }

    fn pull_all(self) -> Result<Vec<Application>> {
        let mut stmt = self.connection.prepare("SELECT * FROM applications")?;

        let tmp = stmt.query_map([], |row| {
            Ok(Application {
                id: Some(row.get(0)?),
                company: row.get(1)?,
                role: row.get(2)?,
                date: row.get(3)?,
                status: row.get(4)?
            })
        })?;

        Ok(tmp.map(|q| q.unwrap()).collect())
    }
}

impl Database<Interview> for AppDB {
    fn insert(self, item: Interview) -> Result<()> {

        self.connection.execute(
            "INSERT INTO interviews (
            company, role, date, status) VALUES (
            ?1, ?2, ?3, ?4)
            ",
            params![item.company, item.role, item.date, item.status]
        )?;

        Ok(())
    }

    fn delete(self, id: i64) -> Result<()> {
        self.connection.execute("DELETE FROM interviews WHERE id=?1", params![id])?;

        Ok(())
    }

    fn edit(self, item: Interview, id: i64) -> Result<()> {
        self.connection.execute(
            "UPDATE interviews
            SET company=?1, role=?2, date=?3, status=?4
            WHERE id=?5
            ",
            params![item.company, item.role, item.date, item.status, id]
        )?;

        Ok(())
    }

    fn pull_all(self) -> Result<Vec<Interview>> {
        let mut stmt = self.connection.prepare("SELECT * FROM interviews")?;

        let tmp = stmt.query_map([], |row| {
            Ok(Interview {
                id: Some(row.get(0)?),
                company: row.get(1)?,
                role: row.get(2)?,
                date: row.get(3)?,
                status: row.get(4)?
            })
        })?;

        Ok(tmp.map(|q| q.unwrap()).collect())
    }
}

pub fn init_db(db: &Connection) -> Result<()> {

    db.execute(
        "CREATE TABLE IF NOT EXISTS applications (
            id      INTEGER PRIMARY KEY,
            company TEXT,
            role    TEXT,
            date    TEXT,
            status  TEXT
        )",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS interviews (
            id      INTEGER PRIMARY KEY,
            company TEXT,
            role    TEXT,
            date    TEXT,
            status  TEXT
        )",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS apps_extra (
            app_id        INTEGER,
            response_time TEXT,
            FOREIGN KEY (app_id) REFERENCES applications (id)
        )",
        (),
    )?;

    Ok(())
}

//TODO
// make funcs:
//  - search apps
//  - search interviews
//
// use traits to make app/interview agnostic funcs