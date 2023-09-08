use std::thread::sleep;

use crate::zetris::{Zetris, TetrisView, SLEEP_DURATION};

pub struct DebugView {
    tetris: Zetris,
}

impl DebugView {
    pub fn new(tetris: Zetris) -> Self {
        Self { tetris }
    }

    fn print_debug_info(&self) {
        println!(
            "[TICK] zetromino max y = {}",
            self.tetris.current.max_y()
        );
    }
}

impl TetrisView for DebugView {
    fn run(&mut self) {
        let mut done = false;
        while !done {
            sleep(SLEEP_DURATION);
            self.tetris.update();
            self.print_debug_info();
        }
    }
}
