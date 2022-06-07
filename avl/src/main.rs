use std::process::exit;
use std::io::{Write, stdout, stdin};

mod bst;
use bst::BST;

const PROMPT: &str = "Enter a command ((c)ontains, i(s)_empty, (e)mpty, (i)nsert, (p)rint, (f)ind_min, find_(m)ax, (q)uit): ";

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
    let mut tree: BST<String> = BST::new();

    loop {
        println!();
        let input = grab_input(PROMPT);

        match input.as_str() {
            "c" => {
                let input = grab_input("Enter a string to search for: ");
                let result = tree.contains(&input);
                println!("String {input} in tree: {result}");
            },
            "s" => {
                let result = if tree.is_empty() { "true" } else { "false" };
                println!("Tree is empty: {result}");
            },
            "e" => {
                tree = BST::new(); // this should run the destructor
                println!("Tree is empty.");
            },
            "i" => {
                let input = grab_input("Enter a string to insert: ");
                tree.insert(input.clone());
                println!("String {input} inserted.");
            },
            "p" => tree.print(),
            "f" => {
                let m_val = match tree.find_min() {
                    Ok(val) => val,
                    Err(e) => {
                        eprintln!("Error finding min: {e}");
                        continue;
                    }
                };
                println!("Min value is {m_val}");
            },
            "m" => {
                let m_val = match tree.find_max() {
                    Ok(val) => val,
                    Err(e) => {
                        eprintln!("Error finding min: {e}");
                        continue;
                    }
                };
                println!("Max value is {m_val}");
            },
            "q" => break,
            _ => println!("Command {input} is not valid"),
        }
    }
}
