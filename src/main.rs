use std::io::Read;

use labirent_alg::{Board, MovingPlayer, Position, WallType};

fn main() {
    let test = vec![1, 2, 3, 4, 5];

    let mut file = std::fs::File::open("parsy.txt").unwrap();

    let mut file_text = String::new();

    file.read_to_string(&mut file_text).unwrap();

    // println!("{file_text}");

    let mut board = Board::parse_from_str(&file_text);

    println!("{:?}", board.bottom_player);
    println!("{}", board);

    // board.move_player(MovingPlayer::Bottom, MoveDirection::Bottom);
    // board
    //     .bottom_player
    //     .move_to(&mut board, MoveDirection::Bottom);

    board.check_if_wall_is_possible(
        MovingPlayer::Bottom,
        Position { x: 1, y: 1 },
        WallType::Left,
    );

    println!("\nAfter\n");

    println!("{:?}", board.bottom_player);
    println!("{}", board);
}
