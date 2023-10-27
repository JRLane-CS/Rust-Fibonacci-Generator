# Rust Fibonacci Generator

## Overview

### Purpose

The Rust Fibonacci Generator was written so I could familiarize myself with the Rust language. I wanted to develop a simple application that would allow a user to select where a Fibonacci sequence would start, set an integer limit for the sequence, and display the result in the console. The idea was to familiarize myself with Rust to the point where I could create a functional program showcasing the language.

### Operation

Upon starting the application from the command line, the program explains that the Fibonacci sequence always begins with the sequence of 1, 1, 2. It then prompts the user to select a starting point for the sequence. Once the user selects a starting number, program asks the user to select an integer. This is used as the limit for the Fibonacci sequence. There are many error handling conditionals contained within the program which automatically default to safe values in case of erroneous user input. Once the sequence has been displayed, the user is prompted as to whether another sequence should be run. If the user selects y, Y, yes, or YES the program will repeat from the point of requesting where the user would like the sequence to begin.

## Demo Video

[Rust Fibonacci Generator Video Demo](https://youtu.be/B5fdefOct6I)

## Development Environment

The Rust Fibonacci Generator was developed using Microsoft's Visual Studio Code with the following extensions: Even Better TOML, Rust Extension Pack, Rust Test Explorer, rust-analyzer, Error Lens, crates, and CodeLLDB.

The Rust programming language was used to create this application and only the Rust standard library was utilized.  

## Useful Websites

- [Rust Tutorial Full Course](https://www.youtube.com/watch?v=ygL_xcavzQ4&t=78s)
- [Rust Language Documentation](https://doc.rust-lang.org/std/index.html)
- [The Rust Reference](https://doc.rust-lang.org/reference/introduction.html)
- [How to Accept an Input on One Line](https://users.rust-lang.org/t/how-to-accept-an-input-on-one-line-with-the-text/18976)
- [Intro to Rust Lang (Slices)](https://www.youtube.com/watch?v=yP8N3dd_Kd4)

## Potential Future Work

Some future work may include the following:

- Change the operation from console to a user interface
- Either remove the calculate_Fibonacci_number function or alter the main loop to remove redundant code
- Create a function to handle the repetative code for sequence start value
