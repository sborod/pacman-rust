extern crate pacman_rust;

use pacman_rust::green_tea_engine;
use pacman_rust::game;

fn main() {
    // зачем тут переменная main_loop?
    // заменить init на run
    let mut _game = game::Game::init(  );

    let mut main_loop = green_tea_engine::MainLoop::run( &mut _game );

}