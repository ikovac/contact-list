use std::io::{self, Write};

mod lists;

use lists::List;

#[derive(Debug)]
struct Contact {
    name: String,
    email: String,
}

impl Contact {
    fn new(name: String, email: String) -> Contact {
        Contact { name, email }
    }
}

fn main() {
    println!("Welcome to the contacts app");
    let mut list: List<Contact> = List::new_empty();
    loop {
        println!("********* Options *********");
        println!("** List contacts - 1");
        println!("** Add contact - 2");
        println!("** Type any other number for exit");
        println!("Type your choice");

        let choice = take_user_input();

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match choice {
            1 => {
                println!("Listing all contacts...");
                print_contacts(&list);
            }
            2 => {
                let contact = add_contact();
                list.append_start(contact);
                println!("New contact added");
            }
            _ => {
                println!("exit");
                break;
            }
        }

        println!();
    }
}

fn print_contacts(list: &List<Contact>) {
    for item in list.iter() {
        let contact = &item.borrow().data;
        println!("*** Contact ***");
        println!("Name: {}", contact.name);
        println!("Email: {}", contact.email);
    }
}

fn add_contact() -> Contact {
    println!("Add new contact");
    print!("Name: ");
    io::stdout().flush().unwrap();
    let name = take_user_input().trim().to_string();

    print!("Email: ");
    io::stdout().flush().unwrap();
    let email = take_user_input().trim().to_string();

    Contact::new(name, email)
}

fn take_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}
