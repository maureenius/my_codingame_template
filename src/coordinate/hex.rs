use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HexCoordinate {
    x: i32,
    y: i32,
}
impl HexCoordinate {
    pub fn new(x: i32, y: i32) -> Self {
        HexCoordinate { x, y }
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}
impl FromStr for Direction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Direction::East),
            "1" => Ok(Direction::NorthEast),
            "2" => Ok(Direction::NorthWest),
            "3" => Ok(Direction::West),
            "4" => Ok(Direction::SouthWest),
            "5" => Ok(Direction::SouthEast),
            _ => Err("invalid direction"),
        }
    }
}