extern crate termion;

use ::green_tea_engine;

use self::termion::raw::IntoRawMode;
use self::termion::async_stdin;
use std::io::{Read, stdout};

pub enum Cell {
    Empty,
    Wall,
    Player,
}

pub struct Game {
    game_field: Vec< Vec<Cell> >
}

impl green_tea_engine::Gamable for Game {
    fn input_handling( &mut self ) {
        self.input_handle();
    }

    fn updating( &mut self ) {
        self.update();
    }

    fn rendering( &self ) {
        self.draw();
    }

}

impl Game {
    pub fn init() -> Game {
        let stdout = stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();
        
        // вот это
        let mut stdin = async_stdin().bytes();

        Game {
            game_field: initialize_field()
        }
    }

    pub fn input_handle( &mut self ) {
        let byte = stdin.next();

        if let Some( Ok( b'q' ) ) = byte {
            // break;
        }
    }

    pub fn update( &mut self ) {
    }

    pub fn draw( &self ) {
        print!( "{clear}", clear = termion::clear::All );
        print!( "{goto}", goto = termion::cursor::Goto(1, 1) );

        let mut tmp_string: String = String::from("");
        

        for column in self.game_field.iter() {
            for cell in column.iter() {
                tmp_string.push( get_symbol( cell ) );
            }

            tmp_string.push_str( "\r\n" );
        }

        print!( "{}", tmp_string );

        //
        // println!( "{reset}", reset = color::Fg( color::Reset ) );
    }
}

pub fn initialize_field() -> Vec< Vec<Cell> >{
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

fn get_symbol( cell: &Cell ) -> char {
    match *cell {
        Cell::Empty => '_',
        Cell::Wall =>  '#',
        Cell::Player => '@'
    }
}