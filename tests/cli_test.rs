use std::process::Command;
use std::fs;
use std::path::Path;

// Helper function to run the CLI binary
fn run_cli(args: &[&str]) -> Result<std::process::Output, std::io::Error> {
    Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .args(args)
        .output()
}

// Helper to clean up test output files
fn cleanup(path: &str) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_cli_level1_same_schema() {
    let output = "csv_testing_output/test_cli_output_level1.csv";
    
    let result = run_cli(&[
        "csv_samples/employees1.csv",
        "csv_samples/employees2.csv",
        "-o", output,
    ]);
    
    assert!(result.is_ok(), "CLI command should succeed");
    let output_result = result.unwrap();
    assert!(output_result.status.success(), "Command should exit successfully");
    assert!(Path::new(output).exists(), "Output file should be created");
    
    // Verify output has correct number of lines
    let contents = fs::read_to_string(output).expect("Should read output file");
    let line_count = contents.lines().count();
    assert_eq!(line_count, 11, "Output should have 11 lines (1 header + 10 data rows)");
    
    cleanup(output);
}

#[test]
fn test_cli_level2_column_order_mismatch() {
    let output = "csv_testing_output/test_cli_output_level2.csv";
    
    let result = run_cli(&[
        "csv_samples/employees1.csv",
        "csv_samples/employees3.csv",
        "-o", output,
    ]);
    
    assert!(result.is_ok(), "CLI command should succeed");
    let output_result = result.unwrap();
    assert!(output_result.status.success(), "Command should exit successfully");
    
    let contents = fs::read_to_string(output).expect("Should read output file");
    let line_count = contents.lines().count();
    assert_eq!(line_count, 11, "Output should have 11 lines");
    
    // Check that header is correct
    let first_line = contents.lines().next().unwrap();
    assert_eq!(first_line, "id,name,department,salary", "Header should match expected order");
    
    cleanup(output);
}

#[test]
fn test_cli_level3_missing_columns() {
    let output = "csv_testing_output/test_cli_output_level3.csv";
    
    let result = run_cli(&[
        "csv_samples/employees1.csv",
        "csv_samples/employees4.csv",
        "-o", output,
        "-e", "EMPTY",
    ]);
    
    assert!(result.is_ok(), "CLI command should succeed");
    let output_result = result.unwrap();
    assert!(output_result.status.success(), "Command should exit successfully");
    
    let contents = fs::read_to_string(output).expect("Should read output file");
    let lines: Vec<&str> = contents.lines().collect();
    
    // Check header includes gender column
    assert!(lines[0].contains("gender"), "Header should include 'gender' column");
    
    // Check that first 5 rows have EMPTY for gender
    for i in 1..6 {
        assert!(lines[i].contains("EMPTY"), "Missing fields should have EMPTY value");
    }
    
    cleanup(output);
}

#[test]
fn test_cli_level7_duplicate_rows() {
    let output = "csv_testing_output/test_cli_output_level7.csv";
    
    let result = run_cli(&[
        "csv_samples/employees1.csv",
        "csv_samples/employees7.csv",
        "-o", output,
        "-e", "EMPTY",
        "--remove-duplicates",
    ]);
    
    assert!(result.is_ok(), "CLI command should succeed");
    let output_result = result.unwrap();
    assert!(output_result.status.success(), "Command should exit successfully");
    
    let contents = fs::read_to_string(output).expect("Should read output file");
    let line_count = contents.lines().count();
    assert_eq!(line_count, 9, "Output should have 9 rows with deduplication (1 header + 8 unique data rows)");
    
    cleanup(output);
}

#[test]
fn test_cli_multiple_files() {
    let output = "csv_testing_output/test_cli_output_multiple.csv";
    
    let result = run_cli(&[
        "csv_samples/employees1.csv",
        "csv_samples/employees2.csv",
        "csv_samples/employees3.csv",
        "-o", output,
    ]);
    
    assert!(result.is_ok(), "CLI command should succeed");
    let output_result = result.unwrap();
    assert!(output_result.status.success(), "Command should exit successfully");
    
    let contents = fs::read_to_string(output).expect("Should read output file");
    let line_count = contents.lines().count();
    assert_eq!(line_count, 16, "Output should have 16 rows (1 header + 15 data rows)");
    
    cleanup(output);
}

#[test]
fn test_cli_merge_duplicates() {
    let output = "csv_testing_output/test_cli_output_merge.csv";
    
    let result = run_cli(&[
        "csv_samples/employees1_name.csv",
        "csv_samples/employees1_department.csv",
        "csv_samples/employees1_salary.csv",
        "-o", output,
        "--keys", "id",
        "--merge-duplicates",
        "-e", "EMPTY",
    ]);
    
    assert!(result.is_ok(), "CLI command should succeed");
    let output_result = result.unwrap();
    assert!(output_result.status.success(), "Command should exit successfully");
    
    let contents = fs::read_to_string(output).expect("Should read output file");
    let lines: Vec<&str> = contents.lines().collect();
    
    // Check header has all 4 columns
    let header = lines[0];
    assert!(header.contains("id"), "Header should contain 'id'");
    assert!(header.contains("name"), "Header should contain 'name'");
    assert!(header.contains("department"), "Header should contain 'department'");
    assert!(header.contains("salary"), "Header should contain 'salary'");
    
    // Should have 5 merged rows + 1 header = 6 total
    assert_eq!(lines.len(), 6, "Output should have 6 lines (1 header + 5 merged rows)");
    
    // Verify no EMPTY values in data rows (all should be filled from merge)
    for i in 1..lines.len() {
        assert!(!lines[i].contains("EMPTY"), "Merged rows should not contain EMPTY values");
    }
    
    cleanup(output);
}

#[test]
fn test_cli_custom_delimiter() {
    let output = "csv_testing_output/test_cli_output_delimiter.csv";
    
    let result = run_cli(&[
        "csv_samples/employees1.csv",
        "csv_samples/employees2.csv",
        "-o", output,
        "-d", ",",
    ]);
    
    assert!(result.is_ok(), "CLI command should succeed");
    let output_result = result.unwrap();
    assert!(output_result.status.success(), "Command should exit successfully");
    
    let contents = fs::read_to_string(output).expect("Should read output file");
    let first_line = contents.lines().next().unwrap();
    assert!(first_line.contains(","), "Output should use comma delimiter");
    
    cleanup(output);
}

#[test]
fn test_cli_error_no_output() {
    let result = run_cli(&[
        "csv_samples/employees1.csv",
        "csv_samples/employees2.csv",
    ]);
    
    assert!(result.is_ok(), "Command should run");
    let output = result.unwrap();
    assert!(!output.status.success(), "Command should fail without output file");
}

#[test]
fn test_cli_error_conflicting_flags() {
    let output = "csv_testing_output/test_cli_output_conflict.csv";
    
    let result = run_cli(&[
        "csv_samples/employees1.csv",
        "csv_samples/employees2.csv",
        "-o", output,
        "--remove-duplicates",
        "--merge-duplicates",
    ]);
    
    assert!(result.is_ok(), "Command should run");
    let output_result = result.unwrap();
    assert!(!output_result.status.success(), "Command should fail with conflicting flags");
    
    cleanup(output);
}
