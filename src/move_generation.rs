use crate::gamestate::GameState;
use crate::board::{Board, Piece, PieceType, Color};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: usize,                      // Index of the source square
    pub to: usize,                        // Index of the destination square
    pub promotion: Option<PieceType>,     // For pawn promotion
    // Additional fields like captured piece, castling, en passant can be added
}

// Check if an index is within the bounds of the board
fn is_valid_index(index: isize) -> bool {
    index >= 0 && index < 64
}

// Get rank and file from index
fn get_rank_file(index: usize) -> (isize, isize) {
    let rank = (index / 8) as isize;
    let file = (index % 8) as isize;
    (rank, file)
}

// Convert rank and file back to index
fn get_index(rank: isize, file: isize) -> usize {
    (rank * 8 + file) as usize
}

pub fn generate_pawn_moves(board: &Board, position: usize, color: Color) -> Vec<Move> {
    let mut moves = Vec::new();
    let (rank, file) = get_rank_file(position);

    let (forward_step, start_rank, promotion_rank) = match color {
        Color::White => (1, 1, 6),
        Color::Black => (-1, 6, 1),
    };

    // Single forward move
    let forward_rank = rank + forward_step;
    if forward_rank >= 0 && forward_rank < 8 {
        let forward_index = get_index(forward_rank, file);
        if board[forward_index].is_none() {
            // Check for promotion
            if forward_rank == promotion_rank {
                // Add promotion options
                for promotion_piece in &[PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                    moves.push(Move {
                        from: position,
                        to: forward_index,
                        promotion: Some(*promotion_piece),
                    });
                }
            } else {
                moves.push(Move {
                    from: position,
                    to: forward_index,
                    promotion: None,
                });
            }

            // Double forward move from starting rank
            if rank == start_rank {
                let double_forward_rank = rank + 2 * forward_step;
                let double_forward_index = get_index(double_forward_rank, file);
                if board[double_forward_index].is_none() {
                    moves.push(Move {
                        from: position,
                        to: double_forward_index,
                        promotion: None,
                    });
                }
            }
        }
    }

    // Captures
    for &capture_file_offset in &[-1, 1] {
        let capture_file = file + capture_file_offset;
        if capture_file >= 0 && capture_file < 8 {
            let capture_rank = rank + forward_step;
            if capture_rank >= 0 && capture_rank < 8 {
                let capture_index = get_index(capture_rank, capture_file);
                if let Some(target_piece) = board[capture_index] {
                    if target_piece.color != color {
                        // Check for promotion
                        if capture_rank == promotion_rank {
                            for promotion_piece in &[PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                                moves.push(Move {
                                    from: position,
                                    to: capture_index,
                                    promotion: Some(*promotion_piece),
                                });
                            }
                        } else {
                            moves.push(Move {
                                from: position,
                                to: capture_index,
                                promotion: None,
                            });
                        }
                    }
                }
                // TODO: Handle en passant
            }
        }
    }

    moves
}

pub fn generate_knight_moves(board: &Board, position: usize, color: Color) -> Vec<Move> {
    let mut moves = Vec::new();
    let (rank, file) = get_rank_file(position);

    let knight_moves = [
        (-2, -1), (-2, 1),   // Up 2, Left/Right 1
        (-1, -2), (-1, 2),   // Up 1, Left/Right 2
        (1, -2), (1, 2),     // Down 1, Left/Right 2
        (2, -1), (2, 1),     // Down 2, Left/Right 1
    ];

    for &(dr, df) in &knight_moves {
        let new_rank = rank + dr;
        let new_file = file + df;

        if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
            let target_index = get_index(new_rank, new_file);
            if let Some(target_piece) = board[target_index] {
                if target_piece.color != color {
                    moves.push(Move {
                        from: position,
                        to: target_index,
                        promotion: None,
                    });
                }
            } else {
                moves.push(Move {
                    from: position,
                    to: target_index,
                    promotion: None,
                });
            }
        }
    }

    moves
}

pub fn generate_bishop_moves(board: &Board, position: usize, color: Color) -> Vec<Move> {
    generate_sliding_moves(board, position, color, &[
        (-1, -1), (-1, 1), (1, -1), (1, 1),
    ])
}

pub fn generate_rook_moves(board: &Board, position: usize, color: Color) -> Vec<Move> {
    generate_sliding_moves(board, position, color, &[
        (-1, 0), (1, 0), (0, -1), (0, 1),
    ])
}

pub fn generate_queen_moves(board: &Board, position: usize, color: Color) -> Vec<Move> {
    generate_sliding_moves(board, position, color, &[
        (-1, -1), (-1, 1), (1, -1), (1, 1),
        (-1, 0), (1, 0), (0, -1), (0, 1),
    ])
}

pub fn generate_king_moves(board: &Board, position: usize, color: Color) -> Vec<Move> {
    let mut moves = Vec::new();
    let (rank, file) = get_rank_file(position);

    let king_moves = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    for &(dr, df) in &king_moves {
        let new_rank = rank + dr;
        let new_file = file + df;

        if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
            let target_index = get_index(new_rank, new_file);
            if let Some(target_piece) = board[target_index] {
                if target_piece.color != color {
                    moves.push(Move {
                        from: position,
                        to: target_index,
                        promotion: None,
                    });
                }
            } else {
                moves.push(Move {
                    from: position,
                    to: target_index,
                    promotion: None,
                });
            }
        }
    }

    // TODO: Handle castling

    moves
}


pub fn generate_sliding_moves(
    board: &Board,
    position: usize,
    color: Color,
    directions: &[(isize, isize)],
) -> Vec<Move> {
    let mut moves = Vec::new();
    let (rank, file) = get_rank_file(position);

    for &(dr, df) in directions {
        let mut new_rank = rank;
        let mut new_file = file;

        loop {
            new_rank += dr;
            new_file += df;

            if new_rank < 0 || new_rank >= 8 || new_file < 0 || new_file >= 8 {
                break;
            }

            let target_index = get_index(new_rank, new_file);
            if let Some(target_piece) = board[target_index] {
                if target_piece.color != color {
                    moves.push(Move {
                        from: position,
                        to: target_index,
                        promotion: None,
                    });
                }
                break; // Can't move past blocking piece
            } else {
                moves.push(Move {
                    from: position,
                    to: target_index,
                    promotion: None,
                });
            }
        }
    }

    moves
}

pub fn generate_piece_moves(board: &Board, position: usize) -> Vec<Move> {
    if let Some(piece) = board[position] {
        match piece.piece_type {
            PieceType::Pawn => generate_pawn_moves(board, position, piece.color),
            PieceType::Knight => generate_knight_moves(board, position, piece.color),
            PieceType::Bishop => generate_bishop_moves(board, position, piece.color),
            PieceType::Rook => generate_rook_moves(board, position, piece.color),
            PieceType::Queen => generate_queen_moves(board, position, piece.color),
            PieceType::King => generate_king_moves(board, position, piece.color),
        }
    } else {
        Vec::new()
    }
}

pub fn is_in_check(board: &Board, color: Color) -> bool {
    // Find the king's position
    let king_position = board.iter().position(|&piece_option| {
        if let Some(piece) = piece_option {
            piece.piece_type == PieceType::King && piece.color == color
        } else {
            false
        }
    });

    if let Some(king_pos) = king_position {
        // Generate all opponent's moves
        let opponent_color = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        let opponent_moves = generate_all_pseudo_moves(board, opponent_color);

        // Check if any move attacks the king
        opponent_moves.iter().any(|mv| mv.to == king_pos)
    } else {
        // King is not on the board (should not happen in a valid game)
        false
    }
}

pub fn generate_all_pseudo_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut moves = Vec::new();
    for position in 0..64 {
        if let Some(piece) = board[position] {
            if piece.color == color {
                moves.extend(generate_piece_moves(board, position));
            }
        }
    }
    moves
}

pub fn generate_legal_moves(game_state: &GameState) -> Vec<Move> {
    let color = game_state.current_turn;
    let board = &game_state.board;
    let pseudo_moves = generate_all_pseudo_moves(board, color);
    let mut legal_moves = Vec::new();

    for mv in pseudo_moves {
        let mut new_game_state = game_state.clone(); // Clone the GameState for simulation
        make_move(&mut new_game_state, &mv);

        // Check if the move leaves the player in check
        if !is_in_check(&new_game_state.board, color) {
            legal_moves.push(mv);
        }
    }

    legal_moves
}


pub fn make_move(game_state: &mut GameState, mv: &Move) {
    let board = &mut game_state.board;

    // Get the piece from the source square
    if let Some(mut piece) = board[mv.from] {
        // Determine if the halfmove clock should be reset
        let mut reset_halfmove_clock = false;

        // Check if a pawn has moved
        if piece.piece_type == PieceType::Pawn {
            reset_halfmove_clock = true;
        }

        // Check if a capture has been made
        if board[mv.to].is_some() {
            reset_halfmove_clock = true;
        }

        // Handle pawn promotion
        if let Some(promotion_piece_type) = mv.promotion {
            piece.piece_type = promotion_piece_type;
            reset_halfmove_clock = true; // Promotion resets halfmove clock
        }

        // Move the piece to the destination square
        board[mv.to] = Some(piece);

        // Clear the source square
        board[mv.from] = None;

        // Reset or increment the halfmove clock
        if reset_halfmove_clock {
            game_state.halfmove_clock = 0;
        } else {
            game_state.halfmove_clock += 1;
        }

        // TODO: Handle special moves like castling and en passant
    } else {
        panic!("No piece at the source square {}!", mv.from);
    }
}