# Bill Manager
It is a command line application that is built on rust. It manages personal bills and expenses by ensuring one can:
 1. Add a bill with name and amount owed
 2. View all the bills.
 3. Remove the bills that have been paid from the list.
 4. Edit existing bills.
 5. Go back if one changes their mind. (exit)

 # Project Structure
 interactive_bill_manager/
├── Cargo.toml - project configuration and metadata
├── Cargo.lock - Dependency version lock file
├── README.md
├── .gitignore - prevents unnecessary files from being committed.
└── src/
    └── main.rs - Contains main project source code.

#   Data Structure Used
- Bills are stored in a  Rust hashmap.
## Key - is a string which is a "Bill name"
## Value - is a f64 which is the amount owed. "eg 3455"

# Build Commands
1. Cargo check - checks for compilation errors
2. Cargo build - builds the project 
3. cargo run- runs the project

# Sample session
==== Bills Manager ====
1. Add Bill
2. View Bills
3. Remove Bill
4. Edit Bill
5. Exit
Choose an option:
1
Enter bill name:
Water Bill
Enter amount owed:
3456
Bill added successfully!

==== Bills Manager ====
1. Add Bill
2. View Bills
3. Remove Bill
4. Edit Bill
5. Exit
Choose an option:
2

Your Bills:
Water Bill: KES 3456.00

==== Bills Manager ====
1. Add Bill
2. View Bills
3. Remove Bill
4. Edit Bill
5. Exit
Choose an option:
4
Enter the name of the bill to edit (or type 'back' to cancel):
Water Bill
Current amount: $3456.00
Enter the new amount (or type 'back' to cancel):
3589
Bill updated: Water Bill - $3589.00

==== Bills Manager ====
1. Add Bill
2. View Bills
3. Remove Bill
4. Edit Bill
5. Exit
Choose an option:
3
Enter the name of the bill to remove:
Bill name:
Water Bill
Bill removed: Water Bill

==== Bills Manager ====
1. Add Bill
2. View Bills
3. Remove Bill
4. Edit Bill
5. Exit
Choose an option:
5
Goodbye!
