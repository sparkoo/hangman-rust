use io::{stdin, stdout};
use std::io;
use std::io::{Read, Write};

fn main() -> io::Result<()> {
    let word = pick_word();

    let entered = prompt();

    println!("bla '{}'", entered);
    Ok(())
}

fn pick_word() -> String {
    "blabol".to_string()
}

fn prompt() -> u8 {
    let mut buffer = vec![];

    print!("Enter letter: ");
    stdout().flush().unwrap();
    let entered = stdin().bytes().next().and_then(|r| r.ok());
    match entered {
        Ok(e) => {
            println!("{}", e)
        }
        Err(_) => {
            println!("failed")
        }
    }

    entered.unwrap().unwrap()
}
