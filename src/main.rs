use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use urlencoding::encode;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer);
    let request_line = request.lines().next().unwrap_or("");
    let path = request_line.split_whitespace().nth(1).unwrap_or("/");

    // Strip query parameters if they exist
    let path = path.split('?').next().unwrap_or("/");

    // Define hardcoded file contents
    let file_contents = match path {
        "/main.rs" => r#"use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer);
    let request_line = request.lines().next().unwrap_or("");
    let path = request_line.split_whitespace().nth(1).unwrap_or("/");

    // Strip query parameters if they exist
    let path = path.split('?').next().unwrap_or("/");

    // Define hardcoded file contents
    let file_contents = match path {

use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer);
    let request_line = request.lines().next().unwrap_or("");
    let path = request_line.split_whitespace().nth(1).unwrap_or("/");

    let resource_path = PathBuf::from(".").join(&path.trim_start_matches('/'));
    println!("Requested path: {:?}", resource_path);

    if resource_path.is_dir() {
        let mut html = String::new();
        html.push_str("<html><body><ul>");

        for entry in fs::read_dir(&resource_path)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            let encoded_name = encode(file_name_str.as_ref());
            let display_name = file_name_str.clone(); // Clone here to avoid moving issues
            html.push_str(&format!("<li><a href=\"/{}/\">{}</a></li>", encoded_name, display_name));
        }

        html.push_str("</ul></body></html>");
        stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n")?;
        stream.write_all(html.as_bytes())?;
    } else if !file_contents.is_empty() {
        println!("Serving hardcoded file content for path: {}", path);
        let content_type = if path.ends_with(".html") {
            "text/html"
        } else if path.ends_with(".rs") {
            "text/plain"
        } else {
            "text/html"
        };

        stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: ")?;
        stream.write_all(content_type.as_bytes())?;
        stream.write_all(b"\r\n\r\n")?;
        stream.write_all(file_contents.as_bytes())?;

    Ok(())
}


}
"#,
        "/index.html" => r#"<!DOCTYPE html>
<html>
<head>
    <title>Index Page</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            background-color: #f4f4f4;
            margin: 0;
            padding: 0;
        }
        header {
            background-color: #333;
            color: #fff;
            padding: 1em;
            text-align: center;
        }
        h1 {
            color: #333;
        }
        img {
            max-width: 100%;
            height: auto;
            display: block;
            margin: 0 auto;
        }
        .content {
            padding: 20px;
        }
        footer {
            background-color: #333;
            color: #fff;
            text-align: center;
            padding: 1em;
            position: fixed;
            bottom: 0;
            width: 100%;
        }
    </style>
</head>
<body>
    <header>
        <h1>Welcome to My Web Server</h1>
    </header>
    <div class="content">
        <h2>Index Page</h2>
        <p>This is an example HTML file with embedded CSS styling.</p>
        <img src="/image" alt="Example Image">
    </div>
    <footer>
        <p>&copy; 2024 My Web Server</p>
    </footer>
</body>
</html>"#,
        "/image" => r#"<html><body><img src="https://i.pinimg.com/736x/20/93/a8/2093a852ee076e7a4bbd0fdb65d057f4.jpg" alt="Example Image"></body></html>"#,
        "/Cargo.toml" => r#"[package]
name = "simple-file-server"
version = "0.1.0"
edition = "2021"

[dependencies]
urlencoding = "1.1"
infer = "0.11"
"#,
        _ => "",
    };

    // Determine if the path is a directory or a file
    if path == "/" {
        // Serve the directory listing
        println!("Serving directory listing");
        let mut html = String::new();
        html.push_str("<html><body><ul>");

        html.push_str(r#"<li><a href="/main.rs">main.rs</a></li>"#);
        html.push_str(r#"<li><a href="/index.html">index.html</a></li>"#);
        html.push_str(r#"<li><a href="/image">image</a></li>"#);
        html.push_str(r#"<li><a href="/Cargo.toml">Cargo.toml</a></li>"#);

        html.push_str("</ul></body></html>");
        stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n")?;
        stream.write_all(html.as_bytes())?;
    } else if !file_contents.is_empty() {
        // Serve hardcoded file contents
        println!("Serving hardcoded file content for path: {}", path);
        let content_type = if path.ends_with(".html") {
            "text/html"
        } else if path.ends_with(".rs") {
            "text/plain"
        } else if path.ends_with(".toml") {
            "text/plain"
        } else {
            "text/html"
        };

        stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: ")?;
        stream.write_all(content_type.as_bytes())?;
        stream.write_all(b"\r\n\r\n")?;
        stream.write_all(file_contents.as_bytes())?;
    } else {
        // Not found
        println!("404 Not Found for path: {}", path);
        stream.write_all(b"HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\n\r\n")?;
        stream.write_all(b"<html><body><h1>404 Not Found</h1></body></html>")?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    
    println!("Server is running on http://0.0.0.0:8080");
    println!("If using Google Cloud Shell, access your server via the web preview link:");
    println!("https://8080-dot-your-cloudshell-url-abcde.googleusercontent.com/");
    
    for stream in listener.incoming() {
        let stream = stream?;
        std::thread::spawn(move || {
            if let Err(e) = handle_client(stream) {
                eprintln!("Error handling client: {:?}", e);
            }
        });
    }

    Ok(())
}
