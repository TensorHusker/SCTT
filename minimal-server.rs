use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("🚀 SCTT Server running at http://localhost:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");

    let (status_line, content) = if request_line.starts_with("GET / ") {
        ("HTTP/1.1 200 OK", get_index_html())
    } else if request_line.starts_with("GET /api/health") {
        ("HTTP/1.1 200 OK", r#"{"status": "ok", "service": "SCTT"}"#.to_string())
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404 Not Found".to_string())
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_index_html() -> String {
    fs::read_to_string("static/index.html")
        .unwrap_or_else(|_| r#"
<!DOCTYPE html>
<html>
<head>
    <title>SCTT</title>
    <style>
        body { font-family: system-ui; margin: 40px; }
        h1 { color: #6366f1; }
        .container { max-width: 800px; margin: 0 auto; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Smooth Cubical Type Theory</h1>
        <p>Welcome to SCTT - Where Mathematics Meets Business</p>
        <h2>Quick Start</h2>
        <pre>
// Define smooth functions
smooth f : C∞(ℝ, ℝ)
f x = sin(x) * exp(-x²/2)
        </pre>
        <h2>Business Opportunities</h2>
        <ul>
            <li>Proof Verification as a Service</li>
            <li>Academic Research Platform</li>
            <li>Blockchain Smart Contract Verification</li>
            <li>AI Model Safety Guarantees</li>
        </ul>
    </div>
</body>
</html>
        "#.to_string())
}