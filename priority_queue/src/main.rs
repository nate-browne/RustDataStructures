use std::io::{stdin, stdout, Write};
use std::process::exit;

mod priority_queue;
use priority_queue::PriorityQueue;

const PROMPT: &str = "Enter a command ((e)mpty, (s)ize, p(u)sh, (p)op, (t)op, (q)uit): ";

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

fn convert_to_num(value: String) -> usize {
    let converted: usize = match value.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error parsing input \"{value}\": not a number.");
            exit(1);
        }
    };
    converted
}

fn main() {
    let mut min_heap: bool = false;

    let input = grab_input("Use min heap (y/n)? ");

    match input.as_str() {
        "y" => min_heap = true,
        _ => (),
    };

    let mut pq = PriorityQueue::<usize>::new(!min_heap);

    loop {
        println!("");
        let input = grab_input(PROMPT);

        match input.as_str() {
            "e" => {
                let res = if pq.empty() { "is" } else { "is not" };
                println!("Priority queue {res} empty");
            }
            "s" => println!("Priority queue has {} elements.", pq.size()),
            "u" => {
                let val = grab_input("Enter a value to push: ");
                let val = convert_to_num(val);
                pq.push(val);
                println!("Value {val} pushed.");
            }
            "p" => match pq.pop() {
                Ok(_) => println!("Top value removed"),
                Err(e) => eprintln!("PriorityQueue error: {e}"),
            },
            "t" => match pq.top() {
                Ok(val) => println!("Value at top: {val}"),
                Err(e) => eprintln!("PriorityQueue error: {e}"),
            },
            "q" => break,
            _ => println!("Command {input} is not valid."),
        }
    }
}
