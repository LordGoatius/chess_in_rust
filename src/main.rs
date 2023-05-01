use rust_chess::*;
use std::io;

fn main() {
    let mut board = Board { ..Default::default()};
    board.print_board();

    

    'main_loop: loop {
        let mut start_move = String::new();
        let mut end_move = String::new();
        
        println!("Input start: ");
        io::stdin()
            .read_line(&mut start_move)
            .expect("Failed to read line");
        println!("");

        println!("Input end: ");
        io::stdin()
            .read_line(&mut end_move)
            .expect("Failed to read line");
        println!("");
        
        if start_move == "0" {
            break 'main_loop;
        }
        println!("{}",board.make_move_chess_notation(&start_move, &end_move));
        board.print_board();
    }

    // //Rook
    // println!("True: {}",board.check_collison_chess_notation("d3", "d8"));
    // println!("False: {}",board.check_collison_chess_notation("d3", "d7"));
    // println!("False: {}",board.check_collison_chess_notation("d3", "d6"));
    // //Pawn
    // println!("True: {}",board.check_collison_chess_notation("d2", "d3"));
    // println!("True: {}",board.check_collison_chess_notation("d2", "d4"));
    // println!("False: {}",board.check_collison_chess_notation("c2", "c3"));
    // //Queen
    // println!("False: {}",board.check_collison_chess_notation("c4", "c2"));
    // println!("True: {}",board.check_collison_chess_notation("c4", "c1"));
    // println!("True: {}",board.check_collison_chess_notation("c4", "e2"));
    // println!("False: {}",board.check_collison_chess_notation("c4", "d3"));
    // println!("False: {}",board.check_collison_chess_notation("c4", "c3"));
    // println!("False: {}",board.check_collison_chess_notation("c4", "b5"));
    // //King
    // println!("True: {}",board.check_collison_chess_notation("e1", "d2"));
    // println!("False: {}",board.check_collison_chess_notation("e1", "e2"));
    // //Knight
    // println!("True: {}",board.check_collison_chess_notation("b1", "d2"));
    // println!("False: {}",board.check_collison_chess_notation("b1", "a3"));
}
