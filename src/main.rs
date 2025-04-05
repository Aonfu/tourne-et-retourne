use macroquad::prelude::*;
use serde::Deserialize;
use serde_json::{json,Value};

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
    identifier: String,
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

struct Entity {
    coord : Vec2,
    dest_size : Option<Vec2>,
    velocity : Vec2,
}

impl Entity{
    fn draw(&mut self){
        let _draw_param = DrawTextureParams{
            dest_size: self.dest_size,
            ..Default::default()
        };
        draw_rectangle(self.coord.x, self.coord.y, self.dest_size.unwrap().x, self.dest_size.unwrap().y, PURPLE);
        
        // we use let Some ... because else we have to use clone or a lifetime to have texture in the function
        // draw_texture_ex(self.texture, self.coord.x, self.coord.y, WHITE, draw_param);
    }
    fn movement(&mut self){
        if is_key_down(KeyCode::A){
            self.coord.x -= self.velocity.x;
        }
        if is_key_down(KeyCode::D){
            self.coord.x += self.velocity.x;
        }
        if is_key_down(KeyCode::Space) && self.coord.y == 400.{
            self.velocity.y = -12.;
        }
        if self.coord.y < 400.{
            self.velocity.y +=0.5;
        }
        self.coord.y += self.velocity.y;
    }
}


struct _Resource{
    textures : Vec<Texture2D>,
} 

#[macroquad::main(window_conf)]
async fn main() {
    let mut maho_shojo = Entity {
        coord : vec2(100., 400.),
        dest_size : Some(vec2(30., 50.)),
        velocity : vec2(5., 0.),
    };
    let file = load_string("assets/test.ldtk").await.unwrap();
    let project: LDtkProject = serde_json::from_str(&file).unwrap();
    let level = &project.levels[0];
    let layers = level.layer_instances.as_ref().unwrap();

    
    loop {
        clear_background(SKYBLUE);
        maho_shojo.movement();
        maho_shojo.draw();
        for layer in layers {
            if layer.identifier == "Base" {
                for tile in &layer.tiles {
                    let x = tile.position[0] as f32;
                    let y = tile.position[1] as f32;
                    draw_rectangle(x, y, 16.0, 16.0, DARKGREEN);
                }
            }
        }
        next_frame().await; 
    }
}