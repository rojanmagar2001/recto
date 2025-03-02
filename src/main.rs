use std::io::{self, Read};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    crossterm::terminal::enable_raw_mode().context("couldn't enable raw mode")?;
    for b in io::stdin().bytes() {
        let b = b.context("couldn't get the input byte")?;
        let ch = b as char;

        if ch.is_control() {
            println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
        } else {
            println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, ch);
        }

        if ch == 'q' {
            crossterm::terminal::disable_raw_mode().context("couldn't disable raw mode")?;
            break;
        }
    }

    Ok(())
}
