extern crate pokemon_go_protobuf;

use std::default::Default;
use pokemon_go_protobuf::POGOProtos_Data::PlayerData;

pub fn main() {
    let x: PlayerData = PlayerData::new();
    println!("PLAYER DATA: {:?}", x);
}
