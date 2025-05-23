use std::collections::HashMap;
use macroquad::prelude::*;
use crate::constants::*;

use crate::game::GameContext;
use crate::spell::{Element, Spell, SpellType};
use crate::traits::collidable::Collidable;
use crate::traits::entity::*;

pub struct Player {
    hitbox : Rect,
    vx : f32,
    vy : f32,
    on_floor: bool,
}

impl Player{
    pub fn new(x: f32, y: f32) -> Player {
        Player {
            hitbox : Rect::new(x, y,16., 28.),
            vx : 0.,
            vy : 0.,
            on_floor : true,
        }
    }

    pub fn update_inputs(&mut self, game_context : &mut GameContext){
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

        if is_key_pressed(KeyCode::E) {
            game_context.spells_to_spawn.push(Spell::new(self.hitbox.x, self.hitbox.y, SpellType::Ball, Element::Fire));
        }

    }

    pub async fn draw2(&self){
        let _draw_param = DrawTextureParams{
            dest_size: Some(vec2(self.hitbox.w,self.hitbox.h)),
            ..Default::default()
        };
        let tex = load_texture("assets/player.png").await.unwrap();
        let d = DrawTextureParams {
            source : Some(Rect::new(0., 0., TILE_SIZE as f32, TILE_SIZE as f32)),
            ..Default::default()
        };
        draw_texture(&tex, self.hitbox.x, self.hitbox.y, WHITE);
        //draw_rectangle(self.hitbox.x, self.hitbox.y, self.hitbox.w, self.hitbox.h, GREEN);
    }

}

impl Entity for Player {

    fn draw(&self){
        let _draw_param = DrawTextureParams{
            dest_size: Some(vec2(self.hitbox.w,self.hitbox.h)),
            ..Default::default()
        };
        draw_rectangle(self.hitbox.x, self.hitbox.y, self.hitbox.w, self.hitbox.h, DARKPURPLE);
    }

    fn get_hitbox(&self) -> Rect {
        self.hitbox
    }

    fn update(&mut self, game_context : &mut GameContext){
        self.update_inputs(game_context);
        self.apply_physics(game_context.map);
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

        self.hitbox.x += self.vx * FIXED_TIMESTEP;
        self.check_collision_x(map);

        self.hitbox.y += self.vy * FIXED_TIMESTEP;
        self.check_collision_y(map);

        if !self.on_floor{
            self.vy += GRAVITY * FIXED_TIMESTEP;
        }

        self.hitbox.x = self.hitbox.x.round();
        self.hitbox.y = self.hitbox.y.round();
    }
}
