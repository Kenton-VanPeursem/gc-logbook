use rusqlite::{Connection, Result};
use chrono::NaiveDate;

pub mod patient;
use patient::{Gender, Specialty, Patient};

/// Database connection to SQLite.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Constructor for Database
    pub fn new(dbname: &str) -> Database {
        let db = Database {
            conn: Connection::open(dbname).unwrap(),
        };
        return db;
    }

    /// Create the patient table.
    pub fn create_patient_table(&self) {
        match self.conn.execute(
            "CREATE TABLE IF NOT EXISTS patients (date DATE, specialty VARCHAR(9), gender VARCHAR(7), age INTEGER, indication VARCHAR(250), summary CLOB);",
            ()
        ) {
            Ok(_) => (),
            Err(err) => println!("Err: {}", err)
        }
    }

    /// Insert patient object.
    pub fn insert_patient(&self, p: Patient) -> Result<()> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO patients (date, specialty, gender, age, indication, summary) VALUES (?, ?, ?, ?, ?, ?);"
        )?;
        stmt.execute(
            (p.date, p.specialty.as_str(), p.gender.as_str(), p.age, p.indication, p.summary)
        )?;

        Ok(())
    }

    /// Lookup patient by date seen.
    pub fn lookup_patient_by_date(&self, date: NaiveDate) -> Result<Vec<Patient>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM patients WHERE date=? ORDER BY date;")?;
        let patients = stmt.query_map([date], |row| {
            Ok(Patient {
                date: row.get(0)?,
                specialty: Specialty::from_str(row.get(1)?),
                gender: Gender::from_str(row.get(2)?),
                age: row.get(3)?,
                indication: row.get(4)?,
                summary: row.get(5)?,
            })
        })?;

        let mut pats: Vec<Patient> = Vec::new();
        for p in patients {
            pats.push(p?);
        }

        Ok(pats)
    }

    /// Lookup patient by specialty.
    pub fn lookup_patient_by_specialty(&self, specialty: Specialty) -> Result<Vec<Patient>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM patients WHERE specialty=? ORDER BY date;")?;
        let patients = stmt.query_map([specialty.as_str()], |row| {
            Ok(Patient {
                date: row.get(0)?,
                specialty: Specialty::from_str(row.get(1)?),
                gender: Gender::from_str(row.get(2)?),
                age: row.get(3)?,
                indication: row.get(4)?,
                summary: row.get(5)?,
            })
        })?;

        let mut pats: Vec<Patient> = Vec::new();
        for p in patients {
            pats.push(p?);
        }

        Ok(pats)
    }
}
