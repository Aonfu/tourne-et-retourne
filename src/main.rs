use std::collections::HashSet;
use macroquad::prelude::*;
use serde::Deserialize;

const TILE_SIZE: i32 = 16;


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

struct Player {
    hitbox : Rect,
    vx : f32,
    vy : f32,
    on_floor: bool,
}

impl Player{
    const SPEED: f32 = 3.;
    const GRAVITY: f32 = 0.3;
    const JUMP_FORCE: f32 = -8.;

    fn update_inputs(&mut self){
        self.vx = 0.;

        if is_key_down(KeyCode::D){
            self.vx += Self::SPEED;
        }

        if is_key_down(KeyCode::A){
            self.vx += -Self::SPEED;
        }

        if is_key_down(KeyCode::Space) && self.on_floor {
            self.vy = Self::JUMP_FORCE;
            self.on_floor = false;
        }

        if is_key_pressed(KeyCode::Enter){
            self.hitbox.x = 9.*16.;
            self.hitbox.y = 13.*16.-24.;
        }

    }

    fn check_collision_x(&mut self, map : &HashSet<(i32, i32)>){

        let left = self.hitbox.x as i32;
        let right = (self.hitbox.x + self.hitbox.w) as i32;
        let top = self.hitbox.y as i32;
        let bottom = (self.hitbox.y + self.hitbox.h) as i32;

        let x = if self.vx > 0. {
            right
        } else {
            left
        };

        for y in (top..=bottom).step_by(TILE_SIZE as usize) {
            let tile_x = x / TILE_SIZE * TILE_SIZE;
            let tile_y = y / TILE_SIZE * TILE_SIZE;

            if map.contains(&(tile_x,tile_y)) {
                if x == left {
                    self.hitbox.x = (tile_x + TILE_SIZE) as f32;
                    self.vx = 0.; // line useless for now but useful for understanding
                } else {
                    self.hitbox.x = tile_x as f32 - self.hitbox.w;
                    self.vx = 0.; // line useless for now but useful for understanding
                }
            }
        }
    }

    fn check_collision_y(&mut self, map : &HashSet<(i32, i32)>){

        self.on_floor = false; // it will true if the floor is detected

        let left = self.hitbox.x as i32;
        let right = (self.hitbox.x + self.hitbox.w) as i32;
        let top = self.hitbox.y as i32;
        let bottom = (self.hitbox.y + self.hitbox.h) as i32;

        let y = if self.vy < 0. {
            top
        } else {
            bottom
        };

        for x in (left..=right).filter(|x| if right%TILE_SIZE==0 {*x!=right} else {true}).step_by(TILE_SIZE as usize) {
            let tile_x: i32 = x / TILE_SIZE * TILE_SIZE;
            let tile_y: i32 = y / TILE_SIZE * TILE_SIZE;

            if map.contains(&(tile_x,tile_y)) {
                if y == bottom {
                    self.hitbox.y = tile_y as f32 - self.hitbox.h;
                    self.vy = 0.;
                    self.on_floor = true;
                } else {
                    self.hitbox.y = (tile_y + TILE_SIZE) as f32;
                    self.vy = 0.;
                }
            }
        }
    }

    fn apply_physics(&mut self, map:&HashSet<(i32,i32)>){

        self.hitbox.x += self.vx;
        self.check_collision_x(map);

        self.hitbox.y += self.vy;
        self.check_collision_y(map);

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

    fn update(&mut self, map : &HashSet<(i32, i32)>){
        self.update_inputs();
        self.apply_physics(map);
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
        hitbox : Rect::new(9.*16., 13.*16.-24.,16., 20.),
        vx : 0.,
        vy : 0.,
        on_floor : true,
    };

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

        camera.target = lerp_vec2(camera.target, vec2(maho_shojo.hitbox.x,maho_shojo.hitbox.y), 0.05);
        set_camera(&camera);
        maho_shojo.update(&map);
        
        layers.iter().find(|layer| layer.identifier == "Base").unwrap()
        .tiles.iter()
        .for_each(|tile| 
            draw_rectangle(tile.position[0] as f32, tile.position[1] as f32, 16., 16., DARKGREEN));
        next_frame().await;
    }
}