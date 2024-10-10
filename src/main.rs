mod board;

fn main() {
    let board = board::initialize_board();
    board::print_board(&board);
}
