#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    None,
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn get_basic_directions() -> &'static [Direction; 4] {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        &DIRECTIONS
    }

    pub fn rotate(&self, dir: &Direction) -> Direction {
        if dir == &Direction::Right {
            return match self {
                &Direction::None => Direction::None,
                &Direction::Up => Direction::Right,
                &Direction::Right => Direction::Down,
                &Direction::Down => Direction::Left,
                &Direction::Left => Direction::Up,
            };
        } else if dir == &Direction::Left {
            return match self {
                &Direction::None => Direction::None,
                &Direction::Up => Direction::Left,
                &Direction::Left => Direction::Down,
                &Direction::Down => Direction::Right,
                &Direction::Right => Direction::Up,
            };
        }
        *self
    }
}

pub struct Coordinates {
    x: isize,
    y: isize,
}

pub struct Boundary {
    low_x_boundary: isize,
    low_y_boundary: isize,
    high_x_boundary: isize,
    high_y_boundary: isize,
}

impl Coordinates {
    pub fn new(x: isize, y: isize) -> Coordinates {
        Coordinates { x, y }
    }

    pub fn get_x(&self) -> isize {
        self.x
    }

    pub fn get_y(&self) -> isize {
        self.y
    }
}

impl Boundary {
    pub fn new(
        low_x_boundary: isize,
        low_y_boundary: isize,
        high_x_boundary: isize,
        high_y_boundary: isize,
    ) -> Boundary {
        return Boundary {
            low_x_boundary,
            low_y_boundary,
            high_x_boundary,
            high_y_boundary,
        };
    }

    pub fn new_array_boundary(high_x_boundary: usize, high_y_boundary: usize) -> Boundary {
        return Boundary {
            low_x_boundary: 0,
            low_y_boundary: 0,
            high_x_boundary: high_x_boundary as isize,
            high_y_boundary: high_y_boundary as isize,
        };
    }

    pub fn new_infinite_boundary() -> Boundary {
        return Boundary {
            low_x_boundary: isize::MIN,
            low_y_boundary: isize::MIN,
            high_x_boundary: isize::MAX,
            high_y_boundary: isize::MAX,
        };
    }

    pub fn move_array_iterator(
        &self,
        (i, j): (usize, usize),
        dir: &Direction,
    ) -> Option<(usize, usize)> {
        if let Some((new_i, new_j)) = self.move_iterator((i as isize, j as isize), dir) {
            return Some((new_i as usize, new_j as usize));
        }
        return None;
    }

    pub fn move_iterator(&self, (i, j): (isize, isize), dir: &Direction) -> Option<(isize, isize)> {
        match dir {
            Direction::None => Some((i, j)),
            Direction::Down => {
                if i >= self.high_y_boundary - 1 {
                    None
                } else {
                    Some((i + 1, j))
                }
            }
            Direction::Right => {
                if j >= self.high_x_boundary - 1 || i < self.low_y_boundary || i >= self.high_y_boundary {
                    None
                } else {
                    Some((i, j + 1))
                }
            }
            Direction::Left => {
                if j <= self.low_x_boundary || i < self.low_y_boundary || i >= self.high_y_boundary {
                    None
                } else {
                    Some((i, j - 1))
                }
            }
            Direction::Up => {
                if i <= self.low_y_boundary {
                    None
                } else {
                    Some((i - 1, j))
                }
            }
        }
    }

    pub fn wrap_array_iterator(&self, (i, j): (usize, usize), dir: &Direction) -> (usize, usize) {
        match dir {
            Direction::None => (i, j),
            Direction::Down => (0, j),
            Direction::Right => (i, 0),
            Direction::Left => (i, self.high_x_boundary as usize - 1),
            Direction::Up => (self.high_y_boundary as usize - 1, j),
        }
    }
}
