pub struct Movement {
    pub amount: i32,
    pub direction: Direction
}

pub struct Position {
    pub x: i32,
    pub y: i32,
    pub aim: i32
}

impl Position {
    pub fn product(&self) -> i32 {
        self.x * self.y
    }

    pub fn apply_step(&self, step: Movement) -> Position {
        match step.direction {
            Direction::Forward => Position{x: self.x + step.amount, y: self.y + (self.aim * step.amount), aim: self.aim},
            Direction::Down => Position{x: self.x, y: self.y, aim: self.aim + step.amount},
            Direction::Up => Position{x: self.x, y: self.y, aim: self.aim - step.amount}
        }
    }
}

pub enum Direction {
    Forward,
    Down,
    Up,
}

pub fn get_direction_by_name(name: String) -> Result<Direction, &'static str> {
    match &name[..] {
        "forward" => Ok(Direction::Forward),
        "down" => Ok(Direction::Down),
        "up" => Ok(Direction::Up),
        _ => Err("Direction is not valid")
    }
}

