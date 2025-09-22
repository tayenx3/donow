mod cli;
mod task;
mod config;

use crate::cli::*;
use crate::task::*;
use crate::config::*;

use chrono::Utc;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use clap::Parser;
use colored::Colorize;
use serde_json::from_str;

fn load_config(path: &Path) -> Result<AppConfig, Box<dyn std::error::Error>> {
    if !path.exists() {
        let default_config = AppConfig {
            data_path: "tasks.json".to_string()
        };
        let json = serde_json::to_string_pretty(&default_config)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, json)?;
    }
    
    let json_string = std::fs::read_to_string(path)?;
    let config: AppConfig = from_str(&json_string)?;
    Ok(config)
}

fn get_project_dirs() -> Result<directories::ProjectDirs, Box<dyn std::error::Error>> {
    directories::ProjectDirs::from("rs", "DoNow", "donow")
        .ok_or("Could not determine project directories".into())
}

fn load_tasks(path: &Path) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let mut file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) => return Ok(Vec::new()),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.trim().is_empty() {
        return Ok(Vec::new());
    }

    let tasks: Vec<Task> = from_str(&contents)?;
    Ok(tasks)
}

fn save_tasks(tasks: &Vec<Task>, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = serde_json::to_string_pretty(tasks)?;
    let mut file = fs::File::create(path)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let proj_dirs = get_project_dirs()?;
    
    let config_dir = proj_dirs.config_dir();
    std::fs::create_dir_all(config_dir)?;
    let config_path = config_dir.join("config.json");
    
    let data_dir = proj_dirs.data_dir();
    std::fs::create_dir_all(data_dir)?;
    let data_path = data_dir.join("tasks.json");

    let config = load_config(&config_path)?;
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
            if is_valid_id(&loaded_tasks, new_task.id) {
                loaded_tasks.push(new_task);
                let result = save_tasks(&loaded_tasks, &data_path);
                match result {
                    Ok(()) => println!("{}", "Operation successful".green().bold()),
                    Err(e) => eprintln!("{}", e.to_string().red().bold())
                }
            } else {
                eprintln!("{}", "ID is taken".red().bold())
            }
        },
        Command::DeleteByName { name, first } => {
            match first {
                true => delete_first_task_name(&mut loaded_tasks, &name),
                false => delete_task_name(&mut loaded_tasks, &name)
            }
            let result = save_tasks(&loaded_tasks, &data_path);
            match result {
                Ok(()) => println!("{}", "Operation successful".green().bold()),
                Err(e) => eprintln!("{}", e.to_string().red().bold())
            }
        },
        Command::DeleteByID { id } => {
            delete_task_id(&mut loaded_tasks, id);
            let result = save_tasks(&loaded_tasks, &data_path);
            match result {
                Ok(()) => println!("{}", "Operation successful".green().bold()),
                Err(e) => eprintln!("{}", e.to_string().red().bold())
            }
        },
        Command::UpdateByName { status , name, first } => {
            match first {
                true => update_first_task_name(&mut loaded_tasks, &name, &status),
                false => update_task_name(&mut loaded_tasks, &name, &status)
            }
            let result = save_tasks(&loaded_tasks, &data_path);
            match result {
                Ok(()) => println!("{}", "Operation successful".green().bold()),
                Err(e) => eprintln!("{}", e.to_string().red().bold())
            }
        },
        Command::UpdateByID { status , id} => {
            update_id(&mut loaded_tasks, id, &status);
            let result = save_tasks(&loaded_tasks, &data_path);
            match result {
                Ok(()) => println!("{}", "Operation successful".green().bold()),
                Err(e) => eprintln!("{}", e.to_string().red().bold())
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
                let result = save_tasks(&vec![], &data_path);
                match result {
                    Ok(()) => println!("{}", "Tasks cleared successfully".green().bold()),
                    Err(e) => eprintln!("{}", e.to_string().red().bold())
                }
            }
        },
        Command::EditName { id, new_name } => {
            edit_task_name(&mut loaded_tasks, id, &new_name);
            let result = save_tasks(&loaded_tasks, &data_path);
            match result {
                Ok(()) => println!("{}", "Operation successful".green().bold()),
                Err(e) => eprintln!("{}", e.to_string().red().bold())
            }
        },
        Command::EditDescription { id, new_description } => {
            edit_task_description(&mut loaded_tasks, id, &new_description);
            let result = save_tasks(&loaded_tasks, &data_path);
            match result {
                Ok(()) => println!("{}", "Operation successful".green().bold()),
                Err(e) => eprintln!("{}", e.to_string().red().bold())
            }
        }
    }

    Ok(())
}