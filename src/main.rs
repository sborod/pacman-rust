enum Cell {
    Empty,
    Wall,
    Player,
}

fn get_symbol( cell: Cell ) -> char {
    match cell {
        Cell::Empty => '_',
        Cell::Wall =>  '#',
        Cell::Player => '@'
    }
}

fn initialize_field( ) -> Vec<Vec<Cell>>{
    //ToDo: parse from file

    let mut result_field: Vec<Vec<Cell>> = Vec::new();

    let mut row: Vec<Cell> = Vec::new();
    row.push(Cell::Empty);
    row.push(Cell::Wall);
    result_field.push(row);

    let mut row: Vec<Cell> = Vec::new();
    row.push(Cell::Player);
    row.push(Cell::Wall);
    result_field.push(row);

    return result_field;
}

fn main() {
    let game_field = initialize_field();

    // let tmp_char = get_symbol( &game_field[0][0] );

    for row in game_field.into_iter() {
        for cell in row.into_iter() {
            let tmp = get_symbol( cell );
            print!("{}", tmp );
        }

        println!( "" );
    }
}
