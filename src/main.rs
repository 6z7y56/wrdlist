mod generator;
mod cli;
mod utils;

use clap::{CommandFactory, Parser};
use cli::Cli;
use generator::WordlistGenerator;
use utils::{estimate_size, format_file_size};
use std::io;

fn main() {
    // Special handling for help and version flags
    let args: Vec<String> = std::env::args().collect();
    
    // Check if no arguments were provided
    if args.len() == 1 {
        println!("For more information, try '--help'.");
        return;
    }
    
    // Handle help flags explicitly
    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        let mut cmd = Cli::command();
        cmd.print_help().expect("Failed to print help");
        return;
    }
    
    // Handle version flag explicitly
    if args.len() == 2 && (args[1] == "--version" || args[1] == "-v") {
        println!("wrdlist version 0.0.5");
        return;
    }

    // Parse arguments for normal operation
    let cli = match Cli::try_parse() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // Generate the wordlist
    match WordlistGenerator::new(&cli.pattern) {
        Ok(generator) => {
            // Estimate the size of the wordlist
            if let Some(_output_path) = &cli.output {
                match estimate_size(&generator.tokens) {
                    Ok(size) => {
                        let avg_word_size = 10; // Average size of each word in bytes
                        let expected_file_size = size * avg_word_size;
                        let formatted_size = format_file_size(expected_file_size);

                        println!("Expected file size: {}", formatted_size);

                        if expected_file_size > 100_000 {
                            println!("Warning: This operation will generate a large wordlist.");
                        }

                        println!("Do you want to save the wordlist to the file? (y/n)");
                        let mut response = String::new();
                        io::stdin().read_line(&mut response).expect("Failed to read line");

                        if response.trim().to_lowercase() != "y" {
                            println!("Operation cancelled.");
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }

            // Generate and handle the wordlist
            let mut wordlist = generator.generate();
            if cli.random {
                generator.shuffle(&mut wordlist);
            } else if cli.inverse {
                generator.reverse(&mut wordlist);
            }

            // Handle output (print or save to file)
            if let Some(path) = cli.output {
                if let Err(e) = generator::save_to_file(wordlist, &path) {
                    eprintln!("Error saving file: {}", e);
                    std::process::exit(1);
                } else {
                    println!("Wordlist saved to {}", path);
                }
            } else {
                for word in wordlist {
                    println!("{}", word);
                }
            }
        }
        Err(e) => {
            eprintln!("Syntax Error: {}", e);
            std::process::exit(1);
        }
    }
}
