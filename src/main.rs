extern crate time;
extern crate termion;

use termion::{color,style,clear,cursor};
use termion::input::{MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout};

const FRAME_PER_SECOND: i64 = 120;

fn timestamp() -> i64 {
    let current_time = time::get_time();
    current_time.sec*1000 + i64::from( current_time.nsec / 1_000_000 )
}

struct MainLoop {
    initial_time:        i64,
    current_time:        i64,
    current_frame:       i64,
    last_rendered_frame: i64,
    game_objects:        Vec<GameObject>
}

impl MainLoop {
    fn init() -> MainLoop {
        MainLoop {
            initial_time:        timestamp(),
            current_time:        0,
            current_frame:       0,
            last_rendered_frame: 0,
            game_objects:        vec![]
        }
    }

    fn update( &mut self ) {
        self.current_time = timestamp() - self.initial_time;
        self.current_frame = self.current_time*FRAME_PER_SECOND / 1000;

        if self.current_frame > self.last_rendered_frame {
            self.game_objects[0].charge += 100.0;

            if self.game_objects[0].charge > self.game_objects[0].threshold {
                self.game_objects[0].charge = 0.0;
            }
        }
    }

    fn render( &mut self ) {
        if self.current_frame > self.last_rendered_frame {
            println!( "{clear}", clear = clear::All );
            println!( "{goto}", goto = cursor::Goto(1, 1) );

            //
            println!( "Current time: {t}", t = ( self.current_time as f64 ) / 1000.0 );
            println!( "Neuron charge: {charge}", charge = self.game_objects[0].charge );
            
            //
            println!( "{reset}", reset = color::Fg( color::Reset ) );
            
            // 
            self.last_rendered_frame = self.current_frame;
        }
    }
}

enum Cell {
    Empty,
    Wall,
    Player,
}

fn get_symbol( cell: &Cell ) -> char {
    match *cell {
        Cell::Empty => '_',
        Cell::Wall =>  '#',
        Cell::Player => '@'
    }
}

fn initialize_field( ) -> Vec<Vec<Cell>>{
    //ToDo: parse from file

    let mut result_field: Vec< Vec<Cell> > = Vec::new();

    let mut row: Vec<Cell> = Vec::new();
    row.push(Cell::Empty);
    row.push(Cell::Wall);
    result_field.push(row);

    let mut row: Vec<Cell> = Vec::new();
    row.push(Cell::Player);
    row.push(Cell::Wall);
    result_field.push(row);

    result_field
}

fn main() {
    let mut main_loop = MainLoop::init();
    let game_field = initialize_field();

    loop {
        main_loop.update();
        main_loop.render();
    }

    draw(game_field);
}

fn draw( game_field: Vec< Vec<Cell> > ) {
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    writeln!(stdout, "{}", termion::clear::All);

    for (i, column) in game_field.iter().enumerate() {
        for(j, cell) in column.iter().enumerate() {
            let tmp = get_symbol( cell );
            write!(stdout, "{}{}\r\n", 
                 termion::cursor::Goto((i + 1) as u16, (j + 1) as u16), tmp );
        }
    }
}