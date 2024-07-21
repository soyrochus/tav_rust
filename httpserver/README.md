Sure, I'll add detailed documentation to the code, explaining each part and its purpose. This will make it more tutorial-like for someone learning Rust.

```rust
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::Path;

/// Reads the content of a file asynchronously and returns it as an HTTP response.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the file to be served.
///
/// # Returns
///
/// A `Result` containing a `Response<Body>` with the file contents if successful,
/// or an `std::io::Error` if the file cannot be read.
async fn serve_file(path: &str) -> Result<Response<Body>, std::io::Error> {
    let mut file = File::open(path).await?;  // Open the file asynchronously
    let mut contents = vec![];               // Create a buffer to hold the file contents
    file.read_to_end(&mut contents).await?;  // Read the file contents into the buffer
    Ok(Response::new(Body::from(contents)))  // Create and return an HTTP response with the file contents
}

/// Handles incoming HTTP requests and routes them to the appropriate handler function.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request.
///
/// # Returns
///
/// A `Result` containing a `Response<Body>` which will be sent back to the client.
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = format!(".{}", req.uri().path());  // Construct the file path from the request URI

    // Check if the requested path is a file and serve it if it is
    let response = if Path::new(&path).is_file() {
        match serve_file(&path).await {
            Ok(resp) => resp,
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Server Error"))
                .unwrap(),
        }
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap()
    };

    Ok(response)
}

#[tokio::main]
async fn main() {
    // Define the address to listen on (127.0.0.1:8080)
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // Create a service handler for incoming connections
    let make_svc = make_service_fn(|_conn| {
        async {
            // Create a service function to handle requests
            Ok::<_, Infallible>(service_fn(handle_request))
        }
    });

    // Create an HTTP server bound to the specified address
    let server = Server::bind(&addr).serve(make_svc);

    // Print the address the server is listening on
    println!("Listening on http://{}", addr);

    // Run the server and log any errors
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
```

### Explanation

#### Imports

```rust
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::Path;
```

- `hyper::service::{make_service_fn, service_fn}`: These functions help create service handlers for incoming connections.
- `hyper::{Body, Request, Response, Server, StatusCode}`: Core types and functions from the `hyper` crate to work with HTTP.
- `tokio::fs::File`: Asynchronous file handling from the `tokio` crate.
- `tokio::io::AsyncReadExt`: Provides asynchronous read methods for files.
- `std::convert::Infallible`: A type used when a function cannot fail.
- `std::net::SocketAddr`: Represents a socket address.
- `std::path::Path`: Represents file system paths.

#### Function `serve_file`

```rust
async fn serve_file(path: &str) -> Result<Response<Body>, std::io::Error> {
    let mut file = File::open(path).await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;
    Ok(Response::new(Body::from(contents)))
}
```

- Opens a file asynchronously.
- Reads the file contents into a buffer.
- Returns an HTTP response containing the file contents.

#### Function `handle_request`

```rust
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = format!(".{}", req.uri().path());

    let response = if Path::new(&path).is_file() {
        match serve_file(&path).await {
            Ok(resp) => resp,
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Server Error"))
                .unwrap(),
        }
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap()
    };

    Ok(response)
}
```

- Constructs the file path from the request URI.
- Checks if the path corresponds to a file and serves it if it exists.
- Returns a 404 Not Found response if the file does not exist.

#### Main Function

```rust
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let make_svc = make_service_fn(|_conn| {
        async {
            Ok::<_, Infallible>(service_fn(handle_request))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
```

- Defines the server address (`127.0.0.1:8080`).
- Creates a service handler using `make_service_fn` and `service_fn`.
- Binds the server to the specified address and starts listening for connections.
- Prints the listening address and logs any errors.

### Running the Server

To run the server, use the following command:

```sh
cargo run
```

This will start the server, listening on `http://127.0.0.1:8080`. You can access any file in the current directory by navigating to `http://127.0.0.1:8080/filename`.

### Summary

This tutorial demonstrates how to create a simple HTTP server in Rust using the `hyper` and `tokio` crates. The server reads and serves files from the current directory asynchronously. The provided code includes detailed documentation to help understand each part of the implementation.