use std::collections::HashMap;

use serde::Deserialize;


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
}

impl LDtkLayer {
    pub fn get_tiles(&self) -> &Vec<LDtkTile> {
        &self.tiles
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
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

pub fn map_from_tiles(tiles : &Vec<LDtkTile>) -> HashMap<(i32,i32),(i32,i32)> {
    let mut map = HashMap::new();
    tiles.iter().for_each(|tile| {map.insert((tile.get_position()[0], tile.get_position()[1]),(tile.get_texture_position()[0], tile.get_texture_position()[1]));} );
    map
}