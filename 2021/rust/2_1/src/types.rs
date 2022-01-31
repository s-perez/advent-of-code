pub struct Movement {
    pub amount: i32,
    pub direction: Direction
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
