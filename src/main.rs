mod player;
mod constants;
mod traits;

use std::collections::HashSet;
use macroquad::prelude::*;
use serde::Deserialize;
use player::Player;
use traits::entity::Entity;


fn window_conf() -> Conf {
    Conf {
        window_title: "Refind Maho".to_string(),
        window_width: 640,
        window_height: 320,
        fullscreen: false,
        ..Default::default()
    }
}

#[derive(Deserialize, Debug)]
struct LDtkProject {
    levels: Vec<LDtkLevel>,
}

#[derive(Deserialize, Debug)]
struct LDtkLevel {
    #[serde(rename = "identifier")]
    _identifier: String,
    #[serde(rename = "layerInstances")]
    layer_instances: Option<Vec<LDtkLayer>>,
}

#[derive(Deserialize, Debug)]
struct LDtkLayer {
    #[serde(rename = "__identifier")]
    identifier: String,

    #[serde(rename = "gridTiles")]
    tiles: Vec<LDtkTile>,
}

#[derive(Deserialize, Debug)]
struct LDtkTile {
    #[serde(rename = "px")]
    position: [i32; 2],
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

    let file = load_string("assets/test.ldtk").await.unwrap();
    let project: LDtkProject = serde_json::from_str(&file).unwrap();
    let level = &project.levels[0];
    let layers = level.layer_instances.as_ref().unwrap();
    let mut map: HashSet<(i32,i32)> = HashSet::new();
    layers.iter() 
    .find(|layer| layer.identifier == "Base")
    .unwrap().tiles.iter()
    .for_each(|tile| {map.insert((tile.position[0], tile.position[1]));} );
    
    loop {
        clear_background(SKYBLUE);

        camera.target = lerp_vec2(camera.target, vec2(maho_shojo.get_hitbox().x,maho_shojo.get_hitbox().y), 0.05);
        set_camera(&camera);
        maho_shojo.update(&map);
        
        layers.iter().find(|layer| layer.identifier == "Base").unwrap()
        .tiles.iter()
        .for_each(|tile| 
            draw_rectangle(tile.position[0] as f32, tile.position[1] as f32, 16., 16., DARKGREEN));
        next_frame().await;
        print!("{}\n",get_fps());
    }
}