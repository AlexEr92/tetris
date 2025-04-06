use super::tetromino::Tetromino;

pub struct Tetris {
    game_map: Vec<Vec<u8>>,
    current_level: u32,
    score: u32,
    cleared_lines: u32,
    current_tetromino: Option<Tetromino>,
}

impl Tetris {
    pub fn new() -> Tetris {
        let mut game_map = Vec::new();
        for _ in 0..16 {
            game_map.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
        Tetris {
            game_map: game_map,
            current_level: 1,
            score: 0,
            cleared_lines: 0,
            current_tetromino: None,
        }
    }
}
