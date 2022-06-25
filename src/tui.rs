use crate::board::Position;

pub fn print_position(pos: &Position) {
    let pos = pos.into_char_vec();

    for (i, c) in pos.into_iter().rev().enumerate() {
        if c == ' ' {
            print!("|·");
        } else {
            print!("|{c}");
        }

        if (i + 1) % 8 == 0 {
            println!("|");
        }
    }
}
