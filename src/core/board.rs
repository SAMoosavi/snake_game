use super::{direction::Direction, point::Point};

use rand::Rng;

pub struct Board<const N: usize> {
    game_table: [[i8; N]; N],
    length: u8,
    score: usize,
    direction: Direction,
}

impl<const N: usize> Board<N> {
    pub fn new(length: u8) -> Result<Self, String> {
        if (length as usize) + 2 > N {
            return Err(format!(
                "the table size must be grater than of start snake length + 2: {}",
                length + 2
            ));
        }

        Ok(Self {
            game_table: Self::create_table(length),
            length,
            score: 0,
            direction: Direction::Right,
        })
    }

    fn create_table(length: u8) -> [[i8; N]; N] {
        let mut game_table = [[0; N]; N];
        game_table[0].fill(-1);
        game_table[N - 1].fill(-1);
        for row in &mut game_table[1..N - 1] {
            row[0] = -1;
            row[N - 1] = -1;
        }

        let half = (N - 1) / 2;
        let offset = (length / 2) as isize;

        if length % 2 != 0 {
            for (j, i) in (-offset..=offset).rev().enumerate() {
                game_table[half][((half as isize) + i) as usize] = (j + 1) as i8;
            }
        } else {
            for (j, i) in (-offset..offset).rev().enumerate() {
                game_table[half][((half as isize) + i) as usize] = (j + 1) as i8;
            }
        }

        let Point { x, y } = Self::find_lunch_point(&game_table);
        game_table[x][y] = -2;

        game_table
    }

    pub fn get_table(&self) -> &[[i8; N]; N] {
        &self.game_table
    }

    pub fn get_score(&self) -> &usize {
        &self.score
    }

    pub fn walk(&mut self) -> bool {
        let mut tail_point = Point { x: 0, y: 0 };
        let mut head_point = Point { x: 0, y: 0 };
        for (x, row) in &mut self.game_table.iter_mut().enumerate() {
            if x != 1 || x != N - 1 {
                for (y, cell) in row.iter_mut().enumerate() {
                    if *cell > 0 {
                        if *cell == self.length as i8 {
                            *cell = 0;
                            tail_point = Point { x, y };
                        } else {
                            *cell += 1;
                            if *cell == 2 {
                                head_point = match &self.direction {
                                    Direction::Up => Point { x: x - 1, y },
                                    Direction::Down => Point { x: x + 1, y },
                                    Direction::Left => Point { x, y: y - 1 },
                                    Direction::Right => Point { x, y: y + 1 },
                                };
                            }
                        }
                    }
                }
            }
        }

        match self.game_table[head_point.x][head_point.y] {
            -2 => {
                self.length += 1;
                self.game_table[tail_point.x][tail_point.y] = self.length as i8;
                self.game_table[head_point.x][head_point.y] = 1;
                self.score += 1;

                let lunch_point = Self::find_lunch_point(&self.game_table);
                self.game_table[lunch_point.x][lunch_point.y] = -2;
                true
            }
            0 => {
                self.game_table[head_point.x][head_point.y] = 1;

                true
            }
            _ => false,
        }
    }

    pub fn rotation(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn find_lunch_point(game_table: &[[i8; N]; N]) -> Point {
        let mut rng = rand::thread_rng();
        let mut x;
        let mut y;
        loop {
            x = rng.gen_range(1..N - 1);
            y = rng.gen_range(1..N - 1);

            if game_table[x][y] == 0 {
                return Point { x, y };
            }
        }
    }
}
