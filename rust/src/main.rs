// fn main() {
//     // let x: i32 = 1;
//     // println!("{}", x);
  
//     // let is_male: bool = false;
//     // let is_above_18: bool = true;
    
//     // if is_male {
//     //     println!("You are a male");

//     // } else {
//     //     println!("You are not a male");
//     // }

//     // if is_male && is_above_18 {
//     //     print!("You are a legal male");
//     // }

//     // let greeting: String = String::from("hello world");
//     // println!("{}", greeting);


//     // let is_even: bool = true;

//     // if is_even {
//     //     println!("The number is even");
//     // }
//     // else {
//     //     println!("The number is odd");
//     // }

//     let sentence: String = String::from("The quick brown fox jumps over the lazy dog");
//     let first_word :String =  get_first_word(sentence);

//     print!("first word is: {}" , first_word);
// }


// fn get_first_word(sentence: String) -> String {
//     let mut ans: String = String::from("");

//     for char in sentence.chars() {
//         ans.push_str(char.to_string().as_str());
//         if char == ' ' {
//             break;
//         }
//     }
//     return ans;
// }

use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use serde::{Deserialize, Serialize};
use clap::{Arg, Command};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
}

const TASK_FILE: &str = "tasks.json";

fn load_tasks() -> Vec<Task> {
    let mut file = OpenOptions::new().read(true).write(true).create(true).open(TASK_FILE).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    
    if contents.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    fs::write(TASK_FILE, json).expect("Failed to write tasks to file");
}

fn add_task(description: String) {
    let mut tasks = load_tasks();
    let new_task = Task { id: tasks.len() + 1, description };
    tasks.push(new_task);
    save_tasks(&tasks);
    println!("Task added successfully!");
}

fn list_tasks() {
    let tasks = load_tasks();
    if tasks.is_empty() {
        println!("No tasks found!");
    } else {
        for task in &tasks {
            println!("{}: {}", task.id, task.description);
        }
    }
}

fn remove_task(id: usize) {
    let mut tasks = load_tasks();
    if let Some(pos) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(pos);
        save_tasks(&tasks);
        println!("Task removed successfully!");
    } else {
        println!("Task ID not found!");
    }
}

fn main() {
    let matches = Command::new("Task Manager")
        .version("1.0")
        .author("Rust CLI Dev")
        .about("Manages your tasks")
        .subcommand(Command::new("add")
            .about("Adds a new task")
            .arg(Arg::new("description").required(true)))
        .subcommand(Command::new("list").about("Lists all tasks"))
        .subcommand(Command::new("remove")
            .about("Removes a task")
            .arg(Arg::new("id").required(true)))
        .get_matches();
    
    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let desc = sub_m.get_one::<String>("description").unwrap().to_string();
            add_task(desc);
        }
        Some(("list", _)) => list_tasks(),
        Some(("remove", sub_m)) => {
            let id = sub_m.get_one::<String>("id").unwrap().parse::<usize>().expect("Invalid task ID");
            remove_task(id);
        }
        _ => println!("Use add, list, or remove commands"),
    }
}
