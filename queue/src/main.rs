use std::process::exit;
use std::io::{Write, stdout, stdin};

mod queue;
use queue::Queue;

const PROMPT: &str = "Please enter a command (is (e)mpty, (s)ize, (f)ront, (b)ack, (p)ush, p(o)p, (q)uit): ";

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
    let mut queue = Queue::<String>::new();

    loop {
        println!("");

        let input = grab_input(PROMPT);

        match input.as_str() {
            "e" => {
                let res = if queue.empty() { "is" } else { "is not" };
                println!("Queue {res} empty.");
            },
            "s" => println!("Number of elements in the queue is: {}", queue.size()),
            "f" => {
                if queue.empty() {
                    println!("No element at front.");
                } else {
                    println!("Element at front of queue: {}", queue.front());
                }
            },
            "b" => {
                if queue.empty() {
                    println!("No element at back.");
                } else {
                    println!("Element at back of queue: {}", queue.back());
                }
            },
            "p" => {
                let item = grab_input("Please enter string to insert: ");
                queue.push(item);
            },
            "o" => {
                println!("Element removed from front: {}", queue.front());
                queue.pop();
            },
            "q" => break,
            _ => println!("Invalid input given."),
        }
    }
}
