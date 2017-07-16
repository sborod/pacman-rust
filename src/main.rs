extern crate pacman_rust;

use pacman_rust::green_tea_engine;
use pacman_rust::game;

fn main() {
    let game_field = game::initialize_field();
    let main_loop = green_tea_engine::MainLoop::init( &|| {
        game::draw( &game_field );
    } );
}