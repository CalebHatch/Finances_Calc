#![allow(unused)]

use std::io;
use std::fs::File;
use std::io::{Write, Read, BufReader, BufRead, ErrorKind};
use std::cmp::Ordering;

fn get_expenses(rent: i32, groceries: i32) -> i32{
    return rent + groceries
}

fn main() {
    let mut x: bool = true;

    while x == true {

        println!("What is your monthly income?");
        let mut monthly_income = String::new();
        io::stdin()
            .read_line(&mut monthly_income)
            .expect("Didn't receive input");

        println!("Your monthly income is: {}", monthly_income.trim_end());

        println!("How much do you spend on rent?");
        let mut rent = String::new();
        io::stdin()
            .read_line(&mut rent)
            .expect("Didn't receive input");

        let rent_int = rent.trim().parse::<i32>().unwrap();

        println!("How much do you spend on groceries per month?");
        let mut groceries = String::new();
        io::stdin()
            .read_line(&mut groceries)
            .expect("Didn't receive input");

        let groceries_int = groceries.trim().parse::<i32>().unwrap();

        println!("Total expenses: {}", get_expenses(rent_int, groceries_int));

        println!("Would you like to redo? (y/n)");
        let mut user_redo = String::new();
        io::stdin()
            .read_line(&mut user_redo)
            .expect("Invalid input");

        let mut user_redo = user_redo.trim();
        
        if user_redo == "y"{
            x = true;
        }
        else {
            x = false;
        }

    }
    
}
