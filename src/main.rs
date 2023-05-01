use rust_chess::*;

fn main() {
    let mut board = Board { ..Default::default()};
    // board.print_board();
    // println!("{}",board.check_legal_move_chess_notation("e2", "e4"));
    // board.move_piece_with_chess_notation("e2", "e4");
    // for i in (0..8).rev() {
    //     println!("{}",i);
    // }
    board.move_piece_with_chess_notation("a1", "d3");
    board.move_piece_with_chess_notation("e2", "e3");
    board.move_piece_with_chess_notation("d1", "c4");
    board.print_board();



    // //Rook
    // println!("True: {}",board.check_collison_chess_notation("d3", "d7"));
    // println!("False: {}",board.check_collison_chess_notation("d3", "d6"));
    // //Pawn
    // println!("True: {}",board.check_collison_chess_notation("d2", "d3"));
    // println!("True: {}",board.check_collison_chess_notation("d2", "d4"));
    // println!("False: {}",board.check_collison_chess_notation("c2", "c3"));
    // //Queen
    // println!("True: {}",board.check_collison_chess_notation("c4", "c2"));
    // println!("True: {}",board.check_collison_chess_notation("c4", "d3"));
    // println!("False: {}",board.check_collison_chess_notation("c4", "c3"));
    // println!("False: {}",board.check_collison_chess_notation("c4", "b5"));
    // //King
    // println!("True: {}",board.check_collison_chess_notation("e1", "d2"));
    // println!("False: {}",board.check_collison_chess_notation("e1", "e2"));
    // //Knight
    // println!("True: {}",board.check_collison_chess_notation("b1", "d2"));
    // println!("False: {}",board.check_collison_chess_notation("b1", "a3"));
}
