mod board;
mod move_generation;
mod gamestate;

use crate::gamestate::GameState;
use crate::board::{print_board, Color};
use crate::move_generation::{generate_legal_moves, make_move, is_in_check, Move};
use rand::seq::SliceRandom;

fn main() {
    let mut game_state = GameState::new();
    let mut move_count = 0;

    while !is_game_over(&game_state) {
        println!(
            "Move {}: {:?}'s turn (Halfmove clock: {})",
            move_count + 1,
            game_state.current_turn,
            game_state.halfmove_clock
        );
        print_board(&game_state.board);

        let legal_moves = generate_legal_moves(&game_state);

        if legal_moves.is_empty() {
            // Game over handled in is_game_over function
            break;
        }

        // Select a move randomly
        let selected_move = select_move(&legal_moves);

        // Make the move
        make_move(&mut game_state, &selected_move);

        // Switch turns
        game_state.current_turn = match game_state.current_turn {
            Color::White => {
                game_state.fullmove_number += 1;
                Color::Black
            },
            Color::Black => Color::White,
        };

        move_count += 1;
    }

    println!("Game over after {} moves.", move_count);
    print_board(&game_state.board);
}

fn is_game_over(game_state: &GameState) -> bool {
    let legal_moves = generate_legal_moves(game_state);

    if !legal_moves.is_empty() {
        if game_state.halfmove_clock >= 100 {
            println!("Draw by fifty-move rule.");
            return true;
        }
        return false;
    }

    if is_in_check(&game_state.board, game_state.current_turn) {
        println!("{:?} is checkmated!", game_state.current_turn);
    } else {
        println!("Stalemate!");
    }
    true
}

fn select_move(legal_moves: &[Move]) -> Move {
    let mut rng = rand::thread_rng();
    *legal_moves.choose(&mut rng).unwrap()
}
