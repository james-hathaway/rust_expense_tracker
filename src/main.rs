fn main() {
    // Create a new vector to store expenses
    let mut expenses = Vec::new();

    // Loop until the user enters the 'quit' command, mimicing real world functionality
    loop {
        // Print out available commands
        println!("Enter a command:");
        println!("'add' - add an expense");
        println!("'remove' - remove an expense");
        println!("'list' - list all expenses");
        println!("'total' - show total expenses");
        println!("'quit' - quit the program");

        // Read user input and remove whitespace, like i did in wordcount project
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        // Match the command to a function
        match command {
            "add" => add_expense(&mut expenses),
            "remove" => remove_expense(&mut expenses),
            "list" => list_expenses(&expenses),
            "total" => println!("Total expenses: ${:.2}", total_expenses(&expenses)),
            "quit" => break,
            _ => println!("Invalid command"), // _ to represent anything else! super cool
        }
    }
}

// Struct to represent an expense
struct Expense {
    description: String,
    amount: f64,
}

// Function to add an expense
fn add_expense(expenses: &mut Vec<Expense>) {
    println!("Enter a description for the expense:");
    let mut description = String::new();
    std::io::stdin().read_line(&mut description).unwrap();

    println!("Enter the expense amount:");
    let mut amount = String::new();
    std::io::stdin().read_line(&mut amount).unwrap();

    // Parse the amount string to a floating point value
    let amount = amount.trim().parse::<f64>().unwrap();

    // Create a new Expense struct and add it onto the expenses data structure
    let expense = Expense {
        description: description.trim().to_string(),
        amount,
    };
    expenses.push(expense);

    println!("Expense added!");
}

// Function to remove an expense from the expenses
fn remove_expense(expenses: &mut Vec<Expense>) {
    println!("Enter the index of the expense you want to remove:");
    let mut index = String::new();
    std::io::stdin().read_line(&mut index).unwrap();


    let index = index.trim().parse::<usize>().unwrap();

    // Remove the expense from the expenses
    expenses.remove(index);

    println!("Expense removed!");
}

// Function to print out all expenses in the expenses vector
fn list_expenses(expenses: &Vec<Expense>) {
    println!("Expenses:");

    for (i, expense) in expenses.iter().enumerate() {
        println!("{}: {} - ${}", i, expense.description, expense.amount);
    }
}

// Function to calculate the total of all expenses in the expenses vector
fn total_expenses(expenses: &Vec<Expense>) -> f64 {
    let mut total = 0.0;

    for expense in expenses.iter() {
        total += expense.amount;
    }

    total
}
