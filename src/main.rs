use clap::{Parser, Subcommand};
use todo_app::{Priority, TodoList};
use uuid::Uuid;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo
    Add {
        /// The todo text
        text: String,
        /// Priority level (low, medium, high)
        #[arg(short, long, default_value = "medium")]
        priority: String,
    },
    /// List all todos
    List,
    /// Mark a todo as complete
    Complete {
        /// The todo ID
        id: String,
    },
    /// Remove a todo
    Remove {
        /// The todo ID
        id: String,
    },
    /// Show completed todos
    Completed,
}

fn main() {
    let cli = Cli::parse();

    // TODO: Implement CLI functionality
    // Should load todos from a file (e.g., todos.json)
    // Should execute the command
    // Should save todos back to file

    println!("Todo CLI - Not yet implemented!");
    println!("Command: {:?}", cli.command);
}
