use clap::{Parser, Subcommand};
use std::path::PathBuf;
use anyhow::{Result, Context};

mod parser;
mod codegen;

/// Tool for generating C++ bindings for Rust functions via WebAssembly
#[derive(Parser)]
#[command(name = "wasm-interop")]
#[command(about = "WebAssembly-based language interoperability tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate C++ bindings for Rust code
    Generate {
        /// Input Rust file or directory
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output directory for generated bindings
        #[arg(short, long)]
        output: PathBuf,
        
        /// Name of the output module
        #[arg(short, long, default_value = "rustmodule")]
        name: String,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate { input, output, name } => {
            // Create output directory if it doesn't exist
            std::fs::create_dir_all(output)
                .context("Failed to create output directory")?;
            
            // Parse Rust code to extract function signatures
            let functions = parser::parse_rust_file(input)
                .context("Failed to parse Rust code")?;
            
            println!("Found {} exported functions", functions.len());
            
            // Generate C++ bindings
            codegen::generate_cpp_bindings(functions, output, name)
                .context("Failed to generate C++ bindings")?;
            
            println!("Generated C++ bindings in {:?}", output);
            
            Ok(())
        }
    }
}