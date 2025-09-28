# Simple HTTP Server

A simple Rust API server built with Axum, PostgreSQL, and Docker.

## Features

- RESTful API with Axum framework
- PostgreSQL database integration
- OpenAPI documentation support (planned)
- Docker Compose setup for development
- Error handling with JSON responses

## Prerequisites

- [Rust](https://rustup.rs/) (edition 2024)
- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)

## Getting Started

### 1. Clone the Repository

```bash
git clone <repository-url>
cd simple-http-server
```

### 2. Start the Database

```bash
docker-compose up -d
```

This will start:
- PostgreSQL on port 5432
- CloudBeaver web UI on port 8978 (for database management)

### 3. Run the API Server

```bash
cargo run
```

The server will start on `http://localhost:3000`

## API Endpoints

### Hello World
- **URL:** `GET /api/v1`
- **Response:**
  ```json
  {
    "message": "Hello world!"
  }
  ```

### 404 Not Found
- **URL:** Any undefined route
- **Response:**
  ```json
  {
    "error": "Route not found"
  }
  ```

## Database Access

### CloudBeaver Web UI
Access the database web interface at: http://localhost:8978

### Connection Details
- **Host:** localhost
- **Port:** 5432
- **Database:** simple_api
- **Username:** lino
- **Password:** lino

## Project Structure

```
├── src/
│   └── main.rs              # Main application entry point
├── Cargo.toml              # Rust dependencies
├── docker-compose.yml      # Docker services configuration
└── README.md              # This file
```

## Dependencies

- `tokio` - Async runtime
- `axum` - Web framework
- `serde` - JSON serialization
- `serde_json` - JSON handling

## Development

### Running Tests

```bash
cargo test
```

### Building for Production

```bash
cargo build --release
```

### Linting

```bash
cargo clippy
```

### Formatting

```bash
cargo fmt
```

## Stopping the Services

```bash
docker-compose down
```

To remove volumes (data):
```bash
docker-compose down -v
```

## Future Enhancements

- [ ] OpenAPI/Swagger documentation
- [ ] Database models and queries
- [ ] Authentication middleware
- [ ] Configuration management
- [ ] Logging and monitoring
- [ ] Health check endpoint