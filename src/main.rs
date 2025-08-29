//! omnitype - A hybrid type checker for Python and other dynamic languages.

mod ui;

use clap::Parser;
use log::LevelFilter;
use omnitype::analyzer::{AnalysisResult, Analyzer};
use omnitype::fixer::Fixer;
use omnitype::prelude::*;
use omnitype::types::TypeEnv;
use omnitype::utils::find_python_files;
use std::{io, path::PathBuf};

/// Command-line interface for omnitype.
#[derive(Parser, Debug)]
#[command(
    name = "omnitype",
    version,
    about = "A hybrid type checker for Python and other dynamic languages",
    long_about = None
)]
struct Cli {
    /// Sets the verbosity level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// Run in terminal UI mode
    #[arg(short, long)]
    tui: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Check types in the specified project
    Check {
        /// Path to the project directory or file
        path: PathBuf,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Fix type annotations in the specified project
    Fix {
        /// Path to the project directory or file
        path: PathBuf,

        /// Apply changes in-place
        #[arg(short, long)]
        in_place: bool,
    },

    /// Run the runtime type tracer
    Trace {
        /// Path to the test file or module to trace
        path: PathBuf,

        /// Test function to run (default: run all tests)
        #[arg(short, long)]
        test: Option<String>,
    },
}

fn setup_logging(level: &str) -> Result<()> {
    let log_level = match level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    env_logger::Builder::new()
        .filter_level(log_level)
        .format_timestamp(None)
        .init();

    Ok(())
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // Set up logging
    setup_logging(&cli.log_level).map_err(io::Error::other)?;

    // If no command is provided or TUI flag is set, run the TUI
    if cli.command.is_none() || cli.tui {
        let mut app = ui::App::new();
        return app.run();
    }

    // Handle command-line commands
    if let Some(command) = cli.command {
        match command {
            Commands::Check { path, format } => {
                let path_exists = std::fs::metadata(&path)
                    .map(|m| m.is_file() || m.is_dir())
                    .unwrap_or(false);
                if !path_exists {
                    eprintln!("Path not found: {:?}", path);
                    return Ok(());
                }

                let mut results: Vec<AnalysisResult> = Vec::new();
                if path.is_file() {
                    if path.extension().and_then(|e| e.to_str()) == Some("py") {
                        match Analyzer::analyze_python_file(&path) {
                            Ok(res) => results.push(res),
                            Err(e) => eprintln!("Failed to analyze {:?}: {}", path, e),
                        }
                    } else {
                        eprintln!("File is not a Python file: {:?}", path);
                    }
                } else {
                    for file in find_python_files(&path) {
                        match Analyzer::analyze_python_file(&file) {
                            Ok(res) => results.push(res),
                            Err(e) => eprintln!("Failed to analyze {:?}: {}", file, e),
                        }
                    }
                }

                let mut total_diagnostics = 0usize;
                match format.as_str() {
                    "json" => match serde_json::to_string_pretty(&results) {
                        Ok(s) => println!("{}", s),
                        Err(e) => eprintln!("Failed to serialize JSON: {}", e),
                    },
                    _ => {
                        if results.is_empty() {
                            println!("No Python files found or all analyses failed.");
                        } else {
                            for r in &results {
                                println!(
                                    "{}: functions={}, classes={}",
                                    r.path, r.function_count, r.class_count
                                );
                                for d in &r.diagnostics {
                                    println!(
                                        "  {}:{}:{}: {} {}",
                                        r.path,
                                        d.line + 1,
                                        d.column + 1,
                                        d.severity,
                                        d.message
                                    );
                                }
                                total_diagnostics += r.diagnostics.len();
                            }
                        }
                    },
                }
                if total_diagnostics > 0 {
                    std::process::exit(1);
                }
            },
            Commands::Fix { path, in_place } => {
                let fixer = Fixer::new(TypeEnv::new(), in_place);
                if let Err(e) = fixer.fix_path(&path) {
                    eprintln!("Fix failed: {}", e);
                } else {
                    println!("Fix completed{}", if in_place { " (in-place)" } else { "" });
                }
            },
            Commands::Trace { path, test } => {
                use omnitype::tracer::RuntimeTracer;
                
                let verbose = matches!(cli.log_level.to_lowercase().as_str(), "debug" | "trace");
                let mut tracer = RuntimeTracer::new(verbose);
                
                match tracer.run(&path, test.as_deref()) {
                    Ok(()) => {
                        let traces = tracer.traces();
                        
                        // Output results in a structured format
                        if !traces.variables.is_empty() || !traces.functions.is_empty() {
                            println!("Runtime tracing completed successfully!");
                            
                            if !traces.variables.is_empty() {
                                println!("\nVariable type observations:");
                                for (name, types) in &traces.variables {
                                    let unique_types: std::collections::HashSet<String> = 
                                        types.iter().map(|t| t.to_string()).collect();
                                    let type_list: Vec<String> = unique_types.into_iter().collect();
                                    println!("  {}: {}", name, type_list.join(" | "));
                                }
                            }
                            
                            if !traces.functions.is_empty() {
                                println!("\nFunction call signatures:");
                                for (name, (arg_calls, return_calls)) in &traces.functions {
                                    println!("  {}:", name);
                                    for (args, ret) in arg_calls.iter().zip(return_calls.iter()) {
                                        let arg_strs: Vec<String> = args.iter().map(|t| t.to_string()).collect();
                                        println!("    ({}) -> {}", arg_strs.join(", "), ret);
                                    }
                                }
                            }
                        } else {
                            println!("No type information collected. Make sure the Python file contains executable code.");
                        }
                    }
                    Err(e) => {
                        eprintln!("Runtime tracing failed: {}", e);
                        std::process::exit(1);
                    }
                }
            },
        }
    }

    Ok(())
}
