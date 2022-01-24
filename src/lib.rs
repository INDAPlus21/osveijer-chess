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

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Colour {
    White,
    Black
}

#[derive(PartialEq, Copy, Clone, Debug)]
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
    fn unwrap(&self) -> Colour {
        match self {
            Piece::King(x) => x.to_owned(),
            Piece::Queen(x) => x.to_owned(),
            Piece::Bishop(x) => x.to_owned(),
            Piece::Knight(x) => x.to_owned(),
            Piece::Rook(x) => x.to_owned(),
            Piece::Pawn(x) => x.to_owned()
        }
    }

    fn dis(&self) -> String {
        let out = match self {
            Piece::King(_) => "K  ".to_owned(),
            Piece::Queen(_) => "Q  ".to_owned(),
            Piece::Bishop(_) => "B  ".to_owned(),
            Piece::Knight(_) => "Kn ".to_owned(),
            Piece::Rook(_) => "R  ".to_owned(),
            Piece::Pawn(_) => "P  ".to_owned()
        };
        match self.unwrap() {
            Colour::Black => out.to_lowercase(),
            Colour::White => out
        }
    }
}

#[derive(Copy,Clone)]
pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    pub active: Colour,
    pub board: [[Option<Piece>;8];8],
    white_promotion: Piece,
    black_promotion: Piece
}


// take &str in format "<file><rank>" and convert to vector of i8 with format [<column>,<row>]
fn decode_position(_position: &str) -> Vec<usize> {
    let mut vec: Vec<usize> = Vec::default();
    vec.push(_position.chars().nth(1).unwrap().to_string().parse::<usize>().unwrap() - 1); // - 1 because index starts at 0
    vec.push(FILES.iter().position(|&c| c == _position.chars().nth(0).unwrap()).unwrap());
    vec
}

fn code_moves(_moves: &Vec<Vec<usize>>) -> Vec<String> {
    let mut moves: Vec<String> = Vec::default();
    for i in _moves {
        moves.push(FILES[i[1]].to_string() + &(i[0]+1).to_string());
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
        if mv[0] >= 0 && mv[0] <= 7 && mv[1] >= 0 && mv[1] <= 7 && !check_occupied(&mv, _game, _colour) && (!_checkable || !check_check(&vec![mv[0] as usize, mv[1] as usize], _game, _colour)) {
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

fn get_takes(_piece: &Piece, _position: &Vec<usize>, _colour: Colour, _game: &Game) -> Vec<Vec<usize>> {
    match _piece {
        Piece::Pawn(c) => get_pawn_takes(_position, *c),
        Piece::King(_) => get_king_takes(_position),
        Piece::Knight(_) => get_knight_takes(_position),
        Piece::Rook(_) => get_straight_takes(_position, _colour, _game),
        Piece::Bishop(_) => get_diagonal_takes(_position, _colour, _game),
        Piece::Queen(_) => {
            let mut out = get_diagonal_takes(_position, _colour, _game);
            out.append(&mut get_straight_takes(_position, _colour, _game));
            out
        }
    }
}

fn get_pawn_takes(_position: &Vec<usize>, _colour: Colour) -> Vec<Vec<usize>> {
    // check colour to get direction of movement
    match _colour {
        Colour::White => {
            let mut mvs: Vec<Vec<usize>> = Vec::default();
            // check if on edge, meaning that moving towards that edge is not possible
            let takes: Vec<Vec<isize>> = match _position[1] {
                0 => vec![vec![1,1]],
                7 => vec![vec![1,-1]],
                _ => vec![vec![1,1],vec![1,-1]]
            };
            // check if there is a piece to take
            for mv in takes {
                let m = vec![(_position[0] as isize + mv[0]) as usize, (_position[1] as isize + mv[1]) as usize];
                mvs.push(m);
            }

            mvs
        },
        Colour::Black => {
            let mut mvs: Vec<Vec<usize>> = Vec::default();
            // check if on side, meaning that moving to that side is not possible
            let takes: Vec<Vec<isize>> = match _position[1] {
                0 => vec![vec![-1,1]],
                7 => vec![vec![-1,-1]],
                _ => vec![vec![-1,1],vec![-1,-1]]
            };
            // check if there is a piece to take
            for mv in takes {
                let m = vec![(_position[0] as isize + mv[0]) as usize, (_position[1] as isize + mv[1]) as usize];
                mvs.push(m);
            }

            mvs
        }

    }
}

fn get_king_takes(_position: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut takes: Vec<Vec<usize>> = Vec::default();
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

    for i in offsets {
        let mv = vec![_position[0] as isize + i.0, _position[1] as isize + i.1];
        if mv[0] >= 0 && mv[0] <= 7 && mv[1] >= 0 && mv[1] <= 7 {
            takes.push(vec![mv[0] as usize, mv[1] as usize]);
        }
    }

    takes
}

fn get_knight_takes(_position: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut takes: Vec<Vec<usize>> = Vec::default();
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
    
    for i in offsets {
        let mv = vec![_position[0] as isize + i.0, _position[1] as isize + i.1];
        if mv[0] >= 0 && mv[0] <= 7 && mv[1] >= 0 && mv[1] <= 7 {
            takes.push(vec![mv[0] as usize, mv[1] as usize]);
        }
    }

    takes
}

fn get_straight_takes(_position: &Vec<usize>, _colour: Colour, _game: &Game) -> Vec<Vec<usize>> {
    let mut takes: Vec<Vec<usize>> = end_takes(get_line(_position, 1, 0), _colour, _game);
    takes.append(&mut end_takes(get_line(_position, -1, 0), _colour, _game));
    takes.append(&mut end_takes(get_line(_position, 0, 1), _colour, _game));
    takes.append(&mut end_takes(get_line(_position, 0, -1), _colour, _game));
    takes
}

fn get_diagonal_takes(_position: &Vec<usize>, _colour: Colour, _game: &Game) -> Vec<Vec<usize>> {
    let mut takes: Vec<Vec<usize>> = end_takes(get_line(_position, 1, -1), _colour, _game);
    takes.append(&mut end_takes(get_line(_position, -1, 1), _colour, _game));
    takes.append(&mut end_takes(get_line(_position, 1, 1), _colour, _game));
    takes.append(&mut end_takes(get_line(_position, -1, -1), _colour, _game));
    takes
}

fn end_takes(_line: Vec<Vec<usize>>, _colour: Colour, _game: &Game) -> Vec<Vec<usize>> {
    let mut out: Vec<Vec<usize>> = Vec::default();
    for i in _line[1..].iter() {
        if _game.board[i[0]][i[1]] == None || _game.board[i[0]][i[1]] == Some(Piece::King(_colour)) {
            out.push(i.to_owned());
        } else {
            out.push(i.to_owned());
            break
        }
    }
    out
}

fn get_straight_moves(_position: &Vec<usize>, _game: &Game, _colour: Colour) -> Vec<Vec<usize>> {
    let mut moves: Vec<Vec<usize>> = Vec::default();

    // up
    for i in (0.._position[0]).rev() {
        match _game.board[i][_position[1]]{
            Some(x) => {match x.unwrap() == _colour {
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
            Some(x) => {match x.unwrap() == _colour {
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
            Some(x) => {match x.unwrap() == _colour {
                true => break,
                false => {
                    moves.push(vec![_position[0], i]);
                    break
                }
            }},
            None => moves.push(vec![_position[0], i])
        }
    }

    // left
    for i in (0.._position[1]).rev() {
        match _game.board[_position[0]][i]{
            Some(x) => {match x.unwrap() == _colour {
                true => break,
                false => {
                    moves.push(vec![_position[0], i]);
                    break
                }
            }},
            None => moves.push(vec![_position[0], i])
        }
    }
    
    moves
}

fn get_diagonal_moves(_position: &Vec<usize>, _game: &Game, _colour: Colour) -> Vec<Vec<usize>> {
    let mut moves: Vec<Vec<usize>> = Vec::default();
    
    // down right
    let line = get_line(_position, 1, 1);
    for i in line[1..].iter() {
        match _game.board[i[0] as usize][i[1] as usize] {
            Some(x) => {match x.unwrap() == _colour {
                true => break,
                false => {
                    moves.push(vec![i[0] as usize, i[1] as usize]);
                    break
                }
            }},
            None => moves.push(vec![i[0] as usize, i[1] as usize])
        }
    }

    // down left
    let line = get_line(_position, 1, -1);
    for i in line[1..].iter() {
        match _game.board[i[0] as usize][i[1] as usize] {
            Some(x) => {match x.unwrap() == _colour {
                true => break,
                false => {
                    moves.push(vec![i[0] as usize, i[1] as usize]);
                    break
                }
            }},
            None => moves.push(vec![i[0] as usize, i[1] as usize])
        }
    }

    // up right
    let line = get_line(_position, -1, 1);
    for i in line[1..].iter() {
        match _game.board[i[0] as usize][i[1] as usize] {
            Some(x) => {match x.unwrap() == _colour {
                true => break,
                false => {
                    moves.push(vec![i[0] as usize, i[1] as usize]);
                    break
                }
            }},
            None => moves.push(vec![i[0] as usize, i[1] as usize])
        }
    }

    // up left
    let line = get_line(_position, -1, -1);
    for i in line[1..].iter() {
        match _game.board[i[0] as usize][i[1] as usize] {
            Some(x) => {match x.unwrap() == _colour {
                true => break,
                false => {
                    moves.push(vec![i[0] as usize, i[1] as usize]);
                    break
                }
            }},
            None => moves.push(vec![i[0] as usize, i[1] as usize])
        }
    }

    moves
}

fn check_occupied(_position: &Vec<isize>, _game: &Game, _colour: Colour) -> bool {
    match _game.board[_position[0] as usize][_position[1] as usize] {
        Some(piece) => piece.unwrap() == _colour,
        None => false
    }
}

fn check_check(_postion: &Vec<usize>, _game: &Game, _colour: Colour) -> bool {
    let mut in_check = false;
    for i in _game.board.iter() {
        for j in i {
            match j {
                Some(p) => {match p.unwrap() != _colour {
                        true => {
                            let file = _game.board.iter().position(|x| x == i).unwrap();
                            let rank = _game.board[file].iter().position(|x| x == j).unwrap();
                            let pos = vec![file,rank]; 
                            let takes = get_takes(p, &pos, _colour, _game);
                            in_check = takes.iter().any(|x| x == &vec![_postion[0] as usize, _postion[1] as usize]);
                            if in_check { break }
                        },
                        _ => continue
                    }
                },
                None => continue
            };
        }
        if in_check { break }
    }
    in_check
}

fn check_pinned(_position: &Vec<usize>, _game: &Game, _moves: &Vec<Vec<usize>>, _colour: Colour) -> Vec<Vec<usize>> {
    // get position of king
    let king_position = get_king_pos(_game, _colour);

    // get the squares a possible pinning piece can be blocked from
    // worth noting is that the only pieces that can pin is the Queen, Bishops and Rooks
    let mut pin_line: Vec<Vec<usize>> = Vec::default();
    let mut pinned: bool = false;
    for i in _game.board.iter() {
        for j in i {
            match j {
                Some(p) => {
                    let file = _game.board.iter().position(|x| x == i).unwrap();
                    let rank = i.iter().position(|x| x == j).unwrap();
                    let positon = vec![file,rank];
                    match p {
                        Piece::Queen(c) => {if *c != _colour {
                            match get_diagonal_pin(&positon,_position,&king_position,_game,0) {
                                Some(x) => {
                                    pin_line = x;
                                    pinned = true;
                                    break
                                },
                                None => ()
                            }
                            match get_straight_pin(&positon,_position,&king_position,_game,0) {
                                Some(x) => {
                                    pin_line = x;
                                    pinned = true;
                                    break
                                },
                                None => ()
                            }
                        }},
                        Piece::Bishop(c) => {if *c != _colour {
                            match get_diagonal_pin(&positon,_position,&king_position,_game,0) {
                                Some(x) => {
                                    pin_line = x;
                                    pinned = true;
                                    break
                                },
                                None => ()
                            }
                        }},
                        Piece::Rook(c) => {if *c != _colour {
                            match get_straight_pin(&positon,_position,&king_position,_game,0) {
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

fn get_diagonal_pin(_position: &Vec<usize>, _pinned: &Vec<usize>, _king: &Vec<usize>, _game: &Game, _dir: usize) -> Option<Vec<Vec<usize>>> {
    let dirs: Vec<Vec<isize>> = vec![vec![-1,1],vec![1,-1],vec![-1,-1],vec![1,1]];
    let mut line = get_line(_position, dirs[_dir][0], dirs[_dir][1]);
    let king_pos = line.iter().position(|p| p == _king);
    match king_pos {
        Some(x) => {
            while line.len() > x {
                line.remove(x);
            }
            match line.iter().position(|p| p == _pinned) {
                Some(x) => {
                    line.remove(x);
                    if line.len() == 1 {
                        Some(line)
                    } else if line.len() != 1 && line[1..].iter().any(|p| _game.board[p[0]][p[1]] != None) {
                        None
                    } else {
                        Some(line)
                    }
                },
                None => None
            }
        },
        None => {
            match _dir {
                0..=2 => get_diagonal_pin(_position, _pinned, _king, _game, _dir + 1),
                _ => None
            }
        }
    }
}

fn get_line(_position: &Vec<usize>, _dirx: isize, _diry: isize) -> Vec<Vec<usize>> {
    let mut line: Vec<Vec<usize>> = Vec::default();

    let mut pos = vec![_position[0] as isize,_position[1] as isize];
    
    while pos[0] >= 0 && pos[0] <= 7 && pos[1] >= 0 && pos[1] <= 7 {
        let upos = vec![pos[0] as usize, pos[1] as usize];
        line.push(upos);
        pos[0] += _dirx;
        pos[1] += _diry;
    }

    line
}

fn get_straight_pin(_position: &Vec<usize>, _pinned: &Vec<usize>, _king: &Vec<usize>, _game: &Game, _dir: usize) -> Option<Vec<Vec<usize>>> {
    let dirs: Vec<Vec<isize>> = vec![vec![-1,0],vec![1,0],vec![0,-1],vec![0,1]];
    let mut line = get_line(_position, dirs[_dir][0], dirs[_dir][1]);
    let king_pos = line.iter().position(|p| p == _king);
    match king_pos {
        Some(x) => {
            while line.len() > x {
                line.remove(x);
            }
            match line.iter().position(|p| p == _pinned) {
                Some(x) => {
                    line.remove(x);
                    if line.len() == 1 {
                        Some(line)
                    } else if line[1..].iter().any(|p| _game.board[p[0]][p[1]] != None) {
                        None
                    } else {
                        Some(line)
                    }
                },
                None => None
            }
        },
        None => {
            match _dir {
                0..=2 => get_straight_pin(_position, _pinned, _king, _game, _dir + 1),
                _ => None
            }
        }
    }
}

fn resolve_check(_position: &Vec<usize>, _game: &Game, _moves: &Vec<Vec<usize>>, _colour: Colour) -> Vec<Vec<usize>> {
    // get position of king
    let king_position: Vec<usize> = get_king_pos(_game, _colour);

    let mut checking: Vec<Vec<usize>> = Vec::default();
    for i in _game.board.iter() {
        for j in i {
            match j {
                Some(p) => {
                    if p.unwrap() != _colour {
                        let file = _game.board.iter().position(|x| x == i).unwrap();
                        let rank = i.iter().position(|x| x == j).unwrap();
                        let positon = vec![file,rank];

                        if get_availble_moves(*p, &positon, _game, false).iter().any(|x| *x == king_position) {
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
                match get_diagonal_check(&checking[0], &king_position, _game, c, 0) {
                    Some(x) => x,
                    None => get_straight_check(&checking[0], &king_position, _game, c, 0).unwrap()
                }
            },
            Piece::Bishop(c) => get_diagonal_check(&checking[0], &king_position, _game, c, 0).unwrap(),
            Piece::Rook(c) => get_straight_check(&checking[0], &king_position, _game, c, 0).unwrap(),
            _ => vec![checking[0].clone()]
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

fn get_diagonal_check(_position: &Vec<usize>, _king: &Vec<usize>, _game: &Game, _colour: Colour, _dir: usize) -> Option<Vec<Vec<usize>>> {
    let dirs: Vec<Vec<isize>> = vec![vec![-1,1],vec![1,-1],vec![-1,-1],vec![1,1]];
    let mut line = get_line(_position, dirs[_dir][0], dirs[_dir][1]);
    let king_pos = line.iter().position(|p| p == _king);
    match king_pos {
        Some(x) => {
            while line.len() > x {
                line.remove(x);
            }
            if line[1..].iter().any(|p| _game.board[p[0]][p[1]] != None) {
                None
            } else {
                Some(line)
            }
        },
        None => {
            match _dir {
                0..=2 => get_diagonal_check(_position, _king, _game, _colour, _dir + 1),
                _ => None
            }
        }
    }
}

fn get_straight_check(_position: &Vec<usize>, _king: &Vec<usize>, _game: &Game, _colour: Colour, _dir: usize) -> Option<Vec<Vec<usize>>> {
    let dirs: Vec<Vec<isize>> = vec![vec![-1,0],vec![1,0],vec![0,-1],vec![0,1]];
    let mut line = get_line(_position, dirs[_dir][0], dirs[_dir][1]);
    let king_pos = line.iter().position(|p| p == _king);
    match king_pos {
        Some(x) => {
            while line.len() > x {
                line.remove(x);
            }
            if line[1..].iter().any(|p| _game.board[p[0]][p[1]] != None) {
                None
            } else {
                Some(line)
            }
        },
        None => {
            match _dir {
                0..=2 => get_straight_check(_position, _king, _game, _colour, _dir + 1),
                _ => None
            }
        }
    }
}

fn game_state_change(_game: &mut Game) {
    _game.state = GameState::InProgress;
    let colour = match _game.active {
        Colour::Black => Colour::White,
        Colour::White => Colour::Black
    };
    if check_check(&get_king_pos(_game, colour), _game, colour) {
        _game.state = GameState::Check;
        let prev = _game.active;
        _game.active = colour;
        if check_mate(_game, colour) {
            _game.state = GameState::GameOver;
        }
        _game.active = prev;
    }
}

fn get_king_pos(_game: &Game, _colour: Colour) -> Vec<usize> {
    let mut king_position: Vec<usize> = Vec::default();
    for i in _game.board.iter() {
        for j in i {
            match j {
                Some(p) => {
                    match p {
                        Piece::King(c) => if *c == _colour { 
                            king_position = vec![_game.board.iter().position(|x| x == i).unwrap(), i.iter().position(|x| x == j).unwrap()];
                            break 
                        },
                        _ => continue
                    }
                },
                None => continue
            }
        }
        if king_position != Vec::default() { break }
    }
    king_position
}

fn check_mate(_game: &Game, _colour: Colour) -> bool {
    let mut mate = true;
    for i in _game.board.iter() {
        for j in i {
            match j {
                Some(p) => {
                    match p.unwrap() == _colour {
                        true => {
                            let pos = code_moves(&vec![vec![_game.board.iter().position(|f| f == i).unwrap(), i.iter().position(|r| r == j).unwrap()]])[0].clone();
                            let moves = _game.get_possible_moves(pos.clone());
                            match moves {
                                Some(x) => {
                                    match x.len() {
                                        0 => continue,
                                        _ => {
                                            mate = false;
                                            break
                                        }
                                    }
                                },
                                None => continue
                            }
                        },
                        false => continue
                    }
                },
                None => continue
            }
        }
        if !mate { break }
    }
    mate
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
                    [Some(Piece::Rook(Colour::Black)),Some(Piece::Knight(Colour::Black)),Some(Piece::Bishop(Colour::Black)),Some(Piece::Queen(Colour::Black)),Some(Piece::King(Colour::Black)),Some(Piece::Bishop(Colour::Black)),Some(Piece::Knight(Colour::Black)),Some(Piece::Rook(Colour::Black))]],
            white_promotion: Piece::Queen(Colour::White),
            black_promotion: Piece::Queen(Colour::Black)
        }
    }

    /// If the current game state is InProgress and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        match self.state {
            GameState::GameOver => None,
            _ => {
                let from_position = decode_position(&_from);
                let to_position = decode_position(&_to);
                let piece = self.board[from_position[0]][from_position[1]];
                match piece {
                    Some(p) => {
                        match p.unwrap() == self.active {
                            true => {
                                match self.get_possible_moves(_from).unwrap().iter().any(|m| m == &_to) {
                                    true => {
                                        self.board[to_position[0]][to_position[1]] = self.board[from_position[0]][from_position[1]];
                                        self.board[from_position[0]][from_position[1]] = None;
                                        match p {
                                            Piece::Pawn(c) => {
                                                match c {
                                                    Colour::Black => {
                                                        if to_position[0] == 0 {
                                                            self.board[to_position[0]][to_position[1]] = Some(self.black_promotion);
                                                        }
                                                    },
                                                    Colour::White => {
                                                        if to_position[0] == 7 {
                                                            self.board[to_position[0]][to_position[1]] = Some(self.white_promotion);
                                                        }
                                                    }
                                                }
                                            },
                                            _ => ()
                                        };
                                        game_state_change(self);
                                        self.active = match self.active {
                                            Colour::Black => Colour::White,
                                            Colour::White => Colour::Black
                                        };
                                        Some(self.state)
                                    },
                                    false => None
                                }
                            },
                            false => None
                        }
                    },
                    None => None
         
                }
            }
        }
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _piece: String) -> () {
        match self.active {
            Colour::Black => {
                self.black_promotion = match &_piece[..] {
                    "q" => Piece::Queen(Colour::Black),
                    "kn" => Piece::Knight(Colour::Black),
                    "r" => Piece::Rook(Colour::Black),
                    "b" => Piece::Bishop(Colour::Black),
                    _ => Piece::Queen(Colour::Black)
                };
            },
            Colour::White => {
                self.white_promotion = match &_piece[..] {
                    "q" => Piece::Queen(Colour::White),
                    "kn" => Piece::Knight(Colour::White),
                    "r" => Piece::Rook(Colour::White),
                    "b" => Piece::Bishop(Colour::White),
                    _ => Piece::Queen(Colour::White)
                };
            }
        }
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
        let position = decode_position(&_position);
        match self.board[position[0]][position[1]] {
            Some(piece) => {
                if piece.unwrap() != self.active {
                    return None
                }
                let moves = get_availble_moves(piece, &position, &self, true);
        
                let c_moves = code_moves(&moves);
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
        let mut board: String = "\n|:------------------------:|".to_owned();
        for i in self.board.iter() {
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
    use super::Piece;
    use super::Colour;

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

    // check available moves for all kinds of pieces
    #[test]
    fn all_legal_moves_gotten() {
        
        let mut game = Game::new();

        game.board =    [[None,None,None,None,None,None,None,None],
                        [None,None,Some(Piece::Rook(Colour::White)),None,None,None,None,None],
                        [None,None,None,None,Some(Piece::King(Colour::White)),None,None,Some(Piece::Bishop(Colour::White))],
                        [None,Some(Piece::Pawn(Colour::White)),None,Some(Piece::Pawn(Colour::White)),None,None,None,None],
                        [None,None,Some(Piece::Queen(Colour::Black)),None,None,None,None,None],
                        [None,Some(Piece::Knight(Colour::White)),Some(Piece::King(Colour::Black)),None,None,None,None,None],
                        [None,None,None,Some(Piece::Pawn(Colour::Black)),Some(Piece::Queen(Colour::White)),None,None,None],
                        [None,None,None,None,None,Some(Piece::Knight(Colour::Black)),None,None]];
        let mut moves = game.get_possible_moves("c2".to_string()).unwrap();
        moves.sort();
        assert_eq!(moves, vec!["a2".to_string(), "b2".to_string(), "c1".to_string(), "c3".to_string(), "c4".to_string(), "c5".to_string(), "d2".to_string(), "e2".to_string(), "f2".to_string(), "g2".to_string(), "h2".to_string()]);
        
        moves = game.get_possible_moves("h3".to_string()).unwrap();
        moves.sort();
        assert_eq!(moves, vec!["d7".to_string(), "e6".to_string(), "f1".to_string(), "f5".to_string(), "g2".to_string(), "g4".to_string()]);

        moves = game.get_possible_moves("b6".to_string()).unwrap();
        moves.sort();
        assert_eq!(moves, vec!["a4".to_string(), "a8".to_string(), "c4".to_string(), "c8".to_string(), "d5".to_string(), "d7".to_string()]);

        moves = game.get_possible_moves("e7".to_string()).unwrap();
        moves.sort();
        assert_eq!(moves, vec!["c5".to_string(), "d6".to_string(), "d7".to_string(), "d8".to_string(), "e4".to_string(), "e5".to_string(), "e6".to_string(), "e8".to_string(), "f6".to_string(), "f7".to_string(), "f8".to_string(), "g5".to_string(), "g7".to_string(), "h4".to_string(), "h7".to_string()]);
        
        moves = game.get_possible_moves("b4".to_string()).unwrap();
        moves.sort();
        assert_eq!(moves, vec!["b5".to_string(), "c5".to_string()]);

        // pin
        moves = game.get_possible_moves("d4".to_string()).unwrap();
        moves.sort();
        assert_eq!(moves, vec!["c5".to_string()]);
        
        game.active = Colour::Black;
        
        moves = game.get_possible_moves("c6".to_string()).unwrap();
        moves.sort();
        assert_eq!(moves, vec!["b5".to_string(), "b6".to_string(), "b7".to_string(), "c7".to_string()]); 
        
        // pin
        moves = game.get_possible_moves("c5".to_string()).unwrap();
        moves.sort();
        assert_eq!(moves, vec!["c2".to_string(), "c3".to_string(), "c4".to_string()]); 
        
        moves = game.get_possible_moves("d7".to_string()).unwrap();
        moves.sort();
        assert_eq!(moves, vec!["d5".to_string(), "d6".to_string()]); 
        
        moves = game.get_possible_moves("f8".to_string()).unwrap();
        moves.sort();
        assert_eq!(moves, vec!["e6".to_string(), "g6".to_string(), "h7".to_string()]); 
        
        // resolving checks

        game.board = [[None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [None,None,Some(Piece::King(Colour::White)),None,None,None,None,None],
        [None,None,None,Some(Piece::Bishop(Colour::White)),None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [None,None,None,Some(Piece::Knight(Colour::Black)),None,None,Some(Piece::King(Colour::Black)),None],
        [None,None,None,None,None,None,None,None]];
        game.state = GameState::Check;
        game.active = Colour::Black;

        moves = game.get_possible_moves("d7".to_string()).unwrap();
        moves.sort();
        assert_eq!(moves, vec!["e5".to_string(), "f6".to_string()]);

        moves = game.get_possible_moves("g7".to_string()).unwrap();
        moves.sort();
        assert_eq!(moves, vec!["f7".to_string(), "f8".to_string(), "g6".to_string(), "g8".to_string(), "h6".to_string(), "h7".to_string()]);

        game.board = [[None,None,None,None,None,None,None,None],
        [None,None,None,None,None,Some(Piece::Bishop(Colour::White)),None,None],
        [None,None,Some(Piece::King(Colour::White)),None,None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,Some(Piece::King(Colour::Black)),None],
        [None,None,Some(Piece::Rook(Colour::Black)),None,None,None,None,None]];
        game.state = GameState::Check;
        game.active = Colour::White;

        moves = game.get_possible_moves("f2".to_string()).unwrap();
        moves.sort();
        assert_eq!(moves, vec!["c5".to_string()]);

        moves = game.get_possible_moves("c3".to_string()).unwrap();
        moves.sort();
        assert_eq!(moves, vec!["b2".to_string(), "b3".to_string(), "b4".to_string(), "d2".to_string(), "d3".to_string(), "d4".to_string()]);
    }

    #[test]
    fn move_piece() {
        let mut game = Game::new();
        assert_ne!(game.make_move("a2".to_string(), "a4".to_string()), None);
        assert_ne!(game.make_move("h7".to_string(), "h5".to_string()), None);
        assert_ne!(game.make_move("b1".to_string(), "c3".to_string()), None);
        assert_ne!(game.make_move("e7".to_string(), "e6".to_string()), None);
        assert_ne!(game.make_move("g1".to_string(), "h3".to_string()), None);
        assert_ne!(game.make_move("d8".to_string(), "h4".to_string()), None);
        assert_ne!(game.make_move("c3".to_string(), "d5".to_string()), None);
        assert_ne!(game.make_move("f8".to_string(), "c5".to_string()), None);
        assert_ne!(game.make_move("d5".to_string(), "c7".to_string()), None);
        assert_eq!(game.state, GameState::Check);
        assert_ne!(game.make_move("e8".to_string(), "e7".to_string()), None);
        assert_eq!(game.state, GameState::InProgress);
        assert_ne!(game.make_move("c7".to_string(), "a8".to_string()), None);
        assert_ne!(game.make_move("g8".to_string(), "f6".to_string()), None);
        assert_ne!(game.make_move("a1".to_string(), "a3".to_string()), None);
        assert_ne!(game.make_move("f6".to_string(), "e4".to_string()), None);
        assert_ne!(game.make_move("d2".to_string(), "d3".to_string()), None);
        assert_ne!(game.make_move("c5".to_string(), "f2".to_string()), None);
        assert_eq!(game.state, GameState::Check);
        assert_ne!(game.make_move("h3".to_string(), "f2".to_string()), None);
        assert_eq!(game.state, GameState::InProgress);
        assert_ne!(game.make_move("h4".to_string(), "f2".to_string()), None);
        assert_eq!(game.state, GameState::GameOver);

        println!("{:?}", game);
    }

    #[test]
    fn promotion() {
        let mut game = Game::new();

        game.board = [[None,None,None,None,None,None,None,Some(Piece::King(Colour::Black))],
        [Some(Piece::Pawn(Colour::Black)),Some(Piece::Pawn(Colour::Black)),Some(Piece::Pawn(Colour::Black)),Some(Piece::Pawn(Colour::Black)),Some(Piece::Pawn(Colour::Black)),Some(Piece::Pawn(Colour::Black)),Some(Piece::Pawn(Colour::Black)),Some(Piece::Pawn(Colour::Black))],
        [None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [Some(Piece::Pawn(Colour::White)),Some(Piece::Pawn(Colour::White)),Some(Piece::Pawn(Colour::White)),Some(Piece::Pawn(Colour::White)),Some(Piece::Pawn(Colour::White)),Some(Piece::Pawn(Colour::White)),Some(Piece::Pawn(Colour::White)),Some(Piece::Pawn(Colour::White))],
        [None,None,None,None,None,None,None,Some(Piece::King(Colour::White))]];

        game.make_move("a7".to_string(), "a8".to_string());
        assert_eq!(game.board[7][0], Some(Piece::Queen(Colour::White)));

        game.make_move("a2".to_string(), "a1".to_string());
        assert_eq!(game.board[0][0], Some(Piece::Queen(Colour::Black)));

        game.set_promotion("kn".to_string());
        game.make_move("b7".to_string(), "b8".to_string());
        assert_eq!(game.board[7][1], Some(Piece::Knight(Colour::White)));

        game.set_promotion("kn".to_string());
        game.make_move("b2".to_string(), "b1".to_string());
        assert_eq!(game.board[0][1], Some(Piece::Knight(Colour::Black)));

        game.set_promotion("r".to_string());
        game.make_move("c7".to_string(), "c8".to_string());
        assert_eq!(game.board[7][2], Some(Piece::Rook(Colour::White)));

        game.set_promotion("r".to_string());
        game.make_move("c2".to_string(), "c1".to_string());
        assert_eq!(game.board[0][2], Some(Piece::Rook(Colour::Black)));

        game.set_promotion("b".to_string());
        game.make_move("d7".to_string(), "d8".to_string());
        assert_eq!(game.board[7][3], Some(Piece::Bishop(Colour::White)));

        game.set_promotion("b".to_string());
        game.make_move("d2".to_string(), "d1".to_string());
        assert_eq!(game.board[0][3], Some(Piece::Bishop(Colour::Black)));

        println!("{:?}", game);
    }
}
