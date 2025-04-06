use super::tetromino::{
    Tetromino, TetrominoGenerator, TetrominoI, TetrominoJ, TetrominoL, TetrominoO, TetrominoS,
    TetrominoT, TetrominoZ,
};

pub struct Tetris {
    game_map: Vec<Vec<u8>>,
    current_level: u32,
    score: u32,
    cleared_lines: u32,
    current_tetromino: Option<Tetromino>,
    current_rand_number: u8,
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
            current_rand_number: 7,
        }
    }

    fn check_lines(&mut self) {
        let mut y = 0;

        while y < self.game_map.len() {
            let mut complete = true;

            for x in &self.game_map[y] {
                if *x == 0 {
                    complete = false;
                    break;
                }
            }

            if complete == true {
                self.game_map.remove(y);
                y -= 1;
            }

            y += 1;
        }

        while self.game_map.len() < 16 {
            self.game_map.insert(0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
    }

    fn create_new_tetromino(&mut self) -> Tetromino {
        let mut rand_number = rand::random::<u8>() % 7;
        // if the generated tetromino is the same as the previous one,
        // we generate another one
        if rand_number == self.current_rand_number {
            rand_number = rand::random::<u8>() % 7;
        }
        self.current_rand_number = rand_number;

        match self.current_rand_number {
            0 => TetrominoI::new(),
            1 => TetrominoL::new(),
            2 => TetrominoJ::new(),
            3 => TetrominoO::new(),
            4 => TetrominoS::new(),
            5 => TetrominoZ::new(),
            6 => TetrominoT::new(),
            _ => unreachable!(),
        }
    }
}
