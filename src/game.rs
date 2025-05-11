use std::collections::HashMap;
use crate::constants::{FIXED_TIMESTEP, TILE_SIZE};
use crate::spell::{self, Spell};
use crate::textures::AssetManager;
use macroquad::prelude::*;
use crate::player::Player;
use crate::traits::entity::Entity;
use crate::ldtk::*;


pub struct Game {
    accumulator : f32,
    textures : AssetManager,
    camera : Camera2D,
    map : HashMap<(i32,i32), (i32,i32)>,
    to_spawn : Vec<Box<dyn Entity>>,
    mobs : Vec<Box<dyn Entity>>,
    player : Player,
    spells_to_spawn : Vec<Spell>,
    spells : Vec<Spell>
}

pub struct GameContext<'a> {
    pub map : &'a mut HashMap<(i32,i32), (i32,i32)>,
    pub player_hitbox : Rect,
    pub spells_to_spawn : &'a mut Vec<Spell>
}

impl Game {
    pub async fn new() -> Game {
        let file = load_string("assets/test.ldtk").await.unwrap();
        let project: LDtkProject = serde_json::from_str(&file).unwrap();
        let level = &project.get_levels()[2];
        let layers = level.get_layer_instances().unwrap();
        let entity_layer = layers.iter().find(|layer| layer.get_identifier() == "Entities").unwrap();
        let to_spawn = entity_to_spawn(entity_layer);
        Game {
            accumulator : 0.0,
            textures : AssetManager::load().await,
            camera : Camera2D {
                target: vec2(100.0, 100.0),
                zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()), 
                ..Default::default()
            },
            map : map_from_tiles(layers.iter() 
            .find(|layer| layer.get_identifier() == "Base")
            .unwrap().get_tiles()),
            to_spawn,
            mobs : Vec::new(),
            player : player_ldtk(entity_layer),
            spells_to_spawn : Vec::new(),
            spells : Vec::new()
        }
    }

    pub async fn update(&mut self){
        clear_background(SKYBLUE);
        let deltatime = get_frame_time();
        self.accumulator += deltatime;

        for entity in self.to_spawn.drain(..){
            self.mobs.push(entity);
        }

        let mut game_context = GameContext {
            player_hitbox : self.player.get_hitbox(),
            map: &mut self.map,
            spells_to_spawn : &mut self.spells_to_spawn
        };
        
        set_camera(&self.camera);



        while self.accumulator >= FIXED_TIMESTEP {

            self.camera.target = vec2(self.player.get_hitbox().x.round() ,self.player.get_hitbox().y.round());
            set_camera(&self.camera);

            for entity in self.mobs.iter_mut() {
                entity.update(&mut game_context);
            }

            self.player.update(&mut game_context);

            for spell in self.spells.iter_mut() {
                spell.update(&mut game_context);
            }

            self.accumulator -= FIXED_TIMESTEP;
        }

        for entity in self.mobs.iter() {
            entity.draw();
        }

        self.player.draw2().await;

        // draw_text("I LOVE KENNETH", 265., 125., 24., RED);

        self.map.iter().for_each(|tile| {
            let d = DrawTextureParams {
                source : Some(Rect::new(tile.1.0 as f32, tile.1.1 as f32, TILE_SIZE as f32, TILE_SIZE as f32)),
                ..Default::default()
            };
            draw_texture_ex(&self.textures.tileset,tile.0.0 as f32, tile.0.1 as f32, WHITE, d);
        });

        for spell in self.spells.iter() {
            spell.draw();
        }

        for spell in self.spells_to_spawn.drain(..){
            self.spells.push(spell);
        }

        self.spells.retain(|spell| spell.get_hitbox().x < self.camera.target.x + (screen_width() / self.camera.zoom.x.abs()) / 2. );

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