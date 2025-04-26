use std::{collections::HashMap, thread::sleep, time::Duration};
use crate::constants::FIXED_TIMESTEP;
use macroquad::prelude::*;
use crate::mobs::Slime;
use crate::player::Player;
use crate::traits::{collidable::Collidable, entity::Entity};
use crate::ldtk::*;


pub struct Game {
    accumulator : f32,
    camera : Camera2D,
    player : Player,
    map : HashMap<(i32,i32), (i32,i32)>,
    limule : Slime

}

impl Game {
    pub async fn new() -> Game {
        let file = load_string("assets/test.ldtk").await.unwrap();
        let project: LDtkProject = serde_json::from_str(&file).unwrap();
        let level = &project.get_levels()[0];
        let layers = level.get_layer_instances().unwrap();
        Game {
            accumulator : 0.0,
            camera : Camera2D {
                target: vec2(100.0, 100.0),
                zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()), 
                ..Default::default()
            },
            player : Player::new(),
            map : map_from_tiles(layers.iter() 
            .find(|layer| layer.get_identifier() == "Base")
            .unwrap().get_tiles()),
            limule : Slime::new(),
        }
    }

    pub async fn update(&mut self){
        clear_background(SKYBLUE);
        draw_fps();
        let deltatime = get_frame_time();
        self.accumulator += deltatime;

        self.camera.target = lerp_vec2(self.camera.target, vec2(self.player.get_hitbox().x,self.player.get_hitbox().y), 0.05);
        set_camera(&self.camera);
        // sleep(Duration::from_millis((1000.) as u64));
        while self.accumulator >= FIXED_TIMESTEP {
            self.camera.target = lerp_vec2(self.camera.target, vec2(self.player.get_hitbox().x,self.player.get_hitbox().y), 0.05);
            set_camera(&self.camera);
            self.player.update_inputs();
            self.player.apply_physics(&self.map, FIXED_TIMESTEP);

            self.limule.apply_physics(&self.map, FIXED_TIMESTEP);
            self.limule.behavior(&self.map, &self.player);

            self.accumulator -= FIXED_TIMESTEP;
        }

        self.player.draw();
        self.limule.draw();
        // maho_shojo.update(&self.map);
        self.map.iter().for_each(|tile| draw_rectangle(tile.0.0 as f32, tile.0.1 as f32, 16., 16., DARKGREEN));
        next_frame().await;
    }
}


fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn lerp_vec2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    vec2(
        lerp(a.x, b.x, t),
        lerp(a.y, b.y, t),
    )
}