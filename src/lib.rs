use std::fmt;

#[derive(Default, Clone, Debug)]
pub struct Pixel {
    pub top_wall: bool,
    // pub bottom_wall: bool,
    pub left_wall: bool,
}

type Row = [Pixel; 9];

#[derive(Default, Clone, Debug)]
pub struct Player {
    pub x: usize,
    pub y: usize,
}

pub enum MoveDirection {
    Top,
    Bottom,
    Left,
    Right,
}

pub enum MovingPlayer {
    Top,
    Bottom,
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
                'T' => board.top_player = Player { x, y },
                'B' => board.bottom_player = Player { x, y },
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
        let player = match chosen_player {
            MovingPlayer::Top => &mut self.top_player,
            MovingPlayer::Bottom => &mut self.bottom_player,
        };

        match direction {
            MoveDirection::Top => {
                let current_pixel = &self.rows[player.y][player.x];

                if player.y == 0 || current_pixel.top_wall {
                    panic!("Illegal top move")
                }

                player.y -= 1;
            }
            MoveDirection::Bottom => {
                let max_val = self.rows.len() - 1;
                if player.y == max_val || self.rows[player.y + 1][player.x].top_wall {
                    panic!("Illegal bottom move")
                }

                player.y += 1;
            }
            MoveDirection::Left => {
                let current_pixel = &self.rows[player.y][player.x];

                if player.y == 0 || current_pixel.left_wall {
                    panic!("Illegal left move")
                }

                player.x -= 1;
            }
            MoveDirection::Right => {
                let max_val = self.rows[0].len() - 1;
                if player.x == max_val || self.rows[player.y][player.x + 1].left_wall {
                    panic!("Illegal right move")
                }

                player.x += 1;
            }
        };
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = self.to_owned();
        let mut val = String::with_capacity(board.rows.len() * board.rows[0].len());

        for (y, row) in board.rows.into_iter().enumerate() {
            let mut is_empty = true;

            for (x, pixel) in row.into_iter().enumerate() {
                if (board.top_player.x, board.top_player.y) == (x, y) {
                    val += "T";
                    is_empty = false;
                }
                if (board.bottom_player.x, board.bottom_player.y) == (x, y) {
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
