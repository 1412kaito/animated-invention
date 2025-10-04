use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Read;

//#[derive] is a Rust attribute that automatically generates code for common traits.
#[derive(Debug, Deserialize, Serialize)]
struct Task {
    id: u32,
    description: String,
    status: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
}

// explanation: return a Result type. If all-good, return a vector of Tasks. Otherwise throws an error
// Box<dyn Error> essentially means "any type of error"
fn read_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    // file handler are always mutable, even if we are just reading from it.
    let mut file = File::open("tasks.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tasks: Vec<Task> = serde_json::from_str(&contents)?;
    Ok(tasks)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: task-cli <command> [args]");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "list" => match read_tasks() {
            Ok(tasks) => {
                for task in tasks {
                    println!(
                        "ID: {} | Status: {} | Description: {}",
                        task.id, task.status, task.description
                    );
                }
            }
            Err(e) => {
                eprintln!("Error reading tasks: {}", e);
            }
        },
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
