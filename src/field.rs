//field.rs
//implements the 3x3 playing field struct, the Element Enum and the Player Enum


pub struct Field {
    positions: Vec<Element>,
    player_turn: Player,
}

pub struct Element {
    position: u8,
    typie: ElementType,
    owner: Player,
}

impl Element {
    fn new(position: u8, typie: ElementType) -> Element {
        return Element {
            position,
            typie,
            owner: Player::NoPlayer,
        }
    }

    fn to_string(&self) -> String {
        return match self.typie {
            ElementType::Empty => String::from(format!("{}",self.position)),
            _ => self.owner.get_token().to_string()
            
        }

    }
}

pub enum ElementType {
Empty,
Nought,
Cross
}

impl ElementType {
    fn to_string(&self) -> String {
        return match self {
            ElementType::Nought => String::from("O"),
            ElementType::Cross => String::from("X"),
            _ => String::from("-")
        }
    }
}

pub enum Player {
Player1,
Player2,
NoPlayer
}

impl Player {
    fn to_string(&self) -> String {
        return match self {
        Player::Player1 => String::from("Player 1"),
        Player::Player2 => String::from("Player 2"),
        _ => String::from("No One")
        };
    }

    fn get_token(&self) -> ElementType{
        return match self {    
            Player::Player1 => ElementType::Nought,
            Player::Player2 => ElementType::Cross,
            _ => ElementType::Empty
        }
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
        println!("  {}  ##  {}  ##  {} ",self.g_pos(1).to_string(),self.g_pos(2).to_string(),self.g_pos(3).to_string());
        println!("     ##     ##    ");
        println!("###################");
        println!("     ##     ##    ");
        println!("  {}  ##  {}  ##  {} ",self.g_pos(4).to_string(),self.g_pos(5).to_string(),self.g_pos(6).to_string());
        println!("     ##     ##    ");
        println!("###################");
        println!("     ##     ##    ");
        println!("  {}  ##  {}  ##  {} ",self.g_pos(7).to_string(),self.g_pos(8).to_string(),self.g_pos(9).to_string());
        println!("     ##     ##    ");
//        println!("\n\n Please enter your move: ");

    }

    pub fn check_win_condition() -> Player {
        return Player::Player1;
    }


    pub fn place_token(&mut self, pos: u8, p: Player) -> Message {
        self.positions[pos as usize].typie=p.get_token();
        self.positions[pos as usize].owner=p;
        return Message::Success;
    }
    
    pub fn new_game(&mut self) {
        self.positions.clear();
        for i in 1..10 {
           self.positions.push(Element::new(i,ElementType::Empty));
        }
    }

    fn g_pos(&self, pos: u8) -> &Element {
        //The elements are ordered per definition, so we can use direct access
        let index: usize = (pos-1) as usize;
        return &(self.positions[index]);
    }


}
