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

fn get_availble_moves(_piece: Piece, _position: Vec<usize>, _game: &Game, _checkable: bool) -> Vec<Vec<usize>> {
    // get all moves
    let mut moves: Vec<Vec<usize>> = match _piece {
        Piece::King(_) => get_king_movement(&_position, _game, _checkable),
        Piece::Queen(_) => {
            let mut moves = get_straight_moves(&_position, _game);
            moves.append(&mut get_diagonal_moves(&_position,_game));
            moves
        },
        Piece::Bishop(_) => get_diagonal_moves(&_position, _game),
        Piece::Knight(_) => get_knight_moves(&_position,_game),
        Piece::Rook(_) => get_straight_moves(&_position, _game),
        Piece::Pawn(_) => get_pawn_moves(&_position,_game)
    };

    // remove illegal moves
    // if pinned, remove moves that reveal king
    match _piece {
        Piece::King(_) => (),
        _ => check_pinned(&_position, _game, &mut moves)
    }

    // if in check, remove moves that do not resolve check
    if _game.state == GameState::Check {
        resolve_check(&_position, _game, &mut moves);
    }

    moves
}


fn get_king_movement(_position: &Vec<usize>, _game: &Game, _checkable: bool) -> Vec<Vec<usize>> {
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
        if mv[0] < 0 || mv[0] > 7 || mv[1] < 0 || mv[1] > 7 || check_occupied(&mv, _game) || check_check(&mv, _game) {
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

fn get_knight_moves(_position: &Vec<usize>, _game: &Game) -> Vec<Vec<usize>> {
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
        if mv[0] < 0 || mv[0] > 7 || mv[1] < 0 || mv[1] > 7 || check_occupied(&mv, _game) {
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
            // check if on side, meaning that moving to that side is not possible
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

fn get_straight_moves(_position: &Vec<usize>, _game: &Game) -> Vec<Vec<usize>> {
    let mut moves: Vec<Vec<usize>> = Vec::default();

    // down
    for i in _position[0]+1..8 {
        match _game.board[i][_position[1]]{
            Some(x) => {match x.unwrap() == &_game.active {
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
            Some(x) => {match x.unwrap() == &_game.active {
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
            Some(x) => {match x.unwrap() == &_game.active {
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
            Some(x) => {match x.unwrap() == &_game.active {
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
            Some(x) => {match x.unwrap() == &_game.active {
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

fn get_diagonal_moves(_position: &Vec<usize>, _game: &Game) -> Vec<Vec<usize>> {
    let mut moves: Vec<Vec<usize>> = Vec::default();
    let mut pos: Vec<isize> = vec![_position[0] as isize,_position[1] as isize];
    
    // down right
    while pos[0] < 7 && pos[1] < 7 {
        pos[0] += 1;
        pos[1] += 1;
        match _game.board[pos[0] as usize][pos[1] as usize] {
            Some(x) => {match x.unwrap() == &_game.active {
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
            Some(x) => {match x.unwrap() == &_game.active {
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
            Some(x) => {match x.unwrap() == &_game.active {
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
            Some(x) => {match x.unwrap() == &_game.active {
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

fn check_occupied(_position: &Vec<isize>, _game: &Game) -> bool {
    match _game.board[_position[0] as usize][_position[1] as usize] {
        Some(piece) => piece.unwrap() == &_game.active,
        None => false
    }
}

fn check_check(_postion: &Vec<isize>, _game: &Game) -> bool {
    for i in _game.board {
        for j in i {
            match j {
                Some(x) => {match x.unwrap() != &_game.active {
                        true => {
                            let file = _game.board.iter().position(|x| *x == i).unwrap();
                            let rank = _game.board[file].iter().position(|x| *x == j).unwrap(); 
                            return get_availble_moves(x, vec![file, rank], _game, false).iter().any(|x| x == &vec![_postion[0] as usize, _postion[1] as usize])
                        },
                        _ => return false
                    }
                },
                None => return false
            }
        }
    }
    false
}

fn check_pinned(_position: &Vec<usize>, _game: &Game, _moves: &mut Vec<Vec<usize>>) {

}

fn resolve_check(_position: &Vec<usize>, _game: &Game, _moves: &mut Vec<Vec<usize>>) {

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
                let moves = get_availble_moves(piece, position, &self, true);
        
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
