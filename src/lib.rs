pub mod player;

use std::{fmt, thread::current};

pub use player::{MovingPlayer, Player, Position};

#[derive(Default, Clone, Debug)]
pub struct Pixel {
    pub top_wall: bool,
    // pub bottom_wall: bool,
    pub left_wall: bool,
}

type Row = [Pixel; 9];

pub enum WallType {
    Top,
    Left,
}

pub enum MoveDirection {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Default, Clone, Debug)]
pub struct Board {
    pub rows: [Row; 9],
    pub top_player: Player,
    pub bottom_player: Player,
}

impl Board {
    pub fn parse_from_str(data: &str) -> Board {
        let mut board = Board::default();

        let (mut x, mut y) = (0, 0);

        let mut current_pixel: Option<Pixel> = None;

        for c in data.chars() {
            match c {
                'x' => {
                    x += 1;
                }
                '\n' => {
                    if current_pixel.is_some() {
                        board.rows[y][x] = current_pixel.take().unwrap();
                    }
                    y += 1;
                    x = 0;
                }
                ' ' => {
                    if current_pixel.is_some() {
                        board.rows[y][x] = current_pixel.take().unwrap();
                        x += 1;
                    }
                }
                'T' => {
                    board.top_player.position = Position { x, y };
                    if current_pixel.is_none() {
                        current_pixel = Some(Pixel::default())
                    }
                    // x += 1;
                }
                'B' => {
                    board.bottom_player.position = Position { x, y };
                    if current_pixel.is_none() {
                        current_pixel = Some(Pixel::default())
                    }
                    // x += 1;
                }
                '-' => match current_pixel {
                    Some(ref mut val) => val.top_wall = true,
                    None => {
                        current_pixel = Some(Pixel {
                            top_wall: true,
                            left_wall: false,
                        })
                    }
                },
                '|' => match current_pixel {
                    Some(ref mut val) => val.left_wall = true,
                    None => {
                        current_pixel = Some(Pixel {
                            left_wall: true,
                            top_wall: false,
                        })
                    }
                },
                _ => panic!("On hell, unexpected character: {c}"),
            }
        }

        board
    }

    pub fn move_player(&mut self, chosen_player: MovingPlayer, direction: MoveDirection) {
        let new_pos = match chosen_player {
            MovingPlayer::Top => &self.top_player,
            MovingPlayer::Bottom => &self.bottom_player,
        }
        .position
        .try_moving(self, direction)
        .expect("Illegal move");

        println!("new pos is: {new_pos:?}");
        match chosen_player {
            MovingPlayer::Top => self.top_player.position = new_pos,
            MovingPlayer::Bottom => self.bottom_player.position = new_pos,
        };
    }

    pub fn check_if_wall_is_possible(
        &self,
        moving_player: MovingPlayer,
        position: Position,
        wall_type: WallType,
    ) {
        let mut virtual_board = self.to_owned();
        match wall_type {
            WallType::Top => virtual_board.rows[position.y][position.x].top_wall = true,
            WallType::Left => virtual_board.rows[position.y][position.x].left_wall = true,
        }
        let player = match moving_player {
            MovingPlayer::Top => &virtual_board.top_player,
            MovingPlayer::Bottom => &virtual_board.bottom_player,
        };

        let mut past_moves: Vec<Position> = vec![];
        let mut current_moves = vec![player.position.to_owned()];
        let mut push_moves: Vec<Position> = vec![];

        while !current_moves.is_empty() {
            for m in &current_moves {
                let clone_pushes = push_moves.to_owned();

                push_moves.extend(
                    [
                        MoveDirection::Top,
                        MoveDirection::Left,
                        MoveDirection::Right,
                        MoveDirection::Bottom,
                    ]
                    .into_iter()
                    .filter_map(|dir| m.try_moving(&virtual_board, dir).ok())
                    .filter(|pos| {
                        // ![&current_moves, &clone_pushes, &past_moves]
                        ![&clone_pushes, &past_moves].iter().any(|moves| {
                            moves
                                .iter()
                                .any(|exist_pos| pos.x == exist_pos.x && pos.y == exist_pos.y)
                        })
                    }),
                );
            }

            past_moves.extend(current_moves);
            current_moves = push_moves;
            push_moves = vec![];
        }

        let win_condition = |pos: Position| match moving_player {
            MovingPlayer::Bottom => pos.y == 0,
            MovingPlayer::Top => pos.y == virtual_board.rows.len() - 1,
        };

        let if_has_win_path = past_moves
            .iter()
            .filter(|&pos| win_condition(pos.clone()))
            .collect::<Vec<_>>();
        // .count();
        // > 0;

        // println!("If has win path: {if_has_win_path}");
        println!("Past moves: {past_moves:?}");
        println!(
            "

If has win path: {if_has_win_path:?}"
        );
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = self.to_owned();
        let mut val = String::with_capacity(board.rows.len() * board.rows[0].len());

        for (y, row) in board.rows.into_iter().enumerate() {
            let mut is_empty = true;

            for (x, pixel) in row.into_iter().enumerate() {
                if (board.top_player.position.x, board.top_player.position.y) == (x, y) {
                    val += "T";
                    is_empty = false;
                }
                if (
                    board.bottom_player.position.x,
                    board.bottom_player.position.y,
                ) == (x, y)
                {
                    val += "B";
                    is_empty = false;
                }

                if pixel.top_wall {
                    val += "-";
                    is_empty = false;
                }

                if pixel.left_wall {
                    val += "|";
                    is_empty = false;
                }

                if is_empty {
                    val += "x";
                }

                val += " ";
                is_empty = true;
            }
            val += "\n"
        }

        f.write_str(val.as_str())
    }
}
