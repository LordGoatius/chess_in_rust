#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn(bool),
    Rook(bool),
    Bishop,
    Knight,
    King(bool),
    Queen,
}
impl PieceType {
    fn ret_type_as_char(&self) -> char {
        match &self {
            PieceType::Pawn(_) => 'P',
            PieceType::Rook(_) => 'R',
            PieceType::Bishop => 'B',
            PieceType::Knight => 'N',
            PieceType::King(_) => 'K',
            PieceType::Queen => 'Q',
        }
    }

    fn ret_can_make_move(&self, beg_pos: (usize,usize), end_pos: (usize,usize)) -> bool {
        let (beg_rank, beg_file) = beg_pos;
        let (end_rank, end_file) = end_pos;

        let diff_file: i32 = (beg_file as i32 - end_file as i32).abs();
        let diff_rank: i32 = (beg_rank as i32 - end_rank as i32).abs();

        match self {
            PieceType::Pawn(moved) => {
                if !*moved {
                    diff_rank < 3
                } else {
                    diff_rank == 1
                }
            },
            PieceType::Rook(_) => {
                (diff_rank == 0) | (diff_file == 0)
            },
            PieceType::King(_) => {
                (diff_file <= 1) & (diff_rank <= 1)
            },
            PieceType::Bishop => {
                diff_file == diff_rank
            },
            PieceType::Knight => {
                ((diff_file == 2) & (diff_rank == 1)) | ((diff_rank == 2) & (diff_file == 1))
            },
            PieceType::Queen => {
                (diff_file == diff_rank) | ((beg_file == end_file) | (beg_rank == end_rank))
            },
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
        let mut row_num: i8 = 8;
        println!("  ---------------------------------");
        for row in &self.board {
            print!("{} |",row_num);
            row_num -= 1;
            for piece in row {
                match piece {
                    Some(_) => print!(" {} |",piece.as_ref().unwrap().get_piece_as_char()),
                    None => print!("   |"),
                }
            }
            println!("");
            println!("  ---------------------------------");
        }
        println!("    A   B   C   D   E   F   G   H  ");
    }

    pub fn build_starting_board(&mut self) {
        self.board = build_starting_board();
    }

    pub fn print_piece(&self, pos: &str) {
        let (rank, file) = chess_notation_to_array_notation(pos);
        println!("{:?}",self.board[rank][file].unwrap());
    }

    pub fn move_piece(&mut self, beginning_pos: (usize, usize), ending_pos: (usize, usize)) {
        let board = &mut self.board;

        let (beg_rank, beg_file) = beginning_pos;
        let (end_rank, end_file) = ending_pos;

        match board[beg_rank][ beg_file] {
            Some (piece) => {
                match piece.piece_type {
                    PieceType::King(false) => {
                        board[beg_rank][beg_file] = Some(Piece {
                            piece_type: PieceType::King(true),
                            color: piece.color,
                        })
                    },
                    PieceType::Pawn(false) => {
                        board[beg_rank][beg_file] = Some(Piece {
                            piece_type: PieceType::Pawn(true),
                            color: piece.color,
                        })
                    },
                    PieceType::Rook(false) => {
                        board[beg_rank][beg_file] = Some(Piece {
                            piece_type: PieceType::Rook(true),
                            color: piece.color,
                        })
                    },
                    _ => (),
                }
            },
            None => (),
        }

        board[end_rank][end_file] = board[beg_rank][beg_file].clone();
        board[beg_rank][beg_file] = None;
    }

    pub fn move_piece_with_chess_notation(&mut self, start: &str, end: &str) {
        self.move_piece(chess_notation_to_array_notation(start), chess_notation_to_array_notation(end));
    }

    pub fn check_legal_move(&self, beginning_pos: (usize, usize), ending_pos: (usize, usize)) -> bool {
        let (beg_rank, beg_file) = beginning_pos;

        match self.board[beg_rank][beg_file] {
            Some(piece) => {
                piece.piece_type.ret_can_make_move(beginning_pos, ending_pos)
            },
            None => false,
        }
    }

    pub fn check_legal_move_chess_notation(&self, beginning_pos: &str, ending_pos: &str) -> bool {
        self.check_legal_move(chess_notation_to_array_notation(beginning_pos), chess_notation_to_array_notation(ending_pos))
    }
}

fn build_piece(piece: PieceType, color: Color) -> Option<Piece>{
    let to_build = Piece { piece_type: piece, color: color, };
    let ret: Option<Piece> = Some(to_build);
    return ret;
}
fn build_starting_board() -> [[Option<Piece>; 8]; 8] {
    let mut board = [[None, None, None, None, None, None, None, None],[None, None, None, None, None, None, None, None], [None, None, None, None, None, None, None, None],[None, None, None, None, None, None, None, None],[None, None, None, None, None, None, None, None],[None, None, None, None, None, None, None, None],[None, None, None, None, None, None, None, None],[None, None, None, None, None, None, None, None]];
    board[0][0] = build_piece(PieceType::Rook(false), Color::Black);
    board[0][1] = build_piece(PieceType::Knight, Color::Black);
    board[0][2] = build_piece(PieceType::Bishop, Color::Black);
    board[0][3] = build_piece(PieceType::Queen, Color::Black);
    board[0][4] = build_piece(PieceType::King(false), Color::Black);
    board[0][5] = build_piece(PieceType::Bishop, Color::Black);
    board[0][6] = build_piece(PieceType::Knight, Color::Black);
    board[0][7] = build_piece(PieceType::Rook(false), Color::Black);

    for i in 0..8 {
        board[1][i] = build_piece(PieceType::Pawn(false), Color::Black);
        board[6][i] = build_piece(PieceType::Pawn(false), Color::White)
    }

    for i in 2..6 {
        for j in 0..8 {
            board[i][j] = None;
        }
    }

    board[7][0] = build_piece(PieceType::Rook(false), Color::White);
    board[7][1] = build_piece(PieceType::Knight, Color::White);
    board[7][2] = build_piece(PieceType::Bishop, Color::White);
    board[7][3] = build_piece(PieceType::Queen, Color::White);
    board[7][4] = build_piece(PieceType::King(false), Color::White);
    board[7][5] = build_piece(PieceType::Bishop, Color::White);
    board[7][6] = build_piece(PieceType::Knight, Color::White);
    board[7][7] = build_piece(PieceType::Rook(false), Color::White);
    board
}

pub fn chess_notation_to_array_notation(chess_not: &str) -> (usize, usize) /* file is columns*/ {
    let file: usize = chess_not.chars().nth(0).unwrap().to_ascii_uppercase() as usize - 65;
    let rank: usize = 7 - (chess_not.chars().nth(1).unwrap() as usize - 49);

    (rank, file)
}