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
    
    /// Merge duplicate rows based on key columns
    #[arg(short = 'm', long)]
    merge_duplicates: bool,
    
    /// Value to use for missing columns [default: ""]
    #[arg(short = 'e', long, default_value = "", hide_default_value = true)]
    empty_value: String,
    
    /// Display license information
    #[arg(long)]
    license: bool,
}

fn main() {
    let args = Args::parse();
    
    // Handle --license flag
    if args.license {
        println!("csv_combiner is dual-licensed under:");
        println!("  - Apache License 2.0 (LICENSE-APACHE.txt)");
        println!("  - MIT License (LICENSE-MIT.txt)");
        println!("\nSee the respective license files for full terms.");
        println!("Third-party dependencies are listed in THIRD-PARTY-LICENSES.txt");
        process::exit(0);
    }
    
    // Validate that remove_duplicates and merge_duplicates are not both true
    if args.remove_duplicates && args.merge_duplicates {
        eprintln!("Error: --remove-duplicates and --merge-duplicates cannot be used together");
        process::exit(1);
    }
    
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
        args.merge_duplicates,
    );
    
    // Handle errors
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
    
    println!("Successfully combined {} files into {}", args.input_files.len(), args.output);
}
