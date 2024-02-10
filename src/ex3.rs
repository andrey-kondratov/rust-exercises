use db::Database;
use std::error::Error;
use std::io::{stdin, stdout, Write};

mod add;
mod db;
mod list;

/*
   Task:
       Using a hash map and vectors, create a text interface
       to allow a user to add employee names to a department
       in a company. For example, “Add Sally to Engineering”
       or “Add Amir to Sales.” Then let the user retrieve a
       list of all people in a department or all people in
       the company by department, sorted alphabetically.
*/

pub fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Employee DB, version 0.1.0.\n\n{}\n{}\nWrite `quit` or press Ctrl+C to quit.",
        add::HELP_TEXT,
        list::HELP_TEXT
    );

    let mut db = Database::new();

    loop {
        print!("\n> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        let mut words = input.split_ascii_whitespace();
        match words.next() {
            Some(command) => {
                let args: Vec<&str> = words.collect();
                match command.trim().to_lowercase().as_str() {
                    "add" => add::handle(&mut db, args),
                    "list" => list::handle(&db, args),
                    "quit" => break,
                    _ => println!("Sorry, I couldn't understand the command."),
                }
            }
            None => continue,
        }
    }

    println!("Goodbye.");

    Ok(())
}
