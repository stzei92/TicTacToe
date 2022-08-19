use crate::field::Field;
use crate::field::Player;
use std::io;
pub mod field;

fn main() {
    let mut f = Field::new();
    f.new_game();
    loop {
        f.print_field();
        println!("Enter your next move: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Whoopsie Daisy!");
        let p_1 = Player::Player1;
        let pos: u8 = match input.parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        f.place_token(pos,p_1);
    }
}
