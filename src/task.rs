use crate::cli::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::fmt;
use colored::Colorize;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    pub id: usize,
    pub status: Status,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {} - {}\n | Created at: {}\n | Last updated: {}\n | Status: {}\n | Description: {}",
            self.id,
            self.name.cyan().bold(),
            self.created_at.to_string().cyan(),
            self.updated_at.to_string().cyan(),
            self.status.to_string().bold(),
            self.description.cyan()
        )
    }
}

impl Task {
    pub fn new(name: String, id: usize, status: Status, description: String, created_at: DateTime<Utc>) -> Self {
        Task {
            name,
            id,
            status,
            description,
            created_at,
            updated_at: created_at
        }
    }
}

pub fn determine_id(tasks: &[Task]) -> usize {
    let mut candidate: usize = 1;
    while tasks.iter().any(|task| task.id == candidate) {
        candidate += 1;
    }

    candidate
}

pub fn delete_task_name(tasks: &mut Vec<Task>, target: &str) {
    tasks.retain(|task| task.name != target)
}

pub fn delete_first_task_name(tasks: &mut Vec<Task>, target: &str) {
    
    if let Some(index) = tasks.iter_mut().position(|task| task.name == target) {
        tasks.drain(index..=index);
    }
}

pub fn delete_task_id(tasks: &mut Vec<Task>, target: usize) {
    tasks.retain(|task| task.id != target)
}

pub fn search_task_name(tasks: &Vec<Task>, target: &str) -> Vec<Task> {
    tasks.iter()
        .filter(|task| task.name == target)
        .cloned()
        .collect()
}

pub fn search_first_task_name(tasks: &Vec<Task>, target: &str) -> Option<Task> {
    tasks.iter()
        .find(|task| task.name == target)
        .cloned()
}

pub fn search_id(tasks: &Vec<Task>, target: usize) -> Option<Task> {
    tasks.iter()
        .find(|task| task.id == target)
        .cloned()
}

pub fn update_task_name(tasks: &mut Vec<Task>, target: &str, new_status: &Status) {
    tasks
    .iter_mut()
    .for_each(|task| {
        if task.name == target {
            task.status = new_status.clone();
            task.updated_at = Utc::now();
        }
    });
}

pub fn update_first_task_name(tasks: &mut Vec<Task>, target: &str, new_status: &Status) {
    if let Some(task) = tasks.iter_mut().find(|task| task.name == target) {
        task.status = new_status.clone();
        task.updated_at = Utc::now();
    }
}

pub fn update_id(tasks: &mut Vec<Task>, target: usize, new_status: &Status) {
    tasks
    .iter_mut()
    .for_each(|task| {
        if task.id == target {
            task.status = new_status.clone();
            task.updated_at = Utc::now();
        }
    });
}

pub fn edit_task_name(tasks: &mut Vec<Task>, target: usize, new_name: &str) {
    if let Some(task) = tasks
    .iter_mut()
    .find(|cand| cand.id == target) {
        task.name = new_name.to_string();
        task.updated_at = Utc::now();
    }
}

pub fn edit_task_description(tasks: &mut Vec<Task>, target: usize, new_description: &str) {
    if let Some(task) = tasks
    .iter_mut()
    .find(|cand| cand.id == target) {
        task.description = new_description.to_string();
        task.updated_at = Utc::now();
    }
}