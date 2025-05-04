use std::collections::HashMap;

use serde::Deserialize;

use crate::{lever::Lever, mobs::Slime, player::{self, Player}, traits::entity::{self, Entity}};


#[derive(Deserialize, Debug)]
pub struct LDtkProject {
    levels: Vec<LDtkLevel>,
}

impl LDtkProject {
    pub fn get_levels(&self) -> &Vec<LDtkLevel>{
        &self.levels
    }
}

#[derive(Deserialize, Debug)]
pub struct LDtkLevel {
    #[serde(rename = "identifier")]
    // it's unuse so the name is to del the warning
    _identifier: String,
    #[serde(rename = "layerInstances")]
    layer_instances: Option<Vec<LDtkLayer>>,
}

impl LDtkLevel {
    pub fn get_layer_instances(&self) -> Option<&Vec<LDtkLayer>>{
        self.layer_instances.as_ref()
    }
}

#[derive(Deserialize, Debug)]
pub struct LDtkLayer {
    #[serde(rename = "__identifier")]
    identifier: String,

    #[serde(rename = "gridTiles")]
    tiles: Vec<LDtkTile>,

    #[serde(rename = "entityInstances")]
    entities : Vec<LDtkEntity>,
}

impl LDtkLayer {
    pub fn get_tiles(&self) -> &Vec<LDtkTile> {
        &self.tiles
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }

    pub fn get_entities(&self) -> &Vec<LDtkEntity> {
        &self.entities
    }
}

#[derive(Deserialize, Debug)]
pub struct LDtkTile {
    #[serde(rename = "px")]
    position: [i32; 2],
    #[serde(rename = "src")]
    texture_position: [i32; 2],
}

impl LDtkTile {
    pub fn get_position(&self) -> &[i32; 2] {
        &self.position
    }

    pub fn get_texture_position(&self) -> &[i32; 2] {
        &self.texture_position
    }
}

#[derive(Deserialize, Debug)]
pub struct LDtkEntity{
    #[serde(rename = "__identifier")]
    identifier : String,
    #[serde(rename = "px")]
    position: [i32; 2],
}

pub fn map_from_tiles(tiles : &Vec<LDtkTile>) -> HashMap<(i32,i32),(i32,i32)> {
    let mut map = HashMap::new();
    tiles.iter().for_each(|tile| {map.insert((tile.get_position()[0], tile.get_position()[1]),(tile.get_texture_position()[0], tile.get_texture_position()[1]));} );
    map
}

pub fn entity_to_spawn(entity_layer: &LDtkLayer) -> Vec<Box<dyn Entity>> {
    let mut entities: Vec<Box<dyn Entity>>= Vec::new();
    for entity in entity_layer.get_entities().iter() {
        match entity.identifier.as_str() {
            "Slime" => entities.push(Box::new(Slime::new(entity.position[0] as f32, entity.position[1] as f32))),
            "Lever" => entities.push(Box::new(Lever::new(entity.position[0] as f32, entity.position[1] as f32))),
            _ => ()
        }
    }
    entities
}

pub fn player_ldtk(entity_layer: &LDtkLayer) -> Player {
    let ldtk_player = entity_layer.get_entities().iter().find(|entity| entity.identifier == "Player").unwrap();
    Player::new(ldtk_player.position[0] as f32, ldtk_player.position[1] as f32)

}