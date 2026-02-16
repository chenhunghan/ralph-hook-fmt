mod extract;
mod format;
mod project;

use std::env;
use std::io::{self, Read};

use extract::extract_file_path;
use format::format_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Handle --version flag
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let debug = args.iter().any(|a| a == "--debug");
    let project_only = args.iter().any(|a| a == "--project-only");

    // Read JSON input from stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        print_response(debug, true, "Failed to read input");
        return;
    }

    // Extract file path from input
    let file_path = match extract_file_path(&input) {
        Some(path) => path,
        None => {
            print_response(debug, true, "Could not extract file path from input");
            return;
        }
    };

    // Check if file exists
    if !file_path.exists() {
        print_response(
            debug,
            true,
            &format!("File does not exist: {}", file_path.display()),
        );
        return;
    }

    // Format the file
    let result = format_file(&file_path, project_only);

    // Build the response message
    let message = format!("[ralph-hook-fmt] {}", result.message);

    print_response(debug, true, &message);
}

fn escape_json(message: &str) -> String {
    message
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn print_response(debug: bool, continue_execution: bool, message: &str) {
    if continue_execution && !debug {
        println!("{{\"continue\":true}}");
        return;
    }

    let escaped_message = escape_json(message);

    if continue_execution {
        println!(
            r#"{{"continue":true,"systemMessage":"{}"}}"#,
            escaped_message
        );
    } else if debug {
        println!(
            r#"{{"decision":"block","reason":"{}","systemMessage":"{}"}}"#,
            escaped_message, escaped_message
        );
    } else {
        println!(r#"{{"decision":"block","reason":"{}"}}"#, escaped_message);
    }
}
