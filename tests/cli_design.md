# CLI Design for CSV Combiner

## Basic Usage

```bash
# Simplest case: combine two files
csv_combiner file1.csv file2.csv -o output.csv

# Combine multiple files with glob pattern
csv_combiner csv_samples/*.csv -o combined.csv
```

## Options

### Output File
```bash
# Using -o or --output
csv_combiner file1.csv file2.csv -o output.csv
csv_combiner file1.csv file2.csv --output output.csv
```

### Delimiter
```bash
# Default is comma, but can specify others
csv_combiner file1.tsv file2.tsv -o output.tsv --delimiter "\t"
csv_combiner file1.csv file2.csv -o output.csv -d ","
csv_combiner file1.csv file2.csv -o output.csv --delimiter ";"
```

### Key Columns (for deduplication)
```bash
# Specify which columns form the unique key
csv_combiner file1.csv file2.csv -o output.csv --keys id
csv_combiner file1.csv file2.csv -o output.csv --keys id,name
csv_combiner file1.csv file2.csv -o output.csv -k id,email

# If not specified, uses all columns from first file's header as keys
```

### Remove Duplicates
```bash
# Enable duplicate detection and removal (keeps first occurrence)
csv_combiner file1.csv file2.csv -o output.csv --remove-duplicates
csv_combiner file1.csv file2.csv -o output.csv --keys id --remove-duplicates
csv_combiner file1.csv file2.csv -o output.csv -r

# Note: When duplicates are found based on key columns, the first instance
# encountered is kept and subsequent duplicates are skipped
```

### Empty Field Value
```bash
# Specify what to use for missing columns (default: empty string "")
csv_combiner file1.csv file2.csv -o output.csv --empty-value "N/A"
csv_combiner file1.csv file2.csv -o output.csv --empty "NULL"
csv_combiner file1.csv file2.csv -o output.csv -e "EMPTY"
```

## Complete Examples

```bash
# Minimal - just combine files with defaults
csv_combiner employees1.csv employees2.csv -o combined.csv

# With deduplication on ID column
csv_combiner employees*.csv -o all_employees.csv --keys id --remove-duplicates

# Custom delimiter (TSV) with custom empty value
csv_combiner data1.tsv data2.tsv -o merged.tsv -d "\t" -e "MISSING"

# All options
csv_combiner csv_samples/employees*.csv \
    --output results/merged.csv \
    --delimiter "," \
    --keys id,email \
    --remove-duplicates \
    --empty-value "N/A"
```

## Help/Version
```bash
csv_combiner --help
csv_combiner -h
csv_combiner --version
csv_combiner -V
```

### Expected Help Output

```
csv_combiner 0.1.0
Combine multiple CSV files with different schemas into a single output file

USAGE:
    csv_combiner [OPTIONS] --output <FILE> <INPUT_FILES>...

ARGS:
    <INPUT_FILES>...    Input CSV files to combine (at least one required)

OPTIONS:
    -o, --output <FILE>              Output file path (required)
    -d, --delimiter <CHAR>           Field delimiter character [default: ,]
    -k, --keys <COLUMNS>             Key columns for deduplication (comma-separated)
                                     [default: all columns from first file's header]
    -r, --remove-duplicates          Remove duplicate rows based on key columns (keeps first)
    -e, --empty-value <STRING>       Value to use for missing columns [default: ""]
    -h, --help                       Print help information
    -V, --version                    Print version information

EXAMPLES:
    # Basic merge
    csv_combiner file1.csv file2.csv -o merged.csv

    # With deduplication on specific columns
    csv_combiner *.csv -o output.csv --keys id,email --remove-duplicates

    # Custom delimiter and empty value
    csv_combiner data1.tsv data2.tsv -o merged.tsv -d "\t" -e "N/A"
```

## Error Cases

```bash
# No input files
csv_combiner -o output.csv
# Error: At least one input file required

# Output file not specified
csv_combiner file1.csv file2.csv
# Error: Output file required (use -o or --output)

# Input file doesn't exist
csv_combiner missing.csv other.csv -o output.csv
# Error: Input file 'missing.csv' not found

# Invalid delimiter
csv_combiner file1.csv file2.csv -o output.csv -d "abc"
# Error: Delimiter must be a single character
```

## Implementation Notes

- Input files: positional arguments (all non-flag arguments are input files)
- Output file: required, specified via `-o` or `--output`
- Delimiter: single character, default `,`
- Key columns: comma-separated list, defaults to all columns from first file
- Remove duplicates: flag, default false; when enabled, keeps first occurrence of each unique key
- Empty value: string, default `""` (empty string)
