mod caeser_cypher_encrypt;
mod caeser_cypher_decrypt;
mod password_generation;
mod setup;

pub use crate::caeser_cypher_decrypt::caeser_decrypt;
pub use crate::password_generation::password_generate;
pub use crate::caeser_cypher_encrypt::caeser_encrypt;
pub use crate::setup::setup;

use sha2::{Sha256, Digest};
use std::env;
use std::process::Command;
use rusqlite::{Connection, Result, params};
use rusqlite::NO_PARAMS;

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
        let path = if cfg!(unix) {
            "~/.password.db"
        } else {
            "CSIDL_PROGRAM_FILES_COMMON/.password.db" 
        };
    
        let conn = Connection::open(path)?;
    
        let mut line = String::new();
        println!("Enter your masterpassword");
        let input: usize = std::io::stdin().read_line(&mut line).unwrap();
        let input: String = input.to_string();
        let unencrypted_string = &input[..];
        let mut hasher = Sha256::new();
        hasher.update(unencrypted_string);
        let masterhash: String = format!("{:X}", hasher.finalize());
    
        let mut stmt = conn.prepare("SELECT masterpassword FROM master")?;
    
        let person_iter = stmt.query_map(params![], |row| {
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
    
        if check.masterpassword == masterhash {
            println!("Congrats, you entered the password correctly!");
            help();
            
            let mut run = true;

            while run {
                println!("Enter Command");
                let input: usize = std::io::stdin().read_line(&mut line).unwrap();
                let input: String = input.to_string();
                let input = &input[..];
                
                if input == "SITE" {
                    let mut stmt = conn.prepare("SELECT id,website,username,password FROM passwords WHERE website = (?1)")?;
                    
                    println!("Enter the website's name: ");
                    let website_name: usize = std::io::stdin().read_line(&mut line).unwrap();
                    let website_name: String = website_name.to_string();
                    let website_name = &website_name[..];

                    let password_iter = stmt.query_map(params![website_name], |row| {
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
                        let website = caeser_decrypt(check.website);
                        let username = caeser_decrypt(check.username);
                        let password = caeser_decrypt(check.password);

                        println!("For {},\n Username: {},\n Password: {}", website, username, password);
                    }
                }
                
                else if input == "ALL"{
                    let mut stmt = conn.prepare("SELECT * FROM passwords")?;
                    let allpaswords: usize = std::io::stdin().read_line(&mut line).unwrap();
                    let allpaswords: String = allpaswords.to_string();
                    let allpaswords = &allpaswords[..];

                    let password_iter = stmt.query_map(params![allpaswords], |row| {
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

                        let website = caeser_decrypt(check.website);
                        let username = caeser_decrypt(check.username);
                        let password = caeser_decrypt(check.password);

                        println!("For {},\n Username: {},\n Password: {}", website, username, password);
                    }
                }

                else if input == "ADD" {
                    println!("Enter the website you want (no url, just the name): ");
                    let website: usize = std::io::stdin().read_line(&mut line).unwrap();
                    let website: String = caeser_encrypt(website.to_string());
                    let website = &website[..];

                    println!("Enter the username: ");
                    let username: usize = std::io::stdin().read_line(&mut line).unwrap();
                    let username: String = caeser_encrypt(username.to_string());
                    let username = &username[..];

                    println!("Enter the password you want for this website: ");
                    let password: usize = std::io::stdin().read_line(&mut line).unwrap();
                    let password: String = caeser_encrypt(password.to_string());
                    let password = &password[..];

                    

                        conn.execute(
                            "INSERT INTO password (website, username, password)",
                            params![website, username, password],
                        )?;
                    }

                else if input == "HELP" {
                    help()
                }

                else if input == "EXIT" {
                    run = false
                }

                else if input == "RESET" {
                    if cfg!(unix) {
                        Command::new("touch ~/.password.db")
                                    .spawn()
                                    .expect("failed to delete database");
                    } else {
                        Command::new("rm CSIDL_PROGRAM_FILES_COMMON/.password.db")
                                    .spawn()
                                    .expect("failed to delete database");
                    }
                }

                

                }

                

            }

        else {
            println!("YOU DIRTY SCUM, GET OUT. NO PASSWORDS FOR YOU");
        }
    }
    
    Ok(())
}

pub fn help() {
    println!("List of commands:");
    println!("To Add a password, type: ADD");
    println!("To Display all the websites, type: ALL");
    println!("To Display a specific site, type: SITE");
    println!("To exit, type: EXIT");
    println!("to display this message, type: HELP");
    println!("To reset the password manager, type: RESET");
}
