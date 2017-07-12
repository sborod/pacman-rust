extern crate termion;

use termion::input::{MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout};

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
    let game_field = initialize_field();

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