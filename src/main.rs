extern crate pacman_rust;

use pacman_rust::green_tea_engine;
use pacman_rust::game;

fn main() {
    //let mut _game = game::Game::init(  );
    green_tea_engine::MainLoop::run( &mut game::Game::init(  ) );
}