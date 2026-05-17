/*
Stage1
Add bills, include the name of the bill and the amount owed.
View the list of bills, including the name and amount owed.
Stage2
Remove bills from the list when they are paid.
Stage3
Edit existing bills.
Go back if mind changes.
*/

use std::collections::HashMap;
use std::io;

// Function to read user input and return it as a String
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

// Function to add a bill
fn add_bill(bills: &mut HashMap<String, f64>) {
    let name = get_input("Enter bill name:");
    let amount_input = get_input("Enter amount owed:");

    // Converting the input into a number
    let amount: f64 = amount_input
        .parse()
        .expect("Please enter a valid number.");

    bills.insert(name, amount);

    println!("Bill added successfully!");
}

// Function to view all bills
fn view_bills(bills: &HashMap<String, f64>) {
    if bills.is_empty() {
        println!("No bills found.");
        return;
    }

    println!("\nYour Bills:");
    for (name, amount) in bills {
        println!("{}: KES {:.2}", name, amount);
    }
}
// Function to remove a bill
fn remove_bill(bills: &mut HashMap<String, f64>) {
    println!("Enter the name of the bill to remove:");
    let name = get_input("Bill name:");

    if bills.remove(&name).is_some() {
        println!("Bill removed: {}", name);
    } else {
        println!("Bill not found: {}", name);
    }
}

fn edit_bill(bills: &mut HashMap<String, f64>) {
    let name = get_input("Enter the name of the bill to edit (or type 'back' to cancel):");

    if name.to_lowercase() == "back" {
        println!("Edit cancelled.");
        return;
    }

    if let Some(amount) = bills.get_mut(&name) {
        println!("Current amount: ${:.2}", *amount);

        let input = get_input("Enter the new amount (or type 'back' to cancel):");

        if input.to_lowercase() == "back" {
            println!("Edit cancelled.");
            return;
        }

        let new_amount: f64 = input
            .parse()
            .expect("Please enter a valid number.");

        *amount = new_amount;

        println!("Bill updated: {} - ${:.2}", name, *amount);
    } else {
        println!("Bill not found: {}", name);
    }
}

// Main function
fn main() {
    let mut bills: HashMap<String, f64> = HashMap::new();

    loop {
        println!("\n==== Bills Manager ====");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Edit Bill");
        println!("5. Exit");

        let choice = get_input("Choose an option:");

        match choice.as_str() {
            "1" => add_bill(&mut bills),
            "2" => view_bills(&bills),
            "3" => remove_bill(&mut bills),
            "4" => edit_bill(&mut bills),
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Try again."),
        }
    }
}