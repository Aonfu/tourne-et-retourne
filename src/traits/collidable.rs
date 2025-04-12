use std::collections::HashSet;
use macroquad::prelude::*;

pub trait Collidable {

    fn check_collision_x(&mut self, map: &HashSet<(i32, i32)>);

    fn check_collision_y(&mut self, map: &HashSet<(i32, i32)>);

    fn apply_physics(&mut self, map: &HashSet<(i32, i32)>);
}