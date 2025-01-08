use std::io::BufRead;
use crate::io::read;

mod module;
mod core;
mod caribbean_game;
mod io;
mod coordinate;

fn main() {
    loop {
        let game_rule = read::read_one_turn(&mut std::io::stdin().lock().lines()).unwrap();
        eprintln!("{:?}", game_rule);
        
        for _ in 0..game_rule.my_ship_count {
            println!("WAIT");
        }
    }
}