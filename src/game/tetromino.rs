type Piece = Vec<Vec<u8>>;
type States = Vec<Piece>;

pub struct Tetromino {
    pub states: States,    // list of possible states
    pub x: isize,          // x position
    pub y: usize,          // y position
    pub current_state: u8, // current state
}

pub trait TetrominoGenerator {
    fn new() -> Tetromino;
}

pub struct TetrominoI;

impl TetrominoGenerator for TetrominoI {
    fn new() -> Tetromino {
        Tetromino {
            states: vec![
                vec![
                    vec![1, 1, 1, 1],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetrominoL;

impl TetrominoGenerator for TetrominoL {
    fn new() -> Tetromino {
        Tetromino {
            states: vec![
                vec![
                    vec![2, 2, 2, 0],
                    vec![2, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![2, 2, 0, 0],
                    vec![0, 2, 0, 0],
                    vec![0, 2, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 0, 2, 0],
                    vec![2, 2, 2, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![2, 0, 0, 0],
                    vec![2, 0, 0, 0],
                    vec![2, 2, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetrominoJ;

impl TetrominoGenerator for TetrominoJ {
    fn new() -> Tetromino {
        Tetromino {
            states: vec![
                vec![
                    vec![3, 3, 3, 0],
                    vec![0, 0, 3, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 3, 0, 0],
                    vec![0, 3, 0, 0],
                    vec![3, 3, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![3, 0, 0, 0],
                    vec![3, 3, 3, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![3, 3, 0, 0],
                    vec![3, 0, 0, 0],
                    vec![3, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetrominoO;

impl TetrominoGenerator for TetrominoO {
    fn new() -> Tetromino {
        Tetromino {
            states: vec![vec![
                vec![4, 4, 0, 0],
                vec![4, 4, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]],
            x: 5,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetrominoS;

impl TetrominoGenerator for TetrominoS {
    fn new() -> Tetromino {
        Tetromino {
            states: vec![
                vec![
                    vec![0, 5, 5, 0],
                    vec![5, 5, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 5, 0, 0],
                    vec![0, 5, 5, 0],
                    vec![0, 0, 5, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetrominoZ;

impl TetrominoGenerator for TetrominoZ {
    fn new() -> Tetromino {
        Tetromino {
            states: vec![
                vec![
                    vec![6, 6, 0, 0],
                    vec![0, 6, 6, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 0, 6, 0],
                    vec![0, 6, 6, 0],
                    vec![0, 6, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetrominoT;

impl TetrominoGenerator for TetrominoT {
    fn new() -> Tetromino {
        Tetromino {
            states: vec![
                vec![
                    vec![7, 7, 7, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![7, 7, 0, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![7, 7, 7, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![0, 7, 7, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}
