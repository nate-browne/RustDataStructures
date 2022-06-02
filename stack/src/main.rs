use std::process::exit;
use std::io::{Write, stdout, stdin};

mod stack;
use stack::Stack;

const PROMPT: &str = "Please enter a command (is (e)mpty, (s)ize, (t)op, (p)ush, p(o)p, (q)uit): ";

fn grab_input(prompt: &str) -> String {
    print!("{prompt}");
    stdout().flush().unwrap();

    let mut option = String::new();

    match stdin().read_line(&mut option) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error occurred while reading string: {e}");
            exit(1);
        }
    };
    String::from(option.trim())
}

fn main() {

    let mut stack = Stack::<String>::new();

    loop {
        println!("");
        let input = grab_input(PROMPT);

        match input.as_str() {
            "e" => {
                let val = if stack.empty() { "is" } else { "is not" };
                println!("Stack {val} empty.");
            },
            "s" => println!("Number of elements in stack is: {}", stack.size()),
            "t" => {
                if stack.empty() {
                    println!("No element to top.");
                } else {
                    println!("Item topped is {}", stack.top());
                }
            },
            "p" => {
                let item = grab_input("Please enter string to insert: ");
                stack.push(item);
            },
            "o" => {
                if stack.empty() {
                    println!("No element to remove.");
                } else {
                    println!("Element popped was {}", stack.top());
                    stack.pop();
                }
            },
            "q" => break,
            _ => println!("Invalid command entered."),
        }
    }
}
