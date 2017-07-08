enum Cell {
    Empty,
    Wall,
    Player
}

fn get_symbol( cell: &Cell ) -> char {
    match cell {
        Empty => '_',
        Wall =>  '#',
        Player => '@'
    }
}

fn main() {
    let mut game_field = [ [ Cell::Empty, Cell::Wall ],
                           [ Cell::Empty, Cell::Empty ] ];

    // let tmp_char = get_symbol( &game_field[0][0] );

    for row in game_field.into_iter() {
        for cell in row.into_iter() {
            let tmp = get_symbol( &cell );
            println!("{}", tmp );
        }

        println!( "\r\n" );
    }
}
