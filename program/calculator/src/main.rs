// Finances Calculator
// by Caleb Hatch

#![allow(unused)]

use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::fs::File;
use std::io::{Write, Read, BufReader, BufRead, ErrorKind};
use std::cmp::Ordering;

fn get_expenses(rent: i32, groceries: i32, restaurants: i32, other: i32) -> i32{
    return rent + groceries + restaurants + other
}

fn get_yearly_income(monthly_income: i32) -> i32{
    return monthly_income * 12
}

fn main() {
    let mut x: bool = true;

    while x == true {

        println!("What is your monthly income (after taxes)?");
        let mut monthly_income = String::new();
        io::stdin()
            .read_line(&mut monthly_income)
            .expect("Didn't receive input");

        println!("Your monthly income is: {}", monthly_income.trim_end());
        println!();

        let monthly_int = monthly_income.trim().parse::<i32>().unwrap();

        // Get rent amount
        println!("How much do you spend on rent/mortgage per month (including utilities)?");
        let mut rent = String::new();
        io::stdin()
            .read_line(&mut rent)
            .expect("Didn't receive input");

        let rent_int = rent.trim().parse::<i32>().unwrap();  // Change rent input to int

        println!();

        // Get groceries amount
        println!("How much do you spend on groceries per month?");
        let mut groceries = String::new();
        io::stdin()
            .read_line(&mut groceries)
            .expect("Didn't receive input");

        println!();

        let groceries_int = groceries.trim().parse::<i32>().unwrap();  // Change groceries input to int

        println!("How much do you spend on eating out per month?");
        let mut restaurants = String::new();
        io::stdin()
            .read_line(&mut restaurants)
            .expect("Didn't receive input");

        println!();

        let rest_int = restaurants.trim().parse::<i32>().unwrap();

        println!("How much do you spend on other things per month?");
        let mut other = String::new();
        io::stdin()
            .read_line(&mut other)
            .expect("Didn't receive input");

        println!();

        let other_int = other.trim().parse::<i32>().unwrap();

        // Get percentage
        println!("What percentage of your total monthly income do you want to save? (Only input percentage number without sign)");
        let mut savings_amount = String::new();
        io::stdin()
            .read_line(&mut savings_amount)
            .expect("Didn't receive input");

        println!();

        let savings_int = savings_amount.trim().parse::<i32>().unwrap();
        let savings_amount_int = (monthly_int / savings_int);  // Get savings percentage out of monthly income

        // Get total expenses
        let total_expenses = get_expenses(rent_int, groceries_int, rest_int, other_int);

        // Get total expenses with savings
        let final_expenses = (total_expenses + savings_amount_int);

        // Calculate yearly income
        let yearly_income = get_yearly_income(monthly_int);

        // Print yearly income
        println!("Your yearly income (after taxes) is: {}", yearly_income);

        // Print total expenses
        println!("Total expenses: {}", final_expenses);  

        println!();

        if final_expenses <= monthly_int{
            println!("Your total expenses amount is less than or equal to your income. Great job!")
        } else if final_expenses > monthly_int{
            println!("Your total expenses are greater than your monthly income. Try to lower your expenses or increase your income.")
        }

        // Create HashMap and add values
        let mut expenses_map = HashMap::new();

        expenses_map.insert("Rent Total", rent);
        expenses_map.insert("Groceries Total", groceries);
        expenses_map.insert("Eating Out Total", restaurants);
        expenses_map.insert("Savings Total", savings_amount);
        expenses_map.insert("Other Expenses Total", other);

        // Ask user if they want to redo
        println!("Would you like to make a list of your expenses? (y/n)");
        let mut user_map = String::new();
        io::stdin()
            .read_line(&mut user_map)
            .expect("Invalid input");

        let mut user_map = user_map.trim();

        if user_map == "y"{
            println!();
            for(k, v) in expenses_map.iter(){
                println!("{} = {} ", k, v);
            }
        }
        else {
            println!();
        }

        // Ask user if they want to redo
        println!("Would you like to redo? (y/n)");
        let mut user_redo = String::new();
        io::stdin()
            .read_line(&mut user_redo)
            .expect("Invalid input");

        let mut user_redo = user_redo.trim();
        
        if user_redo == "y"{
            x = true;  // Keeps letting while loop be true
        }
        else {
            x = false;  // Makes while loop false
        }

        println!();

    }
    
}
