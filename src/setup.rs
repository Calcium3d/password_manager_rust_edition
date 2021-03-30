use std::process::Command;
use rusqlite::{Connection, Result, params};
use rusqlite::NO_PARAMS;
use sha2::{Sha256, Digest};

struct MasterPassword {
    id: i32, 
    masterpassword: String
}

pub fn setup() -> Result<()> {

    // placing the file in the correct file path both on windows and linux
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

    //connection to the sqlite database
    let conn = Connection::open(path)?;

    //creating the masterpassword table
    conn.execute(
        "CREATE TABLE master (
            id  INTEGER PRIMARY KEY,
            masterpassword TEXT NOT NULL
        )",
        NO_PARAMS
    )?;

    //creating the regular password table
    conn.execute(
        "CREATE TABLE passwords (
            id INTEGER PRIMARY KEY,
            website TEXT NOT NULL,
            username TEXT NOT NULL,
            passwords TEXT NOT NULL
        )",
        NO_PARAMS
    )?;

    let mut line = String::new();

    let mut hasher = Sha256::new();

    //taking the input
    println!("Welcome to the setup wizard. I will guide you through setting up the password manager");
    println!("Enter the master password. Now remember, remember this or youll lose access");
    let input: usize = std::io::stdin().read_line(&mut line).unwrap();
    let input: String = input.to_string();
    hasher.update(input); //hashing the input
    let result: String = format!("{:X}", hasher.finalize());

    let data = MasterPassword {
        id: 0,
        masterpassword: result
    };

    //insert the password
    conn.execute(
        "INSERT INTO master (masterpassword) VALUES (?1)",
        params![data.masterpassword],
    )?;

    Ok(())
}