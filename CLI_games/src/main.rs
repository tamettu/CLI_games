mod create_vec_map;
mod tactictoe;
mod input;
//use input::input;
use tactictoe::Tactictoe;

pub fn main() {
    let mut tactictoe = Tactictoe::default();
    tactictoe.game_start_cli();
}
