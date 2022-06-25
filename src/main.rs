mod board;
mod tui;

use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("FEN: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    println!();

    let pos = board::Position::from_fen(input).unwrap();
    tui::print_position(&pos);
}
