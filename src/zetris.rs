use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use crate::zetromino::{MinoColor, Zetromino, PLAYFIELD_HCELLS, PLAYFIELD_VCELLS};

pub const SLEEP_DURATION: Duration = Duration::from_millis(100);

const GAME_DELAY: Duration = Duration::from_millis(200);
const INPUT_DELAY: Duration = Duration::from_millis(100);

#[derive(Clone, Copy, Debug)]
pub struct PlayfieldCell {
    pub occupied: bool,
    pub color: MinoColor,
}

impl PlayfieldCell {
    fn new() -> Self {
        Self {
            occupied: false,
            color: MinoColor::default(),
        }
    }
}

pub struct Zetris {
    pub playfield: [[PlayfieldCell; PLAYFIELD_HCELLS]; PLAYFIELD_VCELLS],
    pub current: Zetromino,
    // score: usize,
    speed: f32,        // ticks per second
    t0: Instant,       // time control for regular game flow
    input_t0: Instant, // time control for player actions
    game_over: bool,
}

pub trait TetrisView {
    fn run(&mut self);
}

enum Direction {
    Left,
    Right,
    Down,
}

impl Zetris {
    pub fn new() -> Self {
        Zetris {
            playfield: [[PlayfieldCell::new(); PLAYFIELD_HCELLS]; PLAYFIELD_VCELLS],
            current: Zetromino::random_zetromino(),
            // score: 0,
            speed: 1.0,
            t0: Instant::now(),
            input_t0: Instant::now(),
            game_over: false,
        }
    }

    pub fn update(&mut self) {
        if !self.time_for_next_game_tick() {
            return;
        }
        self.t0 = Instant::now();
        if self.zetromino_can_move(Direction::Down) {
            self.current.down();
        } else {
            for mino in self.current.minos.iter() {
                self.playfield[mino.y as usize][mino.x as usize].occupied = true;
                self.playfield[mino.y as usize][mino.x as usize].color = self.current.color;
            }
            self.current = Zetromino::random_zetromino();
        }
    }

    fn time_for_next_game_tick(&self) -> bool {
        let elapsed = Instant::now() - self.t0;
        elapsed > GAME_DELAY
    }

    fn time_for_next_input_tick(&self) -> bool {
        let elapsed = Instant::now() - self.input_t0;
        elapsed > INPUT_DELAY
    }

    pub fn move_left(&mut self) {
        if self.time_for_next_input_tick() {
            self.input_t0 = Instant::now();
            if self.zetromino_can_move(Direction::Left) {
                self.current.left();
            }
        }
    }

    pub fn move_right(&mut self) {
        if self.time_for_next_input_tick() {
            self.input_t0 = Instant::now();
            if self.zetromino_can_move(Direction::Right) {
                self.current.right();
            }
        }
    }

    fn game_over(&self) -> bool {
        self.game_over
    }

    fn zetromino_can_move(&self, direction: Direction) -> bool {
        let mut dummy = self.current.clone();
        match direction {
            Direction::Left => dummy.left(),
            Direction::Right => dummy.right(),
            Direction::Down => dummy.down(),
        }
        !self.check_collisions(&dummy)
    }

    fn check_collisions(&self, zetromino: &Zetromino) -> bool {
        zetromino.minos.iter().filter(|m| m.y >= 0).any(|m| {
            m.x < 0
                || m.y >= PLAYFIELD_VCELLS as i8
                || m.x >= PLAYFIELD_HCELLS as i8
                || self.playfield[m.y as usize][m.x as usize].occupied
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::zetromino::Kind;

    use super::*;

    #[test]
    fn zetromino_in_vanish_area_should_not_collide() {
        let zetris = Zetris::new();
        let z = Zetromino::new(Kind::O);
        assert!(!zetris.check_collisions(&z));
    }

    #[test]
    fn zetromino_with_negative_x_should_collide() {
        let zetris = Zetris::new();
        let mut z = Zetromino::new(Kind::O);
        while z.min_y() < 0 {
            z.down();
        }
        while z.min_x() >= 0 {
            z.left();
        }
        assert!(zetris.check_collisions(&z));
    }

    #[test]
    fn zetromino_with_max_x_greater_than_playfield_width_should_collide() {
        let zetris = Zetris::new();
        let mut z = Zetromino::new(Kind::O);
        while z.min_y() < 0 {
            z.down();
        }
        while z.max_x() < PLAYFIELD_HCELLS as i8 {
            z.right();
        }
        assert!(zetris.check_collisions(&z));
    }

    #[test]
    fn zetromino_with_max_y_greater_than_playfield_height_should_collide() {
        let zetris = Zetris::new();
        let mut z = Zetromino::new(Kind::O);
        while z.max_y() < PLAYFIELD_VCELLS as i8 {
            z.down();
        }
        assert!(zetris.check_collisions(&z));
    }

    #[test]
    fn zetromino_should_collide_with_occupied_cells_in_playfield() {
        let mut zetris = Zetris::new();
        let mut cell = PlayfieldCell::new();
        cell.occupied = true;
        zetris.playfield[0] = [cell; PLAYFIELD_HCELLS];
        let mut z = Zetromino::new(Kind::O);
        while z.min_y() < 0 {
            z.down();
        }
        assert!(zetris.check_collisions(&z));
    }
}
