# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a task tracker CLI application written in Rust for a roadmap.sh learning project. The application manages tasks through command-line interface and stores data in a JSON file.

## Development Commands

- Build the project: `cargo build`
- Run in development: `cargo run`
- Build release: `cargo build --release`
- Test: `cargo test`
- Lint: `cargo clippy`
- Format: `cargo fmt`

## Architecture

The project is a basic Rust CLI application that needs to implement:

- Task management (add, update, delete, mark status)
- JSON file storage for tasks
- Command-line argument parsing
- Task status tracking (todo, in-progress, done)

## Requirements and Constraints

- No external dependencies allowed (no cargo installs)
- Must use native Rust file system modules
- JSON storage file created automatically if doesn't exist
- Support optional --input/--output arguments for file paths
- Handle errors and edge cases gracefully
- Use positional arguments for user inputs

## Task Data Structure

Each task must contain:
- id: unique identifier
- description: task description
- status: todo/in-progress/done
- createdAt: creation timestamp
- updatedAt: last update timestamp

## CLI Commands

The application should support these commands:
- `task-cli add "description"` - Add new task
- `task-cli update <id> "description"` - Update task
- `task-cli delete <id>` - Delete task
- `task-cli mark-in-progress <id>` - Mark as in progress
- `task-cli mark-done <id>` - Mark as done
- `task-cli list` - List all tasks
- `task-cli list <status>` - List by status (done/todo/in-progress)