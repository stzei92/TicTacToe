use crate::field::Field;
use crate::field::Player;
use std::io;
pub mod field;

fn main() {
    let mut f = Field::new();
    f.new_game();
    let mut counter = 0;
    loop {       
        let res: u8 = counter%2;
        let p_1 = Player::Player1;
        let p_2 = Player::Player2;
        let p = match res {
            0 => p_1,
            1 => p_2,
            _ => todo!()
        };
        f.print_field();
        println!("\n\nIt's your turn {} ({})! Enter your move:",p.to_string(),p.get_token().to_string());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Whoopsie Daisy!");
        let pos: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("abc"); continue;}
        };
        f.place_token(pos,p);
        counter +=1;
        match f.check_win_condition() {
            Some(Player::Player1) => {f.print_field(); println!("Player 1 won the game!"); break},
            Some(Player::Player2) => {f.print_field(); println!("Player 2 won the game!"); break},
            Some(Player::NoPlayer) => {f.print_field(); println!("It's a DRAW!"); break}
            None => continue
        }


    }
}
