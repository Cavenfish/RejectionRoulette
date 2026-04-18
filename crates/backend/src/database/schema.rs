use std::{fs, path::PathBuf};

use anyhow::Result;
use rusqlite::{Connection, Row, params};

use crate::utils::compute_file_hash;

pub trait RowRead: Sized {
    fn from_row(row: &Row) -> rusqlite::Result<Self>;
}

pub trait RowInsert {
    fn add_row(&self, conn: &Connection) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct NewApplication {
    pub resume_id: Option<i64>,
    pub company: String,
    pub role: String,
    pub location: String,
    pub status: String,
    pub submit_date: String,
}

impl RowInsert for NewApplication {
    fn add_row(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO Applications 
            (resume_id, company, role, location, status, submit_date) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ",
            params![
                self.resume_id,
                self.company,
                self.role,
                self.location,
                self.status,
                self.submit_date
            ],
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Application {
    pub id: i64,
    pub resume_id: Option<i64>,
    pub company: String,
    pub role: String,
    pub location: String,
    pub status: String,
    pub submit_date: String,
}

impl RowRead for Application {
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            resume_id: row.get(1)?,
            company: row.get(2)?,
            role: row.get(3)?,
            location: row.get(4)?,
            status: row.get(5)?,
            submit_date: row.get(6)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct NewInterview {
    pub application_id: i64,
    pub interview_date: String,
    pub interview_type: String,
    pub notes: String,
}

impl RowInsert for NewInterview {
    fn add_row(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO Interviews 
            (application_id, interview_date, interview_type, notes) 
            VALUES (?1, ?2, ?3, ?4)",
            params![
                self.application_id,
                self.interview_date,
                self.interview_type,
                self.notes
            ],
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Interview {
    pub id: i64,
    pub application_id: i64,
    pub company: String,
    pub role: String,
    pub interview_date: String,
    pub interview_type: String,
    pub notes: String,
}

impl RowRead for Interview {
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            application_id: row.get(1)?,
            company: row.get(2)?,
            role: row.get(3)?,
            interview_date: row.get(4)?,
            interview_type: row.get(5)?,
            notes: row.get(6)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct NewOffer {
    pub application_id: i64,
    pub base_salary: i64,
    pub bonus: i64,
    pub equity_details: String,
    pub expiration_date: String,
    pub is_accepted: bool,
}

impl RowInsert for NewOffer {
    fn add_row(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO Offers
            (application_id, base_salary, bonus, equity_details, expiration_date, is_accepted)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                self.application_id,
                self.base_salary,
                self.bonus,
                self.equity_details,
                self.expiration_date,
                self.is_accepted
            ],
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Offer {
    pub id: i64,
    pub application_id: i64,
    pub company: String,
    pub role: String,
    pub base_salary: i64,
    pub bonus: i64,
    pub equity_details: String,
    pub expiration_date: String,
    pub is_accepted: bool,
}

impl RowRead for Offer {
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            application_id: row.get(1)?,
            company: row.get(2)?,
            role: row.get(3)?,
            base_salary: row.get(4)?,
            bonus: row.get(5)?,
            equity_details: row.get(6)?,
            expiration_date: row.get(7)?,
            is_accepted: row.get(8)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct NewResume {
    pub file_path: PathBuf,
    pub name: String,
}

impl RowInsert for NewResume {
    fn add_row(&self, conn: &Connection) -> Result<()> {
        let hash = compute_file_hash(&self.file_path)?;
        let dest_dir = dirs::data_dir().unwrap().join("RejectionRoulette/Resumes/");
        let mut dest_path = dest_dir.join(&hash);
        dest_path.set_extension("pdf");

        conn.execute(
            "INSERT INTO Resumes (name, hash) VALUES (?1, ?2)",
            params![self.name, hash],
        )?;

        // Avoid duplication
        if !dest_path.exists() {
            fs::copy(&self.file_path, dest_path)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Resume {
    pub id: i64,
    pub name: String,
    pub hash: String,
}

impl RowRead for Resume {
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            hash: row.get(2)?,
        })
    }
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
    conn.execute("PRAGMA schema_version = 1;", [])?;

    let schema = "
        BEGIN;

        -- Parent Table
        CREATE TABLE IF NOT EXISTS Applications (
            id          INTEGER PRIMARY KEY,
            resume_id   INTEGER,
            company     TEXT,
            role        TEXT,
            location    TEXT,
            status      TEXT NOT NULL,
            submit_date DATETIME,
            FOREIGN KEY (resume_id) REFERENCES Resumes (id)
        );

        -- Child Table: Interviews
        CREATE TABLE IF NOT EXISTS Interviews (
            id                INTEGER PRIMARY KEY,
            application_id    INTEGER NOT NULL,
            interview_date    DATETIME NOT NULL,
            interview_type    TEXT,
            notes             TEXT,
            FOREIGN KEY (application_id) REFERENCES Applications (id) ON DELETE CASCADE
        );

        -- Child Table: Offers
        CREATE TABLE IF NOT EXISTS Offers (
            id              INTEGER PRIMARY KEY,
            application_id  INTEGER NOT NULL,
            base_salary     INTEGER,
            bonus           INTEGER,
            equity_details  TEXT,
            expiration_date DATETIME,
            is_accepted     BOOLEAN,
            FOREIGN KEY (application_id) REFERENCES Applications (id) ON DELETE CASCADE
        );

        -- Resume Tracking Table
        CREATE TABLE IF NOT EXISTS Resumes (
            id   INTEGER PRIMARY KEY,
            name TEXT UNIQUE,
            hash TEXT NOT NULL
        );

        COMMIT;
    ";

    conn.execute_batch(schema)?;

    Ok(())
}
