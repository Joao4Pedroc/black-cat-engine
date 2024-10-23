// Remove any definition of GameState from here

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

pub type Board = [Option<Piece>; 64];

// Initialization and print functions
pub fn initialize_board() -> Board {
    let mut board: Board = [None; 64];

    // Place white pieces
    let white_back_rank = [
        PieceType::Rook,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Queen,
        PieceType::King,
        PieceType::Bishop,
        PieceType::Knight,
        PieceType::Rook,
    ];
    for (i, &piece_type) in white_back_rank.iter().enumerate() {
        board[i] = Some(Piece {
            piece_type,
            color: Color::White,
        });
    }

    // Place white pawns
    for i in 8..16 {
        board[i] = Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        });
    }

    // Place black pawns
    for i in 48..56 {
        board[i] = Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        });
    }

    // Place black pieces
    let black_back_rank = [
        PieceType::Rook,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Queen,
        PieceType::King,
        PieceType::Bishop,
        PieceType::Knight,
        PieceType::Rook,
    ];
    for (i, &piece_type) in black_back_rank.iter().enumerate() {
        board[56 + i] = Some(Piece {
            piece_type,
            color: Color::Black,
        });
    }

    board
}

pub fn print_board(board: &Board) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let index = rank * 8 + file;
            let symbol = match board[index] {
                Some(piece) => {
                    let piece_char = match piece.piece_type {
                        PieceType::Pawn => 'P',
                        PieceType::Knight => 'N',
                        PieceType::Bishop => 'B',
                        PieceType::Rook => 'R',
                        PieceType::Queen => 'Q',
                        PieceType::King => 'K',
                    };
                    match piece.color {
                        Color::White => piece_char,
                        Color::Black => piece_char.to_ascii_lowercase(),
                    }
                }
                None => '.',
            };
            print!("{} ", symbol);
        }
        println!();
    }
}
