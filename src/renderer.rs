extern crate uuid;
extern crate piston_window;
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_texture;

extern crate sprite;
extern crate ai_behavior;

use self::uuid::Uuid;
use self::piston_window::*;
use self::sprite::{Sprite,Scene,MoveTo};

use self::ai_behavior::{
    Action,
};

use std::collections::HashMap;

use values::*;
use textures::Textures;

// TODO: De-dup with main.rs
const CELL_WIDTH: f64 = 32.0;
const CELL_HEIGHT: f64 = 32.0;
const GRID_HEIGHT: usize = 13;
const GRID_WIDTH: usize = 6;

pub struct Renderer<I: ImageSize, R> where R: gfx::Resources {
    scene: Scene<I>,
    textures: Textures<R>,
    sprites: HashMap<Block, Uuid>,
}

impl<I: ImageSize, R> Renderer<I, R> where R: gfx::Resources {
    pub fn new(textures: Textures<R>) -> Self {
        Renderer {
            textures: textures,
            sprites: HashMap::new(),
            scene: Scene::new(),
        }
    }
}

pub trait BlockRenderer {
    fn event(&mut self, event: &PistonWindow) {}
    fn add_block(&mut self, block: Block, position: Position) {}
    fn move_block(&mut self, block: Block, position: Position) {}
    fn drop_block(&mut self, block: Block, position: Position) {}
}

impl BlockRenderer for Renderer<Texture<gfx_device_gl::Resources>, gfx_device_gl::Resources> {
    fn add_block(&mut self, block: Block, position: Position) {
        let texture = self.textures.get(block.color.to_texture_name());
        let mut sprite = Sprite::from_texture(texture);
        sprite.set_anchor(0.0, 0.0);

        let id = self.scene.add_child(sprite);
        self.sprites.insert(block, id);
    }

    fn move_block(&mut self, block: Block, position: Position) {
        let sprite = self.sprites.get(&block).unwrap();

        self.scene.stop_all(*sprite);
        self.scene.run(*sprite,
            &Action(
                MoveTo(0.01,
                    position.x as f64 * CELL_WIDTH,
                    (GRID_HEIGHT - position.y - 1) as f64 * CELL_HEIGHT
                )
            )
        );
    }

    fn drop_block(&mut self, block: Block, position: Position) {
        let sprite = self.sprites.get(&block).unwrap();

        self.scene.stop_all(*sprite);
        self.scene.run(*sprite,
            &Action(
                MoveTo(0.1,
                    position.x as f64 * CELL_WIDTH,
                    (GRID_HEIGHT - position.y - 1) as f64 * CELL_HEIGHT
                )
            )
        );
    }

    fn event(&mut self, event: &PistonWindow) {
        self.scene.event(event);
        event.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            self.scene.draw(c.transform, g);
        });
    }
}

