use std::io::{self, Read};

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    for ch in io::stdin().bytes() {
        let ch = ch.unwrap() as char;

        println!("{}", ch);

        if ch == 'q' {
            crossterm::terminal::disable_raw_mode().unwrap();
            break;
        }
    }
}
