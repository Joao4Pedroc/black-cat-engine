use crate::board::{Board, initialize_board, print_board, Color};
use crate::move_generation::{generate_legal_moves, make_move, is_in_check, Move};
use rand::seq::SliceRandom;

fn main() {
    let mut board = initialize_board();
    let mut current_turn = Color::White;
    let mut move_count = 0;

    while !is_game_over(&board, current_turn) {
        println!("Move {}: {:?}'s turn", move_count + 1, current_turn);
        print_board(&board);

        let legal_moves = generate_legal_moves(&board, current_turn);

        if legal_moves.is_empty() {
            // Game over handled in is_game_over function
            break;
        }

        // Select a move randomly
        let selected_move = select_move(&legal_moves);

        // Make the move
        make_move(&mut board, &selected_move);

        // Switch turns
        current_turn = match current_turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        move_count += 1;
    }

    println!("Game over after {} moves.", move_count);
    print_board(&board);
}

fn is_game_over(board: &Board, current_turn: Color) -> bool {
    let legal_moves = generate_legal_moves(board, current_turn);
    if !legal_moves.is_empty() {
        return false;
    }

    if is_in_check(board, current_turn) {
        println!("{:?} is checkmated!", current_turn);
    } else {
        println!("Stalemate!");
    }
    true
}

fn select_move(legal_moves: &[Move]) -> Move {
    let mut rng = rand::thread_rng();
    *legal_moves.choose(&mut rng).unwrap()
}
