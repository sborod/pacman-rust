extern crate termion;

use ::game::Cell;

fn get_symbol( cell: &Cell ) -> char {
    match *cell {
        Cell::Empty => '_',
        Cell::Wall =>  '#',
        Cell::Player => '@'
    }
}

pub fn draw( game_field: &Vec< Vec<Cell> > ) {
    print!( "{clear}", clear = termion::clear::All );
    print!( "{goto}", goto = termion::cursor::Goto(1, 1) );

    let mut tmp_string: String = String::from("");
    

    for column in game_field.iter() {
        for cell in column.iter() {
            tmp_string.push( get_symbol( cell ) );
        }

        tmp_string.push_str( "\r\n" );
    }

    print!( "{}", tmp_string );

    //
    // println!( "{reset}", reset = color::Fg( color::Reset ) );
}