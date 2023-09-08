#![allow(unused)]

mod sdl_view;
mod debug_view;
mod zetromino;
mod zetris;

use zetris::{Zetris, TetrisView};
use debug_view::DebugView;
use sdl_view::SdlView;

fn main() {
    let tetris = Zetris::new();
    let mut tetris_ui = SdlView::new(tetris);
    // let mut tetris_ui = DebugView::new(tetris);
    tetris_ui.run();
}
