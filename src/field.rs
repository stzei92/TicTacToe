//field.rs
//implements the 3x3 playing field struct, the Element Enum and the Player Enum


pub struct Field {
    positions: Vec<Element>,
    player_turn: Player,
}

pub struct Element {
    position: u8,
    typie: ElementType,
}

impl Element {
    fn new(position: u8, typie: ElementType) -> Element {
        return Element {
            position,
            typie
        }
    }
}

pub enum ElementType {
Empty(u8),
Nought(u8),
Cross(u8)
}

pub enum Player {
Player1,
Player2,
NoPlayer
}

impl Player {
    fn to_string(&self) -> String {
        return match &self {
        Player::Player1 => String::from("Player 1"),
        Player::Player2 => String::from("Player 2"),
        _ => String::from("No One")
        };
    }
}

pub enum Message {
Success,
Occupied,
WrongNumber
}


impl Field {
    pub fn new() -> Field {
        return Field {
            positions: Vec::new(),
            player_turn: Player::Player1
        };
    }

    pub fn print_field(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);        
        println!("Tic Tac Toe Game");
        println!("It is {}'s turn!",self.player_turn.to_string());
        println!("     ##     ##    ");
        println!("  {}  ##  {}  ##  {} ",1,2,3);
        println!("     ##     ##    ");
        println!("###################");
        println!("     ##     ##    ");
        println!("  {}  ##  {}  ##  {} ",4,5,6);
        println!("     ##     ##    ");
        println!("###################");
        println!("     ##     ##    ");
        println!("  {}  ##  {}  ##  {} ",7,8,9);
        println!("     ##     ##    ");
        println!("\n\n Please enter your guess: ");

    }

    pub fn check_win_condition() -> Player {
        return Player::Player1;
    }


    pub fn place_tile(pos: u8) -> Message {
        return Message::Success;
    }
    
    pub fn new_game(&self) {
        self.positions.clear();
        for i in [1..10] {
            positions.push(Element::new(i,ElementType::Empty));
        }
    }

    fn get_element_at_pos(&self, pos: u8) -> Element {
        //The elements are ordered per definition, so we can use direct access
        return self.positions[pos-1];
    }


}
