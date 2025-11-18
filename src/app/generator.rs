use rand::Rng;
use rand::seq::IndexedRandom;

use crate::types::Buffer;

const FN_NAMES: &[&str] = &[
    "process",
    "calculate",
    "update",
    "render",
    "initialize",
    "finalize",
    "handle_event",
    "load_data",
    "save_data",
    "compute_result",
    "transform",
    "validate",
    "parse_input",
    "generate_report",
    "send_request",
    "receive_response",
];

const VAR_NAMES: &[&str] = &[
    "data", "result", "temp", "index", "value", "count", "item", "buffer", "config", "status",
    "input", "output", "flag", "message", "response", "request", "user", "session", "state",
];

const TYPE_NAMES: &[&str] = &[
    "i32",
    "u32",
    "f64",
    "String",
    "Vec<u8>",
    "HashMap<String, String>",
    "Option<String>",
    "Result<i32, Error>",
    "bool",
    "&str",
    "Box<dyn Trait>",
    "Vec<String>",
    "HashSet<i32>",
];

const STRUCT_NAMES: &[&str] = &[
    "Config", "User", "Request", "Response", "Session", "State", "Data", "Message", "Event",
    "Handler", "Manager", "Service", "Client",
];

const IMPORTS: &[&str] = &[
    "use std::io;",
    "use std::fs::File;",
    "use std::collections::HashMap;",
    "use std::time::Duration;",
    "use std::thread;",
    "use rand::Rng;",
    "use serde::{Serialize, Deserialize};",
    "use tracing::info;",
    "use tokio::sync::mpsc;",
    "use futures::future::join_all;",
    "use anyhow::{Result, Context};",
    "use chrono::{DateTime, Utc};",
    "use log::{debug, error, warn};",
    "use crossbeam::channel::{unbounded, Sender, Receiver};",
    "use serde_json::json;",
    "use std::fmt;",
];

/// Generates a random Rust code buffer, almost definitely won't compile :)
pub fn generate_random_rust_code_buffer() -> Buffer {
    let mut rng = rand::rng();
    let mut buffer = Buffer::default();

    // Add some imports
    let import_count = rng.random_range(3..7);
    let mut used_imports = vec![];

    for _ in 0..import_count {
        let import = IMPORTS.choose(&mut rng).unwrap();
        if !used_imports.contains(import) {
            buffer.push_line(import.to_string());
            used_imports.push(import);
        }
    }
    buffer.push_line("".to_string());

    // Generate some structs
    let struct_count = rng.random_range(1..4);
    for _ in 0..struct_count {
        let lines = random_struct_definition();
        for line in lines {
            buffer.push_line(line);
        }
        buffer.push_line("".to_string());
    }

    // Start main function
    buffer.push_line("fn main() {".to_string());

    // Generate some random code blocks with proper indentation
    let block_count = rng.random_range(10..20);
    for _ in 0..block_count {
        let block_type = rng.random_range(1..=10);
        match block_type {
            1 => {
                // Generate a random match block
                let lines = random_match_block();
                for line in lines {
                    buffer.push_line(format!("    {}", line));
                }
            }
            2 => {
                // Generate a random for loop
                let lines = random_for_loop();
                for line in lines {
                    buffer.push_line(format!("    {}", line));
                }
            }
            3 => {
                // Generate a random while loop
                let lines = random_while_loop();
                for line in lines {
                    buffer.push_line(format!("    {}", line));
                }
            }
            4..=7 => {
                // Generate a random function call line
                let line = random_one_line_function_call();
                buffer.push_line(format!("    {}", line));
            }
            8..=10 => {
                // Generate a random let statement
                let line = random_let();
                buffer.push_line(format!("    {}", line));
            }
            _ => {}
        }

        let should_have_empty_line = rng.random_bool(0.7);
        if should_have_empty_line {
            buffer.push_line("".to_string());
        }
    }

    // Close main function
    buffer.push_line("}".to_string());

    buffer
}

fn random_struct_definition() -> Vec<String> {
    let mut rng = rand::rng();
    let mut lines = vec![];

    let struct_name = STRUCT_NAMES.choose(&mut rng).unwrap();
    lines.push(format!("struct {} {{", struct_name));

    let field_count = rng.random_range(2..5);
    for _ in 0..field_count {
        let field_name = VAR_NAMES.choose(&mut rng).unwrap();
        let field_type = TYPE_NAMES.choose(&mut rng).unwrap();
        lines.push(format!("    {}: {},", field_name, field_type));
    }

    lines.push("}".to_string());
    lines
}

/// Generate a random sudo code block with pattern matching
fn random_match_block() -> Vec<String> {
    let mut rng = rand::rng();
    let mut lines = vec![];

    let arms = rng.random_range(2..5);
    lines.push("match input {".to_string());
    for _ in 0..arms {
        let pattern = VAR_NAMES.choose(&mut rng).unwrap();
        let action = random_one_line_function_call();
        lines.push(format!("    {} => {{ {} }},", pattern, action));
    }

    let no_match_action = random_one_line_function_call();
    lines.push(format!("    _ => {{ {} }},", no_match_action));
    lines.push("}".to_string());
    lines
}

fn random_let() -> String {
    let mut rng = rand::rng();
    let var_name = VAR_NAMES.choose(&mut rng).unwrap();
    let fn_name = FN_NAMES.choose(&mut rng).unwrap();
    format!("let {} = {}::{};", var_name, var_name, fn_name)
}

fn random_while_loop() -> Vec<String> {
    let mut rng = rand::rng();
    let mut lines = vec![];

    let var = VAR_NAMES.choose(&mut rng).unwrap();
    let var2 = VAR_NAMES.choose(&mut rng).unwrap();
    lines.push(format!("while {}.{}() {{", var, var2));

    let statement_count = rng.random_range(1..5);
    for _ in 0..statement_count {
        let is_let = rng.random_bool(0.5);
        let stmt = if is_let {
            random_let()
        } else {
            random_one_line_function_call()
        };

        lines.push(format!("    {}", stmt));
    }

    // Always add a function call at the end
    lines.push(format!("    {}", random_one_line_function_call()));
    lines.push("}".to_string());

    lines
}

/// Generate a random for loop block
fn random_for_loop() -> Vec<String> {
    let mut rng = rand::rng();
    let mut lines = vec![];

    let var = VAR_NAMES.choose(&mut rng).unwrap();
    lines.push(format!("for {} in {}s {{", var, var));

    // Add 1-3 statements inside the loop
    let statement_count = rng.random_range(1..4);
    for _ in 0..statement_count {
        let function_call = random_one_line_function_call();
        lines.push(format!("    {}", function_call));
    }

    lines.push("}".to_string());

    lines
}

/// Generate a random one-line function call
fn random_one_line_function_call() -> String {
    let mut rng = rand::rng();
    let fn_name = FN_NAMES.choose(&mut rng).unwrap();
    let arg_count = rng.random_range(0..4);
    let mut args = vec![];
    for _ in 0..arg_count {
        let arg = VAR_NAMES.choose(&mut rng).unwrap();
        args.push(arg.to_string());
    }
    format!("{}({});", fn_name, args.join(", "))
}

#[cfg(test)]
mod generator_tests {
    use super::*;

    #[test]
    fn test_generate_random_rust_code() {
        let buffer = generate_random_rust_code_buffer();
        println!("{}", buffer);
    }

    #[test]
    fn test_match_block_generation() {
        let lines = random_match_block();
        for line in lines {
            println!("{}", line);
        }
    }

    #[test]
    fn test_while_loop_generation() {
        let lines = random_while_loop();
        for line in lines {
            println!("{}", line);
        }
    }

    #[test]
    fn test_for_loop_generation() {
        let lines = random_for_loop();
        for line in lines {
            println!("{}", line);
        }
    }

    #[test]
    fn test_let_statement_generation() {
        let line = random_let();
        println!("{}", line);
    }

    #[test]
    fn test_function_call_generation() {
        let line = random_one_line_function_call();
        println!("{}", line);
    }

    #[test]
    fn test_struct_definition_generation() {
        let lines = random_struct_definition();
        for line in lines {
            println!("{}", line);
        }
    }
}
