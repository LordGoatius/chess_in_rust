use rust_chess::*;

fn main() {
    let mut board = Board { ..Default::default()};
    board.print_board();
    board.print_piece("e2");
    board.move_piece_with_chess_notation("e2", "e4");
    board.print_board();
    
}
