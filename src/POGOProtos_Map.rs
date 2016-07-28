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
pub struct MapCell {
    // message fields
    s2_cell_id: ::std::option::Option<u64>,
    current_timestamp_ms: ::std::option::Option<i64>,
    forts: ::protobuf::RepeatedField<super::POGOProtos_Map_Fort::FortData>,
    spawn_points: ::protobuf::RepeatedField<SpawnPoint>,
    deleted_objects: ::protobuf::RepeatedField<::std::string::String>,
    is_truncated_list: ::std::option::Option<bool>,
    fort_summaries: ::protobuf::RepeatedField<super::POGOProtos_Map_Fort::FortSummary>,
    decimated_spawn_points: ::protobuf::RepeatedField<SpawnPoint>,
    wild_pokemons: ::protobuf::RepeatedField<super::POGOProtos_Map_Pokemon::WildPokemon>,
    catchable_pokemons: ::protobuf::RepeatedField<super::POGOProtos_Map_Pokemon::MapPokemon>,
    nearby_pokemons: ::protobuf::RepeatedField<super::POGOProtos_Map_Pokemon::NearbyPokemon>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MapCell {}

impl MapCell {
    pub fn new() -> MapCell {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MapCell {
        static mut instance: ::protobuf::lazy::Lazy<MapCell> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MapCell,
        };
        unsafe {
            instance.get(|| {
                MapCell {
                    s2_cell_id: ::std::option::Option::None,
                    current_timestamp_ms: ::std::option::Option::None,
                    forts: ::protobuf::RepeatedField::new(),
                    spawn_points: ::protobuf::RepeatedField::new(),
                    deleted_objects: ::protobuf::RepeatedField::new(),
                    is_truncated_list: ::std::option::Option::None,
                    fort_summaries: ::protobuf::RepeatedField::new(),
                    decimated_spawn_points: ::protobuf::RepeatedField::new(),
                    wild_pokemons: ::protobuf::RepeatedField::new(),
                    catchable_pokemons: ::protobuf::RepeatedField::new(),
                    nearby_pokemons: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 s2_cell_id = 1;

    pub fn clear_s2_cell_id(&mut self) {
        self.s2_cell_id = ::std::option::Option::None;
    }

    pub fn has_s2_cell_id(&self) -> bool {
        self.s2_cell_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s2_cell_id(&mut self, v: u64) {
        self.s2_cell_id = ::std::option::Option::Some(v);
    }

    pub fn get_s2_cell_id(&self) -> u64 {
        self.s2_cell_id.unwrap_or(0)
    }

    // optional int64 current_timestamp_ms = 2;

    pub fn clear_current_timestamp_ms(&mut self) {
        self.current_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_current_timestamp_ms(&self) -> bool {
        self.current_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_current_timestamp_ms(&mut self, v: i64) {
        self.current_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_current_timestamp_ms(&self) -> i64 {
        self.current_timestamp_ms.unwrap_or(0)
    }

    // repeated .POGOProtos.Map.Fort.FortData forts = 3;

    pub fn clear_forts(&mut self) {
        self.forts.clear();
    }

    // Param is passed by value, moved
    pub fn set_forts(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Map_Fort::FortData>) {
        self.forts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_forts(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Map_Fort::FortData> {
        &mut self.forts
    }

    // Take field
    pub fn take_forts(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Map_Fort::FortData> {
        ::std::mem::replace(&mut self.forts, ::protobuf::RepeatedField::new())
    }

    pub fn get_forts(&self) -> &[super::POGOProtos_Map_Fort::FortData] {
        &self.forts
    }

    // repeated .POGOProtos.Map.SpawnPoint spawn_points = 4;

    pub fn clear_spawn_points(&mut self) {
        self.spawn_points.clear();
    }

    // Param is passed by value, moved
    pub fn set_spawn_points(&mut self, v: ::protobuf::RepeatedField<SpawnPoint>) {
        self.spawn_points = v;
    }

    // Mutable pointer to the field.
    pub fn mut_spawn_points(&mut self) -> &mut ::protobuf::RepeatedField<SpawnPoint> {
        &mut self.spawn_points
    }

    // Take field
    pub fn take_spawn_points(&mut self) -> ::protobuf::RepeatedField<SpawnPoint> {
        ::std::mem::replace(&mut self.spawn_points, ::protobuf::RepeatedField::new())
    }

    pub fn get_spawn_points(&self) -> &[SpawnPoint] {
        &self.spawn_points
    }

    // repeated string deleted_objects = 6;

    pub fn clear_deleted_objects(&mut self) {
        self.deleted_objects.clear();
    }

    // Param is passed by value, moved
    pub fn set_deleted_objects(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.deleted_objects = v;
    }

    // Mutable pointer to the field.
    pub fn mut_deleted_objects(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.deleted_objects
    }

    // Take field
    pub fn take_deleted_objects(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.deleted_objects, ::protobuf::RepeatedField::new())
    }

    pub fn get_deleted_objects(&self) -> &[::std::string::String] {
        &self.deleted_objects
    }

    // optional bool is_truncated_list = 7;

    pub fn clear_is_truncated_list(&mut self) {
        self.is_truncated_list = ::std::option::Option::None;
    }

    pub fn has_is_truncated_list(&self) -> bool {
        self.is_truncated_list.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_truncated_list(&mut self, v: bool) {
        self.is_truncated_list = ::std::option::Option::Some(v);
    }

    pub fn get_is_truncated_list(&self) -> bool {
        self.is_truncated_list.unwrap_or(false)
    }

    // repeated .POGOProtos.Map.Fort.FortSummary fort_summaries = 8;

    pub fn clear_fort_summaries(&mut self) {
        self.fort_summaries.clear();
    }

    // Param is passed by value, moved
    pub fn set_fort_summaries(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Map_Fort::FortSummary>) {
        self.fort_summaries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fort_summaries(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Map_Fort::FortSummary> {
        &mut self.fort_summaries
    }

    // Take field
    pub fn take_fort_summaries(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Map_Fort::FortSummary> {
        ::std::mem::replace(&mut self.fort_summaries, ::protobuf::RepeatedField::new())
    }

    pub fn get_fort_summaries(&self) -> &[super::POGOProtos_Map_Fort::FortSummary] {
        &self.fort_summaries
    }

    // repeated .POGOProtos.Map.SpawnPoint decimated_spawn_points = 9;

    pub fn clear_decimated_spawn_points(&mut self) {
        self.decimated_spawn_points.clear();
    }

    // Param is passed by value, moved
    pub fn set_decimated_spawn_points(&mut self, v: ::protobuf::RepeatedField<SpawnPoint>) {
        self.decimated_spawn_points = v;
    }

    // Mutable pointer to the field.
    pub fn mut_decimated_spawn_points(&mut self) -> &mut ::protobuf::RepeatedField<SpawnPoint> {
        &mut self.decimated_spawn_points
    }

    // Take field
    pub fn take_decimated_spawn_points(&mut self) -> ::protobuf::RepeatedField<SpawnPoint> {
        ::std::mem::replace(&mut self.decimated_spawn_points, ::protobuf::RepeatedField::new())
    }

    pub fn get_decimated_spawn_points(&self) -> &[SpawnPoint] {
        &self.decimated_spawn_points
    }

    // repeated .POGOProtos.Map.Pokemon.WildPokemon wild_pokemons = 5;

    pub fn clear_wild_pokemons(&mut self) {
        self.wild_pokemons.clear();
    }

    // Param is passed by value, moved
    pub fn set_wild_pokemons(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Map_Pokemon::WildPokemon>) {
        self.wild_pokemons = v;
    }

    // Mutable pointer to the field.
    pub fn mut_wild_pokemons(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Map_Pokemon::WildPokemon> {
        &mut self.wild_pokemons
    }

    // Take field
    pub fn take_wild_pokemons(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Map_Pokemon::WildPokemon> {
        ::std::mem::replace(&mut self.wild_pokemons, ::protobuf::RepeatedField::new())
    }

    pub fn get_wild_pokemons(&self) -> &[super::POGOProtos_Map_Pokemon::WildPokemon] {
        &self.wild_pokemons
    }

    // repeated .POGOProtos.Map.Pokemon.MapPokemon catchable_pokemons = 10;

    pub fn clear_catchable_pokemons(&mut self) {
        self.catchable_pokemons.clear();
    }

    // Param is passed by value, moved
    pub fn set_catchable_pokemons(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Map_Pokemon::MapPokemon>) {
        self.catchable_pokemons = v;
    }

    // Mutable pointer to the field.
    pub fn mut_catchable_pokemons(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Map_Pokemon::MapPokemon> {
        &mut self.catchable_pokemons
    }

    // Take field
    pub fn take_catchable_pokemons(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Map_Pokemon::MapPokemon> {
        ::std::mem::replace(&mut self.catchable_pokemons, ::protobuf::RepeatedField::new())
    }

    pub fn get_catchable_pokemons(&self) -> &[super::POGOProtos_Map_Pokemon::MapPokemon] {
        &self.catchable_pokemons
    }

    // repeated .POGOProtos.Map.Pokemon.NearbyPokemon nearby_pokemons = 11;

    pub fn clear_nearby_pokemons(&mut self) {
        self.nearby_pokemons.clear();
    }

    // Param is passed by value, moved
    pub fn set_nearby_pokemons(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Map_Pokemon::NearbyPokemon>) {
        self.nearby_pokemons = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nearby_pokemons(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Map_Pokemon::NearbyPokemon> {
        &mut self.nearby_pokemons
    }

    // Take field
    pub fn take_nearby_pokemons(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Map_Pokemon::NearbyPokemon> {
        ::std::mem::replace(&mut self.nearby_pokemons, ::protobuf::RepeatedField::new())
    }

    pub fn get_nearby_pokemons(&self) -> &[super::POGOProtos_Map_Pokemon::NearbyPokemon] {
        &self.nearby_pokemons
    }
}

impl ::protobuf::Message for MapCell {
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
                    let tmp = try!(is.read_uint64());
                    self.s2_cell_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.current_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.forts));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.spawn_points));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.deleted_objects));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_truncated_list = ::std::option::Option::Some(tmp);
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.fort_summaries));
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.decimated_spawn_points));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.wild_pokemons));
                },
                10 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.catchable_pokemons));
                },
                11 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.nearby_pokemons));
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
        for value in &self.s2_cell_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.current_timestamp_ms {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.forts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.spawn_points {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.deleted_objects {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        if self.is_truncated_list.is_some() {
            my_size += 2;
        };
        for value in &self.fort_summaries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.decimated_spawn_points {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.wild_pokemons {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.catchable_pokemons {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.nearby_pokemons {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.s2_cell_id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.current_timestamp_ms {
            try!(os.write_int64(2, v));
        };
        for v in &self.forts {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.spawn_points {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.deleted_objects {
            try!(os.write_string(6, &v));
        };
        if let Some(v) = self.is_truncated_list {
            try!(os.write_bool(7, v));
        };
        for v in &self.fort_summaries {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.decimated_spawn_points {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.wild_pokemons {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.catchable_pokemons {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.nearby_pokemons {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<MapCell>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MapCell {
    fn new() -> MapCell {
        MapCell::new()
    }

    fn descriptor_static(_: ::std::option::Option<MapCell>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "s2_cell_id",
                    MapCell::has_s2_cell_id,
                    MapCell::get_s2_cell_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "current_timestamp_ms",
                    MapCell::has_current_timestamp_ms,
                    MapCell::get_current_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "forts",
                    MapCell::get_forts,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "spawn_points",
                    MapCell::get_spawn_points,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "deleted_objects",
                    MapCell::get_deleted_objects,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_truncated_list",
                    MapCell::has_is_truncated_list,
                    MapCell::get_is_truncated_list,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "fort_summaries",
                    MapCell::get_fort_summaries,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "decimated_spawn_points",
                    MapCell::get_decimated_spawn_points,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "wild_pokemons",
                    MapCell::get_wild_pokemons,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "catchable_pokemons",
                    MapCell::get_catchable_pokemons,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "nearby_pokemons",
                    MapCell::get_nearby_pokemons,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MapCell>(
                    "MapCell",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MapCell {
    fn clear(&mut self) {
        self.clear_s2_cell_id();
        self.clear_current_timestamp_ms();
        self.clear_forts();
        self.clear_spawn_points();
        self.clear_deleted_objects();
        self.clear_is_truncated_list();
        self.clear_fort_summaries();
        self.clear_decimated_spawn_points();
        self.clear_wild_pokemons();
        self.clear_catchable_pokemons();
        self.clear_nearby_pokemons();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for MapCell {
    fn eq(&self, other: &MapCell) -> bool {
        self.s2_cell_id == other.s2_cell_id &&
        self.current_timestamp_ms == other.current_timestamp_ms &&
        self.forts == other.forts &&
        self.spawn_points == other.spawn_points &&
        self.deleted_objects == other.deleted_objects &&
        self.is_truncated_list == other.is_truncated_list &&
        self.fort_summaries == other.fort_summaries &&
        self.decimated_spawn_points == other.decimated_spawn_points &&
        self.wild_pokemons == other.wild_pokemons &&
        self.catchable_pokemons == other.catchable_pokemons &&
        self.nearby_pokemons == other.nearby_pokemons &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for MapCell {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SpawnPoint {
    // message fields
    latitude: ::std::option::Option<f64>,
    longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SpawnPoint {}

impl SpawnPoint {
    pub fn new() -> SpawnPoint {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SpawnPoint {
        static mut instance: ::protobuf::lazy::Lazy<SpawnPoint> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SpawnPoint,
        };
        unsafe {
            instance.get(|| {
                SpawnPoint {
                    latitude: ::std::option::Option::None,
                    longitude: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional double latitude = 2;

    pub fn clear_latitude(&mut self) {
        self.latitude = ::std::option::Option::None;
    }

    pub fn has_latitude(&self) -> bool {
        self.latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latitude(&mut self, v: f64) {
        self.latitude = ::std::option::Option::Some(v);
    }

    pub fn get_latitude(&self) -> f64 {
        self.latitude.unwrap_or(0.)
    }

    // optional double longitude = 3;

    pub fn clear_longitude(&mut self) {
        self.longitude = ::std::option::Option::None;
    }

    pub fn has_longitude(&self) -> bool {
        self.longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_longitude(&mut self, v: f64) {
        self.longitude = ::std::option::Option::Some(v);
    }

    pub fn get_longitude(&self) -> f64 {
        self.longitude.unwrap_or(0.)
    }
}

impl ::protobuf::Message for SpawnPoint {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.latitude = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.longitude = ::std::option::Option::Some(tmp);
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
        if self.latitude.is_some() {
            my_size += 9;
        };
        if self.longitude.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.latitude {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.longitude {
            try!(os.write_double(3, v));
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
        ::std::any::TypeId::of::<SpawnPoint>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SpawnPoint {
    fn new() -> SpawnPoint {
        SpawnPoint::new()
    }

    fn descriptor_static(_: ::std::option::Option<SpawnPoint>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "latitude",
                    SpawnPoint::has_latitude,
                    SpawnPoint::get_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "longitude",
                    SpawnPoint::has_longitude,
                    SpawnPoint::get_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SpawnPoint>(
                    "SpawnPoint",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SpawnPoint {
    fn clear(&mut self) {
        self.clear_latitude();
        self.clear_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SpawnPoint {
    fn eq(&self, other: &SpawnPoint) -> bool {
        self.latitude == other.latitude &&
        self.longitude == other.longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SpawnPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MapObjectsStatus {
    UNSET_STATUS = 0,
    SUCCESS = 1,
    LOCATION_UNSET = 2,
}

impl ::protobuf::ProtobufEnum for MapObjectsStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MapObjectsStatus> {
        match value {
            0 => ::std::option::Option::Some(MapObjectsStatus::UNSET_STATUS),
            1 => ::std::option::Option::Some(MapObjectsStatus::SUCCESS),
            2 => ::std::option::Option::Some(MapObjectsStatus::LOCATION_UNSET),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MapObjectsStatus] = &[
            MapObjectsStatus::UNSET_STATUS,
            MapObjectsStatus::SUCCESS,
            MapObjectsStatus::LOCATION_UNSET,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MapObjectsStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MapObjectsStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MapObjectsStatus {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x14, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x1a, 0x19, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x1c, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61,
    0x70, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x50,
    0x00, 0x50, 0x01, 0x22, 0xaa, 0x05, 0x0a, 0x07, 0x4d, 0x61, 0x70, 0x43, 0x65, 0x6c, 0x6c, 0x12,
    0x1c, 0x0a, 0x0a, 0x73, 0x32, 0x5f, 0x63, 0x65, 0x6c, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x08, 0x73, 0x32, 0x43, 0x65, 0x6c, 0x6c, 0x49, 0x64, 0x12, 0x30, 0x0a,
    0x14, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
    0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x12, 0x63, 0x75, 0x72,
    0x72, 0x65, 0x6e, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12,
    0x33, 0x0a, 0x05, 0x66, 0x6f, 0x72, 0x74, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1d,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e,
    0x46, 0x6f, 0x72, 0x74, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x44, 0x61, 0x74, 0x61, 0x52, 0x05, 0x66,
    0x6f, 0x72, 0x74, 0x73, 0x12, 0x3d, 0x0a, 0x0c, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x5f, 0x70, 0x6f,
    0x69, 0x6e, 0x74, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x53, 0x70, 0x61, 0x77,
    0x6e, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x52, 0x0b, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x50, 0x6f, 0x69,
    0x6e, 0x74, 0x73, 0x12, 0x27, 0x0a, 0x0f, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x5f, 0x6f,
    0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0e, 0x64, 0x65,
    0x6c, 0x65, 0x74, 0x65, 0x64, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x12, 0x2a, 0x0a, 0x11,
    0x69, 0x73, 0x5f, 0x74, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x6c, 0x69, 0x73,
    0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0f, 0x69, 0x73, 0x54, 0x72, 0x75, 0x6e, 0x63,
    0x61, 0x74, 0x65, 0x64, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x47, 0x0a, 0x0e, 0x66, 0x6f, 0x72, 0x74,
    0x5f, 0x73, 0x75, 0x6d, 0x6d, 0x61, 0x72, 0x69, 0x65, 0x73, 0x18, 0x08, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x20, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61,
    0x70, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x53, 0x75, 0x6d, 0x6d, 0x61,
    0x72, 0x79, 0x52, 0x0d, 0x66, 0x6f, 0x72, 0x74, 0x53, 0x75, 0x6d, 0x6d, 0x61, 0x72, 0x69, 0x65,
    0x73, 0x12, 0x50, 0x0a, 0x16, 0x64, 0x65, 0x63, 0x69, 0x6d, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x73,
    0x70, 0x61, 0x77, 0x6e, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x73, 0x18, 0x09, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x1a, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d,
    0x61, 0x70, 0x2e, 0x53, 0x70, 0x61, 0x77, 0x6e, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x52, 0x14, 0x64,
    0x65, 0x63, 0x69, 0x6d, 0x61, 0x74, 0x65, 0x64, 0x53, 0x70, 0x61, 0x77, 0x6e, 0x50, 0x6f, 0x69,
    0x6e, 0x74, 0x73, 0x12, 0x48, 0x0a, 0x0d, 0x77, 0x69, 0x6c, 0x64, 0x5f, 0x70, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x50, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x2e, 0x57, 0x69, 0x6c, 0x64, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52,
    0x0c, 0x77, 0x69, 0x6c, 0x64, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x73, 0x12, 0x51, 0x0a,
    0x12, 0x63, 0x61, 0x74, 0x63, 0x68, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x73, 0x18, 0x0a, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x2e, 0x4d, 0x61, 0x70, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x11, 0x63,
    0x61, 0x74, 0x63, 0x68, 0x61, 0x62, 0x6c, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x73,
    0x12, 0x4e, 0x0a, 0x0f, 0x6e, 0x65, 0x61, 0x72, 0x62, 0x79, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x73, 0x18, 0x0b, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x2e, 0x4e, 0x65, 0x61, 0x72, 0x62, 0x79, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e,
    0x52, 0x0e, 0x6e, 0x65, 0x61, 0x72, 0x62, 0x79, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x73,
    0x22, 0x46, 0x0a, 0x0a, 0x53, 0x70, 0x61, 0x77, 0x6e, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x12, 0x1a,
    0x0a, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01,
    0x52, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x6c, 0x6f,
    0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x09, 0x6c,
    0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x2a, 0x45, 0x0a, 0x10, 0x4d, 0x61, 0x70, 0x4f,
    0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x10, 0x0a, 0x0c,
    0x55, 0x4e, 0x53, 0x45, 0x54, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x10, 0x00, 0x12, 0x0b,
    0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x12, 0x0a, 0x0e, 0x4c,
    0x4f, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x02, 0x4a,
    0x8c, 0x0b, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x20, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x16, 0x0a, 0x09,
    0x0a, 0x02, 0x0a, 0x00, 0x12, 0x03, 0x03, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x03, 0x0e, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x04, 0x07, 0x0d, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x0e, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x06, 0x00, 0x17, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06,
    0x08, 0x0f, 0x0a, 0x83, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x08, 0x1e,
    0x1a, 0x76, 0x20, 0x53, 0x32, 0x20, 0x67, 0x65, 0x6f, 0x67, 0x72, 0x61, 0x70, 0x68, 0x69, 0x63,
    0x20, 0x61, 0x72, 0x65, 0x61, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63,
    0x65, 0x6c, 0x6c, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x73, 0x20, 0x28, 0x68, 0x74, 0x74, 0x70,
    0x3a, 0x2f, 0x2f, 0x73, 0x32, 0x6d, 0x61, 0x70, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x29, 0x20, 0x28,
    0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x63, 0x6f, 0x64, 0x65, 0x2e, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x61, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65, 0x2f,
    0x70, 0x2f, 0x73, 0x32, 0x2d, 0x67, 0x65, 0x6f, 0x6d, 0x65, 0x74, 0x72, 0x79, 0x2d, 0x6c, 0x69,
    0x62, 0x72, 0x61, 0x72, 0x79, 0x2f, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x08, 0x08, 0x06, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x08, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x08, 0x0f, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x1c,
    0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x08, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x09, 0x08, 0x08, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x09, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x0e, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x09, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x0a, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0a,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0a, 0x11, 0x2e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x2f, 0x34, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x08, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x0b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06,
    0x12, 0x03, 0x0b, 0x11, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x0b, 0x2c, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0b, 0x3b,
    0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0c, 0x08, 0x2c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x0c, 0x18, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x0c, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03,
    0x0d, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x04, 0x0d, 0x08,
    0x0c, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0d, 0x08, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0d, 0x0d, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0d, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0e, 0x08, 0x45, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x06, 0x04, 0x12, 0x03, 0x0e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06,
    0x12, 0x03, 0x0e, 0x11, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x0e, 0x32, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x0e, 0x43,
    0x44, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x0f, 0x08, 0x47, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x0f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x07, 0x06, 0x12, 0x03, 0x0f, 0x11, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x01, 0x12, 0x03, 0x0f, 0x2c, 0x42, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x03, 0x12, 0x03, 0x0f, 0x45, 0x46, 0x0a, 0x2e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03,
    0x12, 0x08, 0x47, 0x1a, 0x21, 0x20, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x69, 0x6e, 0x20, 0x32, 0x20, 0x73, 0x74, 0x65, 0x70, 0x73, 0x20, 0x6f, 0x72, 0x20,
    0x6c, 0x65, 0x73, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12,
    0x03, 0x12, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x06, 0x12, 0x03, 0x12,
    0x11, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x12, 0x35, 0x42,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x12, 0x45, 0x46, 0x0a, 0x2d,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x14, 0x08, 0x4c, 0x1a, 0x20, 0x20, 0x50, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x20, 0x77, 0x69, 0x74, 0x68, 0x69, 0x6e, 0x20, 0x31, 0x20, 0x73,
    0x74, 0x65, 0x70, 0x20, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x6e, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x14, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x09, 0x06, 0x12, 0x03, 0x14, 0x11, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x09, 0x01, 0x12, 0x03, 0x14, 0x34, 0x46, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03,
    0x12, 0x03, 0x14, 0x49, 0x4b, 0x0a, 0x48, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x16,
    0x08, 0x4c, 0x1a, 0x3b, 0x20, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x20, 0x66, 0x61, 0x72,
    0x74, 0x68, 0x65, 0x72, 0x20, 0x61, 0x77, 0x61, 0x79, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x32,
    0x20, 0x73, 0x74, 0x65, 0x70, 0x73, 0x2c, 0x20, 0x62, 0x75, 0x74, 0x20, 0x73, 0x74, 0x69, 0x6c,
    0x6c, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x72, 0x65, 0x61, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x16, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0a, 0x06, 0x12, 0x03, 0x16, 0x11, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x16, 0x37, 0x46, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0a, 0x03, 0x12, 0x03, 0x16, 0x49, 0x4b, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x18,
    0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x18, 0x05, 0x15, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x19, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x19, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x1a, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x1a, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x1a, 0x12,
    0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1b, 0x08, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1b, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x1d, 0x00, 0x20, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x1d,
    0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x08, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x1e, 0x08, 0x1d, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1e, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x0f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x1f, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x1f, 0x08, 0x1e, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1f,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1f, 0x0f, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1f, 0x1b, 0x1c, 0x62, 0x06,
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
