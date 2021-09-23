use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */


static FILES: [char; 8] = ['a','b','c','d','e','f','g','h'];

#[derive(PartialEq, Copy, Clone)]
pub enum Colour {
    White,
    Black
}

#[derive(PartialEq, Copy, Clone)]
pub enum Piece {
    // Include piece type and colour
    King(Colour), 
    Queen(Colour), 
    Bishop(Colour), 
    Knight(Colour), 
    Rook(Colour),
    Pawn(Colour)
}

impl Piece {
    fn unwrap(&self) -> &Colour {
        match self {
            Piece::King(x) => x,
            Piece::Queen(x) => x,
            Piece::Bishop(x) => x,
            Piece::Knight(x) => x,
            Piece::Rook(x) => x,
            Piece::Pawn(x) => x
        }
    }

    fn dis(&self) -> String {
        match self {
            Piece::King(_) => "K  ".to_owned(),
            Piece::Queen(_) => "Q  ".to_owned(),
            Piece::Bishop(_) => "B  ".to_owned(),
            Piece::Knight(_) => "Kn ".to_owned(),
            Piece::Rook(_) => "R  ".to_owned(),
            Piece::Pawn(_) => "P  ".to_owned()
        }
    }
}

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    active: Colour,
    board: [[Option<Piece>; 8]; 8]
}


// take &str in format "<file><rank>" and convert to vector of i8 with format [<column>,<row>]
fn decode_position(_position: &str) -> Option<Vec<usize>> {
    let mut vec: Vec<usize> = Vec::default();
    vec.push(FILES.iter().position(|&c| c == _position.chars().nth(1).unwrap()).unwrap());
    vec.push(_position.chars().nth(2).unwrap().to_string().parse::<usize>().unwrap() - 1); // - 1 because index starts at 0
    Some(vec)
}

fn code_moves(_moves: Vec<Vec<usize>>) -> Vec<String> {
    let mut moves: Vec<String> = Vec::default();
    for i in _moves {
        moves.push(FILES[i[0]].to_string() + &(i[1]+1).to_string());
    }
    moves
}

fn get_availble_moves(_piece: Piece, _position: &Vec<usize>, _game: &Game, _checkable: bool) -> Vec<Vec<usize>> {
    // get all moves
    let mut moves: Vec<Vec<usize>> = match _piece {
        Piece::King(c) => get_king_movement(&_position, _game, _checkable, c),
        Piece::Queen(c) => {
            let mut moves = get_straight_moves(&_position, _game, c);
            moves.append(&mut get_diagonal_moves(&_position,_game, c));
            moves
        },
        Piece::Bishop(c) => get_diagonal_moves(&_position, _game, c),
        Piece::Knight(c) => get_knight_moves(&_position,_game, c),
        Piece::Rook(c) => get_straight_moves(&_position, _game, c),
        Piece::Pawn(_) => get_pawn_moves(&_position,_game)
    };

    // remove illegal moves
    if _checkable {
        // if pinned, remove moves that reveal king
        moves = match _piece {
            Piece::King(_) => moves,
            _ => check_pinned(&_position, _game, &moves, _game.active)
        };

        // if in check, remove moves that do not resolve check
        if _game.state == GameState::Check {
            moves = match _piece {
                Piece::King(_) => moves,
                _ => resolve_check(&_position, _game, &moves, _game.active)
            };
        }
    }

    moves
}


fn get_king_movement(_position: &Vec<usize>, _game: &Game, _checkable: bool, _colour: Colour) -> Vec<Vec<usize>> {
    let mut moves: Vec<Vec<isize>> = Vec::default();
    let offsets: Vec<(isize,isize)> = vec![
        (0,1),
        (1,1),
        (1,0),
        (1,-1),
        (0,-1),
        (-1,-1),
        (-1,0),
        (-1,1)
    ];

    // generate legal moves
    for i in offsets {
        let mv = vec![_position[0] as isize + i.0, _position[1] as isize + i.1];
        if mv[0] >= 0 && mv[0] <= 7 && mv[1] >= 0 && mv[1] <= 7 && !check_occupied(&mv, _game, _colour) && (!check_check(&mv, _game, _colour) || !_checkable) {
            moves.push(mv.to_owned());
        }
    }

    // convert isize to usize
    let mut umoves: Vec<Vec<usize>> = Vec::default();
    for i in moves {
        umoves.push(vec![i[0] as usize, i[1] as usize]);
    }

    umoves
}

fn get_knight_moves(_position: &Vec<usize>, _game: &Game, _colour: Colour) -> Vec<Vec<usize>> {
    let mut moves: Vec<Vec<isize>> = Vec::default();
    let offsets: Vec<(isize,isize)> = vec![
        (1,2),
        (2,1),
        (-1,2),
        (-1,-2),
        (2,-1),
        (-2,-1),
        (1,-2),
        (-2,1)
    ];

    // generate legal moves
    for i in offsets {
        let mv = vec![_position[0] as isize + i.0, _position[1] as isize + i.1];
        if mv[0] >= 0 && mv[0] <= 7 && mv[1] >= 0 && mv[1] <= 7 && !check_occupied(&mv, _game, _colour) {
            moves.push(mv.to_owned());
        }
    }

    // convert isize to usize
    let mut umoves: Vec<Vec<usize>> = Vec::default();
    for i in moves {
        umoves.push(vec![i[0] as usize, i[1] as usize]);
    }

    umoves
}

fn get_pawn_moves(_position: &Vec<usize>, _game: &Game) -> Vec<Vec<usize>> {
    let piece = _game.board[_position[0]][_position[1]].unwrap();
    // check colour to get direction of movement
    match piece.unwrap() {
        Colour::White => {
            let mut mvs: Vec<Vec<usize>> = Vec::default();
            // check if double move is possible
            let offset: Vec<Vec<isize>> = match _position[0] {
                1 => vec![vec![1,0],vec![2,0]],
                _ => vec![vec![1,0]]
            };
            // check if blocked
            for mv in offset {
                let m = vec![(_position[0] as isize + mv[0]) as usize, (_position[1] as isize + mv[1]) as usize];
                match _game.board[m[0]][m[1]] {
                    Some(_) => break,
                    None => mvs.push(m)
                }
            }
            // check if on edge, meaning that moving towards that edge is not possible
            let takes: Vec<Vec<isize>> = match _position[1] {
                0 => vec![vec![1,1]],
                7 => vec![vec![1,-1]],
                _ => vec![vec![1,1],vec![1,-1]]
            };
            // check if there is a piece to take
            for mv in takes {
                let m = vec![(_position[0] as isize + mv[0]) as usize, (_position[1] as isize + mv[1]) as usize];
                match _game.board[m[0]][m[1]] {
                    Some(x) => match x.unwrap() {
                        Colour::Black => mvs.push(m),
                        _ => continue
                    },
                    None => continue
                }
            }

            mvs
        },
        Colour::Black => {
            let mut mvs: Vec<Vec<usize>> = Vec::default();
            // check if double move is possible
            let offset: Vec<Vec<isize>> = match _position[0] {
                6 => vec![vec![-1,0],vec![-2,0]],
                _ => vec![vec![-1,0]]
            };
            // check if blocked
            for mv in offset {
                let m = vec![(_position[0] as isize + mv[0]) as usize, (_position[1] as isize + mv[1]) as usize];
                match _game.board[m[0]][m[1]] {
                    Some(_) => break,
                    None => mvs.push(m)
                }
            }
            // check if on side, meaning that moving to that side is not possible
            let takes: Vec<Vec<isize>> = match _position[1] {
                0 => vec![vec![-1,1]],
                7 => vec![vec![-1,-1]],
                _ => vec![vec![-1,1],vec![-1,-1]]
            };
            // check if there is a piece to take
            for mv in takes {
                let m = vec![(_position[0] as isize + mv[0]) as usize, (_position[1] as isize + mv[1]) as usize];
                match _game.board[m[0]][m[1]] {
                    Some(x) => match x.unwrap() {
                        Colour::White => mvs.push(m),
                        _ => continue
                    },
                    None => continue
                }
            }

            mvs
        }

    }
}

fn get_straight_moves(_position: &Vec<usize>, _game: &Game, _colour: Colour) -> Vec<Vec<usize>> {
    let mut moves: Vec<Vec<usize>> = Vec::default();

    // down
    for i in _position[0]+1..8 {
        match _game.board[i][_position[1]]{
            Some(x) => {match x.unwrap() == &_colour {
                true => break,
                false => {
                    moves.push(vec![i, _position[1]]);
                    break
                }
            }},
            None => moves.push(vec![i, _position[1]])
        }
    }

    // up
    for i in (_position[1]-1..=0).rev() {
        match _game.board[i][_position[1]]{
            Some(x) => {match x.unwrap() == &_colour {
                true => break,
                false => {
                    moves.push(vec![i, _position[1]]);
                    break
                }
            }},
            None => moves.push(vec![i, _position[1]])
        }
    }

    // down
    for i in _position[0]+1..8 {
        match _game.board[i][_position[1]]{
            Some(x) => {match x.unwrap() == &_colour {
                true => break,
                false => {
                    moves.push(vec![i, _position[1]]);
                    break
                }
            }},
            None => moves.push(vec![i, _position[1]])
        }
    }

    // right
    for i in _position[1]+1..8 {
        match _game.board[_position[0]][i]{
            Some(x) => {match x.unwrap() == &_colour {
                true => break,
                false => {
                    moves.push(vec![i, _position[1]]);
                    break
                }
            }},
            None => moves.push(vec![i, _position[1]])
        }
    }

    // left
    for i in (_position[1]-1..=0).rev() {
        match _game.board[_position[0]][i]{
            Some(x) => {match x.unwrap() == &_colour {
                true => break,
                false => {
                    moves.push(vec![i, _position[1]]);
                    break
                }
            }},
            None => moves.push(vec![i, _position[1]])
        }
    }
    
    moves
}

fn get_diagonal_moves(_position: &Vec<usize>, _game: &Game, _colour: Colour) -> Vec<Vec<usize>> {
    let mut moves: Vec<Vec<usize>> = Vec::default();
    let mut pos: Vec<isize> = vec![_position[0] as isize,_position[1] as isize];
    
    // down right
    while pos[0] < 7 && pos[1] < 7 {
        pos[0] += 1;
        pos[1] += 1;
        match _game.board[pos[0] as usize][pos[1] as usize] {
            Some(x) => {match x.unwrap() == &_colour {
                true => break,
                false => {
                    moves.push(vec![pos[0] as usize, pos[1] as usize]);
                    break
                }
            }},
            None => moves.push(vec![pos[0] as usize, pos[1] as usize])
        }
    }

    pos = vec![_position[0] as isize,_position[1] as isize];

    // down left
    while pos[0] < 7 && pos[1] > 0 {
        pos[0] += 1;
        pos[1] -= 1;
        match _game.board[pos[0] as usize][pos[1] as usize] {
            Some(x) => {match x.unwrap() == &_colour {
                true => break,
                false => {
                    moves.push(vec![pos[0] as usize, pos[1] as usize]);
                    break
                }
            }},
            None => moves.push(vec![pos[0] as usize, pos[1] as usize])
        }
    }

    pos = vec![_position[0] as isize,_position[1] as isize];

    // up right
    while pos[0] > 0 && pos[1] < 7 {
        pos[0] -= 1;
        pos[1] += 1;
        match _game.board[pos[0] as usize][pos[1] as usize] {
            Some(x) => {match x.unwrap() == &_colour {
                true => break,
                false => {
                    moves.push(vec![pos[0] as usize, pos[1] as usize]);
                    break
                }
            }},
            None => moves.push(vec![pos[0] as usize, pos[1] as usize])
        }
    }

    pos = vec![_position[0] as isize,_position[1] as isize];

    // up left
    while pos[0] > 0 && pos[1] > 0 {
        pos[0] -= 1;
        pos[1] -= 1;
        match _game.board[pos[0] as usize][pos[1] as usize] {
            Some(x) => {match x.unwrap() == &_colour {
                true => break,
                false => {
                    moves.push(vec![pos[0] as usize, pos[1] as usize]);
                    break
                }
            }},
            None => moves.push(vec![pos[0] as usize, pos[1] as usize])
        }
    }

    moves
}

fn check_occupied(_position: &Vec<isize>, _game: &Game, _colour: Colour) -> bool {
    match _game.board[_position[0] as usize][_position[1] as usize] {
        Some(piece) => piece.unwrap() == &_colour,
        None => false
    }
}

fn check_check(_postion: &Vec<isize>, _game: &Game, _colour: Colour) -> bool {
    let mut in_check = false;
    for i in _game.board {
        for j in i {
            match j {
                Some(x) => {match x.unwrap() != &_colour {
                        true => {
                            let file = _game.board.iter().position(|x| *x == i).unwrap();
                            let rank = _game.board[file].iter().position(|x| *x == j).unwrap(); 
                            in_check = get_availble_moves(x, vec![file, rank], _game, false).iter().any(|x| x == &vec![_postion[0] as usize, _postion[1] as usize])
                        },
                        _ => continue
                    }
                },
                None => continue
            };
        }
    }
    in_check
}

fn check_pinned(_position: &Vec<usize>, _game: &Game, _moves: &Vec<Vec<usize>>, _colour: Colour) -> Vec<Vec<usize>> {
    // get position of king
    let mut king_position: Vec<usize> = Vec::default();
    for i in _game.board {
        for j in i {
            match j {
                Some(p) => {
                    match p {
                        Piece::King(c) => if c == _colour { king_position = vec![_game.board.iter().position(|x| *x == i).unwrap(), i.iter().position(|x| *x == j).unwrap()] },
                        _ => continue
                    }
                },
                None => continue
            }
        }
    }

    // get the squares a possible pinning piece can be blocked from
    // worth noting is that the only pieces that can pin is the Queen, Bishops and Rooks
    let mut pin_line: Vec<Vec<usize>> = Vec::default();
    let mut pinned: bool = false;
    for i in _game.board {
        for j in i {
            match j {
                Some(p) => {
                    let file = _game.board.iter().position(|x| *x == i).unwrap();
                    let rank = i.iter().position(|x| *x == j).unwrap();
                    let positon = vec![file,rank];
                    match p {
                        Piece::Queen(c) => {if c != _colour {
                            match get_diagonal_pin(&positon,_position,&king_position,_game) {
                                Some(x) => {
                                    pin_line = x;
                                    pinned = true;
                                    break
                                },
                                None => ()
                            }
                            match get_straight_pin(&positon,_position,&king_position,_game) {
                                Some(x) => {
                                    pin_line = x;
                                    pinned = true;
                                    break
                                },
                                None => ()
                            }
                        }},
                        Piece::Bishop(c) => {if c != _colour {
                            match get_diagonal_pin(&positon,_position,&king_position,_game) {
                                Some(x) => {
                                    pin_line = x;
                                    pinned = true;
                                    break
                                },
                                None => ()
                            }
                        }},
                        Piece::Rook(c) => {if c != _colour {
                            match get_straight_pin(&positon,_position,&king_position,_game) {
                                Some(x) => {
                                    pin_line = x;
                                    pinned = true;
                                    break
                                },
                                None => ()
                            }
                        }},
                        _ => continue
                    }
                },
                None => continue
            }
        }
        if pinned { break }
    }
    
    // if a pin is found, create new vector of all moves that don't reveal the king
    if pinned {
        let mut nmv: Vec<Vec<usize>> = Vec::default();
        for i in &pin_line {
            if _moves.iter().any(|x| x == i) {
                nmv.push(i.to_owned());
            }
        }
        nmv
    } else {
        _moves.to_owned()
    }
}

fn get_diagonal_pin(_position: &Vec<usize>, _pinned: &Vec<usize>, _king: &Vec<usize>, _game: &Game) -> Option<Vec<Vec<usize>>> {
    None
}

fn get_straight_pin(_position: &Vec<usize>, _pinned: &Vec<usize>, _king: &Vec<usize>, _game: &Game) -> Option<Vec<Vec<usize>>> {
    None
}

fn resolve_check(_position: &Vec<usize>, _game: &Game, _moves: &Vec<Vec<usize>>, _colour: Colour) -> Vec<Vec<usize>> {
    // get position of king
    let mut king_position: Vec<usize> = Vec::default();
    for i in _game.board {
        for j in i {
            match j {
                Some(p) => {
                    match p {
                        Piece::King(c) => if c == _colour { king_position = vec![_game.board.iter().position(|x| *x == i).unwrap(), i.iter().position(|x| *x == j).unwrap()] },
                        _ => continue
                    }
                },
                None => continue
            }
        }
    }

    let mut checking: Vec<Vec<usize>> = Vec::default();
    for i in _game.board {
        for j in i {
            match j {
                Some(p) => {
                    if p.unwrap() != &_colour {
                        let file = _game.board.iter().position(|x| *x == i).unwrap();
                        let rank = i.iter().position(|x| *x == j).unwrap();
                        let positon = vec![file,rank];

                        if get_availble_moves(p, &positon, _game, false).iter().any(|x| *x == king_position) {
                            checking.push(positon);
                        }
                    }
                },
                None => continue
            }
        }
    }

    // when there are multiple pices checking there is no way to resolve all checks in a single move exept moving the king
    if checking.len() > 1 {
        vec![vec![]]
    } else {

        let check_resolve = match _game.board[checking[0][0]][checking[0][1]].unwrap() {
            Piece::Queen(c) => {
                match match get_diagonal_check(&checking[0], &king_position, _game, c) {
                    Some(x) => Some(x),
                    None => None
                } {
                    Some(x) => x,
                    None => get_straight_check(&checking[0], &king_position, _game, c).unwrap()
                }
            },
            Piece::Bishop(c) => get_diagonal_check(&checking[0], &king_position, _game, c).unwrap(),
            Piece::Rook(c) => get_straight_check(&checking[0], &king_position, _game, c).unwrap(),
            _ => vec![checking[0]]
        };
        let mut nmv: Vec<Vec<usize>> = Vec::default();
        
        for i in _moves {
            if check_resolve.iter().any(|x| x == i) {
                nmv.push(i.to_owned());
            }
        }

        nmv
    }

}

fn get_diagonal_check(_position: &Vec<usize>, _king: &Vec<usize>, _game: &Game, _colour: Colour) -> Option<Vec<Vec<usize>>> {
    None
}

fn get_straight_check(_position: &Vec<usize>, _king: &Vec<usize>, _game: &Game, _colour: Colour) -> Option<Vec<Vec<usize>>> {
    None
}


impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            active: Colour::White,
            board: [[Some(Piece::Rook(Colour::White)),Some(Piece::Knight(Colour::White)),Some(Piece::Bishop(Colour::White)),Some(Piece::Queen(Colour::White)),Some(Piece::King(Colour::White)),Some(Piece::Bishop(Colour::White)),Some(Piece::Knight(Colour::White)),Some(Piece::Rook(Colour::White))],
                    [Some(Piece::Pawn(Colour::White)),Some(Piece::Pawn(Colour::White)),Some(Piece::Pawn(Colour::White)),Some(Piece::Pawn(Colour::White)),Some(Piece::Pawn(Colour::White)),Some(Piece::Pawn(Colour::White)),Some(Piece::Pawn(Colour::White)),Some(Piece::Pawn(Colour::White))],
                    [None,None,None,None,None,None,None,None],
                    [None,None,None,None,None,None,None,None],
                    [None,None,None,None,None,None,None,None],
                    [None,None,None,None,None,None,None,None],
                    [Some(Piece::Pawn(Colour::Black)),Some(Piece::Pawn(Colour::Black)),Some(Piece::Pawn(Colour::Black)),Some(Piece::Pawn(Colour::Black)),Some(Piece::Pawn(Colour::Black)),Some(Piece::Pawn(Colour::Black)),Some(Piece::Pawn(Colour::Black)),Some(Piece::Pawn(Colour::Black))],
                    [Some(Piece::Rook(Colour::Black)),Some(Piece::Knight(Colour::Black)),Some(Piece::Bishop(Colour::Black)),Some(Piece::Queen(Colour::Black)),Some(Piece::King(Colour::Black)),Some(Piece::Bishop(Colour::Black)),Some(Piece::Knight(Colour::Black)),Some(Piece::Rook(Colour::Black))]]
        }
    }

    /// If the current game state is InProgress and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        None
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _piece: String) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _position: String) -> Option<Vec<String>> {
        let position = decode_position(&_position).unwrap();
        match self.board[position[0]][position[1]] {
            Some(piece) => {
                if piece.unwrap() != &self.active {
                    return None
                }
                let moves = get_availble_moves(piece, &position, &self, true);
        
                let c_moves = code_moves(moves);
                Some(c_moves)
            }
            None => None
        }
    }
}

/// Implement print routine for Game.
/// 
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        let mut board: String = "|:------------------------:|".to_owned();
        for i in self.board {
            board += "\n|  ";
            for j in i {
                let piece: String = match j {
                    Some(p) => p.dis(),
                    None => "*  ".to_owned()
                };
                board += &piece;
            }
            board += "|";
        }
        board += "\n|:------------------------:|";

        write!(f, "{}", board)
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {

        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}
