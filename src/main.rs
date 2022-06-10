use io::{stdin, stdout};
use std::io;
use std::io::Write;

const MAX_TRIES: i32 = 5;

fn main() -> io::Result<()> {
    let word = pick_word();
    game_loop(&word);
    Ok(())
}

fn game_loop(word: &String) {
    let mut revealed: Vec<u8> = vec![b'_'; word.len()];
    let mut tries_left = MAX_TRIES;
    loop {
        print_revealed(&revealed);
        match prompt() {
            Ok(v) => {
                match reveal(v, &mut revealed, &word) {
                    left if left == 0 => {
                        println!("You Won!");
                        return;
                    },
                    left => {
                        tries_left = tries_left - 1;
                        if tries_left == 0 {
                            println!("You lost!");
                            return;
                        }
                        println!("{} letters left. You have {} more tries.", left, tries_left);
                    }
                }
            }
            Err(err) => {
                println!("err '{}'", err);
                continue;
            }
        };
    }
}

fn reveal(letter: u8, revealed: &mut Vec<u8>, word: &String) -> i32 {
    let word_bytes = word.as_bytes();
    let mut left = 0;
    for i in 0..word.len() {
        if word_bytes[i] == letter {
            revealed[i] = letter;
        }
        if revealed[i] == b'_' {
            left = left + 1
        }
    }
    left
}

fn print_revealed(revealed: &Vec<u8>) {
    println!();
    for c in revealed.iter() {
        print!("{} ", *c as char)
    }
    println!();
    println!();
}

fn pick_word() -> String {
    "blabol".to_string()
}

fn prompt<'a>() -> Result<u8, String> {
    print!("Enter letter: ");
    stdout().flush().unwrap();

    let mut buffer = String::new();
    match stdin().read_line(&mut buffer) {
        Ok(n) => {
            if n != 2 {
                return Err("enter single letter".to_string());
            }
            return match buffer.as_bytes()[0] {
                l if l >= 97 && l <= 122 => Ok(l),
                _ => Err("enter [a-z]".to_string()),
            };
        }
        Err(n) => {
            return Err(n.to_string());
        }
    }
}
