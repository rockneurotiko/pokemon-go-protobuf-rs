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
pub struct BattleParticipant {
    // message fields
    active_pokemon: ::protobuf::SingularPtrField<BattlePokemonInfo>,
    trainer_public_profile: ::protobuf::SingularPtrField<super::POGOProtos_Data_Player::PlayerPublicProfile>,
    reverse_pokemon: ::protobuf::RepeatedField<BattlePokemonInfo>,
    defeated_pokemon: ::protobuf::RepeatedField<BattlePokemonInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BattleParticipant {}

impl BattleParticipant {
    pub fn new() -> BattleParticipant {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BattleParticipant {
        static mut instance: ::protobuf::lazy::Lazy<BattleParticipant> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BattleParticipant,
        };
        unsafe {
            instance.get(|| {
                BattleParticipant {
                    active_pokemon: ::protobuf::SingularPtrField::none(),
                    trainer_public_profile: ::protobuf::SingularPtrField::none(),
                    reverse_pokemon: ::protobuf::RepeatedField::new(),
                    defeated_pokemon: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Data.Battle.BattlePokemonInfo active_pokemon = 1;

    pub fn clear_active_pokemon(&mut self) {
        self.active_pokemon.clear();
    }

    pub fn has_active_pokemon(&self) -> bool {
        self.active_pokemon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_active_pokemon(&mut self, v: BattlePokemonInfo) {
        self.active_pokemon = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_active_pokemon(&mut self) -> &mut BattlePokemonInfo {
        if self.active_pokemon.is_none() {
            self.active_pokemon.set_default();
        };
        self.active_pokemon.as_mut().unwrap()
    }

    // Take field
    pub fn take_active_pokemon(&mut self) -> BattlePokemonInfo {
        self.active_pokemon.take().unwrap_or_else(|| BattlePokemonInfo::new())
    }

    pub fn get_active_pokemon(&self) -> &BattlePokemonInfo {
        self.active_pokemon.as_ref().unwrap_or_else(|| BattlePokemonInfo::default_instance())
    }

    // optional .POGOProtos.Data.Player.PlayerPublicProfile trainer_public_profile = 2;

    pub fn clear_trainer_public_profile(&mut self) {
        self.trainer_public_profile.clear();
    }

    pub fn has_trainer_public_profile(&self) -> bool {
        self.trainer_public_profile.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trainer_public_profile(&mut self, v: super::POGOProtos_Data_Player::PlayerPublicProfile) {
        self.trainer_public_profile = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_trainer_public_profile(&mut self) -> &mut super::POGOProtos_Data_Player::PlayerPublicProfile {
        if self.trainer_public_profile.is_none() {
            self.trainer_public_profile.set_default();
        };
        self.trainer_public_profile.as_mut().unwrap()
    }

    // Take field
    pub fn take_trainer_public_profile(&mut self) -> super::POGOProtos_Data_Player::PlayerPublicProfile {
        self.trainer_public_profile.take().unwrap_or_else(|| super::POGOProtos_Data_Player::PlayerPublicProfile::new())
    }

    pub fn get_trainer_public_profile(&self) -> &super::POGOProtos_Data_Player::PlayerPublicProfile {
        self.trainer_public_profile.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Player::PlayerPublicProfile::default_instance())
    }

    // repeated .POGOProtos.Data.Battle.BattlePokemonInfo reverse_pokemon = 3;

    pub fn clear_reverse_pokemon(&mut self) {
        self.reverse_pokemon.clear();
    }

    // Param is passed by value, moved
    pub fn set_reverse_pokemon(&mut self, v: ::protobuf::RepeatedField<BattlePokemonInfo>) {
        self.reverse_pokemon = v;
    }

    // Mutable pointer to the field.
    pub fn mut_reverse_pokemon(&mut self) -> &mut ::protobuf::RepeatedField<BattlePokemonInfo> {
        &mut self.reverse_pokemon
    }

    // Take field
    pub fn take_reverse_pokemon(&mut self) -> ::protobuf::RepeatedField<BattlePokemonInfo> {
        ::std::mem::replace(&mut self.reverse_pokemon, ::protobuf::RepeatedField::new())
    }

    pub fn get_reverse_pokemon(&self) -> &[BattlePokemonInfo] {
        &self.reverse_pokemon
    }

    // repeated .POGOProtos.Data.Battle.BattlePokemonInfo defeated_pokemon = 4;

    pub fn clear_defeated_pokemon(&mut self) {
        self.defeated_pokemon.clear();
    }

    // Param is passed by value, moved
    pub fn set_defeated_pokemon(&mut self, v: ::protobuf::RepeatedField<BattlePokemonInfo>) {
        self.defeated_pokemon = v;
    }

    // Mutable pointer to the field.
    pub fn mut_defeated_pokemon(&mut self) -> &mut ::protobuf::RepeatedField<BattlePokemonInfo> {
        &mut self.defeated_pokemon
    }

    // Take field
    pub fn take_defeated_pokemon(&mut self) -> ::protobuf::RepeatedField<BattlePokemonInfo> {
        ::std::mem::replace(&mut self.defeated_pokemon, ::protobuf::RepeatedField::new())
    }

    pub fn get_defeated_pokemon(&self) -> &[BattlePokemonInfo] {
        &self.defeated_pokemon
    }
}

impl ::protobuf::Message for BattleParticipant {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.active_pokemon));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.trainer_public_profile));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.reverse_pokemon));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.defeated_pokemon));
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
        for value in &self.active_pokemon {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.trainer_public_profile {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.reverse_pokemon {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.defeated_pokemon {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.active_pokemon.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.trainer_public_profile.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.reverse_pokemon {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.defeated_pokemon {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<BattleParticipant>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BattleParticipant {
    fn new() -> BattleParticipant {
        BattleParticipant::new()
    }

    fn descriptor_static(_: ::std::option::Option<BattleParticipant>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "active_pokemon",
                    BattleParticipant::has_active_pokemon,
                    BattleParticipant::get_active_pokemon,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "trainer_public_profile",
                    BattleParticipant::has_trainer_public_profile,
                    BattleParticipant::get_trainer_public_profile,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "reverse_pokemon",
                    BattleParticipant::get_reverse_pokemon,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "defeated_pokemon",
                    BattleParticipant::get_defeated_pokemon,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BattleParticipant>(
                    "BattleParticipant",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BattleParticipant {
    fn clear(&mut self) {
        self.clear_active_pokemon();
        self.clear_trainer_public_profile();
        self.clear_reverse_pokemon();
        self.clear_defeated_pokemon();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BattleParticipant {
    fn eq(&self, other: &BattleParticipant) -> bool {
        self.active_pokemon == other.active_pokemon &&
        self.trainer_public_profile == other.trainer_public_profile &&
        self.reverse_pokemon == other.reverse_pokemon &&
        self.defeated_pokemon == other.defeated_pokemon &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BattleParticipant {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BattlePokemonInfo {
    // message fields
    pokemon_data: ::protobuf::SingularPtrField<super::POGOProtos_Data::PokemonData>,
    current_health: ::std::option::Option<i32>,
    current_energy: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BattlePokemonInfo {}

impl BattlePokemonInfo {
    pub fn new() -> BattlePokemonInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BattlePokemonInfo {
        static mut instance: ::protobuf::lazy::Lazy<BattlePokemonInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BattlePokemonInfo,
        };
        unsafe {
            instance.get(|| {
                BattlePokemonInfo {
                    pokemon_data: ::protobuf::SingularPtrField::none(),
                    current_health: ::std::option::Option::None,
                    current_energy: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Data.PokemonData pokemon_data = 1;

    pub fn clear_pokemon_data(&mut self) {
        self.pokemon_data.clear();
    }

    pub fn has_pokemon_data(&self) -> bool {
        self.pokemon_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_data(&mut self, v: super::POGOProtos_Data::PokemonData) {
        self.pokemon_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pokemon_data(&mut self) -> &mut super::POGOProtos_Data::PokemonData {
        if self.pokemon_data.is_none() {
            self.pokemon_data.set_default();
        };
        self.pokemon_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_pokemon_data(&mut self) -> super::POGOProtos_Data::PokemonData {
        self.pokemon_data.take().unwrap_or_else(|| super::POGOProtos_Data::PokemonData::new())
    }

    pub fn get_pokemon_data(&self) -> &super::POGOProtos_Data::PokemonData {
        self.pokemon_data.as_ref().unwrap_or_else(|| super::POGOProtos_Data::PokemonData::default_instance())
    }

    // optional int32 current_health = 2;

    pub fn clear_current_health(&mut self) {
        self.current_health = ::std::option::Option::None;
    }

    pub fn has_current_health(&self) -> bool {
        self.current_health.is_some()
    }

    // Param is passed by value, moved
    pub fn set_current_health(&mut self, v: i32) {
        self.current_health = ::std::option::Option::Some(v);
    }

    pub fn get_current_health(&self) -> i32 {
        self.current_health.unwrap_or(0)
    }

    // optional int32 current_energy = 3;

    pub fn clear_current_energy(&mut self) {
        self.current_energy = ::std::option::Option::None;
    }

    pub fn has_current_energy(&self) -> bool {
        self.current_energy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_current_energy(&mut self, v: i32) {
        self.current_energy = ::std::option::Option::Some(v);
    }

    pub fn get_current_energy(&self) -> i32 {
        self.current_energy.unwrap_or(0)
    }
}

impl ::protobuf::Message for BattlePokemonInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pokemon_data));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.current_health = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.current_energy = ::std::option::Option::Some(tmp);
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
        for value in &self.pokemon_data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.current_health {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.current_energy {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pokemon_data.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.current_health {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.current_energy {
            try!(os.write_int32(3, v));
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
        ::std::any::TypeId::of::<BattlePokemonInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BattlePokemonInfo {
    fn new() -> BattlePokemonInfo {
        BattlePokemonInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<BattlePokemonInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pokemon_data",
                    BattlePokemonInfo::has_pokemon_data,
                    BattlePokemonInfo::get_pokemon_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "current_health",
                    BattlePokemonInfo::has_current_health,
                    BattlePokemonInfo::get_current_health,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "current_energy",
                    BattlePokemonInfo::has_current_energy,
                    BattlePokemonInfo::get_current_energy,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BattlePokemonInfo>(
                    "BattlePokemonInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BattlePokemonInfo {
    fn clear(&mut self) {
        self.clear_pokemon_data();
        self.clear_current_health();
        self.clear_current_energy();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BattlePokemonInfo {
    fn eq(&self, other: &BattlePokemonInfo) -> bool {
        self.pokemon_data == other.pokemon_data &&
        self.current_health == other.current_health &&
        self.current_energy == other.current_energy &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BattlePokemonInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BattleAction {
    // message fields
    Type: ::std::option::Option<BattleActionType>,
    action_start_ms: ::std::option::Option<i64>,
    duration_ms: ::std::option::Option<i32>,
    energy_delta: ::std::option::Option<i32>,
    attacker_index: ::std::option::Option<i32>,
    target_index: ::std::option::Option<i32>,
    active_pokemon_id: ::std::option::Option<u64>,
    player_joined: ::protobuf::SingularPtrField<BattleParticipant>,
    battle_results: ::protobuf::SingularPtrField<BattleResults>,
    damage_windows_start_timestamp_mss: ::std::option::Option<i64>,
    damage_windows_end_timestamp_mss: ::std::option::Option<i64>,
    player_left: ::protobuf::SingularPtrField<BattleParticipant>,
    target_pokemon_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BattleAction {}

impl BattleAction {
    pub fn new() -> BattleAction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BattleAction {
        static mut instance: ::protobuf::lazy::Lazy<BattleAction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BattleAction,
        };
        unsafe {
            instance.get(|| {
                BattleAction {
                    Type: ::std::option::Option::None,
                    action_start_ms: ::std::option::Option::None,
                    duration_ms: ::std::option::Option::None,
                    energy_delta: ::std::option::Option::None,
                    attacker_index: ::std::option::Option::None,
                    target_index: ::std::option::Option::None,
                    active_pokemon_id: ::std::option::Option::None,
                    player_joined: ::protobuf::SingularPtrField::none(),
                    battle_results: ::protobuf::SingularPtrField::none(),
                    damage_windows_start_timestamp_mss: ::std::option::Option::None,
                    damage_windows_end_timestamp_mss: ::std::option::Option::None,
                    player_left: ::protobuf::SingularPtrField::none(),
                    target_pokemon_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Data.Battle.BattleActionType Type = 1;

    pub fn clear_Type(&mut self) {
        self.Type = ::std::option::Option::None;
    }

    pub fn has_Type(&self) -> bool {
        self.Type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Type(&mut self, v: BattleActionType) {
        self.Type = ::std::option::Option::Some(v);
    }

    pub fn get_Type(&self) -> BattleActionType {
        self.Type.unwrap_or(BattleActionType::ACTION_UNSET)
    }

    // optional int64 action_start_ms = 2;

    pub fn clear_action_start_ms(&mut self) {
        self.action_start_ms = ::std::option::Option::None;
    }

    pub fn has_action_start_ms(&self) -> bool {
        self.action_start_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action_start_ms(&mut self, v: i64) {
        self.action_start_ms = ::std::option::Option::Some(v);
    }

    pub fn get_action_start_ms(&self) -> i64 {
        self.action_start_ms.unwrap_or(0)
    }

    // optional int32 duration_ms = 3;

    pub fn clear_duration_ms(&mut self) {
        self.duration_ms = ::std::option::Option::None;
    }

    pub fn has_duration_ms(&self) -> bool {
        self.duration_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration_ms(&mut self, v: i32) {
        self.duration_ms = ::std::option::Option::Some(v);
    }

    pub fn get_duration_ms(&self) -> i32 {
        self.duration_ms.unwrap_or(0)
    }

    // optional int32 energy_delta = 5;

    pub fn clear_energy_delta(&mut self) {
        self.energy_delta = ::std::option::Option::None;
    }

    pub fn has_energy_delta(&self) -> bool {
        self.energy_delta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_energy_delta(&mut self, v: i32) {
        self.energy_delta = ::std::option::Option::Some(v);
    }

    pub fn get_energy_delta(&self) -> i32 {
        self.energy_delta.unwrap_or(0)
    }

    // optional int32 attacker_index = 6;

    pub fn clear_attacker_index(&mut self) {
        self.attacker_index = ::std::option::Option::None;
    }

    pub fn has_attacker_index(&self) -> bool {
        self.attacker_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attacker_index(&mut self, v: i32) {
        self.attacker_index = ::std::option::Option::Some(v);
    }

    pub fn get_attacker_index(&self) -> i32 {
        self.attacker_index.unwrap_or(0)
    }

    // optional int32 target_index = 7;

    pub fn clear_target_index(&mut self) {
        self.target_index = ::std::option::Option::None;
    }

    pub fn has_target_index(&self) -> bool {
        self.target_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_index(&mut self, v: i32) {
        self.target_index = ::std::option::Option::Some(v);
    }

    pub fn get_target_index(&self) -> i32 {
        self.target_index.unwrap_or(0)
    }

    // optional fixed64 active_pokemon_id = 8;

    pub fn clear_active_pokemon_id(&mut self) {
        self.active_pokemon_id = ::std::option::Option::None;
    }

    pub fn has_active_pokemon_id(&self) -> bool {
        self.active_pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_active_pokemon_id(&mut self, v: u64) {
        self.active_pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_active_pokemon_id(&self) -> u64 {
        self.active_pokemon_id.unwrap_or(0)
    }

    // optional .POGOProtos.Data.Battle.BattleParticipant player_joined = 9;

    pub fn clear_player_joined(&mut self) {
        self.player_joined.clear();
    }

    pub fn has_player_joined(&self) -> bool {
        self.player_joined.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_joined(&mut self, v: BattleParticipant) {
        self.player_joined = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_joined(&mut self) -> &mut BattleParticipant {
        if self.player_joined.is_none() {
            self.player_joined.set_default();
        };
        self.player_joined.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_joined(&mut self) -> BattleParticipant {
        self.player_joined.take().unwrap_or_else(|| BattleParticipant::new())
    }

    pub fn get_player_joined(&self) -> &BattleParticipant {
        self.player_joined.as_ref().unwrap_or_else(|| BattleParticipant::default_instance())
    }

    // optional .POGOProtos.Data.Battle.BattleResults battle_results = 10;

    pub fn clear_battle_results(&mut self) {
        self.battle_results.clear();
    }

    pub fn has_battle_results(&self) -> bool {
        self.battle_results.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battle_results(&mut self, v: BattleResults) {
        self.battle_results = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_battle_results(&mut self) -> &mut BattleResults {
        if self.battle_results.is_none() {
            self.battle_results.set_default();
        };
        self.battle_results.as_mut().unwrap()
    }

    // Take field
    pub fn take_battle_results(&mut self) -> BattleResults {
        self.battle_results.take().unwrap_or_else(|| BattleResults::new())
    }

    pub fn get_battle_results(&self) -> &BattleResults {
        self.battle_results.as_ref().unwrap_or_else(|| BattleResults::default_instance())
    }

    // optional int64 damage_windows_start_timestamp_mss = 11;

    pub fn clear_damage_windows_start_timestamp_mss(&mut self) {
        self.damage_windows_start_timestamp_mss = ::std::option::Option::None;
    }

    pub fn has_damage_windows_start_timestamp_mss(&self) -> bool {
        self.damage_windows_start_timestamp_mss.is_some()
    }

    // Param is passed by value, moved
    pub fn set_damage_windows_start_timestamp_mss(&mut self, v: i64) {
        self.damage_windows_start_timestamp_mss = ::std::option::Option::Some(v);
    }

    pub fn get_damage_windows_start_timestamp_mss(&self) -> i64 {
        self.damage_windows_start_timestamp_mss.unwrap_or(0)
    }

    // optional int64 damage_windows_end_timestamp_mss = 12;

    pub fn clear_damage_windows_end_timestamp_mss(&mut self) {
        self.damage_windows_end_timestamp_mss = ::std::option::Option::None;
    }

    pub fn has_damage_windows_end_timestamp_mss(&self) -> bool {
        self.damage_windows_end_timestamp_mss.is_some()
    }

    // Param is passed by value, moved
    pub fn set_damage_windows_end_timestamp_mss(&mut self, v: i64) {
        self.damage_windows_end_timestamp_mss = ::std::option::Option::Some(v);
    }

    pub fn get_damage_windows_end_timestamp_mss(&self) -> i64 {
        self.damage_windows_end_timestamp_mss.unwrap_or(0)
    }

    // optional .POGOProtos.Data.Battle.BattleParticipant player_left = 13;

    pub fn clear_player_left(&mut self) {
        self.player_left.clear();
    }

    pub fn has_player_left(&self) -> bool {
        self.player_left.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_left(&mut self, v: BattleParticipant) {
        self.player_left = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_left(&mut self) -> &mut BattleParticipant {
        if self.player_left.is_none() {
            self.player_left.set_default();
        };
        self.player_left.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_left(&mut self) -> BattleParticipant {
        self.player_left.take().unwrap_or_else(|| BattleParticipant::new())
    }

    pub fn get_player_left(&self) -> &BattleParticipant {
        self.player_left.as_ref().unwrap_or_else(|| BattleParticipant::default_instance())
    }

    // optional fixed64 target_pokemon_id = 14;

    pub fn clear_target_pokemon_id(&mut self) {
        self.target_pokemon_id = ::std::option::Option::None;
    }

    pub fn has_target_pokemon_id(&self) -> bool {
        self.target_pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_pokemon_id(&mut self, v: u64) {
        self.target_pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_target_pokemon_id(&self) -> u64 {
        self.target_pokemon_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for BattleAction {
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
                    self.Type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.action_start_ms = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.duration_ms = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.energy_delta = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.attacker_index = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.target_index = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.active_pokemon_id = ::std::option::Option::Some(tmp);
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_joined));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.battle_results));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.damage_windows_start_timestamp_mss = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.damage_windows_end_timestamp_mss = ::std::option::Option::Some(tmp);
                },
                13 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_left));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.target_pokemon_id = ::std::option::Option::Some(tmp);
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
        for value in &self.Type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.action_start_ms {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.duration_ms {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.energy_delta {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.attacker_index {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.target_index {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.active_pokemon_id.is_some() {
            my_size += 9;
        };
        for value in &self.player_joined {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.battle_results {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.damage_windows_start_timestamp_mss {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.damage_windows_end_timestamp_mss {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.player_left {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.target_pokemon_id.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.Type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.action_start_ms {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.duration_ms {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.energy_delta {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.attacker_index {
            try!(os.write_int32(6, v));
        };
        if let Some(v) = self.target_index {
            try!(os.write_int32(7, v));
        };
        if let Some(v) = self.active_pokemon_id {
            try!(os.write_fixed64(8, v));
        };
        if let Some(v) = self.player_joined.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.battle_results.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.damage_windows_start_timestamp_mss {
            try!(os.write_int64(11, v));
        };
        if let Some(v) = self.damage_windows_end_timestamp_mss {
            try!(os.write_int64(12, v));
        };
        if let Some(v) = self.player_left.as_ref() {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.target_pokemon_id {
            try!(os.write_fixed64(14, v));
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
        ::std::any::TypeId::of::<BattleAction>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BattleAction {
    fn new() -> BattleAction {
        BattleAction::new()
    }

    fn descriptor_static(_: ::std::option::Option<BattleAction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "Type",
                    BattleAction::has_Type,
                    BattleAction::get_Type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "action_start_ms",
                    BattleAction::has_action_start_ms,
                    BattleAction::get_action_start_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "duration_ms",
                    BattleAction::has_duration_ms,
                    BattleAction::get_duration_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "energy_delta",
                    BattleAction::has_energy_delta,
                    BattleAction::get_energy_delta,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "attacker_index",
                    BattleAction::has_attacker_index,
                    BattleAction::get_attacker_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "target_index",
                    BattleAction::has_target_index,
                    BattleAction::get_target_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "active_pokemon_id",
                    BattleAction::has_active_pokemon_id,
                    BattleAction::get_active_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "player_joined",
                    BattleAction::has_player_joined,
                    BattleAction::get_player_joined,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "battle_results",
                    BattleAction::has_battle_results,
                    BattleAction::get_battle_results,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "damage_windows_start_timestamp_mss",
                    BattleAction::has_damage_windows_start_timestamp_mss,
                    BattleAction::get_damage_windows_start_timestamp_mss,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "damage_windows_end_timestamp_mss",
                    BattleAction::has_damage_windows_end_timestamp_mss,
                    BattleAction::get_damage_windows_end_timestamp_mss,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "player_left",
                    BattleAction::has_player_left,
                    BattleAction::get_player_left,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "target_pokemon_id",
                    BattleAction::has_target_pokemon_id,
                    BattleAction::get_target_pokemon_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BattleAction>(
                    "BattleAction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BattleAction {
    fn clear(&mut self) {
        self.clear_Type();
        self.clear_action_start_ms();
        self.clear_duration_ms();
        self.clear_energy_delta();
        self.clear_attacker_index();
        self.clear_target_index();
        self.clear_active_pokemon_id();
        self.clear_player_joined();
        self.clear_battle_results();
        self.clear_damage_windows_start_timestamp_mss();
        self.clear_damage_windows_end_timestamp_mss();
        self.clear_player_left();
        self.clear_target_pokemon_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BattleAction {
    fn eq(&self, other: &BattleAction) -> bool {
        self.Type == other.Type &&
        self.action_start_ms == other.action_start_ms &&
        self.duration_ms == other.duration_ms &&
        self.energy_delta == other.energy_delta &&
        self.attacker_index == other.attacker_index &&
        self.target_index == other.target_index &&
        self.active_pokemon_id == other.active_pokemon_id &&
        self.player_joined == other.player_joined &&
        self.battle_results == other.battle_results &&
        self.damage_windows_start_timestamp_mss == other.damage_windows_start_timestamp_mss &&
        self.damage_windows_end_timestamp_mss == other.damage_windows_end_timestamp_mss &&
        self.player_left == other.player_left &&
        self.target_pokemon_id == other.target_pokemon_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BattleAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BattleResults {
    // message fields
    gym_state: ::protobuf::SingularPtrField<super::POGOProtos_Data_Gym::GymState>,
    attackers: ::protobuf::RepeatedField<BattleParticipant>,
    player_experience_awarded: ::std::vec::Vec<i32>,
    next_defender_pokemon_id: ::std::option::Option<i64>,
    gym_points_delta: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BattleResults {}

impl BattleResults {
    pub fn new() -> BattleResults {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BattleResults {
        static mut instance: ::protobuf::lazy::Lazy<BattleResults> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BattleResults,
        };
        unsafe {
            instance.get(|| {
                BattleResults {
                    gym_state: ::protobuf::SingularPtrField::none(),
                    attackers: ::protobuf::RepeatedField::new(),
                    player_experience_awarded: ::std::vec::Vec::new(),
                    next_defender_pokemon_id: ::std::option::Option::None,
                    gym_points_delta: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Data.Gym.GymState gym_state = 1;

    pub fn clear_gym_state(&mut self) {
        self.gym_state.clear();
    }

    pub fn has_gym_state(&self) -> bool {
        self.gym_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gym_state(&mut self, v: super::POGOProtos_Data_Gym::GymState) {
        self.gym_state = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gym_state(&mut self) -> &mut super::POGOProtos_Data_Gym::GymState {
        if self.gym_state.is_none() {
            self.gym_state.set_default();
        };
        self.gym_state.as_mut().unwrap()
    }

    // Take field
    pub fn take_gym_state(&mut self) -> super::POGOProtos_Data_Gym::GymState {
        self.gym_state.take().unwrap_or_else(|| super::POGOProtos_Data_Gym::GymState::new())
    }

    pub fn get_gym_state(&self) -> &super::POGOProtos_Data_Gym::GymState {
        self.gym_state.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Gym::GymState::default_instance())
    }

    // repeated .POGOProtos.Data.Battle.BattleParticipant attackers = 2;

    pub fn clear_attackers(&mut self) {
        self.attackers.clear();
    }

    // Param is passed by value, moved
    pub fn set_attackers(&mut self, v: ::protobuf::RepeatedField<BattleParticipant>) {
        self.attackers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attackers(&mut self) -> &mut ::protobuf::RepeatedField<BattleParticipant> {
        &mut self.attackers
    }

    // Take field
    pub fn take_attackers(&mut self) -> ::protobuf::RepeatedField<BattleParticipant> {
        ::std::mem::replace(&mut self.attackers, ::protobuf::RepeatedField::new())
    }

    pub fn get_attackers(&self) -> &[BattleParticipant] {
        &self.attackers
    }

    // repeated int32 player_experience_awarded = 3;

    pub fn clear_player_experience_awarded(&mut self) {
        self.player_experience_awarded.clear();
    }

    // Param is passed by value, moved
    pub fn set_player_experience_awarded(&mut self, v: ::std::vec::Vec<i32>) {
        self.player_experience_awarded = v;
    }

    // Mutable pointer to the field.
    pub fn mut_player_experience_awarded(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.player_experience_awarded
    }

    // Take field
    pub fn take_player_experience_awarded(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.player_experience_awarded, ::std::vec::Vec::new())
    }

    pub fn get_player_experience_awarded(&self) -> &[i32] {
        &self.player_experience_awarded
    }

    // optional int64 next_defender_pokemon_id = 4;

    pub fn clear_next_defender_pokemon_id(&mut self) {
        self.next_defender_pokemon_id = ::std::option::Option::None;
    }

    pub fn has_next_defender_pokemon_id(&self) -> bool {
        self.next_defender_pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_next_defender_pokemon_id(&mut self, v: i64) {
        self.next_defender_pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_next_defender_pokemon_id(&self) -> i64 {
        self.next_defender_pokemon_id.unwrap_or(0)
    }

    // optional int32 gym_points_delta = 5;

    pub fn clear_gym_points_delta(&mut self) {
        self.gym_points_delta = ::std::option::Option::None;
    }

    pub fn has_gym_points_delta(&self) -> bool {
        self.gym_points_delta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gym_points_delta(&mut self, v: i32) {
        self.gym_points_delta = ::std::option::Option::Some(v);
    }

    pub fn get_gym_points_delta(&self) -> i32 {
        self.gym_points_delta.unwrap_or(0)
    }
}

impl ::protobuf::Message for BattleResults {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.gym_state));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.attackers));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.player_experience_awarded));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.next_defender_pokemon_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.gym_points_delta = ::std::option::Option::Some(tmp);
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
        for value in &self.gym_state {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.attackers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.player_experience_awarded {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.next_defender_pokemon_id {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.gym_points_delta {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gym_state.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.attackers {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.player_experience_awarded {
            try!(os.write_int32(3, *v));
        };
        if let Some(v) = self.next_defender_pokemon_id {
            try!(os.write_int64(4, v));
        };
        if let Some(v) = self.gym_points_delta {
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
        ::std::any::TypeId::of::<BattleResults>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BattleResults {
    fn new() -> BattleResults {
        BattleResults::new()
    }

    fn descriptor_static(_: ::std::option::Option<BattleResults>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "gym_state",
                    BattleResults::has_gym_state,
                    BattleResults::get_gym_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "attackers",
                    BattleResults::get_attackers,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "player_experience_awarded",
                    BattleResults::get_player_experience_awarded,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "next_defender_pokemon_id",
                    BattleResults::has_next_defender_pokemon_id,
                    BattleResults::get_next_defender_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "gym_points_delta",
                    BattleResults::has_gym_points_delta,
                    BattleResults::get_gym_points_delta,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BattleResults>(
                    "BattleResults",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BattleResults {
    fn clear(&mut self) {
        self.clear_gym_state();
        self.clear_attackers();
        self.clear_player_experience_awarded();
        self.clear_next_defender_pokemon_id();
        self.clear_gym_points_delta();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BattleResults {
    fn eq(&self, other: &BattleResults) -> bool {
        self.gym_state == other.gym_state &&
        self.attackers == other.attackers &&
        self.player_experience_awarded == other.player_experience_awarded &&
        self.next_defender_pokemon_id == other.next_defender_pokemon_id &&
        self.gym_points_delta == other.gym_points_delta &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BattleResults {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BattleLog {
    // message fields
    state: ::std::option::Option<BattleState>,
    battle_type: ::std::option::Option<BattleType>,
    server_ms: ::std::option::Option<i64>,
    battle_actions: ::protobuf::RepeatedField<BattleAction>,
    battle_start_timestamp_ms: ::std::option::Option<i64>,
    battle_end_timestamp_ms: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BattleLog {}

impl BattleLog {
    pub fn new() -> BattleLog {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BattleLog {
        static mut instance: ::protobuf::lazy::Lazy<BattleLog> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BattleLog,
        };
        unsafe {
            instance.get(|| {
                BattleLog {
                    state: ::std::option::Option::None,
                    battle_type: ::std::option::Option::None,
                    server_ms: ::std::option::Option::None,
                    battle_actions: ::protobuf::RepeatedField::new(),
                    battle_start_timestamp_ms: ::std::option::Option::None,
                    battle_end_timestamp_ms: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Data.Battle.BattleState state = 1;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: BattleState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> BattleState {
        self.state.unwrap_or(BattleState::STATE_UNSET)
    }

    // optional .POGOProtos.Data.Battle.BattleType battle_type = 2;

    pub fn clear_battle_type(&mut self) {
        self.battle_type = ::std::option::Option::None;
    }

    pub fn has_battle_type(&self) -> bool {
        self.battle_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battle_type(&mut self, v: BattleType) {
        self.battle_type = ::std::option::Option::Some(v);
    }

    pub fn get_battle_type(&self) -> BattleType {
        self.battle_type.unwrap_or(BattleType::BATTLE_TYPE_UNSET)
    }

    // optional int64 server_ms = 3;

    pub fn clear_server_ms(&mut self) {
        self.server_ms = ::std::option::Option::None;
    }

    pub fn has_server_ms(&self) -> bool {
        self.server_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_ms(&mut self, v: i64) {
        self.server_ms = ::std::option::Option::Some(v);
    }

    pub fn get_server_ms(&self) -> i64 {
        self.server_ms.unwrap_or(0)
    }

    // repeated .POGOProtos.Data.Battle.BattleAction battle_actions = 4;

    pub fn clear_battle_actions(&mut self) {
        self.battle_actions.clear();
    }

    // Param is passed by value, moved
    pub fn set_battle_actions(&mut self, v: ::protobuf::RepeatedField<BattleAction>) {
        self.battle_actions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_battle_actions(&mut self) -> &mut ::protobuf::RepeatedField<BattleAction> {
        &mut self.battle_actions
    }

    // Take field
    pub fn take_battle_actions(&mut self) -> ::protobuf::RepeatedField<BattleAction> {
        ::std::mem::replace(&mut self.battle_actions, ::protobuf::RepeatedField::new())
    }

    pub fn get_battle_actions(&self) -> &[BattleAction] {
        &self.battle_actions
    }

    // optional int64 battle_start_timestamp_ms = 5;

    pub fn clear_battle_start_timestamp_ms(&mut self) {
        self.battle_start_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_battle_start_timestamp_ms(&self) -> bool {
        self.battle_start_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battle_start_timestamp_ms(&mut self, v: i64) {
        self.battle_start_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_battle_start_timestamp_ms(&self) -> i64 {
        self.battle_start_timestamp_ms.unwrap_or(0)
    }

    // optional int64 battle_end_timestamp_ms = 6;

    pub fn clear_battle_end_timestamp_ms(&mut self) {
        self.battle_end_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_battle_end_timestamp_ms(&self) -> bool {
        self.battle_end_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battle_end_timestamp_ms(&mut self, v: i64) {
        self.battle_end_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_battle_end_timestamp_ms(&self) -> i64 {
        self.battle_end_timestamp_ms.unwrap_or(0)
    }
}

impl ::protobuf::Message for BattleLog {
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
                    self.state = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.battle_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.server_ms = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.battle_actions));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.battle_start_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.battle_end_timestamp_ms = ::std::option::Option::Some(tmp);
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
        for value in &self.state {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.battle_type {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.server_ms {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.battle_actions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.battle_start_timestamp_ms {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.battle_end_timestamp_ms {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.state {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.battle_type {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.server_ms {
            try!(os.write_int64(3, v));
        };
        for v in &self.battle_actions {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.battle_start_timestamp_ms {
            try!(os.write_int64(5, v));
        };
        if let Some(v) = self.battle_end_timestamp_ms {
            try!(os.write_int64(6, v));
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
        ::std::any::TypeId::of::<BattleLog>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BattleLog {
    fn new() -> BattleLog {
        BattleLog::new()
    }

    fn descriptor_static(_: ::std::option::Option<BattleLog>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "state",
                    BattleLog::has_state,
                    BattleLog::get_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "battle_type",
                    BattleLog::has_battle_type,
                    BattleLog::get_battle_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "server_ms",
                    BattleLog::has_server_ms,
                    BattleLog::get_server_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "battle_actions",
                    BattleLog::get_battle_actions,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "battle_start_timestamp_ms",
                    BattleLog::has_battle_start_timestamp_ms,
                    BattleLog::get_battle_start_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "battle_end_timestamp_ms",
                    BattleLog::has_battle_end_timestamp_ms,
                    BattleLog::get_battle_end_timestamp_ms,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BattleLog>(
                    "BattleLog",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BattleLog {
    fn clear(&mut self) {
        self.clear_state();
        self.clear_battle_type();
        self.clear_server_ms();
        self.clear_battle_actions();
        self.clear_battle_start_timestamp_ms();
        self.clear_battle_end_timestamp_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BattleLog {
    fn eq(&self, other: &BattleLog) -> bool {
        self.state == other.state &&
        self.battle_type == other.battle_type &&
        self.server_ms == other.server_ms &&
        self.battle_actions == other.battle_actions &&
        self.battle_start_timestamp_ms == other.battle_start_timestamp_ms &&
        self.battle_end_timestamp_ms == other.battle_end_timestamp_ms &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BattleLog {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum BattleType {
    BATTLE_TYPE_UNSET = 0,
    BATTLE_TYPE_NORMAL = 1,
    BATTLE_TYPE_TRAINING = 2,
}

impl ::protobuf::ProtobufEnum for BattleType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<BattleType> {
        match value {
            0 => ::std::option::Option::Some(BattleType::BATTLE_TYPE_UNSET),
            1 => ::std::option::Option::Some(BattleType::BATTLE_TYPE_NORMAL),
            2 => ::std::option::Option::Some(BattleType::BATTLE_TYPE_TRAINING),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [BattleType] = &[
            BattleType::BATTLE_TYPE_UNSET,
            BattleType::BATTLE_TYPE_NORMAL,
            BattleType::BATTLE_TYPE_TRAINING,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<BattleType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("BattleType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for BattleType {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum BattleState {
    STATE_UNSET = 0,
    ACTIVE = 1,
    VICTORY = 2,
    DEFEATED = 3,
    TIMED_OUT = 4,
}

impl ::protobuf::ProtobufEnum for BattleState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<BattleState> {
        match value {
            0 => ::std::option::Option::Some(BattleState::STATE_UNSET),
            1 => ::std::option::Option::Some(BattleState::ACTIVE),
            2 => ::std::option::Option::Some(BattleState::VICTORY),
            3 => ::std::option::Option::Some(BattleState::DEFEATED),
            4 => ::std::option::Option::Some(BattleState::TIMED_OUT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [BattleState] = &[
            BattleState::STATE_UNSET,
            BattleState::ACTIVE,
            BattleState::VICTORY,
            BattleState::DEFEATED,
            BattleState::TIMED_OUT,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<BattleState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("BattleState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for BattleState {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum BattleActionType {
    ACTION_UNSET = 0,
    ACTION_ATTACK = 1,
    ACTION_DODGE = 2,
    ACTION_SPECIAL_ATTACK = 3,
    ACTION_SWAP_POKEMON = 4,
    ACTION_FAINT = 5,
    ACTION_PLAYER_JOIN = 6,
    ACTION_PLAYER_QUIT = 7,
    ACTION_VICTORY = 8,
    ACTION_DEFEAT = 9,
    ACTION_TIMED_OUT = 10,
}

impl ::protobuf::ProtobufEnum for BattleActionType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<BattleActionType> {
        match value {
            0 => ::std::option::Option::Some(BattleActionType::ACTION_UNSET),
            1 => ::std::option::Option::Some(BattleActionType::ACTION_ATTACK),
            2 => ::std::option::Option::Some(BattleActionType::ACTION_DODGE),
            3 => ::std::option::Option::Some(BattleActionType::ACTION_SPECIAL_ATTACK),
            4 => ::std::option::Option::Some(BattleActionType::ACTION_SWAP_POKEMON),
            5 => ::std::option::Option::Some(BattleActionType::ACTION_FAINT),
            6 => ::std::option::Option::Some(BattleActionType::ACTION_PLAYER_JOIN),
            7 => ::std::option::Option::Some(BattleActionType::ACTION_PLAYER_QUIT),
            8 => ::std::option::Option::Some(BattleActionType::ACTION_VICTORY),
            9 => ::std::option::Option::Some(BattleActionType::ACTION_DEFEAT),
            10 => ::std::option::Option::Some(BattleActionType::ACTION_TIMED_OUT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [BattleActionType] = &[
            BattleActionType::ACTION_UNSET,
            BattleActionType::ACTION_ATTACK,
            BattleActionType::ACTION_DODGE,
            BattleActionType::ACTION_SPECIAL_ATTACK,
            BattleActionType::ACTION_SWAP_POKEMON,
            BattleActionType::ACTION_FAINT,
            BattleActionType::ACTION_PLAYER_JOIN,
            BattleActionType::ACTION_PLAYER_QUIT,
            BattleActionType::ACTION_VICTORY,
            BattleActionType::ACTION_DEFEAT,
            BattleActionType::ACTION_TIMED_OUT,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<BattleActionType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("BattleActionType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for BattleActionType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1c, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x16,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e,
    0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x1a, 0x1c, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x15, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x19, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x47, 0x79, 0x6d,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x00, 0x50, 0x01, 0x50, 0x02, 0x22, 0xf2, 0x02, 0x0a,
    0x11, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x69, 0x70, 0x61,
    0x6e, 0x74, 0x12, 0x50, 0x0a, 0x0e, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x70, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x29, 0x2e, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x42, 0x61, 0x74,
    0x74, 0x6c, 0x65, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f,
    0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x0d, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x50, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x12, 0x61, 0x0a, 0x16, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x5f,
    0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x70, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x50, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x50, 0x72, 0x6f, 0x66, 0x69, 0x6c,
    0x65, 0x52, 0x14, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63,
    0x50, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x12, 0x52, 0x0a, 0x0f, 0x72, 0x65, 0x76, 0x65, 0x72,
    0x73, 0x65, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x29, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61,
    0x74, 0x61, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65,
    0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x0e, 0x72, 0x65, 0x76,
    0x65, 0x72, 0x73, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x12, 0x54, 0x0a, 0x10, 0x64,
    0x65, 0x66, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x18,
    0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x29, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x42,
    0x61, 0x74, 0x74, 0x6c, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f,
    0x52, 0x0f, 0x64, 0x65, 0x66, 0x65, 0x61, 0x74, 0x65, 0x64, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f,
    0x6e, 0x22, 0xa2, 0x01, 0x0a, 0x11, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x50, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x3f, 0x0a, 0x0c, 0x70, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e,
    0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x52, 0x0b, 0x70, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x74, 0x5f, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x0d, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x12,
    0x25, 0x0a, 0x0e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x65, 0x6e, 0x65, 0x72, 0x67,
    0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74,
    0x45, 0x6e, 0x65, 0x72, 0x67, 0x79, 0x22, 0xd8, 0x05, 0x0a, 0x0c, 0x42, 0x61, 0x74, 0x74, 0x6c,
    0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x3c, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x28, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x42,
    0x61, 0x74, 0x74, 0x6c, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x52,
    0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x26, 0x0a, 0x0f, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f,
    0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0d,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x4d, 0x73, 0x12, 0x1f, 0x0a,
    0x0b, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x73, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x0a, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x73, 0x12, 0x21,
    0x0a, 0x0c, 0x65, 0x6e, 0x65, 0x72, 0x67, 0x79, 0x5f, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x65, 0x6e, 0x65, 0x72, 0x67, 0x79, 0x44, 0x65, 0x6c, 0x74,
    0x61, 0x12, 0x25, 0x0a, 0x0e, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x5f, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x61, 0x74, 0x74, 0x61, 0x63,
    0x6b, 0x65, 0x72, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x21, 0x0a, 0x0c, 0x74, 0x61, 0x72, 0x67,
    0x65, 0x74, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b,
    0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x2a, 0x0a, 0x11, 0x61,
    0x63, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64,
    0x18, 0x08, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0f, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x50, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x4e, 0x0a, 0x0d, 0x70, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x5f, 0x6a, 0x6f, 0x69, 0x6e, 0x65, 0x64, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x29,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61,
    0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x50, 0x61,
    0x72, 0x74, 0x69, 0x63, 0x69, 0x70, 0x61, 0x6e, 0x74, 0x52, 0x0c, 0x70, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x4a, 0x6f, 0x69, 0x6e, 0x65, 0x64, 0x12, 0x4c, 0x0a, 0x0e, 0x62, 0x61, 0x74, 0x74, 0x6c,
    0x65, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x25, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x52,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x52, 0x0d, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x52, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x73, 0x12, 0x4a, 0x0a, 0x22, 0x64, 0x61, 0x6d, 0x61, 0x67, 0x65, 0x5f,
    0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x73, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x74, 0x69,
    0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x73, 0x18, 0x0b, 0x20, 0x01, 0x28,
    0x03, 0x52, 0x1e, 0x64, 0x61, 0x6d, 0x61, 0x67, 0x65, 0x57, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x73,
    0x53, 0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73,
    0x73, 0x12, 0x46, 0x0a, 0x20, 0x64, 0x61, 0x6d, 0x61, 0x67, 0x65, 0x5f, 0x77, 0x69, 0x6e, 0x64,
    0x6f, 0x77, 0x73, 0x5f, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d,
    0x70, 0x5f, 0x6d, 0x73, 0x73, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x03, 0x52, 0x1c, 0x64, 0x61, 0x6d,
    0x61, 0x67, 0x65, 0x57, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x73, 0x45, 0x6e, 0x64, 0x54, 0x69, 0x6d,
    0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x73, 0x12, 0x4a, 0x0a, 0x0b, 0x70, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x5f, 0x6c, 0x65, 0x66, 0x74, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x29,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61,
    0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x50, 0x61,
    0x72, 0x74, 0x69, 0x63, 0x69, 0x70, 0x61, 0x6e, 0x74, 0x52, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x4c, 0x65, 0x66, 0x74, 0x12, 0x2a, 0x0a, 0x11, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f,
    0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x06,
    0x52, 0x0f, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49,
    0x64, 0x22, 0xb3, 0x02, 0x0a, 0x0d, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x73, 0x12, 0x3a, 0x0a, 0x09, 0x67, 0x79, 0x6d, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x47, 0x79, 0x6d, 0x2e, 0x47, 0x79, 0x6d,
    0x53, 0x74, 0x61, 0x74, 0x65, 0x52, 0x08, 0x67, 0x79, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12,
    0x47, 0x0a, 0x09, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x73, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x29, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x44, 0x61, 0x74, 0x61, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x42, 0x61, 0x74, 0x74,
    0x6c, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x69, 0x70, 0x61, 0x6e, 0x74, 0x52, 0x09, 0x61,
    0x74, 0x74, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x73, 0x12, 0x3a, 0x0a, 0x19, 0x70, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x5f, 0x65, 0x78, 0x70, 0x65, 0x72, 0x69, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x61, 0x77,
    0x61, 0x72, 0x64, 0x65, 0x64, 0x18, 0x03, 0x20, 0x03, 0x28, 0x05, 0x52, 0x17, 0x70, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x45, 0x78, 0x70, 0x65, 0x72, 0x69, 0x65, 0x6e, 0x63, 0x65, 0x41, 0x77, 0x61,
    0x72, 0x64, 0x65, 0x64, 0x12, 0x37, 0x0a, 0x18, 0x6e, 0x65, 0x78, 0x74, 0x5f, 0x64, 0x65, 0x66,
    0x65, 0x6e, 0x64, 0x65, 0x72, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x52, 0x15, 0x6e, 0x65, 0x78, 0x74, 0x44, 0x65, 0x66, 0x65,
    0x6e, 0x64, 0x65, 0x72, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x28, 0x0a,
    0x10, 0x67, 0x79, 0x6d, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x73, 0x5f, 0x64, 0x65, 0x6c, 0x74,
    0x61, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0e, 0x67, 0x79, 0x6d, 0x50, 0x6f, 0x69, 0x6e,
    0x74, 0x73, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x22, 0xe7, 0x02, 0x0a, 0x09, 0x42, 0x61, 0x74, 0x74,
    0x6c, 0x65, 0x4c, 0x6f, 0x67, 0x12, 0x39, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x23, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x42, 0x61,
    0x74, 0x74, 0x6c, 0x65, 0x53, 0x74, 0x61, 0x74, 0x65, 0x52, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x12, 0x43, 0x0a, 0x0b, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x22, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x42,
    0x61, 0x74, 0x74, 0x6c, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0a, 0x62, 0x61, 0x74, 0x74, 0x6c,
    0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f,
    0x6d, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x52, 0x08, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x4d, 0x73, 0x12, 0x4b, 0x0a, 0x0e, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x5f, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x42, 0x61, 0x74,
    0x74, 0x6c, 0x65, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x52, 0x0d, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12,
    0x39, 0x0a, 0x19, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x03, 0x52, 0x16, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x53, 0x74, 0x61, 0x72, 0x74, 0x54,
    0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12, 0x35, 0x0a, 0x17, 0x62, 0x61,
    0x74, 0x74, 0x6c, 0x65, 0x5f, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
    0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x03, 0x52, 0x14, 0x62, 0x61, 0x74,
    0x74, 0x6c, 0x65, 0x45, 0x6e, 0x64, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d,
    0x73, 0x2a, 0x55, 0x0a, 0x0a, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x15, 0x0a, 0x11, 0x42, 0x41, 0x54, 0x54, 0x4c, 0x45, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x55,
    0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x16, 0x0a, 0x12, 0x42, 0x41, 0x54, 0x54, 0x4c, 0x45,
    0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x4e, 0x4f, 0x52, 0x4d, 0x41, 0x4c, 0x10, 0x01, 0x12, 0x18,
    0x0a, 0x14, 0x42, 0x41, 0x54, 0x54, 0x4c, 0x45, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x54, 0x52,
    0x41, 0x49, 0x4e, 0x49, 0x4e, 0x47, 0x10, 0x02, 0x2a, 0x54, 0x0a, 0x0b, 0x42, 0x61, 0x74, 0x74,
    0x6c, 0x65, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x0f, 0x0a, 0x0b, 0x53, 0x54, 0x41, 0x54, 0x45,
    0x5f, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06, 0x41, 0x43, 0x54, 0x49,
    0x56, 0x45, 0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x56, 0x49, 0x43, 0x54, 0x4f, 0x52, 0x59, 0x10,
    0x02, 0x12, 0x0c, 0x0a, 0x08, 0x44, 0x45, 0x46, 0x45, 0x41, 0x54, 0x45, 0x44, 0x10, 0x03, 0x12,
    0x0d, 0x0a, 0x09, 0x54, 0x49, 0x4d, 0x45, 0x44, 0x5f, 0x4f, 0x55, 0x54, 0x10, 0x04, 0x2a, 0xfc,
    0x01, 0x0a, 0x10, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x10, 0x0a, 0x0c, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x55, 0x4e,
    0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x11, 0x0a, 0x0d, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f,
    0x41, 0x54, 0x54, 0x41, 0x43, 0x4b, 0x10, 0x01, 0x12, 0x10, 0x0a, 0x0c, 0x41, 0x43, 0x54, 0x49,
    0x4f, 0x4e, 0x5f, 0x44, 0x4f, 0x44, 0x47, 0x45, 0x10, 0x02, 0x12, 0x19, 0x0a, 0x15, 0x41, 0x43,
    0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x53, 0x50, 0x45, 0x43, 0x49, 0x41, 0x4c, 0x5f, 0x41, 0x54, 0x54,
    0x41, 0x43, 0x4b, 0x10, 0x03, 0x12, 0x17, 0x0a, 0x13, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f,
    0x53, 0x57, 0x41, 0x50, 0x5f, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x10, 0x04, 0x12, 0x10,
    0x0a, 0x0c, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x46, 0x41, 0x49, 0x4e, 0x54, 0x10, 0x05,
    0x12, 0x16, 0x0a, 0x12, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x50, 0x4c, 0x41, 0x59, 0x45,
    0x52, 0x5f, 0x4a, 0x4f, 0x49, 0x4e, 0x10, 0x06, 0x12, 0x16, 0x0a, 0x12, 0x41, 0x43, 0x54, 0x49,
    0x4f, 0x4e, 0x5f, 0x50, 0x4c, 0x41, 0x59, 0x45, 0x52, 0x5f, 0x51, 0x55, 0x49, 0x54, 0x10, 0x07,
    0x12, 0x12, 0x0a, 0x0e, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x56, 0x49, 0x43, 0x54, 0x4f,
    0x52, 0x59, 0x10, 0x08, 0x12, 0x11, 0x0a, 0x0d, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x44,
    0x45, 0x46, 0x45, 0x41, 0x54, 0x10, 0x09, 0x12, 0x14, 0x0a, 0x10, 0x41, 0x43, 0x54, 0x49, 0x4f,
    0x4e, 0x5f, 0x54, 0x49, 0x4d, 0x45, 0x44, 0x5f, 0x4f, 0x55, 0x54, 0x10, 0x0a, 0x4a, 0x9e, 0x19,
    0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x48, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x1e, 0x0a, 0x09, 0x0a, 0x02,
    0x0a, 0x00, 0x12, 0x03, 0x03, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03,
    0x0e, 0x2c, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x04, 0x07, 0x0d, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x0e, 0x25, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x02, 0x12, 0x03,
    0x05, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x05, 0x0e, 0x29, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x07, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x08, 0x08, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x08, 0x08,
    0x07, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x08, 0x08, 0x31,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x32, 0x40, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x43, 0x44, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x08, 0x4f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x09, 0x08, 0x08, 0x45, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x09, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x09, 0x34, 0x4a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x09,
    0x4d, 0x4e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0a, 0x08, 0x4f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0a, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0a, 0x11, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x3b, 0x4a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x0a, 0x4d, 0x4e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x0b, 0x08, 0x50, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0b,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x0b, 0x11, 0x3a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0b, 0x3b, 0x4b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0b, 0x4e, 0x4f, 0x0a, 0x0a, 0x0a, 0x02,
    0x05, 0x00, 0x12, 0x04, 0x0d, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12,
    0x03, 0x0d, 0x05, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x08,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0e, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x0f, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x10, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x08,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x10, 0x1f, 0x20, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x12, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x12, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x13, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x13,
    0x08, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x13, 0x08,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x25, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x13, 0x34, 0x35, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x14, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x14, 0x08, 0x13, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x14, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x14, 0x0e, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x14, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x15, 0x08, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0x15, 0x08, 0x14, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x15, 0x08, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x15, 0x0e, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x15, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x17, 0x00, 0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x17, 0x08,
    0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x18, 0x08, 0x3a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x18, 0x08, 0x17, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x18, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x31, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x18, 0x38, 0x39, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x19, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x19,
    0x08, 0x18, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x19, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x19, 0x0e, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x19, 0x20, 0x21, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x1a, 0x08, 0x19, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x1a, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x1a, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x1a, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x08, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x04, 0x1b, 0x08, 0x1a, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x1b, 0x08, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1b, 0x0e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1b, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x1c, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x1c, 0x08, 0x1b, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x1c, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1c, 0x0e,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1c, 0x1f, 0x20, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1d, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x05, 0x04, 0x12, 0x04, 0x1d, 0x08, 0x1c, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x05, 0x05, 0x12, 0x03, 0x1d, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x1d, 0x0e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x1d, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1e,
    0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x04, 0x12, 0x04, 0x1e, 0x08, 0x1d,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x05, 0x12, 0x03, 0x1e, 0x08, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1e, 0x10, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x06, 0x03, 0x12, 0x03, 0x1e, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x07, 0x12, 0x03, 0x1f, 0x08, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07,
    0x04, 0x12, 0x04, 0x1f, 0x08, 0x1e, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x06,
    0x12, 0x03, 0x1f, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x1f, 0x32, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x1f, 0x42,
    0x43, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x08, 0x12, 0x03, 0x20, 0x08, 0x42, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x04, 0x12, 0x04, 0x20, 0x08, 0x1f, 0x44, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x08, 0x06, 0x12, 0x03, 0x20, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x08, 0x01, 0x12, 0x03, 0x20, 0x2e, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x08, 0x03, 0x12, 0x03, 0x20, 0x3f, 0x41, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x09, 0x12,
    0x03, 0x21, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x04, 0x12, 0x04, 0x21,
    0x08, 0x20, 0x42, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x05, 0x12, 0x03, 0x21, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x01, 0x12, 0x03, 0x21, 0x0e, 0x30, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x03, 0x12, 0x03, 0x21, 0x33, 0x35, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x0a, 0x12, 0x03, 0x22, 0x08, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x0a, 0x04, 0x12, 0x04, 0x22, 0x08, 0x21, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x0a, 0x05, 0x12, 0x03, 0x22, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x01,
    0x12, 0x03, 0x22, 0x0e, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x03, 0x12, 0x03,
    0x22, 0x31, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x0b, 0x12, 0x03, 0x23, 0x08, 0x43,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x04, 0x12, 0x04, 0x23, 0x08, 0x22, 0x34, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x06, 0x12, 0x03, 0x23, 0x08, 0x31, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x23, 0x32, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x23, 0x40, 0x42, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x0c, 0x12, 0x03, 0x24, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0c, 0x04, 0x12,
    0x04, 0x24, 0x08, 0x23, 0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0c, 0x05, 0x12, 0x03,
    0x24, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x24, 0x10,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x24, 0x24, 0x26, 0x0a,
    0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x26, 0x00, 0x2c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05,
    0x01, 0x01, 0x12, 0x03, 0x26, 0x05, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x27, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x27,
    0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x27, 0x16, 0x17,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x28, 0x08, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x28, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x28, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x02, 0x12, 0x03, 0x29, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x29, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x29,
    0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x03, 0x12, 0x03, 0x2a, 0x08, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x03, 0x02, 0x12, 0x03, 0x2a, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x01, 0x02, 0x04, 0x12, 0x03, 0x2b, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x2b, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x04, 0x02, 0x12,
    0x03, 0x2b, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x02, 0x12, 0x04, 0x2d, 0x00, 0x39, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x05, 0x15, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x02, 0x02, 0x00, 0x12, 0x03, 0x2e, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x2e, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2f,
    0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x02, 0x12, 0x03, 0x2f, 0x18, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x02, 0x02, 0x02, 0x12, 0x03, 0x30, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x30, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x30, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x31, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x31,
    0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x02, 0x12, 0x03, 0x31, 0x20, 0x21,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x32, 0x08, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x32, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x04, 0x02, 0x12, 0x03, 0x32, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x33, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x33, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x05, 0x02, 0x12, 0x03, 0x33,
    0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x34, 0x08, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x34, 0x08, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x06, 0x02, 0x12, 0x03, 0x34, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x02, 0x02, 0x07, 0x12, 0x03, 0x35, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x07,
    0x01, 0x12, 0x03, 0x35, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x07, 0x02, 0x12,
    0x03, 0x35, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x08, 0x12, 0x03, 0x36, 0x08,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x08, 0x01, 0x12, 0x03, 0x36, 0x08, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x08, 0x02, 0x12, 0x03, 0x36, 0x19, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x02, 0x02, 0x09, 0x12, 0x03, 0x37, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02,
    0x02, 0x09, 0x01, 0x12, 0x03, 0x37, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x09,
    0x02, 0x12, 0x03, 0x37, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x0a, 0x12, 0x03,
    0x38, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x38, 0x08,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x38, 0x1b, 0x1d, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x3a, 0x00, 0x40, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x03, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12,
    0x03, 0x3b, 0x08, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x3b,
    0x08, 0x3a, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3b, 0x08,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3b, 0x26, 0x2f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3b, 0x32, 0x33, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x3c, 0x08, 0x49, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x3c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x3c, 0x11, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x3c, 0x3b, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3c,
    0x47, 0x48, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x3d, 0x08, 0x35, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3d, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3d, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3d, 0x17, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x3d, 0x33, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12,
    0x03, 0x3e, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x04, 0x3e,
    0x08, 0x3d, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x3e, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3e, 0x0e, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x3e, 0x29, 0x2a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x3f, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x3f, 0x08, 0x3e, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x3f, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x3f, 0x0e, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x3f, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x41, 0x00, 0x48, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x41, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x00, 0x12, 0x03, 0x42, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x42, 0x08, 0x41, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x42, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x42, 0x2c, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x42, 0x34,
    0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x43, 0x08, 0x3b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x04, 0x43, 0x08, 0x42, 0x36, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x43, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x43, 0x2b, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x43, 0x39, 0x3a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12,
    0x03, 0x44, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x04, 0x44,
    0x08, 0x43, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x44, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x44, 0x0e, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x44, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x45, 0x08, 0x49, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x45, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03,
    0x06, 0x12, 0x03, 0x45, 0x11, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x45, 0x36, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x45,
    0x47, 0x48, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x46, 0x08, 0x2c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x04, 0x12, 0x04, 0x46, 0x08, 0x45, 0x49, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x05, 0x12, 0x03, 0x46, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x04, 0x01, 0x12, 0x03, 0x46, 0x0e, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x46, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05,
    0x12, 0x03, 0x47, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x04, 0x12, 0x04,
    0x47, 0x08, 0x46, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x05, 0x12, 0x03, 0x47,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x47, 0x0e, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x03, 0x12, 0x03, 0x47, 0x28, 0x29, 0x62, 0x06,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
