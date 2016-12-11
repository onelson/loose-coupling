
extern crate time;
extern crate specs;
extern crate rand;
extern crate radiant_rs;

use radiant_rs::{DisplayInfo, Display, Renderer, Layer, Sprite, Font, FontInfo, Color, blendmodes, utils};

mod assets;
mod components;
mod game;
mod systems;

fn main() {
    let (width, height) = (300, 300);
    let display = Display::new(DisplayInfo { width: width, height: height, vsync: true, ..DisplayInfo::default() });
    let renderer = Renderer::new(&display);

    let game = game::Game::new(&renderer);

    std::thread::spawn(|| {
        let mut game = game;
        while game.tick() {}
    });

    utils::renderloop(|state| {

        game.layer.clear();

        renderer.clear_target(Color::white());
        renderer.draw_layer(&game.layer);
        renderer.swap_target();

        !display.poll_events().was_closed()
    });

}