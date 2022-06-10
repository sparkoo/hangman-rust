use io::{stdin, stdout};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use rand::seq::SliceRandom;

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
                    (_, l) if l == 0 => {
                        println!("You Won! '{}'", word);
                        return;
                    },
                    (g, l) => {
                        if g == 0 {
                            tries_left = tries_left - 1;
                        }
                        if tries_left == 0 {
                            println!("You lost! Word was '{}'", word);
                            return;
                        }
                        println!("You hit {} letters, {} letters left.", g, l);
                        println!("You have {} more tries!", tries_left);
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

fn reveal(letter: u8, revealed: &mut Vec<u8>, word: &String) -> (i32, i32) {
    let word_bytes = word.as_bytes();
    let mut left = 0;
    let mut guessed = 0;
    for i in 0..word.len() {
        if word_bytes[i] == letter {
            guessed = guessed + 1;
            revealed[i] = letter;
        }
        if revealed[i] == b'_' {
            left = left + 1
        }
    }
    (guessed, left)
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
    let filename = "words.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut words: Vec<String> = vec![];
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        words.push(line);
    }

    match words.choose(&mut rand::thread_rng()) {
        Some(w) => {w.clone()}
        None => {"".to_string()}
    }
}

fn prompt() -> Result<u8, String> {
    print!("Enter letter: ");
    stdout().flush().unwrap();

    let mut buffer = String::new();
    return match stdin().read_line(&mut buffer) {
        Ok(n) => {
            match buffer.as_bytes() {
                b if n <= 1 || (b[1] != 10 && b[1] != 13) => { // second character must be \b on linux or \v on windows
                    Err("enter single letter".to_string())
                },
                b => {
                    match b[0] {
                        l if l >= 97 && l <= 122 => Ok(l),
                        _ => Err("enter [a-z]".to_string()),
                    }
                }
            }
        }
        Err(n) => {
            Err(n.to_string())
        }
    }
}
