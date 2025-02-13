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
{
    "flights": [
        ["JFK", "LAX"],
        ["LAX", "SFO"],
        ["SFO", "SEA"]
    ]
}
```

- **Response**:

```json
{
    "0": "JFK",
    "1": "LAX",
    "2": "SFO",
    "3": "SEA"
}
```

## Example Request

You can use `curl` to test the endpoint:

```sh
curl -X POST http://localhost:8080/calculate \
-H "Content-Type: application/json" \
-d '[
    ["JFK", "LAX"],
    ["LAX", "SFO"],
    ["SFO", "SEA"]
]'
```

## License

This project is licensed under the MIT License.