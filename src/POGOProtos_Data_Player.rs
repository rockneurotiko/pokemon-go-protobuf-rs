// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct PlayerStats {
    // message fields
    level: ::std::option::Option<i32>,
    experience: ::std::option::Option<i64>,
    prev_level_xp: ::std::option::Option<i64>,
    next_level_xp: ::std::option::Option<i64>,
    km_walked: ::std::option::Option<f32>,
    pokemons_encountered: ::std::option::Option<i32>,
    unique_pokedex_entries: ::std::option::Option<i32>,
    pokemons_captured: ::std::option::Option<i32>,
    evolutions: ::std::option::Option<i32>,
    poke_stop_visits: ::std::option::Option<i32>,
    pokeballs_thrown: ::std::option::Option<i32>,
    eggs_hatched: ::std::option::Option<i32>,
    big_magikarp_caught: ::std::option::Option<i32>,
    battle_attack_won: ::std::option::Option<i32>,
    battle_attack_total: ::std::option::Option<i32>,
    battle_defended_won: ::std::option::Option<i32>,
    battle_training_won: ::std::option::Option<i32>,
    battle_training_total: ::std::option::Option<i32>,
    prestige_raised_total: ::std::option::Option<i32>,
    prestige_dropped_total: ::std::option::Option<i32>,
    pokemon_deployed: ::std::option::Option<i32>,
    pokemon_caught_by_type: ::std::vec::Vec<i32>,
    small_rattata_caught: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayerStats {}

impl PlayerStats {
    pub fn new() -> PlayerStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayerStats {
        static mut instance: ::protobuf::lazy::Lazy<PlayerStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayerStats,
        };
        unsafe {
            instance.get(|| {
                PlayerStats {
                    level: ::std::option::Option::None,
                    experience: ::std::option::Option::None,
                    prev_level_xp: ::std::option::Option::None,
                    next_level_xp: ::std::option::Option::None,
                    km_walked: ::std::option::Option::None,
                    pokemons_encountered: ::std::option::Option::None,
                    unique_pokedex_entries: ::std::option::Option::None,
                    pokemons_captured: ::std::option::Option::None,
                    evolutions: ::std::option::Option::None,
                    poke_stop_visits: ::std::option::Option::None,
                    pokeballs_thrown: ::std::option::Option::None,
                    eggs_hatched: ::std::option::Option::None,
                    big_magikarp_caught: ::std::option::Option::None,
                    battle_attack_won: ::std::option::Option::None,
                    battle_attack_total: ::std::option::Option::None,
                    battle_defended_won: ::std::option::Option::None,
                    battle_training_won: ::std::option::Option::None,
                    battle_training_total: ::std::option::Option::None,
                    prestige_raised_total: ::std::option::Option::None,
                    prestige_dropped_total: ::std::option::Option::None,
                    pokemon_deployed: ::std::option::Option::None,
                    pokemon_caught_by_type: ::std::vec::Vec::new(),
                    small_rattata_caught: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 level = 1;

    pub fn clear_level(&mut self) {
        self.level = ::std::option::Option::None;
    }

    pub fn has_level(&self) -> bool {
        self.level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_level(&mut self, v: i32) {
        self.level = ::std::option::Option::Some(v);
    }

    pub fn get_level(&self) -> i32 {
        self.level.unwrap_or(0)
    }

    // optional int64 experience = 2;

    pub fn clear_experience(&mut self) {
        self.experience = ::std::option::Option::None;
    }

    pub fn has_experience(&self) -> bool {
        self.experience.is_some()
    }

    // Param is passed by value, moved
    pub fn set_experience(&mut self, v: i64) {
        self.experience = ::std::option::Option::Some(v);
    }

    pub fn get_experience(&self) -> i64 {
        self.experience.unwrap_or(0)
    }

    // optional int64 prev_level_xp = 3;

    pub fn clear_prev_level_xp(&mut self) {
        self.prev_level_xp = ::std::option::Option::None;
    }

    pub fn has_prev_level_xp(&self) -> bool {
        self.prev_level_xp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prev_level_xp(&mut self, v: i64) {
        self.prev_level_xp = ::std::option::Option::Some(v);
    }

    pub fn get_prev_level_xp(&self) -> i64 {
        self.prev_level_xp.unwrap_or(0)
    }

    // optional int64 next_level_xp = 4;

    pub fn clear_next_level_xp(&mut self) {
        self.next_level_xp = ::std::option::Option::None;
    }

    pub fn has_next_level_xp(&self) -> bool {
        self.next_level_xp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_next_level_xp(&mut self, v: i64) {
        self.next_level_xp = ::std::option::Option::Some(v);
    }

    pub fn get_next_level_xp(&self) -> i64 {
        self.next_level_xp.unwrap_or(0)
    }

    // optional float km_walked = 5;

    pub fn clear_km_walked(&mut self) {
        self.km_walked = ::std::option::Option::None;
    }

    pub fn has_km_walked(&self) -> bool {
        self.km_walked.is_some()
    }

    // Param is passed by value, moved
    pub fn set_km_walked(&mut self, v: f32) {
        self.km_walked = ::std::option::Option::Some(v);
    }

    pub fn get_km_walked(&self) -> f32 {
        self.km_walked.unwrap_or(0.)
    }

    // optional int32 pokemons_encountered = 6;

    pub fn clear_pokemons_encountered(&mut self) {
        self.pokemons_encountered = ::std::option::Option::None;
    }

    pub fn has_pokemons_encountered(&self) -> bool {
        self.pokemons_encountered.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemons_encountered(&mut self, v: i32) {
        self.pokemons_encountered = ::std::option::Option::Some(v);
    }

    pub fn get_pokemons_encountered(&self) -> i32 {
        self.pokemons_encountered.unwrap_or(0)
    }

    // optional int32 unique_pokedex_entries = 7;

    pub fn clear_unique_pokedex_entries(&mut self) {
        self.unique_pokedex_entries = ::std::option::Option::None;
    }

    pub fn has_unique_pokedex_entries(&self) -> bool {
        self.unique_pokedex_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unique_pokedex_entries(&mut self, v: i32) {
        self.unique_pokedex_entries = ::std::option::Option::Some(v);
    }

    pub fn get_unique_pokedex_entries(&self) -> i32 {
        self.unique_pokedex_entries.unwrap_or(0)
    }

    // optional int32 pokemons_captured = 8;

    pub fn clear_pokemons_captured(&mut self) {
        self.pokemons_captured = ::std::option::Option::None;
    }

    pub fn has_pokemons_captured(&self) -> bool {
        self.pokemons_captured.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemons_captured(&mut self, v: i32) {
        self.pokemons_captured = ::std::option::Option::Some(v);
    }

    pub fn get_pokemons_captured(&self) -> i32 {
        self.pokemons_captured.unwrap_or(0)
    }

    // optional int32 evolutions = 9;

    pub fn clear_evolutions(&mut self) {
        self.evolutions = ::std::option::Option::None;
    }

    pub fn has_evolutions(&self) -> bool {
        self.evolutions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_evolutions(&mut self, v: i32) {
        self.evolutions = ::std::option::Option::Some(v);
    }

    pub fn get_evolutions(&self) -> i32 {
        self.evolutions.unwrap_or(0)
    }

    // optional int32 poke_stop_visits = 10;

    pub fn clear_poke_stop_visits(&mut self) {
        self.poke_stop_visits = ::std::option::Option::None;
    }

    pub fn has_poke_stop_visits(&self) -> bool {
        self.poke_stop_visits.is_some()
    }

    // Param is passed by value, moved
    pub fn set_poke_stop_visits(&mut self, v: i32) {
        self.poke_stop_visits = ::std::option::Option::Some(v);
    }

    pub fn get_poke_stop_visits(&self) -> i32 {
        self.poke_stop_visits.unwrap_or(0)
    }

    // optional int32 pokeballs_thrown = 11;

    pub fn clear_pokeballs_thrown(&mut self) {
        self.pokeballs_thrown = ::std::option::Option::None;
    }

    pub fn has_pokeballs_thrown(&self) -> bool {
        self.pokeballs_thrown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokeballs_thrown(&mut self, v: i32) {
        self.pokeballs_thrown = ::std::option::Option::Some(v);
    }

    pub fn get_pokeballs_thrown(&self) -> i32 {
        self.pokeballs_thrown.unwrap_or(0)
    }

    // optional int32 eggs_hatched = 12;

    pub fn clear_eggs_hatched(&mut self) {
        self.eggs_hatched = ::std::option::Option::None;
    }

    pub fn has_eggs_hatched(&self) -> bool {
        self.eggs_hatched.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eggs_hatched(&mut self, v: i32) {
        self.eggs_hatched = ::std::option::Option::Some(v);
    }

    pub fn get_eggs_hatched(&self) -> i32 {
        self.eggs_hatched.unwrap_or(0)
    }

    // optional int32 big_magikarp_caught = 13;

    pub fn clear_big_magikarp_caught(&mut self) {
        self.big_magikarp_caught = ::std::option::Option::None;
    }

    pub fn has_big_magikarp_caught(&self) -> bool {
        self.big_magikarp_caught.is_some()
    }

    // Param is passed by value, moved
    pub fn set_big_magikarp_caught(&mut self, v: i32) {
        self.big_magikarp_caught = ::std::option::Option::Some(v);
    }

    pub fn get_big_magikarp_caught(&self) -> i32 {
        self.big_magikarp_caught.unwrap_or(0)
    }

    // optional int32 battle_attack_won = 14;

    pub fn clear_battle_attack_won(&mut self) {
        self.battle_attack_won = ::std::option::Option::None;
    }

    pub fn has_battle_attack_won(&self) -> bool {
        self.battle_attack_won.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battle_attack_won(&mut self, v: i32) {
        self.battle_attack_won = ::std::option::Option::Some(v);
    }

    pub fn get_battle_attack_won(&self) -> i32 {
        self.battle_attack_won.unwrap_or(0)
    }

    // optional int32 battle_attack_total = 15;

    pub fn clear_battle_attack_total(&mut self) {
        self.battle_attack_total = ::std::option::Option::None;
    }

    pub fn has_battle_attack_total(&self) -> bool {
        self.battle_attack_total.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battle_attack_total(&mut self, v: i32) {
        self.battle_attack_total = ::std::option::Option::Some(v);
    }

    pub fn get_battle_attack_total(&self) -> i32 {
        self.battle_attack_total.unwrap_or(0)
    }

    // optional int32 battle_defended_won = 16;

    pub fn clear_battle_defended_won(&mut self) {
        self.battle_defended_won = ::std::option::Option::None;
    }

    pub fn has_battle_defended_won(&self) -> bool {
        self.battle_defended_won.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battle_defended_won(&mut self, v: i32) {
        self.battle_defended_won = ::std::option::Option::Some(v);
    }

    pub fn get_battle_defended_won(&self) -> i32 {
        self.battle_defended_won.unwrap_or(0)
    }

    // optional int32 battle_training_won = 17;

    pub fn clear_battle_training_won(&mut self) {
        self.battle_training_won = ::std::option::Option::None;
    }

    pub fn has_battle_training_won(&self) -> bool {
        self.battle_training_won.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battle_training_won(&mut self, v: i32) {
        self.battle_training_won = ::std::option::Option::Some(v);
    }

    pub fn get_battle_training_won(&self) -> i32 {
        self.battle_training_won.unwrap_or(0)
    }

    // optional int32 battle_training_total = 18;

    pub fn clear_battle_training_total(&mut self) {
        self.battle_training_total = ::std::option::Option::None;
    }

    pub fn has_battle_training_total(&self) -> bool {
        self.battle_training_total.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battle_training_total(&mut self, v: i32) {
        self.battle_training_total = ::std::option::Option::Some(v);
    }

    pub fn get_battle_training_total(&self) -> i32 {
        self.battle_training_total.unwrap_or(0)
    }

    // optional int32 prestige_raised_total = 19;

    pub fn clear_prestige_raised_total(&mut self) {
        self.prestige_raised_total = ::std::option::Option::None;
    }

    pub fn has_prestige_raised_total(&self) -> bool {
        self.prestige_raised_total.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prestige_raised_total(&mut self, v: i32) {
        self.prestige_raised_total = ::std::option::Option::Some(v);
    }

    pub fn get_prestige_raised_total(&self) -> i32 {
        self.prestige_raised_total.unwrap_or(0)
    }

    // optional int32 prestige_dropped_total = 20;

    pub fn clear_prestige_dropped_total(&mut self) {
        self.prestige_dropped_total = ::std::option::Option::None;
    }

    pub fn has_prestige_dropped_total(&self) -> bool {
        self.prestige_dropped_total.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prestige_dropped_total(&mut self, v: i32) {
        self.prestige_dropped_total = ::std::option::Option::Some(v);
    }

    pub fn get_prestige_dropped_total(&self) -> i32 {
        self.prestige_dropped_total.unwrap_or(0)
    }

    // optional int32 pokemon_deployed = 21;

    pub fn clear_pokemon_deployed(&mut self) {
        self.pokemon_deployed = ::std::option::Option::None;
    }

    pub fn has_pokemon_deployed(&self) -> bool {
        self.pokemon_deployed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_deployed(&mut self, v: i32) {
        self.pokemon_deployed = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_deployed(&self) -> i32 {
        self.pokemon_deployed.unwrap_or(0)
    }

    // repeated int32 pokemon_caught_by_type = 22;

    pub fn clear_pokemon_caught_by_type(&mut self) {
        self.pokemon_caught_by_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_pokemon_caught_by_type(&mut self, v: ::std::vec::Vec<i32>) {
        self.pokemon_caught_by_type = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pokemon_caught_by_type(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.pokemon_caught_by_type
    }

    // Take field
    pub fn take_pokemon_caught_by_type(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.pokemon_caught_by_type, ::std::vec::Vec::new())
    }

    pub fn get_pokemon_caught_by_type(&self) -> &[i32] {
        &self.pokemon_caught_by_type
    }

    // optional int32 small_rattata_caught = 23;

    pub fn clear_small_rattata_caught(&mut self) {
        self.small_rattata_caught = ::std::option::Option::None;
    }

    pub fn has_small_rattata_caught(&self) -> bool {
        self.small_rattata_caught.is_some()
    }

    // Param is passed by value, moved
    pub fn set_small_rattata_caught(&mut self, v: i32) {
        self.small_rattata_caught = ::std::option::Option::Some(v);
    }

    pub fn get_small_rattata_caught(&self) -> i32 {
        self.small_rattata_caught.unwrap_or(0)
    }
}

impl ::protobuf::Message for PlayerStats {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.level = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.experience = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.prev_level_xp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.next_level_xp = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.km_walked = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.pokemons_encountered = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.unique_pokedex_entries = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.pokemons_captured = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.evolutions = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.poke_stop_visits = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.pokeballs_thrown = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.eggs_hatched = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.big_magikarp_caught = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.battle_attack_won = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.battle_attack_total = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.battle_defended_won = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.battle_training_won = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.battle_training_total = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.prestige_raised_total = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.prestige_dropped_total = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.pokemon_deployed = ::std::option::Option::Some(tmp);
                },
                22 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.pokemon_caught_by_type));
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.small_rattata_caught = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.level {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.experience {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.prev_level_xp {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.next_level_xp {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.km_walked.is_some() {
            my_size += 5;
        };
        for value in &self.pokemons_encountered {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.unique_pokedex_entries {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pokemons_captured {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.evolutions {
            my_size += ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.poke_stop_visits {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pokeballs_thrown {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.eggs_hatched {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.big_magikarp_caught {
            my_size += ::protobuf::rt::value_size(13, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.battle_attack_won {
            my_size += ::protobuf::rt::value_size(14, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.battle_attack_total {
            my_size += ::protobuf::rt::value_size(15, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.battle_defended_won {
            my_size += ::protobuf::rt::value_size(16, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.battle_training_won {
            my_size += ::protobuf::rt::value_size(17, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.battle_training_total {
            my_size += ::protobuf::rt::value_size(18, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.prestige_raised_total {
            my_size += ::protobuf::rt::value_size(19, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.prestige_dropped_total {
            my_size += ::protobuf::rt::value_size(20, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pokemon_deployed {
            my_size += ::protobuf::rt::value_size(21, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pokemon_caught_by_type {
            my_size += ::protobuf::rt::value_size(22, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.small_rattata_caught {
            my_size += ::protobuf::rt::value_size(23, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.level {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.experience {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.prev_level_xp {
            try!(os.write_int64(3, v));
        };
        if let Some(v) = self.next_level_xp {
            try!(os.write_int64(4, v));
        };
        if let Some(v) = self.km_walked {
            try!(os.write_float(5, v));
        };
        if let Some(v) = self.pokemons_encountered {
            try!(os.write_int32(6, v));
        };
        if let Some(v) = self.unique_pokedex_entries {
            try!(os.write_int32(7, v));
        };
        if let Some(v) = self.pokemons_captured {
            try!(os.write_int32(8, v));
        };
        if let Some(v) = self.evolutions {
            try!(os.write_int32(9, v));
        };
        if let Some(v) = self.poke_stop_visits {
            try!(os.write_int32(10, v));
        };
        if let Some(v) = self.pokeballs_thrown {
            try!(os.write_int32(11, v));
        };
        if let Some(v) = self.eggs_hatched {
            try!(os.write_int32(12, v));
        };
        if let Some(v) = self.big_magikarp_caught {
            try!(os.write_int32(13, v));
        };
        if let Some(v) = self.battle_attack_won {
            try!(os.write_int32(14, v));
        };
        if let Some(v) = self.battle_attack_total {
            try!(os.write_int32(15, v));
        };
        if let Some(v) = self.battle_defended_won {
            try!(os.write_int32(16, v));
        };
        if let Some(v) = self.battle_training_won {
            try!(os.write_int32(17, v));
        };
        if let Some(v) = self.battle_training_total {
            try!(os.write_int32(18, v));
        };
        if let Some(v) = self.prestige_raised_total {
            try!(os.write_int32(19, v));
        };
        if let Some(v) = self.prestige_dropped_total {
            try!(os.write_int32(20, v));
        };
        if let Some(v) = self.pokemon_deployed {
            try!(os.write_int32(21, v));
        };
        for v in &self.pokemon_caught_by_type {
            try!(os.write_int32(22, *v));
        };
        if let Some(v) = self.small_rattata_caught {
            try!(os.write_int32(23, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PlayerStats>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PlayerStats {
    fn new() -> PlayerStats {
        PlayerStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayerStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "level",
                    PlayerStats::has_level,
                    PlayerStats::get_level,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "experience",
                    PlayerStats::has_experience,
                    PlayerStats::get_experience,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "prev_level_xp",
                    PlayerStats::has_prev_level_xp,
                    PlayerStats::get_prev_level_xp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "next_level_xp",
                    PlayerStats::has_next_level_xp,
                    PlayerStats::get_next_level_xp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "km_walked",
                    PlayerStats::has_km_walked,
                    PlayerStats::get_km_walked,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "pokemons_encountered",
                    PlayerStats::has_pokemons_encountered,
                    PlayerStats::get_pokemons_encountered,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "unique_pokedex_entries",
                    PlayerStats::has_unique_pokedex_entries,
                    PlayerStats::get_unique_pokedex_entries,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "pokemons_captured",
                    PlayerStats::has_pokemons_captured,
                    PlayerStats::get_pokemons_captured,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "evolutions",
                    PlayerStats::has_evolutions,
                    PlayerStats::get_evolutions,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "poke_stop_visits",
                    PlayerStats::has_poke_stop_visits,
                    PlayerStats::get_poke_stop_visits,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "pokeballs_thrown",
                    PlayerStats::has_pokeballs_thrown,
                    PlayerStats::get_pokeballs_thrown,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "eggs_hatched",
                    PlayerStats::has_eggs_hatched,
                    PlayerStats::get_eggs_hatched,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "big_magikarp_caught",
                    PlayerStats::has_big_magikarp_caught,
                    PlayerStats::get_big_magikarp_caught,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "battle_attack_won",
                    PlayerStats::has_battle_attack_won,
                    PlayerStats::get_battle_attack_won,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "battle_attack_total",
                    PlayerStats::has_battle_attack_total,
                    PlayerStats::get_battle_attack_total,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "battle_defended_won",
                    PlayerStats::has_battle_defended_won,
                    PlayerStats::get_battle_defended_won,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "battle_training_won",
                    PlayerStats::has_battle_training_won,
                    PlayerStats::get_battle_training_won,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "battle_training_total",
                    PlayerStats::has_battle_training_total,
                    PlayerStats::get_battle_training_total,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "prestige_raised_total",
                    PlayerStats::has_prestige_raised_total,
                    PlayerStats::get_prestige_raised_total,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "prestige_dropped_total",
                    PlayerStats::has_prestige_dropped_total,
                    PlayerStats::get_prestige_dropped_total,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "pokemon_deployed",
                    PlayerStats::has_pokemon_deployed,
                    PlayerStats::get_pokemon_deployed,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "pokemon_caught_by_type",
                    PlayerStats::get_pokemon_caught_by_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "small_rattata_caught",
                    PlayerStats::has_small_rattata_caught,
                    PlayerStats::get_small_rattata_caught,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlayerStats>(
                    "PlayerStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayerStats {
    fn clear(&mut self) {
        self.clear_level();
        self.clear_experience();
        self.clear_prev_level_xp();
        self.clear_next_level_xp();
        self.clear_km_walked();
        self.clear_pokemons_encountered();
        self.clear_unique_pokedex_entries();
        self.clear_pokemons_captured();
        self.clear_evolutions();
        self.clear_poke_stop_visits();
        self.clear_pokeballs_thrown();
        self.clear_eggs_hatched();
        self.clear_big_magikarp_caught();
        self.clear_battle_attack_won();
        self.clear_battle_attack_total();
        self.clear_battle_defended_won();
        self.clear_battle_training_won();
        self.clear_battle_training_total();
        self.clear_prestige_raised_total();
        self.clear_prestige_dropped_total();
        self.clear_pokemon_deployed();
        self.clear_pokemon_caught_by_type();
        self.clear_small_rattata_caught();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PlayerStats {
    fn eq(&self, other: &PlayerStats) -> bool {
        self.level == other.level &&
        self.experience == other.experience &&
        self.prev_level_xp == other.prev_level_xp &&
        self.next_level_xp == other.next_level_xp &&
        self.km_walked == other.km_walked &&
        self.pokemons_encountered == other.pokemons_encountered &&
        self.unique_pokedex_entries == other.unique_pokedex_entries &&
        self.pokemons_captured == other.pokemons_captured &&
        self.evolutions == other.evolutions &&
        self.poke_stop_visits == other.poke_stop_visits &&
        self.pokeballs_thrown == other.pokeballs_thrown &&
        self.eggs_hatched == other.eggs_hatched &&
        self.big_magikarp_caught == other.big_magikarp_caught &&
        self.battle_attack_won == other.battle_attack_won &&
        self.battle_attack_total == other.battle_attack_total &&
        self.battle_defended_won == other.battle_defended_won &&
        self.battle_training_won == other.battle_training_won &&
        self.battle_training_total == other.battle_training_total &&
        self.prestige_raised_total == other.prestige_raised_total &&
        self.prestige_dropped_total == other.prestige_dropped_total &&
        self.pokemon_deployed == other.pokemon_deployed &&
        self.pokemon_caught_by_type == other.pokemon_caught_by_type &&
        self.small_rattata_caught == other.small_rattata_caught &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PlayerStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EquippedBadge {
    // message fields
    badge_type: ::std::option::Option<super::POGOProtos_Enums::BadgeType>,
    level: ::std::option::Option<i32>,
    next_equip_change_allowed_timestamp_ms: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EquippedBadge {}

impl EquippedBadge {
    pub fn new() -> EquippedBadge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EquippedBadge {
        static mut instance: ::protobuf::lazy::Lazy<EquippedBadge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EquippedBadge,
        };
        unsafe {
            instance.get(|| {
                EquippedBadge {
                    badge_type: ::std::option::Option::None,
                    level: ::std::option::Option::None,
                    next_equip_change_allowed_timestamp_ms: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Enums.BadgeType badge_type = 1;

    pub fn clear_badge_type(&mut self) {
        self.badge_type = ::std::option::Option::None;
    }

    pub fn has_badge_type(&self) -> bool {
        self.badge_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_badge_type(&mut self, v: super::POGOProtos_Enums::BadgeType) {
        self.badge_type = ::std::option::Option::Some(v);
    }

    pub fn get_badge_type(&self) -> super::POGOProtos_Enums::BadgeType {
        self.badge_type.unwrap_or(super::POGOProtos_Enums::BadgeType::BADGE_UNSET)
    }

    // optional int32 level = 2;

    pub fn clear_level(&mut self) {
        self.level = ::std::option::Option::None;
    }

    pub fn has_level(&self) -> bool {
        self.level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_level(&mut self, v: i32) {
        self.level = ::std::option::Option::Some(v);
    }

    pub fn get_level(&self) -> i32 {
        self.level.unwrap_or(0)
    }

    // optional int64 next_equip_change_allowed_timestamp_ms = 3;

    pub fn clear_next_equip_change_allowed_timestamp_ms(&mut self) {
        self.next_equip_change_allowed_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_next_equip_change_allowed_timestamp_ms(&self) -> bool {
        self.next_equip_change_allowed_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_next_equip_change_allowed_timestamp_ms(&mut self, v: i64) {
        self.next_equip_change_allowed_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_next_equip_change_allowed_timestamp_ms(&self) -> i64 {
        self.next_equip_change_allowed_timestamp_ms.unwrap_or(0)
    }
}

impl ::protobuf::Message for EquippedBadge {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.badge_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.level = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.next_equip_change_allowed_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.badge_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.level {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.next_equip_change_allowed_timestamp_ms {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.badge_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.level {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.next_equip_change_allowed_timestamp_ms {
            try!(os.write_int64(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<EquippedBadge>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EquippedBadge {
    fn new() -> EquippedBadge {
        EquippedBadge::new()
    }

    fn descriptor_static(_: ::std::option::Option<EquippedBadge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "badge_type",
                    EquippedBadge::has_badge_type,
                    EquippedBadge::get_badge_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "level",
                    EquippedBadge::has_level,
                    EquippedBadge::get_level,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "next_equip_change_allowed_timestamp_ms",
                    EquippedBadge::has_next_equip_change_allowed_timestamp_ms,
                    EquippedBadge::get_next_equip_change_allowed_timestamp_ms,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EquippedBadge>(
                    "EquippedBadge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EquippedBadge {
    fn clear(&mut self) {
        self.clear_badge_type();
        self.clear_level();
        self.clear_next_equip_change_allowed_timestamp_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EquippedBadge {
    fn eq(&self, other: &EquippedBadge) -> bool {
        self.badge_type == other.badge_type &&
        self.level == other.level &&
        self.next_equip_change_allowed_timestamp_ms == other.next_equip_change_allowed_timestamp_ms &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EquippedBadge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PlayerPublicProfile {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    level: ::std::option::Option<i32>,
    avatar: ::protobuf::SingularPtrField<PlayerAvatar>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayerPublicProfile {}

impl PlayerPublicProfile {
    pub fn new() -> PlayerPublicProfile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayerPublicProfile {
        static mut instance: ::protobuf::lazy::Lazy<PlayerPublicProfile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayerPublicProfile,
        };
        unsafe {
            instance.get(|| {
                PlayerPublicProfile {
                    name: ::protobuf::SingularField::none(),
                    level: ::std::option::Option::None,
                    avatar: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 level = 2;

    pub fn clear_level(&mut self) {
        self.level = ::std::option::Option::None;
    }

    pub fn has_level(&self) -> bool {
        self.level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_level(&mut self, v: i32) {
        self.level = ::std::option::Option::Some(v);
    }

    pub fn get_level(&self) -> i32 {
        self.level.unwrap_or(0)
    }

    // optional .POGOProtos.Data.Player.PlayerAvatar avatar = 3;

    pub fn clear_avatar(&mut self) {
        self.avatar.clear();
    }

    pub fn has_avatar(&self) -> bool {
        self.avatar.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avatar(&mut self, v: PlayerAvatar) {
        self.avatar = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_avatar(&mut self) -> &mut PlayerAvatar {
        if self.avatar.is_none() {
            self.avatar.set_default();
        };
        self.avatar.as_mut().unwrap()
    }

    // Take field
    pub fn take_avatar(&mut self) -> PlayerAvatar {
        self.avatar.take().unwrap_or_else(|| PlayerAvatar::new())
    }

    pub fn get_avatar(&self) -> &PlayerAvatar {
        self.avatar.as_ref().unwrap_or_else(|| PlayerAvatar::default_instance())
    }
}

impl ::protobuf::Message for PlayerPublicProfile {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.level = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.avatar));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.level {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.avatar {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.level {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.avatar.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PlayerPublicProfile>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PlayerPublicProfile {
    fn new() -> PlayerPublicProfile {
        PlayerPublicProfile::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayerPublicProfile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    PlayerPublicProfile::has_name,
                    PlayerPublicProfile::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "level",
                    PlayerPublicProfile::has_level,
                    PlayerPublicProfile::get_level,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "avatar",
                    PlayerPublicProfile::has_avatar,
                    PlayerPublicProfile::get_avatar,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlayerPublicProfile>(
                    "PlayerPublicProfile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayerPublicProfile {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_level();
        self.clear_avatar();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PlayerPublicProfile {
    fn eq(&self, other: &PlayerPublicProfile) -> bool {
        self.name == other.name &&
        self.level == other.level &&
        self.avatar == other.avatar &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PlayerPublicProfile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PlayerCamera {
    // message fields
    is_default_camera: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayerCamera {}

impl PlayerCamera {
    pub fn new() -> PlayerCamera {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayerCamera {
        static mut instance: ::protobuf::lazy::Lazy<PlayerCamera> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayerCamera,
        };
        unsafe {
            instance.get(|| {
                PlayerCamera {
                    is_default_camera: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool is_default_camera = 1;

    pub fn clear_is_default_camera(&mut self) {
        self.is_default_camera = ::std::option::Option::None;
    }

    pub fn has_is_default_camera(&self) -> bool {
        self.is_default_camera.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_default_camera(&mut self, v: bool) {
        self.is_default_camera = ::std::option::Option::Some(v);
    }

    pub fn get_is_default_camera(&self) -> bool {
        self.is_default_camera.unwrap_or(false)
    }
}

impl ::protobuf::Message for PlayerCamera {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_default_camera = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.is_default_camera.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.is_default_camera {
            try!(os.write_bool(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PlayerCamera>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PlayerCamera {
    fn new() -> PlayerCamera {
        PlayerCamera::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayerCamera>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_default_camera",
                    PlayerCamera::has_is_default_camera,
                    PlayerCamera::get_is_default_camera,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlayerCamera>(
                    "PlayerCamera",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayerCamera {
    fn clear(&mut self) {
        self.clear_is_default_camera();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PlayerCamera {
    fn eq(&self, other: &PlayerCamera) -> bool {
        self.is_default_camera == other.is_default_camera &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PlayerCamera {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PlayerAvatar {
    // message fields
    skin: ::std::option::Option<i32>,
    hair: ::std::option::Option<i32>,
    shirt: ::std::option::Option<i32>,
    pants: ::std::option::Option<i32>,
    hat: ::std::option::Option<i32>,
    shoes: ::std::option::Option<i32>,
    gender: ::std::option::Option<super::POGOProtos_Enums::Gender>,
    eyes: ::std::option::Option<i32>,
    backpack: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayerAvatar {}

impl PlayerAvatar {
    pub fn new() -> PlayerAvatar {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayerAvatar {
        static mut instance: ::protobuf::lazy::Lazy<PlayerAvatar> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayerAvatar,
        };
        unsafe {
            instance.get(|| {
                PlayerAvatar {
                    skin: ::std::option::Option::None,
                    hair: ::std::option::Option::None,
                    shirt: ::std::option::Option::None,
                    pants: ::std::option::Option::None,
                    hat: ::std::option::Option::None,
                    shoes: ::std::option::Option::None,
                    gender: ::std::option::Option::None,
                    eyes: ::std::option::Option::None,
                    backpack: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 skin = 2;

    pub fn clear_skin(&mut self) {
        self.skin = ::std::option::Option::None;
    }

    pub fn has_skin(&self) -> bool {
        self.skin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_skin(&mut self, v: i32) {
        self.skin = ::std::option::Option::Some(v);
    }

    pub fn get_skin(&self) -> i32 {
        self.skin.unwrap_or(0)
    }

    // optional int32 hair = 3;

    pub fn clear_hair(&mut self) {
        self.hair = ::std::option::Option::None;
    }

    pub fn has_hair(&self) -> bool {
        self.hair.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hair(&mut self, v: i32) {
        self.hair = ::std::option::Option::Some(v);
    }

    pub fn get_hair(&self) -> i32 {
        self.hair.unwrap_or(0)
    }

    // optional int32 shirt = 4;

    pub fn clear_shirt(&mut self) {
        self.shirt = ::std::option::Option::None;
    }

    pub fn has_shirt(&self) -> bool {
        self.shirt.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shirt(&mut self, v: i32) {
        self.shirt = ::std::option::Option::Some(v);
    }

    pub fn get_shirt(&self) -> i32 {
        self.shirt.unwrap_or(0)
    }

    // optional int32 pants = 5;

    pub fn clear_pants(&mut self) {
        self.pants = ::std::option::Option::None;
    }

    pub fn has_pants(&self) -> bool {
        self.pants.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pants(&mut self, v: i32) {
        self.pants = ::std::option::Option::Some(v);
    }

    pub fn get_pants(&self) -> i32 {
        self.pants.unwrap_or(0)
    }

    // optional int32 hat = 6;

    pub fn clear_hat(&mut self) {
        self.hat = ::std::option::Option::None;
    }

    pub fn has_hat(&self) -> bool {
        self.hat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hat(&mut self, v: i32) {
        self.hat = ::std::option::Option::Some(v);
    }

    pub fn get_hat(&self) -> i32 {
        self.hat.unwrap_or(0)
    }

    // optional int32 shoes = 7;

    pub fn clear_shoes(&mut self) {
        self.shoes = ::std::option::Option::None;
    }

    pub fn has_shoes(&self) -> bool {
        self.shoes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shoes(&mut self, v: i32) {
        self.shoes = ::std::option::Option::Some(v);
    }

    pub fn get_shoes(&self) -> i32 {
        self.shoes.unwrap_or(0)
    }

    // optional .POGOProtos.Enums.Gender gender = 8;

    pub fn clear_gender(&mut self) {
        self.gender = ::std::option::Option::None;
    }

    pub fn has_gender(&self) -> bool {
        self.gender.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gender(&mut self, v: super::POGOProtos_Enums::Gender) {
        self.gender = ::std::option::Option::Some(v);
    }

    pub fn get_gender(&self) -> super::POGOProtos_Enums::Gender {
        self.gender.unwrap_or(super::POGOProtos_Enums::Gender::MALE)
    }

    // optional int32 eyes = 9;

    pub fn clear_eyes(&mut self) {
        self.eyes = ::std::option::Option::None;
    }

    pub fn has_eyes(&self) -> bool {
        self.eyes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eyes(&mut self, v: i32) {
        self.eyes = ::std::option::Option::Some(v);
    }

    pub fn get_eyes(&self) -> i32 {
        self.eyes.unwrap_or(0)
    }

    // optional int32 backpack = 10;

    pub fn clear_backpack(&mut self) {
        self.backpack = ::std::option::Option::None;
    }

    pub fn has_backpack(&self) -> bool {
        self.backpack.is_some()
    }

    // Param is passed by value, moved
    pub fn set_backpack(&mut self, v: i32) {
        self.backpack = ::std::option::Option::Some(v);
    }

    pub fn get_backpack(&self) -> i32 {
        self.backpack.unwrap_or(0)
    }
}

impl ::protobuf::Message for PlayerAvatar {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.skin = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.hair = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.shirt = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.pants = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.hat = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.shoes = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.gender = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.eyes = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.backpack = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.skin {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.hair {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.shirt {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pants {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.hat {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.shoes {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.gender {
            my_size += ::protobuf::rt::enum_size(8, *value);
        };
        for value in &self.eyes {
            my_size += ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.backpack {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.skin {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.hair {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.shirt {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.pants {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.hat {
            try!(os.write_int32(6, v));
        };
        if let Some(v) = self.shoes {
            try!(os.write_int32(7, v));
        };
        if let Some(v) = self.gender {
            try!(os.write_enum(8, v.value()));
        };
        if let Some(v) = self.eyes {
            try!(os.write_int32(9, v));
        };
        if let Some(v) = self.backpack {
            try!(os.write_int32(10, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PlayerAvatar>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PlayerAvatar {
    fn new() -> PlayerAvatar {
        PlayerAvatar::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayerAvatar>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "skin",
                    PlayerAvatar::has_skin,
                    PlayerAvatar::get_skin,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "hair",
                    PlayerAvatar::has_hair,
                    PlayerAvatar::get_hair,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "shirt",
                    PlayerAvatar::has_shirt,
                    PlayerAvatar::get_shirt,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "pants",
                    PlayerAvatar::has_pants,
                    PlayerAvatar::get_pants,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "hat",
                    PlayerAvatar::has_hat,
                    PlayerAvatar::get_hat,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "shoes",
                    PlayerAvatar::has_shoes,
                    PlayerAvatar::get_shoes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "gender",
                    PlayerAvatar::has_gender,
                    PlayerAvatar::get_gender,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "eyes",
                    PlayerAvatar::has_eyes,
                    PlayerAvatar::get_eyes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "backpack",
                    PlayerAvatar::has_backpack,
                    PlayerAvatar::get_backpack,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlayerAvatar>(
                    "PlayerAvatar",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayerAvatar {
    fn clear(&mut self) {
        self.clear_skin();
        self.clear_hair();
        self.clear_shirt();
        self.clear_pants();
        self.clear_hat();
        self.clear_shoes();
        self.clear_gender();
        self.clear_eyes();
        self.clear_backpack();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PlayerAvatar {
    fn eq(&self, other: &PlayerAvatar) -> bool {
        self.skin == other.skin &&
        self.hair == other.hair &&
        self.shirt == other.shirt &&
        self.pants == other.pants &&
        self.hat == other.hat &&
        self.shoes == other.shoes &&
        self.gender == other.gender &&
        self.eyes == other.eyes &&
        self.backpack == other.backpack &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PlayerAvatar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ContactSettings {
    // message fields
    send_marketing_emails: ::std::option::Option<bool>,
    send_push_notifications: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ContactSettings {}

impl ContactSettings {
    pub fn new() -> ContactSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ContactSettings {
        static mut instance: ::protobuf::lazy::Lazy<ContactSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContactSettings,
        };
        unsafe {
            instance.get(|| {
                ContactSettings {
                    send_marketing_emails: ::std::option::Option::None,
                    send_push_notifications: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool send_marketing_emails = 1;

    pub fn clear_send_marketing_emails(&mut self) {
        self.send_marketing_emails = ::std::option::Option::None;
    }

    pub fn has_send_marketing_emails(&self) -> bool {
        self.send_marketing_emails.is_some()
    }

    // Param is passed by value, moved
    pub fn set_send_marketing_emails(&mut self, v: bool) {
        self.send_marketing_emails = ::std::option::Option::Some(v);
    }

    pub fn get_send_marketing_emails(&self) -> bool {
        self.send_marketing_emails.unwrap_or(false)
    }

    // optional bool send_push_notifications = 2;

    pub fn clear_send_push_notifications(&mut self) {
        self.send_push_notifications = ::std::option::Option::None;
    }

    pub fn has_send_push_notifications(&self) -> bool {
        self.send_push_notifications.is_some()
    }

    // Param is passed by value, moved
    pub fn set_send_push_notifications(&mut self, v: bool) {
        self.send_push_notifications = ::std::option::Option::Some(v);
    }

    pub fn get_send_push_notifications(&self) -> bool {
        self.send_push_notifications.unwrap_or(false)
    }
}

impl ::protobuf::Message for ContactSettings {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.send_marketing_emails = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.send_push_notifications = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.send_marketing_emails.is_some() {
            my_size += 2;
        };
        if self.send_push_notifications.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.send_marketing_emails {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.send_push_notifications {
            try!(os.write_bool(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ContactSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ContactSettings {
    fn new() -> ContactSettings {
        ContactSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<ContactSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "send_marketing_emails",
                    ContactSettings::has_send_marketing_emails,
                    ContactSettings::get_send_marketing_emails,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "send_push_notifications",
                    ContactSettings::has_send_push_notifications,
                    ContactSettings::get_send_push_notifications,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ContactSettings>(
                    "ContactSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ContactSettings {
    fn clear(&mut self) {
        self.clear_send_marketing_emails();
        self.clear_send_push_notifications();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ContactSettings {
    fn eq(&self, other: &ContactSettings) -> bool {
        self.send_marketing_emails == other.send_marketing_emails &&
        self.send_push_notifications == other.send_push_notifications &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ContactSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Currency {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    amount: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Currency {}

impl Currency {
    pub fn new() -> Currency {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Currency {
        static mut instance: ::protobuf::lazy::Lazy<Currency> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Currency,
        };
        unsafe {
            instance.get(|| {
                Currency {
                    name: ::protobuf::SingularField::none(),
                    amount: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 amount = 2;

    pub fn clear_amount(&mut self) {
        self.amount = ::std::option::Option::None;
    }

    pub fn has_amount(&self) -> bool {
        self.amount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: i32) {
        self.amount = ::std::option::Option::Some(v);
    }

    pub fn get_amount(&self) -> i32 {
        self.amount.unwrap_or(0)
    }
}

impl ::protobuf::Message for Currency {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.amount = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.amount {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.amount {
            try!(os.write_int32(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Currency>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Currency {
    fn new() -> Currency {
        Currency::new()
    }

    fn descriptor_static(_: ::std::option::Option<Currency>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Currency::has_name,
                    Currency::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "amount",
                    Currency::has_amount,
                    Currency::get_amount,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Currency>(
                    "Currency",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Currency {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_amount();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Currency {
    fn eq(&self, other: &Currency) -> bool {
        self.name == other.name &&
        self.amount == other.amount &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Currency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DailyBonus {
    // message fields
    next_collected_timestamp_ms: ::std::option::Option<i64>,
    next_defender_bonus_collect_timestamp_ms: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DailyBonus {}

impl DailyBonus {
    pub fn new() -> DailyBonus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DailyBonus {
        static mut instance: ::protobuf::lazy::Lazy<DailyBonus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DailyBonus,
        };
        unsafe {
            instance.get(|| {
                DailyBonus {
                    next_collected_timestamp_ms: ::std::option::Option::None,
                    next_defender_bonus_collect_timestamp_ms: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 next_collected_timestamp_ms = 1;

    pub fn clear_next_collected_timestamp_ms(&mut self) {
        self.next_collected_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_next_collected_timestamp_ms(&self) -> bool {
        self.next_collected_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_next_collected_timestamp_ms(&mut self, v: i64) {
        self.next_collected_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_next_collected_timestamp_ms(&self) -> i64 {
        self.next_collected_timestamp_ms.unwrap_or(0)
    }

    // optional int64 next_defender_bonus_collect_timestamp_ms = 2;

    pub fn clear_next_defender_bonus_collect_timestamp_ms(&mut self) {
        self.next_defender_bonus_collect_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_next_defender_bonus_collect_timestamp_ms(&self) -> bool {
        self.next_defender_bonus_collect_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_next_defender_bonus_collect_timestamp_ms(&mut self, v: i64) {
        self.next_defender_bonus_collect_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_next_defender_bonus_collect_timestamp_ms(&self) -> i64 {
        self.next_defender_bonus_collect_timestamp_ms.unwrap_or(0)
    }
}

impl ::protobuf::Message for DailyBonus {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.next_collected_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.next_defender_bonus_collect_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.next_collected_timestamp_ms {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.next_defender_bonus_collect_timestamp_ms {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.next_collected_timestamp_ms {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.next_defender_bonus_collect_timestamp_ms {
            try!(os.write_int64(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DailyBonus>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DailyBonus {
    fn new() -> DailyBonus {
        DailyBonus::new()
    }

    fn descriptor_static(_: ::std::option::Option<DailyBonus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "next_collected_timestamp_ms",
                    DailyBonus::has_next_collected_timestamp_ms,
                    DailyBonus::get_next_collected_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "next_defender_bonus_collect_timestamp_ms",
                    DailyBonus::has_next_defender_bonus_collect_timestamp_ms,
                    DailyBonus::get_next_defender_bonus_collect_timestamp_ms,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DailyBonus>(
                    "DailyBonus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DailyBonus {
    fn clear(&mut self) {
        self.clear_next_collected_timestamp_ms();
        self.clear_next_defender_bonus_collect_timestamp_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DailyBonus {
    fn eq(&self, other: &DailyBonus) -> bool {
        self.next_collected_timestamp_ms == other.next_collected_timestamp_ms &&
        self.next_defender_bonus_collect_timestamp_ms == other.next_defender_bonus_collect_timestamp_ms &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DailyBonus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PlayerCurrency {
    // message fields
    gems: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayerCurrency {}

impl PlayerCurrency {
    pub fn new() -> PlayerCurrency {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayerCurrency {
        static mut instance: ::protobuf::lazy::Lazy<PlayerCurrency> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayerCurrency,
        };
        unsafe {
            instance.get(|| {
                PlayerCurrency {
                    gems: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 gems = 1;

    pub fn clear_gems(&mut self) {
        self.gems = ::std::option::Option::None;
    }

    pub fn has_gems(&self) -> bool {
        self.gems.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gems(&mut self, v: i32) {
        self.gems = ::std::option::Option::Some(v);
    }

    pub fn get_gems(&self) -> i32 {
        self.gems.unwrap_or(0)
    }
}

impl ::protobuf::Message for PlayerCurrency {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.gems = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.gems {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gems {
            try!(os.write_int32(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PlayerCurrency>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PlayerCurrency {
    fn new() -> PlayerCurrency {
        PlayerCurrency::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayerCurrency>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "gems",
                    PlayerCurrency::has_gems,
                    PlayerCurrency::get_gems,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlayerCurrency>(
                    "PlayerCurrency",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayerCurrency {
    fn clear(&mut self) {
        self.clear_gems();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PlayerCurrency {
    fn eq(&self, other: &PlayerCurrency) -> bool {
        self.gems == other.gems &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PlayerCurrency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1c, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x16,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e,
    0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x1a, 0x16, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x00,
    0x22, 0xf2, 0x07, 0x0a, 0x0b, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x73,
    0x12, 0x14, 0x0a, 0x05, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x05, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x1e, 0x0a, 0x0a, 0x65, 0x78, 0x70, 0x65, 0x72, 0x69,
    0x65, 0x6e, 0x63, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0a, 0x65, 0x78, 0x70, 0x65,
    0x72, 0x69, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x22, 0x0a, 0x0d, 0x70, 0x72, 0x65, 0x76, 0x5f, 0x6c,
    0x65, 0x76, 0x65, 0x6c, 0x5f, 0x78, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0b, 0x70,
    0x72, 0x65, 0x76, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x58, 0x70, 0x12, 0x22, 0x0a, 0x0d, 0x6e, 0x65,
    0x78, 0x74, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x5f, 0x78, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x03, 0x52, 0x0b, 0x6e, 0x65, 0x78, 0x74, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x58, 0x70, 0x12, 0x1b,
    0x0a, 0x09, 0x6b, 0x6d, 0x5f, 0x77, 0x61, 0x6c, 0x6b, 0x65, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x02, 0x52, 0x08, 0x6b, 0x6d, 0x57, 0x61, 0x6c, 0x6b, 0x65, 0x64, 0x12, 0x31, 0x0a, 0x14, 0x70,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x73, 0x5f, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65,
    0x72, 0x65, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52, 0x13, 0x70, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x73, 0x45, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x64, 0x12, 0x34,
    0x0a, 0x16, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x64, 0x65, 0x78,
    0x5f, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x52, 0x14,
    0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x64, 0x65, 0x78, 0x45, 0x6e, 0x74,
    0x72, 0x69, 0x65, 0x73, 0x12, 0x2b, 0x0a, 0x11, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x73,
    0x5f, 0x63, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x10, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x73, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65,
    0x64, 0x12, 0x1e, 0x0a, 0x0a, 0x65, 0x76, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18,
    0x09, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x65, 0x76, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x12, 0x28, 0x0a, 0x10, 0x70, 0x6f, 0x6b, 0x65, 0x5f, 0x73, 0x74, 0x6f, 0x70, 0x5f, 0x76,
    0x69, 0x73, 0x69, 0x74, 0x73, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0e, 0x70, 0x6f, 0x6b,
    0x65, 0x53, 0x74, 0x6f, 0x70, 0x56, 0x69, 0x73, 0x69, 0x74, 0x73, 0x12, 0x29, 0x0a, 0x10, 0x70,
    0x6f, 0x6b, 0x65, 0x62, 0x61, 0x6c, 0x6c, 0x73, 0x5f, 0x74, 0x68, 0x72, 0x6f, 0x77, 0x6e, 0x18,
    0x0b, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0f, 0x70, 0x6f, 0x6b, 0x65, 0x62, 0x61, 0x6c, 0x6c, 0x73,
    0x54, 0x68, 0x72, 0x6f, 0x77, 0x6e, 0x12, 0x21, 0x0a, 0x0c, 0x65, 0x67, 0x67, 0x73, 0x5f, 0x68,
    0x61, 0x74, 0x63, 0x68, 0x65, 0x64, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x65, 0x67,
    0x67, 0x73, 0x48, 0x61, 0x74, 0x63, 0x68, 0x65, 0x64, 0x12, 0x2e, 0x0a, 0x13, 0x62, 0x69, 0x67,
    0x5f, 0x6d, 0x61, 0x67, 0x69, 0x6b, 0x61, 0x72, 0x70, 0x5f, 0x63, 0x61, 0x75, 0x67, 0x68, 0x74,
    0x18, 0x0d, 0x20, 0x01, 0x28, 0x05, 0x52, 0x11, 0x62, 0x69, 0x67, 0x4d, 0x61, 0x67, 0x69, 0x6b,
    0x61, 0x72, 0x70, 0x43, 0x61, 0x75, 0x67, 0x68, 0x74, 0x12, 0x2a, 0x0a, 0x11, 0x62, 0x61, 0x74,
    0x74, 0x6c, 0x65, 0x5f, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x5f, 0x77, 0x6f, 0x6e, 0x18, 0x0e,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x0f, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x41, 0x74, 0x74, 0x61,
    0x63, 0x6b, 0x57, 0x6f, 0x6e, 0x12, 0x2e, 0x0a, 0x13, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x5f,
    0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x5f, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x18, 0x0f, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x11, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x41, 0x74, 0x74, 0x61, 0x63, 0x6b,
    0x54, 0x6f, 0x74, 0x61, 0x6c, 0x12, 0x2e, 0x0a, 0x13, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x5f,
    0x64, 0x65, 0x66, 0x65, 0x6e, 0x64, 0x65, 0x64, 0x5f, 0x77, 0x6f, 0x6e, 0x18, 0x10, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x11, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x44, 0x65, 0x66, 0x65, 0x6e, 0x64,
    0x65, 0x64, 0x57, 0x6f, 0x6e, 0x12, 0x2e, 0x0a, 0x13, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x5f,
    0x74, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x5f, 0x77, 0x6f, 0x6e, 0x18, 0x11, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x11, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x69,
    0x6e, 0x67, 0x57, 0x6f, 0x6e, 0x12, 0x32, 0x0a, 0x15, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x5f,
    0x74, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x18, 0x12,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x13, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x54, 0x72, 0x61, 0x69,
    0x6e, 0x69, 0x6e, 0x67, 0x54, 0x6f, 0x74, 0x61, 0x6c, 0x12, 0x32, 0x0a, 0x15, 0x70, 0x72, 0x65,
    0x73, 0x74, 0x69, 0x67, 0x65, 0x5f, 0x72, 0x61, 0x69, 0x73, 0x65, 0x64, 0x5f, 0x74, 0x6f, 0x74,
    0x61, 0x6c, 0x18, 0x13, 0x20, 0x01, 0x28, 0x05, 0x52, 0x13, 0x70, 0x72, 0x65, 0x73, 0x74, 0x69,
    0x67, 0x65, 0x52, 0x61, 0x69, 0x73, 0x65, 0x64, 0x54, 0x6f, 0x74, 0x61, 0x6c, 0x12, 0x34, 0x0a,
    0x16, 0x70, 0x72, 0x65, 0x73, 0x74, 0x69, 0x67, 0x65, 0x5f, 0x64, 0x72, 0x6f, 0x70, 0x70, 0x65,
    0x64, 0x5f, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x18, 0x14, 0x20, 0x01, 0x28, 0x05, 0x52, 0x14, 0x70,
    0x72, 0x65, 0x73, 0x74, 0x69, 0x67, 0x65, 0x44, 0x72, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x54, 0x6f,
    0x74, 0x61, 0x6c, 0x12, 0x29, 0x0a, 0x10, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x64,
    0x65, 0x70, 0x6c, 0x6f, 0x79, 0x65, 0x64, 0x18, 0x15, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0f, 0x70,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x65, 0x70, 0x6c, 0x6f, 0x79, 0x65, 0x64, 0x12, 0x33,
    0x0a, 0x16, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x63, 0x61, 0x75, 0x67, 0x68, 0x74,
    0x5f, 0x62, 0x79, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x16, 0x20, 0x03, 0x28, 0x05, 0x52, 0x13,
    0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x43, 0x61, 0x75, 0x67, 0x68, 0x74, 0x42, 0x79, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x30, 0x0a, 0x14, 0x73, 0x6d, 0x61, 0x6c, 0x6c, 0x5f, 0x72, 0x61, 0x74,
    0x74, 0x61, 0x74, 0x61, 0x5f, 0x63, 0x61, 0x75, 0x67, 0x68, 0x74, 0x18, 0x17, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x12, 0x73, 0x6d, 0x61, 0x6c, 0x6c, 0x52, 0x61, 0x74, 0x74, 0x61, 0x74, 0x61, 0x43,
    0x61, 0x75, 0x67, 0x68, 0x74, 0x22, 0xb4, 0x01, 0x0a, 0x0d, 0x45, 0x71, 0x75, 0x69, 0x70, 0x70,
    0x65, 0x64, 0x42, 0x61, 0x64, 0x67, 0x65, 0x12, 0x3a, 0x0a, 0x0a, 0x62, 0x61, 0x64, 0x67, 0x65,
    0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x42,
    0x61, 0x64, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x09, 0x62, 0x61, 0x64, 0x67, 0x65, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x05, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x51, 0x0a, 0x26, 0x6e, 0x65, 0x78,
    0x74, 0x5f, 0x65, 0x71, 0x75, 0x69, 0x70, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x61,
    0x6c, 0x6c, 0x6f, 0x77, 0x65, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
    0x5f, 0x6d, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x52, 0x21, 0x6e, 0x65, 0x78, 0x74, 0x45,
    0x71, 0x75, 0x69, 0x70, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x41, 0x6c, 0x6c, 0x6f, 0x77, 0x65,
    0x64, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x22, 0x7d, 0x0a, 0x13,
    0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x50, 0x72, 0x6f, 0x66,
    0x69, 0x6c, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x6c, 0x65, 0x76, 0x65, 0x6c,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x05, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x3c, 0x0a,
    0x06, 0x61, 0x76, 0x61, 0x74, 0x61, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e,
    0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x41, 0x76, 0x61,
    0x74, 0x61, 0x72, 0x52, 0x06, 0x61, 0x76, 0x61, 0x74, 0x61, 0x72, 0x22, 0x3a, 0x0a, 0x0c, 0x50,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x12, 0x2a, 0x0a, 0x11, 0x69,
    0x73, 0x5f, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x5f, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0f, 0x69, 0x73, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c,
    0x74, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x22, 0xec, 0x01, 0x0a, 0x0c, 0x50, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x41, 0x76, 0x61, 0x74, 0x61, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x6b, 0x69, 0x6e,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x73, 0x6b, 0x69, 0x6e, 0x12, 0x12, 0x0a, 0x04,
    0x68, 0x61, 0x69, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x68, 0x61, 0x69, 0x72,
    0x12, 0x14, 0x0a, 0x05, 0x73, 0x68, 0x69, 0x72, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x05, 0x73, 0x68, 0x69, 0x72, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x6e, 0x74, 0x73, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x05, 0x52, 0x05, 0x70, 0x61, 0x6e, 0x74, 0x73, 0x12, 0x10, 0x0a, 0x03,
    0x68, 0x61, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52, 0x03, 0x68, 0x61, 0x74, 0x12, 0x14,
    0x0a, 0x05, 0x73, 0x68, 0x6f, 0x65, 0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x52, 0x05, 0x73,
    0x68, 0x6f, 0x65, 0x73, 0x12, 0x30, 0x0a, 0x06, 0x67, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x18, 0x08,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x18, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x47, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x52, 0x06,
    0x67, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x65, 0x79, 0x65, 0x73, 0x18, 0x09,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x65, 0x79, 0x65, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x62, 0x61,
    0x63, 0x6b, 0x70, 0x61, 0x63, 0x6b, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x62, 0x61,
    0x63, 0x6b, 0x70, 0x61, 0x63, 0x6b, 0x22, 0x7d, 0x0a, 0x0f, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x63,
    0x74, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x32, 0x0a, 0x15, 0x73, 0x65, 0x6e,
    0x64, 0x5f, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x65, 0x6d, 0x61, 0x69,
    0x6c, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x13, 0x73, 0x65, 0x6e, 0x64, 0x4d, 0x61,
    0x72, 0x6b, 0x65, 0x74, 0x69, 0x6e, 0x67, 0x45, 0x6d, 0x61, 0x69, 0x6c, 0x73, 0x12, 0x36, 0x0a,
    0x17, 0x73, 0x65, 0x6e, 0x64, 0x5f, 0x70, 0x75, 0x73, 0x68, 0x5f, 0x6e, 0x6f, 0x74, 0x69, 0x66,
    0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x15,
    0x73, 0x65, 0x6e, 0x64, 0x50, 0x75, 0x73, 0x68, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x22, 0x36, 0x0a, 0x08, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63,
    0x79, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x22, 0xa2, 0x01,
    0x0a, 0x0a, 0x44, 0x61, 0x69, 0x6c, 0x79, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x12, 0x3d, 0x0a, 0x1b,
    0x6e, 0x65, 0x78, 0x74, 0x5f, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x5f, 0x74,
    0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x03, 0x52, 0x18, 0x6e, 0x65, 0x78, 0x74, 0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x65, 0x64,
    0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12, 0x55, 0x0a, 0x28, 0x6e,
    0x65, 0x78, 0x74, 0x5f, 0x64, 0x65, 0x66, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x5f, 0x62, 0x6f, 0x6e,
    0x75, 0x73, 0x5f, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x23, 0x6e,
    0x65, 0x78, 0x74, 0x44, 0x65, 0x66, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x42, 0x6f, 0x6e, 0x75, 0x73,
    0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
    0x4d, 0x73, 0x22, 0x24, 0x0a, 0x0e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x63, 0x79, 0x12, 0x12, 0x0a, 0x04, 0x67, 0x65, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x04, 0x67, 0x65, 0x6d, 0x73, 0x4a, 0x9d, 0x1b, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x44, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x1e, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x00, 0x12, 0x03, 0x03,
    0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x0e, 0x26, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x05, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x05, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06,
    0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x06, 0x08, 0x05,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x06, 0x08, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x0e, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x06, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x07, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x07, 0x08, 0x06, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x07, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x07, 0x0e, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x07, 0x1b,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x08, 0x08, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x08, 0x08, 0x07, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x08, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x08, 0x0e, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x08, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x09, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x09,
    0x08, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x09, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x09, 0x0e, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x09, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0a, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x0a, 0x08, 0x09, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x0a, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x0a, 0x0e, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x0a, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0b, 0x08, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x04, 0x0b, 0x08, 0x0a, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0b, 0x08, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0b, 0x0e, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0b, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x06, 0x12, 0x03, 0x0c, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12,
    0x04, 0x0c, 0x08, 0x0b, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03,
    0x0c, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0c, 0x0e,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x0c, 0x27, 0x28, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x0d, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x04, 0x0d, 0x08, 0x0c, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x0d, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x01, 0x12, 0x03, 0x0d, 0x0e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03,
    0x12, 0x03, 0x0d, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x0e,
    0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x04, 0x0e, 0x08, 0x0d,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x0e, 0x08, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0e, 0x0e, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x0e, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x09, 0x12, 0x03, 0x0f, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09,
    0x04, 0x12, 0x04, 0x0f, 0x08, 0x0e, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05,
    0x12, 0x03, 0x0f, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03,
    0x0f, 0x0e, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x0f, 0x21,
    0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x10, 0x08, 0x24, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x10, 0x08, 0x0f, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x10, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x10, 0x0e, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0a, 0x03, 0x12, 0x03, 0x10, 0x21, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12,
    0x03, 0x11, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x04, 0x11,
    0x08, 0x10, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x11, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x11, 0x0e, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x11, 0x1d, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x12, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0c, 0x04, 0x12, 0x04, 0x12, 0x08, 0x11, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0c, 0x05, 0x12, 0x03, 0x12, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x01,
    0x12, 0x03, 0x12, 0x0e, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x03, 0x12, 0x03,
    0x12, 0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x13, 0x08, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x04, 0x12, 0x04, 0x13, 0x08, 0x12, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x13, 0x08, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x13, 0x0e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x13, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x0e, 0x12, 0x03, 0x14, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x04, 0x12,
    0x04, 0x14, 0x08, 0x13, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x05, 0x12, 0x03,
    0x14, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x14, 0x0e,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x14, 0x24, 0x26, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0f, 0x12, 0x03, 0x15, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0f, 0x04, 0x12, 0x04, 0x15, 0x08, 0x14, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x15, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0f, 0x01, 0x12, 0x03, 0x15, 0x0e, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x03,
    0x12, 0x03, 0x15, 0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x10, 0x12, 0x03, 0x16,
    0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x04, 0x12, 0x04, 0x16, 0x08, 0x15,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x05, 0x12, 0x03, 0x16, 0x08, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x01, 0x12, 0x03, 0x16, 0x0e, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x10, 0x03, 0x12, 0x03, 0x16, 0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x11, 0x12, 0x03, 0x17, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11,
    0x04, 0x12, 0x04, 0x17, 0x08, 0x16, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x05,
    0x12, 0x03, 0x17, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x01, 0x12, 0x03,
    0x17, 0x0e, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x03, 0x12, 0x03, 0x17, 0x26,
    0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x12, 0x12, 0x03, 0x18, 0x08, 0x29, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x04, 0x12, 0x04, 0x18, 0x08, 0x17, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x12, 0x05, 0x12, 0x03, 0x18, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x12, 0x01, 0x12, 0x03, 0x18, 0x0e, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x12, 0x03, 0x12, 0x03, 0x18, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x13, 0x12,
    0x03, 0x19, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x04, 0x12, 0x04, 0x19,
    0x08, 0x18, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x05, 0x12, 0x03, 0x19, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x01, 0x12, 0x03, 0x19, 0x0e, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x03, 0x12, 0x03, 0x19, 0x27, 0x29, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x14, 0x12, 0x03, 0x1a, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x14, 0x04, 0x12, 0x04, 0x1a, 0x08, 0x19, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x14, 0x05, 0x12, 0x03, 0x1a, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x14, 0x01,
    0x12, 0x03, 0x1a, 0x0e, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x14, 0x03, 0x12, 0x03,
    0x1a, 0x21, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x15, 0x12, 0x03, 0x1b, 0x08, 0x33,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x15, 0x04, 0x12, 0x03, 0x1b, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x15, 0x05, 0x12, 0x03, 0x1b, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x15, 0x01, 0x12, 0x03, 0x1b, 0x17, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x15, 0x03, 0x12, 0x03, 0x1b, 0x30, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x16,
    0x12, 0x03, 0x1c, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x16, 0x04, 0x12, 0x04,
    0x1c, 0x08, 0x1b, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x16, 0x05, 0x12, 0x03, 0x1c,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x16, 0x01, 0x12, 0x03, 0x1c, 0x0e, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x16, 0x03, 0x12, 0x03, 0x1c, 0x25, 0x27, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1e, 0x00, 0x22, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x1e, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x1f, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x1f, 0x08,
    0x1e, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1f, 0x08, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x24, 0x2e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1f, 0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x20, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x20, 0x08, 0x1f, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x20, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x20, 0x0e, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x20,
    0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x21, 0x08, 0x39, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0x21, 0x08, 0x20, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x21, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x21, 0x0e, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x21, 0x37, 0x38, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04,
    0x23, 0x00, 0x27, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x23, 0x08, 0x1b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x24, 0x08, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x24, 0x08, 0x23, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x24, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x24, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x25, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x25, 0x08,
    0x24, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x25, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x25, 0x0e, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x25, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x26, 0x08, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x04, 0x12, 0x04, 0x26, 0x08, 0x25, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x26, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x26, 0x2d, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x26,
    0x36, 0x37, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x28, 0x00, 0x2a, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x28, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x00, 0x12, 0x03, 0x29, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x29, 0x08, 0x28, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x29, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29,
    0x0d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x21, 0x22,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2b, 0x00, 0x35, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x04, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00,
    0x12, 0x03, 0x2c, 0x08, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x2c, 0x08, 0x2b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2c,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x0e, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x15, 0x16, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x08, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x04, 0x12, 0x04, 0x2d, 0x08, 0x2c, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x2d, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x2d, 0x0e, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x2d, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x2e, 0x08,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x04, 0x2e, 0x08, 0x2d, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2e, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x0e, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2e, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x03, 0x12, 0x03, 0x2f, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04,
    0x12, 0x04, 0x2f, 0x08, 0x2e, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x2f, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2f,
    0x0e, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2f, 0x16, 0x17,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x30, 0x08, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x04, 0x04, 0x12, 0x04, 0x30, 0x08, 0x2f, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x04, 0x05, 0x12, 0x03, 0x30, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x30, 0x0e, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x30, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x03,
    0x31, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x04, 0x12, 0x04, 0x31, 0x08,
    0x30, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x05, 0x12, 0x03, 0x31, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x31, 0x0e, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x03, 0x12, 0x03, 0x31, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x06, 0x12, 0x03, 0x32, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x06, 0x04, 0x12, 0x04, 0x32, 0x08, 0x31, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06,
    0x06, 0x12, 0x03, 0x32, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x32, 0x21, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x03, 0x12, 0x03, 0x32,
    0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x07, 0x12, 0x03, 0x33, 0x08, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x04, 0x12, 0x04, 0x33, 0x08, 0x32, 0x2c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x05, 0x12, 0x03, 0x33, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x07, 0x01, 0x12, 0x03, 0x33, 0x0e, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x07, 0x03, 0x12, 0x03, 0x33, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x08,
    0x12, 0x03, 0x34, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x04, 0x12, 0x04,
    0x34, 0x08, 0x33, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x05, 0x12, 0x03, 0x34,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x01, 0x12, 0x03, 0x34, 0x0e, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x03, 0x12, 0x03, 0x34, 0x19, 0x1b, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x36, 0x00, 0x39, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05,
    0x01, 0x12, 0x03, 0x36, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03,
    0x37, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x37, 0x08,
    0x36, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x37, 0x08, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x37, 0x0d, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x37, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x38, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x38, 0x08, 0x37, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x38, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x38, 0x0d, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x38,
    0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x3a, 0x00, 0x3d, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x00, 0x12, 0x03, 0x3b, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x3b, 0x04, 0x3a, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x3b, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3b,
    0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3b, 0x12, 0x13,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x3c, 0x04, 0x15, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x04, 0x3c, 0x04, 0x3b, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3c, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x3c, 0x0a, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x3c, 0x13, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x3e, 0x00,
    0x41, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x12, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x3f, 0x08, 0x3e, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x3f, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x3f, 0x0e, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x3f, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x40, 0x08,
    0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04, 0x40, 0x08, 0x3f, 0x2e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x40, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x40, 0x0e, 0x36, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x40, 0x39, 0x3a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08,
    0x12, 0x04, 0x42, 0x00, 0x44, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x42,
    0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x43, 0x08, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0x43, 0x08, 0x42, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x43, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x0e, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x43, 0x15, 0x16, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
