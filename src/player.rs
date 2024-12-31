use crate::{Board, MoveDirection};

#[derive(Default, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Default, Clone, Debug)]
pub struct Player {
    pub position: Position,
}

pub enum MovingPlayer {
    Top,
    Bottom,
}

impl Position {
    pub fn try_moving(
        &self,
        board: &Board,
        direction: MoveDirection,
    ) -> Result<Position, &'static str> {
        let mut new_position = self.to_owned();

        // if (new_position.y == board.rows.len() - 1 || new_position.y == 0)
        //     || (new_position.x == board.rows[0].len() - 1 || new_position.x == 0)
        // {
        //     return Err("");
        // }

        match direction {
            MoveDirection::Top => {
                let current_pixel = &board.rows[self.y][self.x];

                if self.y == 0 || current_pixel.top_wall {
                    return Err("");
                }

                new_position.y -= 1;
            }
            MoveDirection::Bottom => {
                let max_val = board.rows.len() - 1;
                if self.y == max_val || board.rows[self.y + 1][self.x].top_wall {
                    return Err("");
                }

                new_position.y += 1;
            }
            MoveDirection::Left => {
                let current_pixel = &board.rows[self.y][self.x];

                if self.x == 0 || current_pixel.left_wall {
                    return Err("");
                }

                new_position.x -= 1;
            }
            MoveDirection::Right => {
                let max_val = board.rows[0].len() - 1;
                if self.x == max_val || board.rows[self.y][self.x + 1].left_wall {
                    return Err("");
                }
                new_position.x += 1;
            }
        };

        Ok(new_position)
    }
}
