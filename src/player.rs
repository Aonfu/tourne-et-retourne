use std::collections::HashMap;
use macroquad::prelude::*;
use crate::constants::*;

use crate::traits::collidable::Collidable;
use crate::traits::entity::*;

pub struct Player {
    hitbox : Rect,
    vx : f32,
    vy : f32,
    on_floor: bool,
}

impl Player{
    pub fn new() -> Player {
        Player {
            hitbox : Rect::new(9.*16., 13.*16.-24.,16., 16.),
            vx : 0.,
            vy : 0.,
            on_floor : true,
        }
    }

    fn update_inputs(&mut self){
        self.vx = 0.;

        if is_key_down(KeyCode::D){
            self.vx += SPEED;
        }

        if is_key_down(KeyCode::A){
            self.vx += -SPEED;
        }

        if is_key_down(KeyCode::Space) && self.on_floor {
            self.vy = JUMP_FORCE;
            self.on_floor = false;
        }

        if is_key_pressed(KeyCode::Enter){
            self.hitbox.x = 9.*16.;
            self.hitbox.y = 13.*16.-24.;
        }

    }

    pub fn update(& mut self, map : &HashMap<(i32, i32),(i32, i32)>){
        self.update_inputs();
        self.apply_physics(map);
        self.draw();
    }

}

impl Entity for Player {

    fn draw(&self){
        let _draw_param = DrawTextureParams{
            dest_size: Some(vec2(self.hitbox.w,self.hitbox.h)),
            ..Default::default()
        };
        draw_rectangle(self.hitbox.x, self.hitbox.y, self.hitbox.w, self.hitbox.h, PURPLE);
    }

    fn get_hitbox(&self) -> Rect {
        self.hitbox
    }
}

impl Collidable for Player {
    
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

        self.hitbox.x += self.vx * get_frame_time();
        self.check_collision_x(map);

        self.hitbox.y += self.vy * get_frame_time();
        self.check_collision_y(map);

        if !self.on_floor{
            self.vy += GRAVITY * get_frame_time();
        }
    }
}