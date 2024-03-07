#[derive(Copy, Clone, PartialEq)]
enum Colour {
    WHITE,
    BLACK,
}

#[derive(Copy, Clone)]
struct ImplPiece<'a> {
    pub colour: Colour,
    data: &'a dyn Piece,
}

struct Pos {
    pub x: usize,
    pub y: usize,
}

type Square<'a> = Option<ImplPiece<'a>>;
type Board<'a> = [[Square<'a>; 8]; 8];

trait Piece {
    fn can_reach(&self, board: &Board, target: Pos, from: Pos) -> bool;
    fn glyph(&self) -> char;
    fn move_to(&mut self, board: &mut Board, target: Pos, from: Pos);
}

struct Pawn {
    moved: bool,
}

impl Piece for Pawn {
    fn can_reach(&self, board: &Board, target: Pos, from: Pos) -> bool {
        let piece = &board[from.x][from.y];
        if let None = piece {
            return false;
        }
        let eating = &board[target.x][target.y];
        let free = eating.is_none();
        let col = piece.as_ref().unwrap().colour;
        let attackable = match eating {
            None => false,
            Some(p) => p.colour == col,
        };
        let direction = match col {
            Colour::BLACK => 1,
            Colour::WHITE => -1,
        };
        let dx = (from.x as isize - target.x as isize).abs();
        let dy = (from.y as isize - target.y as isize).abs();
        if dx == 1 && dy == direction {
            return attackable;
        }
        if dx >= 1 {
            return false;
        }
        match self.moved {
            true => dy == direction && free,
            false => {
                if dy / dy.abs() != direction {
                    return false;
                }
                if dy.abs() > 2 {
                    return false;
                }
                if dy.abs() == 1 {
                    return free;
                }
                let first = dy / 2;
                // TODO: can this ever cause a wrap? maybe on invalid input
                if board[target.x][target.y - first as usize].is_none() {
                    return free;
                }
                false
            }
        }
    }

    fn glyph(&self) -> char {
        todo!()
    }

    fn move_to(&mut self, board: &mut Board, target: Pos, from: Pos) {
		board[target.x][target.y] = board[from.x][from.y];
		board[from.x][from.y] = None;
	}
}

fn main() {
    println!("Hello, world!");
}
