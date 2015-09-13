mod textures;
mod block_grid;
mod values;
mod renderer;

extern crate piston_window;
extern crate uuid;
extern crate graphics;
extern crate find_folder;
extern crate gfx;
extern crate gfx_texture;

use piston_window::*;

use textures::Textures;
use block_grid::{BlockGrid};
use values::{Position,Block,Color,Piece,Direction,PositionedBlock};
use renderer::{BlockRenderer,Renderer};

struct Game {
    renderer: Box<BlockRenderer>,

    // State
    grid: BlockGrid,

    // Time since last block step.
    step_accumulator: f64,

    // Seconds between block steps.
    speed: f64,

    // Currently falling fiece
    current_piece: Option<Piece>,
}

impl Game {
    fn new(renderer: Box<BlockRenderer>, dimensions: (usize, usize)) -> Self {
        let (w, h) = dimensions;

        Game {
            renderer: renderer,
            step_accumulator: 0.0,
            speed: 0.3,
            current_piece: None,
            grid: BlockGrid::new(w, h),
        }
    }

    // Attempt to modify the current piece if present. modifier will be called
    // with the current piece and should return a desired modification. If it
    // is valid (no blocks are in the way), the current piece is replaced with
    // it and true is returned. Otherwise, returns false.
    fn move_piece<F>(&mut self, modifier: F) -> bool
        where F : Fn(Piece) -> Piece {

        let ref mut grid = self.grid;

        if let Some(piece) = self.current_piece {
            let new_piece = modifier(piece);

            let occupied = new_piece.positions().iter().any(|p| {
                !grid.empty(*p)
            });

            if !occupied {
                for pb in new_piece.blocks().iter() {
                    self.renderer.move_block(pb.block, pb.position);
                }
                self.current_piece = Some(new_piece);
                return true;
            }
        }
        false
    }

    fn update(&mut self, e: &PistonWindow) {
        if let Some(button) = e.release_args() {
            use piston_window::Button::Keyboard;
            use piston_window::Key;

            match button {
                Keyboard(Key::Space) => {
                    self.speed = 0.3
                },
                _ => {},
            }
        }
        if let Some(button) = e.press_args() {
            use piston_window::Button::Keyboard;
            use piston_window::Key;

            // TODO: Handle key repeat on our own timer.
            match button {
                Keyboard(Key::Up) => {
                    self.move_piece(|current| { current.anti_clockwise() });
                },
                Keyboard(Key::Down) => {
                    self.move_piece(|current| { current.clockwise() });
                },
                Keyboard(Key::Left) => {
                    self.move_piece(|current| { current.offset(Direction::Left) });
                },
                Keyboard(Key::Right) => {
                    self.move_piece(|current| { current.offset(Direction::Right) });
                },
                Keyboard(Key::Space) => {
                    self.speed = 0.05;
                }
                _ => {},
            }
        }

        e.update(|args| {
            self.step_accumulator += args.dt;

            if self.step_accumulator > self.speed {
                self.step_accumulator -= self.speed;

                if !self.move_piece(|current| current.offset(Direction::Down) ) {
                    if let Some(piece) = self.current_piece {
                        for pb in piece.blocks().iter() {
                            let resting = self.grid.bottom(*pb);
                            self.grid.set(resting.position, Some(pb.block));

                            self.renderer.drop_block(pb.block, resting.position);
                        }
                        self.current_piece = None;
                    }
                }

                if self.current_piece.is_none() {
                    let pos = Position { x: 2, y: GRID_HEIGHT as i8 - 1 };
                    let block = Block::active(Color::rand());
                    let pb1 = PositionedBlock::new(block, pos);
                    self.renderer.add_block(pb1.block, pb1.position);

                    let pos = Position { x: 3, y: GRID_HEIGHT as i8 - 1 };
                    let block = Block::active(Color::rand());
                    let pb2 = PositionedBlock::new(block, pos);
                    self.renderer.add_block(pb2.block, pb2.position);

                    self.current_piece = Some(Piece {
                        blocks: [pb1.block, pb2.block],
                        position: pb1.position,
                        direction: Direction::Right,
                    })
                }
            }
        });

        self.renderer.event(&e);
    }
}

const GRID_HEIGHT: u8 = 13;
const GRID_WIDTH: u8 = 6;
const CELL_WIDTH: f64 = 32.0;
const CELL_HEIGHT: f64 = 32.0;

fn main() {
    let width = (GRID_WIDTH as u32 * CELL_WIDTH as u32) as u32;
    let height = (GRID_HEIGHT as u32 * CELL_HEIGHT as u32) as u32;
    let window: PistonWindow =
        WindowSettings::new("Puzzle Fighter Turbo II", (width, height))
        .exit_on_esc(true)
        .build()
        .unwrap();

    let textures = Textures::new(&window);

    let renderer = Renderer::new(textures);

    let mut game = Game::new(Box::new(renderer), (GRID_WIDTH as usize, GRID_HEIGHT as usize));

    for e in window {
        game.update(&e);
    }
}
