use macroquad::prelude::*;

#[macroquad::main("tourne-et-retourne")]
async fn main() {
    let texture = load_texture("assets/theo.png").await.unwrap();
    let mut rot = 0.;
    loop {
        clear_background(SKYBLUE);
        rot = rot + 0.1;
        draw_texture_ex(&texture, 100., 100., WHITE,
             DrawTextureParams{
                dest_size: Some(vec2(500., 500.)),
                rotation: rot,
                ..Default::default()
                });
        if rot >= 6.282 {
            rot = 0.;
        }
        next_frame().await; 
    }
}