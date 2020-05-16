// some-old-feature
// what do to?(k/d/s/?) > s
use std::io;
use std::io::{Read, Write};
fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    let mut stdin = io::stdin().bytes();

    loop {
        write!(stdout, "Type something >").unwrap();
        stdout.flush().unwrap();
        let c = char::from(stdin.next().unwrap().unwrap());
        if c == 'q' {
            break;
        }
        write!(stdout, "You typed '{}'\n\r", c).unwrap();
        stdout.flush().unwrap();
    }

    crossterm::terminal::disable_raw_mode().unwrap();
}
