use std::io::Result;
use std::fs::File;
use std::io::{BufRead, Write, BufReader, BufWriter,Lines};
use std::collections::{HashSet, HashMap};
use std::mem;
// use std::cmp::max;


// Define line ending based on OS https://stackoverflow.com/questions/47541191/how-to-get-current-platform-end-of-line-character-sequence-in-rust
#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn parse_line(line: &str, delimiter: char,in_quotes: bool) -> (Vec<String>,bool) {
    //parses a single line into fields, returning whether we are still in quotes at end of line
    let line = line.trim();
    let mut fields: Vec<String> = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = in_quotes;
    for char in line.chars(){
        if char == delimiter && !in_quotes {
            fields.push(current_field);
            current_field = String::new();
        } else if char == '"' {
            current_field.push(char); // keeps the quote as this is going into the final file as-is.
            in_quotes = !in_quotes;
        } else {
            current_field.push(char);
        }
    }
    fields.push(current_field);
    return (fields, in_quotes);//returns wheter we are still in quotes at end of line.  Concatenation is left to the caller
}

fn parse_next_line(lines: &mut Lines<BufReader<File>>, delimiter: char) -> Result<Option<Vec<String>>> {
    //helper function between parse_line which takes the lines iterator so that it can read multiple lines if needed to parse out multiline fields
    let line: String = match lines.next() {
        None => return Ok(None),
        Some(result) => result?,
    };
    let (mut fields, mut in_quotes) = parse_line(&line, delimiter, false);
    while in_quotes {//read next line, then combine last and first fields.  if still in quotes, repeat
        let next_line = lines.next().unwrap()?;
        let (next_fields, still_in_quotes) = parse_line(&next_line, delimiter, in_quotes);
        let mut last_field = fields.pop().unwrap();
        last_field.push_str(LINE_ENDING);
        last_field.push_str(&next_fields[0]);
        fields.push(last_field);
        fields.extend_from_slice(&next_fields[1..]);
        in_quotes = still_in_quotes;
    }
    // first_lines is still in scope here if we need to read more lines for multiline fields
    Ok(Some(fields))
}

pub fn combine_files_by_keys(filenames: &[&str], output_filename: &str, key_columns: Option<&[&str]>,delimiter: char,empty_field_value: &str,remove_duplicates: bool , merge_duplicates: bool) -> Result<()> {
    // Determine key columns: either from parameter or from first header

    //process first file to get ideas.
    
    
    
    let key_columns: Vec<String> = match key_columns {
        Some(cols) => cols.iter().map(|s| s.to_string()).collect(),
        None => {
            //derive from first file
            let first_file = File::open(filenames[0])?;
            let first_reader = BufReader::new(first_file);
            let mut first_lines = first_reader.lines();
            parse_next_line(&mut first_lines, delimiter)?.unwrap()
        }
    };
    //estabilsh column mapping

    let mut index_maps_by_file_index: Vec<Vec<usize>> = Vec::with_capacity(filenames.len());
    
    
    let mut output_header_vec = key_columns.clone();
    //read headers in other files to see if there are any new columns
    for &filename in filenames.iter() {
        let current_file = File::open(filename)?;
        let current_reader = BufReader::new(current_file);
        let mut current_lines = current_reader.lines();
        // let current_header = current_lines.next().unwrap()?;
        let current_header_vec = parse_next_line(&mut current_lines, delimiter)?.unwrap();
        // output_header_vec.resize(max(current_header_vec.len(),output_header_vec.len()), String::new()); //this was meant to ensure capacity but likely not needed
        index_maps_by_file_index.push(Vec::with_capacity(current_header_vec.len())); 
        let last_index = index_maps_by_file_index.len() - 1;
        let index_map: &mut Vec<usize> = &mut index_maps_by_file_index[last_index];
        // let mut new_column_index_offset: usize = 0;
        for header in current_header_vec.into_iter() {
            match output_header_vec.iter().position(|x| *x == header){
                Some(i) => { index_map.push(i); },
                None => {//new column adds to output header
                    index_map.push(output_header_vec.len());
                    output_header_vec.push(header);
                }
            }
        }
    }
    //write output header
    let output_file = File::create(output_filename)?;
    let mut output_writer = BufWriter::new(output_file);
    writeln!(output_writer, "{}", output_header_vec.join(&delimiter.to_string()))?;
    let mut seen_keys: HashSet<Vec<String>> = HashSet::new();
    let mut merged_rows: HashMap<Vec<String>, Vec<String>> = HashMap::new();
    //read data rows and write to output
    for (file_index, &filename) in filenames.iter().enumerate(){
        let current_file = File::open(filename)?;
        let current_reader = BufReader::new(current_file);
        let mut current_lines = current_reader.lines();
        let _current_header = parse_next_line(&mut current_lines, delimiter)?; //skip header
        let index_map: &Vec<usize> = &index_maps_by_file_index[file_index];
        while let Some(fields) = parse_next_line(&mut current_lines, delimiter)? {
            let mut output_fields: Vec<String> = vec![empty_field_value.to_string(); output_header_vec.len()];
            for (field_index, field) in fields.into_iter().enumerate() {
                let output_index = index_map[field_index];
                output_fields[output_index] = field;
            }
            //check for duplicates if needed
            if remove_duplicates {
                let key_fields: Vec<String> = output_fields[..key_columns.len()].to_vec();
                if seen_keys.contains(&key_fields){
                    continue;//skipping the duplicate.  also skips the write below
                } else {
                    seen_keys.insert(key_fields);
                }
            }
            if merge_duplicates {
                let key_fields: Vec<String> = output_fields[..key_columns.len()].to_vec();
                if let Some(existing_fields) = merged_rows.get_mut(&key_fields){ //found existing row to merge into
                    for i in key_columns.len()..output_fields.len() {
                        if existing_fields[i - key_columns.len()] == empty_field_value {
                            existing_fields[i - key_columns.len()] = mem::take(&mut output_fields[i]);
                        }
                    }
                } else { //new row to possibly merge into later
                    merged_rows.insert(key_fields, output_fields[key_columns.len()..].to_vec());
                }
                continue; //skip writing now, will write later
            }
            writeln!(output_writer, "{}", output_fields.join(&delimiter.to_string()))?; //write row immediately.  
        }
    }
    if merge_duplicates{ //write merged rows now
        for (key_fields, value_fields) in merged_rows.into_iter(){
            write!(output_writer, "{}", key_fields.join(&delimiter.to_string()))?;
            if !value_fields.is_empty() {
                write!(output_writer, "{}{}", delimiter, value_fields.join(&delimiter.to_string()))?;
            }
            writeln!(output_writer)?;
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_level1_same_schema() -> Result<()> {
        // Level 1: Basic merge with identical schemas
        // println!("Running test_level1_same_schema============");
        let output = "csv_testing_output/test_output_level1.csv";
        let result = combine_files_by_keys(
            &["csv_samples/employees1.csv", "csv_samples/employees2.csv"],
            output,
            None,
            ',',
            "EMPTY",
            false,
            false
        );
        
        assert!(result.is_ok(), "Combine should succeed");
        
        // TODO: Verify output has correct number of rows (header + 10 data rows)
        let output_file = File::open(output)?;
        let output_reader = BufReader::new(output_file);
        let mut line_counter: usize = 0;
        // println!("*****Output file contents:");
        for _line in output_reader.lines() {
            line_counter += 1;
            // println!("{}", _line?);
        }
        // println!("************************");
        // TODO: Verify no duplicate headers
        // println!("Total lines in output: {}", line_counter);
        assert_eq!(line_counter, 11, "Output should have 11 lines (1 header + 10 data rows)");
        
        // Cleanup
        let _ = fs::remove_file(output);
        Ok(())
    }

    #[test]
    fn test_level2_column_order_mismatch() -> Result<()>{
        // println!("Running test_level2_column_order_mismatch============");
        // Level 2: Same columns but different order
        let output = "csv_testing_output/test_output_level2.csv";
        let result = combine_files_by_keys(
            &["csv_samples/employees1.csv", "csv_samples/employees3.csv"],
            output,
            None,
            ',',
            "EMPTY",
            false,
            false
        );
        
        assert!(result.is_ok(), "Combine should succeed");
        
        // TODO: Verify columns are properly aligned
        let output_file = File::open(output)?;
        let output_reader = BufReader::new(output_file);
        let mut line_counter: usize = 0;
        // println!("*****Output file contents:");
        for _line in output_reader.lines() {
            line_counter += 1;
            // println!("{}", _line?);
        }
        assert_eq!(line_counter, 11, "Output should have 11 lines (1 header + 10 data rows)");
        // println!("************************");
        // TODO: Data should match despite different column order
        let output_reader = BufReader::new(File::open(output)?);
        let mut output_lines = output_reader.lines();
        let header = parse_next_line(&mut output_lines, ',')?.unwrap();
        let expected_header = vec!["id".to_string(), "name".to_string(), "department".to_string(), "salary".to_string()];
        assert_eq!(header, expected_header, "Header should match expected order");
        while let Some(fields) = parse_next_line(&mut output_lines, ',')? {
            assert_eq!(fields.len(), 4, "Each data row should have 4 fields");
            let id_field = &fields[0];
            assert!(id_field.parse::<i32>().is_ok(), "ID field should be an integer!");
        }
        // Cleanup
        let _ = fs::remove_file(output);
        Ok(())
    }

    #[test]
    fn test_level3_missing_columns() -> Result<()> {
        // Level 3: One file has extra column (gender)
        let output = "csv_testing_output/test_output_level3.csv";
        let result = combine_files_by_keys(
            &["csv_samples/employees1.csv", "csv_samples/employees4.csv"],
            output,
            None,
            ',',
            "EMPTY",
            false,
            false
        );
        
        assert!(result.is_ok(), "Combine should succeed");
        
        // Verify merged schema includes all columns (union)
        let output_file = File::open(output)?;
        let output_reader = BufReader::new(output_file);
        let mut output_lines = output_reader.lines();
        
        // println!("*****Output file contents:");
        // for line in &output_lines {
        //     println!("{}", line);
        // }
        // println!("************************");
        
        let header_fields = parse_next_line(&mut output_lines, ',')?.unwrap();
        assert!(header_fields.contains(&"gender".to_string()), "Header should include 'gender' column");
        assert_eq!(header_fields.len(), 5, "Header should have 5 columns (including gender)");
        
        // Verify missing values are filled with "EMPTY"
        let gender_index = header_fields.iter().position(|x| x == "gender").unwrap();
        for _ in 0..5 { // First 5 rows from employees1 (no gender)
            let fields = parse_next_line(&mut output_lines, ',')?.unwrap();
            assert_eq!(fields[gender_index], "EMPTY", "Missing gender field should be 'EMPTY'");
        }
        
        // Cleanup
        let _ = fs::remove_file(output);
        Ok(())
    }

    #[test]
    fn test_level4_quoted_fields_with_commas() -> Result<()> {
        // Level 4: Quoted fields containing commas
        let output = "csv_testing_output/test_output_level4.csv";
        let result = combine_files_by_keys(
            &["csv_samples/employees1.csv", "csv_samples/employees5.csv"],
            output,
            None,
            ',',
            "EMPTY",
            false,
            false
        );
        
        assert!(result.is_ok(), "Combine should succeed");
        
        // Verify commas within quotes are preserved
        let output_file = File::open(output)?;
        let output_reader = BufReader::new(output_file);
        let mut line_counter: usize = 0;
        // println!("*****Output file contents:");
        for _line in output_reader.lines() {
            line_counter += 1;
            // println!("{:?}", _line?);
        }
        // println!("************************");
        
        assert!(line_counter > 1, "Output should have header and data rows");
        
        // Cleanup
        let _ = fs::remove_file(output);
        Ok(())
    }

    #[test]
    fn test_level5_multiline_fields() -> Result<()> {
        // Level 5: Multiline fields within quotes (HARD)
        let output = "csv_testing_output/test_output_level5.csv";
        let result = combine_files_by_keys(
            &["csv_samples/employees1.csv", "csv_samples/employees6.csv"],
            output,
            None,
            ',',
            "EMPTY",
            false,
            false
        );
        
        assert!(result.is_ok(), "Combine should succeed");
        
        // Verify newlines within quotes are preserved
        let output_file = File::open(output)?;
        let output_reader = BufReader::new(output_file);
        let mut output_lines = output_reader.lines();
        let mut row_counter: usize = 0;
        
        // println!("*****Output file contents:");
        while let Some(_fields) = parse_next_line(&mut output_lines, ',')? {
            row_counter += 1;
            // println!("{:?}", _fields);
        }
        // println!("************************");
        
        // Verify we have data rows (header + 10 data rows)
        assert_eq!(row_counter, 11, "Output should have 11 rows (1 header + 10 data rows)");
        
        // Cleanup
        let _ = fs::remove_file(output);
        Ok(())
    }

    #[test]
    fn test_level6_incompatible_schemas() -> Result<()> {
        // Level 6: Completely different schemas
        let output = "csv_testing_output/test_output_level6.csv";
        let result = combine_files_by_keys(
            &["csv_samples/employees1.csv", "csv_samples/products.csv"],
            output,
            None,
            ',',
            "EMPTY",
            false,
            false
        );
        
        assert!(result.is_ok(), "Combine should succeed");
        
        // Verify merged schema includes all columns from both files
        let output_file = File::open(output)?;
        let output_reader = BufReader::new(output_file);
        let mut line_counter: usize = 0;
        // println!("*****Output file contents:");
        for _line in output_reader.lines() {
            line_counter += 1;
            // println!("{:?}", _line?);
        }
        // println!("************************");
        
        // Should have columns from both employees and products
        // Employees: id, name, department, salary
        // Products: id, product_name, category, price
        // Many fields should be EMPTY due to schema mismatch
        assert!(line_counter == 11, "Should have header and 10 data rows");
        
        // Cleanup
        let _ = fs::remove_file(output);
        Ok(())
    }

    #[test]
    fn test_level7_duplicate_rows() -> Result<()> {
        // Level 7: Duplicate row detection
        let output = "csv_testing_output/test_output_level7.csv";
        let result = combine_files_by_keys(
            &["csv_samples/employees1.csv", "csv_samples/employees7.csv"],
            output,
            None,
            ',',
            "EMPTY",
            true,
            false
        );
        
        assert!(result.is_ok(), "Combine should succeed");
        
        // Verify duplicate detection/handling
        // employees7 has 3 duplicates from employees1 (IDs 1, 2, 3)
        let output_file = File::open(output)?;
        let output_reader = BufReader::new(output_file);
        let mut output_lines = output_reader.lines();
        let mut row_counter: usize = 0;
        
        // println!("*****Output file contents:");
        while let Some(_fields) = parse_next_line(&mut output_lines, ',')? {
            row_counter += 1;
            // println!("{:?}", _fields);
        }
        // println!("************************");
        
        // TODO: Should we skip duplicates, keep them, or warn?
        // With deduplication on key columns (id): header + 8 unique = 9
        assert_eq!(row_counter, 9, "Output should have 9 rows with deduplication (1 header + 8 unique data rows)");
        
        // Cleanup
        let _ = fs::remove_file(output);
        Ok(())
    }

    #[test]
    fn test_multiple_files() -> Result<()> {
        // Bonus: Combine multiple files at once
        let output = "csv_testing_output/test_output_multiple.csv";
        let result = combine_files_by_keys(
            &[
                "csv_samples/employees1.csv",
                "csv_samples/employees2.csv",
                "csv_samples/employees3.csv"
            ],
            output,
            None,
            ',',
            "EMPTY",
            false,
            false
        );
        
        assert!(result.is_ok(), "Combine should succeed");
        
        // Verify all data is included
        let output_file = File::open(output)?;
        let output_reader = BufReader::new(output_file);
        let mut output_lines = output_reader.lines();
        let mut row_counter: usize = 0;
        
        // println!("*****Output file contents:");
        while let Some(_fields) = parse_next_line(&mut output_lines, ',')? {
            row_counter += 1;
            // println!("{:?}", _fields);
        }
        // println!("************************");
        
        // Should have header + 15 data rows (5 from each file)
        assert_eq!(row_counter, 16, "Output should have 16 rows (1 header + 15 data rows)");
        
        // Cleanup
        let _ = fs::remove_file(output);
        Ok(())
    }

    #[test]
    fn test_merge_duplicates() -> Result<()> {
        // Test merging rows with same key from multiple files
        let output = "csv_testing_output/test_output_merge.csv";
        let result = combine_files_by_keys(
            &[
                "csv_samples/employees1_name.csv",
                "csv_samples/employees1_department.csv",
                "csv_samples/employees1_salary.csv"
            ],
            output,
            Some(&["id"]),
            ',',
            "EMPTY",
            false,
            true
        );
        
        assert!(result.is_ok(), "Combine should succeed");
        
        // Verify merged rows reconstruct the original data
        let output_file = File::open(output)?;
        let output_reader = BufReader::new(output_file);
        let mut output_lines = output_reader.lines();
        let mut row_counter: usize = 0;
        
        // Check header
        let header = parse_next_line(&mut output_lines, ',')?.unwrap();
        assert_eq!(header.len(), 4, "Header should have 4 columns");
        assert!(header.contains(&"id".to_string()), "Header should contain 'id'");
        assert!(header.contains(&"name".to_string()), "Header should contain 'name'");
        assert!(header.contains(&"department".to_string()), "Header should contain 'department'");
        assert!(header.contains(&"salary".to_string()), "Header should contain 'salary'");
        
        // Count and verify data rows
        while let Some(fields) = parse_next_line(&mut output_lines, ',')? {
            row_counter += 1;
            assert_eq!(fields.len(), 4, "Each row should have 4 fields");
            // Verify no EMPTY values (all should be filled from merge)
            for (i, field) in fields.iter().enumerate() {
                if i == 0 { // id field
                    assert!(field.parse::<i32>().is_ok(), "ID should be an integer");
                } else {
                    assert_ne!(field, "EMPTY", "Merged fields should not be EMPTY");
                }
            }
        }
        
        // Should have 5 merged rows (one for each employee)
        assert_eq!(row_counter, 5, "Output should have 5 merged rows");
        
        // Print output file contents
        // println!("*****Output file contents:");
        // let output_file_debug = File::open(output)?;
        // let output_reader_debug = BufReader::new(output_file_debug);
        // for line in output_reader_debug.lines() {
        //     println!("{}", line?);
        // }
        // println!("************************");
        
        // Cleanup
        let _ = fs::remove_file(output);
        Ok(())
    }
}
