//! This project is liscenced under the [MIT](https://mit-license.org/) license, meaning you can use this in any way, shape or form
//! you like, but without warranty of any kind.
//!  
//! A simple command-line application that reads a math expression, tokenizes and converts it to reverse-polish-notation
//! using the [Shunting-Yard algorithm](https://en.wikipedia.org/wiki/Shunting_Yard_algorithm).
//! Using the evaluate function, it evaluates the expression to an `f64` and prints it to the console.
//! This binary can be used as a simple command-line calculator.
//! 
//! It is my first ever completed project in rust as a means to develop some real world eperience.


use lib::Expression;

mod lib;

fn main() {
    loop {
        // ask for math expression
        println!("Input any valid math expression without functions.");

        // get input from stdin and trim the whitespace from left and right
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // check if user wants to quit
        if input == "exit" || input == "quit" {
            break;
        }

        // convert to math expression and return result
        let expression = Expression::new(&input);
        let result = expression.evaluate();

        println!("'{}' evaluates to: {}", expression.as_str(), result);
    }
}
