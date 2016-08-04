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
pub struct PokemonData {
    // message fields
    id: ::std::option::Option<u64>,
    pokemon_id: ::std::option::Option<super::POGOProtos_Enums::PokemonId>,
    cp: ::std::option::Option<i32>,
    stamina: ::std::option::Option<i32>,
    stamina_max: ::std::option::Option<i32>,
    move_1: ::std::option::Option<super::POGOProtos_Enums::PokemonMove>,
    move_2: ::std::option::Option<super::POGOProtos_Enums::PokemonMove>,
    deployed_fort_id: ::protobuf::SingularField<::std::string::String>,
    owner_name: ::protobuf::SingularField<::std::string::String>,
    is_egg: ::std::option::Option<bool>,
    egg_km_walked_target: ::std::option::Option<f64>,
    egg_km_walked_start: ::std::option::Option<f64>,
    origin: ::std::option::Option<i32>,
    height_m: ::std::option::Option<f32>,
    weight_kg: ::std::option::Option<f32>,
    individual_attack: ::std::option::Option<i32>,
    individual_defense: ::std::option::Option<i32>,
    individual_stamina: ::std::option::Option<i32>,
    cp_multiplier: ::std::option::Option<f32>,
    pokeball: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemId>,
    captured_cell_id: ::std::option::Option<u64>,
    battles_attacked: ::std::option::Option<i32>,
    battles_defended: ::std::option::Option<i32>,
    egg_incubator_id: ::protobuf::SingularField<::std::string::String>,
    creation_time_ms: ::std::option::Option<u64>,
    num_upgrades: ::std::option::Option<i32>,
    additional_cp_multiplier: ::std::option::Option<f32>,
    favorite: ::std::option::Option<i32>,
    nickname: ::protobuf::SingularField<::std::string::String>,
    from_fort: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PokemonData {}

impl PokemonData {
    pub fn new() -> PokemonData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PokemonData {
        static mut instance: ::protobuf::lazy::Lazy<PokemonData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PokemonData,
        };
        unsafe {
            instance.get(|| {
                PokemonData {
                    id: ::std::option::Option::None,
                    pokemon_id: ::std::option::Option::None,
                    cp: ::std::option::Option::None,
                    stamina: ::std::option::Option::None,
                    stamina_max: ::std::option::Option::None,
                    move_1: ::std::option::Option::None,
                    move_2: ::std::option::Option::None,
                    deployed_fort_id: ::protobuf::SingularField::none(),
                    owner_name: ::protobuf::SingularField::none(),
                    is_egg: ::std::option::Option::None,
                    egg_km_walked_target: ::std::option::Option::None,
                    egg_km_walked_start: ::std::option::Option::None,
                    origin: ::std::option::Option::None,
                    height_m: ::std::option::Option::None,
                    weight_kg: ::std::option::Option::None,
                    individual_attack: ::std::option::Option::None,
                    individual_defense: ::std::option::Option::None,
                    individual_stamina: ::std::option::Option::None,
                    cp_multiplier: ::std::option::Option::None,
                    pokeball: ::std::option::Option::None,
                    captured_cell_id: ::std::option::Option::None,
                    battles_attacked: ::std::option::Option::None,
                    battles_defended: ::std::option::Option::None,
                    egg_incubator_id: ::protobuf::SingularField::none(),
                    creation_time_ms: ::std::option::Option::None,
                    num_upgrades: ::std::option::Option::None,
                    additional_cp_multiplier: ::std::option::Option::None,
                    favorite: ::std::option::Option::None,
                    nickname: ::protobuf::SingularField::none(),
                    from_fort: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional fixed64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    // optional .POGOProtos.Enums.PokemonId pokemon_id = 2;

    pub fn clear_pokemon_id(&mut self) {
        self.pokemon_id = ::std::option::Option::None;
    }

    pub fn has_pokemon_id(&self) -> bool {
        self.pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_id(&mut self, v: super::POGOProtos_Enums::PokemonId) {
        self.pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_id(&self) -> super::POGOProtos_Enums::PokemonId {
        self.pokemon_id.unwrap_or(super::POGOProtos_Enums::PokemonId::MISSINGNO)
    }

    // optional int32 cp = 3;

    pub fn clear_cp(&mut self) {
        self.cp = ::std::option::Option::None;
    }

    pub fn has_cp(&self) -> bool {
        self.cp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cp(&mut self, v: i32) {
        self.cp = ::std::option::Option::Some(v);
    }

    pub fn get_cp(&self) -> i32 {
        self.cp.unwrap_or(0)
    }

    // optional int32 stamina = 4;

    pub fn clear_stamina(&mut self) {
        self.stamina = ::std::option::Option::None;
    }

    pub fn has_stamina(&self) -> bool {
        self.stamina.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stamina(&mut self, v: i32) {
        self.stamina = ::std::option::Option::Some(v);
    }

    pub fn get_stamina(&self) -> i32 {
        self.stamina.unwrap_or(0)
    }

    // optional int32 stamina_max = 5;

    pub fn clear_stamina_max(&mut self) {
        self.stamina_max = ::std::option::Option::None;
    }

    pub fn has_stamina_max(&self) -> bool {
        self.stamina_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stamina_max(&mut self, v: i32) {
        self.stamina_max = ::std::option::Option::Some(v);
    }

    pub fn get_stamina_max(&self) -> i32 {
        self.stamina_max.unwrap_or(0)
    }

    // optional .POGOProtos.Enums.PokemonMove move_1 = 6;

    pub fn clear_move_1(&mut self) {
        self.move_1 = ::std::option::Option::None;
    }

    pub fn has_move_1(&self) -> bool {
        self.move_1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_move_1(&mut self, v: super::POGOProtos_Enums::PokemonMove) {
        self.move_1 = ::std::option::Option::Some(v);
    }

    pub fn get_move_1(&self) -> super::POGOProtos_Enums::PokemonMove {
        self.move_1.unwrap_or(super::POGOProtos_Enums::PokemonMove::MOVE_UNSET)
    }

    // optional .POGOProtos.Enums.PokemonMove move_2 = 7;

    pub fn clear_move_2(&mut self) {
        self.move_2 = ::std::option::Option::None;
    }

    pub fn has_move_2(&self) -> bool {
        self.move_2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_move_2(&mut self, v: super::POGOProtos_Enums::PokemonMove) {
        self.move_2 = ::std::option::Option::Some(v);
    }

    pub fn get_move_2(&self) -> super::POGOProtos_Enums::PokemonMove {
        self.move_2.unwrap_or(super::POGOProtos_Enums::PokemonMove::MOVE_UNSET)
    }

    // optional string deployed_fort_id = 8;

    pub fn clear_deployed_fort_id(&mut self) {
        self.deployed_fort_id.clear();
    }

    pub fn has_deployed_fort_id(&self) -> bool {
        self.deployed_fort_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deployed_fort_id(&mut self, v: ::std::string::String) {
        self.deployed_fort_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deployed_fort_id(&mut self) -> &mut ::std::string::String {
        if self.deployed_fort_id.is_none() {
            self.deployed_fort_id.set_default();
        };
        self.deployed_fort_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_deployed_fort_id(&mut self) -> ::std::string::String {
        self.deployed_fort_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_deployed_fort_id(&self) -> &str {
        match self.deployed_fort_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string owner_name = 9;

    pub fn clear_owner_name(&mut self) {
        self.owner_name.clear();
    }

    pub fn has_owner_name(&self) -> bool {
        self.owner_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_name(&mut self, v: ::std::string::String) {
        self.owner_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner_name(&mut self) -> &mut ::std::string::String {
        if self.owner_name.is_none() {
            self.owner_name.set_default();
        };
        self.owner_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner_name(&mut self) -> ::std::string::String {
        self.owner_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_owner_name(&self) -> &str {
        match self.owner_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool is_egg = 10;

    pub fn clear_is_egg(&mut self) {
        self.is_egg = ::std::option::Option::None;
    }

    pub fn has_is_egg(&self) -> bool {
        self.is_egg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_egg(&mut self, v: bool) {
        self.is_egg = ::std::option::Option::Some(v);
    }

    pub fn get_is_egg(&self) -> bool {
        self.is_egg.unwrap_or(false)
    }

    // optional double egg_km_walked_target = 11;

    pub fn clear_egg_km_walked_target(&mut self) {
        self.egg_km_walked_target = ::std::option::Option::None;
    }

    pub fn has_egg_km_walked_target(&self) -> bool {
        self.egg_km_walked_target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_egg_km_walked_target(&mut self, v: f64) {
        self.egg_km_walked_target = ::std::option::Option::Some(v);
    }

    pub fn get_egg_km_walked_target(&self) -> f64 {
        self.egg_km_walked_target.unwrap_or(0.)
    }

    // optional double egg_km_walked_start = 12;

    pub fn clear_egg_km_walked_start(&mut self) {
        self.egg_km_walked_start = ::std::option::Option::None;
    }

    pub fn has_egg_km_walked_start(&self) -> bool {
        self.egg_km_walked_start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_egg_km_walked_start(&mut self, v: f64) {
        self.egg_km_walked_start = ::std::option::Option::Some(v);
    }

    pub fn get_egg_km_walked_start(&self) -> f64 {
        self.egg_km_walked_start.unwrap_or(0.)
    }

    // optional int32 origin = 14;

    pub fn clear_origin(&mut self) {
        self.origin = ::std::option::Option::None;
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: i32) {
        self.origin = ::std::option::Option::Some(v);
    }

    pub fn get_origin(&self) -> i32 {
        self.origin.unwrap_or(0)
    }

    // optional float height_m = 15;

    pub fn clear_height_m(&mut self) {
        self.height_m = ::std::option::Option::None;
    }

    pub fn has_height_m(&self) -> bool {
        self.height_m.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height_m(&mut self, v: f32) {
        self.height_m = ::std::option::Option::Some(v);
    }

    pub fn get_height_m(&self) -> f32 {
        self.height_m.unwrap_or(0.)
    }

    // optional float weight_kg = 16;

    pub fn clear_weight_kg(&mut self) {
        self.weight_kg = ::std::option::Option::None;
    }

    pub fn has_weight_kg(&self) -> bool {
        self.weight_kg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_weight_kg(&mut self, v: f32) {
        self.weight_kg = ::std::option::Option::Some(v);
    }

    pub fn get_weight_kg(&self) -> f32 {
        self.weight_kg.unwrap_or(0.)
    }

    // optional int32 individual_attack = 17;

    pub fn clear_individual_attack(&mut self) {
        self.individual_attack = ::std::option::Option::None;
    }

    pub fn has_individual_attack(&self) -> bool {
        self.individual_attack.is_some()
    }

    // Param is passed by value, moved
    pub fn set_individual_attack(&mut self, v: i32) {
        self.individual_attack = ::std::option::Option::Some(v);
    }

    pub fn get_individual_attack(&self) -> i32 {
        self.individual_attack.unwrap_or(0)
    }

    // optional int32 individual_defense = 18;

    pub fn clear_individual_defense(&mut self) {
        self.individual_defense = ::std::option::Option::None;
    }

    pub fn has_individual_defense(&self) -> bool {
        self.individual_defense.is_some()
    }

    // Param is passed by value, moved
    pub fn set_individual_defense(&mut self, v: i32) {
        self.individual_defense = ::std::option::Option::Some(v);
    }

    pub fn get_individual_defense(&self) -> i32 {
        self.individual_defense.unwrap_or(0)
    }

    // optional int32 individual_stamina = 19;

    pub fn clear_individual_stamina(&mut self) {
        self.individual_stamina = ::std::option::Option::None;
    }

    pub fn has_individual_stamina(&self) -> bool {
        self.individual_stamina.is_some()
    }

    // Param is passed by value, moved
    pub fn set_individual_stamina(&mut self, v: i32) {
        self.individual_stamina = ::std::option::Option::Some(v);
    }

    pub fn get_individual_stamina(&self) -> i32 {
        self.individual_stamina.unwrap_or(0)
    }

    // optional float cp_multiplier = 20;

    pub fn clear_cp_multiplier(&mut self) {
        self.cp_multiplier = ::std::option::Option::None;
    }

    pub fn has_cp_multiplier(&self) -> bool {
        self.cp_multiplier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cp_multiplier(&mut self, v: f32) {
        self.cp_multiplier = ::std::option::Option::Some(v);
    }

    pub fn get_cp_multiplier(&self) -> f32 {
        self.cp_multiplier.unwrap_or(0.)
    }

    // optional .POGOProtos.Inventory.Item.ItemId pokeball = 21;

    pub fn clear_pokeball(&mut self) {
        self.pokeball = ::std::option::Option::None;
    }

    pub fn has_pokeball(&self) -> bool {
        self.pokeball.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokeball(&mut self, v: super::POGOProtos_Inventory_Item::ItemId) {
        self.pokeball = ::std::option::Option::Some(v);
    }

    pub fn get_pokeball(&self) -> super::POGOProtos_Inventory_Item::ItemId {
        self.pokeball.unwrap_or(super::POGOProtos_Inventory_Item::ItemId::ITEM_UNKNOWN)
    }

    // optional uint64 captured_cell_id = 22;

    pub fn clear_captured_cell_id(&mut self) {
        self.captured_cell_id = ::std::option::Option::None;
    }

    pub fn has_captured_cell_id(&self) -> bool {
        self.captured_cell_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_captured_cell_id(&mut self, v: u64) {
        self.captured_cell_id = ::std::option::Option::Some(v);
    }

    pub fn get_captured_cell_id(&self) -> u64 {
        self.captured_cell_id.unwrap_or(0)
    }

    // optional int32 battles_attacked = 23;

    pub fn clear_battles_attacked(&mut self) {
        self.battles_attacked = ::std::option::Option::None;
    }

    pub fn has_battles_attacked(&self) -> bool {
        self.battles_attacked.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battles_attacked(&mut self, v: i32) {
        self.battles_attacked = ::std::option::Option::Some(v);
    }

    pub fn get_battles_attacked(&self) -> i32 {
        self.battles_attacked.unwrap_or(0)
    }

    // optional int32 battles_defended = 24;

    pub fn clear_battles_defended(&mut self) {
        self.battles_defended = ::std::option::Option::None;
    }

    pub fn has_battles_defended(&self) -> bool {
        self.battles_defended.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battles_defended(&mut self, v: i32) {
        self.battles_defended = ::std::option::Option::Some(v);
    }

    pub fn get_battles_defended(&self) -> i32 {
        self.battles_defended.unwrap_or(0)
    }

    // optional string egg_incubator_id = 25;

    pub fn clear_egg_incubator_id(&mut self) {
        self.egg_incubator_id.clear();
    }

    pub fn has_egg_incubator_id(&self) -> bool {
        self.egg_incubator_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_egg_incubator_id(&mut self, v: ::std::string::String) {
        self.egg_incubator_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_egg_incubator_id(&mut self) -> &mut ::std::string::String {
        if self.egg_incubator_id.is_none() {
            self.egg_incubator_id.set_default();
        };
        self.egg_incubator_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_egg_incubator_id(&mut self) -> ::std::string::String {
        self.egg_incubator_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_egg_incubator_id(&self) -> &str {
        match self.egg_incubator_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint64 creation_time_ms = 26;

    pub fn clear_creation_time_ms(&mut self) {
        self.creation_time_ms = ::std::option::Option::None;
    }

    pub fn has_creation_time_ms(&self) -> bool {
        self.creation_time_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creation_time_ms(&mut self, v: u64) {
        self.creation_time_ms = ::std::option::Option::Some(v);
    }

    pub fn get_creation_time_ms(&self) -> u64 {
        self.creation_time_ms.unwrap_or(0)
    }

    // optional int32 num_upgrades = 27;

    pub fn clear_num_upgrades(&mut self) {
        self.num_upgrades = ::std::option::Option::None;
    }

    pub fn has_num_upgrades(&self) -> bool {
        self.num_upgrades.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_upgrades(&mut self, v: i32) {
        self.num_upgrades = ::std::option::Option::Some(v);
    }

    pub fn get_num_upgrades(&self) -> i32 {
        self.num_upgrades.unwrap_or(0)
    }

    // optional float additional_cp_multiplier = 28;

    pub fn clear_additional_cp_multiplier(&mut self) {
        self.additional_cp_multiplier = ::std::option::Option::None;
    }

    pub fn has_additional_cp_multiplier(&self) -> bool {
        self.additional_cp_multiplier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_additional_cp_multiplier(&mut self, v: f32) {
        self.additional_cp_multiplier = ::std::option::Option::Some(v);
    }

    pub fn get_additional_cp_multiplier(&self) -> f32 {
        self.additional_cp_multiplier.unwrap_or(0.)
    }

    // optional int32 favorite = 29;

    pub fn clear_favorite(&mut self) {
        self.favorite = ::std::option::Option::None;
    }

    pub fn has_favorite(&self) -> bool {
        self.favorite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_favorite(&mut self, v: i32) {
        self.favorite = ::std::option::Option::Some(v);
    }

    pub fn get_favorite(&self) -> i32 {
        self.favorite.unwrap_or(0)
    }

    // optional string nickname = 30;

    pub fn clear_nickname(&mut self) {
        self.nickname.clear();
    }

    pub fn has_nickname(&self) -> bool {
        self.nickname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nickname(&mut self, v: ::std::string::String) {
        self.nickname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nickname(&mut self) -> &mut ::std::string::String {
        if self.nickname.is_none() {
            self.nickname.set_default();
        };
        self.nickname.as_mut().unwrap()
    }

    // Take field
    pub fn take_nickname(&mut self) -> ::std::string::String {
        self.nickname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_nickname(&self) -> &str {
        match self.nickname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 from_fort = 31;

    pub fn clear_from_fort(&mut self) {
        self.from_fort = ::std::option::Option::None;
    }

    pub fn has_from_fort(&self) -> bool {
        self.from_fort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_fort(&mut self, v: i32) {
        self.from_fort = ::std::option::Option::Some(v);
    }

    pub fn get_from_fort(&self) -> i32 {
        self.from_fort.unwrap_or(0)
    }
}

impl ::protobuf::Message for PokemonData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.pokemon_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.cp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.stamina = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.stamina_max = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.move_1 = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.move_2 = ::std::option::Option::Some(tmp);
                },
                8 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.deployed_fort_id));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.owner_name));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_egg = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.egg_km_walked_target = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.egg_km_walked_start = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.origin = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.height_m = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.weight_kg = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.individual_attack = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.individual_defense = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.individual_stamina = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.cp_multiplier = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.pokeball = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.captured_cell_id = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.battles_attacked = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.battles_defended = ::std::option::Option::Some(tmp);
                },
                25 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.egg_incubator_id));
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.creation_time_ms = ::std::option::Option::Some(tmp);
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.num_upgrades = ::std::option::Option::Some(tmp);
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.additional_cp_multiplier = ::std::option::Option::Some(tmp);
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.favorite = ::std::option::Option::Some(tmp);
                },
                30 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.nickname));
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.from_fort = ::std::option::Option::Some(tmp);
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
        if self.id.is_some() {
            my_size += 9;
        };
        for value in &self.pokemon_id {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.cp {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.stamina {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.stamina_max {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.move_1 {
            my_size += ::protobuf::rt::enum_size(6, *value);
        };
        for value in &self.move_2 {
            my_size += ::protobuf::rt::enum_size(7, *value);
        };
        for value in &self.deployed_fort_id {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        for value in &self.owner_name {
            my_size += ::protobuf::rt::string_size(9, &value);
        };
        if self.is_egg.is_some() {
            my_size += 2;
        };
        if self.egg_km_walked_target.is_some() {
            my_size += 9;
        };
        if self.egg_km_walked_start.is_some() {
            my_size += 9;
        };
        for value in &self.origin {
            my_size += ::protobuf::rt::value_size(14, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.height_m.is_some() {
            my_size += 5;
        };
        if self.weight_kg.is_some() {
            my_size += 6;
        };
        for value in &self.individual_attack {
            my_size += ::protobuf::rt::value_size(17, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.individual_defense {
            my_size += ::protobuf::rt::value_size(18, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.individual_stamina {
            my_size += ::protobuf::rt::value_size(19, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.cp_multiplier.is_some() {
            my_size += 6;
        };
        for value in &self.pokeball {
            my_size += ::protobuf::rt::enum_size(21, *value);
        };
        for value in &self.captured_cell_id {
            my_size += ::protobuf::rt::value_size(22, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.battles_attacked {
            my_size += ::protobuf::rt::value_size(23, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.battles_defended {
            my_size += ::protobuf::rt::value_size(24, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.egg_incubator_id {
            my_size += ::protobuf::rt::string_size(25, &value);
        };
        for value in &self.creation_time_ms {
            my_size += ::protobuf::rt::value_size(26, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.num_upgrades {
            my_size += ::protobuf::rt::value_size(27, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.additional_cp_multiplier.is_some() {
            my_size += 6;
        };
        for value in &self.favorite {
            my_size += ::protobuf::rt::value_size(29, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.nickname {
            my_size += ::protobuf::rt::string_size(30, &value);
        };
        for value in &self.from_fort {
            my_size += ::protobuf::rt::value_size(31, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_fixed64(1, v));
        };
        if let Some(v) = self.pokemon_id {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.cp {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.stamina {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.stamina_max {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.move_1 {
            try!(os.write_enum(6, v.value()));
        };
        if let Some(v) = self.move_2 {
            try!(os.write_enum(7, v.value()));
        };
        if let Some(v) = self.deployed_fort_id.as_ref() {
            try!(os.write_string(8, &v));
        };
        if let Some(v) = self.owner_name.as_ref() {
            try!(os.write_string(9, &v));
        };
        if let Some(v) = self.is_egg {
            try!(os.write_bool(10, v));
        };
        if let Some(v) = self.egg_km_walked_target {
            try!(os.write_double(11, v));
        };
        if let Some(v) = self.egg_km_walked_start {
            try!(os.write_double(12, v));
        };
        if let Some(v) = self.origin {
            try!(os.write_int32(14, v));
        };
        if let Some(v) = self.height_m {
            try!(os.write_float(15, v));
        };
        if let Some(v) = self.weight_kg {
            try!(os.write_float(16, v));
        };
        if let Some(v) = self.individual_attack {
            try!(os.write_int32(17, v));
        };
        if let Some(v) = self.individual_defense {
            try!(os.write_int32(18, v));
        };
        if let Some(v) = self.individual_stamina {
            try!(os.write_int32(19, v));
        };
        if let Some(v) = self.cp_multiplier {
            try!(os.write_float(20, v));
        };
        if let Some(v) = self.pokeball {
            try!(os.write_enum(21, v.value()));
        };
        if let Some(v) = self.captured_cell_id {
            try!(os.write_uint64(22, v));
        };
        if let Some(v) = self.battles_attacked {
            try!(os.write_int32(23, v));
        };
        if let Some(v) = self.battles_defended {
            try!(os.write_int32(24, v));
        };
        if let Some(v) = self.egg_incubator_id.as_ref() {
            try!(os.write_string(25, &v));
        };
        if let Some(v) = self.creation_time_ms {
            try!(os.write_uint64(26, v));
        };
        if let Some(v) = self.num_upgrades {
            try!(os.write_int32(27, v));
        };
        if let Some(v) = self.additional_cp_multiplier {
            try!(os.write_float(28, v));
        };
        if let Some(v) = self.favorite {
            try!(os.write_int32(29, v));
        };
        if let Some(v) = self.nickname.as_ref() {
            try!(os.write_string(30, &v));
        };
        if let Some(v) = self.from_fort {
            try!(os.write_int32(31, v));
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
        ::std::any::TypeId::of::<PokemonData>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PokemonData {
    fn new() -> PokemonData {
        PokemonData::new()
    }

    fn descriptor_static(_: ::std::option::Option<PokemonData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "id",
                    PokemonData::has_id,
                    PokemonData::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "pokemon_id",
                    PokemonData::has_pokemon_id,
                    PokemonData::get_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "cp",
                    PokemonData::has_cp,
                    PokemonData::get_cp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "stamina",
                    PokemonData::has_stamina,
                    PokemonData::get_stamina,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "stamina_max",
                    PokemonData::has_stamina_max,
                    PokemonData::get_stamina_max,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "move_1",
                    PokemonData::has_move_1,
                    PokemonData::get_move_1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "move_2",
                    PokemonData::has_move_2,
                    PokemonData::get_move_2,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "deployed_fort_id",
                    PokemonData::has_deployed_fort_id,
                    PokemonData::get_deployed_fort_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "owner_name",
                    PokemonData::has_owner_name,
                    PokemonData::get_owner_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_egg",
                    PokemonData::has_is_egg,
                    PokemonData::get_is_egg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "egg_km_walked_target",
                    PokemonData::has_egg_km_walked_target,
                    PokemonData::get_egg_km_walked_target,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "egg_km_walked_start",
                    PokemonData::has_egg_km_walked_start,
                    PokemonData::get_egg_km_walked_start,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "origin",
                    PokemonData::has_origin,
                    PokemonData::get_origin,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "height_m",
                    PokemonData::has_height_m,
                    PokemonData::get_height_m,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "weight_kg",
                    PokemonData::has_weight_kg,
                    PokemonData::get_weight_kg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "individual_attack",
                    PokemonData::has_individual_attack,
                    PokemonData::get_individual_attack,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "individual_defense",
                    PokemonData::has_individual_defense,
                    PokemonData::get_individual_defense,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "individual_stamina",
                    PokemonData::has_individual_stamina,
                    PokemonData::get_individual_stamina,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "cp_multiplier",
                    PokemonData::has_cp_multiplier,
                    PokemonData::get_cp_multiplier,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "pokeball",
                    PokemonData::has_pokeball,
                    PokemonData::get_pokeball,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "captured_cell_id",
                    PokemonData::has_captured_cell_id,
                    PokemonData::get_captured_cell_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "battles_attacked",
                    PokemonData::has_battles_attacked,
                    PokemonData::get_battles_attacked,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "battles_defended",
                    PokemonData::has_battles_defended,
                    PokemonData::get_battles_defended,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "egg_incubator_id",
                    PokemonData::has_egg_incubator_id,
                    PokemonData::get_egg_incubator_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "creation_time_ms",
                    PokemonData::has_creation_time_ms,
                    PokemonData::get_creation_time_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "num_upgrades",
                    PokemonData::has_num_upgrades,
                    PokemonData::get_num_upgrades,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "additional_cp_multiplier",
                    PokemonData::has_additional_cp_multiplier,
                    PokemonData::get_additional_cp_multiplier,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "favorite",
                    PokemonData::has_favorite,
                    PokemonData::get_favorite,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "nickname",
                    PokemonData::has_nickname,
                    PokemonData::get_nickname,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "from_fort",
                    PokemonData::has_from_fort,
                    PokemonData::get_from_fort,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PokemonData>(
                    "PokemonData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PokemonData {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_pokemon_id();
        self.clear_cp();
        self.clear_stamina();
        self.clear_stamina_max();
        self.clear_move_1();
        self.clear_move_2();
        self.clear_deployed_fort_id();
        self.clear_owner_name();
        self.clear_is_egg();
        self.clear_egg_km_walked_target();
        self.clear_egg_km_walked_start();
        self.clear_origin();
        self.clear_height_m();
        self.clear_weight_kg();
        self.clear_individual_attack();
        self.clear_individual_defense();
        self.clear_individual_stamina();
        self.clear_cp_multiplier();
        self.clear_pokeball();
        self.clear_captured_cell_id();
        self.clear_battles_attacked();
        self.clear_battles_defended();
        self.clear_egg_incubator_id();
        self.clear_creation_time_ms();
        self.clear_num_upgrades();
        self.clear_additional_cp_multiplier();
        self.clear_favorite();
        self.clear_nickname();
        self.clear_from_fort();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PokemonData {
    fn eq(&self, other: &PokemonData) -> bool {
        self.id == other.id &&
        self.pokemon_id == other.pokemon_id &&
        self.cp == other.cp &&
        self.stamina == other.stamina &&
        self.stamina_max == other.stamina_max &&
        self.move_1 == other.move_1 &&
        self.move_2 == other.move_2 &&
        self.deployed_fort_id == other.deployed_fort_id &&
        self.owner_name == other.owner_name &&
        self.is_egg == other.is_egg &&
        self.egg_km_walked_target == other.egg_km_walked_target &&
        self.egg_km_walked_start == other.egg_km_walked_start &&
        self.origin == other.origin &&
        self.height_m == other.height_m &&
        self.weight_kg == other.weight_kg &&
        self.individual_attack == other.individual_attack &&
        self.individual_defense == other.individual_defense &&
        self.individual_stamina == other.individual_stamina &&
        self.cp_multiplier == other.cp_multiplier &&
        self.pokeball == other.pokeball &&
        self.captured_cell_id == other.captured_cell_id &&
        self.battles_attacked == other.battles_attacked &&
        self.battles_defended == other.battles_defended &&
        self.egg_incubator_id == other.egg_incubator_id &&
        self.creation_time_ms == other.creation_time_ms &&
        self.num_upgrades == other.num_upgrades &&
        self.additional_cp_multiplier == other.additional_cp_multiplier &&
        self.favorite == other.favorite &&
        self.nickname == other.nickname &&
        self.from_fort == other.from_fort &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PokemonData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PlayerBadge {
    // message fields
    badge_type: ::std::option::Option<super::POGOProtos_Enums::BadgeType>,
    rank: ::std::option::Option<i32>,
    start_value: ::std::option::Option<i32>,
    end_value: ::std::option::Option<i32>,
    current_value: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayerBadge {}

impl PlayerBadge {
    pub fn new() -> PlayerBadge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayerBadge {
        static mut instance: ::protobuf::lazy::Lazy<PlayerBadge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayerBadge,
        };
        unsafe {
            instance.get(|| {
                PlayerBadge {
                    badge_type: ::std::option::Option::None,
                    rank: ::std::option::Option::None,
                    start_value: ::std::option::Option::None,
                    end_value: ::std::option::Option::None,
                    current_value: ::std::option::Option::None,
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

    // optional int32 rank = 2;

    pub fn clear_rank(&mut self) {
        self.rank = ::std::option::Option::None;
    }

    pub fn has_rank(&self) -> bool {
        self.rank.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rank(&mut self, v: i32) {
        self.rank = ::std::option::Option::Some(v);
    }

    pub fn get_rank(&self) -> i32 {
        self.rank.unwrap_or(0)
    }

    // optional int32 start_value = 3;

    pub fn clear_start_value(&mut self) {
        self.start_value = ::std::option::Option::None;
    }

    pub fn has_start_value(&self) -> bool {
        self.start_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_value(&mut self, v: i32) {
        self.start_value = ::std::option::Option::Some(v);
    }

    pub fn get_start_value(&self) -> i32 {
        self.start_value.unwrap_or(0)
    }

    // optional int32 end_value = 4;

    pub fn clear_end_value(&mut self) {
        self.end_value = ::std::option::Option::None;
    }

    pub fn has_end_value(&self) -> bool {
        self.end_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_value(&mut self, v: i32) {
        self.end_value = ::std::option::Option::Some(v);
    }

    pub fn get_end_value(&self) -> i32 {
        self.end_value.unwrap_or(0)
    }

    // optional double current_value = 5;

    pub fn clear_current_value(&mut self) {
        self.current_value = ::std::option::Option::None;
    }

    pub fn has_current_value(&self) -> bool {
        self.current_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_current_value(&mut self, v: f64) {
        self.current_value = ::std::option::Option::Some(v);
    }

    pub fn get_current_value(&self) -> f64 {
        self.current_value.unwrap_or(0.)
    }
}

impl ::protobuf::Message for PlayerBadge {
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
                    self.rank = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.start_value = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.end_value = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.current_value = ::std::option::Option::Some(tmp);
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
        for value in &self.rank {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.start_value {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.end_value {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.current_value.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.badge_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.rank {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.start_value {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.end_value {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.current_value {
            try!(os.write_double(5, v));
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
        ::std::any::TypeId::of::<PlayerBadge>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PlayerBadge {
    fn new() -> PlayerBadge {
        PlayerBadge::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayerBadge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "badge_type",
                    PlayerBadge::has_badge_type,
                    PlayerBadge::get_badge_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "rank",
                    PlayerBadge::has_rank,
                    PlayerBadge::get_rank,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "start_value",
                    PlayerBadge::has_start_value,
                    PlayerBadge::get_start_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "end_value",
                    PlayerBadge::has_end_value,
                    PlayerBadge::get_end_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "current_value",
                    PlayerBadge::has_current_value,
                    PlayerBadge::get_current_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlayerBadge>(
                    "PlayerBadge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayerBadge {
    fn clear(&mut self) {
        self.clear_badge_type();
        self.clear_rank();
        self.clear_start_value();
        self.clear_end_value();
        self.clear_current_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PlayerBadge {
    fn eq(&self, other: &PlayerBadge) -> bool {
        self.badge_type == other.badge_type &&
        self.rank == other.rank &&
        self.start_value == other.start_value &&
        self.end_value == other.end_value &&
        self.current_value == other.current_value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PlayerBadge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DownloadUrlEntry {
    // message fields
    asset_id: ::protobuf::SingularField<::std::string::String>,
    url: ::protobuf::SingularField<::std::string::String>,
    size: ::std::option::Option<i32>,
    checksum: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DownloadUrlEntry {}

impl DownloadUrlEntry {
    pub fn new() -> DownloadUrlEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DownloadUrlEntry {
        static mut instance: ::protobuf::lazy::Lazy<DownloadUrlEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DownloadUrlEntry,
        };
        unsafe {
            instance.get(|| {
                DownloadUrlEntry {
                    asset_id: ::protobuf::SingularField::none(),
                    url: ::protobuf::SingularField::none(),
                    size: ::std::option::Option::None,
                    checksum: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string asset_id = 1;

    pub fn clear_asset_id(&mut self) {
        self.asset_id.clear();
    }

    pub fn has_asset_id(&self) -> bool {
        self.asset_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_asset_id(&mut self, v: ::std::string::String) {
        self.asset_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_asset_id(&mut self) -> &mut ::std::string::String {
        if self.asset_id.is_none() {
            self.asset_id.set_default();
        };
        self.asset_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_asset_id(&mut self) -> ::std::string::String {
        self.asset_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_asset_id(&self) -> &str {
        match self.asset_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string url = 2;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        if self.url.is_none() {
            self.url.set_default();
        };
        self.url.as_mut().unwrap()
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        self.url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        match self.url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 size = 3;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: i32) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> i32 {
        self.size.unwrap_or(0)
    }

    // optional fixed32 checksum = 4;

    pub fn clear_checksum(&mut self) {
        self.checksum = ::std::option::Option::None;
    }

    pub fn has_checksum(&self) -> bool {
        self.checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksum(&mut self, v: u32) {
        self.checksum = ::std::option::Option::Some(v);
    }

    pub fn get_checksum(&self) -> u32 {
        self.checksum.unwrap_or(0)
    }
}

impl ::protobuf::Message for DownloadUrlEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.asset_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.size = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed32());
                    self.checksum = ::std::option::Option::Some(tmp);
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
        for value in &self.asset_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.url {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.size {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.checksum.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.asset_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.url.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.size {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.checksum {
            try!(os.write_fixed32(4, v));
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
        ::std::any::TypeId::of::<DownloadUrlEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DownloadUrlEntry {
    fn new() -> DownloadUrlEntry {
        DownloadUrlEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<DownloadUrlEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "asset_id",
                    DownloadUrlEntry::has_asset_id,
                    DownloadUrlEntry::get_asset_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "url",
                    DownloadUrlEntry::has_url,
                    DownloadUrlEntry::get_url,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "size",
                    DownloadUrlEntry::has_size,
                    DownloadUrlEntry::get_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "checksum",
                    DownloadUrlEntry::has_checksum,
                    DownloadUrlEntry::get_checksum,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DownloadUrlEntry>(
                    "DownloadUrlEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DownloadUrlEntry {
    fn clear(&mut self) {
        self.clear_asset_id();
        self.clear_url();
        self.clear_size();
        self.clear_checksum();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DownloadUrlEntry {
    fn eq(&self, other: &DownloadUrlEntry) -> bool {
        self.asset_id == other.asset_id &&
        self.url == other.url &&
        self.size == other.size &&
        self.checksum == other.checksum &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DownloadUrlEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AssetDigestEntry {
    // message fields
    asset_id: ::protobuf::SingularField<::std::string::String>,
    bundle_name: ::protobuf::SingularField<::std::string::String>,
    version: ::std::option::Option<i64>,
    checksum: ::std::option::Option<u32>,
    size: ::std::option::Option<i32>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AssetDigestEntry {}

impl AssetDigestEntry {
    pub fn new() -> AssetDigestEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AssetDigestEntry {
        static mut instance: ::protobuf::lazy::Lazy<AssetDigestEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AssetDigestEntry,
        };
        unsafe {
            instance.get(|| {
                AssetDigestEntry {
                    asset_id: ::protobuf::SingularField::none(),
                    bundle_name: ::protobuf::SingularField::none(),
                    version: ::std::option::Option::None,
                    checksum: ::std::option::Option::None,
                    size: ::std::option::Option::None,
                    key: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string asset_id = 1;

    pub fn clear_asset_id(&mut self) {
        self.asset_id.clear();
    }

    pub fn has_asset_id(&self) -> bool {
        self.asset_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_asset_id(&mut self, v: ::std::string::String) {
        self.asset_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_asset_id(&mut self) -> &mut ::std::string::String {
        if self.asset_id.is_none() {
            self.asset_id.set_default();
        };
        self.asset_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_asset_id(&mut self) -> ::std::string::String {
        self.asset_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_asset_id(&self) -> &str {
        match self.asset_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string bundle_name = 2;

    pub fn clear_bundle_name(&mut self) {
        self.bundle_name.clear();
    }

    pub fn has_bundle_name(&self) -> bool {
        self.bundle_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bundle_name(&mut self, v: ::std::string::String) {
        self.bundle_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bundle_name(&mut self) -> &mut ::std::string::String {
        if self.bundle_name.is_none() {
            self.bundle_name.set_default();
        };
        self.bundle_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_bundle_name(&mut self) -> ::std::string::String {
        self.bundle_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_bundle_name(&self) -> &str {
        match self.bundle_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int64 version = 3;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> i64 {
        self.version.unwrap_or(0)
    }

    // optional fixed32 checksum = 4;

    pub fn clear_checksum(&mut self) {
        self.checksum = ::std::option::Option::None;
    }

    pub fn has_checksum(&self) -> bool {
        self.checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksum(&mut self, v: u32) {
        self.checksum = ::std::option::Option::Some(v);
    }

    pub fn get_checksum(&self) -> u32 {
        self.checksum.unwrap_or(0)
    }

    // optional int32 size = 5;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: i32) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> i32 {
        self.size.unwrap_or(0)
    }

    // optional bytes key = 6;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for AssetDigestEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.asset_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.bundle_name));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.version = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed32());
                    self.checksum = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.size = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
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
        for value in &self.asset_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.bundle_name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.version {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.checksum.is_some() {
            my_size += 5;
        };
        for value in &self.size {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(6, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.asset_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.bundle_name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.version {
            try!(os.write_int64(3, v));
        };
        if let Some(v) = self.checksum {
            try!(os.write_fixed32(4, v));
        };
        if let Some(v) = self.size {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(6, &v));
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
        ::std::any::TypeId::of::<AssetDigestEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AssetDigestEntry {
    fn new() -> AssetDigestEntry {
        AssetDigestEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<AssetDigestEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "asset_id",
                    AssetDigestEntry::has_asset_id,
                    AssetDigestEntry::get_asset_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "bundle_name",
                    AssetDigestEntry::has_bundle_name,
                    AssetDigestEntry::get_bundle_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "version",
                    AssetDigestEntry::has_version,
                    AssetDigestEntry::get_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "checksum",
                    AssetDigestEntry::has_checksum,
                    AssetDigestEntry::get_checksum,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "size",
                    AssetDigestEntry::has_size,
                    AssetDigestEntry::get_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    AssetDigestEntry::has_key,
                    AssetDigestEntry::get_key,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AssetDigestEntry>(
                    "AssetDigestEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AssetDigestEntry {
    fn clear(&mut self) {
        self.clear_asset_id();
        self.clear_bundle_name();
        self.clear_version();
        self.clear_checksum();
        self.clear_size();
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AssetDigestEntry {
    fn eq(&self, other: &AssetDigestEntry) -> bool {
        self.asset_id == other.asset_id &&
        self.bundle_name == other.bundle_name &&
        self.version == other.version &&
        self.checksum == other.checksum &&
        self.size == other.size &&
        self.key == other.key &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AssetDigestEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PokedexEntry {
    // message fields
    pokemon_id: ::std::option::Option<super::POGOProtos_Enums::PokemonId>,
    times_encountered: ::std::option::Option<i32>,
    times_captured: ::std::option::Option<i32>,
    evolution_stone_pieces: ::std::option::Option<i32>,
    evolution_stones: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PokedexEntry {}

impl PokedexEntry {
    pub fn new() -> PokedexEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PokedexEntry {
        static mut instance: ::protobuf::lazy::Lazy<PokedexEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PokedexEntry,
        };
        unsafe {
            instance.get(|| {
                PokedexEntry {
                    pokemon_id: ::std::option::Option::None,
                    times_encountered: ::std::option::Option::None,
                    times_captured: ::std::option::Option::None,
                    evolution_stone_pieces: ::std::option::Option::None,
                    evolution_stones: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Enums.PokemonId pokemon_id = 1;

    pub fn clear_pokemon_id(&mut self) {
        self.pokemon_id = ::std::option::Option::None;
    }

    pub fn has_pokemon_id(&self) -> bool {
        self.pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_id(&mut self, v: super::POGOProtos_Enums::PokemonId) {
        self.pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_id(&self) -> super::POGOProtos_Enums::PokemonId {
        self.pokemon_id.unwrap_or(super::POGOProtos_Enums::PokemonId::MISSINGNO)
    }

    // optional int32 times_encountered = 2;

    pub fn clear_times_encountered(&mut self) {
        self.times_encountered = ::std::option::Option::None;
    }

    pub fn has_times_encountered(&self) -> bool {
        self.times_encountered.is_some()
    }

    // Param is passed by value, moved
    pub fn set_times_encountered(&mut self, v: i32) {
        self.times_encountered = ::std::option::Option::Some(v);
    }

    pub fn get_times_encountered(&self) -> i32 {
        self.times_encountered.unwrap_or(0)
    }

    // optional int32 times_captured = 3;

    pub fn clear_times_captured(&mut self) {
        self.times_captured = ::std::option::Option::None;
    }

    pub fn has_times_captured(&self) -> bool {
        self.times_captured.is_some()
    }

    // Param is passed by value, moved
    pub fn set_times_captured(&mut self, v: i32) {
        self.times_captured = ::std::option::Option::Some(v);
    }

    pub fn get_times_captured(&self) -> i32 {
        self.times_captured.unwrap_or(0)
    }

    // optional int32 evolution_stone_pieces = 4;

    pub fn clear_evolution_stone_pieces(&mut self) {
        self.evolution_stone_pieces = ::std::option::Option::None;
    }

    pub fn has_evolution_stone_pieces(&self) -> bool {
        self.evolution_stone_pieces.is_some()
    }

    // Param is passed by value, moved
    pub fn set_evolution_stone_pieces(&mut self, v: i32) {
        self.evolution_stone_pieces = ::std::option::Option::Some(v);
    }

    pub fn get_evolution_stone_pieces(&self) -> i32 {
        self.evolution_stone_pieces.unwrap_or(0)
    }

    // optional int32 evolution_stones = 5;

    pub fn clear_evolution_stones(&mut self) {
        self.evolution_stones = ::std::option::Option::None;
    }

    pub fn has_evolution_stones(&self) -> bool {
        self.evolution_stones.is_some()
    }

    // Param is passed by value, moved
    pub fn set_evolution_stones(&mut self, v: i32) {
        self.evolution_stones = ::std::option::Option::Some(v);
    }

    pub fn get_evolution_stones(&self) -> i32 {
        self.evolution_stones.unwrap_or(0)
    }
}

impl ::protobuf::Message for PokedexEntry {
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
                    self.pokemon_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.times_encountered = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.times_captured = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.evolution_stone_pieces = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.evolution_stones = ::std::option::Option::Some(tmp);
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
        for value in &self.pokemon_id {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.times_encountered {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.times_captured {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.evolution_stone_pieces {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.evolution_stones {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pokemon_id {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.times_encountered {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.times_captured {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.evolution_stone_pieces {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.evolution_stones {
            try!(os.write_int32(5, v));
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
        ::std::any::TypeId::of::<PokedexEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PokedexEntry {
    fn new() -> PokedexEntry {
        PokedexEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<PokedexEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "pokemon_id",
                    PokedexEntry::has_pokemon_id,
                    PokedexEntry::get_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "times_encountered",
                    PokedexEntry::has_times_encountered,
                    PokedexEntry::get_times_encountered,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "times_captured",
                    PokedexEntry::has_times_captured,
                    PokedexEntry::get_times_captured,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "evolution_stone_pieces",
                    PokedexEntry::has_evolution_stone_pieces,
                    PokedexEntry::get_evolution_stone_pieces,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "evolution_stones",
                    PokedexEntry::has_evolution_stones,
                    PokedexEntry::get_evolution_stones,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PokedexEntry>(
                    "PokedexEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PokedexEntry {
    fn clear(&mut self) {
        self.clear_pokemon_id();
        self.clear_times_encountered();
        self.clear_times_captured();
        self.clear_evolution_stone_pieces();
        self.clear_evolution_stones();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PokedexEntry {
    fn eq(&self, other: &PokedexEntry) -> bool {
        self.pokemon_id == other.pokemon_id &&
        self.times_encountered == other.times_encountered &&
        self.times_captured == other.times_captured &&
        self.evolution_stone_pieces == other.evolution_stone_pieces &&
        self.evolution_stones == other.evolution_stones &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PokedexEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PlayerData {
    // message fields
    creation_timestamp_ms: ::std::option::Option<i64>,
    username: ::protobuf::SingularField<::std::string::String>,
    team: ::std::option::Option<super::POGOProtos_Enums::TeamColor>,
    tutorial_state: ::std::vec::Vec<super::POGOProtos_Enums::TutorialState>,
    avatar: ::protobuf::SingularPtrField<super::POGOProtos_Data_Player::PlayerAvatar>,
    max_pokemon_storage: ::std::option::Option<i32>,
    max_item_storage: ::std::option::Option<i32>,
    daily_bonus: ::protobuf::SingularPtrField<super::POGOProtos_Data_Player::DailyBonus>,
    equipped_badge: ::protobuf::SingularPtrField<super::POGOProtos_Data_Player::EquippedBadge>,
    contact_settings: ::protobuf::SingularPtrField<super::POGOProtos_Data_Player::ContactSettings>,
    currencies: ::protobuf::RepeatedField<super::POGOProtos_Data_Player::Currency>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayerData {}

impl PlayerData {
    pub fn new() -> PlayerData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayerData {
        static mut instance: ::protobuf::lazy::Lazy<PlayerData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayerData,
        };
        unsafe {
            instance.get(|| {
                PlayerData {
                    creation_timestamp_ms: ::std::option::Option::None,
                    username: ::protobuf::SingularField::none(),
                    team: ::std::option::Option::None,
                    tutorial_state: ::std::vec::Vec::new(),
                    avatar: ::protobuf::SingularPtrField::none(),
                    max_pokemon_storage: ::std::option::Option::None,
                    max_item_storage: ::std::option::Option::None,
                    daily_bonus: ::protobuf::SingularPtrField::none(),
                    equipped_badge: ::protobuf::SingularPtrField::none(),
                    contact_settings: ::protobuf::SingularPtrField::none(),
                    currencies: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 creation_timestamp_ms = 1;

    pub fn clear_creation_timestamp_ms(&mut self) {
        self.creation_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_creation_timestamp_ms(&self) -> bool {
        self.creation_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creation_timestamp_ms(&mut self, v: i64) {
        self.creation_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_creation_timestamp_ms(&self) -> i64 {
        self.creation_timestamp_ms.unwrap_or(0)
    }

    // optional string username = 2;

    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    pub fn has_username(&self) -> bool {
        self.username.is_some()
    }

    // Param is passed by value, moved
    pub fn set_username(&mut self, v: ::std::string::String) {
        self.username = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username(&mut self) -> &mut ::std::string::String {
        if self.username.is_none() {
            self.username.set_default();
        };
        self.username.as_mut().unwrap()
    }

    // Take field
    pub fn take_username(&mut self) -> ::std::string::String {
        self.username.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_username(&self) -> &str {
        match self.username.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .POGOProtos.Enums.TeamColor team = 5;

    pub fn clear_team(&mut self) {
        self.team = ::std::option::Option::None;
    }

    pub fn has_team(&self) -> bool {
        self.team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team(&mut self, v: super::POGOProtos_Enums::TeamColor) {
        self.team = ::std::option::Option::Some(v);
    }

    pub fn get_team(&self) -> super::POGOProtos_Enums::TeamColor {
        self.team.unwrap_or(super::POGOProtos_Enums::TeamColor::NEUTRAL)
    }

    // repeated .POGOProtos.Enums.TutorialState tutorial_state = 7;

    pub fn clear_tutorial_state(&mut self) {
        self.tutorial_state.clear();
    }

    // Param is passed by value, moved
    pub fn set_tutorial_state(&mut self, v: ::std::vec::Vec<super::POGOProtos_Enums::TutorialState>) {
        self.tutorial_state = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tutorial_state(&mut self) -> &mut ::std::vec::Vec<super::POGOProtos_Enums::TutorialState> {
        &mut self.tutorial_state
    }

    // Take field
    pub fn take_tutorial_state(&mut self) -> ::std::vec::Vec<super::POGOProtos_Enums::TutorialState> {
        ::std::mem::replace(&mut self.tutorial_state, ::std::vec::Vec::new())
    }

    pub fn get_tutorial_state(&self) -> &[super::POGOProtos_Enums::TutorialState] {
        &self.tutorial_state
    }

    // optional .POGOProtos.Data.Player.PlayerAvatar avatar = 8;

    pub fn clear_avatar(&mut self) {
        self.avatar.clear();
    }

    pub fn has_avatar(&self) -> bool {
        self.avatar.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avatar(&mut self, v: super::POGOProtos_Data_Player::PlayerAvatar) {
        self.avatar = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_avatar(&mut self) -> &mut super::POGOProtos_Data_Player::PlayerAvatar {
        if self.avatar.is_none() {
            self.avatar.set_default();
        };
        self.avatar.as_mut().unwrap()
    }

    // Take field
    pub fn take_avatar(&mut self) -> super::POGOProtos_Data_Player::PlayerAvatar {
        self.avatar.take().unwrap_or_else(|| super::POGOProtos_Data_Player::PlayerAvatar::new())
    }

    pub fn get_avatar(&self) -> &super::POGOProtos_Data_Player::PlayerAvatar {
        self.avatar.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Player::PlayerAvatar::default_instance())
    }

    // optional int32 max_pokemon_storage = 9;

    pub fn clear_max_pokemon_storage(&mut self) {
        self.max_pokemon_storage = ::std::option::Option::None;
    }

    pub fn has_max_pokemon_storage(&self) -> bool {
        self.max_pokemon_storage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_pokemon_storage(&mut self, v: i32) {
        self.max_pokemon_storage = ::std::option::Option::Some(v);
    }

    pub fn get_max_pokemon_storage(&self) -> i32 {
        self.max_pokemon_storage.unwrap_or(0)
    }

    // optional int32 max_item_storage = 10;

    pub fn clear_max_item_storage(&mut self) {
        self.max_item_storage = ::std::option::Option::None;
    }

    pub fn has_max_item_storage(&self) -> bool {
        self.max_item_storage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_item_storage(&mut self, v: i32) {
        self.max_item_storage = ::std::option::Option::Some(v);
    }

    pub fn get_max_item_storage(&self) -> i32 {
        self.max_item_storage.unwrap_or(0)
    }

    // optional .POGOProtos.Data.Player.DailyBonus daily_bonus = 11;

    pub fn clear_daily_bonus(&mut self) {
        self.daily_bonus.clear();
    }

    pub fn has_daily_bonus(&self) -> bool {
        self.daily_bonus.is_some()
    }

    // Param is passed by value, moved
    pub fn set_daily_bonus(&mut self, v: super::POGOProtos_Data_Player::DailyBonus) {
        self.daily_bonus = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_daily_bonus(&mut self) -> &mut super::POGOProtos_Data_Player::DailyBonus {
        if self.daily_bonus.is_none() {
            self.daily_bonus.set_default();
        };
        self.daily_bonus.as_mut().unwrap()
    }

    // Take field
    pub fn take_daily_bonus(&mut self) -> super::POGOProtos_Data_Player::DailyBonus {
        self.daily_bonus.take().unwrap_or_else(|| super::POGOProtos_Data_Player::DailyBonus::new())
    }

    pub fn get_daily_bonus(&self) -> &super::POGOProtos_Data_Player::DailyBonus {
        self.daily_bonus.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Player::DailyBonus::default_instance())
    }

    // optional .POGOProtos.Data.Player.EquippedBadge equipped_badge = 12;

    pub fn clear_equipped_badge(&mut self) {
        self.equipped_badge.clear();
    }

    pub fn has_equipped_badge(&self) -> bool {
        self.equipped_badge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_equipped_badge(&mut self, v: super::POGOProtos_Data_Player::EquippedBadge) {
        self.equipped_badge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_equipped_badge(&mut self) -> &mut super::POGOProtos_Data_Player::EquippedBadge {
        if self.equipped_badge.is_none() {
            self.equipped_badge.set_default();
        };
        self.equipped_badge.as_mut().unwrap()
    }

    // Take field
    pub fn take_equipped_badge(&mut self) -> super::POGOProtos_Data_Player::EquippedBadge {
        self.equipped_badge.take().unwrap_or_else(|| super::POGOProtos_Data_Player::EquippedBadge::new())
    }

    pub fn get_equipped_badge(&self) -> &super::POGOProtos_Data_Player::EquippedBadge {
        self.equipped_badge.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Player::EquippedBadge::default_instance())
    }

    // optional .POGOProtos.Data.Player.ContactSettings contact_settings = 13;

    pub fn clear_contact_settings(&mut self) {
        self.contact_settings.clear();
    }

    pub fn has_contact_settings(&self) -> bool {
        self.contact_settings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contact_settings(&mut self, v: super::POGOProtos_Data_Player::ContactSettings) {
        self.contact_settings = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contact_settings(&mut self) -> &mut super::POGOProtos_Data_Player::ContactSettings {
        if self.contact_settings.is_none() {
            self.contact_settings.set_default();
        };
        self.contact_settings.as_mut().unwrap()
    }

    // Take field
    pub fn take_contact_settings(&mut self) -> super::POGOProtos_Data_Player::ContactSettings {
        self.contact_settings.take().unwrap_or_else(|| super::POGOProtos_Data_Player::ContactSettings::new())
    }

    pub fn get_contact_settings(&self) -> &super::POGOProtos_Data_Player::ContactSettings {
        self.contact_settings.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Player::ContactSettings::default_instance())
    }

    // repeated .POGOProtos.Data.Player.Currency currencies = 14;

    pub fn clear_currencies(&mut self) {
        self.currencies.clear();
    }

    // Param is passed by value, moved
    pub fn set_currencies(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Data_Player::Currency>) {
        self.currencies = v;
    }

    // Mutable pointer to the field.
    pub fn mut_currencies(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Data_Player::Currency> {
        &mut self.currencies
    }

    // Take field
    pub fn take_currencies(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Data_Player::Currency> {
        ::std::mem::replace(&mut self.currencies, ::protobuf::RepeatedField::new())
    }

    pub fn get_currencies(&self) -> &[super::POGOProtos_Data_Player::Currency] {
        &self.currencies
    }
}

impl ::protobuf::Message for PlayerData {
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
                    self.creation_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.username));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.team = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.tutorial_state));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.avatar));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.max_pokemon_storage = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.max_item_storage = ::std::option::Option::Some(tmp);
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.daily_bonus));
                },
                12 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.equipped_badge));
                },
                13 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.contact_settings));
                },
                14 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.currencies));
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
        for value in &self.creation_timestamp_ms {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.username {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.team {
            my_size += ::protobuf::rt::enum_size(5, *value);
        };
        if !self.tutorial_state.is_empty() {
            my_size += ::protobuf::rt::vec_packed_enum_size(7, &self.tutorial_state);
        };
        for value in &self.avatar {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.max_pokemon_storage {
            my_size += ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.max_item_storage {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.daily_bonus {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.equipped_badge {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.contact_settings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.currencies {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.creation_timestamp_ms {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.username.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.team {
            try!(os.write_enum(5, v.value()));
        };
        if !self.tutorial_state.is_empty() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_enum_data_size(&self.tutorial_state)));
            for v in &self.tutorial_state {
                try!(os.write_enum_no_tag(v.value()));
            };
        };
        if let Some(v) = self.avatar.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.max_pokemon_storage {
            try!(os.write_int32(9, v));
        };
        if let Some(v) = self.max_item_storage {
            try!(os.write_int32(10, v));
        };
        if let Some(v) = self.daily_bonus.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.equipped_badge.as_ref() {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.contact_settings.as_ref() {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.currencies {
            try!(os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<PlayerData>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PlayerData {
    fn new() -> PlayerData {
        PlayerData::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayerData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "creation_timestamp_ms",
                    PlayerData::has_creation_timestamp_ms,
                    PlayerData::get_creation_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "username",
                    PlayerData::has_username,
                    PlayerData::get_username,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "team",
                    PlayerData::has_team,
                    PlayerData::get_team,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "tutorial_state",
                    PlayerData::get_tutorial_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "avatar",
                    PlayerData::has_avatar,
                    PlayerData::get_avatar,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "max_pokemon_storage",
                    PlayerData::has_max_pokemon_storage,
                    PlayerData::get_max_pokemon_storage,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "max_item_storage",
                    PlayerData::has_max_item_storage,
                    PlayerData::get_max_item_storage,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "daily_bonus",
                    PlayerData::has_daily_bonus,
                    PlayerData::get_daily_bonus,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "equipped_badge",
                    PlayerData::has_equipped_badge,
                    PlayerData::get_equipped_badge,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "contact_settings",
                    PlayerData::has_contact_settings,
                    PlayerData::get_contact_settings,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "currencies",
                    PlayerData::get_currencies,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlayerData>(
                    "PlayerData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayerData {
    fn clear(&mut self) {
        self.clear_creation_timestamp_ms();
        self.clear_username();
        self.clear_team();
        self.clear_tutorial_state();
        self.clear_avatar();
        self.clear_max_pokemon_storage();
        self.clear_max_item_storage();
        self.clear_daily_bonus();
        self.clear_equipped_badge();
        self.clear_contact_settings();
        self.clear_currencies();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PlayerData {
    fn eq(&self, other: &PlayerData) -> bool {
        self.creation_timestamp_ms == other.creation_timestamp_ms &&
        self.username == other.username &&
        self.team == other.team &&
        self.tutorial_state == other.tutorial_state &&
        self.avatar == other.avatar &&
        self.max_pokemon_storage == other.max_pokemon_storage &&
        self.max_item_storage == other.max_item_storage &&
        self.daily_bonus == other.daily_bonus &&
        self.equipped_badge == other.equipped_badge &&
        self.contact_settings == other.contact_settings &&
        self.currencies == other.currencies &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PlayerData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x15, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x1a, 0x16, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x1f, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76,
    0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x1c, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61,
    0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x50,
    0x00, 0x50, 0x01, 0x50, 0x02, 0x22, 0x95, 0x09, 0x0a, 0x0b, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f,
    0x6e, 0x44, 0x61, 0x74, 0x61, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x06, 0x52, 0x02, 0x69, 0x64, 0x12, 0x3a, 0x0a, 0x0a, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e,
    0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x52, 0x09, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49,
    0x64, 0x12, 0x0e, 0x0a, 0x02, 0x63, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x02, 0x63,
    0x70, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x74, 0x61, 0x6d, 0x69, 0x6e, 0x61, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x07, 0x73, 0x74, 0x61, 0x6d, 0x69, 0x6e, 0x61, 0x12, 0x1f, 0x0a, 0x0b, 0x73,
    0x74, 0x61, 0x6d, 0x69, 0x6e, 0x61, 0x5f, 0x6d, 0x61, 0x78, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x0a, 0x73, 0x74, 0x61, 0x6d, 0x69, 0x6e, 0x61, 0x4d, 0x61, 0x78, 0x12, 0x34, 0x0a, 0x06,
    0x6d, 0x6f, 0x76, 0x65, 0x5f, 0x31, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e,
    0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x4d, 0x6f, 0x76, 0x65, 0x52, 0x05, 0x6d, 0x6f, 0x76,
    0x65, 0x31, 0x12, 0x34, 0x0a, 0x06, 0x6d, 0x6f, 0x76, 0x65, 0x5f, 0x32, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x4d, 0x6f, 0x76,
    0x65, 0x52, 0x05, 0x6d, 0x6f, 0x76, 0x65, 0x32, 0x12, 0x28, 0x0a, 0x10, 0x64, 0x65, 0x70, 0x6c,
    0x6f, 0x79, 0x65, 0x64, 0x5f, 0x66, 0x6f, 0x72, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0e, 0x64, 0x65, 0x70, 0x6c, 0x6f, 0x79, 0x65, 0x64, 0x46, 0x6f, 0x72, 0x74,
    0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x09, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x4e, 0x61, 0x6d,
    0x65, 0x12, 0x15, 0x0a, 0x06, 0x69, 0x73, 0x5f, 0x65, 0x67, 0x67, 0x18, 0x0a, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x05, 0x69, 0x73, 0x45, 0x67, 0x67, 0x12, 0x2f, 0x0a, 0x14, 0x65, 0x67, 0x67, 0x5f,
    0x6b, 0x6d, 0x5f, 0x77, 0x61, 0x6c, 0x6b, 0x65, 0x64, 0x5f, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74,
    0x18, 0x0b, 0x20, 0x01, 0x28, 0x01, 0x52, 0x11, 0x65, 0x67, 0x67, 0x4b, 0x6d, 0x57, 0x61, 0x6c,
    0x6b, 0x65, 0x64, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74, 0x12, 0x2d, 0x0a, 0x13, 0x65, 0x67, 0x67,
    0x5f, 0x6b, 0x6d, 0x5f, 0x77, 0x61, 0x6c, 0x6b, 0x65, 0x64, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74,
    0x18, 0x0c, 0x20, 0x01, 0x28, 0x01, 0x52, 0x10, 0x65, 0x67, 0x67, 0x4b, 0x6d, 0x57, 0x61, 0x6c,
    0x6b, 0x65, 0x64, 0x53, 0x74, 0x61, 0x72, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67,
    0x69, 0x6e, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x12, 0x19, 0x0a, 0x08, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x6d, 0x18, 0x0f, 0x20, 0x01,
    0x28, 0x02, 0x52, 0x07, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x4d, 0x12, 0x1b, 0x0a, 0x09, 0x77,
    0x65, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x6b, 0x67, 0x18, 0x10, 0x20, 0x01, 0x28, 0x02, 0x52, 0x08,
    0x77, 0x65, 0x69, 0x67, 0x68, 0x74, 0x4b, 0x67, 0x12, 0x2b, 0x0a, 0x11, 0x69, 0x6e, 0x64, 0x69,
    0x76, 0x69, 0x64, 0x75, 0x61, 0x6c, 0x5f, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x18, 0x11, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x10, 0x69, 0x6e, 0x64, 0x69, 0x76, 0x69, 0x64, 0x75, 0x61, 0x6c, 0x41,
    0x74, 0x74, 0x61, 0x63, 0x6b, 0x12, 0x2d, 0x0a, 0x12, 0x69, 0x6e, 0x64, 0x69, 0x76, 0x69, 0x64,
    0x75, 0x61, 0x6c, 0x5f, 0x64, 0x65, 0x66, 0x65, 0x6e, 0x73, 0x65, 0x18, 0x12, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x11, 0x69, 0x6e, 0x64, 0x69, 0x76, 0x69, 0x64, 0x75, 0x61, 0x6c, 0x44, 0x65, 0x66,
    0x65, 0x6e, 0x73, 0x65, 0x12, 0x2d, 0x0a, 0x12, 0x69, 0x6e, 0x64, 0x69, 0x76, 0x69, 0x64, 0x75,
    0x61, 0x6c, 0x5f, 0x73, 0x74, 0x61, 0x6d, 0x69, 0x6e, 0x61, 0x18, 0x13, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x11, 0x69, 0x6e, 0x64, 0x69, 0x76, 0x69, 0x64, 0x75, 0x61, 0x6c, 0x53, 0x74, 0x61, 0x6d,
    0x69, 0x6e, 0x61, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x70, 0x5f, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70,
    0x6c, 0x69, 0x65, 0x72, 0x18, 0x14, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0c, 0x63, 0x70, 0x4d, 0x75,
    0x6c, 0x74, 0x69, 0x70, 0x6c, 0x69, 0x65, 0x72, 0x12, 0x3d, 0x0a, 0x08, 0x70, 0x6f, 0x6b, 0x65,
    0x62, 0x61, 0x6c, 0x6c, 0x18, 0x15, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72,
    0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x52, 0x08, 0x70,
    0x6f, 0x6b, 0x65, 0x62, 0x61, 0x6c, 0x6c, 0x12, 0x28, 0x0a, 0x10, 0x63, 0x61, 0x70, 0x74, 0x75,
    0x72, 0x65, 0x64, 0x5f, 0x63, 0x65, 0x6c, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x16, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x0e, 0x63, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x64, 0x43, 0x65, 0x6c, 0x6c, 0x49,
    0x64, 0x12, 0x29, 0x0a, 0x10, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x73, 0x5f, 0x61, 0x74, 0x74,
    0x61, 0x63, 0x6b, 0x65, 0x64, 0x18, 0x17, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0f, 0x62, 0x61, 0x74,
    0x74, 0x6c, 0x65, 0x73, 0x41, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x12, 0x29, 0x0a, 0x10,
    0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x73, 0x5f, 0x64, 0x65, 0x66, 0x65, 0x6e, 0x64, 0x65, 0x64,
    0x18, 0x18, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0f, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x73, 0x44,
    0x65, 0x66, 0x65, 0x6e, 0x64, 0x65, 0x64, 0x12, 0x28, 0x0a, 0x10, 0x65, 0x67, 0x67, 0x5f, 0x69,
    0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x19, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x0e, 0x65, 0x67, 0x67, 0x49, 0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x49,
    0x64, 0x12, 0x28, 0x0a, 0x10, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x74, 0x69,
    0x6d, 0x65, 0x5f, 0x6d, 0x73, 0x18, 0x1a, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0e, 0x63, 0x72, 0x65,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x69, 0x6d, 0x65, 0x4d, 0x73, 0x12, 0x21, 0x0a, 0x0c, 0x6e,
    0x75, 0x6d, 0x5f, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x73, 0x18, 0x1b, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x0b, 0x6e, 0x75, 0x6d, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x73, 0x12, 0x38,
    0x0a, 0x18, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x5f, 0x63, 0x70, 0x5f,
    0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x69, 0x65, 0x72, 0x18, 0x1c, 0x20, 0x01, 0x28, 0x02,
    0x52, 0x16, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x43, 0x70, 0x4d, 0x75,
    0x6c, 0x74, 0x69, 0x70, 0x6c, 0x69, 0x65, 0x72, 0x12, 0x1a, 0x0a, 0x08, 0x66, 0x61, 0x76, 0x6f,
    0x72, 0x69, 0x74, 0x65, 0x18, 0x1d, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x66, 0x61, 0x76, 0x6f,
    0x72, 0x69, 0x74, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x6e, 0x69, 0x63, 0x6b, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x1e, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x6e, 0x69, 0x63, 0x6b, 0x6e, 0x61, 0x6d, 0x65,
    0x12, 0x1b, 0x0a, 0x09, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x66, 0x6f, 0x72, 0x74, 0x18, 0x1f, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x08, 0x66, 0x72, 0x6f, 0x6d, 0x46, 0x6f, 0x72, 0x74, 0x22, 0xc0, 0x01,
    0x0a, 0x0b, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x42, 0x61, 0x64, 0x67, 0x65, 0x12, 0x3a, 0x0a,
    0x0a, 0x62, 0x61, 0x64, 0x67, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45,
    0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x42, 0x61, 0x64, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x09,
    0x62, 0x61, 0x64, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x72, 0x61, 0x6e,
    0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x72, 0x61, 0x6e, 0x6b, 0x12, 0x1f, 0x0a,
    0x0b, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x72, 0x74, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x1b,
    0x0a, 0x09, 0x65, 0x6e, 0x64, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x08, 0x65, 0x6e, 0x64, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x23, 0x0a, 0x0d, 0x63,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x01, 0x52, 0x0c, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x56, 0x61, 0x6c, 0x75, 0x65,
    0x22, 0x6f, 0x0a, 0x10, 0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x55, 0x72, 0x6c, 0x45,
    0x6e, 0x74, 0x72, 0x79, 0x12, 0x19, 0x0a, 0x08, 0x61, 0x73, 0x73, 0x65, 0x74, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x73, 0x73, 0x65, 0x74, 0x49, 0x64, 0x12,
    0x10, 0x0a, 0x03, 0x75, 0x72, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x75, 0x72,
    0x6c, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x04, 0x73, 0x69, 0x7a, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75,
    0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x07, 0x52, 0x08, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75,
    0x6d, 0x22, 0xaa, 0x01, 0x0a, 0x10, 0x41, 0x73, 0x73, 0x65, 0x74, 0x44, 0x69, 0x67, 0x65, 0x73,
    0x74, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x19, 0x0a, 0x08, 0x61, 0x73, 0x73, 0x65, 0x74, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x73, 0x73, 0x65, 0x74, 0x49,
    0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x62, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x62, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x4e, 0x61,
    0x6d, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x03, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x1a, 0x0a, 0x08,
    0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x07, 0x52, 0x08,
    0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x12, 0x10, 0x0a, 0x03,
    0x6b, 0x65, 0x79, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x22, 0xff,
    0x01, 0x0a, 0x0c, 0x50, 0x6f, 0x6b, 0x65, 0x64, 0x65, 0x78, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12,
    0x3a, 0x0a, 0x0a, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64,
    0x52, 0x09, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x2b, 0x0a, 0x11, 0x74,
    0x69, 0x6d, 0x65, 0x73, 0x5f, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x10, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x45, 0x6e, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x64, 0x12, 0x25, 0x0a, 0x0e, 0x74, 0x69, 0x6d, 0x65,
    0x73, 0x5f, 0x63, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x0d, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x64, 0x12,
    0x34, 0x0a, 0x16, 0x65, 0x76, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x73, 0x74, 0x6f,
    0x6e, 0x65, 0x5f, 0x70, 0x69, 0x65, 0x63, 0x65, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x14, 0x65, 0x76, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x6f, 0x6e, 0x65, 0x50,
    0x69, 0x65, 0x63, 0x65, 0x73, 0x12, 0x29, 0x0a, 0x10, 0x65, 0x76, 0x6f, 0x6c, 0x75, 0x74, 0x69,
    0x6f, 0x6e, 0x5f, 0x73, 0x74, 0x6f, 0x6e, 0x65, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x0f, 0x65, 0x76, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x6f, 0x6e, 0x65, 0x73,
    0x22, 0x9a, 0x05, 0x0a, 0x0a, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x44, 0x61, 0x74, 0x61, 0x12,
    0x32, 0x0a, 0x15, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x74, 0x69, 0x6d, 0x65,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x13,
    0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d,
    0x70, 0x4d, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x12,
    0x2f, 0x0a, 0x04, 0x74, 0x65, 0x61, 0x6d, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73,
    0x2e, 0x54, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6c, 0x6f, 0x72, 0x52, 0x04, 0x74, 0x65, 0x61, 0x6d,
    0x12, 0x4a, 0x0a, 0x0e, 0x74, 0x75, 0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c, 0x5f, 0x73, 0x74, 0x61,
    0x74, 0x65, 0x18, 0x07, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x1f, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x54, 0x75, 0x74, 0x6f,
    0x72, 0x69, 0x61, 0x6c, 0x53, 0x74, 0x61, 0x74, 0x65, 0x42, 0x02, 0x10, 0x01, 0x52, 0x0d, 0x74,
    0x75, 0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x3c, 0x0a, 0x06,
    0x61, 0x76, 0x61, 0x74, 0x61, 0x72, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x41, 0x76, 0x61, 0x74,
    0x61, 0x72, 0x52, 0x06, 0x61, 0x76, 0x61, 0x74, 0x61, 0x72, 0x12, 0x2e, 0x0a, 0x13, 0x6d, 0x61,
    0x78, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67,
    0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x05, 0x52, 0x11, 0x6d, 0x61, 0x78, 0x50, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x12, 0x28, 0x0a, 0x10, 0x6d, 0x61,
    0x78, 0x5f, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x18, 0x0a,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x0e, 0x6d, 0x61, 0x78, 0x49, 0x74, 0x65, 0x6d, 0x53, 0x74, 0x6f,
    0x72, 0x61, 0x67, 0x65, 0x12, 0x43, 0x0a, 0x0b, 0x64, 0x61, 0x69, 0x6c, 0x79, 0x5f, 0x62, 0x6f,
    0x6e, 0x75, 0x73, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x2e, 0x44, 0x61, 0x69, 0x6c, 0x79, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x52, 0x0a, 0x64,
    0x61, 0x69, 0x6c, 0x79, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x12, 0x4c, 0x0a, 0x0e, 0x65, 0x71, 0x75,
    0x69, 0x70, 0x70, 0x65, 0x64, 0x5f, 0x62, 0x61, 0x64, 0x67, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x25, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44,
    0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x45, 0x71, 0x75, 0x69, 0x70,
    0x70, 0x65, 0x64, 0x42, 0x61, 0x64, 0x67, 0x65, 0x52, 0x0d, 0x65, 0x71, 0x75, 0x69, 0x70, 0x70,
    0x65, 0x64, 0x42, 0x61, 0x64, 0x67, 0x65, 0x12, 0x52, 0x0a, 0x10, 0x63, 0x6f, 0x6e, 0x74, 0x61,
    0x63, 0x74, 0x5f, 0x73, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x0d, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x27, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44,
    0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x61,
    0x63, 0x74, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x52, 0x0f, 0x63, 0x6f, 0x6e, 0x74,
    0x61, 0x63, 0x74, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x40, 0x0a, 0x0a, 0x63,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x69, 0x65, 0x73, 0x18, 0x0e, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x20, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63,
    0x79, 0x52, 0x0a, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x69, 0x65, 0x73, 0x4a, 0x84, 0x24,
    0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x4f, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x17, 0x0a, 0x09, 0x0a, 0x02,
    0x0a, 0x00, 0x12, 0x03, 0x03, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03,
    0x0e, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x04, 0x07, 0x0d, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x0e, 0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x02, 0x12, 0x03,
    0x05, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x05, 0x0e, 0x2c, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x07, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x08, 0x08, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x08, 0x08,
    0x07, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x08, 0x08, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x10, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x09, 0x08, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x09, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x09, 0x24, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x09,
    0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0a, 0x08, 0x15, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x0a, 0x08, 0x09, 0x33, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0a, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x0e, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x0b, 0x08, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04,
    0x0b, 0x08, 0x0a, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0b,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0b, 0x0e, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0b, 0x18, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0c, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x04, 0x12, 0x04, 0x0c, 0x08, 0x0b, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x0c, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x0c, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x0c, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x08,
    0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x04, 0x0d, 0x08, 0x0c, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x0d, 0x08, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0d, 0x26, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0d, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x06, 0x12, 0x03, 0x0e, 0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04,
    0x12, 0x04, 0x0e, 0x08, 0x0d, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12,
    0x03, 0x0e, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0e,
    0x26, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x0e, 0x2f, 0x30,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x0f, 0x08, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x04, 0x0f, 0x08, 0x0e, 0x31, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x0f, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x01, 0x12, 0x03, 0x0f, 0x0f, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x03, 0x12, 0x03, 0x0f, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03,
    0x10, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x04, 0x10, 0x08,
    0x0f, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x10, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x10, 0x0f, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x10, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x11, 0x08, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x09, 0x04, 0x12, 0x04, 0x11, 0x08, 0x10, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09,
    0x05, 0x12, 0x03, 0x11, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12,
    0x03, 0x11, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x11,
    0x16, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x12, 0x08, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x12, 0x08, 0x11, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x12, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x12, 0x0f, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0a, 0x03, 0x12, 0x03, 0x12, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b,
    0x12, 0x03, 0x13, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x04,
    0x13, 0x08, 0x12, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x13,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x13, 0x0f, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x13, 0x25, 0x27, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x14, 0x08, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0c, 0x04, 0x12, 0x04, 0x14, 0x08, 0x13, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0c, 0x05, 0x12, 0x03, 0x14, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c,
    0x01, 0x12, 0x03, 0x14, 0x0e, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x03, 0x12,
    0x03, 0x14, 0x17, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x15, 0x08,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x04, 0x12, 0x04, 0x15, 0x08, 0x14, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x15, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x15, 0x0e, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x15, 0x19, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x0e, 0x12, 0x03, 0x16, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x04,
    0x12, 0x04, 0x16, 0x08, 0x15, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x05, 0x12,
    0x03, 0x16, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x16,
    0x0e, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x16, 0x1a, 0x1c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0f, 0x12, 0x03, 0x17, 0x08, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0f, 0x04, 0x12, 0x04, 0x17, 0x08, 0x16, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x17, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0f, 0x01, 0x12, 0x03, 0x17, 0x0e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f,
    0x03, 0x12, 0x03, 0x17, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x10, 0x12, 0x03,
    0x18, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x04, 0x12, 0x04, 0x18, 0x08,
    0x17, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x05, 0x12, 0x03, 0x18, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x01, 0x12, 0x03, 0x18, 0x0e, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x03, 0x12, 0x03, 0x18, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x11, 0x12, 0x03, 0x19, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x11, 0x04, 0x12, 0x04, 0x19, 0x08, 0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11,
    0x05, 0x12, 0x03, 0x19, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x01, 0x12,
    0x03, 0x19, 0x0e, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x03, 0x12, 0x03, 0x19,
    0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x12, 0x12, 0x03, 0x1a, 0x08, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x04, 0x12, 0x04, 0x1a, 0x08, 0x19, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x05, 0x12, 0x03, 0x1a, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x12, 0x01, 0x12, 0x03, 0x1a, 0x0e, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x12, 0x03, 0x12, 0x03, 0x1a, 0x1e, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x13,
    0x12, 0x03, 0x1b, 0x08, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x04, 0x12, 0x04,
    0x1b, 0x08, 0x1a, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x06, 0x12, 0x03, 0x1b,
    0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x01, 0x12, 0x03, 0x1b, 0x2a, 0x32,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x03, 0x12, 0x03, 0x1b, 0x35, 0x37, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x14, 0x12, 0x03, 0x1c, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x14, 0x04, 0x12, 0x04, 0x1c, 0x08, 0x1b, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x14, 0x05, 0x12, 0x03, 0x1c, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x14,
    0x01, 0x12, 0x03, 0x1c, 0x0f, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x14, 0x03, 0x12,
    0x03, 0x1c, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x15, 0x12, 0x03, 0x1d, 0x08,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x15, 0x04, 0x12, 0x04, 0x1d, 0x08, 0x1c, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x15, 0x05, 0x12, 0x03, 0x1d, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x15, 0x01, 0x12, 0x03, 0x1d, 0x0e, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x15, 0x03, 0x12, 0x03, 0x1d, 0x21, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x16, 0x12, 0x03, 0x1e, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x16, 0x04,
    0x12, 0x04, 0x1e, 0x08, 0x1d, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x16, 0x05, 0x12,
    0x03, 0x1e, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x16, 0x01, 0x12, 0x03, 0x1e,
    0x0e, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x16, 0x03, 0x12, 0x03, 0x1e, 0x21, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x17, 0x12, 0x03, 0x1f, 0x08, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x17, 0x04, 0x12, 0x04, 0x1f, 0x08, 0x1e, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x17, 0x05, 0x12, 0x03, 0x1f, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x17, 0x01, 0x12, 0x03, 0x1f, 0x0f, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x17,
    0x03, 0x12, 0x03, 0x1f, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x18, 0x12, 0x03,
    0x20, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x18, 0x04, 0x12, 0x04, 0x20, 0x08,
    0x1f, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x18, 0x05, 0x12, 0x03, 0x20, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x18, 0x01, 0x12, 0x03, 0x20, 0x0f, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x18, 0x03, 0x12, 0x03, 0x20, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x19, 0x12, 0x03, 0x21, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x19, 0x04, 0x12, 0x04, 0x21, 0x08, 0x20, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x19,
    0x05, 0x12, 0x03, 0x21, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x19, 0x01, 0x12,
    0x03, 0x21, 0x0e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x19, 0x03, 0x12, 0x03, 0x21,
    0x1d, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x1a, 0x12, 0x03, 0x22, 0x08, 0x2c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1a, 0x04, 0x12, 0x04, 0x22, 0x08, 0x21, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x1a, 0x05, 0x12, 0x03, 0x22, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x1a, 0x01, 0x12, 0x03, 0x22, 0x0e, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x1a, 0x03, 0x12, 0x03, 0x22, 0x29, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x1b,
    0x12, 0x03, 0x23, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1b, 0x04, 0x12, 0x04,
    0x23, 0x08, 0x22, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1b, 0x05, 0x12, 0x03, 0x23,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1b, 0x01, 0x12, 0x03, 0x23, 0x0e, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1b, 0x03, 0x12, 0x03, 0x23, 0x19, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x1c, 0x12, 0x03, 0x24, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x1c, 0x04, 0x12, 0x04, 0x24, 0x08, 0x23, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x1c, 0x05, 0x12, 0x03, 0x24, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1c,
    0x01, 0x12, 0x03, 0x24, 0x0f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1c, 0x03, 0x12,
    0x03, 0x24, 0x1a, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x1d, 0x12, 0x03, 0x25, 0x08,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1d, 0x04, 0x12, 0x04, 0x25, 0x08, 0x24, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1d, 0x05, 0x12, 0x03, 0x25, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x1d, 0x01, 0x12, 0x03, 0x25, 0x0e, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x1d, 0x03, 0x12, 0x03, 0x25, 0x1a, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x27, 0x00, 0x2d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x27,
    0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x28, 0x08, 0x33, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x28, 0x08, 0x27, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x28, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x28, 0x24, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x28, 0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x29, 0x08, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x29, 0x08, 0x28, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x29,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x0e, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x29, 0x15, 0x16, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x2a, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0x2a, 0x08, 0x29, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x2a, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x2a, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x2a, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x2b, 0x08,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x04, 0x2b, 0x08, 0x2a, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x2b, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2b, 0x0e, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2b, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x04, 0x12, 0x03, 0x2c, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x2c, 0x08, 0x2b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x2c, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2c,
    0x0f, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2c, 0x1f, 0x20,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x2e, 0x00, 0x33, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00,
    0x12, 0x03, 0x2f, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x2f, 0x08, 0x2e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2f,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x0f, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2f, 0x1a, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x30, 0x08, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x30, 0x08, 0x2f, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x30, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x30, 0x0f, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x30, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x31, 0x08,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x04, 0x31, 0x08, 0x30, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x31, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x31, 0x0e, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x31, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x32, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04,
    0x12, 0x04, 0x32, 0x08, 0x31, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x32, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x32,
    0x10, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x32, 0x1b, 0x1c,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x34, 0x00, 0x3b, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x03, 0x01, 0x12, 0x03, 0x34, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00,
    0x12, 0x03, 0x35, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x35, 0x08, 0x34, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x35,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x0f, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x35, 0x1a, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x36, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x04, 0x12, 0x04, 0x36, 0x08, 0x35, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x36, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x36, 0x0f, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x36, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x37, 0x08,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x04, 0x37, 0x08, 0x36, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x37, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x37, 0x0e, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x37, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x03, 0x12, 0x03, 0x38, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04,
    0x12, 0x04, 0x38, 0x08, 0x37, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x38, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x38,
    0x10, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x38, 0x1b, 0x1c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x39, 0x08, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x04, 0x39, 0x08, 0x38, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x39, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x39, 0x0e, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x39, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x05, 0x12, 0x03,
    0x3a, 0x08, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x04, 0x12, 0x04, 0x3a, 0x08,
    0x39, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x05, 0x12, 0x03, 0x3a, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12, 0x03, 0x3a, 0x0e, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x03, 0x12, 0x03, 0x3a, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x3c, 0x00, 0x42, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12,
    0x03, 0x3c, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x3d, 0x08,
    0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x3d, 0x08, 0x3c, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3d, 0x08, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3d, 0x24, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3d, 0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x3e, 0x08, 0x3d, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x3e, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3e,
    0x0e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3e, 0x22, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x3f, 0x08, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x04, 0x3f, 0x08, 0x3e, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3f, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x0e, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x3f, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03,
    0x40, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12, 0x04, 0x40, 0x08,
    0x3f, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05, 0x12, 0x03, 0x40, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x40, 0x0e, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x40, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x41, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x41, 0x08, 0x40, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x41, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x41, 0x0e, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12, 0x03, 0x41,
    0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x43, 0x00, 0x4f, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x43, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x00, 0x12, 0x03, 0x44, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x44, 0x08, 0x43, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x44, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x44,
    0x0e, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x44, 0x27, 0x28,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x45, 0x08, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x04, 0x45, 0x08, 0x44, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x45, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x45, 0x0f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x45, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03,
    0x46, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x04, 0x46, 0x08,
    0x45, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x46, 0x08, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x46, 0x24, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x46, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x47, 0x08, 0x52, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x47, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x06,
    0x12, 0x03, 0x47, 0x11, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x47, 0x31, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x47, 0x42,
    0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x08, 0x12, 0x03, 0x47, 0x44, 0x51, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x05, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x47, 0x45, 0x50,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x05, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x47,
    0x45, 0x4b, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x05, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x47, 0x45, 0x4b, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x05, 0x02, 0x03, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x47, 0x45, 0x4b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x05, 0x02,
    0x03, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x47, 0x4c, 0x50, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x04, 0x12, 0x03, 0x48, 0x08, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04,
    0x04, 0x12, 0x04, 0x48, 0x08, 0x47, 0x52, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x06,
    0x12, 0x03, 0x48, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x48, 0x2d, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x03, 0x48, 0x36,
    0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x05, 0x12, 0x03, 0x49, 0x08, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x04, 0x12, 0x04, 0x49, 0x08, 0x48, 0x38, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x05, 0x05, 0x12, 0x03, 0x49, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x05, 0x01, 0x12, 0x03, 0x49, 0x0e, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x49, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x06, 0x12,
    0x03, 0x4a, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x04, 0x12, 0x04, 0x4a,
    0x08, 0x49, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x05, 0x12, 0x03, 0x4a, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x01, 0x12, 0x03, 0x4a, 0x0e, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x03, 0x12, 0x03, 0x4a, 0x21, 0x23, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x07, 0x12, 0x03, 0x4b, 0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x07, 0x04, 0x12, 0x04, 0x4b, 0x08, 0x4a, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x07, 0x06, 0x12, 0x03, 0x4b, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x4b, 0x2b, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x03, 0x12, 0x03,
    0x4b, 0x39, 0x3b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x08, 0x12, 0x03, 0x4c, 0x08, 0x42,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x04, 0x12, 0x04, 0x4c, 0x08, 0x4b, 0x3c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x06, 0x12, 0x03, 0x4c, 0x08, 0x2d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x08, 0x01, 0x12, 0x03, 0x4c, 0x2e, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x08, 0x03, 0x12, 0x03, 0x4c, 0x3f, 0x41, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x09, 0x12, 0x03, 0x4d, 0x08, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x04, 0x12,
    0x04, 0x4d, 0x08, 0x4c, 0x42, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x06, 0x12, 0x03,
    0x4d, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x01, 0x12, 0x03, 0x4d, 0x30,
    0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x03, 0x12, 0x03, 0x4d, 0x43, 0x45, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0a, 0x12, 0x03, 0x4e, 0x08, 0x42, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x4e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x0a, 0x06, 0x12, 0x03, 0x4e, 0x11, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0a,
    0x01, 0x12, 0x03, 0x4e, 0x32, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0a, 0x03, 0x12,
    0x03, 0x4e, 0x3f, 0x41, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
