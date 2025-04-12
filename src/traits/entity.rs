use macroquad::prelude::*;

pub trait Entity {

    fn draw(&self);

    fn get_hitbox(&self) -> Rect;

}