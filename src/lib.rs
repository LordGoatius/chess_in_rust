#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Knight,
    King,
    Queen,
}
impl PieceType {
    fn ret_type_as_char(&self) -> char {
        match &self {
            PieceType::Pawn => 'P',
            PieceType::Rook => 'R',
            PieceType::Bishop => 'B',
            PieceType::Knight => 'N',
            PieceType::King => 'K',
            PieceType::Queen => 'Q',
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Black,
    White,
}
impl Color {
    fn is_white(&self) -> bool {
        self == &Color::White
        }
    }

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}
impl Piece {
    fn get_piece_as_char(&self) -> char {
        if self.color.is_white() {
            self.piece_type.ret_type_as_char()
        } else {
            self.piece_type.ret_type_as_char().to_ascii_lowercase()
        }
    }
}

pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
    pub turn: i8,
    pub turn_number: i8,
}
impl Default for Board {
    fn default() -> Self {
        Board {
            board: build_starting_board(),
            turn: 0,
            turn_number: 0,
        }
    }
}
impl Board {
    pub fn print_board(&self) {
        println!("---------------------------------");
        for row in &self.board {
            print!("|");
            for piece in row {
                match piece {
                    Some(_) => print!(" {} |",piece.as_ref().unwrap().get_piece_as_char()),
                    None => print!("   |"),
                }
            }
            println!("");
            println!("---------------------------------");
        }
    }

    pub fn build_starting_board(&mut self) {
        let board = &mut self.board;
        board[0][0] = build_piece(PieceType::Rook, Color::Black);
        board[0][1] = build_piece(PieceType::Knight, Color::Black);
        board[0][2] = build_piece(PieceType::Bishop, Color::Black);
        board[0][3] = build_piece(PieceType::Queen, Color::Black);
        board[0][4] = build_piece(PieceType::King, Color::Black);
        board[0][5] = build_piece(PieceType::Bishop, Color::Black);
        board[0][6] = build_piece(PieceType::Knight, Color::Black);
        board[0][7] = build_piece(PieceType::Rook, Color::Black);

        for i in 0..8 {
            board[1][i] = build_piece(PieceType::Pawn, Color::Black);
            board[6][i] = build_piece(PieceType::Pawn, Color::White)
        }

        for i in 2..6 {
            for j in 0..8 {
                board[i][j] = None;
            }
        }

        board[7][0] = build_piece(PieceType::Rook, Color::White);
        board[7][1] = build_piece(PieceType::Knight, Color::White);
        board[7][2] = build_piece(PieceType::Bishop, Color::White);
        board[7][3] = build_piece(PieceType::Queen, Color::White);
        board[7][4] = build_piece(PieceType::King, Color::White);
        board[7][5] = build_piece(PieceType::Bishop, Color::White);
        board[7][6] = build_piece(PieceType::Knight, Color::White);
        board[7][7] = build_piece(PieceType::Rook, Color::White);
        
    }

    pub fn move_piece(&mut self, beginning_pos: (i32, i32), ending_pos: (i32, i32)) {
        let board = &mut self.board;
        let (beg_x, beg_y) = beginning_pos;
        let (end_x, end_y) = ending_pos;
        board[end_y as usize][end_x as usize] = board[beg_y as usize][beg_x as usize].clone();
        board[beg_y as usize][beg_x as usize] = None;
    }
}

fn build_piece(piece: PieceType, color: Color) -> Option<Piece>{
    let to_build = Piece { piece_type: piece, color: color, };
    let ret: Option<Piece> = Some(to_build);
    return ret;
}
fn build_starting_board() -> [[Option<Piece>; 8]; 8] {
    let mut board = [[None, None, None, None, None, None, None, None],[None, None, None, None, None, None, None, None], [None, None, None, None, None, None, None, None],[None, None, None, None, None, None, None, None],[None, None, None, None, None, None, None, None],[None, None, None, None, None, None, None, None],[None, None, None, None, None, None, None, None],[None, None, None, None, None, None, None, None]];
    board[0][0] = build_piece(PieceType::Rook, Color::Black);
    board[0][1] = build_piece(PieceType::Knight, Color::Black);
    board[0][2] = build_piece(PieceType::Bishop, Color::Black);
    board[0][3] = build_piece(PieceType::Queen, Color::Black);
    board[0][4] = build_piece(PieceType::King, Color::Black);
    board[0][5] = build_piece(PieceType::Bishop, Color::Black);
    board[0][6] = build_piece(PieceType::Knight, Color::Black);
    board[0][7] = build_piece(PieceType::Rook, Color::Black);

    for i in 0..8 {
        board[1][i] = build_piece(PieceType::Pawn, Color::Black);
        board[6][i] = build_piece(PieceType::Pawn, Color::White)
    }

    for i in 2..6 {
        for j in 0..8 {
            board[i][j] = None;
        }
    }

    board[7][0] = build_piece(PieceType::Rook, Color::White);
    board[7][1] = build_piece(PieceType::Knight, Color::White);
    board[7][2] = build_piece(PieceType::Bishop, Color::White);
    board[7][3] = build_piece(PieceType::Queen, Color::White);
    board[7][4] = build_piece(PieceType::King, Color::White);
    board[7][5] = build_piece(PieceType::Bishop, Color::White);
    board[7][6] = build_piece(PieceType::Knight, Color::White);
    board[7][7] = build_piece(PieceType::Rook, Color::White);
    board
}