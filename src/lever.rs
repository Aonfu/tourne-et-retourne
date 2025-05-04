use crate::constants::TILE_SIZE;
use crate::traits::entity::{distance, Entity};
use crate::game::GameContext;
use macroquad::prelude::*;

pub struct Lever {
    hitbox : Rect,
    actived : bool,
}

impl Lever {
    pub fn new(x: f32, y: f32) -> Lever {
        Lever {
            hitbox : Rect::new(x, y,8., 8.),
            actived : false,
        }
    }
}

impl Entity for Lever {
    fn draw(&self) {
        draw_rectangle(self.hitbox.x, self.hitbox.y, self.hitbox.w, self.hitbox.h, ORANGE);
    }

    fn get_hitbox(&self) -> Rect {
        self.hitbox
    }

    fn update(&mut self, game_context : &mut GameContext){
        if distance(self.hitbox, game_context.player_hitbox) < 2.*TILE_SIZE as f32 {
            draw_text("Appuyer sur R", self.hitbox.x - 50., self.hitbox.y - 40., 24., ORANGE);
            if is_key_pressed(KeyCode::R) && !self.actived {
                game_context.map.remove(&(18*TILE_SIZE,9*TILE_SIZE));
                game_context.map.remove(&(18*TILE_SIZE,10*TILE_SIZE));
                self.actived = true;
            } else if  is_key_pressed(KeyCode::R) && self.actived{
                game_context.map.insert((18*TILE_SIZE,9*TILE_SIZE), (16,16));
                game_context.map.insert((18*TILE_SIZE,10*TILE_SIZE), (16,16));
                self.actived = false;
            }
        }
    }
}