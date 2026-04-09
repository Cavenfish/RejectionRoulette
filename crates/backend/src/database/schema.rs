use anyhow::Result;
use chrono::Local;
use rusqlite::{Connection, Row, params};

pub trait RowRead: Sized {
    fn from_row(row: &Row) -> rusqlite::Result<Self>;
}

pub trait RowInsert {
    fn add_row(&self, conn: &Connection) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct NewApplication {
    pub company: String,
    pub role: String,
    pub status: String,
    pub submit_date: String,
}

impl RowInsert for NewApplication {
    fn add_row(&self, conn: &Connection) -> Result<()> {
        let date = match self.submit_date.as_str() {
            // Handle default value
            "today" => {
                let tmp = Local::now();

                &tmp.format("%Y/%m/%d").to_string()
            }

            // Handle user input
            _ => &self.submit_date,
        };

        conn.execute(
            "INSERT INTO Applications (
            company, role, status, submit_date) VALUES (
            ?1, ?2, ?3, ?4)
            ",
            params![self.company, self.role, self.status, date],
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Application {
    pub id: i64,
    pub company: String,
    pub role: String,
    pub status: String,
    pub submit_date: String,
}

impl RowRead for Application {
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            company: row.get(1)?,
            role: row.get(2)?,
            status: row.get(3)?,
            submit_date: row.get(4)?,
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
    conn.execute("PRAGMA schema_version = 1;", [])?;

    let schema = "
        BEGIN;

        -- Parent Table
        CREATE TABLE IF NOT EXISTS Applications (
            id          INTEGER PRIMARY KEY,
            company     TEXT NOT NULL,
            role        TEXT NOT NULL,
            status      TEXT NOT NULL,
            submit_date DATETIME
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

        COMMIT;
    ";

    conn.execute_batch(schema)?;

    Ok(())
}
