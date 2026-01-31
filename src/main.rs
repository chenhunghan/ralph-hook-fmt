mod extract;
mod format;
mod project;

use std::io::{self, Read};

use extract::extract_file_path;
use format::format_file;

fn main() {
    // Handle --version flag
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "--version" {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    // Read JSON input from stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        print_response(true, "Failed to read input");
        return;
    }

    // Extract file path from input
    let file_path = match extract_file_path(&input) {
        Some(path) => path,
        None => {
            print_response(true, "Could not extract file path from input");
            return;
        }
    };

    // Check if file exists
    if !file_path.exists() {
        print_response(
            true,
            &format!("File does not exist: {}", file_path.display()),
        );
        return;
    }

    // Format the file
    let result = format_file(&file_path);

    // Build the response message
    let message = format!("[ralph-hook-fmt] {}", result.message);

    print_response(true, &message);
}

fn print_response(continue_execution: bool, message: &str) {
    // Escape special characters in the message for JSON
    let escaped_message = message
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");

    println!(
        r#"{{"continue":{},"systemMessage":"{}"}}"#,
        continue_execution, escaped_message
    );
}
