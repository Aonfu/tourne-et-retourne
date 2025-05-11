use crate::{constants::{FIXED_TIMESTEP, TILE_SIZE}, game::GameContext};
use macroquad::prelude::*;

use crate::{constants::SPEED, traits::entity::Entity};

pub enum Element {
    Fire,
    Water,
    Wind,
    Earth,
}

pub enum SpellType {
    Ball,
}

pub struct Spell {
    hitbox: Rect,
    velocity: Vec2,
    spell_type : SpellType,
    element : Element,
}

impl Spell {
    pub fn new(x : f32, y : f32, spell_type : SpellType, element : Element) -> Spell {
        Spell { 
            hitbox: Rect::new(x, y, 4., 4.),
            velocity: vec2(SPEED, 0.),
            spell_type: SpellType::Ball,
            element: Element::Fire,
        }
    }
}

impl Entity for Spell {
    fn update(&mut self, game_context: &mut GameContext) {
        self.hitbox.x += self.velocity.x * FIXED_TIMESTEP;
        self.hitbox.y += self.velocity.y * FIXED_TIMESTEP;

        let mut a = false;
        for tile in &mut *game_context.map {
            if self.hitbox.overlaps(&Rect { x: tile.0.0 as f32, y: tile.0.1 as f32, w: TILE_SIZE as f32, h: TILE_SIZE as f32}) && tile.1.0 == 16 && tile.0.1 == 16 {
                a = true;
            }
        }
        if a {
        game_context.map.retain(|_,v| *v !=(16,16));
        }
    }

    fn draw(&self) {
        draw_rectangle(self.hitbox.x, self.hitbox.y, self.hitbox.w, self.hitbox.h, RED);
    }

    fn get_hitbox(&self) -> Rect {
        self.hitbox
    }
}
