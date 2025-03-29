use macroquad::prelude::*;


#[macroquad::main("tourne-et-retourne")]
async fn main() {
    loop {
        clear_background(SKYBLUE); 

        next_frame().await; 
    }
}