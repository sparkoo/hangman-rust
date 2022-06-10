use io::{stdin, stdout};
use std::io;
use std::io::{Read, Write};

fn main() -> io::Result<()> {
    let word = pick_word();
    game_loop(&word);
    Ok(())
}

fn game_loop(word: &String) {
    loop {
        let entered = match prompt() {
            Ok(v) => {
                v
            }
            Err(err) => {
                println!("err '{}'", err);
                continue;
            }
        };
    }
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
