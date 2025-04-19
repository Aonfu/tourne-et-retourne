mod player;
mod constants;
mod traits;
mod mobs;
mod ldtk;

use std::collections::HashMap;
use macroquad::prelude::*;
use mobs::Slime;
use player::Player;
use traits::entity::Entity;
use ldtk::*;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Refind Maho".to_string(),
        window_width: 640,
        window_height: 320,
        fullscreen: false,
        ..Default::default()
    };
    // to have a maximum of fps
    conf.platform.swap_interval = Some(0); 
    conf
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

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera2D {
        target: vec2(100.0, 100.0),
        zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()), 
        ..Default::default()
    };
    let mut maho_shojo = Player::new();

    let mut limule = Slime::new();

    let file = load_string("assets/test.ldtk").await.unwrap();
    let project: LDtkProject = serde_json::from_str(&file).unwrap();
    let level = &project.get_levels()[0];
    let layers = level.get_layer_instances().unwrap();
    let map: HashMap<(i32,i32), (i32,i32)> = map_from_tiles(layers.iter() 
    .find(|layer| layer.get_identifier() == "Base")
    .unwrap().get_tiles());
    
    loop {
        clear_background(SKYBLUE);
        draw_fps();

        camera.target = lerp_vec2(camera.target, vec2(maho_shojo.get_hitbox().x,maho_shojo.get_hitbox().y), 0.05);
        set_camera(&camera);
        limule.update(&map, &maho_shojo);
        maho_shojo.update(&map);
        
        map.iter().for_each(|tile| draw_rectangle(tile.0.0 as f32, tile.0.1 as f32, 16., 16., DARKGREEN));
        next_frame().await;
    }
}