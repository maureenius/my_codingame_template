use std::any::Any;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use crate::coordinate::hex::{Direction, HexCoordinate};

#[derive(Debug)]
pub struct CaribbeanGameRule {
    static_game_rule: StaticGameRule,
    pub my_ship_count: usize,
    ships: HashMap<EntityId, Ship>,
    barrels: HashMap<EntityId, Barrel>,
    cannon_balls: HashMap<EntityId, CannonBall>,
    mines: HashMap<EntityId, Mine>,
    current: Turn,
}
impl CaribbeanGameRule {
    pub fn new(static_game_rule: &StaticGameRule, my_ship_count: &usize, ships: &HashMap<EntityId, Ship>, barrels: &HashMap<EntityId, Barrel>, cannon_balls: &HashMap<EntityId, CannonBall>, mines: &HashMap<EntityId, Mine>) -> Self {
        CaribbeanGameRule {
            static_game_rule: static_game_rule.clone(),
            my_ship_count: *my_ship_count,
            ships: ships.clone(),
            barrels: barrels.clone(),
            cannon_balls: cannon_balls.clone(),
            mines: mines.clone(),
            current: Turn(0),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct StaticGameRule {
    pub width: usize,
    pub height: usize,
}
impl StaticGameRule {
    pub fn new() -> Self {
        StaticGameRule {
            width: 23,
            height: 21,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Turn(pub usize);
impl FromStr for Turn {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Turn)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct EntityId(pub usize);
impl FromStr for EntityId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(EntityId)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Speed {
    Full,
    Half,
    Stop,
}
impl FromStr for Speed {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Speed::Stop),
            "1" => Ok(Speed::Half),
            "2" => Ok(Speed::Full),
            _ => Err("invalid speed"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RumCount(pub usize);
impl FromStr for RumCount {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(RumCount)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Owner {
    Me,
    Opponent,
}
impl FromStr for Owner {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Owner::Opponent),
            "1" => Ok(Owner::Me),
            _ => Err("invalid owner"),
        }
    }
}

pub trait Entity: Any {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Ship {
    pub entity_id: EntityId,
    position: HexCoordinate,
    rotation: Direction,
    speed: Speed,
    rum_count: RumCount,
    owner: Owner
}
impl Entity for Ship {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Ship {
    pub fn new(entity_id: &EntityId, position: &HexCoordinate, rotation: &Direction, speed: &Speed, rum_count: &RumCount, owner: &Owner) -> Self {
        Ship {
            entity_id: *entity_id,
            position: *position,
            rotation: *rotation,
            speed: *speed,
            rum_count: *rum_count,
            owner: *owner,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Barrel {
    pub entity_id: EntityId,
    position: HexCoordinate,
    rum_count: RumCount,
}
impl Entity for Barrel {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Barrel {
    pub fn new(entity_id: &EntityId, position: &HexCoordinate, rum_count: &RumCount) -> Self {
        Barrel {
            entity_id: *entity_id,
            position: *position,
            rum_count: *rum_count,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CannonBall {
    pub entity_id: EntityId,
    position: HexCoordinate,
    firer: EntityId,
    impact_count: Turn,
}
impl Entity for CannonBall {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl CannonBall {
    pub fn new(entity_id: &EntityId, position: &HexCoordinate, firer: &EntityId, impact_count: &Turn) -> Self {
        CannonBall {
            entity_id: *entity_id,
            position: *position,
            firer: *firer,
            impact_count: *impact_count,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Mine {
    pub entity_id: EntityId,
    position: HexCoordinate,
}
impl Entity for Mine {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Mine {
    pub fn new(entity_id: &EntityId, position: &HexCoordinate) -> Self {
        Mine {
            entity_id: *entity_id,
            position: *position,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ShipAction {
    Move(HexCoordinate),
    Fire(HexCoordinate),
    Mine,
    Port,
    Starboard,
    Faster,
    Slower,
    Wait,
}
