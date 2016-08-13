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
pub struct WildPokemon {
    // message fields
    encounter_id: ::std::option::Option<u64>,
    last_modified_timestamp_ms: ::std::option::Option<i64>,
    latitude: ::std::option::Option<f64>,
    longitude: ::std::option::Option<f64>,
    spawn_point_id: ::protobuf::SingularField<::std::string::String>,
    pokemon_data: ::protobuf::SingularPtrField<super::POGOProtos_Data::PokemonData>,
    time_till_hidden_ms: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WildPokemon {}

impl WildPokemon {
    pub fn new() -> WildPokemon {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WildPokemon {
        static mut instance: ::protobuf::lazy::Lazy<WildPokemon> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WildPokemon,
        };
        unsafe {
            instance.get(|| {
                WildPokemon {
                    encounter_id: ::std::option::Option::None,
                    last_modified_timestamp_ms: ::std::option::Option::None,
                    latitude: ::std::option::Option::None,
                    longitude: ::std::option::Option::None,
                    spawn_point_id: ::protobuf::SingularField::none(),
                    pokemon_data: ::protobuf::SingularPtrField::none(),
                    time_till_hidden_ms: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional fixed64 encounter_id = 1;

    pub fn clear_encounter_id(&mut self) {
        self.encounter_id = ::std::option::Option::None;
    }

    pub fn has_encounter_id(&self) -> bool {
        self.encounter_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encounter_id(&mut self, v: u64) {
        self.encounter_id = ::std::option::Option::Some(v);
    }

    pub fn get_encounter_id(&self) -> u64 {
        self.encounter_id.unwrap_or(0)
    }

    // optional int64 last_modified_timestamp_ms = 2;

    pub fn clear_last_modified_timestamp_ms(&mut self) {
        self.last_modified_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_last_modified_timestamp_ms(&self) -> bool {
        self.last_modified_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_modified_timestamp_ms(&mut self, v: i64) {
        self.last_modified_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_last_modified_timestamp_ms(&self) -> i64 {
        self.last_modified_timestamp_ms.unwrap_or(0)
    }

    // optional double latitude = 3;

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

    // optional double longitude = 4;

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

    // optional string spawn_point_id = 5;

    pub fn clear_spawn_point_id(&mut self) {
        self.spawn_point_id.clear();
    }

    pub fn has_spawn_point_id(&self) -> bool {
        self.spawn_point_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spawn_point_id(&mut self, v: ::std::string::String) {
        self.spawn_point_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spawn_point_id(&mut self) -> &mut ::std::string::String {
        if self.spawn_point_id.is_none() {
            self.spawn_point_id.set_default();
        };
        self.spawn_point_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_spawn_point_id(&mut self) -> ::std::string::String {
        self.spawn_point_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_spawn_point_id(&self) -> &str {
        match self.spawn_point_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .POGOProtos.Data.PokemonData pokemon_data = 7;

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

    // optional int32 time_till_hidden_ms = 11;

    pub fn clear_time_till_hidden_ms(&mut self) {
        self.time_till_hidden_ms = ::std::option::Option::None;
    }

    pub fn has_time_till_hidden_ms(&self) -> bool {
        self.time_till_hidden_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_till_hidden_ms(&mut self, v: i32) {
        self.time_till_hidden_ms = ::std::option::Option::Some(v);
    }

    pub fn get_time_till_hidden_ms(&self) -> i32 {
        self.time_till_hidden_ms.unwrap_or(0)
    }
}

impl ::protobuf::Message for WildPokemon {
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
                    self.encounter_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.last_modified_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.latitude = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.longitude = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.spawn_point_id));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pokemon_data));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.time_till_hidden_ms = ::std::option::Option::Some(tmp);
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
        if self.encounter_id.is_some() {
            my_size += 9;
        };
        for value in &self.last_modified_timestamp_ms {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.latitude.is_some() {
            my_size += 9;
        };
        if self.longitude.is_some() {
            my_size += 9;
        };
        for value in &self.spawn_point_id {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in &self.pokemon_data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.time_till_hidden_ms {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.encounter_id {
            try!(os.write_fixed64(1, v));
        };
        if let Some(v) = self.last_modified_timestamp_ms {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.latitude {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.longitude {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.spawn_point_id.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.pokemon_data.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.time_till_hidden_ms {
            try!(os.write_int32(11, v));
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
        ::std::any::TypeId::of::<WildPokemon>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WildPokemon {
    fn new() -> WildPokemon {
        WildPokemon::new()
    }

    fn descriptor_static(_: ::std::option::Option<WildPokemon>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "encounter_id",
                    WildPokemon::has_encounter_id,
                    WildPokemon::get_encounter_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "last_modified_timestamp_ms",
                    WildPokemon::has_last_modified_timestamp_ms,
                    WildPokemon::get_last_modified_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "latitude",
                    WildPokemon::has_latitude,
                    WildPokemon::get_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "longitude",
                    WildPokemon::has_longitude,
                    WildPokemon::get_longitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "spawn_point_id",
                    WildPokemon::has_spawn_point_id,
                    WildPokemon::get_spawn_point_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pokemon_data",
                    WildPokemon::has_pokemon_data,
                    WildPokemon::get_pokemon_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "time_till_hidden_ms",
                    WildPokemon::has_time_till_hidden_ms,
                    WildPokemon::get_time_till_hidden_ms,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WildPokemon>(
                    "WildPokemon",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WildPokemon {
    fn clear(&mut self) {
        self.clear_encounter_id();
        self.clear_last_modified_timestamp_ms();
        self.clear_latitude();
        self.clear_longitude();
        self.clear_spawn_point_id();
        self.clear_pokemon_data();
        self.clear_time_till_hidden_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WildPokemon {
    fn eq(&self, other: &WildPokemon) -> bool {
        self.encounter_id == other.encounter_id &&
        self.last_modified_timestamp_ms == other.last_modified_timestamp_ms &&
        self.latitude == other.latitude &&
        self.longitude == other.longitude &&
        self.spawn_point_id == other.spawn_point_id &&
        self.pokemon_data == other.pokemon_data &&
        self.time_till_hidden_ms == other.time_till_hidden_ms &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WildPokemon {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct NearbyPokemon {
    // message fields
    pokemon_id: ::std::option::Option<super::POGOProtos_Enums::PokemonId>,
    distance_in_meters: ::std::option::Option<f32>,
    encounter_id: ::std::option::Option<u64>,
    fort_id: ::protobuf::SingularField<::std::string::String>,
    fort_image_url: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NearbyPokemon {}

impl NearbyPokemon {
    pub fn new() -> NearbyPokemon {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NearbyPokemon {
        static mut instance: ::protobuf::lazy::Lazy<NearbyPokemon> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NearbyPokemon,
        };
        unsafe {
            instance.get(|| {
                NearbyPokemon {
                    pokemon_id: ::std::option::Option::None,
                    distance_in_meters: ::std::option::Option::None,
                    encounter_id: ::std::option::Option::None,
                    fort_id: ::protobuf::SingularField::none(),
                    fort_image_url: ::protobuf::SingularField::none(),
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

    // optional float distance_in_meters = 2;

    pub fn clear_distance_in_meters(&mut self) {
        self.distance_in_meters = ::std::option::Option::None;
    }

    pub fn has_distance_in_meters(&self) -> bool {
        self.distance_in_meters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_distance_in_meters(&mut self, v: f32) {
        self.distance_in_meters = ::std::option::Option::Some(v);
    }

    pub fn get_distance_in_meters(&self) -> f32 {
        self.distance_in_meters.unwrap_or(0.)
    }

    // optional fixed64 encounter_id = 3;

    pub fn clear_encounter_id(&mut self) {
        self.encounter_id = ::std::option::Option::None;
    }

    pub fn has_encounter_id(&self) -> bool {
        self.encounter_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encounter_id(&mut self, v: u64) {
        self.encounter_id = ::std::option::Option::Some(v);
    }

    pub fn get_encounter_id(&self) -> u64 {
        self.encounter_id.unwrap_or(0)
    }

    // optional string fort_id = 4;

    pub fn clear_fort_id(&mut self) {
        self.fort_id.clear();
    }

    pub fn has_fort_id(&self) -> bool {
        self.fort_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fort_id(&mut self, v: ::std::string::String) {
        self.fort_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fort_id(&mut self) -> &mut ::std::string::String {
        if self.fort_id.is_none() {
            self.fort_id.set_default();
        };
        self.fort_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_fort_id(&mut self) -> ::std::string::String {
        self.fort_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_fort_id(&self) -> &str {
        match self.fort_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string fort_image_url = 5;

    pub fn clear_fort_image_url(&mut self) {
        self.fort_image_url.clear();
    }

    pub fn has_fort_image_url(&self) -> bool {
        self.fort_image_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fort_image_url(&mut self, v: ::std::string::String) {
        self.fort_image_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fort_image_url(&mut self) -> &mut ::std::string::String {
        if self.fort_image_url.is_none() {
            self.fort_image_url.set_default();
        };
        self.fort_image_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_fort_image_url(&mut self) -> ::std::string::String {
        self.fort_image_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_fort_image_url(&self) -> &str {
        match self.fort_image_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for NearbyPokemon {
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.distance_in_meters = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.encounter_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fort_id));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fort_image_url));
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
        if self.distance_in_meters.is_some() {
            my_size += 5;
        };
        if self.encounter_id.is_some() {
            my_size += 9;
        };
        for value in &self.fort_id {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.fort_image_url {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pokemon_id {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.distance_in_meters {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.encounter_id {
            try!(os.write_fixed64(3, v));
        };
        if let Some(v) = self.fort_id.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.fort_image_url.as_ref() {
            try!(os.write_string(5, &v));
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
        ::std::any::TypeId::of::<NearbyPokemon>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NearbyPokemon {
    fn new() -> NearbyPokemon {
        NearbyPokemon::new()
    }

    fn descriptor_static(_: ::std::option::Option<NearbyPokemon>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "pokemon_id",
                    NearbyPokemon::has_pokemon_id,
                    NearbyPokemon::get_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "distance_in_meters",
                    NearbyPokemon::has_distance_in_meters,
                    NearbyPokemon::get_distance_in_meters,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "encounter_id",
                    NearbyPokemon::has_encounter_id,
                    NearbyPokemon::get_encounter_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "fort_id",
                    NearbyPokemon::has_fort_id,
                    NearbyPokemon::get_fort_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "fort_image_url",
                    NearbyPokemon::has_fort_image_url,
                    NearbyPokemon::get_fort_image_url,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NearbyPokemon>(
                    "NearbyPokemon",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NearbyPokemon {
    fn clear(&mut self) {
        self.clear_pokemon_id();
        self.clear_distance_in_meters();
        self.clear_encounter_id();
        self.clear_fort_id();
        self.clear_fort_image_url();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for NearbyPokemon {
    fn eq(&self, other: &NearbyPokemon) -> bool {
        self.pokemon_id == other.pokemon_id &&
        self.distance_in_meters == other.distance_in_meters &&
        self.encounter_id == other.encounter_id &&
        self.fort_id == other.fort_id &&
        self.fort_image_url == other.fort_image_url &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for NearbyPokemon {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct MapPokemon {
    // message fields
    spawn_point_id: ::protobuf::SingularField<::std::string::String>,
    encounter_id: ::std::option::Option<u64>,
    pokemon_id: ::std::option::Option<super::POGOProtos_Enums::PokemonId>,
    expiration_timestamp_ms: ::std::option::Option<i64>,
    latitude: ::std::option::Option<f64>,
    longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MapPokemon {}

impl MapPokemon {
    pub fn new() -> MapPokemon {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MapPokemon {
        static mut instance: ::protobuf::lazy::Lazy<MapPokemon> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MapPokemon,
        };
        unsafe {
            instance.get(|| {
                MapPokemon {
                    spawn_point_id: ::protobuf::SingularField::none(),
                    encounter_id: ::std::option::Option::None,
                    pokemon_id: ::std::option::Option::None,
                    expiration_timestamp_ms: ::std::option::Option::None,
                    latitude: ::std::option::Option::None,
                    longitude: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string spawn_point_id = 1;

    pub fn clear_spawn_point_id(&mut self) {
        self.spawn_point_id.clear();
    }

    pub fn has_spawn_point_id(&self) -> bool {
        self.spawn_point_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spawn_point_id(&mut self, v: ::std::string::String) {
        self.spawn_point_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spawn_point_id(&mut self) -> &mut ::std::string::String {
        if self.spawn_point_id.is_none() {
            self.spawn_point_id.set_default();
        };
        self.spawn_point_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_spawn_point_id(&mut self) -> ::std::string::String {
        self.spawn_point_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_spawn_point_id(&self) -> &str {
        match self.spawn_point_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional fixed64 encounter_id = 2;

    pub fn clear_encounter_id(&mut self) {
        self.encounter_id = ::std::option::Option::None;
    }

    pub fn has_encounter_id(&self) -> bool {
        self.encounter_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encounter_id(&mut self, v: u64) {
        self.encounter_id = ::std::option::Option::Some(v);
    }

    pub fn get_encounter_id(&self) -> u64 {
        self.encounter_id.unwrap_or(0)
    }

    // optional .POGOProtos.Enums.PokemonId pokemon_id = 3;

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

    // optional int64 expiration_timestamp_ms = 4;

    pub fn clear_expiration_timestamp_ms(&mut self) {
        self.expiration_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_expiration_timestamp_ms(&self) -> bool {
        self.expiration_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expiration_timestamp_ms(&mut self, v: i64) {
        self.expiration_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_expiration_timestamp_ms(&self) -> i64 {
        self.expiration_timestamp_ms.unwrap_or(0)
    }

    // optional double latitude = 5;

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

    // optional double longitude = 6;

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

impl ::protobuf::Message for MapPokemon {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.spawn_point_id));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.encounter_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.pokemon_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.expiration_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.latitude = ::std::option::Option::Some(tmp);
                },
                6 => {
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
        for value in &self.spawn_point_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.encounter_id.is_some() {
            my_size += 9;
        };
        for value in &self.pokemon_id {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in &self.expiration_timestamp_ms {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
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
        if let Some(v) = self.spawn_point_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.encounter_id {
            try!(os.write_fixed64(2, v));
        };
        if let Some(v) = self.pokemon_id {
            try!(os.write_enum(3, v.value()));
        };
        if let Some(v) = self.expiration_timestamp_ms {
            try!(os.write_int64(4, v));
        };
        if let Some(v) = self.latitude {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.longitude {
            try!(os.write_double(6, v));
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
        ::std::any::TypeId::of::<MapPokemon>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MapPokemon {
    fn new() -> MapPokemon {
        MapPokemon::new()
    }

    fn descriptor_static(_: ::std::option::Option<MapPokemon>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "spawn_point_id",
                    MapPokemon::has_spawn_point_id,
                    MapPokemon::get_spawn_point_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "encounter_id",
                    MapPokemon::has_encounter_id,
                    MapPokemon::get_encounter_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "pokemon_id",
                    MapPokemon::has_pokemon_id,
                    MapPokemon::get_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "expiration_timestamp_ms",
                    MapPokemon::has_expiration_timestamp_ms,
                    MapPokemon::get_expiration_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "latitude",
                    MapPokemon::has_latitude,
                    MapPokemon::get_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "longitude",
                    MapPokemon::has_longitude,
                    MapPokemon::get_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MapPokemon>(
                    "MapPokemon",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MapPokemon {
    fn clear(&mut self) {
        self.clear_spawn_point_id();
        self.clear_encounter_id();
        self.clear_pokemon_id();
        self.clear_expiration_timestamp_ms();
        self.clear_latitude();
        self.clear_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for MapPokemon {
    fn eq(&self, other: &MapPokemon) -> bool {
        self.spawn_point_id == other.spawn_point_id &&
        self.encounter_id == other.encounter_id &&
        self.pokemon_id == other.pokemon_id &&
        self.expiration_timestamp_ms == other.expiration_timestamp_ms &&
        self.latitude == other.latitude &&
        self.longitude == other.longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for MapPokemon {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1c, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70,
    0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x16,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x50,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x1a, 0x15, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x16, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x00, 0x50, 0x01, 0x22, 0xbd, 0x02, 0x0a, 0x0b, 0x57, 0x69,
    0x6c, 0x64, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x12, 0x21, 0x0a, 0x0c, 0x65, 0x6e, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52,
    0x0b, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x49, 0x64, 0x12, 0x3b, 0x0a, 0x1a,
    0x6c, 0x61, 0x73, 0x74, 0x5f, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x5f, 0x74, 0x69,
    0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03,
    0x52, 0x17, 0x6c, 0x61, 0x73, 0x74, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x54, 0x69,
    0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x6c, 0x61, 0x74,
    0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x08, 0x6c, 0x61, 0x74,
    0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75,
    0x64, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x52, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74,
    0x75, 0x64, 0x65, 0x12, 0x24, 0x0a, 0x0e, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x5f, 0x70, 0x6f, 0x69,
    0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x73, 0x70, 0x61,
    0x77, 0x6e, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x3f, 0x0a, 0x0c, 0x70, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1c, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x52, 0x0b, 0x70,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x12, 0x2d, 0x0a, 0x13, 0x74, 0x69,
    0x6d, 0x65, 0x5f, 0x74, 0x69, 0x6c, 0x6c, 0x5f, 0x68, 0x69, 0x64, 0x64, 0x65, 0x6e, 0x5f, 0x6d,
    0x73, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x05, 0x52, 0x10, 0x74, 0x69, 0x6d, 0x65, 0x54, 0x69, 0x6c,
    0x6c, 0x48, 0x69, 0x64, 0x64, 0x65, 0x6e, 0x4d, 0x73, 0x22, 0xdb, 0x01, 0x0a, 0x0d, 0x4e, 0x65,
    0x61, 0x72, 0x62, 0x79, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x12, 0x3a, 0x0a, 0x0a, 0x70,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75,
    0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x52, 0x09, 0x70, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x2c, 0x0a, 0x12, 0x64, 0x69, 0x73, 0x74, 0x61,
    0x6e, 0x63, 0x65, 0x5f, 0x69, 0x6e, 0x5f, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x02, 0x52, 0x10, 0x64, 0x69, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x49, 0x6e, 0x4d,
    0x65, 0x74, 0x65, 0x72, 0x73, 0x12, 0x21, 0x0a, 0x0c, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0b, 0x65, 0x6e, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x49, 0x64, 0x12, 0x17, 0x0a, 0x07, 0x66, 0x6f, 0x72, 0x74,
    0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x66, 0x6f, 0x72, 0x74, 0x49,
    0x64, 0x12, 0x24, 0x0a, 0x0e, 0x66, 0x6f, 0x72, 0x74, 0x5f, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x5f,
    0x75, 0x72, 0x6c, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x66, 0x6f, 0x72, 0x74, 0x49,
    0x6d, 0x61, 0x67, 0x65, 0x55, 0x72, 0x6c, 0x22, 0x83, 0x02, 0x0a, 0x0a, 0x4d, 0x61, 0x70, 0x50,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x12, 0x24, 0x0a, 0x0e, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x5f,
    0x70, 0x6f, 0x69, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c,
    0x73, 0x70, 0x61, 0x77, 0x6e, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c,
    0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x06, 0x52, 0x0b, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x49, 0x64, 0x12,
    0x3a, 0x0a, 0x0a, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64,
    0x52, 0x09, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x36, 0x0a, 0x17, 0x65,
    0x78, 0x70, 0x69, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x52, 0x15, 0x65, 0x78,
    0x70, 0x69, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d,
    0x70, 0x4d, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x01, 0x52, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12,
    0x1c, 0x0a, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x01, 0x52, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x4a, 0xaf, 0x0b,
    0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x1e, 0x0a, 0x09, 0x0a, 0x02,
    0x0a, 0x00, 0x12, 0x03, 0x03, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03,
    0x0e, 0x25, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x04, 0x07, 0x0d, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x0e, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x06, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x13,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x08, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x07, 0x08, 0x06, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x07, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x07, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x08, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x08, 0x08,
    0x07, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x08, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x08, 0x0e, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x08, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x09, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x04, 0x09, 0x08, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x09, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x09, 0x0f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x09,
    0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x08, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x0a, 0x08, 0x09, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0a, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0a, 0x0f, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x0a, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04,
    0x12, 0x03, 0x0b, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x04,
    0x0b, 0x08, 0x0a, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0b,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0b, 0x0f, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0b, 0x20, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0c, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x04, 0x12, 0x04, 0x0c, 0x08, 0x0b, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x05, 0x06, 0x12, 0x03, 0x0c, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x0c, 0x25, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x0c, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0d, 0x08,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x04, 0x0d, 0x08, 0x0c, 0x36,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x0d, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0d, 0x0e, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x0d, 0x24, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x0f, 0x00, 0x15, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0f,
    0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x10, 0x08, 0x33, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x10, 0x08, 0x0f, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x10, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x24, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x10, 0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x11, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x11, 0x08, 0x10, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x11,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x11, 0x0e, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x11, 0x23, 0x24, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x12, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0x12, 0x08, 0x11, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x12, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x12, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x12, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x13, 0x08,
    0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x04, 0x13, 0x08, 0x12, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x13, 0x08, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x13, 0x0f, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x13, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x04, 0x12, 0x03, 0x14, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x14, 0x08, 0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x14, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x14,
    0x0f, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x14, 0x20, 0x21,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x16, 0x00, 0x1e, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x16, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00,
    0x12, 0x03, 0x17, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x17, 0x08, 0x16, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x17,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17, 0x0f, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x17, 0x20, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x18, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x18, 0x08, 0x17, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x18, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x18, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x18, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x19, 0x08,
    0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x04, 0x19, 0x08, 0x18, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03, 0x19, 0x08, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x19, 0x24, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x19, 0x31, 0x32, 0x0a, 0x3e, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x1b, 0x08, 0x2a, 0x1a, 0x31, 0x20, 0x41, 0x66, 0x74, 0x65, 0x72, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2c, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x62, 0x65, 0x20, 0x67, 0x6f, 0x6e, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x03, 0x04, 0x12, 0x04, 0x1b, 0x08, 0x19, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x1b, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x1b, 0x0e, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x1b, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1c, 0x08, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1c, 0x08, 0x1b, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x1c, 0x08, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1c, 0x0f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1c, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x1d, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04, 0x12,
    0x04, 0x1d, 0x08, 0x1c, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x05, 0x12, 0x03,
    0x1d, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1d, 0x0f,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1d, 0x1b, 0x1c, 0x62,
    0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
