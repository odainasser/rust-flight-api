# Rust Flight API

This is a simple Rust API for calculating flight paths using the Axum framework.

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- Cargo (comes with Rust installation)

## Installation

1. Clone the repository:

```sh
git clone https://github.com/odainasser/rust-flight-api.git
cd rust-flight-api
```

2. Build the project:

```sh
cargo build
```

## Running the Server

To run the server, use the following command:

```sh
cargo run
```

The server will start and listen on `http://0.0.0.0:8080`.

## API Endpoints

### Calculate Flight Path

- **URL**: `/calculate`
- **Method**: `POST`
- **Content-Type**: `application/json`
- **Request Body**:

```json
[
        ["JFK", "LAX"],
        ["LAX", "SFO"],
        ["SFO", "SEA"]
]
```

- **Response**:

```json
[
    "JFK",
    "LAX",
    "SFO",
    "SEA"
]
```

## Example Request

You can use `curl` to test the endpoint:

```sh
curl -X POST http://localhost:8080/calculate \
-H "Content-Type: application/json" \
-H "Authorization: Bearer e9c8f8e7d6a5b4c3a2f1e0d9c8b7a6f5" \
-d '[
    ["JFK", "LAX"],
    ["LAX", "SFO"],
    ["SFO", "SEA"]
]'
```

## License

This project is licensed under the MIT License.