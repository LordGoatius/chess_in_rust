use rust_chess::*;

fn main() {
    let mut board = Board { ..Default::default()};
    board.print_board();
    board.move_piece((0,1), (0,3));
    board.print_board();
}
