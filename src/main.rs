use std::collections::HashSet;
use macroquad::prelude::*;
use serde::Deserialize;

const _TILE_SIZE: i32 = 16;


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

struct Player {
    hitbox : Rect,
    vx : f32,
    vy : f32,
    on_floor: bool,
}

impl Player{
    const SPEED: f32 = 10.;
    const GRAVITY: f32 = 0.4;
    const JUMP_FORCE: f32 = -10.;

    fn update_inputs(&mut self){
        self.vx = 0.;

        if is_key_down(KeyCode::D){
            self.vx += Self::SPEED;
        }

        if is_key_down(KeyCode::A){
            self.vx += -Self::SPEED;
        }

        if is_key_pressed(KeyCode::Space) && self.on_floor {
            self.vy = Self::JUMP_FORCE;
            self.on_floor = false;
        }

    }

    fn apply_physics(&mut self){
        self.hitbox.x += self.vx;

        self.hitbox.y += self.vy;

        if !self.on_floor{
            self.vy += Self::GRAVITY;
        }
    }

    fn draw(&mut self){
        let _draw_param = DrawTextureParams{
            dest_size: Some(vec2(self.hitbox.w,self.hitbox.h)),
            ..Default::default()
        };
        draw_rectangle(self.hitbox.x, self.hitbox.y, self.hitbox.w, self.hitbox.h, PURPLE);
        
        // we use let Some ... because else we have to use clone or a lifetime to have texture in the function
        // draw_texture_ex(self.texture, self.coord.x, self.coord.y, WHITE, draw_param);
    }
    fn _action(&mut self, _map : &HashSet<[i32;2]>){
        if is_key_down(KeyCode::A){
            self.hitbox.x += -Self::SPEED;
        }
        if is_key_down(KeyCode::D){
            self.hitbox.x += Self::SPEED;
        }
        if is_key_pressed(KeyCode::Space) && self.on_floor {
            self.vy = Self::JUMP_FORCE;
            self.on_floor = false;
        }
        if !self.on_floor{
            self.vy += Self::GRAVITY;
        }
        print!("{}",self.vy);
    }

    fn update(&mut self){
        self.update_inputs();
        self.apply_physics();
        self.draw();
    }
}

struct _Resource{
    textures : Vec<Texture2D>,
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
    let mut maho_shojo = Player {
        hitbox : Rect::new(2.*16., 16.*16.-24.,20., 24.),
        vx : 3.,
        vy : 0.,
        on_floor : true,
    };

    let file = load_string("assets/test.ldtk").await.unwrap();
    let project: LDtkProject = serde_json::from_str(&file).unwrap();
    let level = &project.levels[0];
    let layers = level.layer_instances.as_ref().unwrap();
    let mut map = HashSet::new();
    layers.iter()
    .find(|layer| layer.identifier == "Base")
    .unwrap().tiles.iter()
    .for_each(|tile| {map.insert(tile.position);} );
    
    loop {
        clear_background(SKYBLUE);

        camera.target = lerp_vec2(camera.target, vec2(maho_shojo.hitbox.x,maho_shojo.hitbox.y), 0.05);
        set_camera(&camera);
        maho_shojo.update();
        
        layers.iter().find(|layer| layer.identifier == "Base").unwrap()
        .tiles.iter()
        .for_each(|tile| 
            draw_rectangle(tile.position[0] as f32, tile.position[1] as f32, 16., 16., DARKGREEN));
        next_frame().await;
    }
}