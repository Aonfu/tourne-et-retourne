use std::collections::HashMap;

use macroquad::math::Rect;
use crate::{
    constants::{FIXED_TIMESTEP, GRAVITY, JUMP_FORCE, SPEED, TILE_SIZE}, game::GameContext, player::Player, traits::{collidable::Collidable, entity::{distance, Entity}}
};
use macroquad::prelude::*;

pub struct Slime {
    hitbox : Rect,
    vx : f32,
    vy : f32,
    on_floor: bool,
}

impl Slime {
    pub fn new(x: f32, y: f32) -> Slime {
        Slime {
            hitbox : Rect::new(x, y,20., 20.),
            vx : 0.,
            vy : 0.,
            on_floor : true,
        }
    }

    pub fn behavior(&mut self, map: &HashMap<(i32, i32),(i32, i32)> ,player_hitbox : Rect) {
        // the slime moves by jump on player if player is in it range

        if distance(self.get_hitbox(), player_hitbox) <= 100. && self.on_floor{ 
            self.vy = JUMP_FORCE * 0.5;
            self.on_floor = false;
        }

        self.vx = if distance(self.get_hitbox(), player_hitbox) <= 100. && !self.on_floor {
            let direction = if self.get_hitbox().x > player_hitbox.x {-1.} else {1.};
            direction * SPEED * 0.5

        } else {0.};

    }
}

impl Entity for Slime {
    fn draw(&self) {
        draw_rectangle(self.hitbox.x, self.hitbox.y, self.hitbox.w, self.hitbox.h, DARKBLUE);
        //range temp
        draw_circle_lines(self.hitbox.x + self.hitbox.w/2., self.hitbox.y + self.hitbox.h/2., 100.,1., DARKBLUE);
    }

    fn get_hitbox(&self) -> Rect {
        Rect { x: self.hitbox.x, y: self.hitbox.y, w: self.hitbox.w, h: self.hitbox.h }
    }

    fn update(&mut self, game_context : &mut GameContext) {
        self.behavior(game_context.map, game_context.player_hitbox);
        self.apply_physics(game_context.map);
        self.draw();
    }
}

impl Collidable for Slime {

    fn check_collision_x(&mut self, map : &HashMap<(i32, i32),(i32, i32)>){

        let left = self.hitbox.x as i32;
        let right = (self.hitbox.x + self.hitbox.w) as i32;
        let top = self.hitbox.y as i32;
        let bottom = (self.hitbox.y + self.hitbox.h) as i32;

        let x = if self.vx > 0. {
            right
        } else {
            left
        };

        for y in (top..=bottom).filter(|y| if bottom%TILE_SIZE==0 {*y!=bottom} else {true}).step_by(TILE_SIZE as usize) {
            let tile_x = x / TILE_SIZE * TILE_SIZE;
            let tile_y = y / TILE_SIZE * TILE_SIZE;

            if map.contains_key(&(tile_x,tile_y)) {
                if x == left {
                    self.hitbox.x = (tile_x + TILE_SIZE) as f32;
                    self.vx = 0.; // line useless for now but useful for understanding
                } else {
                    self.hitbox.x = tile_x as f32 - self.hitbox.w;
                    self.vx = 0.; // line useless for now but useful for understanding
                }
            }
        }
    }

    fn check_collision_y(&mut self, map : &HashMap<(i32, i32),(i32, i32)>){

        self.on_floor = false; // it will true if the floor is detected

        let left = self.hitbox.x as i32;
        let right = (self.hitbox.x + self.hitbox.w) as i32;
        let top = self.hitbox.y as i32;
        let bottom = (self.hitbox.y + self.hitbox.h) as i32;

        let y = if self.vy < 0. {
            top
        } else {
            bottom
        };

        for x in (left..=right).filter(|x| if right%TILE_SIZE==0 {*x!=right} else {true}).step_by(TILE_SIZE as usize) {
            let tile_x: i32 = x / TILE_SIZE * TILE_SIZE;
            let tile_y: i32 = y / TILE_SIZE * TILE_SIZE;

            if map.contains_key(&(tile_x,tile_y)) {
                if y == bottom {
                    self.hitbox.y = tile_y as f32 - self.hitbox.h;
                    self.vy = 0.;
                    self.on_floor = true;
                } else {
                    self.hitbox.y = (tile_y + TILE_SIZE) as f32;
                    self.vy = 0.;
                }
            }
        }
    }

    fn apply_physics(&mut self, map:&HashMap<(i32, i32),(i32, i32)>){

        self.hitbox.x += self.vx * FIXED_TIMESTEP;
        self.check_collision_x(map);

        self.hitbox.y += self.vy * FIXED_TIMESTEP;
        self.check_collision_y(map);

        if !self.on_floor{
            self.vy += GRAVITY * FIXED_TIMESTEP;
        }
    }
}
