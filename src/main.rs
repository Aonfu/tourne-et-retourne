use macroquad::prelude::*;

struct Entity<'a> {
    coord : Vec2,
    texture : &'a Texture2D,
    rotation : f32,
    dest_size : Option<Vec2>,
}

impl Entity {
    fn draw(&mut self){
        self.rotation = self.rotation % 6.28;
        let draw_param = DrawTextureParams{
            dest_size: self.dest_size,
            rotation : self.rotation,
            ..Default::default()
        };
        // we use let Some ... because else we have to use clone or a lifetime to have texture in the function
        draw_texture_ex(self.texture, self.coord.x, self.coord.y, WHITE, draw_param);
    }
}
impl Default for Entity{
    fn default() -> Self {
        Entity {
            coord : Vec2 {x: 0., y: 0.},
            texture : None,
            rotation : 0.,
            dest_size : None,
        }
    }
}

#[macroquad::main("tourne-et-retourne")]
async fn main() {
    let mut theo = Entity {
        texture : Some(load_texture("assets/theo.png").await.unwrap()),
        dest_size: Some(vec2(500., 500.)),
        ..Default::default()
    };
    loop {
        clear_background(SKYBLUE);
        theo.rotation = theo.rotation + 0.1;
        theo.draw();
        next_frame().await; 
    }
}