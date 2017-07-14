extern crate pacman_rust;

use pacman_rust::green_tea_engine;
use pacman_rust::game;
use pacman_rust::renderer;

fn main() {
    let mut main_loop = green_tea_engine::MainLoop::init();
    let game_field = game::initialize_field();

    loop {
        main_loop.update();
        main_loop.render( &|| {
            renderer::draw( &game_field );
        });
    }
}