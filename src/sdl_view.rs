use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::sys::SDL_Delay;
use sdl2::video::{Window, WindowContext};

use crate::zetris::{TetrisView, Zetris};
use crate::zetromino::{PLAYFIELD_HCELLS, PLAYFIELD_VCELLS};

const SCREEN_WIDTH: usize = 300;
const SCREEN_HEIGHT: usize = 550;

const BLOCK_WIDTH_PX: u32 = 25;
const BLOCK_HEIGHT_PX: u32 = 25;

const BLOCKS_TEXTURE_PATH: &str = "assets/blocks.png";

pub struct SdlView {
    tetris: Zetris,
}

impl SdlView {
    pub fn new(tetris: Zetris) -> Self {
        Self { tetris }
    }

    fn draw_tetrion(&mut self, canvas: &mut Canvas<Window>, blocks: &Texture<'_>) {
        for i in 0..PLAYFIELD_VCELLS + 2 {
            for j in 0..PLAYFIELD_HCELLS + 2 {
                if (i == 0 || i == PLAYFIELD_VCELLS + 1) || (j == 0 || j == PLAYFIELD_HCELLS + 1) {
                    canvas.copy(
                        &blocks,
                        Rect::new(0, 0, BLOCK_WIDTH_PX, BLOCK_HEIGHT_PX),
                        Rect::new(
                            j as i32 * BLOCK_WIDTH_PX as i32,
                            i as i32 * BLOCK_HEIGHT_PX as i32,
                            BLOCK_WIDTH_PX as u32,
                            BLOCK_HEIGHT_PX as u32,
                        ),
                    );
                }
            }
        }
    }

    fn draw_playfield(&mut self, canvas: &mut Canvas<Window>, blocks: &Texture<'_>) {
        for i in 0..PLAYFIELD_VCELLS {
            for j in 0..PLAYFIELD_HCELLS {
                let cell = self.tetris.playfield[i][j];
                if cell.occupied {
                    let tx = (cell.color as i32) * BLOCK_WIDTH_PX as i32;
                    canvas.copy(
                        &blocks,
                        Rect::new(tx, 0, BLOCK_WIDTH_PX, BLOCK_HEIGHT_PX),
                        Rect::new(
                            (j as i32 + 1) * BLOCK_WIDTH_PX as i32,
                            (i as i32 + 1) * BLOCK_HEIGHT_PX as i32,
                            BLOCK_WIDTH_PX as u32,
                            BLOCK_HEIGHT_PX as u32,
                        ),
                    );
                }
            }
        }
    }

    fn draw_zetromino(&mut self, canvas: &mut Canvas<Window>, blocks: &Texture<'_>) {
        let tx = self.tetris.current.color as i32 * BLOCK_WIDTH_PX as i32;
        for mino in self.tetris.current.minos.iter() {
            if mino.y < 0 {
                continue;
            }
            canvas.copy(
                &blocks,
                Rect::new(tx, 0, BLOCK_WIDTH_PX, BLOCK_HEIGHT_PX),
                Rect::new(
                    (mino.x as i32 + 1) * BLOCK_WIDTH_PX as i32,
                    (mino.y as i32 + 1) * BLOCK_HEIGHT_PX as i32,
                    BLOCK_WIDTH_PX as u32,
                    BLOCK_HEIGHT_PX as u32,
                ),
            );
        }
    }
}

impl TetrisView for SdlView {
    fn run(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();
        let window = video
            .window("Zetris 0.1", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().accelerated().build().unwrap();
        let _ = sdl2::image::init(InitFlag::PNG);
        let texture_creator = canvas.texture_creator();
        let blocks = texture_creator.load_texture(BLOCKS_TEXTURE_PATH).unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut done = false;
        let mut paused = false;
        while !done {
            if !paused {
                self.tetris.update();
            }
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => done = true,
                    Event::KeyDown {keycode: Some(k), ..} => {
                        match k {
                            Keycode::Left => self.tetris.move_left(),
                            Keycode::Right => self.tetris.move_right(),
                            Keycode::Return => println!("RET"),
                            Keycode::Space => paused = !paused,
                            Keycode::Escape => done = true,
                            _ => (),
                        }
                    },
                    _ => (),
                }
            }

            canvas.clear();

            self.draw_tetrion(&mut canvas, &blocks);
            self.draw_playfield(&mut canvas, &blocks);
            self.draw_zetromino(&mut canvas, &blocks);

            canvas.present();

            sleep(Duration::from_millis(100));
        }
    }
}
