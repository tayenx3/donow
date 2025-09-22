use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use colored::Colorize;

#[derive(Subcommand, Debug)]
pub enum Command {
    // Main
    Add {
        name: String, // Task name
        #[arg(short, long)]
        id: Option<usize>, // Optional force ID (will error if taken or not a valid ID)

        #[arg(short, long)]
        description: Option<String> // Optional description (default is the task's name)
    },
    DeleteByName {
        name: String,
        #[arg(short, long)]
        first: bool
    },
    DeleteByID {
        id: usize
    },
    UpdateByName {
        #[command(subcommand)]
        status: Status,

        name: String,

        #[arg(short, long)]
        first: bool
    },
    UpdateByID {
        #[command(subcommand)]
        status: Status,

        id: usize
    },

    View, // View all tasks

    // Searching
    SearchByName {
        name: String,

        #[arg(short, long)]
        first: bool
    },
    SearchByID {
        id: usize
    },

    Clear {
        #[arg(short, long)]
        force: bool
    },

    EditName {
        id: usize,
        new_name: String
    },
    EditDescription {
        id: usize,
        new_description: String
    }
}

#[derive(Subcommand, Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum Status {
    Todo,
    OnBreak,
    InProgress,
    Done
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Todo => write!(f, "{}", "Todo".yellow()),
            Self::OnBreak => write!(f, "{}", "On-break".cyan()),
            Self::InProgress => write!(f, "{}", "In-progress".blue()),
            Self::Done => write!(f, "{}", "Done".green())
        }
    }
}


#[derive(Parser)]
#[command(
    name = "donow", 
    version, 
    about = "DoNow Task Tracker - Simple task tracker for little dev homies", 
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}