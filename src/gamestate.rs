use crate::board::{Board, initialize_board, Color};

#[derive(Debug, Clone)]
pub struct GameState {
    pub board: Board,
    pub current_turn: Color,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
    // Add other fields if necessary
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: initialize_board(),
            current_turn: Color::White,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }
}
