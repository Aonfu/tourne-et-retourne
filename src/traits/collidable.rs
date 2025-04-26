use std::collections::HashMap;
use macroquad::prelude::*;

pub trait Collidable {

    fn check_collision_x(&mut self, map: &HashMap<(i32, i32),(i32, i32)>);

    fn check_collision_y(&mut self, map: &HashMap<(i32, i32),(i32, i32)>);

    fn apply_physics(&mut self, map: &HashMap<(i32, i32),(i32, i32)>, delta : f32);
}