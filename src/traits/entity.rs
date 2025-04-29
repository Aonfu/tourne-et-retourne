use macroquad::prelude::*;

use crate::{game::GameContext, mobs::Slime};

pub trait Entity {

    fn draw(&self);

    fn get_hitbox(&self) -> Rect;

    fn update(&mut self, game_context : &GameContext);

}

pub fn distance_entity<T1 : Entity, T2 : Entity>(entity1 : &T1, entity2 : &T2) -> f32{
    // distance between two entities

    let dx = entity1.get_hitbox().x + entity1.get_hitbox().w/2. - (entity2.get_hitbox().x + entity2.get_hitbox().w/2.);
    let dy = entity1.get_hitbox().y + entity1.get_hitbox().h/2. - (entity2.get_hitbox().y + entity2.get_hitbox().h/2.);

    ((dx * dx) + (dy * dy)).sqrt()
}

pub fn distance(rect1 : Rect, rect2 : Rect) -> f32 {
    let dx = rect1.center().x - rect2.center().x;
    let dy = rect1.center().y - rect2.center().y;
    ((dx * dx) + (dy * dy)).sqrt()
}