# Task Tracker CLI

A command-line task management application written in Rust for the roadmap.sh learning project.

## Features

- Add, update, and delete tasks
- Mark tasks as in-progress or done
- List all tasks or filter by status
- Persistent storage in JSON format
- Custom file path support with --input/--output arguments

## Requirements

- Rust 2024 edition
- No external dependencies (uses only native Rust modules)

## Usage

```bash
# Adding a new task
task-cli add "Buy groceries"

# Updating and deleting tasks
task-cli update 1 "Buy groceries and cook dinner"
task-cli delete 1

# Marking a task as in progress or done
task-cli mark-in-progress 1
task-cli mark-done 1

# Listing all tasks
task-cli list

# Listing tasks by status
task-cli list done
task-cli list todo
task-cli list in-progress
```

## Task Structure

Each task contains:
- `id`: Unique identifier
- `description`: Task description
- `status`: Current status (todo, in-progress, done)
- `createdAt`: Creation timestamp
- `updatedAt`: Last update timestamp

## Development

```bash
# Build the project
cargo build

# Run in development
cargo run

# Build release version
cargo build --release

# Run tests
cargo test

# Lint code
cargo clippy

# Format code
cargo fmt
```

## Implementation Details

The application stores tasks in a JSON file in the current directory (or custom path if specified). The JSON file is automatically created if it doesn't exist. Error handling is implemented for edge cases like missing files, invalid task IDs, and malformed data.