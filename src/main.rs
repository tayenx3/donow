mod cli;
mod task;
mod config;

use crate::cli::*;
use crate::task::*;
use crate::config::*;

use chrono::Utc;
use std::fs;
use std::io::{Read, Write};

use clap::Parser;
use colored::Colorize;
use serde_json::from_str;

const CONFIG_PATH: &'static str = "./config/config.json";

// PLANS
/*
usage:

$ donow add "Learn Backend, dummy!"
> Task added "Learn Backend, dummy!" (ID: 1)

$ donow search-name "Learn Backend, dummy!"
> Task found (ID: 1)

$ donow search name "Learn C++"
> Task not found

$ donow search id 1
> Task found ("Learn Backend, dummy!")

$ donow del "Learn Backend, dummy!"
> Task deleted

$ donow add "Learn WASM"
> Task added "Learn WASM" (ID: 1)

$ donow view
> Tasks:
1 - "Learn WASM"

*/

fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let json_string = fs::read_to_string(CONFIG_PATH)
    .expect("Missing config file");

    let config: AppConfig = from_str(&json_string)?;

    Ok(config)
}

fn load_tasks(path: &str) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let mut file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) => return Ok(Vec::new()),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.trim().is_empty() {
        return Ok(Vec::new());
    }

    let tasks: Vec<Task> = serde_json::from_str(&contents)?;
    Ok(tasks)
}

fn save_tasks(tasks: &Vec<Task>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = serde_json::to_string_pretty(tasks)
    .expect("Missing task data file");

    let mut file = fs::File::create(&path)?;

    file.write_all(json_string.as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config = load_config()?;
    let data_path = config.data_path;
    let mut loaded_tasks = load_tasks(&data_path)?;

    match cli.command {
        Command::Add { name, id, description } => {
            let new_task = if let Some(actual_description) = description {
                match id {
                    Some(id) => Task::new(name, id, Status::Todo, actual_description, Utc::now()),
                    None => {
                        let id = determine_id(&loaded_tasks);

                        Task::new(name, id, Status::Todo, actual_description, Utc::now())
                    }
                }
            } else {
                match id {
                    Some(id) => Task::new(name.clone(), id, Status::Todo, name, Utc::now()),
                    None => {
                        let id = determine_id(&loaded_tasks);

                        Task::new(name.clone(), id, Status::Todo, name, Utc::now())
                    }
                }
            };
            loaded_tasks.push(new_task);
            let result = save_tasks(&loaded_tasks, &data_path);
            match result {
                Ok(()) => println!("Operation successful"),
                Err(e) => eprintln!("{}", e)
            }
        },
        Command::DeleteByName { name, first } => {
            match first {
                true => delete_first_task_name(&mut loaded_tasks, &name),
                false => delete_task_name(&mut loaded_tasks, &name)
            }
            let result = save_tasks(&loaded_tasks, &data_path);
            match result {
                Ok(()) => println!("Operation successful"),
                Err(e) => eprintln!("{}", e)
            }
        },
        Command::DeleteByID { id } => {
            delete_task_id(&mut loaded_tasks, id);
            let result = save_tasks(&loaded_tasks, &data_path);
            match result {
                Ok(()) => println!("Operation successful"),
                Err(e) => eprintln!("{}", e)
            }
        },
        Command::UpdateByName { status , name, first } => {
            match first {
                true => update_first_task_name(&mut loaded_tasks, &name, &status),
                false => update_task_name(&mut loaded_tasks, &name, &status)
            }
            let result = save_tasks(&loaded_tasks, &data_path);
            match result {
                Ok(()) => println!("Operation successful"),
                Err(e) => eprintln!("{}", e)
            }
        },
        Command::UpdateByID { status , id} => {
            update_id(&mut loaded_tasks, id, &status);
            let result = save_tasks(&loaded_tasks, &data_path);
            match result {
                Ok(()) => println!("Operation successful"),
                Err(e) => eprintln!("{}", e)
            }
        },
        Command::View => {
            let tasks = loaded_tasks;
            println!("{}", "Tasks:".green().bold());
            if tasks.is_empty() {
                println!("Nothing here...")
            } else {
                tasks.iter().for_each(|task| {
                    println!("{}\n", task);
                })
            }
        },
        Command::SearchByName { name, first } => {
            if !first {
                let search_result = search_task_name(&loaded_tasks, &name);
                println!("{}", "Matches:".green().bold());
                if search_result.is_empty() {
                    println!("Nothing here...")
                } else {
                    search_result.iter().for_each(|task| {
                        println!("{}\n", task);
                    })
                }
            } else {
                let search_result = search_first_task_name(&loaded_tasks, &name);
                match search_result {
                    Some(task) => println!("{}\n{}", "Found Task: ".green().bold(), task),
                    None => eprintln!("{}", "Task not found".red())
                }
            }
        },
        Command::SearchByID { id } => {
            let search_result = search_id(&loaded_tasks, id);
            match search_result {
                Some(task) => println!("{}\n{}", "Found Task: ".green().bold(), task),
                None => eprintln!("{}", "Task not found".red())
            }
        },
        Command::Clear { force } => {
            let mut continue_clear: bool = false;
            if force {
                continue_clear = true;
            } else {
               loop {
                    print!("{} This command clears all of your tasks, do you wish to continue? [y/n] ", "WARNING:".yellow().bold());
                    std::io::stdout().flush().expect("Failed to flush stdout");

                    let mut ans = String::new();
                    std::io::stdin().read_line(&mut ans).expect("Failed to read input");
                    if ans.trim().to_lowercase() == "y" {
                        continue_clear = true;
                        break
                    } else if ans.trim().to_lowercase() == "n" {
                        break
                    } else {
                        println!("{}", "Invalid input, please try again".red())
                    }
                } 
            }
            if continue_clear {
                let clear_result = save_tasks(&vec![], &data_path);
                match clear_result {
                    Ok(()) => println!("{}", "Tasks cleared successfully".green().bold()),
                    Err(_) => eprintln!("{}", "Clear failed".red().bold())
                }
            }
        },

        Command::EditName { id, new_name } => {
            edit_task_name(&mut loaded_tasks, id, &new_name);
            let result = save_tasks(&loaded_tasks, &data_path);
            match result {
                Ok(()) => println!("Operation successful"),
                Err(e) => eprintln!("{}", e)
            }
        },
        Command::EditDescription { id, new_description } => {
            edit_task_description(&mut loaded_tasks, id, &new_description);
            let result = save_tasks(&loaded_tasks, &data_path);
            match result {
                Ok(()) => println!("Operation successful"),
                Err(e) => eprintln!("{}", e)
            }
        }
    }

    Ok(())
}
