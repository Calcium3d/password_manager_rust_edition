use std::process::Command;
use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

pub fn setup() -> Result<()> {

    let path = if cfg!(unix) { 
        Command::new("touch ~/.password.db")
                    .spawn()
                    .expect("failed to create database");
        "~/.password.db"
    } else { 
        Command::new("touch CSIDL_PROGRAM_FILES_COMMON/.password.db")
                    .spawn()
                    .expect("Failed to create database");
        "CSIDL_PROGRAM_FILES_COMMON/.password.db" 
    };

    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE master (
            id  INTEGER PRIMARY KEY,
            masterpassword TEXT NOT NULL
        )",
        NO_PARAMS
    )?;

    conn.execute(
        "CREATE TABLE passwords (
            id INTEGER PRIMARY KEY,
            website TEXT NOT NULL,
            passwords TEXT NOT NULL
        )",
        NO_PARAMS
    )?;

    Ok(())
}