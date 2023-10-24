//override rust defaults for non-snake case and unused_assignments
#![allow(unused_assignments)]
#![allow(non_snake_case)]

//import io module from std for input/output operations
use std::io;

//function to calculate the fibonacci number and return its value
fn calculate_fibonacci_number(fibonacci_array: [u128; 3]) -> u128 {

        //get the numbers to be added from the fibonacci array
        let number_2: u128 = fibonacci_array[1];
        let number_3: u128 = fibonacci_array[2];
        
        //add the two numbers together to get new fibonacci number
        let fibonacci_number: u128 = number_2 + number_3;

        //return the new fibonacci number
        return fibonacci_number;   
}

fn main() {

    //set program defaults
    let mut control: bool = true;
    let mut user_input: String = String::new();

    //set main loop
    while control == true {

        //set initial state of the fibonacci numbers
        let mut fibonacci_number: u128 = 0;
        let mut first_number: u128 = 0;
        let mut second_number: u128 = 0;
        
        //inform user of options
        println!("\nA Fibonacci sequence always starts with 1, 1, and 2 as");
        println!("the first 3 numbers. Where do you want to start your ");
        println!("sequence: 1, 1, or 2?");
        println!("To begin the sequence with the first 1, enter 0.");
        println!("To start the sequence at the second 1, enter 1.");
        println!("To begin the sequence at 2, enter 2.");
        
        //create user prompt
        println!("\nSelect starting Fibonacci number: ");

        //clear user input string in preparation for user input
        user_input.clear();

        //get user input through stdin()
        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read from stdin");

        //if user selected 0, set default values to start at the first 1    
        if user_input.trim() == "0" {
            
            //set default values to start sequence at the first 1
            first_number = 1;
            second_number = 0;
            fibonacci_number = 1;

            //alert user of the beginning sequence
            println!("The sequence will start with 1, 1, 2.");
        }

        //if user selected 1, set default values to start at the second 1
        else if user_input.trim() == "1" {
            
            //set default values to start sequence at the second 1
            first_number = 0;
            second_number = 1;
            fibonacci_number = 1;
            println!("The sequence will start with 1, 2, 3.");
        }

        else if user_input.trim() == "2" {
            
            //define default values to start sequence at 2
            first_number = 1;
            second_number = 1;
            fibonacci_number = 2;
            println!("The sequence will start with 2, 3, 5.");
        }

        else {
            
            //inform user of invalid input
            println!(
                "\nInvalid input. The sequence will start with 1, 1, 2.");

            //define default sequence values
            first_number = 1;
            second_number = 0;
            fibonacci_number = 1;
        }

        //set default values for sequence generation
        user_input.clear();
        let mut user_integer: u128 = 0;
        let mut sequence_number: u32 = 0;
        let mut error_code: bool = false;
        let mut fibonacci_array: [u128; 3] = [first_number, 
            second_number, fibonacci_number];
        let max_input: usize = 38;

        //create user prompt
        println!("\nEnter an integer:");

        //get user input through stdin()
        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read from stdin");

        //trim user input to remove white spaces
        let mut parse_input = user_input.trim();

        //if user input is longer than max allowed, slice to max allowed
        if parse_input.len() > max_input {
            println!(
                "\nYou entered \"{}\" as an integer.", 
                parse_input
            );
            println!(
                "It is too large. This calcuator only supports up to e+38.");
            
            //slice the input string to max allowed length
            parse_input = &parse_input[0..max_input];
            println!("{} will be used as your integer instead.", parse_input);
        }

        //parse user input string to integer, assign to user_integer, 
        // else set error code 
        match parse_input.parse::<u128>() {
            Ok(i) => user_integer = i,
            Err(..) => error_code = true,
        };

        //if user input is not an integer, substitute default value
        if error_code == true {
            println!("\nYou entered \"{}\" as an integer.", 
                parse_input);
            println!("It is not an acceptable input.");
            user_integer = 100;
            println!("Problem parsing input fixed. Defaulting to 100.");
        };

        //verify user input is 3 or above, if not default to 3
        if user_integer < 3 {
            println!("\nYou may not enter a number below 3.");
            user_integer = 3;
            println!("Problem fixed. Defaulting to 3.");
        };

        //alert user the sequence will begin and print clean lines
        println!("\nBeginning the Fibonacci Sequence:\n");

        //fibonacci sequence loop
        //loop until the fibonacci number is greater than the user input
        while fibonacci_number <= user_integer {

            //track the sequence number
            sequence_number += 1;

            //print the sequence and value of the current fibonacci number
            println!("Sequence #{}: {}", sequence_number,
                first_number + second_number);
            
            //package all three numbers into an array
            fibonacci_array = [first_number, second_number, fibonacci_number];
            
            //shift the second number to the first number and the fibonacci 
            // number to the second number to keep track of the sequence
            first_number = second_number;
            second_number = fibonacci_number;            
            
            //call the fibonacci function to get the next fibonacci number
            fibonacci_number = calculate_fibonacci_number(fibonacci_array);
        }

        //alert user the sequence has ended and print clean lines
        println!("\nThe Fibonacci Sequence has finished.");

        //create user prompt to repeat, get user input through stdin()
        println!("\nDo you want to run another sequence?");
        println!("(enter y for yes or anythinge else to quit.)");
        
        //clear user input string in preparation for user input
        user_input.clear();
        
        //get user input through stdin()
        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read from stdin");
        
        //if the user did not enter y, say goodbye, exit the main loop
        // convert user input to lower case to cover shift lock
        // and add yes for user confusion as acceptable inputs
        if user_input.trim().to_lowercase() != "y" 
            && user_input.trim().to_lowercase() != "yes" {
            println!("\nThanks for using the Rust Fibonacci Generator!");
            println!("Exiting...\n");
            control = false;
        }
    }
}
