#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

pub type Board = [Option<Piece>; 64];

pub fn initialize_board() -> Board {
    let mut board: Board = [None; 64];

    // Place white pieces
    board[0] = Some(Piece::WhiteRook);
    board[1] = Some(Piece::WhiteKnight);
    board[2] = Some(Piece::WhiteBishop);
    board[3] = Some(Piece::WhiteQueen);
    board[4] = Some(Piece::WhiteKing);
    board[5] = Some(Piece::WhiteBishop);
    board[6] = Some(Piece::WhiteKnight);
    board[7] = Some(Piece::WhiteRook);
    for i in 8..16 {
        board[i] = Some(Piece::WhitePawn);
    }

    // Place black pieces
    board[56] = Some(Piece::BlackRook);
    board[57] = Some(Piece::BlackKnight);
    board[58] = Some(Piece::BlackBishop);
    board[59] = Some(Piece::BlackQueen);
    board[60] = Some(Piece::BlackKing);
    board[61] = Some(Piece::BlackBishop);
    board[62] = Some(Piece::BlackKnight);
    board[63] = Some(Piece::BlackRook);
    for i in 48..56 {
        board[i] = Some(Piece::BlackPawn);
    }

    board
}

pub fn print_board(board: &Board) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let index = rank * 8 + file;
            let symbol = match board[index] {
                Some(piece) => match piece {
                    Piece::WhitePawn => "P",
                    Piece::WhiteKnight => "N",
                    Piece::WhiteBishop => "B",
                    Piece::WhiteRook => "R",
                    Piece::WhiteQueen => "Q",
                    Piece::WhiteKing => "K",
                    Piece::BlackPawn => "p",
                    Piece::BlackKnight => "n",
                    Piece::BlackBishop => "b",
                    Piece::BlackRook => "r",
                    Piece::BlackQueen => "q",
                    Piece::BlackKing => "k",
                },
                None => ".",
            };
            print!("{} ", symbol);
        }
        println!();
    }
}
