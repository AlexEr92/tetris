use std::time::Duration;

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
    pub game_over: bool,
    pub lock_delay_remaining: Option<Duration>,
}

impl Tetris {
    pub fn new() -> Tetris {
        let mut game_map = Vec::new();
        for _ in 0..16 {
            game_map.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
        let mut tetris = Tetris {
            game_map,
            current_level: 1,
            score: 0,
            cleared_lines: 0,
            current_tetromino: None,
            current_rand_number: 7,
            game_over: false,
            lock_delay_remaining: None,
        };
        let first_tetromino = tetris.create_new_tetromino();
        tetris.current_tetromino = Some(first_tetromino);
        tetris
    }

    fn check_lines(&mut self) -> u32 {
        let before = self.game_map.len();
        self.game_map.retain(|row| row.iter().any(|&cell| cell == 0));
        let cleared = (before - self.game_map.len()) as u32;

        while self.game_map.len() < 16 {
            self.game_map.insert(0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
        cleared
    }

    fn update_score_and_level(&mut self, lines_cleared: u32) {
        let points = match lines_cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        } * self.current_level;

        self.score += points;
        self.cleared_lines += lines_cleared;
        self.current_level = (self.cleared_lines / 10) + 1;
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn current_level(&self) -> u32 {
        self.current_level
    }

    pub fn cleared_lines(&self) -> u32 {
        self.cleared_lines
    }

    pub fn game_map(&self) -> &Vec<Vec<u8>> {
        &self.game_map
    }

    pub fn current_tetromino(&self) -> &Option<Tetromino> {
        &self.current_tetromino
    }

    fn is_valid_position(&self, tetromino: &Tetromino) -> bool {
        let state = &tetromino.states[tetromino.current_state as usize];
        for (dy, row) in state.iter().enumerate() {
            for (dx, &cell) in row.iter().enumerate() {
                if cell == 0 {
                    continue;
                }
                let x = tetromino.x + dx as isize;
                let y = tetromino.y + dy;
                if x < 0 || x >= 10 || y >= 16 {
                    return false;
                }
                if y < 16 && self.game_map[y][x as usize] != 0 {
                    return false;
                }
            }
        }
        true
    }

    fn is_on_ground(&self) -> bool {
        if let Some(ref tetromino) = self.current_tetromino {
            let state = &tetromino.states[tetromino.current_state as usize];
            for (dy, row) in state.iter().enumerate() {
                for (dx, &cell) in row.iter().enumerate() {
                    if cell == 0 {
                        continue;
                    }
                    let x = tetromino.x + dx as isize;
                    let y = tetromino.y + dy + 1;
                    if x < 0 || x >= 10 || y >= 16 {
                        return true;
                    }
                    if y < 16 && self.game_map[y][x as usize] != 0 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn reset_lock_delay_if_airborne(&mut self) {
        if !self.is_on_ground() {
            self.lock_delay_remaining = None;
        }
    }

    pub fn move_left(&mut self) {
        if let Some(mut tetromino) = self.current_tetromino.take() {
            tetromino.x -= 1;
            if !self.is_valid_position(&tetromino) {
                tetromino.x += 1;
            }
            self.current_tetromino = Some(tetromino);
            self.reset_lock_delay_if_airborne();
        }
    }

    pub fn move_right(&mut self) {
        if let Some(mut tetromino) = self.current_tetromino.take() {
            tetromino.x += 1;
            if !self.is_valid_position(&tetromino) {
                tetromino.x -= 1;
            }
            self.current_tetromino = Some(tetromino);
            self.reset_lock_delay_if_airborne();
        }
    }

    pub fn rotate(&mut self) {
        if let Some(mut tetromino) = self.current_tetromino.take() {
            let original_state = tetromino.current_state;
            let num_states = tetromino.states.len() as u8;
            tetromino.current_state = (tetromino.current_state + 1) % num_states;

            if !self.is_valid_position(&tetromino) {
                // Wall kick: try shifting left
                tetromino.x -= 1;
                if !self.is_valid_position(&tetromino) {
                    // Try shifting right (from original position)
                    tetromino.x += 2;
                    if !self.is_valid_position(&tetromino) {
                        // Revert everything
                        tetromino.x -= 1;
                        tetromino.current_state = original_state;
                    }
                }
            }
            self.current_tetromino = Some(tetromino);
            self.reset_lock_delay_if_airborne();
        }
    }

    /// Returns true if the tetromino could not move down (locked in place).
    pub fn move_down(&mut self) -> bool {
        if let Some(mut tetromino) = self.current_tetromino.take() {
            tetromino.y += 1;
            if !self.is_valid_position(&tetromino) {
                tetromino.y -= 1;
                self.current_tetromino = Some(tetromino);
                self.lock_delay_remaining = Some(Duration::from_millis(500));
                return true;
            }
            self.current_tetromino = Some(tetromino);
        }
        false
    }

    pub fn soft_drop(&mut self) -> bool {
        self.move_down()
    }

    pub fn drop_interval(&self) -> Duration {
        let seconds = 0.8 - (self.current_level as f64 - 1.0) * 0.05;
        let seconds = seconds.max(0.05);
        Duration::from_secs_f64(seconds)
    }

    fn lock_tetromino(&mut self) {
        if let Some(tetromino) = self.current_tetromino.take() {
            let state = &tetromino.states[tetromino.current_state as usize];
            for (dy, row) in state.iter().enumerate() {
                for (dx, &cell) in row.iter().enumerate() {
                    if cell != 0 {
                        let x = (tetromino.x + dx as isize) as usize;
                        let y = tetromino.y + dy;
                        if y < 16 {
                            self.game_map[y][x] = cell;
                        }
                    }
                }
            }
            let cleared = self.check_lines();
            self.update_score_and_level(cleared);
        }
    }

    pub fn spawn_tetromino(&mut self) {
        let tetromino = self.create_new_tetromino();
        if !self.is_valid_position(&tetromino) {
            self.game_over = true;
        }
        self.current_tetromino = Some(tetromino);
        self.lock_delay_remaining = None;
    }

    pub fn tick_lock_delay(&mut self, delta: Duration) -> bool {
        if let Some(ref mut remaining) = self.lock_delay_remaining {
            if *remaining > delta {
                *remaining -= delta;
                false
            } else {
                self.lock_delay_remaining = None;
                true
            }
        } else {
            false
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
