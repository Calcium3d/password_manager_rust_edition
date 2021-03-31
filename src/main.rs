mod setup;
mod password_generation;

pub use crate::setup::setup;
pub use crate::password_generation::password_generate;

extern crate base64;

use sha2::{Sha256, Digest};
use std::env;
use std::process::Command;
use rusqlite::{Connection, Result, params};
use rusqlite::NO_PARAMS;
use base64::{encode, decode};

struct MasterPassword {
    id: i32, 
    masterpassword: String
}

struct Password {
    id: i32, 
    website: String,
    username: String,
    password: String
}

struct ID {
    id: i32
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if &args[1] == "--setup" {
        setup();
    }

    else {
        //reading teh correct path on an os
        let path = "./.password.db";
    
        let conn = Connection::open(path)?; //opening a connection
    
        let mut line = String::new();
        println!("Enter your masterpassword");
        let input = input_to_string();
        let unencrypted_string = &input[..];
        let mut hasher = Sha256::new();
        hasher.update(unencrypted_string); //hashing
        let masterhash: String = format!("{:X}", hasher.finalize()); //converting hash to string

        //reading the password from the database
        let mut stmt = conn.prepare("SELECT masterpassword FROM master")?;
    
        let person_iter = stmt.query_map(params![], |row| { //querying for the password
            Ok(MasterPassword {
                id: row.get(0)?,
                masterpassword: row.get(1)?,
            })
        })?;
    
        let mut check: MasterPassword = MasterPassword {
            id: 0,
            masterpassword: "".to_string()
        };
    
        for pass in person_iter {
           check = pass.unwrap();
        }
    
        if check.masterpassword == masterhash { //checking the password
            println!("Congrats, you entered the password correctly!");
            help();
            
            let mut run = true;

            while run {
                println!("Enter Command");
                let input = input_to_string();
                let input = &input[..];
                
                if input == "SITE" {
                    site(path);
                }
                
                else if input == "ALL"{
                    all(path);
                }

                else if input == "ADD" {
                    add(path);
                    }

                else if input == "HELP" {
                    help() //displaying the help message
                }

                else if input == "EXIT" {
                    run = false //exiting the look
                }

                else if input == "RESET" {
                    reset();
                }
                }
            }

        else {
            println!("YOU DIRTY SCUM, GET OUT. NO PASSWORDS FOR YOU"); //reminding people not to try these cheap tricks
        }
    }
    
    Ok(())
}

pub fn help() { //the actual help function you know
    println!("List of commands:");
    println!("To Add a password, type: ADD");
    println!("To Display all the websites, type: ALL");
    println!("To Display a specific site, type: SITE");
    println!("To exit, type: EXIT");
    println!("to display this message, type: HELP");
    println!("To reset the password manager, type: RESET");
}

pub fn reset() {
    Command::new("rm/.password.db");
}

pub fn add(path: &str) -> Result<()> {
    let conn = Connection::open(path)?;
    let mut line = String::new();

    //inputting the website name
    println!("Enter the website you want (no url, just the name): ");
    let websites: String = encode(input_to_string());

    //inputting the username
    println!("Enter the username: ");
    let username: String = encode(input_to_string());

    //inputting the password
    println!("Do you want to be assigned a password? (y/n) {{default is n}}");
    let choice = input_to_string();
    let choice: &str = &choice[..];

    let password = if choice == "y" {
        println!("Generating password");

        //getting the length
        println!("What's the length of the password");
        let length = input_to_string();
        let length: i32 = length.parse::<i32>().unwrap();

        //getting the capitalisation
        println!("Do you want capitalisation in the password? (y/n) {{default = n}}");
        let cap = input_to_string();
        let cap: &str = &cap[..];

        let caps = if cap == "y" {
            true
        } else {
            false
        };


        //getting teh numbers
        println!("Do you want numbers in the password? (y/n) {{default is n}}");
        let num = input_to_string();
        let num: &str = &num[..];

        let numbers = if num == "y" {
            true
        } else {
            false
        };

        //getting the symbols
        println!("Do you want symbols in the password? (y/n) {{default is n}}");
        let symbol = input_to_string();
        let symbol: &str = &symbol[..];

        let symbols = if symbol == "y" {
            true
        } else {
            false
        };

        password_generate(length, caps, numbers, symbols)
    } else {
        println!("Enter the password you want for this website: ");
        let password: String = encode(input_to_string());
        password
    };

    let mut stmt = conn.prepare("SELECT id FROM passwords ORDER BY _id  DESC LIMIT 1")?;

    let id_iter = stmt.query_map(params![], |row| {
        Ok(ID {
            id: row.get(0)?
        })
    })?;

    let mut like_id_pls: ID = ID {
        id: 0
    };

    let mut id_dont_know_any_more_names_for_this: ID = ID {
        id: 0
    };

    for like_this_id_name in id_iter {
        id_dont_know_any_more_names_for_this = like_this_id_name.unwrap();
    }

    let entry: Password = Password {
        id: id_dont_know_any_more_names_for_this.id,
        website: websites, 
        username: username,
        password: password
    };

    conn.execute(
        "INSERT INTO passwords (website, username, password)",
        params![entry.website, entry.username, entry.password],
        )?;

    Ok(())
}

pub fn all(path: &str) -> Result<()>{
    let conn = Connection::open(path)?;
    let mut stmt = conn.prepare("SELECT * FROM passwords")?; //selecting everything
                    let allpaswords = input_to_string();
                    let allpaswords = &allpaswords[..];

                    let password_iter = stmt.query_map(params![allpaswords], |row| { //querying all sites
                        Ok(Password {
                            id: row.get(0)?,
                            website: row.get(1)?,
                            username: row.get(2)?,
                            password: row.get(3)?
                        })
                    })?;

                    let mut check: Password = Password {
                        id: 0,
                        website: "".to_string(), 
                        username: "".to_string(), 
                        password: "".to_string()
                    };

                    for pass in password_iter {
                        check = pass.unwrap();

                        let website = decode(check.website).unwrap();
                        let username = decode(check.username).unwrap();
                        let password = decode(check.password).unwrap();

                        println!("For {:?},\n Username: {:?},\n Password: {:?}", website, username, password); //displaying the info
                    }
    Ok(())
}

pub fn site(path: &str) -> Result<()> {
    let conn = Connection::open(path)?;

    let mut stmt = conn.prepare("SELECT id,website,username,password FROM passwords WHERE website = (?1)")?; //selecting a specific site
                    
                    println!("Enter the website's name: ");
                    let website_name = input_to_string();
                    let website_name = &website_name[..];

                    let password_iter = stmt.query_map(params![website_name], |row| { //querying the specific site
                        Ok(Password {
                            id: row.get(0)?,
                            website: row.get(1)?,
                            username: row.get(2)?,
                            password: row.get(3)?
                        })
                    })?;

                    let mut check: Password = Password {
                        id: 0,
                        website: "".to_string(), 
                        username: "".to_string(), 
                        password: "".to_string()
                    };

                    for pass in password_iter {
                        check = pass.unwrap();
                        let website = decode(check.website).unwrap();
                        let username = decode(check.username).unwrap();
                        let password = decode(check.password).unwrap();

                        println!("For {:?},\n Username: {:?},\n Password: {:?}", website, username, password); //displaying the info
                    }
    Ok(())
}

pub fn input_to_string() -> String {
    let mut line = String::new();
    let input: usize = std::io::stdin().read_line(&mut line).unwrap(); 
    let input: String = input.to_string();

    input
}
