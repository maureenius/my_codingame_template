use std::collections::HashMap;
use std::error::Error;
use std::io;
use crate::caribbean_game::game_rule::{Barrel, CannonBall, CaribbeanGameRule, Entity, EntityId, Mine, Owner, RumCount, Ship, Speed, StaticGameRule, Turn};
use crate::coordinate::hex::{Direction, HexCoordinate};

/// 入力行をスペースで分割しベクターにする純粋関数
fn split_input_line(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}

pub fn read_one_turn<I>(lines: &mut I) -> Result<CaribbeanGameRule, Box<dyn Error>>
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    let my_ship_count = lines.next().unwrap()?.as_str().parse()?;
    let entity_count = lines.next().unwrap()?.as_str().parse()?;
    
    let mut ships = HashMap::new();
    let mut barrels = HashMap::new();
    let mut cannon_balls = HashMap::new();
    let mut mines = HashMap::new();
    for line in lines.take(entity_count) {
        match read_entity(line?.as_str()) {
            Ok(entity) => {
                if let Some(ship) = entity.as_any().downcast_ref::<Ship>() {
                    ships.insert(ship.entity_id, *ship);
                } else if let Some(barrel) = entity.as_any().downcast_ref::<Barrel>() {
                    barrels.insert(barrel.entity_id, *barrel);
                } else if let Some(cannon_ball) = entity.as_any().downcast_ref::<CannonBall>() {
                    cannon_balls.insert(cannon_ball.entity_id, *cannon_ball);
                } else if let Some(mine) = entity.as_any().downcast_ref::<Mine>() {
                    mines.insert(mine.entity_id, *mine);
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    
    Ok(CaribbeanGameRule::new(&StaticGameRule::new(), &my_ship_count, &ships, &barrels, &cannon_balls, &mines))
}

fn read_entity(line: &str) -> Result<Box<dyn Entity>, Box<dyn Error>> {
    match split_input_line(line)[..] {
        [entity_id, "SHIP", position_x, position_y, rotation, speed, rum_count, owner, ..] => {
            let entity_id: EntityId = entity_id.parse()?;
            let position: HexCoordinate = HexCoordinate::new(position_x.parse()?, position_y.parse()?);
            let rotation: Direction = rotation.parse()?;
            let speed: Speed = speed.parse()?;
            let rum_count: RumCount = rum_count.parse()?;
            let owner: Owner = owner.parse()?;
            
            Ok(Box::new(Ship::new(&entity_id, &position, &rotation, &speed, &rum_count, &owner)))
        }
        [entity_id, "BARREL", position_x, position_y, rum_count, ..] => {
            let entity_id: EntityId = entity_id.parse()?;
            let position: HexCoordinate = HexCoordinate::new(position_x.parse()?, position_y.parse()?);
            let rum_count: RumCount = rum_count.parse()?;

            Ok(Box::new(Barrel::new(&entity_id, &position, &rum_count)))
        }
        [entity_id, "CANNONBALL", position_x, position_y, firer, impact_count, ..] => {
            let entity_id: EntityId = entity_id.parse()?;
            let position: HexCoordinate = HexCoordinate::new(position_x.parse()?, position_y.parse()?);
            let firer: EntityId = firer.parse()?;
            let impact_count: Turn = impact_count.parse()?;

            Ok(Box::new(CannonBall::new(&entity_id, &position, &firer, &impact_count)))
        }
        [entity_id, "MINE", position_x, position_y, ..] => {
            let entity_id: EntityId = entity_id.parse()?;
            let position: HexCoordinate = HexCoordinate::new(position_x.parse()?, position_y.parse()?);

            Ok(Box::new(Mine::new(&entity_id, &position)))
        }
        _ => {
            Err(format!("Invalid Entity Type {:?}", split_input_line(line)).into())
        }
    }
}
