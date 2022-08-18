//field.rs
//implements the 3x3 playing field struct, the Element Enum and the Player Enum


pub struct Field {
    positions: Vec<Element>
}


pub enum Element {
Empty(pos: u8),
Nought(pos: u8),
Cross(pos: u8)
}

pub enum Player {
Player_1,
Player_2,
NoPlayer
}

pub enum Message {
Success,
Occupied,
WrongNumber
}


impl Field {
    pub fn new() -> Field {
        return Field {
            positions: Vec::new()
        };
    }

    pub fn print_field() {
        
    }

    pub fn check_win_condition() -> Player {

    }


    pub fn place_tile(pos: u8) -> Message {

    }
}
