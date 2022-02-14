use std::fmt;
use std::env;
use std::fs;
use regex::Regex;
use std::io;
use chrono::prelude::*;
use ansi_term::Colour::*;
use ansi_term::Style;
use rand::Rng;

#[derive(Debug)]
struct Todo {
    content: String,
    completed: bool,
    date: String
}

impl Todo {
    fn new(content: String, completed: bool, date: String) -> Todo {
        let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        assert!(date_regex.is_match(&date));
        return Todo{
            content: content,
            completed: completed,
            date: date
        };
    }
}

#[derive(Debug)]
struct CookieDB {
    Cookies: Vec<String>
}

impl CookieDB {
    fn create(all: String) -> CookieDB {
        let s: Vec<&str> = all.split("\n").collect();
        let mut result: Vec<String> = Vec::new();
        for n in s.iter() {
            let mut toAdd = String::from(*n);
            toAdd.pop();
            let temp = toAdd.clone();
            result.push(toAdd);
            // println!("{:?}", String::from(temp));
        };
        return CookieDB{
            Cookies: result
        };
    }

    fn display(&self) -> &String {
        let mut rng = rand::thread_rng();
        let n: f64 = rng.gen_range(0.0..self.Cookies.len() as f64);
        return &self.Cookies[n as usize];
    }
}

enum Action {
    Daily,
    Keeper,
    List,
    Help,
    HelpVerbose,
    Nothing
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let action = &args[2];
    let enabled = ansi_term::enable_ansi_support();
//    let todo = Todo::new("Do bed".to_string(), false, "1422-02-10".to_string());

    let read = fs::read_to_string("src/cookies.txt")
       .expect("Couldn't read cookie file, make sure it exists in the currect path ... | ./cookies.txt |");

    let cookies = CookieDB::create(read);
    println!("> {}", Blue.italic().paint(cookies.display()));

    println!("{:?}", Local::today());
    
    let a = parseCommands(args);

    match a {
        Action::Nothing => {
            println!("No valid action given! {} or {} for possible actions.", 
                Style::new().bold().paint("h"),
                Style::new().bold().paint("help"),
            )
        },
        Action::Daily => {
            loop {
                let a = dreadTodoFromTerminal()
                    .expect("Quitting...");
            }
        },
        Action::Keeper => (),
        Action::List => (),
        Action::Help => (),
        Action::HelpVerbose => (),
    }
}

fn dreadTodoFromTerminal<'a>() -> Result<Todo, &'a str>{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    
    if buffer.ends_with("\n"){
        buffer.pop();
        if buffer.ends_with("\r"){
            buffer.pop();
        }
    }

    if buffer == "quit" {
        return Err("Quiting!")
    }

    let local_date = Local::today().to_string();
    let mut date_string = &*local_date;
    let mut sd:Vec<String> = date_string.split("+").map(|s| s.to_string()).collect();
    let todo = Todo::new(buffer.to_string(), false, sd[0].clone());
    println!("📝 added {} | due {}", todo.content, todo.date);
    Ok(todo)
}

fn parseCommands(command: Vec<String>) -> Action {
    match command[2].as_str() {
        "d" => Action::Daily,
        "k" => Action::Keeper,
        "l" => Action::List,
        "h" =>Action::Help,
        "help" => Action::HelpVerbose,
        _ => Action::Nothing
    }
}