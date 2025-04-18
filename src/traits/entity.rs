use macroquad::prelude::*;

pub trait Entity {

    fn draw(&self);

    fn get_hitbox(&self) -> Rect;

}

pub fn distance<T1 : Entity, T2 : Entity>(entity1 : &T1, entity2 : &T2) -> f32{
    // distance between two entities

    let dx = entity1.get_hitbox().x + entity1.get_hitbox().w/2. - (entity2.get_hitbox().x + entity2.get_hitbox().w/2.);
    let dy = entity1.get_hitbox().y + entity1.get_hitbox().h/2. - (entity2.get_hitbox().y + entity2.get_hitbox().h/2.);

    ((dx * dx) + (dy * dy)).sqrt()
}