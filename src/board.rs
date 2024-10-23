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

pub fn initialize_board() -> Board {
    let mut board: Board = [None; 64];

    // Place white pieces
    board[0] = Some(Piece { piece_type: PieceType::Rook, color: Color::White });
    board[1] = Some(Piece { piece_type: PieceType::Knight, color: Color::White });
    board[2] = Some(Piece { piece_type: PieceType::Bishop, color: Color::White });
    board[3] = Some(Piece { piece_type: PieceType::Queen, color: Color::White });
    board[4] = Some(Piece { piece_type: PieceType::King, color: Color::White });
    board[5] = Some(Piece { piece_type: PieceType::Bishop, color: Color::White });
    board[6] = Some(Piece { piece_type: PieceType::Knight, color: Color::White });
    board[7] = Some(Piece { piece_type: PieceType::Rook, color: Color::White });

    for i in 8..16 {
        board[i] = Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
    }

    // Place black pieces
    board[56] = Some(Piece { piece_type: PieceType::Rook, color: Color::Black });
    board[57] = Some(Piece { piece_type: PieceType::Knight, color: Color::Black });
    board[58] = Some(Piece { piece_type: PieceType::Bishop, color: Color::Black });
    board[59] = Some(Piece { piece_type: PieceType::Queen, color: Color::Black });
    board[60] = Some(Piece { piece_type: PieceType::King, color: Color::Black });
    board[61] = Some(Piece { piece_type: PieceType::Bishop, color: Color::Black });
    board[62] = Some(Piece { piece_type: PieceType::Knight, color: Color::Black });
    board[63] = Some(Piece { piece_type: PieceType::Rook, color: Color::Black });

    for i in 48..56 {
        board[i] = Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
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
                },
                None => '.',
            };
            print!("{} ", symbol);
        }
        println!();
    }
}

