use std::io::{self, Read};

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let ch = b as char;

        if ch.is_control() {
            println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
        } else {
            println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, ch);
        }

        if ch == 'q' {
            crossterm::terminal::disable_raw_mode().unwrap();
            break;
        }
    }
}
