CSV Sample Files Summary
========================

employees1.csv
- Columns: id, name, department, salary
- Standard baseline format
- 5 employees (IDs 1-5)

employees2.csv
- Columns: id, name, department, salary
- Standard format (matches employees1)
- 5 employees (IDs 6-10)

employees3.csv
- Columns: name, id, salary, department
- DIFFERENT COLUMN ORDER than employees1
- Same fields, just reordered
- 5 employees (IDs 11-15)

employees4.csv
- Columns: id, name, gender, department, salary
- EXTRA COLUMN (gender) not in other employee files
- Gender inserted between name and department
- 5 employees (IDs 16-20)

employees5.csv
- Columns: id, name, department, salary, address
- QUOTED FIELDS with commas inside (e.g., "Johnson, Sarah")
- Addresses contain commas within quotes
- ESCAPED QUOTES (doubled: "" for literal ")
- Tests CSV RFC 4180 quoting rules
- 5 employees (IDs 21-25)

employees6.csv
- Columns: id, name, department, salary, notes
- MULTILINE FIELDS within quotes (newlines in notes column)
- Combination of newlines AND escaped quotes
- Tests proper quoted field parsing across line boundaries
- Will break naive line-by-line readers
- 5 employees (IDs 26-30)

employees7.csv
- Columns: id, name, department, salary
- DUPLICATE ENTRIES from employees1.csv (IDs 1, 2, 3)
- Mixed with new entries (IDs 31-33)
- Tests deduplication logic
- Should detect/handle repeated rows
- 6 employees total (3 duplicates, 3 new)

products.csv
- Columns: sku, product_name, category, price, stock
- COMPLETELY DIFFERENT SCHEMA from employee files
- 5 productsemployees7 = duplicate row detection
- employees1 + 

Test Cases (increasing difficulty):
- employees1 + employees2 = easy merge (same schema)
- employees1 + employees3 = column order mismatch
- employees1 + employees4 = missing column (gender)
- employees1 + employees5 = quoted fields with commas
- employees1 + employees6 = multiline fields (HARD - breaks simple parsers)
- employees1 + products = incompatible schemas
