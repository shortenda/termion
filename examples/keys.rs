extern crate termion;


#[cfg(feature = "nightly")]
fn main() {
    use termion::{TermRead, TermWrite, IntoRawMode, Key};
    use std::io::{Write, stdout, stdin};

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    stdout.clear().unwrap();
    stdout.goto(0, 0).unwrap();
    stdout.write(b"q to exit. Type stuff, use alt, and so on.").unwrap();
    stdout.hide_cursor().unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        stdout.goto(5, 5).unwrap();
        stdout.clear_line().unwrap();
        match c {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => println!("×"),
            Key::Invalid => println!("???"),
            Key::Error => println!("ERROR"),
            _ => {},
        }
        stdout.flush().unwrap();
    }

    stdout.show_cursor().unwrap();
}
#[cfg(not(feature = "nightly"))]
fn main() {
    println!("To run this example, you need to enable the `nightly` feature. Use Rust nightly and compile with `--features nightly`.")
}
