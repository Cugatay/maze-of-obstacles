// ;

use std::io::Read;

use labirent_alg::{Board, MoveDirection, MovingPlayer};

fn main() {
    let mut file = std::fs::File::open("parsy.txt").unwrap();

    let mut file_text = String::new();

    file.read_to_string(&mut file_text).unwrap();

    // println!("{file_text}");

    let mut board = Board::parse_from_str(&file_text);

    println!("{:?}", board.bottom_player);
    println!("{}", board);

    board.move_player(MovingPlayer::Bottom, MoveDirection::Bottom);
    // board
    //     .bottom_player
    //     .move_to(&mut board, MoveDirection::Bottom);

    println!("\nAfter\n");

    println!("{:?}", board.bottom_player);
    println!("{}", board);
}
