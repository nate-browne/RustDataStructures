use std::process::exit;
use std::io::{Write, stdout, stdin};

mod avl;
use avl::AVL;

const PROMPT: &str = "Enter a command ((c)ontains, i(s)_empty, (e)mpty, (i)nsert, (r)emove, (p)rint, (f)ind_min, find_(m)ax, (q)uit): ";

fn grab_input(prompt: &str) -> String {
    print!("{prompt}");
    stdout().flush().unwrap();

    let mut option = String::new();

    match stdin().read_line(&mut option) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error occurred while reading string: {e}");
            exit(1);
        }
    };
    String::from(option.trim())
}

fn main() {
    let mut tree: AVL<String> = AVL::new();

    loop {
        let input = grab_input(PROMPT);

        match input.as_str() {
            "c" => {
                let input = grab_input("Enter a string to search for: ");
                let result = if tree.contains(&input) { "true" } else { "false" };
                println!("String {input} in tree: {result}");
            },
            "s" => {
                let result = if tree.is_empty() { "true" } else { "false" };
                println!("Tree is empty: {result}");
            },
            "e" => {
                tree.empty();
                println!("Tree is empty.");
            },
            "i" => {
                let input = grab_input("Enter a string to insert: ");
                tree.insert(input.clone());
                println!("String {input} inserted.");
            },
            "r" => {
                let input = grab_input("Enter a string to remove: ");
                if !tree.contains(&input) {
                    println!("String {input} not in the tree.");
                } else {
                    tree.remove(&input);
                    println!("String {input} removed.");
                }
            },
            "p" => tree.print(),
            "f" => {
                let m_val = tree.find_min();
                println!("Min value is {m_val}");
            },
            "m" => {
                let m_val = tree.find_min();
                println!("Max value is {m_val}");
            },
            "q" => break,
            _ => println!("Command {input} is not valid"),
        }
    }
}