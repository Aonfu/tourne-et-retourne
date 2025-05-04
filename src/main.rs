mod player;
mod constants;
mod traits;
mod mobs;
mod ldtk;
mod game;
mod textures;
mod screen;
mod menu;
mod lever;

use game::Game;
use screen::Screen;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Refind Maho".to_string(),
        window_width: 640,
        window_height: 320,
        fullscreen: false,
        ..Default::default()
    };
    // to have a maximum of fps
    // conf.platform.swap_interval = Some(0); 
    conf
}

#[macroquad::main(window_conf)]
async fn main() {
    let screen = Screen::Level;
    match screen {
        Screen::Menu => {
            
        }

        Screen::Level => {
            let mut game = Game::new().await;
            
            loop {
                game.update().await;
            }
        }
    }
}