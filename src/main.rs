mod csv_combining;

use clap::Parser;
use std::process;

/// Combine multiple CSV files with different schemas into a single output file
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input CSV files to combine (at least one required)
    #[arg(required = true)]
    input_files: Vec<String>,
    
    /// Output file path
    #[arg(short = 'o', long)]
    output: String,
    
    /// Field delimiter character
    #[arg(short = 'd', long, default_value = ",")]
    delimiter: char,
    
    /// Key columns for deduplication (comma-separated)
    #[arg(short = 'k', long, value_delimiter = ',')]
    keys: Option<Vec<String>>,
    
    /// Remove duplicate rows based on key columns (keeps first)
    #[arg(short = 'r', long)]
    remove_duplicates: bool,
    
    /// Value to use for missing columns [default: ""]
    #[arg(short = 'e', long, default_value = "", hide_default_value = true)]
    empty_value: String,
}

fn main() {
    let args = Args::parse();
    
    // Convert input_files Vec<String> to Vec<&str>
    let input_refs: Vec<&str> = args.input_files.iter().map(|s| s.as_str()).collect();
    
    // Convert keys Option<Vec<String>> to Option<Vec<&str>>
    let keys_refs: Option<Vec<&str>> = args.keys.as_ref()
        .map(|v| v.iter().map(|s| s.as_str()).collect());
    
    // Call the combining function
    let result = csv_combining::combine_files_by_keys(
        &input_refs,
        &args.output,
        keys_refs.as_deref(),
        args.delimiter,
        &args.empty_value,
        args.remove_duplicates,
    );
    
    // Handle errors
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
    
    println!("Successfully combined {} files into {}", args.input_files.len(), args.output);
}
