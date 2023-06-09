use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
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

    fn ret_can_make_move(&self, beg_pos: (usize, usize), end_pos: (usize, usize)) -> bool {
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
            }
            PieceType::Rook(_) => (diff_rank == 0) | (diff_file == 0),
            PieceType::King(_) => (diff_file <= 1) & (diff_rank <= 1),
            PieceType::Bishop => diff_file == diff_rank,
            PieceType::Knight => {
                ((diff_file == 2) & (diff_rank == 1)) | ((diff_rank == 2) & (diff_file == 1))
            }
            PieceType::Queen => {
                (diff_file == diff_rank) | ((beg_file == end_file) | (beg_rank == end_rank))
            }
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
    pub can_en_pessant: [(i32, i32); 2],
}

impl Default for Board {
    fn default() -> Self {
        Board {
            board: build_starting_board(),
            turn: 0,
            turn_number: 0,
            can_en_pessant: [(-1, -1), (-1, -1)],
        }
    }
}
impl Board {
    pub fn print_board(&self) {
        let mut row_num: i8 = 8;
        println!("  ---------------------------------");
        for row in &self.board {
            print!("{} |", row_num);
            row_num -= 1;
            for piece in row {
                match piece {
                    Some(_) => print!(" {} |", piece.as_ref().unwrap().get_piece_as_char()),
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
        println!("{:?}", self.board[rank][file].unwrap());
    }

    fn check_en_pessant(&mut self, color: Color, ending_pos: (usize, usize)) {
        let (end_rank, end_file) = ending_pos;
        match color {
            Color::Black => {
                if end_file != 0 && end_file != 7 {
                    self.can_en_pessant[0] =
                        self.check_sides(Color::White, end_rank, end_file - 1);
                    self.can_en_pessant[1] =
                        self.check_sides(Color::White, end_rank, end_file + 1);
                } else if end_file == 0 {
                    self.can_en_pessant[0] =
                        self.check_sides(Color::White, end_rank, end_file + 1);
                } else if end_file == 7 {
                    self.can_en_pessant[0] =
                        self.check_sides(Color::White, end_rank, end_file - 1);
                } else {
                    panic!("Dunno wtf happened ngl");
                }
            },
            Color::White => {
                if end_file != 0 && end_file != 7 {
                    self.can_en_pessant[0] =
                        self.check_sides(Color::Black, end_rank, end_file - 1);
                    self.can_en_pessant[1] =
                        self.check_sides(Color::Black, end_rank, end_file + 1);
                } else if end_file == 0 {
                    self.can_en_pessant[0] =
                        self.check_sides(Color::Black, end_rank, end_file + 1);
                } else if end_file == 7 {
                    self.can_en_pessant[0] =
                        self.check_sides(Color::Black, end_rank, end_file - 1);
                } else {
                    panic!("Dunno wtf happened ngl");
                }
            }
        }
    }

    fn check_sides(&self, color: Color, rank: usize, to_check: usize) -> (i32, i32) {
        if let Some(side_piece) = self.board[rank][to_check] {
            if color == side_piece.color {
                return (rank as i32, to_check as i32);
            }
        }
        (-1, -1)
    }

    fn move_piece(&mut self, beginning_pos: (usize, usize), ending_pos: (usize, usize)) {
        let (beg_rank, beg_file) = beginning_pos;
        let (end_rank, end_file) = ending_pos;

        self.can_en_pessant = [(-1,-1),(-1,-1)];

        let starting_piece = self.board[beg_rank][beg_file];

        match starting_piece {
            Some(piece) => match piece.piece_type {
                PieceType::King(false) => {
                    self.board[beg_rank][beg_file] = Some(Piece {
                        piece_type: PieceType::King(true),
                        color: piece.color,
                    })
                },
                PieceType::Pawn(false) => {
                    let color = piece.color;
                    self.check_en_pessant(color, ending_pos);
                    self.board[beg_rank][beg_file] = Some(Piece {
                        piece_type: PieceType::Pawn(true),
                        color: piece.color,
                    })
                },
                PieceType::Rook(false) => {
                    self.board[beg_rank][beg_file] = Some(Piece {
                        piece_type: PieceType::Rook(true),
                        color: piece.color,
                    })
                },
                _ => (),
            },
            None => (),
        }

        match starting_piece.unwrap().piece_type {
            PieceType::Pawn(true) => {
                match starting_piece.unwrap().color {
                    Color::Black => {
                        if end_rank == 7 {
                            let mut choice = String::new();
                            println!("Select: N, Q, B, R");
                            io::stdin().read_line(&mut choice).expect("Failed to read line");
                            match choice.chars().nth(0).unwrap().to_ascii_uppercase() {
                                'N' => {
                                    self.board[beg_rank][beg_file]= Some(Piece {
                                        piece_type: PieceType::Knight,
                                        color: Color::Black,
                                    })
                                },
                                'Q' => {
                                    self.board[beg_rank][beg_file]= Some(Piece {
                                        piece_type: PieceType::Queen,
                                        color: Color::Black,
                                    })
                                },
                                'B' => {
                                    self.board[beg_rank][beg_file]= Some(Piece {
                                        piece_type: PieceType::Bishop,
                                        color: Color::Black,
                                    })
                                },
                                'R' => {
                                    self.board[beg_rank][beg_file]= Some(Piece {
                                        piece_type: PieceType::Rook(true),
                                        color: Color::Black,
                                    })
                                },
                                _ => panic!("Bro idk what to say I'm too lazy to fix this"),
                            }
                            println!();
                        }
                    },
                    Color::White => {
                        if end_rank == 0 {
                            let mut choice = String::new();
                            println!("Select: N, Q, B, R");
                            io::stdin().read_line(&mut choice).expect("Failed to read line");
                            match choice.chars().nth(0).unwrap().to_ascii_uppercase() {
                                'N' => {
                                    self.board[beg_rank][beg_file]= Some(Piece {
                                        piece_type: PieceType::Knight,
                                        color: Color::White,
                                    })
                                },
                                'Q' => {
                                    self.board[beg_rank][beg_file]= Some(Piece {
                                        piece_type: PieceType::Queen,
                                        color: Color::White,
                                    })
                                },
                                'B' => {
                                    self.board[beg_rank][beg_file]= Some(Piece {
                                        piece_type: PieceType::Bishop,
                                        color: Color::White,
                                    })
                                },
                                'R' => {
                                    self.board[beg_rank][beg_file]= Some(Piece {
                                        piece_type: PieceType::Rook(true),
                                        color: Color::White,
                                    })
                                },
                                _ => panic!("Bro idk what to say I'm too lazy to fix this"),
                            }
                            println!();
                        }
                    },
                }
            },
            _ => (),
        }

        self.board[end_rank][end_file] = self.board[beg_rank][beg_file].clone();
        self.board[beg_rank][beg_file] = None;
    }

    pub fn move_piece_with_chess_notation(&mut self, start: &str, end: &str) {
        self.move_piece(
            chess_notation_to_array_notation(start),
            chess_notation_to_array_notation(end),
        );
    }

    fn check_legal_move(&self, beginning_pos: (usize, usize), ending_pos: (usize, usize)) -> bool {
        let (beg_rank, beg_file) = beginning_pos;

        match self.board[beg_rank][beg_file] {
            Some(piece) => piece
                .piece_type
                .ret_can_make_move(beginning_pos, ending_pos),
            None => false,
        }
    }

    pub fn check_legal_move_chess_notation(&self, beginning_pos: &str, ending_pos: &str) -> bool {
        self.check_legal_move(
            chess_notation_to_array_notation(beginning_pos),
            chess_notation_to_array_notation(ending_pos),
        )
    }

    fn check_collison(&self, beginning_pos: (usize, usize), ending_pos: (usize, usize)) -> bool {
        let board = &self.board;

        let (beg_rank, beg_file) = beginning_pos;
        let (end_rank, end_file) = ending_pos;

        let diff_file: i32 = end_file as i32 - beg_file as i32;
        let diff_rank: i32 = end_rank as i32 - beg_rank as i32;

        match board[beg_rank][beg_file].unwrap().piece_type {
            PieceType::Pawn(_) => {
                for i in ret_range(beg_rank, end_rank) {
                    if let Some(_) = board[i as usize][beg_file] {
                        return true;
                    }
                }
                false
            }
            PieceType::Bishop => {
                for (i, j) in ret_range(beg_rank, end_rank).zip(ret_range(beg_file, end_file)) {
                    let (i, j) = (i, j);
                    if let Some(_) = board[i as usize][j as usize] {
                        return true;
                    }
                }
                false
            }
            PieceType::Rook(_) => {
                if diff_file == 0 {
                    for i in ret_range(beg_rank, end_rank) {
                        if let Some(_) = board[i as usize][beg_file] {
                            return true;
                        }
                    }
                } else {
                    for j in ret_range(beg_file, end_file) {
                        if let Some(_) = board[beg_rank][j as usize] {
                            return true;
                        }
                    }
                }
                false
            }
            PieceType::Queen => {
                if diff_file.abs() == diff_rank.abs() {
                    for (i, j) in ret_range(beg_rank, end_rank).zip(ret_range(beg_file, end_file)) {
                        let (i, j) = (i, j);
                        if let Some(_) = board[i as usize][j as usize] {
                            return true;
                        }
                    }
                    false
                } else {
                    if diff_file == 0 {
                        for i in ret_range(beg_rank, end_rank) {
                            if let Some(_) = board[i as usize][beg_file] {
                                return true;
                            }
                        }
                    } else {
                        for j in ret_range(beg_file, end_file) {
                            if let Some(_) = board[beg_rank][j as usize] {
                                return true;
                            }
                        }
                    }
                    false
                }
            }
            _ => false, // King and Knight cannot have "collisions"
        }
    }

    pub fn check_collison_chess_notation(&self, beginning_pos: &str, ending_pos: &str) -> bool {
        self.check_collison(
            chess_notation_to_array_notation(beginning_pos),
            chess_notation_to_array_notation(ending_pos),
        )
    }

    fn make_move(&mut self, beginning_pos: (usize, usize), ending_pos: (usize, usize)) -> bool {
        let board = &mut self.board;

        let (beg_rank, beg_file) = beginning_pos;
        let (end_rank, end_file) = ending_pos;

        let starting_piece = &board[beg_rank][beg_file].unwrap();
        let ending_piece = &board[end_rank][end_file];

        if !starting_piece.piece_type.ret_can_make_move(beginning_pos, ending_pos) {
            return false;
        }
        match ending_piece {
            Some(end_piece) => {
                if end_piece.color == starting_piece.color {
                    return false;
                }
            }
            None => (),
        }
        if self.check_collison(beginning_pos, ending_pos) {
            return false;
        }
        self.move_piece(beginning_pos, ending_pos);
        true
    }

    pub fn make_move_chess_notation(&mut self, start: &str, end: &str) -> bool {
        self.make_move(
            chess_notation_to_array_notation(start),
            chess_notation_to_array_notation(end),
        )
    }

    fn select_move(&mut self, beginning_pos: (usize, usize), ending_pos: (usize, usize)) -> bool {
        let (beg_rank, beg_file) = beginning_pos;
        let (end_rank, end_file) = ending_pos;

        let diff_file: i32 = (end_file as i32 - beg_file as i32).abs();
        let diff_rank: i32 = end_rank as i32 - beg_rank as i32;

        let starting_piece = self.board[beg_rank][beg_file].unwrap();
        let ending_piece = self.board[end_rank][end_file];

        // check checking needs to be implemented for castling
        if let PieceType::King(false) = starting_piece.piece_type {
            if ending_piece.is_some() {
                if let PieceType::Rook(false) = ending_piece.unwrap().piece_type {
                    if !self.check_collison(ending_pos, beginning_pos) {
                        if end_file > beg_file {
                            self.move_piece(beginning_pos, (beg_rank, beg_file + 2));
                            self.move_piece(ending_pos, (beg_rank, beg_file + 1));
                            return true;
                        } else {
                            self.move_piece(beginning_pos, (beg_rank, beg_file - 2));
                            self.move_piece(ending_pos, (beg_rank, beg_file - 1));
                            return true;
                        }
                    }
                }
            }
        }

        // en pessant
        if let PieceType::Pawn(true) = starting_piece.piece_type {
            match starting_piece.color {
                Color::Black => {
                    if ((beg_rank < end_rank) && (end_rank - beg_rank) == 1) && (diff_file == 1) && ending_piece.is_none() {
                        for (i,j) in self.can_en_pessant {
                            if (i == beg_rank as i32) && (j == beg_file as i32) {
                                self.board[beg_rank][end_file] = None;
                                self.move_piece(beginning_pos, ending_pos);
                                return true;
                            }
                        }
                    }
                }, 
                Color::White => {
                    if ((end_rank < beg_rank) && (beg_rank - end_rank) == 1) && (diff_file == 1) && ending_piece.is_none() {
                        for (i,j) in self.can_en_pessant {
                            if (i == beg_rank as i32) && (j == beg_file as i32) {
                                self.board[beg_rank][end_file] = None;
                                self.move_piece(beginning_pos, ending_pos);
                                return true;
                            }
                        }
                    }
                },
            }
        }

        // pawn everything else
        if let PieceType::Pawn(_) = starting_piece.piece_type {
            if diff_file == 0 {
                return self.make_move(beginning_pos, ending_pos);
            } else if diff_file == 1 {
                match starting_piece.color {
                    Color::Black => {
                        if diff_rank == 1 {
                            match ending_piece {
                                Some(end_piece) => match end_piece.color {
                                    Color::Black => {
                                        return false;
                                    }
                                    Color::White => {
                                        self.move_piece(beginning_pos, ending_pos);
                                        return true;
                                    }
                                },
                                None => return false,
                            }
                        }
                        return false;
                    }
                    Color::White => {
                        if diff_rank == -1 {
                            match ending_piece {
                                Some(end_piece) => match end_piece.color {
                                    Color::White => {
                                        return false;
                                    }
                                    Color::Black => {
                                        self.move_piece(beginning_pos, ending_pos);
                                        return true;
                                    }
                                },
                                None => return false,
                            }
                        }
                        return false;
                    }
                }
            }
            return false;
        }


        
        self.make_move(beginning_pos, ending_pos)
    }

    pub fn select_move_chess_notation(&mut self, start: &str, end: &str) -> bool {
        self.select_move(
            chess_notation_to_array_notation(start),
            chess_notation_to_array_notation(end),
        )
    }
}

fn build_piece(piece: PieceType, color: Color) -> Option<Piece> {
    let to_build = Piece {
        piece_type: piece,
        color: color,
    };
    let ret: Option<Piece> = Some(to_build);
    return ret;
}
fn build_starting_board() -> [[Option<Piece>; 8]; 8] {
    let mut board = [
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
    ];
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

// takes 2 ints and returns range between them from the first to the second, regardless of which is larger (excl..excl)
pub fn ret_range(first: usize, second: usize) -> Box<dyn Iterator<Item = i32>> {
    if first > second {
        Box::new(((second as i32 + 1)..(first as i32)).rev())
    } else {
        Box::new((first as i32 + 1)..(second as i32)) as Box<dyn Iterator<Item = i32>>
    }
}
