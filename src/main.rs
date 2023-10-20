//#![allow(unused)]

use std::io;

fn main() {

    //define default values
    let mut first_number = 1;
    let mut second_number = 0;
    let mut fibonacci_number = 1;
    let mut user_integer = 0;
    let mut error_code: bool = false;
    let mut user_input = String::new();
    let max_input = 38;

    //create user prompt
    println!("\nPlease enter an integer.");

    //get user input through stdin()
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read from stdin");

    //trim user input to remove white spaces
    let parse_input = user_input.trim();

    //exit program if user input is longer than max allowed
    if parse_input.len() > max_input {
            println!(
            "\nSystem exit.\nYou entered {} as an integer.\nIt is too large.\n", 
            parse_input);
        return;
    }

    //parse user input string to integer, assign to user_integer
    match parse_input.parse::<u128>() {
        Ok(i) => user_integer = i,
        Err(..) => error_code = true,
    };

    //if user input is not an integer, exit the program
    if error_code == true {
        println!("\nSystem exit.\nYou entered {} as an integer.\nIt is not.\n", 
            parse_input);
        return;
    };

    //alert user the sequence will begin and print clean lines
    println!("\nBeginning the Fibonacci Sequence up to: {}\n", user_integer);

    //loop until the fibonacci number is greater than the user input
    while fibonacci_number <= user_integer {

        //print the current fibonacci number
        println!("{}", first_number + second_number);
        
        //shift the second number to the first number, the old fibonacci 
        // number to the second number, and add them together to get the 
        // next fibonacci number
        first_number = second_number;
        second_number = fibonacci_number;
        fibonacci_number = first_number + second_number;
    }

    //alert user the sequence has ended and print clean lines
    println!("\nThe Fibonacci Sequence has ended.\n");
}