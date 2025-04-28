use macroquad::prelude::*;

pub struct AssetManager {
    pub tileset : Texture2D,
    //player : Texture2D,
    //slime : Texture2D,
}

impl AssetManager {
    pub async fn load() -> AssetManager {
        let tileset = load_texture("assets/base.png").await.unwrap();
        tileset.set_filter(FilterMode::Nearest);
        //let player = load_texture("player.png");
        //let slime = load_texture("slime.png");
        
        AssetManager {
            tileset,
        }
    }
}
