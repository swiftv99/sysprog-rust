use std::io;
use std::io::Write;

struct Book {
    title: String,
    year: u32,
}

impl Book {
    fn new(title: String) -> Self {
        Self {
            title,
            year: 0,
        }
    }
}

enum Command {
    Add(String),
    Year(String, u32),
    Rem(String),
    Show,
    Help,
    Quit,
}

fn get_cmd() -> Command {
    // print prompt (ignore errors)
    print!("> ");
    io::stdout().flush().ok();

    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
 
    let words: Vec<&str> = command.split_whitespace().collect();

    match words[0] {
        "add" => Command::Add(words[1].to_string()),
        "year" => Command::Year(words[1].to_string(), words[2].parse().unwrap()),
        "rem" => Command::Rem(words[1].to_string()),
        "show" => Command::Show,
        "help" => Command::Help,
        "quit" => Command::Quit,
        _ => panic!("Unexpected command {}", words[0]), 
    }
}

fn cmd_help() {
    println!("The following commands are available:");
    println!("  add <title>");
    println!("  year <title> <year>");
    println!("  rem <title>");
    println!("  show");
    println!("  help");
    println!("  quit");
}

fn main() {
    let mut books = Vec::<Book>::new();

    println!("Welcome! Please enter a command.");

    loop {
        let command = get_cmd();

        match command {
            Command::Add(title) => books.push(Book::new(title)),
            Command::Year(title, year) => {
                let book = books.iter_mut().find(|b| b.title == title).unwrap();
                book.year = year;
            },
            Command::Rem(title) => {
                books.retain(|b| b.title != title);
            },
            Command::Show => {
                for b in &books {
                    println!("{} (published {})", b.title, b.year);
                }
            },
            Command::Help => cmd_help(),
            Command::Quit => break,
        }
    }
}
