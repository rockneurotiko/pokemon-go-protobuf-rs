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
pub struct GetPlayerResponse {
    // message fields
    success: ::std::option::Option<bool>,
    player_data: ::protobuf::SingularPtrField<super::POGOProtos_Data::PlayerData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetPlayerResponse {}

impl GetPlayerResponse {
    pub fn new() -> GetPlayerResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetPlayerResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetPlayerResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetPlayerResponse,
        };
        unsafe {
            instance.get(|| {
                GetPlayerResponse {
                    success: ::std::option::Option::None,
                    player_data: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // optional .POGOProtos.Data.PlayerData player_data = 2;

    pub fn clear_player_data(&mut self) {
        self.player_data.clear();
    }

    pub fn has_player_data(&self) -> bool {
        self.player_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_data(&mut self, v: super::POGOProtos_Data::PlayerData) {
        self.player_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_data(&mut self) -> &mut super::POGOProtos_Data::PlayerData {
        if self.player_data.is_none() {
            self.player_data.set_default();
        };
        self.player_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_data(&mut self) -> super::POGOProtos_Data::PlayerData {
        self.player_data.take().unwrap_or_else(|| super::POGOProtos_Data::PlayerData::new())
    }

    pub fn get_player_data(&self) -> &super::POGOProtos_Data::PlayerData {
        self.player_data.as_ref().unwrap_or_else(|| super::POGOProtos_Data::PlayerData::default_instance())
    }
}

impl ::protobuf::Message for GetPlayerResponse {
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
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_data));
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
        if self.success.is_some() {
            my_size += 2;
        };
        for value in &self.player_data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.player_data.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<GetPlayerResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetPlayerResponse {
    fn new() -> GetPlayerResponse {
        GetPlayerResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetPlayerResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    GetPlayerResponse::has_success,
                    GetPlayerResponse::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "player_data",
                    GetPlayerResponse::has_player_data,
                    GetPlayerResponse::get_player_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetPlayerResponse>(
                    "GetPlayerResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetPlayerResponse {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_player_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetPlayerResponse {
    fn eq(&self, other: &GetPlayerResponse) -> bool {
        self.success == other.success &&
        self.player_data == other.player_data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetPlayerResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetBuddyPokemonResponse {
    // message fields
    result: ::std::option::Option<SetBuddyPokemonResponse_Result>,
    updated_buddy: ::protobuf::SingularPtrField<super::POGOProtos_Data::BuddyPokemon>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetBuddyPokemonResponse {}

impl SetBuddyPokemonResponse {
    pub fn new() -> SetBuddyPokemonResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetBuddyPokemonResponse {
        static mut instance: ::protobuf::lazy::Lazy<SetBuddyPokemonResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetBuddyPokemonResponse,
        };
        unsafe {
            instance.get(|| {
                SetBuddyPokemonResponse {
                    result: ::std::option::Option::None,
                    updated_buddy: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.SetBuddyPokemonResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: SetBuddyPokemonResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> SetBuddyPokemonResponse_Result {
        self.result.unwrap_or(SetBuddyPokemonResponse_Result::UNEST)
    }

    // optional .POGOProtos.Data.BuddyPokemon updated_buddy = 2;

    pub fn clear_updated_buddy(&mut self) {
        self.updated_buddy.clear();
    }

    pub fn has_updated_buddy(&self) -> bool {
        self.updated_buddy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_updated_buddy(&mut self, v: super::POGOProtos_Data::BuddyPokemon) {
        self.updated_buddy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_updated_buddy(&mut self) -> &mut super::POGOProtos_Data::BuddyPokemon {
        if self.updated_buddy.is_none() {
            self.updated_buddy.set_default();
        };
        self.updated_buddy.as_mut().unwrap()
    }

    // Take field
    pub fn take_updated_buddy(&mut self) -> super::POGOProtos_Data::BuddyPokemon {
        self.updated_buddy.take().unwrap_or_else(|| super::POGOProtos_Data::BuddyPokemon::new())
    }

    pub fn get_updated_buddy(&self) -> &super::POGOProtos_Data::BuddyPokemon {
        self.updated_buddy.as_ref().unwrap_or_else(|| super::POGOProtos_Data::BuddyPokemon::default_instance())
    }
}

impl ::protobuf::Message for SetBuddyPokemonResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.updated_buddy));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.updated_buddy {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.updated_buddy.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<SetBuddyPokemonResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetBuddyPokemonResponse {
    fn new() -> SetBuddyPokemonResponse {
        SetBuddyPokemonResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetBuddyPokemonResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    SetBuddyPokemonResponse::has_result,
                    SetBuddyPokemonResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "updated_buddy",
                    SetBuddyPokemonResponse::has_updated_buddy,
                    SetBuddyPokemonResponse::get_updated_buddy,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetBuddyPokemonResponse>(
                    "SetBuddyPokemonResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetBuddyPokemonResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_updated_buddy();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetBuddyPokemonResponse {
    fn eq(&self, other: &SetBuddyPokemonResponse) -> bool {
        self.result == other.result &&
        self.updated_buddy == other.updated_buddy &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetBuddyPokemonResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SetBuddyPokemonResponse_Result {
    UNEST = 0,
    SUCCESS = 1,
    ERROR_POKEMON_DEPLOYED = 2,
    ERROR_POKEMON_NOT_OWNED = 3,
    ERROR_POKEMON_IS_EGG = 4,
}

impl ::protobuf::ProtobufEnum for SetBuddyPokemonResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SetBuddyPokemonResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(SetBuddyPokemonResponse_Result::UNEST),
            1 => ::std::option::Option::Some(SetBuddyPokemonResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(SetBuddyPokemonResponse_Result::ERROR_POKEMON_DEPLOYED),
            3 => ::std::option::Option::Some(SetBuddyPokemonResponse_Result::ERROR_POKEMON_NOT_OWNED),
            4 => ::std::option::Option::Some(SetBuddyPokemonResponse_Result::ERROR_POKEMON_IS_EGG),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SetBuddyPokemonResponse_Result] = &[
            SetBuddyPokemonResponse_Result::UNEST,
            SetBuddyPokemonResponse_Result::SUCCESS,
            SetBuddyPokemonResponse_Result::ERROR_POKEMON_DEPLOYED,
            SetBuddyPokemonResponse_Result::ERROR_POKEMON_NOT_OWNED,
            SetBuddyPokemonResponse_Result::ERROR_POKEMON_IS_EGG,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<SetBuddyPokemonResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SetBuddyPokemonResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SetBuddyPokemonResponse_Result {
}

#[derive(Clone,Default)]
pub struct GetHatchedEggsResponse {
    // message fields
    success: ::std::option::Option<bool>,
    pokemon_id: ::std::vec::Vec<u64>,
    experience_awarded: ::std::vec::Vec<i32>,
    candy_awarded: ::std::vec::Vec<i32>,
    stardust_awarded: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetHatchedEggsResponse {}

impl GetHatchedEggsResponse {
    pub fn new() -> GetHatchedEggsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetHatchedEggsResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetHatchedEggsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetHatchedEggsResponse,
        };
        unsafe {
            instance.get(|| {
                GetHatchedEggsResponse {
                    success: ::std::option::Option::None,
                    pokemon_id: ::std::vec::Vec::new(),
                    experience_awarded: ::std::vec::Vec::new(),
                    candy_awarded: ::std::vec::Vec::new(),
                    stardust_awarded: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // repeated fixed64 pokemon_id = 2;

    pub fn clear_pokemon_id(&mut self) {
        self.pokemon_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_pokemon_id(&mut self, v: ::std::vec::Vec<u64>) {
        self.pokemon_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pokemon_id(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.pokemon_id
    }

    // Take field
    pub fn take_pokemon_id(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.pokemon_id, ::std::vec::Vec::new())
    }

    pub fn get_pokemon_id(&self) -> &[u64] {
        &self.pokemon_id
    }

    // repeated int32 experience_awarded = 3;

    pub fn clear_experience_awarded(&mut self) {
        self.experience_awarded.clear();
    }

    // Param is passed by value, moved
    pub fn set_experience_awarded(&mut self, v: ::std::vec::Vec<i32>) {
        self.experience_awarded = v;
    }

    // Mutable pointer to the field.
    pub fn mut_experience_awarded(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.experience_awarded
    }

    // Take field
    pub fn take_experience_awarded(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.experience_awarded, ::std::vec::Vec::new())
    }

    pub fn get_experience_awarded(&self) -> &[i32] {
        &self.experience_awarded
    }

    // repeated int32 candy_awarded = 4;

    pub fn clear_candy_awarded(&mut self) {
        self.candy_awarded.clear();
    }

    // Param is passed by value, moved
    pub fn set_candy_awarded(&mut self, v: ::std::vec::Vec<i32>) {
        self.candy_awarded = v;
    }

    // Mutable pointer to the field.
    pub fn mut_candy_awarded(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.candy_awarded
    }

    // Take field
    pub fn take_candy_awarded(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.candy_awarded, ::std::vec::Vec::new())
    }

    pub fn get_candy_awarded(&self) -> &[i32] {
        &self.candy_awarded
    }

    // repeated int32 stardust_awarded = 5;

    pub fn clear_stardust_awarded(&mut self) {
        self.stardust_awarded.clear();
    }

    // Param is passed by value, moved
    pub fn set_stardust_awarded(&mut self, v: ::std::vec::Vec<i32>) {
        self.stardust_awarded = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stardust_awarded(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.stardust_awarded
    }

    // Take field
    pub fn take_stardust_awarded(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.stardust_awarded, ::std::vec::Vec::new())
    }

    pub fn get_stardust_awarded(&self) -> &[i32] {
        &self.stardust_awarded
    }
}

impl ::protobuf::Message for GetHatchedEggsResponse {
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
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.pokemon_id));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.experience_awarded));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.candy_awarded));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.stardust_awarded));
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
        if self.success.is_some() {
            my_size += 2;
        };
        if !self.pokemon_id.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.pokemon_id.len() as u32) + (self.pokemon_id.len() * 8) as u32;
        };
        for value in &self.experience_awarded {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.candy_awarded {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.stardust_awarded {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            try!(os.write_bool(1, v));
        };
        if !self.pokemon_id.is_empty() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.pokemon_id.len() * 8) as u32));
            for v in &self.pokemon_id {
                try!(os.write_fixed64_no_tag(*v));
            };
        };
        for v in &self.experience_awarded {
            try!(os.write_int32(3, *v));
        };
        for v in &self.candy_awarded {
            try!(os.write_int32(4, *v));
        };
        for v in &self.stardust_awarded {
            try!(os.write_int32(5, *v));
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
        ::std::any::TypeId::of::<GetHatchedEggsResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetHatchedEggsResponse {
    fn new() -> GetHatchedEggsResponse {
        GetHatchedEggsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetHatchedEggsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    GetHatchedEggsResponse::has_success,
                    GetHatchedEggsResponse::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "pokemon_id",
                    GetHatchedEggsResponse::get_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "experience_awarded",
                    GetHatchedEggsResponse::get_experience_awarded,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "candy_awarded",
                    GetHatchedEggsResponse::get_candy_awarded,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "stardust_awarded",
                    GetHatchedEggsResponse::get_stardust_awarded,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetHatchedEggsResponse>(
                    "GetHatchedEggsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetHatchedEggsResponse {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_pokemon_id();
        self.clear_experience_awarded();
        self.clear_candy_awarded();
        self.clear_stardust_awarded();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetHatchedEggsResponse {
    fn eq(&self, other: &GetHatchedEggsResponse) -> bool {
        self.success == other.success &&
        self.pokemon_id == other.pokemon_id &&
        self.experience_awarded == other.experience_awarded &&
        self.candy_awarded == other.candy_awarded &&
        self.stardust_awarded == other.stardust_awarded &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetHatchedEggsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AttackGymResponse {
    // message fields
    result: ::std::option::Option<AttackGymResponse_Result>,
    battle_log: ::protobuf::SingularPtrField<super::POGOProtos_Data_Battle::BattleLog>,
    battle_id: ::protobuf::SingularField<::std::string::String>,
    active_defender: ::protobuf::SingularPtrField<super::POGOProtos_Data_Battle::BattlePokemonInfo>,
    active_attacker: ::protobuf::SingularPtrField<super::POGOProtos_Data_Battle::BattlePokemonInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AttackGymResponse {}

impl AttackGymResponse {
    pub fn new() -> AttackGymResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AttackGymResponse {
        static mut instance: ::protobuf::lazy::Lazy<AttackGymResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AttackGymResponse,
        };
        unsafe {
            instance.get(|| {
                AttackGymResponse {
                    result: ::std::option::Option::None,
                    battle_log: ::protobuf::SingularPtrField::none(),
                    battle_id: ::protobuf::SingularField::none(),
                    active_defender: ::protobuf::SingularPtrField::none(),
                    active_attacker: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.AttackGymResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: AttackGymResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> AttackGymResponse_Result {
        self.result.unwrap_or(AttackGymResponse_Result::UNSET)
    }

    // optional .POGOProtos.Data.Battle.BattleLog battle_log = 2;

    pub fn clear_battle_log(&mut self) {
        self.battle_log.clear();
    }

    pub fn has_battle_log(&self) -> bool {
        self.battle_log.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battle_log(&mut self, v: super::POGOProtos_Data_Battle::BattleLog) {
        self.battle_log = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_battle_log(&mut self) -> &mut super::POGOProtos_Data_Battle::BattleLog {
        if self.battle_log.is_none() {
            self.battle_log.set_default();
        };
        self.battle_log.as_mut().unwrap()
    }

    // Take field
    pub fn take_battle_log(&mut self) -> super::POGOProtos_Data_Battle::BattleLog {
        self.battle_log.take().unwrap_or_else(|| super::POGOProtos_Data_Battle::BattleLog::new())
    }

    pub fn get_battle_log(&self) -> &super::POGOProtos_Data_Battle::BattleLog {
        self.battle_log.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Battle::BattleLog::default_instance())
    }

    // optional string battle_id = 3;

    pub fn clear_battle_id(&mut self) {
        self.battle_id.clear();
    }

    pub fn has_battle_id(&self) -> bool {
        self.battle_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battle_id(&mut self, v: ::std::string::String) {
        self.battle_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_battle_id(&mut self) -> &mut ::std::string::String {
        if self.battle_id.is_none() {
            self.battle_id.set_default();
        };
        self.battle_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_battle_id(&mut self) -> ::std::string::String {
        self.battle_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_battle_id(&self) -> &str {
        match self.battle_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .POGOProtos.Data.Battle.BattlePokemonInfo active_defender = 4;

    pub fn clear_active_defender(&mut self) {
        self.active_defender.clear();
    }

    pub fn has_active_defender(&self) -> bool {
        self.active_defender.is_some()
    }

    // Param is passed by value, moved
    pub fn set_active_defender(&mut self, v: super::POGOProtos_Data_Battle::BattlePokemonInfo) {
        self.active_defender = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_active_defender(&mut self) -> &mut super::POGOProtos_Data_Battle::BattlePokemonInfo {
        if self.active_defender.is_none() {
            self.active_defender.set_default();
        };
        self.active_defender.as_mut().unwrap()
    }

    // Take field
    pub fn take_active_defender(&mut self) -> super::POGOProtos_Data_Battle::BattlePokemonInfo {
        self.active_defender.take().unwrap_or_else(|| super::POGOProtos_Data_Battle::BattlePokemonInfo::new())
    }

    pub fn get_active_defender(&self) -> &super::POGOProtos_Data_Battle::BattlePokemonInfo {
        self.active_defender.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Battle::BattlePokemonInfo::default_instance())
    }

    // optional .POGOProtos.Data.Battle.BattlePokemonInfo active_attacker = 5;

    pub fn clear_active_attacker(&mut self) {
        self.active_attacker.clear();
    }

    pub fn has_active_attacker(&self) -> bool {
        self.active_attacker.is_some()
    }

    // Param is passed by value, moved
    pub fn set_active_attacker(&mut self, v: super::POGOProtos_Data_Battle::BattlePokemonInfo) {
        self.active_attacker = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_active_attacker(&mut self) -> &mut super::POGOProtos_Data_Battle::BattlePokemonInfo {
        if self.active_attacker.is_none() {
            self.active_attacker.set_default();
        };
        self.active_attacker.as_mut().unwrap()
    }

    // Take field
    pub fn take_active_attacker(&mut self) -> super::POGOProtos_Data_Battle::BattlePokemonInfo {
        self.active_attacker.take().unwrap_or_else(|| super::POGOProtos_Data_Battle::BattlePokemonInfo::new())
    }

    pub fn get_active_attacker(&self) -> &super::POGOProtos_Data_Battle::BattlePokemonInfo {
        self.active_attacker.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Battle::BattlePokemonInfo::default_instance())
    }
}

impl ::protobuf::Message for AttackGymResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.battle_log));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.battle_id));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.active_defender));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.active_attacker));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.battle_log {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.battle_id {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.active_defender {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.active_attacker {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.battle_log.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.battle_id.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.active_defender.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.active_attacker.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<AttackGymResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AttackGymResponse {
    fn new() -> AttackGymResponse {
        AttackGymResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AttackGymResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    AttackGymResponse::has_result,
                    AttackGymResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "battle_log",
                    AttackGymResponse::has_battle_log,
                    AttackGymResponse::get_battle_log,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "battle_id",
                    AttackGymResponse::has_battle_id,
                    AttackGymResponse::get_battle_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "active_defender",
                    AttackGymResponse::has_active_defender,
                    AttackGymResponse::get_active_defender,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "active_attacker",
                    AttackGymResponse::has_active_attacker,
                    AttackGymResponse::get_active_attacker,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AttackGymResponse>(
                    "AttackGymResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AttackGymResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_battle_log();
        self.clear_battle_id();
        self.clear_active_defender();
        self.clear_active_attacker();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AttackGymResponse {
    fn eq(&self, other: &AttackGymResponse) -> bool {
        self.result == other.result &&
        self.battle_log == other.battle_log &&
        self.battle_id == other.battle_id &&
        self.active_defender == other.active_defender &&
        self.active_attacker == other.active_attacker &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AttackGymResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AttackGymResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    ERROR_INVALID_ATTACK_ACTIONS = 2,
    ERROR_NOT_IN_RANGE = 3,
}

impl ::protobuf::ProtobufEnum for AttackGymResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AttackGymResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(AttackGymResponse_Result::UNSET),
            1 => ::std::option::Option::Some(AttackGymResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(AttackGymResponse_Result::ERROR_INVALID_ATTACK_ACTIONS),
            3 => ::std::option::Option::Some(AttackGymResponse_Result::ERROR_NOT_IN_RANGE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AttackGymResponse_Result] = &[
            AttackGymResponse_Result::UNSET,
            AttackGymResponse_Result::SUCCESS,
            AttackGymResponse_Result::ERROR_INVALID_ATTACK_ACTIONS,
            AttackGymResponse_Result::ERROR_NOT_IN_RANGE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<AttackGymResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AttackGymResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AttackGymResponse_Result {
}

#[derive(Clone,Default)]
pub struct GetBuddyWalkedResponse {
    // message fields
    success: ::std::option::Option<bool>,
    family_candy_id: ::std::option::Option<super::POGOProtos_Enums::PokemonFamilyId>,
    candy_earned_count: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBuddyWalkedResponse {}

impl GetBuddyWalkedResponse {
    pub fn new() -> GetBuddyWalkedResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBuddyWalkedResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetBuddyWalkedResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBuddyWalkedResponse,
        };
        unsafe {
            instance.get(|| {
                GetBuddyWalkedResponse {
                    success: ::std::option::Option::None,
                    family_candy_id: ::std::option::Option::None,
                    candy_earned_count: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // optional .POGOProtos.Enums.PokemonFamilyId family_candy_id = 2;

    pub fn clear_family_candy_id(&mut self) {
        self.family_candy_id = ::std::option::Option::None;
    }

    pub fn has_family_candy_id(&self) -> bool {
        self.family_candy_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_family_candy_id(&mut self, v: super::POGOProtos_Enums::PokemonFamilyId) {
        self.family_candy_id = ::std::option::Option::Some(v);
    }

    pub fn get_family_candy_id(&self) -> super::POGOProtos_Enums::PokemonFamilyId {
        self.family_candy_id.unwrap_or(super::POGOProtos_Enums::PokemonFamilyId::FAMILY_UNSET)
    }

    // optional int32 candy_earned_count = 3;

    pub fn clear_candy_earned_count(&mut self) {
        self.candy_earned_count = ::std::option::Option::None;
    }

    pub fn has_candy_earned_count(&self) -> bool {
        self.candy_earned_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_candy_earned_count(&mut self, v: i32) {
        self.candy_earned_count = ::std::option::Option::Some(v);
    }

    pub fn get_candy_earned_count(&self) -> i32 {
        self.candy_earned_count.unwrap_or(0)
    }
}

impl ::protobuf::Message for GetBuddyWalkedResponse {
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
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.family_candy_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.candy_earned_count = ::std::option::Option::Some(tmp);
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
        if self.success.is_some() {
            my_size += 2;
        };
        for value in &self.family_candy_id {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.candy_earned_count {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.family_candy_id {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.candy_earned_count {
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
        ::std::any::TypeId::of::<GetBuddyWalkedResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetBuddyWalkedResponse {
    fn new() -> GetBuddyWalkedResponse {
        GetBuddyWalkedResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBuddyWalkedResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    GetBuddyWalkedResponse::has_success,
                    GetBuddyWalkedResponse::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "family_candy_id",
                    GetBuddyWalkedResponse::has_family_candy_id,
                    GetBuddyWalkedResponse::get_family_candy_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "candy_earned_count",
                    GetBuddyWalkedResponse::has_candy_earned_count,
                    GetBuddyWalkedResponse::get_candy_earned_count,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBuddyWalkedResponse>(
                    "GetBuddyWalkedResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBuddyWalkedResponse {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_family_candy_id();
        self.clear_candy_earned_count();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetBuddyWalkedResponse {
    fn eq(&self, other: &GetBuddyWalkedResponse) -> bool {
        self.success == other.success &&
        self.family_candy_id == other.family_candy_id &&
        self.candy_earned_count == other.candy_earned_count &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetBuddyWalkedResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UseItemXpBoostResponse {
    // message fields
    result: ::std::option::Option<UseItemXpBoostResponse_Result>,
    applied_items: ::protobuf::SingularPtrField<super::POGOProtos_Inventory::AppliedItems>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UseItemXpBoostResponse {}

impl UseItemXpBoostResponse {
    pub fn new() -> UseItemXpBoostResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UseItemXpBoostResponse {
        static mut instance: ::protobuf::lazy::Lazy<UseItemXpBoostResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UseItemXpBoostResponse,
        };
        unsafe {
            instance.get(|| {
                UseItemXpBoostResponse {
                    result: ::std::option::Option::None,
                    applied_items: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.UseItemXpBoostResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: UseItemXpBoostResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> UseItemXpBoostResponse_Result {
        self.result.unwrap_or(UseItemXpBoostResponse_Result::UNSET)
    }

    // optional .POGOProtos.Inventory.AppliedItems applied_items = 2;

    pub fn clear_applied_items(&mut self) {
        self.applied_items.clear();
    }

    pub fn has_applied_items(&self) -> bool {
        self.applied_items.is_some()
    }

    // Param is passed by value, moved
    pub fn set_applied_items(&mut self, v: super::POGOProtos_Inventory::AppliedItems) {
        self.applied_items = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_applied_items(&mut self) -> &mut super::POGOProtos_Inventory::AppliedItems {
        if self.applied_items.is_none() {
            self.applied_items.set_default();
        };
        self.applied_items.as_mut().unwrap()
    }

    // Take field
    pub fn take_applied_items(&mut self) -> super::POGOProtos_Inventory::AppliedItems {
        self.applied_items.take().unwrap_or_else(|| super::POGOProtos_Inventory::AppliedItems::new())
    }

    pub fn get_applied_items(&self) -> &super::POGOProtos_Inventory::AppliedItems {
        self.applied_items.as_ref().unwrap_or_else(|| super::POGOProtos_Inventory::AppliedItems::default_instance())
    }
}

impl ::protobuf::Message for UseItemXpBoostResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.applied_items));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.applied_items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.applied_items.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<UseItemXpBoostResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UseItemXpBoostResponse {
    fn new() -> UseItemXpBoostResponse {
        UseItemXpBoostResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<UseItemXpBoostResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    UseItemXpBoostResponse::has_result,
                    UseItemXpBoostResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "applied_items",
                    UseItemXpBoostResponse::has_applied_items,
                    UseItemXpBoostResponse::get_applied_items,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UseItemXpBoostResponse>(
                    "UseItemXpBoostResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UseItemXpBoostResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_applied_items();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UseItemXpBoostResponse {
    fn eq(&self, other: &UseItemXpBoostResponse) -> bool {
        self.result == other.result &&
        self.applied_items == other.applied_items &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UseItemXpBoostResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum UseItemXpBoostResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    ERROR_INVALID_ITEM_TYPE = 2,
    ERROR_XP_BOOST_ALREADY_ACTIVE = 3,
    ERROR_NO_ITEMS_REMAINING = 4,
    ERROR_LOCATION_UNSET = 5,
}

impl ::protobuf::ProtobufEnum for UseItemXpBoostResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UseItemXpBoostResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(UseItemXpBoostResponse_Result::UNSET),
            1 => ::std::option::Option::Some(UseItemXpBoostResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(UseItemXpBoostResponse_Result::ERROR_INVALID_ITEM_TYPE),
            3 => ::std::option::Option::Some(UseItemXpBoostResponse_Result::ERROR_XP_BOOST_ALREADY_ACTIVE),
            4 => ::std::option::Option::Some(UseItemXpBoostResponse_Result::ERROR_NO_ITEMS_REMAINING),
            5 => ::std::option::Option::Some(UseItemXpBoostResponse_Result::ERROR_LOCATION_UNSET),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [UseItemXpBoostResponse_Result] = &[
            UseItemXpBoostResponse_Result::UNSET,
            UseItemXpBoostResponse_Result::SUCCESS,
            UseItemXpBoostResponse_Result::ERROR_INVALID_ITEM_TYPE,
            UseItemXpBoostResponse_Result::ERROR_XP_BOOST_ALREADY_ACTIVE,
            UseItemXpBoostResponse_Result::ERROR_NO_ITEMS_REMAINING,
            UseItemXpBoostResponse_Result::ERROR_LOCATION_UNSET,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<UseItemXpBoostResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("UseItemXpBoostResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for UseItemXpBoostResponse_Result {
}

#[derive(Clone,Default)]
pub struct FortDetailsResponse {
    // message fields
    fort_id: ::protobuf::SingularField<::std::string::String>,
    team_color: ::std::option::Option<super::POGOProtos_Enums::TeamColor>,
    pokemon_data: ::protobuf::SingularPtrField<super::POGOProtos_Data::PokemonData>,
    name: ::protobuf::SingularField<::std::string::String>,
    image_urls: ::protobuf::RepeatedField<::std::string::String>,
    fp: ::std::option::Option<i32>,
    stamina: ::std::option::Option<i32>,
    max_stamina: ::std::option::Option<i32>,
    field_type: ::std::option::Option<super::POGOProtos_Map_Fort::FortType>,
    latitude: ::std::option::Option<f64>,
    longitude: ::std::option::Option<f64>,
    description: ::protobuf::SingularField<::std::string::String>,
    modifiers: ::protobuf::RepeatedField<super::POGOProtos_Map_Fort::FortModifier>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FortDetailsResponse {}

impl FortDetailsResponse {
    pub fn new() -> FortDetailsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FortDetailsResponse {
        static mut instance: ::protobuf::lazy::Lazy<FortDetailsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FortDetailsResponse,
        };
        unsafe {
            instance.get(|| {
                FortDetailsResponse {
                    fort_id: ::protobuf::SingularField::none(),
                    team_color: ::std::option::Option::None,
                    pokemon_data: ::protobuf::SingularPtrField::none(),
                    name: ::protobuf::SingularField::none(),
                    image_urls: ::protobuf::RepeatedField::new(),
                    fp: ::std::option::Option::None,
                    stamina: ::std::option::Option::None,
                    max_stamina: ::std::option::Option::None,
                    field_type: ::std::option::Option::None,
                    latitude: ::std::option::Option::None,
                    longitude: ::std::option::Option::None,
                    description: ::protobuf::SingularField::none(),
                    modifiers: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string fort_id = 1;

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

    // optional .POGOProtos.Enums.TeamColor team_color = 2;

    pub fn clear_team_color(&mut self) {
        self.team_color = ::std::option::Option::None;
    }

    pub fn has_team_color(&self) -> bool {
        self.team_color.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_color(&mut self, v: super::POGOProtos_Enums::TeamColor) {
        self.team_color = ::std::option::Option::Some(v);
    }

    pub fn get_team_color(&self) -> super::POGOProtos_Enums::TeamColor {
        self.team_color.unwrap_or(super::POGOProtos_Enums::TeamColor::NEUTRAL)
    }

    // optional .POGOProtos.Data.PokemonData pokemon_data = 3;

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

    // optional string name = 4;

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

    // repeated string image_urls = 5;

    pub fn clear_image_urls(&mut self) {
        self.image_urls.clear();
    }

    // Param is passed by value, moved
    pub fn set_image_urls(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.image_urls = v;
    }

    // Mutable pointer to the field.
    pub fn mut_image_urls(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.image_urls
    }

    // Take field
    pub fn take_image_urls(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.image_urls, ::protobuf::RepeatedField::new())
    }

    pub fn get_image_urls(&self) -> &[::std::string::String] {
        &self.image_urls
    }

    // optional int32 fp = 6;

    pub fn clear_fp(&mut self) {
        self.fp = ::std::option::Option::None;
    }

    pub fn has_fp(&self) -> bool {
        self.fp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fp(&mut self, v: i32) {
        self.fp = ::std::option::Option::Some(v);
    }

    pub fn get_fp(&self) -> i32 {
        self.fp.unwrap_or(0)
    }

    // optional int32 stamina = 7;

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

    // optional int32 max_stamina = 8;

    pub fn clear_max_stamina(&mut self) {
        self.max_stamina = ::std::option::Option::None;
    }

    pub fn has_max_stamina(&self) -> bool {
        self.max_stamina.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_stamina(&mut self, v: i32) {
        self.max_stamina = ::std::option::Option::Some(v);
    }

    pub fn get_max_stamina(&self) -> i32 {
        self.max_stamina.unwrap_or(0)
    }

    // optional .POGOProtos.Map.Fort.FortType type = 9;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: super::POGOProtos_Map_Fort::FortType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> super::POGOProtos_Map_Fort::FortType {
        self.field_type.unwrap_or(super::POGOProtos_Map_Fort::FortType::GYM)
    }

    // optional double latitude = 10;

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

    // optional double longitude = 11;

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

    // optional string description = 12;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        match self.description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .POGOProtos.Map.Fort.FortModifier modifiers = 13;

    pub fn clear_modifiers(&mut self) {
        self.modifiers.clear();
    }

    // Param is passed by value, moved
    pub fn set_modifiers(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Map_Fort::FortModifier>) {
        self.modifiers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_modifiers(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Map_Fort::FortModifier> {
        &mut self.modifiers
    }

    // Take field
    pub fn take_modifiers(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Map_Fort::FortModifier> {
        ::std::mem::replace(&mut self.modifiers, ::protobuf::RepeatedField::new())
    }

    pub fn get_modifiers(&self) -> &[super::POGOProtos_Map_Fort::FortModifier] {
        &self.modifiers
    }
}

impl ::protobuf::Message for FortDetailsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fort_id));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.team_color = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pokemon_data));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.image_urls));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.fp = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.stamina = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.max_stamina = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.latitude = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.longitude = ::std::option::Option::Some(tmp);
                },
                12 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.description));
                },
                13 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.modifiers));
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
        for value in &self.fort_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.team_color {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.pokemon_data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.image_urls {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in &self.fp {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.stamina {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.max_stamina {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(9, *value);
        };
        if self.latitude.is_some() {
            my_size += 9;
        };
        if self.longitude.is_some() {
            my_size += 9;
        };
        for value in &self.description {
            my_size += ::protobuf::rt::string_size(12, &value);
        };
        for value in &self.modifiers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fort_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.team_color {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.pokemon_data.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(4, &v));
        };
        for v in &self.image_urls {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.fp {
            try!(os.write_int32(6, v));
        };
        if let Some(v) = self.stamina {
            try!(os.write_int32(7, v));
        };
        if let Some(v) = self.max_stamina {
            try!(os.write_int32(8, v));
        };
        if let Some(v) = self.field_type {
            try!(os.write_enum(9, v.value()));
        };
        if let Some(v) = self.latitude {
            try!(os.write_double(10, v));
        };
        if let Some(v) = self.longitude {
            try!(os.write_double(11, v));
        };
        if let Some(v) = self.description.as_ref() {
            try!(os.write_string(12, &v));
        };
        for v in &self.modifiers {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<FortDetailsResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FortDetailsResponse {
    fn new() -> FortDetailsResponse {
        FortDetailsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<FortDetailsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "fort_id",
                    FortDetailsResponse::has_fort_id,
                    FortDetailsResponse::get_fort_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "team_color",
                    FortDetailsResponse::has_team_color,
                    FortDetailsResponse::get_team_color,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pokemon_data",
                    FortDetailsResponse::has_pokemon_data,
                    FortDetailsResponse::get_pokemon_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    FortDetailsResponse::has_name,
                    FortDetailsResponse::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "image_urls",
                    FortDetailsResponse::get_image_urls,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "fp",
                    FortDetailsResponse::has_fp,
                    FortDetailsResponse::get_fp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "stamina",
                    FortDetailsResponse::has_stamina,
                    FortDetailsResponse::get_stamina,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "max_stamina",
                    FortDetailsResponse::has_max_stamina,
                    FortDetailsResponse::get_max_stamina,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    FortDetailsResponse::has_field_type,
                    FortDetailsResponse::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "latitude",
                    FortDetailsResponse::has_latitude,
                    FortDetailsResponse::get_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "longitude",
                    FortDetailsResponse::has_longitude,
                    FortDetailsResponse::get_longitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "description",
                    FortDetailsResponse::has_description,
                    FortDetailsResponse::get_description,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "modifiers",
                    FortDetailsResponse::get_modifiers,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FortDetailsResponse>(
                    "FortDetailsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FortDetailsResponse {
    fn clear(&mut self) {
        self.clear_fort_id();
        self.clear_team_color();
        self.clear_pokemon_data();
        self.clear_name();
        self.clear_image_urls();
        self.clear_fp();
        self.clear_stamina();
        self.clear_max_stamina();
        self.clear_field_type();
        self.clear_latitude();
        self.clear_longitude();
        self.clear_description();
        self.clear_modifiers();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FortDetailsResponse {
    fn eq(&self, other: &FortDetailsResponse) -> bool {
        self.fort_id == other.fort_id &&
        self.team_color == other.team_color &&
        self.pokemon_data == other.pokemon_data &&
        self.name == other.name &&
        self.image_urls == other.image_urls &&
        self.fp == other.fp &&
        self.stamina == other.stamina &&
        self.max_stamina == other.max_stamina &&
        self.field_type == other.field_type &&
        self.latitude == other.latitude &&
        self.longitude == other.longitude &&
        self.description == other.description &&
        self.modifiers == other.modifiers &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FortDetailsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DownloadSettingsResponse {
    // message fields
    error: ::protobuf::SingularField<::std::string::String>,
    hash: ::protobuf::SingularField<::std::string::String>,
    settings: ::protobuf::SingularPtrField<super::POGOProtos_Settings::GlobalSettings>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DownloadSettingsResponse {}

impl DownloadSettingsResponse {
    pub fn new() -> DownloadSettingsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DownloadSettingsResponse {
        static mut instance: ::protobuf::lazy::Lazy<DownloadSettingsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DownloadSettingsResponse,
        };
        unsafe {
            instance.get(|| {
                DownloadSettingsResponse {
                    error: ::protobuf::SingularField::none(),
                    hash: ::protobuf::SingularField::none(),
                    settings: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string hash = 2;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::string::String) {
        self.hash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::string::String {
        if self.hash.is_none() {
            self.hash.set_default();
        };
        self.hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::string::String {
        self.hash.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hash(&self) -> &str {
        match self.hash.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .POGOProtos.Settings.GlobalSettings settings = 3;

    pub fn clear_settings(&mut self) {
        self.settings.clear();
    }

    pub fn has_settings(&self) -> bool {
        self.settings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_settings(&mut self, v: super::POGOProtos_Settings::GlobalSettings) {
        self.settings = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_settings(&mut self) -> &mut super::POGOProtos_Settings::GlobalSettings {
        if self.settings.is_none() {
            self.settings.set_default();
        };
        self.settings.as_mut().unwrap()
    }

    // Take field
    pub fn take_settings(&mut self) -> super::POGOProtos_Settings::GlobalSettings {
        self.settings.take().unwrap_or_else(|| super::POGOProtos_Settings::GlobalSettings::new())
    }

    pub fn get_settings(&self) -> &super::POGOProtos_Settings::GlobalSettings {
        self.settings.as_ref().unwrap_or_else(|| super::POGOProtos_Settings::GlobalSettings::default_instance())
    }
}

impl ::protobuf::Message for DownloadSettingsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hash));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.settings));
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
        for value in &self.error {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.hash {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.settings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.hash.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.settings.as_ref() {
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
        ::std::any::TypeId::of::<DownloadSettingsResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DownloadSettingsResponse {
    fn new() -> DownloadSettingsResponse {
        DownloadSettingsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DownloadSettingsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "error",
                    DownloadSettingsResponse::has_error,
                    DownloadSettingsResponse::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "hash",
                    DownloadSettingsResponse::has_hash,
                    DownloadSettingsResponse::get_hash,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "settings",
                    DownloadSettingsResponse::has_settings,
                    DownloadSettingsResponse::get_settings,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DownloadSettingsResponse>(
                    "DownloadSettingsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DownloadSettingsResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_hash();
        self.clear_settings();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DownloadSettingsResponse {
    fn eq(&self, other: &DownloadSettingsResponse) -> bool {
        self.error == other.error &&
        self.hash == other.hash &&
        self.settings == other.settings &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DownloadSettingsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UseItemReviveResponse {
    // message fields
    result: ::std::option::Option<UseItemReviveResponse_Result>,
    stamina: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UseItemReviveResponse {}

impl UseItemReviveResponse {
    pub fn new() -> UseItemReviveResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UseItemReviveResponse {
        static mut instance: ::protobuf::lazy::Lazy<UseItemReviveResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UseItemReviveResponse,
        };
        unsafe {
            instance.get(|| {
                UseItemReviveResponse {
                    result: ::std::option::Option::None,
                    stamina: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.UseItemReviveResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: UseItemReviveResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> UseItemReviveResponse_Result {
        self.result.unwrap_or(UseItemReviveResponse_Result::UNSET)
    }

    // optional int32 stamina = 2;

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
}

impl ::protobuf::Message for UseItemReviveResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.stamina = ::std::option::Option::Some(tmp);
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.stamina {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.stamina {
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
        ::std::any::TypeId::of::<UseItemReviveResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UseItemReviveResponse {
    fn new() -> UseItemReviveResponse {
        UseItemReviveResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<UseItemReviveResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    UseItemReviveResponse::has_result,
                    UseItemReviveResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "stamina",
                    UseItemReviveResponse::has_stamina,
                    UseItemReviveResponse::get_stamina,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UseItemReviveResponse>(
                    "UseItemReviveResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UseItemReviveResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_stamina();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UseItemReviveResponse {
    fn eq(&self, other: &UseItemReviveResponse) -> bool {
        self.result == other.result &&
        self.stamina == other.stamina &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UseItemReviveResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum UseItemReviveResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    ERROR_NO_POKEMON = 2,
    ERROR_CANNOT_USE = 3,
    ERROR_DEPLOYED_TO_FORT = 4,
}

impl ::protobuf::ProtobufEnum for UseItemReviveResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UseItemReviveResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(UseItemReviveResponse_Result::UNSET),
            1 => ::std::option::Option::Some(UseItemReviveResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(UseItemReviveResponse_Result::ERROR_NO_POKEMON),
            3 => ::std::option::Option::Some(UseItemReviveResponse_Result::ERROR_CANNOT_USE),
            4 => ::std::option::Option::Some(UseItemReviveResponse_Result::ERROR_DEPLOYED_TO_FORT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [UseItemReviveResponse_Result] = &[
            UseItemReviveResponse_Result::UNSET,
            UseItemReviveResponse_Result::SUCCESS,
            UseItemReviveResponse_Result::ERROR_NO_POKEMON,
            UseItemReviveResponse_Result::ERROR_CANNOT_USE,
            UseItemReviveResponse_Result::ERROR_DEPLOYED_TO_FORT,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<UseItemReviveResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("UseItemReviveResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for UseItemReviveResponse_Result {
}

#[derive(Clone,Default)]
pub struct CheckChallengeResponse {
    // message fields
    show_challenge: ::std::option::Option<bool>,
    challenge_url: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CheckChallengeResponse {}

impl CheckChallengeResponse {
    pub fn new() -> CheckChallengeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckChallengeResponse {
        static mut instance: ::protobuf::lazy::Lazy<CheckChallengeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckChallengeResponse,
        };
        unsafe {
            instance.get(|| {
                CheckChallengeResponse {
                    show_challenge: ::std::option::Option::None,
                    challenge_url: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool show_challenge = 1;

    pub fn clear_show_challenge(&mut self) {
        self.show_challenge = ::std::option::Option::None;
    }

    pub fn has_show_challenge(&self) -> bool {
        self.show_challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_show_challenge(&mut self, v: bool) {
        self.show_challenge = ::std::option::Option::Some(v);
    }

    pub fn get_show_challenge(&self) -> bool {
        self.show_challenge.unwrap_or(false)
    }

    // optional string challenge_url = 2;

    pub fn clear_challenge_url(&mut self) {
        self.challenge_url.clear();
    }

    pub fn has_challenge_url(&self) -> bool {
        self.challenge_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge_url(&mut self, v: ::std::string::String) {
        self.challenge_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_challenge_url(&mut self) -> &mut ::std::string::String {
        if self.challenge_url.is_none() {
            self.challenge_url.set_default();
        };
        self.challenge_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_challenge_url(&mut self) -> ::std::string::String {
        self.challenge_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_challenge_url(&self) -> &str {
        match self.challenge_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CheckChallengeResponse {
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
                    self.show_challenge = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.challenge_url));
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
        if self.show_challenge.is_some() {
            my_size += 2;
        };
        for value in &self.challenge_url {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.show_challenge {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.challenge_url.as_ref() {
            try!(os.write_string(2, &v));
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
        ::std::any::TypeId::of::<CheckChallengeResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CheckChallengeResponse {
    fn new() -> CheckChallengeResponse {
        CheckChallengeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckChallengeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "show_challenge",
                    CheckChallengeResponse::has_show_challenge,
                    CheckChallengeResponse::get_show_challenge,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "challenge_url",
                    CheckChallengeResponse::has_challenge_url,
                    CheckChallengeResponse::get_challenge_url,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CheckChallengeResponse>(
                    "CheckChallengeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckChallengeResponse {
    fn clear(&mut self) {
        self.clear_show_challenge();
        self.clear_challenge_url();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CheckChallengeResponse {
    fn eq(&self, other: &CheckChallengeResponse) -> bool {
        self.show_challenge == other.show_challenge &&
        self.challenge_url == other.challenge_url &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CheckChallengeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct MarkTutorialCompleteResponse {
    // message fields
    success: ::std::option::Option<bool>,
    player_data: ::protobuf::SingularPtrField<super::POGOProtos_Data::PlayerData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MarkTutorialCompleteResponse {}

impl MarkTutorialCompleteResponse {
    pub fn new() -> MarkTutorialCompleteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MarkTutorialCompleteResponse {
        static mut instance: ::protobuf::lazy::Lazy<MarkTutorialCompleteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MarkTutorialCompleteResponse,
        };
        unsafe {
            instance.get(|| {
                MarkTutorialCompleteResponse {
                    success: ::std::option::Option::None,
                    player_data: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // optional .POGOProtos.Data.PlayerData player_data = 2;

    pub fn clear_player_data(&mut self) {
        self.player_data.clear();
    }

    pub fn has_player_data(&self) -> bool {
        self.player_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_data(&mut self, v: super::POGOProtos_Data::PlayerData) {
        self.player_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_data(&mut self) -> &mut super::POGOProtos_Data::PlayerData {
        if self.player_data.is_none() {
            self.player_data.set_default();
        };
        self.player_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_data(&mut self) -> super::POGOProtos_Data::PlayerData {
        self.player_data.take().unwrap_or_else(|| super::POGOProtos_Data::PlayerData::new())
    }

    pub fn get_player_data(&self) -> &super::POGOProtos_Data::PlayerData {
        self.player_data.as_ref().unwrap_or_else(|| super::POGOProtos_Data::PlayerData::default_instance())
    }
}

impl ::protobuf::Message for MarkTutorialCompleteResponse {
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
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_data));
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
        if self.success.is_some() {
            my_size += 2;
        };
        for value in &self.player_data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.player_data.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<MarkTutorialCompleteResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MarkTutorialCompleteResponse {
    fn new() -> MarkTutorialCompleteResponse {
        MarkTutorialCompleteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<MarkTutorialCompleteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    MarkTutorialCompleteResponse::has_success,
                    MarkTutorialCompleteResponse::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "player_data",
                    MarkTutorialCompleteResponse::has_player_data,
                    MarkTutorialCompleteResponse::get_player_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MarkTutorialCompleteResponse>(
                    "MarkTutorialCompleteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MarkTutorialCompleteResponse {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_player_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for MarkTutorialCompleteResponse {
    fn eq(&self, other: &MarkTutorialCompleteResponse) -> bool {
        self.success == other.success &&
        self.player_data == other.player_data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for MarkTutorialCompleteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PlayerUpdateResponse {
    // message fields
    wild_pokemons: ::protobuf::RepeatedField<super::POGOProtos_Map_Pokemon::WildPokemon>,
    forts: ::protobuf::RepeatedField<super::POGOProtos_Map_Fort::FortData>,
    forts_nearby: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayerUpdateResponse {}

impl PlayerUpdateResponse {
    pub fn new() -> PlayerUpdateResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayerUpdateResponse {
        static mut instance: ::protobuf::lazy::Lazy<PlayerUpdateResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayerUpdateResponse,
        };
        unsafe {
            instance.get(|| {
                PlayerUpdateResponse {
                    wild_pokemons: ::protobuf::RepeatedField::new(),
                    forts: ::protobuf::RepeatedField::new(),
                    forts_nearby: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .POGOProtos.Map.Pokemon.WildPokemon wild_pokemons = 1;

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

    // repeated .POGOProtos.Map.Fort.FortData forts = 2;

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

    // optional int32 forts_nearby = 3;

    pub fn clear_forts_nearby(&mut self) {
        self.forts_nearby = ::std::option::Option::None;
    }

    pub fn has_forts_nearby(&self) -> bool {
        self.forts_nearby.is_some()
    }

    // Param is passed by value, moved
    pub fn set_forts_nearby(&mut self, v: i32) {
        self.forts_nearby = ::std::option::Option::Some(v);
    }

    pub fn get_forts_nearby(&self) -> i32 {
        self.forts_nearby.unwrap_or(0)
    }
}

impl ::protobuf::Message for PlayerUpdateResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.wild_pokemons));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.forts));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.forts_nearby = ::std::option::Option::Some(tmp);
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
        for value in &self.wild_pokemons {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.forts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.forts_nearby {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.wild_pokemons {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.forts {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.forts_nearby {
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
        ::std::any::TypeId::of::<PlayerUpdateResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PlayerUpdateResponse {
    fn new() -> PlayerUpdateResponse {
        PlayerUpdateResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayerUpdateResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "wild_pokemons",
                    PlayerUpdateResponse::get_wild_pokemons,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "forts",
                    PlayerUpdateResponse::get_forts,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "forts_nearby",
                    PlayerUpdateResponse::has_forts_nearby,
                    PlayerUpdateResponse::get_forts_nearby,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlayerUpdateResponse>(
                    "PlayerUpdateResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayerUpdateResponse {
    fn clear(&mut self) {
        self.clear_wild_pokemons();
        self.clear_forts();
        self.clear_forts_nearby();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PlayerUpdateResponse {
    fn eq(&self, other: &PlayerUpdateResponse) -> bool {
        self.wild_pokemons == other.wild_pokemons &&
        self.forts == other.forts &&
        self.forts_nearby == other.forts_nearby &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PlayerUpdateResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CheckAwardedBadgesResponse {
    // message fields
    success: ::std::option::Option<bool>,
    awarded_badges: ::std::vec::Vec<super::POGOProtos_Enums::BadgeType>,
    awarded_badge_levels: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CheckAwardedBadgesResponse {}

impl CheckAwardedBadgesResponse {
    pub fn new() -> CheckAwardedBadgesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckAwardedBadgesResponse {
        static mut instance: ::protobuf::lazy::Lazy<CheckAwardedBadgesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckAwardedBadgesResponse,
        };
        unsafe {
            instance.get(|| {
                CheckAwardedBadgesResponse {
                    success: ::std::option::Option::None,
                    awarded_badges: ::std::vec::Vec::new(),
                    awarded_badge_levels: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // repeated .POGOProtos.Enums.BadgeType awarded_badges = 2;

    pub fn clear_awarded_badges(&mut self) {
        self.awarded_badges.clear();
    }

    // Param is passed by value, moved
    pub fn set_awarded_badges(&mut self, v: ::std::vec::Vec<super::POGOProtos_Enums::BadgeType>) {
        self.awarded_badges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_awarded_badges(&mut self) -> &mut ::std::vec::Vec<super::POGOProtos_Enums::BadgeType> {
        &mut self.awarded_badges
    }

    // Take field
    pub fn take_awarded_badges(&mut self) -> ::std::vec::Vec<super::POGOProtos_Enums::BadgeType> {
        ::std::mem::replace(&mut self.awarded_badges, ::std::vec::Vec::new())
    }

    pub fn get_awarded_badges(&self) -> &[super::POGOProtos_Enums::BadgeType] {
        &self.awarded_badges
    }

    // repeated int32 awarded_badge_levels = 3;

    pub fn clear_awarded_badge_levels(&mut self) {
        self.awarded_badge_levels.clear();
    }

    // Param is passed by value, moved
    pub fn set_awarded_badge_levels(&mut self, v: ::std::vec::Vec<i32>) {
        self.awarded_badge_levels = v;
    }

    // Mutable pointer to the field.
    pub fn mut_awarded_badge_levels(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.awarded_badge_levels
    }

    // Take field
    pub fn take_awarded_badge_levels(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.awarded_badge_levels, ::std::vec::Vec::new())
    }

    pub fn get_awarded_badge_levels(&self) -> &[i32] {
        &self.awarded_badge_levels
    }
}

impl ::protobuf::Message for CheckAwardedBadgesResponse {
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
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.awarded_badges));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.awarded_badge_levels));
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
        if self.success.is_some() {
            my_size += 2;
        };
        for value in &self.awarded_badges {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.awarded_badge_levels {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            try!(os.write_bool(1, v));
        };
        for v in &self.awarded_badges {
            try!(os.write_enum(2, v.value()));
        };
        for v in &self.awarded_badge_levels {
            try!(os.write_int32(3, *v));
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
        ::std::any::TypeId::of::<CheckAwardedBadgesResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CheckAwardedBadgesResponse {
    fn new() -> CheckAwardedBadgesResponse {
        CheckAwardedBadgesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckAwardedBadgesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    CheckAwardedBadgesResponse::has_success,
                    CheckAwardedBadgesResponse::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "awarded_badges",
                    CheckAwardedBadgesResponse::get_awarded_badges,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "awarded_badge_levels",
                    CheckAwardedBadgesResponse::get_awarded_badge_levels,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CheckAwardedBadgesResponse>(
                    "CheckAwardedBadgesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckAwardedBadgesResponse {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_awarded_badges();
        self.clear_awarded_badge_levels();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CheckAwardedBadgesResponse {
    fn eq(&self, other: &CheckAwardedBadgesResponse) -> bool {
        self.success == other.success &&
        self.awarded_badges == other.awarded_badges &&
        self.awarded_badge_levels == other.awarded_badge_levels &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CheckAwardedBadgesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DownloadRemoteConfigVersionResponse {
    // message fields
    result: ::std::option::Option<DownloadRemoteConfigVersionResponse_Result>,
    item_templates_timestamp_ms: ::std::option::Option<u64>,
    asset_digest_timestamp_ms: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DownloadRemoteConfigVersionResponse {}

impl DownloadRemoteConfigVersionResponse {
    pub fn new() -> DownloadRemoteConfigVersionResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DownloadRemoteConfigVersionResponse {
        static mut instance: ::protobuf::lazy::Lazy<DownloadRemoteConfigVersionResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DownloadRemoteConfigVersionResponse,
        };
        unsafe {
            instance.get(|| {
                DownloadRemoteConfigVersionResponse {
                    result: ::std::option::Option::None,
                    item_templates_timestamp_ms: ::std::option::Option::None,
                    asset_digest_timestamp_ms: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.DownloadRemoteConfigVersionResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: DownloadRemoteConfigVersionResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> DownloadRemoteConfigVersionResponse_Result {
        self.result.unwrap_or(DownloadRemoteConfigVersionResponse_Result::UNSET)
    }

    // optional uint64 item_templates_timestamp_ms = 2;

    pub fn clear_item_templates_timestamp_ms(&mut self) {
        self.item_templates_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_item_templates_timestamp_ms(&self) -> bool {
        self.item_templates_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_templates_timestamp_ms(&mut self, v: u64) {
        self.item_templates_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_item_templates_timestamp_ms(&self) -> u64 {
        self.item_templates_timestamp_ms.unwrap_or(0)
    }

    // optional uint64 asset_digest_timestamp_ms = 3;

    pub fn clear_asset_digest_timestamp_ms(&mut self) {
        self.asset_digest_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_asset_digest_timestamp_ms(&self) -> bool {
        self.asset_digest_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_asset_digest_timestamp_ms(&mut self, v: u64) {
        self.asset_digest_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_asset_digest_timestamp_ms(&self) -> u64 {
        self.asset_digest_timestamp_ms.unwrap_or(0)
    }
}

impl ::protobuf::Message for DownloadRemoteConfigVersionResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.item_templates_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.asset_digest_timestamp_ms = ::std::option::Option::Some(tmp);
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.item_templates_timestamp_ms {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.asset_digest_timestamp_ms {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.item_templates_timestamp_ms {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.asset_digest_timestamp_ms {
            try!(os.write_uint64(3, v));
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
        ::std::any::TypeId::of::<DownloadRemoteConfigVersionResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DownloadRemoteConfigVersionResponse {
    fn new() -> DownloadRemoteConfigVersionResponse {
        DownloadRemoteConfigVersionResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DownloadRemoteConfigVersionResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    DownloadRemoteConfigVersionResponse::has_result,
                    DownloadRemoteConfigVersionResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "item_templates_timestamp_ms",
                    DownloadRemoteConfigVersionResponse::has_item_templates_timestamp_ms,
                    DownloadRemoteConfigVersionResponse::get_item_templates_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "asset_digest_timestamp_ms",
                    DownloadRemoteConfigVersionResponse::has_asset_digest_timestamp_ms,
                    DownloadRemoteConfigVersionResponse::get_asset_digest_timestamp_ms,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DownloadRemoteConfigVersionResponse>(
                    "DownloadRemoteConfigVersionResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DownloadRemoteConfigVersionResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_item_templates_timestamp_ms();
        self.clear_asset_digest_timestamp_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DownloadRemoteConfigVersionResponse {
    fn eq(&self, other: &DownloadRemoteConfigVersionResponse) -> bool {
        self.result == other.result &&
        self.item_templates_timestamp_ms == other.item_templates_timestamp_ms &&
        self.asset_digest_timestamp_ms == other.asset_digest_timestamp_ms &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DownloadRemoteConfigVersionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DownloadRemoteConfigVersionResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
}

impl ::protobuf::ProtobufEnum for DownloadRemoteConfigVersionResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DownloadRemoteConfigVersionResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(DownloadRemoteConfigVersionResponse_Result::UNSET),
            1 => ::std::option::Option::Some(DownloadRemoteConfigVersionResponse_Result::SUCCESS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DownloadRemoteConfigVersionResponse_Result] = &[
            DownloadRemoteConfigVersionResponse_Result::UNSET,
            DownloadRemoteConfigVersionResponse_Result::SUCCESS,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<DownloadRemoteConfigVersionResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DownloadRemoteConfigVersionResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DownloadRemoteConfigVersionResponse_Result {
}

#[derive(Clone,Default)]
pub struct CollectDailyBonusResponse {
    // message fields
    result: ::std::option::Option<CollectDailyBonusResponse_Result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CollectDailyBonusResponse {}

impl CollectDailyBonusResponse {
    pub fn new() -> CollectDailyBonusResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CollectDailyBonusResponse {
        static mut instance: ::protobuf::lazy::Lazy<CollectDailyBonusResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CollectDailyBonusResponse,
        };
        unsafe {
            instance.get(|| {
                CollectDailyBonusResponse {
                    result: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.CollectDailyBonusResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CollectDailyBonusResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CollectDailyBonusResponse_Result {
        self.result.unwrap_or(CollectDailyBonusResponse_Result::UNSET)
    }
}

impl ::protobuf::Message for CollectDailyBonusResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
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
        ::std::any::TypeId::of::<CollectDailyBonusResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CollectDailyBonusResponse {
    fn new() -> CollectDailyBonusResponse {
        CollectDailyBonusResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CollectDailyBonusResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    CollectDailyBonusResponse::has_result,
                    CollectDailyBonusResponse::get_result,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CollectDailyBonusResponse>(
                    "CollectDailyBonusResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CollectDailyBonusResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CollectDailyBonusResponse {
    fn eq(&self, other: &CollectDailyBonusResponse) -> bool {
        self.result == other.result &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CollectDailyBonusResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CollectDailyBonusResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    FAILURE = 2,
    TOO_SOON = 3,
}

impl ::protobuf::ProtobufEnum for CollectDailyBonusResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CollectDailyBonusResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(CollectDailyBonusResponse_Result::UNSET),
            1 => ::std::option::Option::Some(CollectDailyBonusResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(CollectDailyBonusResponse_Result::FAILURE),
            3 => ::std::option::Option::Some(CollectDailyBonusResponse_Result::TOO_SOON),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CollectDailyBonusResponse_Result] = &[
            CollectDailyBonusResponse_Result::UNSET,
            CollectDailyBonusResponse_Result::SUCCESS,
            CollectDailyBonusResponse_Result::FAILURE,
            CollectDailyBonusResponse_Result::TOO_SOON,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CollectDailyBonusResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CollectDailyBonusResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CollectDailyBonusResponse_Result {
}

#[derive(Clone,Default)]
pub struct GetDownloadUrlsResponse {
    // message fields
    download_urls: ::protobuf::RepeatedField<super::POGOProtos_Data::DownloadUrlEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetDownloadUrlsResponse {}

impl GetDownloadUrlsResponse {
    pub fn new() -> GetDownloadUrlsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetDownloadUrlsResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetDownloadUrlsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetDownloadUrlsResponse,
        };
        unsafe {
            instance.get(|| {
                GetDownloadUrlsResponse {
                    download_urls: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .POGOProtos.Data.DownloadUrlEntry download_urls = 1;

    pub fn clear_download_urls(&mut self) {
        self.download_urls.clear();
    }

    // Param is passed by value, moved
    pub fn set_download_urls(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Data::DownloadUrlEntry>) {
        self.download_urls = v;
    }

    // Mutable pointer to the field.
    pub fn mut_download_urls(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Data::DownloadUrlEntry> {
        &mut self.download_urls
    }

    // Take field
    pub fn take_download_urls(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Data::DownloadUrlEntry> {
        ::std::mem::replace(&mut self.download_urls, ::protobuf::RepeatedField::new())
    }

    pub fn get_download_urls(&self) -> &[super::POGOProtos_Data::DownloadUrlEntry] {
        &self.download_urls
    }
}

impl ::protobuf::Message for GetDownloadUrlsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.download_urls));
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
        for value in &self.download_urls {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.download_urls {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<GetDownloadUrlsResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetDownloadUrlsResponse {
    fn new() -> GetDownloadUrlsResponse {
        GetDownloadUrlsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetDownloadUrlsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "download_urls",
                    GetDownloadUrlsResponse::get_download_urls,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetDownloadUrlsResponse>(
                    "GetDownloadUrlsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetDownloadUrlsResponse {
    fn clear(&mut self) {
        self.clear_download_urls();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetDownloadUrlsResponse {
    fn eq(&self, other: &GetDownloadUrlsResponse) -> bool {
        self.download_urls == other.download_urls &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetDownloadUrlsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetGymDetailsResponse {
    // message fields
    gym_state: ::protobuf::SingularPtrField<super::POGOProtos_Data_Gym::GymState>,
    name: ::protobuf::SingularField<::std::string::String>,
    urls: ::protobuf::RepeatedField<::std::string::String>,
    result: ::std::option::Option<GetGymDetailsResponse_Result>,
    description: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetGymDetailsResponse {}

impl GetGymDetailsResponse {
    pub fn new() -> GetGymDetailsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetGymDetailsResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetGymDetailsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetGymDetailsResponse,
        };
        unsafe {
            instance.get(|| {
                GetGymDetailsResponse {
                    gym_state: ::protobuf::SingularPtrField::none(),
                    name: ::protobuf::SingularField::none(),
                    urls: ::protobuf::RepeatedField::new(),
                    result: ::std::option::Option::None,
                    description: ::protobuf::SingularField::none(),
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

    // optional string name = 2;

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

    // repeated string urls = 3;

    pub fn clear_urls(&mut self) {
        self.urls.clear();
    }

    // Param is passed by value, moved
    pub fn set_urls(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.urls = v;
    }

    // Mutable pointer to the field.
    pub fn mut_urls(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.urls
    }

    // Take field
    pub fn take_urls(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.urls, ::protobuf::RepeatedField::new())
    }

    pub fn get_urls(&self) -> &[::std::string::String] {
        &self.urls
    }

    // optional .POGOProtos.Networking.Responses.GetGymDetailsResponse.Result result = 4;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: GetGymDetailsResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> GetGymDetailsResponse_Result {
        self.result.unwrap_or(GetGymDetailsResponse_Result::UNSET)
    }

    // optional string description = 5;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        match self.description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for GetGymDetailsResponse {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.urls));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.result = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.description));
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
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.urls {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        for value in &self.description {
            my_size += ::protobuf::rt::string_size(5, &value);
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
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
        };
        for v in &self.urls {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.result {
            try!(os.write_enum(4, v.value()));
        };
        if let Some(v) = self.description.as_ref() {
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
        ::std::any::TypeId::of::<GetGymDetailsResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetGymDetailsResponse {
    fn new() -> GetGymDetailsResponse {
        GetGymDetailsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetGymDetailsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "gym_state",
                    GetGymDetailsResponse::has_gym_state,
                    GetGymDetailsResponse::get_gym_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    GetGymDetailsResponse::has_name,
                    GetGymDetailsResponse::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "urls",
                    GetGymDetailsResponse::get_urls,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    GetGymDetailsResponse::has_result,
                    GetGymDetailsResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "description",
                    GetGymDetailsResponse::has_description,
                    GetGymDetailsResponse::get_description,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetGymDetailsResponse>(
                    "GetGymDetailsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetGymDetailsResponse {
    fn clear(&mut self) {
        self.clear_gym_state();
        self.clear_name();
        self.clear_urls();
        self.clear_result();
        self.clear_description();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetGymDetailsResponse {
    fn eq(&self, other: &GetGymDetailsResponse) -> bool {
        self.gym_state == other.gym_state &&
        self.name == other.name &&
        self.urls == other.urls &&
        self.result == other.result &&
        self.description == other.description &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetGymDetailsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum GetGymDetailsResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    ERROR_NOT_IN_RANGE = 2,
}

impl ::protobuf::ProtobufEnum for GetGymDetailsResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<GetGymDetailsResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(GetGymDetailsResponse_Result::UNSET),
            1 => ::std::option::Option::Some(GetGymDetailsResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(GetGymDetailsResponse_Result::ERROR_NOT_IN_RANGE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [GetGymDetailsResponse_Result] = &[
            GetGymDetailsResponse_Result::UNSET,
            GetGymDetailsResponse_Result::SUCCESS,
            GetGymDetailsResponse_Result::ERROR_NOT_IN_RANGE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<GetGymDetailsResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("GetGymDetailsResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for GetGymDetailsResponse_Result {
}

#[derive(Clone,Default)]
pub struct GetSuggestedCodenamesResponse {
    // message fields
    codenames: ::protobuf::RepeatedField<::std::string::String>,
    success: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetSuggestedCodenamesResponse {}

impl GetSuggestedCodenamesResponse {
    pub fn new() -> GetSuggestedCodenamesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetSuggestedCodenamesResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetSuggestedCodenamesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetSuggestedCodenamesResponse,
        };
        unsafe {
            instance.get(|| {
                GetSuggestedCodenamesResponse {
                    codenames: ::protobuf::RepeatedField::new(),
                    success: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string codenames = 1;

    pub fn clear_codenames(&mut self) {
        self.codenames.clear();
    }

    // Param is passed by value, moved
    pub fn set_codenames(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.codenames = v;
    }

    // Mutable pointer to the field.
    pub fn mut_codenames(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.codenames
    }

    // Take field
    pub fn take_codenames(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.codenames, ::protobuf::RepeatedField::new())
    }

    pub fn get_codenames(&self) -> &[::std::string::String] {
        &self.codenames
    }

    // optional bool success = 2;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }
}

impl ::protobuf::Message for GetSuggestedCodenamesResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.codenames));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.success = ::std::option::Option::Some(tmp);
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
        for value in &self.codenames {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.success.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.codenames {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.success {
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
        ::std::any::TypeId::of::<GetSuggestedCodenamesResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetSuggestedCodenamesResponse {
    fn new() -> GetSuggestedCodenamesResponse {
        GetSuggestedCodenamesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetSuggestedCodenamesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "codenames",
                    GetSuggestedCodenamesResponse::get_codenames,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    GetSuggestedCodenamesResponse::has_success,
                    GetSuggestedCodenamesResponse::get_success,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetSuggestedCodenamesResponse>(
                    "GetSuggestedCodenamesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetSuggestedCodenamesResponse {
    fn clear(&mut self) {
        self.clear_codenames();
        self.clear_success();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetSuggestedCodenamesResponse {
    fn eq(&self, other: &GetSuggestedCodenamesResponse) -> bool {
        self.codenames == other.codenames &&
        self.success == other.success &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetSuggestedCodenamesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FortDeployPokemonResponse {
    // message fields
    result: ::std::option::Option<FortDeployPokemonResponse_Result>,
    fort_details: ::protobuf::SingularPtrField<FortDetailsResponse>,
    pokemon_data: ::protobuf::SingularPtrField<super::POGOProtos_Data::PokemonData>,
    gym_state: ::protobuf::SingularPtrField<super::POGOProtos_Data_Gym::GymState>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FortDeployPokemonResponse {}

impl FortDeployPokemonResponse {
    pub fn new() -> FortDeployPokemonResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FortDeployPokemonResponse {
        static mut instance: ::protobuf::lazy::Lazy<FortDeployPokemonResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FortDeployPokemonResponse,
        };
        unsafe {
            instance.get(|| {
                FortDeployPokemonResponse {
                    result: ::std::option::Option::None,
                    fort_details: ::protobuf::SingularPtrField::none(),
                    pokemon_data: ::protobuf::SingularPtrField::none(),
                    gym_state: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.FortDeployPokemonResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: FortDeployPokemonResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> FortDeployPokemonResponse_Result {
        self.result.unwrap_or(FortDeployPokemonResponse_Result::NO_RESULT_SET)
    }

    // optional .POGOProtos.Networking.Responses.FortDetailsResponse fort_details = 2;

    pub fn clear_fort_details(&mut self) {
        self.fort_details.clear();
    }

    pub fn has_fort_details(&self) -> bool {
        self.fort_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fort_details(&mut self, v: FortDetailsResponse) {
        self.fort_details = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fort_details(&mut self) -> &mut FortDetailsResponse {
        if self.fort_details.is_none() {
            self.fort_details.set_default();
        };
        self.fort_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_fort_details(&mut self) -> FortDetailsResponse {
        self.fort_details.take().unwrap_or_else(|| FortDetailsResponse::new())
    }

    pub fn get_fort_details(&self) -> &FortDetailsResponse {
        self.fort_details.as_ref().unwrap_or_else(|| FortDetailsResponse::default_instance())
    }

    // optional .POGOProtos.Data.PokemonData pokemon_data = 3;

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

    // optional .POGOProtos.Data.Gym.GymState gym_state = 4;

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
}

impl ::protobuf::Message for FortDeployPokemonResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fort_details));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pokemon_data));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.gym_state));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.fort_details {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.pokemon_data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.gym_state {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.fort_details.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pokemon_data.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.gym_state.as_ref() {
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
        ::std::any::TypeId::of::<FortDeployPokemonResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FortDeployPokemonResponse {
    fn new() -> FortDeployPokemonResponse {
        FortDeployPokemonResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<FortDeployPokemonResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    FortDeployPokemonResponse::has_result,
                    FortDeployPokemonResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fort_details",
                    FortDeployPokemonResponse::has_fort_details,
                    FortDeployPokemonResponse::get_fort_details,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pokemon_data",
                    FortDeployPokemonResponse::has_pokemon_data,
                    FortDeployPokemonResponse::get_pokemon_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "gym_state",
                    FortDeployPokemonResponse::has_gym_state,
                    FortDeployPokemonResponse::get_gym_state,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FortDeployPokemonResponse>(
                    "FortDeployPokemonResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FortDeployPokemonResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_fort_details();
        self.clear_pokemon_data();
        self.clear_gym_state();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FortDeployPokemonResponse {
    fn eq(&self, other: &FortDeployPokemonResponse) -> bool {
        self.result == other.result &&
        self.fort_details == other.fort_details &&
        self.pokemon_data == other.pokemon_data &&
        self.gym_state == other.gym_state &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FortDeployPokemonResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FortDeployPokemonResponse_Result {
    NO_RESULT_SET = 0,
    SUCCESS = 1,
    ERROR_ALREADY_HAS_POKEMON_ON_FORT = 2,
    ERROR_OPPOSING_TEAM_OWNS_FORT = 3,
    ERROR_FORT_IS_FULL = 4,
    ERROR_NOT_IN_RANGE = 5,
    ERROR_PLAYER_HAS_NO_TEAM = 6,
    ERROR_POKEMON_NOT_FULL_HP = 7,
    ERROR_PLAYER_BELOW_MINIMUM_LEVEL = 8,
    ERROR_POKEMON_IS_BUDDY = 9,
}

impl ::protobuf::ProtobufEnum for FortDeployPokemonResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FortDeployPokemonResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(FortDeployPokemonResponse_Result::NO_RESULT_SET),
            1 => ::std::option::Option::Some(FortDeployPokemonResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(FortDeployPokemonResponse_Result::ERROR_ALREADY_HAS_POKEMON_ON_FORT),
            3 => ::std::option::Option::Some(FortDeployPokemonResponse_Result::ERROR_OPPOSING_TEAM_OWNS_FORT),
            4 => ::std::option::Option::Some(FortDeployPokemonResponse_Result::ERROR_FORT_IS_FULL),
            5 => ::std::option::Option::Some(FortDeployPokemonResponse_Result::ERROR_NOT_IN_RANGE),
            6 => ::std::option::Option::Some(FortDeployPokemonResponse_Result::ERROR_PLAYER_HAS_NO_TEAM),
            7 => ::std::option::Option::Some(FortDeployPokemonResponse_Result::ERROR_POKEMON_NOT_FULL_HP),
            8 => ::std::option::Option::Some(FortDeployPokemonResponse_Result::ERROR_PLAYER_BELOW_MINIMUM_LEVEL),
            9 => ::std::option::Option::Some(FortDeployPokemonResponse_Result::ERROR_POKEMON_IS_BUDDY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FortDeployPokemonResponse_Result] = &[
            FortDeployPokemonResponse_Result::NO_RESULT_SET,
            FortDeployPokemonResponse_Result::SUCCESS,
            FortDeployPokemonResponse_Result::ERROR_ALREADY_HAS_POKEMON_ON_FORT,
            FortDeployPokemonResponse_Result::ERROR_OPPOSING_TEAM_OWNS_FORT,
            FortDeployPokemonResponse_Result::ERROR_FORT_IS_FULL,
            FortDeployPokemonResponse_Result::ERROR_NOT_IN_RANGE,
            FortDeployPokemonResponse_Result::ERROR_PLAYER_HAS_NO_TEAM,
            FortDeployPokemonResponse_Result::ERROR_POKEMON_NOT_FULL_HP,
            FortDeployPokemonResponse_Result::ERROR_PLAYER_BELOW_MINIMUM_LEVEL,
            FortDeployPokemonResponse_Result::ERROR_POKEMON_IS_BUDDY,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<FortDeployPokemonResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("FortDeployPokemonResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for FortDeployPokemonResponse_Result {
}

#[derive(Clone,Default)]
pub struct IncenseEncounterResponse {
    // message fields
    result: ::std::option::Option<IncenseEncounterResponse_Result>,
    pokemon_data: ::protobuf::SingularPtrField<super::POGOProtos_Data::PokemonData>,
    capture_probability: ::protobuf::SingularPtrField<super::POGOProtos_Data_Capture::CaptureProbability>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IncenseEncounterResponse {}

impl IncenseEncounterResponse {
    pub fn new() -> IncenseEncounterResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IncenseEncounterResponse {
        static mut instance: ::protobuf::lazy::Lazy<IncenseEncounterResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IncenseEncounterResponse,
        };
        unsafe {
            instance.get(|| {
                IncenseEncounterResponse {
                    result: ::std::option::Option::None,
                    pokemon_data: ::protobuf::SingularPtrField::none(),
                    capture_probability: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.IncenseEncounterResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: IncenseEncounterResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> IncenseEncounterResponse_Result {
        self.result.unwrap_or(IncenseEncounterResponse_Result::INCENSE_ENCOUNTER_UNKNOWN)
    }

    // optional .POGOProtos.Data.PokemonData pokemon_data = 2;

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

    // optional .POGOProtos.Data.Capture.CaptureProbability capture_probability = 3;

    pub fn clear_capture_probability(&mut self) {
        self.capture_probability.clear();
    }

    pub fn has_capture_probability(&self) -> bool {
        self.capture_probability.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capture_probability(&mut self, v: super::POGOProtos_Data_Capture::CaptureProbability) {
        self.capture_probability = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_capture_probability(&mut self) -> &mut super::POGOProtos_Data_Capture::CaptureProbability {
        if self.capture_probability.is_none() {
            self.capture_probability.set_default();
        };
        self.capture_probability.as_mut().unwrap()
    }

    // Take field
    pub fn take_capture_probability(&mut self) -> super::POGOProtos_Data_Capture::CaptureProbability {
        self.capture_probability.take().unwrap_or_else(|| super::POGOProtos_Data_Capture::CaptureProbability::new())
    }

    pub fn get_capture_probability(&self) -> &super::POGOProtos_Data_Capture::CaptureProbability {
        self.capture_probability.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Capture::CaptureProbability::default_instance())
    }
}

impl ::protobuf::Message for IncenseEncounterResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pokemon_data));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.capture_probability));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.pokemon_data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.capture_probability {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.pokemon_data.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.capture_probability.as_ref() {
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
        ::std::any::TypeId::of::<IncenseEncounterResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IncenseEncounterResponse {
    fn new() -> IncenseEncounterResponse {
        IncenseEncounterResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<IncenseEncounterResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    IncenseEncounterResponse::has_result,
                    IncenseEncounterResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pokemon_data",
                    IncenseEncounterResponse::has_pokemon_data,
                    IncenseEncounterResponse::get_pokemon_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "capture_probability",
                    IncenseEncounterResponse::has_capture_probability,
                    IncenseEncounterResponse::get_capture_probability,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IncenseEncounterResponse>(
                    "IncenseEncounterResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IncenseEncounterResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_pokemon_data();
        self.clear_capture_probability();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IncenseEncounterResponse {
    fn eq(&self, other: &IncenseEncounterResponse) -> bool {
        self.result == other.result &&
        self.pokemon_data == other.pokemon_data &&
        self.capture_probability == other.capture_probability &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IncenseEncounterResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum IncenseEncounterResponse_Result {
    INCENSE_ENCOUNTER_UNKNOWN = 0,
    INCENSE_ENCOUNTER_SUCCESS = 1,
    INCENSE_ENCOUNTER_NOT_AVAILABLE = 2,
    POKEMON_INVENTORY_FULL = 3,
}

impl ::protobuf::ProtobufEnum for IncenseEncounterResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<IncenseEncounterResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(IncenseEncounterResponse_Result::INCENSE_ENCOUNTER_UNKNOWN),
            1 => ::std::option::Option::Some(IncenseEncounterResponse_Result::INCENSE_ENCOUNTER_SUCCESS),
            2 => ::std::option::Option::Some(IncenseEncounterResponse_Result::INCENSE_ENCOUNTER_NOT_AVAILABLE),
            3 => ::std::option::Option::Some(IncenseEncounterResponse_Result::POKEMON_INVENTORY_FULL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [IncenseEncounterResponse_Result] = &[
            IncenseEncounterResponse_Result::INCENSE_ENCOUNTER_UNKNOWN,
            IncenseEncounterResponse_Result::INCENSE_ENCOUNTER_SUCCESS,
            IncenseEncounterResponse_Result::INCENSE_ENCOUNTER_NOT_AVAILABLE,
            IncenseEncounterResponse_Result::POKEMON_INVENTORY_FULL,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<IncenseEncounterResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("IncenseEncounterResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for IncenseEncounterResponse_Result {
}

#[derive(Clone,Default)]
pub struct VerifyChallengeResponse {
    // message fields
    success: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerifyChallengeResponse {}

impl VerifyChallengeResponse {
    pub fn new() -> VerifyChallengeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerifyChallengeResponse {
        static mut instance: ::protobuf::lazy::Lazy<VerifyChallengeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerifyChallengeResponse,
        };
        unsafe {
            instance.get(|| {
                VerifyChallengeResponse {
                    success: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }
}

impl ::protobuf::Message for VerifyChallengeResponse {
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
                    self.success = ::std::option::Option::Some(tmp);
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
        if self.success.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
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
        ::std::any::TypeId::of::<VerifyChallengeResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VerifyChallengeResponse {
    fn new() -> VerifyChallengeResponse {
        VerifyChallengeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerifyChallengeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    VerifyChallengeResponse::has_success,
                    VerifyChallengeResponse::get_success,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VerifyChallengeResponse>(
                    "VerifyChallengeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerifyChallengeResponse {
    fn clear(&mut self) {
        self.clear_success();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VerifyChallengeResponse {
    fn eq(&self, other: &VerifyChallengeResponse) -> bool {
        self.success == other.success &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VerifyChallengeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UseItemPotionResponse {
    // message fields
    result: ::std::option::Option<UseItemPotionResponse_Result>,
    stamina: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UseItemPotionResponse {}

impl UseItemPotionResponse {
    pub fn new() -> UseItemPotionResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UseItemPotionResponse {
        static mut instance: ::protobuf::lazy::Lazy<UseItemPotionResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UseItemPotionResponse,
        };
        unsafe {
            instance.get(|| {
                UseItemPotionResponse {
                    result: ::std::option::Option::None,
                    stamina: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.UseItemPotionResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: UseItemPotionResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> UseItemPotionResponse_Result {
        self.result.unwrap_or(UseItemPotionResponse_Result::UNSET)
    }

    // optional int32 stamina = 2;

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
}

impl ::protobuf::Message for UseItemPotionResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.stamina = ::std::option::Option::Some(tmp);
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.stamina {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.stamina {
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
        ::std::any::TypeId::of::<UseItemPotionResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UseItemPotionResponse {
    fn new() -> UseItemPotionResponse {
        UseItemPotionResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<UseItemPotionResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    UseItemPotionResponse::has_result,
                    UseItemPotionResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "stamina",
                    UseItemPotionResponse::has_stamina,
                    UseItemPotionResponse::get_stamina,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UseItemPotionResponse>(
                    "UseItemPotionResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UseItemPotionResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_stamina();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UseItemPotionResponse {
    fn eq(&self, other: &UseItemPotionResponse) -> bool {
        self.result == other.result &&
        self.stamina == other.stamina &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UseItemPotionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum UseItemPotionResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    ERROR_NO_POKEMON = 2,
    ERROR_CANNOT_USE = 3,
    ERROR_DEPLOYED_TO_FORT = 4,
}

impl ::protobuf::ProtobufEnum for UseItemPotionResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UseItemPotionResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(UseItemPotionResponse_Result::UNSET),
            1 => ::std::option::Option::Some(UseItemPotionResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(UseItemPotionResponse_Result::ERROR_NO_POKEMON),
            3 => ::std::option::Option::Some(UseItemPotionResponse_Result::ERROR_CANNOT_USE),
            4 => ::std::option::Option::Some(UseItemPotionResponse_Result::ERROR_DEPLOYED_TO_FORT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [UseItemPotionResponse_Result] = &[
            UseItemPotionResponse_Result::UNSET,
            UseItemPotionResponse_Result::SUCCESS,
            UseItemPotionResponse_Result::ERROR_NO_POKEMON,
            UseItemPotionResponse_Result::ERROR_CANNOT_USE,
            UseItemPotionResponse_Result::ERROR_DEPLOYED_TO_FORT,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<UseItemPotionResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("UseItemPotionResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for UseItemPotionResponse_Result {
}

#[derive(Clone,Default)]
pub struct EquipBadgeResponse {
    // message fields
    result: ::std::option::Option<EquipBadgeResponse_Result>,
    equipped: ::protobuf::SingularPtrField<super::POGOProtos_Data_Player::EquippedBadge>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EquipBadgeResponse {}

impl EquipBadgeResponse {
    pub fn new() -> EquipBadgeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EquipBadgeResponse {
        static mut instance: ::protobuf::lazy::Lazy<EquipBadgeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EquipBadgeResponse,
        };
        unsafe {
            instance.get(|| {
                EquipBadgeResponse {
                    result: ::std::option::Option::None,
                    equipped: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.EquipBadgeResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: EquipBadgeResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> EquipBadgeResponse_Result {
        self.result.unwrap_or(EquipBadgeResponse_Result::UNSET)
    }

    // optional .POGOProtos.Data.Player.EquippedBadge equipped = 2;

    pub fn clear_equipped(&mut self) {
        self.equipped.clear();
    }

    pub fn has_equipped(&self) -> bool {
        self.equipped.is_some()
    }

    // Param is passed by value, moved
    pub fn set_equipped(&mut self, v: super::POGOProtos_Data_Player::EquippedBadge) {
        self.equipped = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_equipped(&mut self) -> &mut super::POGOProtos_Data_Player::EquippedBadge {
        if self.equipped.is_none() {
            self.equipped.set_default();
        };
        self.equipped.as_mut().unwrap()
    }

    // Take field
    pub fn take_equipped(&mut self) -> super::POGOProtos_Data_Player::EquippedBadge {
        self.equipped.take().unwrap_or_else(|| super::POGOProtos_Data_Player::EquippedBadge::new())
    }

    pub fn get_equipped(&self) -> &super::POGOProtos_Data_Player::EquippedBadge {
        self.equipped.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Player::EquippedBadge::default_instance())
    }
}

impl ::protobuf::Message for EquipBadgeResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.equipped));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.equipped {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.equipped.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<EquipBadgeResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EquipBadgeResponse {
    fn new() -> EquipBadgeResponse {
        EquipBadgeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<EquipBadgeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    EquipBadgeResponse::has_result,
                    EquipBadgeResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "equipped",
                    EquipBadgeResponse::has_equipped,
                    EquipBadgeResponse::get_equipped,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EquipBadgeResponse>(
                    "EquipBadgeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EquipBadgeResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_equipped();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EquipBadgeResponse {
    fn eq(&self, other: &EquipBadgeResponse) -> bool {
        self.result == other.result &&
        self.equipped == other.equipped &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EquipBadgeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EquipBadgeResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    COOLDOWN_ACTIVE = 2,
    NOT_QUALIFIED = 3,
}

impl ::protobuf::ProtobufEnum for EquipBadgeResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EquipBadgeResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(EquipBadgeResponse_Result::UNSET),
            1 => ::std::option::Option::Some(EquipBadgeResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(EquipBadgeResponse_Result::COOLDOWN_ACTIVE),
            3 => ::std::option::Option::Some(EquipBadgeResponse_Result::NOT_QUALIFIED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EquipBadgeResponse_Result] = &[
            EquipBadgeResponse_Result::UNSET,
            EquipBadgeResponse_Result::SUCCESS,
            EquipBadgeResponse_Result::COOLDOWN_ACTIVE,
            EquipBadgeResponse_Result::NOT_QUALIFIED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EquipBadgeResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EquipBadgeResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EquipBadgeResponse_Result {
}

#[derive(Clone,Default)]
pub struct UpgradePokemonResponse {
    // message fields
    result: ::std::option::Option<UpgradePokemonResponse_Result>,
    upgraded_pokemon: ::protobuf::SingularPtrField<super::POGOProtos_Data::PokemonData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpgradePokemonResponse {}

impl UpgradePokemonResponse {
    pub fn new() -> UpgradePokemonResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpgradePokemonResponse {
        static mut instance: ::protobuf::lazy::Lazy<UpgradePokemonResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpgradePokemonResponse,
        };
        unsafe {
            instance.get(|| {
                UpgradePokemonResponse {
                    result: ::std::option::Option::None,
                    upgraded_pokemon: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.UpgradePokemonResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: UpgradePokemonResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> UpgradePokemonResponse_Result {
        self.result.unwrap_or(UpgradePokemonResponse_Result::UNSET)
    }

    // optional .POGOProtos.Data.PokemonData upgraded_pokemon = 2;

    pub fn clear_upgraded_pokemon(&mut self) {
        self.upgraded_pokemon.clear();
    }

    pub fn has_upgraded_pokemon(&self) -> bool {
        self.upgraded_pokemon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upgraded_pokemon(&mut self, v: super::POGOProtos_Data::PokemonData) {
        self.upgraded_pokemon = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_upgraded_pokemon(&mut self) -> &mut super::POGOProtos_Data::PokemonData {
        if self.upgraded_pokemon.is_none() {
            self.upgraded_pokemon.set_default();
        };
        self.upgraded_pokemon.as_mut().unwrap()
    }

    // Take field
    pub fn take_upgraded_pokemon(&mut self) -> super::POGOProtos_Data::PokemonData {
        self.upgraded_pokemon.take().unwrap_or_else(|| super::POGOProtos_Data::PokemonData::new())
    }

    pub fn get_upgraded_pokemon(&self) -> &super::POGOProtos_Data::PokemonData {
        self.upgraded_pokemon.as_ref().unwrap_or_else(|| super::POGOProtos_Data::PokemonData::default_instance())
    }
}

impl ::protobuf::Message for UpgradePokemonResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.upgraded_pokemon));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.upgraded_pokemon {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.upgraded_pokemon.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<UpgradePokemonResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UpgradePokemonResponse {
    fn new() -> UpgradePokemonResponse {
        UpgradePokemonResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpgradePokemonResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    UpgradePokemonResponse::has_result,
                    UpgradePokemonResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "upgraded_pokemon",
                    UpgradePokemonResponse::has_upgraded_pokemon,
                    UpgradePokemonResponse::get_upgraded_pokemon,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpgradePokemonResponse>(
                    "UpgradePokemonResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpgradePokemonResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_upgraded_pokemon();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UpgradePokemonResponse {
    fn eq(&self, other: &UpgradePokemonResponse) -> bool {
        self.result == other.result &&
        self.upgraded_pokemon == other.upgraded_pokemon &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UpgradePokemonResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum UpgradePokemonResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    ERROR_POKEMON_NOT_FOUND = 2,
    ERROR_INSUFFICIENT_RESOURCES = 3,
    ERROR_UPGRADE_NOT_AVAILABLE = 4,
    ERROR_POKEMON_IS_DEPLOYED = 5,
}

impl ::protobuf::ProtobufEnum for UpgradePokemonResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UpgradePokemonResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(UpgradePokemonResponse_Result::UNSET),
            1 => ::std::option::Option::Some(UpgradePokemonResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(UpgradePokemonResponse_Result::ERROR_POKEMON_NOT_FOUND),
            3 => ::std::option::Option::Some(UpgradePokemonResponse_Result::ERROR_INSUFFICIENT_RESOURCES),
            4 => ::std::option::Option::Some(UpgradePokemonResponse_Result::ERROR_UPGRADE_NOT_AVAILABLE),
            5 => ::std::option::Option::Some(UpgradePokemonResponse_Result::ERROR_POKEMON_IS_DEPLOYED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [UpgradePokemonResponse_Result] = &[
            UpgradePokemonResponse_Result::UNSET,
            UpgradePokemonResponse_Result::SUCCESS,
            UpgradePokemonResponse_Result::ERROR_POKEMON_NOT_FOUND,
            UpgradePokemonResponse_Result::ERROR_INSUFFICIENT_RESOURCES,
            UpgradePokemonResponse_Result::ERROR_UPGRADE_NOT_AVAILABLE,
            UpgradePokemonResponse_Result::ERROR_POKEMON_IS_DEPLOYED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<UpgradePokemonResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("UpgradePokemonResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for UpgradePokemonResponse_Result {
}

#[derive(Clone,Default)]
pub struct FortSearchResponse {
    // message fields
    result: ::std::option::Option<FortSearchResponse_Result>,
    items_awarded: ::protobuf::RepeatedField<super::POGOProtos_Inventory_Item::ItemAward>,
    gems_awarded: ::std::option::Option<i32>,
    pokemon_data_egg: ::protobuf::SingularPtrField<super::POGOProtos_Data::PokemonData>,
    experience_awarded: ::std::option::Option<i32>,
    cooldown_complete_timestamp_ms: ::std::option::Option<i64>,
    chain_hack_sequence_number: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FortSearchResponse {}

impl FortSearchResponse {
    pub fn new() -> FortSearchResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FortSearchResponse {
        static mut instance: ::protobuf::lazy::Lazy<FortSearchResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FortSearchResponse,
        };
        unsafe {
            instance.get(|| {
                FortSearchResponse {
                    result: ::std::option::Option::None,
                    items_awarded: ::protobuf::RepeatedField::new(),
                    gems_awarded: ::std::option::Option::None,
                    pokemon_data_egg: ::protobuf::SingularPtrField::none(),
                    experience_awarded: ::std::option::Option::None,
                    cooldown_complete_timestamp_ms: ::std::option::Option::None,
                    chain_hack_sequence_number: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.FortSearchResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: FortSearchResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> FortSearchResponse_Result {
        self.result.unwrap_or(FortSearchResponse_Result::NO_RESULT_SET)
    }

    // repeated .POGOProtos.Inventory.Item.ItemAward items_awarded = 2;

    pub fn clear_items_awarded(&mut self) {
        self.items_awarded.clear();
    }

    // Param is passed by value, moved
    pub fn set_items_awarded(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Inventory_Item::ItemAward>) {
        self.items_awarded = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items_awarded(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Inventory_Item::ItemAward> {
        &mut self.items_awarded
    }

    // Take field
    pub fn take_items_awarded(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Inventory_Item::ItemAward> {
        ::std::mem::replace(&mut self.items_awarded, ::protobuf::RepeatedField::new())
    }

    pub fn get_items_awarded(&self) -> &[super::POGOProtos_Inventory_Item::ItemAward] {
        &self.items_awarded
    }

    // optional int32 gems_awarded = 3;

    pub fn clear_gems_awarded(&mut self) {
        self.gems_awarded = ::std::option::Option::None;
    }

    pub fn has_gems_awarded(&self) -> bool {
        self.gems_awarded.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gems_awarded(&mut self, v: i32) {
        self.gems_awarded = ::std::option::Option::Some(v);
    }

    pub fn get_gems_awarded(&self) -> i32 {
        self.gems_awarded.unwrap_or(0)
    }

    // optional .POGOProtos.Data.PokemonData pokemon_data_egg = 4;

    pub fn clear_pokemon_data_egg(&mut self) {
        self.pokemon_data_egg.clear();
    }

    pub fn has_pokemon_data_egg(&self) -> bool {
        self.pokemon_data_egg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_data_egg(&mut self, v: super::POGOProtos_Data::PokemonData) {
        self.pokemon_data_egg = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pokemon_data_egg(&mut self) -> &mut super::POGOProtos_Data::PokemonData {
        if self.pokemon_data_egg.is_none() {
            self.pokemon_data_egg.set_default();
        };
        self.pokemon_data_egg.as_mut().unwrap()
    }

    // Take field
    pub fn take_pokemon_data_egg(&mut self) -> super::POGOProtos_Data::PokemonData {
        self.pokemon_data_egg.take().unwrap_or_else(|| super::POGOProtos_Data::PokemonData::new())
    }

    pub fn get_pokemon_data_egg(&self) -> &super::POGOProtos_Data::PokemonData {
        self.pokemon_data_egg.as_ref().unwrap_or_else(|| super::POGOProtos_Data::PokemonData::default_instance())
    }

    // optional int32 experience_awarded = 5;

    pub fn clear_experience_awarded(&mut self) {
        self.experience_awarded = ::std::option::Option::None;
    }

    pub fn has_experience_awarded(&self) -> bool {
        self.experience_awarded.is_some()
    }

    // Param is passed by value, moved
    pub fn set_experience_awarded(&mut self, v: i32) {
        self.experience_awarded = ::std::option::Option::Some(v);
    }

    pub fn get_experience_awarded(&self) -> i32 {
        self.experience_awarded.unwrap_or(0)
    }

    // optional int64 cooldown_complete_timestamp_ms = 6;

    pub fn clear_cooldown_complete_timestamp_ms(&mut self) {
        self.cooldown_complete_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_cooldown_complete_timestamp_ms(&self) -> bool {
        self.cooldown_complete_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cooldown_complete_timestamp_ms(&mut self, v: i64) {
        self.cooldown_complete_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_cooldown_complete_timestamp_ms(&self) -> i64 {
        self.cooldown_complete_timestamp_ms.unwrap_or(0)
    }

    // optional int32 chain_hack_sequence_number = 7;

    pub fn clear_chain_hack_sequence_number(&mut self) {
        self.chain_hack_sequence_number = ::std::option::Option::None;
    }

    pub fn has_chain_hack_sequence_number(&self) -> bool {
        self.chain_hack_sequence_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chain_hack_sequence_number(&mut self, v: i32) {
        self.chain_hack_sequence_number = ::std::option::Option::Some(v);
    }

    pub fn get_chain_hack_sequence_number(&self) -> i32 {
        self.chain_hack_sequence_number.unwrap_or(0)
    }
}

impl ::protobuf::Message for FortSearchResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items_awarded));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.gems_awarded = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pokemon_data_egg));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.experience_awarded = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.cooldown_complete_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.chain_hack_sequence_number = ::std::option::Option::Some(tmp);
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.items_awarded {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.gems_awarded {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pokemon_data_egg {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.experience_awarded {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.cooldown_complete_timestamp_ms {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.chain_hack_sequence_number {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        for v in &self.items_awarded {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.gems_awarded {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.pokemon_data_egg.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.experience_awarded {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.cooldown_complete_timestamp_ms {
            try!(os.write_int64(6, v));
        };
        if let Some(v) = self.chain_hack_sequence_number {
            try!(os.write_int32(7, v));
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
        ::std::any::TypeId::of::<FortSearchResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FortSearchResponse {
    fn new() -> FortSearchResponse {
        FortSearchResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<FortSearchResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    FortSearchResponse::has_result,
                    FortSearchResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "items_awarded",
                    FortSearchResponse::get_items_awarded,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "gems_awarded",
                    FortSearchResponse::has_gems_awarded,
                    FortSearchResponse::get_gems_awarded,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pokemon_data_egg",
                    FortSearchResponse::has_pokemon_data_egg,
                    FortSearchResponse::get_pokemon_data_egg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "experience_awarded",
                    FortSearchResponse::has_experience_awarded,
                    FortSearchResponse::get_experience_awarded,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "cooldown_complete_timestamp_ms",
                    FortSearchResponse::has_cooldown_complete_timestamp_ms,
                    FortSearchResponse::get_cooldown_complete_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "chain_hack_sequence_number",
                    FortSearchResponse::has_chain_hack_sequence_number,
                    FortSearchResponse::get_chain_hack_sequence_number,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FortSearchResponse>(
                    "FortSearchResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FortSearchResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_items_awarded();
        self.clear_gems_awarded();
        self.clear_pokemon_data_egg();
        self.clear_experience_awarded();
        self.clear_cooldown_complete_timestamp_ms();
        self.clear_chain_hack_sequence_number();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FortSearchResponse {
    fn eq(&self, other: &FortSearchResponse) -> bool {
        self.result == other.result &&
        self.items_awarded == other.items_awarded &&
        self.gems_awarded == other.gems_awarded &&
        self.pokemon_data_egg == other.pokemon_data_egg &&
        self.experience_awarded == other.experience_awarded &&
        self.cooldown_complete_timestamp_ms == other.cooldown_complete_timestamp_ms &&
        self.chain_hack_sequence_number == other.chain_hack_sequence_number &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FortSearchResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FortSearchResponse_Result {
    NO_RESULT_SET = 0,
    SUCCESS = 1,
    OUT_OF_RANGE = 2,
    IN_COOLDOWN_PERIOD = 3,
    INVENTORY_FULL = 4,
    EXCEEDED_DAILY_LIMIT = 5,
}

impl ::protobuf::ProtobufEnum for FortSearchResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FortSearchResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(FortSearchResponse_Result::NO_RESULT_SET),
            1 => ::std::option::Option::Some(FortSearchResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(FortSearchResponse_Result::OUT_OF_RANGE),
            3 => ::std::option::Option::Some(FortSearchResponse_Result::IN_COOLDOWN_PERIOD),
            4 => ::std::option::Option::Some(FortSearchResponse_Result::INVENTORY_FULL),
            5 => ::std::option::Option::Some(FortSearchResponse_Result::EXCEEDED_DAILY_LIMIT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FortSearchResponse_Result] = &[
            FortSearchResponse_Result::NO_RESULT_SET,
            FortSearchResponse_Result::SUCCESS,
            FortSearchResponse_Result::OUT_OF_RANGE,
            FortSearchResponse_Result::IN_COOLDOWN_PERIOD,
            FortSearchResponse_Result::INVENTORY_FULL,
            FortSearchResponse_Result::EXCEEDED_DAILY_LIMIT,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<FortSearchResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("FortSearchResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for FortSearchResponse_Result {
}

#[derive(Clone,Default)]
pub struct SetFavoritePokemonResponse {
    // message fields
    result: ::std::option::Option<SetFavoritePokemonResponse_Result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetFavoritePokemonResponse {}

impl SetFavoritePokemonResponse {
    pub fn new() -> SetFavoritePokemonResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetFavoritePokemonResponse {
        static mut instance: ::protobuf::lazy::Lazy<SetFavoritePokemonResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetFavoritePokemonResponse,
        };
        unsafe {
            instance.get(|| {
                SetFavoritePokemonResponse {
                    result: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.SetFavoritePokemonResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: SetFavoritePokemonResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> SetFavoritePokemonResponse_Result {
        self.result.unwrap_or(SetFavoritePokemonResponse_Result::UNSET)
    }
}

impl ::protobuf::Message for SetFavoritePokemonResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
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
        ::std::any::TypeId::of::<SetFavoritePokemonResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetFavoritePokemonResponse {
    fn new() -> SetFavoritePokemonResponse {
        SetFavoritePokemonResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetFavoritePokemonResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    SetFavoritePokemonResponse::has_result,
                    SetFavoritePokemonResponse::get_result,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetFavoritePokemonResponse>(
                    "SetFavoritePokemonResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetFavoritePokemonResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetFavoritePokemonResponse {
    fn eq(&self, other: &SetFavoritePokemonResponse) -> bool {
        self.result == other.result &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetFavoritePokemonResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SetFavoritePokemonResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    ERROR_POKEMON_NOT_FOUND = 2,
    ERROR_POKEMON_IS_EGG = 3,
}

impl ::protobuf::ProtobufEnum for SetFavoritePokemonResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SetFavoritePokemonResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(SetFavoritePokemonResponse_Result::UNSET),
            1 => ::std::option::Option::Some(SetFavoritePokemonResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(SetFavoritePokemonResponse_Result::ERROR_POKEMON_NOT_FOUND),
            3 => ::std::option::Option::Some(SetFavoritePokemonResponse_Result::ERROR_POKEMON_IS_EGG),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SetFavoritePokemonResponse_Result] = &[
            SetFavoritePokemonResponse_Result::UNSET,
            SetFavoritePokemonResponse_Result::SUCCESS,
            SetFavoritePokemonResponse_Result::ERROR_POKEMON_NOT_FOUND,
            SetFavoritePokemonResponse_Result::ERROR_POKEMON_IS_EGG,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<SetFavoritePokemonResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SetFavoritePokemonResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SetFavoritePokemonResponse_Result {
}

#[derive(Clone,Default)]
pub struct UseItemGymResponse {
    // message fields
    result: ::std::option::Option<UseItemGymResponse_Result>,
    updated_gp: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UseItemGymResponse {}

impl UseItemGymResponse {
    pub fn new() -> UseItemGymResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UseItemGymResponse {
        static mut instance: ::protobuf::lazy::Lazy<UseItemGymResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UseItemGymResponse,
        };
        unsafe {
            instance.get(|| {
                UseItemGymResponse {
                    result: ::std::option::Option::None,
                    updated_gp: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.UseItemGymResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: UseItemGymResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> UseItemGymResponse_Result {
        self.result.unwrap_or(UseItemGymResponse_Result::UNSET)
    }

    // optional int64 updated_gp = 2;

    pub fn clear_updated_gp(&mut self) {
        self.updated_gp = ::std::option::Option::None;
    }

    pub fn has_updated_gp(&self) -> bool {
        self.updated_gp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_updated_gp(&mut self, v: i64) {
        self.updated_gp = ::std::option::Option::Some(v);
    }

    pub fn get_updated_gp(&self) -> i64 {
        self.updated_gp.unwrap_or(0)
    }
}

impl ::protobuf::Message for UseItemGymResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.updated_gp = ::std::option::Option::Some(tmp);
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.updated_gp {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.updated_gp {
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
        ::std::any::TypeId::of::<UseItemGymResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UseItemGymResponse {
    fn new() -> UseItemGymResponse {
        UseItemGymResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<UseItemGymResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    UseItemGymResponse::has_result,
                    UseItemGymResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "updated_gp",
                    UseItemGymResponse::has_updated_gp,
                    UseItemGymResponse::get_updated_gp,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UseItemGymResponse>(
                    "UseItemGymResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UseItemGymResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_updated_gp();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UseItemGymResponse {
    fn eq(&self, other: &UseItemGymResponse) -> bool {
        self.result == other.result &&
        self.updated_gp == other.updated_gp &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UseItemGymResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum UseItemGymResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    ERROR_CANNOT_USE = 2,
    ERROR_NOT_IN_RANGE = 3,
}

impl ::protobuf::ProtobufEnum for UseItemGymResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UseItemGymResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(UseItemGymResponse_Result::UNSET),
            1 => ::std::option::Option::Some(UseItemGymResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(UseItemGymResponse_Result::ERROR_CANNOT_USE),
            3 => ::std::option::Option::Some(UseItemGymResponse_Result::ERROR_NOT_IN_RANGE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [UseItemGymResponse_Result] = &[
            UseItemGymResponse_Result::UNSET,
            UseItemGymResponse_Result::SUCCESS,
            UseItemGymResponse_Result::ERROR_CANNOT_USE,
            UseItemGymResponse_Result::ERROR_NOT_IN_RANGE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<UseItemGymResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("UseItemGymResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for UseItemGymResponse_Result {
}

#[derive(Clone,Default)]
pub struct CatchPokemonResponse {
    // message fields
    status: ::std::option::Option<CatchPokemonResponse_CatchStatus>,
    miss_percent: ::std::option::Option<f64>,
    captured_pokemon_id: ::std::option::Option<u64>,
    capture_award: ::protobuf::SingularPtrField<super::POGOProtos_Data_Capture::CaptureAward>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CatchPokemonResponse {}

impl CatchPokemonResponse {
    pub fn new() -> CatchPokemonResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CatchPokemonResponse {
        static mut instance: ::protobuf::lazy::Lazy<CatchPokemonResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CatchPokemonResponse,
        };
        unsafe {
            instance.get(|| {
                CatchPokemonResponse {
                    status: ::std::option::Option::None,
                    miss_percent: ::std::option::Option::None,
                    captured_pokemon_id: ::std::option::Option::None,
                    capture_award: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.CatchPokemonResponse.CatchStatus status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: CatchPokemonResponse_CatchStatus) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> CatchPokemonResponse_CatchStatus {
        self.status.unwrap_or(CatchPokemonResponse_CatchStatus::CATCH_ERROR)
    }

    // optional double miss_percent = 2;

    pub fn clear_miss_percent(&mut self) {
        self.miss_percent = ::std::option::Option::None;
    }

    pub fn has_miss_percent(&self) -> bool {
        self.miss_percent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_miss_percent(&mut self, v: f64) {
        self.miss_percent = ::std::option::Option::Some(v);
    }

    pub fn get_miss_percent(&self) -> f64 {
        self.miss_percent.unwrap_or(0.)
    }

    // optional fixed64 captured_pokemon_id = 3;

    pub fn clear_captured_pokemon_id(&mut self) {
        self.captured_pokemon_id = ::std::option::Option::None;
    }

    pub fn has_captured_pokemon_id(&self) -> bool {
        self.captured_pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_captured_pokemon_id(&mut self, v: u64) {
        self.captured_pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_captured_pokemon_id(&self) -> u64 {
        self.captured_pokemon_id.unwrap_or(0)
    }

    // optional .POGOProtos.Data.Capture.CaptureAward capture_award = 4;

    pub fn clear_capture_award(&mut self) {
        self.capture_award.clear();
    }

    pub fn has_capture_award(&self) -> bool {
        self.capture_award.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capture_award(&mut self, v: super::POGOProtos_Data_Capture::CaptureAward) {
        self.capture_award = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_capture_award(&mut self) -> &mut super::POGOProtos_Data_Capture::CaptureAward {
        if self.capture_award.is_none() {
            self.capture_award.set_default();
        };
        self.capture_award.as_mut().unwrap()
    }

    // Take field
    pub fn take_capture_award(&mut self) -> super::POGOProtos_Data_Capture::CaptureAward {
        self.capture_award.take().unwrap_or_else(|| super::POGOProtos_Data_Capture::CaptureAward::new())
    }

    pub fn get_capture_award(&self) -> &super::POGOProtos_Data_Capture::CaptureAward {
        self.capture_award.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Capture::CaptureAward::default_instance())
    }
}

impl ::protobuf::Message for CatchPokemonResponse {
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
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.miss_percent = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.captured_pokemon_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.capture_award));
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
        for value in &self.status {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        if self.miss_percent.is_some() {
            my_size += 9;
        };
        if self.captured_pokemon_id.is_some() {
            my_size += 9;
        };
        for value in &self.capture_award {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.miss_percent {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.captured_pokemon_id {
            try!(os.write_fixed64(3, v));
        };
        if let Some(v) = self.capture_award.as_ref() {
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
        ::std::any::TypeId::of::<CatchPokemonResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CatchPokemonResponse {
    fn new() -> CatchPokemonResponse {
        CatchPokemonResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CatchPokemonResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    CatchPokemonResponse::has_status,
                    CatchPokemonResponse::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "miss_percent",
                    CatchPokemonResponse::has_miss_percent,
                    CatchPokemonResponse::get_miss_percent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "captured_pokemon_id",
                    CatchPokemonResponse::has_captured_pokemon_id,
                    CatchPokemonResponse::get_captured_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "capture_award",
                    CatchPokemonResponse::has_capture_award,
                    CatchPokemonResponse::get_capture_award,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CatchPokemonResponse>(
                    "CatchPokemonResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CatchPokemonResponse {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_miss_percent();
        self.clear_captured_pokemon_id();
        self.clear_capture_award();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CatchPokemonResponse {
    fn eq(&self, other: &CatchPokemonResponse) -> bool {
        self.status == other.status &&
        self.miss_percent == other.miss_percent &&
        self.captured_pokemon_id == other.captured_pokemon_id &&
        self.capture_award == other.capture_award &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CatchPokemonResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CatchPokemonResponse_CatchStatus {
    CATCH_ERROR = 0,
    CATCH_SUCCESS = 1,
    CATCH_ESCAPE = 2,
    CATCH_FLEE = 3,
    CATCH_MISSED = 4,
}

impl ::protobuf::ProtobufEnum for CatchPokemonResponse_CatchStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CatchPokemonResponse_CatchStatus> {
        match value {
            0 => ::std::option::Option::Some(CatchPokemonResponse_CatchStatus::CATCH_ERROR),
            1 => ::std::option::Option::Some(CatchPokemonResponse_CatchStatus::CATCH_SUCCESS),
            2 => ::std::option::Option::Some(CatchPokemonResponse_CatchStatus::CATCH_ESCAPE),
            3 => ::std::option::Option::Some(CatchPokemonResponse_CatchStatus::CATCH_FLEE),
            4 => ::std::option::Option::Some(CatchPokemonResponse_CatchStatus::CATCH_MISSED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CatchPokemonResponse_CatchStatus] = &[
            CatchPokemonResponse_CatchStatus::CATCH_ERROR,
            CatchPokemonResponse_CatchStatus::CATCH_SUCCESS,
            CatchPokemonResponse_CatchStatus::CATCH_ESCAPE,
            CatchPokemonResponse_CatchStatus::CATCH_FLEE,
            CatchPokemonResponse_CatchStatus::CATCH_MISSED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CatchPokemonResponse_CatchStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CatchPokemonResponse_CatchStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CatchPokemonResponse_CatchStatus {
}

#[derive(Clone,Default)]
pub struct SfidaActionLogResponse {
    // message fields
    result: ::std::option::Option<SfidaActionLogResponse_Result>,
    log_entries: ::protobuf::RepeatedField<super::POGOProtos_Data_Logs::ActionLogEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SfidaActionLogResponse {}

impl SfidaActionLogResponse {
    pub fn new() -> SfidaActionLogResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SfidaActionLogResponse {
        static mut instance: ::protobuf::lazy::Lazy<SfidaActionLogResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SfidaActionLogResponse,
        };
        unsafe {
            instance.get(|| {
                SfidaActionLogResponse {
                    result: ::std::option::Option::None,
                    log_entries: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.SfidaActionLogResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: SfidaActionLogResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> SfidaActionLogResponse_Result {
        self.result.unwrap_or(SfidaActionLogResponse_Result::UNSET)
    }

    // repeated .POGOProtos.Data.Logs.ActionLogEntry log_entries = 2;

    pub fn clear_log_entries(&mut self) {
        self.log_entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_log_entries(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Data_Logs::ActionLogEntry>) {
        self.log_entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_log_entries(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Data_Logs::ActionLogEntry> {
        &mut self.log_entries
    }

    // Take field
    pub fn take_log_entries(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Data_Logs::ActionLogEntry> {
        ::std::mem::replace(&mut self.log_entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_log_entries(&self) -> &[super::POGOProtos_Data_Logs::ActionLogEntry] {
        &self.log_entries
    }
}

impl ::protobuf::Message for SfidaActionLogResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.log_entries));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.log_entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        for v in &self.log_entries {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<SfidaActionLogResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SfidaActionLogResponse {
    fn new() -> SfidaActionLogResponse {
        SfidaActionLogResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SfidaActionLogResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    SfidaActionLogResponse::has_result,
                    SfidaActionLogResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "log_entries",
                    SfidaActionLogResponse::get_log_entries,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SfidaActionLogResponse>(
                    "SfidaActionLogResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SfidaActionLogResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_log_entries();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SfidaActionLogResponse {
    fn eq(&self, other: &SfidaActionLogResponse) -> bool {
        self.result == other.result &&
        self.log_entries == other.log_entries &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SfidaActionLogResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SfidaActionLogResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
}

impl ::protobuf::ProtobufEnum for SfidaActionLogResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SfidaActionLogResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(SfidaActionLogResponse_Result::UNSET),
            1 => ::std::option::Option::Some(SfidaActionLogResponse_Result::SUCCESS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SfidaActionLogResponse_Result] = &[
            SfidaActionLogResponse_Result::UNSET,
            SfidaActionLogResponse_Result::SUCCESS,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<SfidaActionLogResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SfidaActionLogResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SfidaActionLogResponse_Result {
}

#[derive(Clone,Default)]
pub struct UseItemCaptureResponse {
    // message fields
    success: ::std::option::Option<bool>,
    item_capture_mult: ::std::option::Option<f64>,
    item_flee_mult: ::std::option::Option<f64>,
    stop_movement: ::std::option::Option<bool>,
    stop_attack: ::std::option::Option<bool>,
    target_max: ::std::option::Option<bool>,
    target_slow: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UseItemCaptureResponse {}

impl UseItemCaptureResponse {
    pub fn new() -> UseItemCaptureResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UseItemCaptureResponse {
        static mut instance: ::protobuf::lazy::Lazy<UseItemCaptureResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UseItemCaptureResponse,
        };
        unsafe {
            instance.get(|| {
                UseItemCaptureResponse {
                    success: ::std::option::Option::None,
                    item_capture_mult: ::std::option::Option::None,
                    item_flee_mult: ::std::option::Option::None,
                    stop_movement: ::std::option::Option::None,
                    stop_attack: ::std::option::Option::None,
                    target_max: ::std::option::Option::None,
                    target_slow: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // optional double item_capture_mult = 2;

    pub fn clear_item_capture_mult(&mut self) {
        self.item_capture_mult = ::std::option::Option::None;
    }

    pub fn has_item_capture_mult(&self) -> bool {
        self.item_capture_mult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_capture_mult(&mut self, v: f64) {
        self.item_capture_mult = ::std::option::Option::Some(v);
    }

    pub fn get_item_capture_mult(&self) -> f64 {
        self.item_capture_mult.unwrap_or(0.)
    }

    // optional double item_flee_mult = 3;

    pub fn clear_item_flee_mult(&mut self) {
        self.item_flee_mult = ::std::option::Option::None;
    }

    pub fn has_item_flee_mult(&self) -> bool {
        self.item_flee_mult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_flee_mult(&mut self, v: f64) {
        self.item_flee_mult = ::std::option::Option::Some(v);
    }

    pub fn get_item_flee_mult(&self) -> f64 {
        self.item_flee_mult.unwrap_or(0.)
    }

    // optional bool stop_movement = 4;

    pub fn clear_stop_movement(&mut self) {
        self.stop_movement = ::std::option::Option::None;
    }

    pub fn has_stop_movement(&self) -> bool {
        self.stop_movement.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stop_movement(&mut self, v: bool) {
        self.stop_movement = ::std::option::Option::Some(v);
    }

    pub fn get_stop_movement(&self) -> bool {
        self.stop_movement.unwrap_or(false)
    }

    // optional bool stop_attack = 5;

    pub fn clear_stop_attack(&mut self) {
        self.stop_attack = ::std::option::Option::None;
    }

    pub fn has_stop_attack(&self) -> bool {
        self.stop_attack.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stop_attack(&mut self, v: bool) {
        self.stop_attack = ::std::option::Option::Some(v);
    }

    pub fn get_stop_attack(&self) -> bool {
        self.stop_attack.unwrap_or(false)
    }

    // optional bool target_max = 6;

    pub fn clear_target_max(&mut self) {
        self.target_max = ::std::option::Option::None;
    }

    pub fn has_target_max(&self) -> bool {
        self.target_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_max(&mut self, v: bool) {
        self.target_max = ::std::option::Option::Some(v);
    }

    pub fn get_target_max(&self) -> bool {
        self.target_max.unwrap_or(false)
    }

    // optional bool target_slow = 7;

    pub fn clear_target_slow(&mut self) {
        self.target_slow = ::std::option::Option::None;
    }

    pub fn has_target_slow(&self) -> bool {
        self.target_slow.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_slow(&mut self, v: bool) {
        self.target_slow = ::std::option::Option::Some(v);
    }

    pub fn get_target_slow(&self) -> bool {
        self.target_slow.unwrap_or(false)
    }
}

impl ::protobuf::Message for UseItemCaptureResponse {
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
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.item_capture_mult = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.item_flee_mult = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.stop_movement = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.stop_attack = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.target_max = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.target_slow = ::std::option::Option::Some(tmp);
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
        if self.success.is_some() {
            my_size += 2;
        };
        if self.item_capture_mult.is_some() {
            my_size += 9;
        };
        if self.item_flee_mult.is_some() {
            my_size += 9;
        };
        if self.stop_movement.is_some() {
            my_size += 2;
        };
        if self.stop_attack.is_some() {
            my_size += 2;
        };
        if self.target_max.is_some() {
            my_size += 2;
        };
        if self.target_slow.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.item_capture_mult {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.item_flee_mult {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.stop_movement {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.stop_attack {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.target_max {
            try!(os.write_bool(6, v));
        };
        if let Some(v) = self.target_slow {
            try!(os.write_bool(7, v));
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
        ::std::any::TypeId::of::<UseItemCaptureResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UseItemCaptureResponse {
    fn new() -> UseItemCaptureResponse {
        UseItemCaptureResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<UseItemCaptureResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    UseItemCaptureResponse::has_success,
                    UseItemCaptureResponse::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "item_capture_mult",
                    UseItemCaptureResponse::has_item_capture_mult,
                    UseItemCaptureResponse::get_item_capture_mult,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "item_flee_mult",
                    UseItemCaptureResponse::has_item_flee_mult,
                    UseItemCaptureResponse::get_item_flee_mult,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "stop_movement",
                    UseItemCaptureResponse::has_stop_movement,
                    UseItemCaptureResponse::get_stop_movement,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "stop_attack",
                    UseItemCaptureResponse::has_stop_attack,
                    UseItemCaptureResponse::get_stop_attack,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "target_max",
                    UseItemCaptureResponse::has_target_max,
                    UseItemCaptureResponse::get_target_max,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "target_slow",
                    UseItemCaptureResponse::has_target_slow,
                    UseItemCaptureResponse::get_target_slow,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UseItemCaptureResponse>(
                    "UseItemCaptureResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UseItemCaptureResponse {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_item_capture_mult();
        self.clear_item_flee_mult();
        self.clear_stop_movement();
        self.clear_stop_attack();
        self.clear_target_max();
        self.clear_target_slow();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UseItemCaptureResponse {
    fn eq(&self, other: &UseItemCaptureResponse) -> bool {
        self.success == other.success &&
        self.item_capture_mult == other.item_capture_mult &&
        self.item_flee_mult == other.item_flee_mult &&
        self.stop_movement == other.stop_movement &&
        self.stop_attack == other.stop_attack &&
        self.target_max == other.target_max &&
        self.target_slow == other.target_slow &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UseItemCaptureResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DownloadItemTemplatesResponse {
    // message fields
    success: ::std::option::Option<bool>,
    item_templates: ::protobuf::RepeatedField<DownloadItemTemplatesResponse_ItemTemplate>,
    timestamp_ms: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DownloadItemTemplatesResponse {}

impl DownloadItemTemplatesResponse {
    pub fn new() -> DownloadItemTemplatesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DownloadItemTemplatesResponse {
        static mut instance: ::protobuf::lazy::Lazy<DownloadItemTemplatesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DownloadItemTemplatesResponse,
        };
        unsafe {
            instance.get(|| {
                DownloadItemTemplatesResponse {
                    success: ::std::option::Option::None,
                    item_templates: ::protobuf::RepeatedField::new(),
                    timestamp_ms: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // repeated .POGOProtos.Networking.Responses.DownloadItemTemplatesResponse.ItemTemplate item_templates = 2;

    pub fn clear_item_templates(&mut self) {
        self.item_templates.clear();
    }

    // Param is passed by value, moved
    pub fn set_item_templates(&mut self, v: ::protobuf::RepeatedField<DownloadItemTemplatesResponse_ItemTemplate>) {
        self.item_templates = v;
    }

    // Mutable pointer to the field.
    pub fn mut_item_templates(&mut self) -> &mut ::protobuf::RepeatedField<DownloadItemTemplatesResponse_ItemTemplate> {
        &mut self.item_templates
    }

    // Take field
    pub fn take_item_templates(&mut self) -> ::protobuf::RepeatedField<DownloadItemTemplatesResponse_ItemTemplate> {
        ::std::mem::replace(&mut self.item_templates, ::protobuf::RepeatedField::new())
    }

    pub fn get_item_templates(&self) -> &[DownloadItemTemplatesResponse_ItemTemplate] {
        &self.item_templates
    }

    // optional uint64 timestamp_ms = 3;

    pub fn clear_timestamp_ms(&mut self) {
        self.timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_timestamp_ms(&self) -> bool {
        self.timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp_ms(&mut self, v: u64) {
        self.timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp_ms(&self) -> u64 {
        self.timestamp_ms.unwrap_or(0)
    }
}

impl ::protobuf::Message for DownloadItemTemplatesResponse {
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
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.item_templates));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.timestamp_ms = ::std::option::Option::Some(tmp);
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
        if self.success.is_some() {
            my_size += 2;
        };
        for value in &self.item_templates {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.timestamp_ms {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            try!(os.write_bool(1, v));
        };
        for v in &self.item_templates {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.timestamp_ms {
            try!(os.write_uint64(3, v));
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
        ::std::any::TypeId::of::<DownloadItemTemplatesResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DownloadItemTemplatesResponse {
    fn new() -> DownloadItemTemplatesResponse {
        DownloadItemTemplatesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DownloadItemTemplatesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    DownloadItemTemplatesResponse::has_success,
                    DownloadItemTemplatesResponse::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "item_templates",
                    DownloadItemTemplatesResponse::get_item_templates,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "timestamp_ms",
                    DownloadItemTemplatesResponse::has_timestamp_ms,
                    DownloadItemTemplatesResponse::get_timestamp_ms,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DownloadItemTemplatesResponse>(
                    "DownloadItemTemplatesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DownloadItemTemplatesResponse {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_item_templates();
        self.clear_timestamp_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DownloadItemTemplatesResponse {
    fn eq(&self, other: &DownloadItemTemplatesResponse) -> bool {
        self.success == other.success &&
        self.item_templates == other.item_templates &&
        self.timestamp_ms == other.timestamp_ms &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DownloadItemTemplatesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DownloadItemTemplatesResponse_ItemTemplate {
    // message fields
    template_id: ::protobuf::SingularField<::std::string::String>,
    pokemon_settings: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master::PokemonSettings>,
    item_settings: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master::ItemSettings>,
    move_settings: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master::MoveSettings>,
    move_sequence_settings: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master::MoveSequenceSettings>,
    type_effective: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master::TypeEffectiveSettings>,
    badge_settings: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master::BadgeSettings>,
    camera: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master::CameraSettings>,
    player_level: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master::PlayerLevelSettings>,
    gym_level: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master::GymLevelSettings>,
    battle_settings: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master::GymBattleSettings>,
    encounter_settings: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master::EncounterSettings>,
    iap_item_display: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master::IapItemDisplay>,
    iap_settings: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master::IapSettings>,
    pokemon_upgrades: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master::PokemonUpgradeSettings>,
    equipped_badges: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master::EquippedBadgeSettings>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DownloadItemTemplatesResponse_ItemTemplate {}

impl DownloadItemTemplatesResponse_ItemTemplate {
    pub fn new() -> DownloadItemTemplatesResponse_ItemTemplate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DownloadItemTemplatesResponse_ItemTemplate {
        static mut instance: ::protobuf::lazy::Lazy<DownloadItemTemplatesResponse_ItemTemplate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DownloadItemTemplatesResponse_ItemTemplate,
        };
        unsafe {
            instance.get(|| {
                DownloadItemTemplatesResponse_ItemTemplate {
                    template_id: ::protobuf::SingularField::none(),
                    pokemon_settings: ::protobuf::SingularPtrField::none(),
                    item_settings: ::protobuf::SingularPtrField::none(),
                    move_settings: ::protobuf::SingularPtrField::none(),
                    move_sequence_settings: ::protobuf::SingularPtrField::none(),
                    type_effective: ::protobuf::SingularPtrField::none(),
                    badge_settings: ::protobuf::SingularPtrField::none(),
                    camera: ::protobuf::SingularPtrField::none(),
                    player_level: ::protobuf::SingularPtrField::none(),
                    gym_level: ::protobuf::SingularPtrField::none(),
                    battle_settings: ::protobuf::SingularPtrField::none(),
                    encounter_settings: ::protobuf::SingularPtrField::none(),
                    iap_item_display: ::protobuf::SingularPtrField::none(),
                    iap_settings: ::protobuf::SingularPtrField::none(),
                    pokemon_upgrades: ::protobuf::SingularPtrField::none(),
                    equipped_badges: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string template_id = 1;

    pub fn clear_template_id(&mut self) {
        self.template_id.clear();
    }

    pub fn has_template_id(&self) -> bool {
        self.template_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_template_id(&mut self, v: ::std::string::String) {
        self.template_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_template_id(&mut self) -> &mut ::std::string::String {
        if self.template_id.is_none() {
            self.template_id.set_default();
        };
        self.template_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_template_id(&mut self) -> ::std::string::String {
        self.template_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_template_id(&self) -> &str {
        match self.template_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .POGOProtos.Settings.Master.PokemonSettings pokemon_settings = 2;

    pub fn clear_pokemon_settings(&mut self) {
        self.pokemon_settings.clear();
    }

    pub fn has_pokemon_settings(&self) -> bool {
        self.pokemon_settings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_settings(&mut self, v: super::POGOProtos_Settings_Master::PokemonSettings) {
        self.pokemon_settings = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pokemon_settings(&mut self) -> &mut super::POGOProtos_Settings_Master::PokemonSettings {
        if self.pokemon_settings.is_none() {
            self.pokemon_settings.set_default();
        };
        self.pokemon_settings.as_mut().unwrap()
    }

    // Take field
    pub fn take_pokemon_settings(&mut self) -> super::POGOProtos_Settings_Master::PokemonSettings {
        self.pokemon_settings.take().unwrap_or_else(|| super::POGOProtos_Settings_Master::PokemonSettings::new())
    }

    pub fn get_pokemon_settings(&self) -> &super::POGOProtos_Settings_Master::PokemonSettings {
        self.pokemon_settings.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master::PokemonSettings::default_instance())
    }

    // optional .POGOProtos.Settings.Master.ItemSettings item_settings = 3;

    pub fn clear_item_settings(&mut self) {
        self.item_settings.clear();
    }

    pub fn has_item_settings(&self) -> bool {
        self.item_settings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_settings(&mut self, v: super::POGOProtos_Settings_Master::ItemSettings) {
        self.item_settings = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_item_settings(&mut self) -> &mut super::POGOProtos_Settings_Master::ItemSettings {
        if self.item_settings.is_none() {
            self.item_settings.set_default();
        };
        self.item_settings.as_mut().unwrap()
    }

    // Take field
    pub fn take_item_settings(&mut self) -> super::POGOProtos_Settings_Master::ItemSettings {
        self.item_settings.take().unwrap_or_else(|| super::POGOProtos_Settings_Master::ItemSettings::new())
    }

    pub fn get_item_settings(&self) -> &super::POGOProtos_Settings_Master::ItemSettings {
        self.item_settings.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master::ItemSettings::default_instance())
    }

    // optional .POGOProtos.Settings.Master.MoveSettings move_settings = 4;

    pub fn clear_move_settings(&mut self) {
        self.move_settings.clear();
    }

    pub fn has_move_settings(&self) -> bool {
        self.move_settings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_move_settings(&mut self, v: super::POGOProtos_Settings_Master::MoveSettings) {
        self.move_settings = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_move_settings(&mut self) -> &mut super::POGOProtos_Settings_Master::MoveSettings {
        if self.move_settings.is_none() {
            self.move_settings.set_default();
        };
        self.move_settings.as_mut().unwrap()
    }

    // Take field
    pub fn take_move_settings(&mut self) -> super::POGOProtos_Settings_Master::MoveSettings {
        self.move_settings.take().unwrap_or_else(|| super::POGOProtos_Settings_Master::MoveSettings::new())
    }

    pub fn get_move_settings(&self) -> &super::POGOProtos_Settings_Master::MoveSettings {
        self.move_settings.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master::MoveSettings::default_instance())
    }

    // optional .POGOProtos.Settings.Master.MoveSequenceSettings move_sequence_settings = 5;

    pub fn clear_move_sequence_settings(&mut self) {
        self.move_sequence_settings.clear();
    }

    pub fn has_move_sequence_settings(&self) -> bool {
        self.move_sequence_settings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_move_sequence_settings(&mut self, v: super::POGOProtos_Settings_Master::MoveSequenceSettings) {
        self.move_sequence_settings = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_move_sequence_settings(&mut self) -> &mut super::POGOProtos_Settings_Master::MoveSequenceSettings {
        if self.move_sequence_settings.is_none() {
            self.move_sequence_settings.set_default();
        };
        self.move_sequence_settings.as_mut().unwrap()
    }

    // Take field
    pub fn take_move_sequence_settings(&mut self) -> super::POGOProtos_Settings_Master::MoveSequenceSettings {
        self.move_sequence_settings.take().unwrap_or_else(|| super::POGOProtos_Settings_Master::MoveSequenceSettings::new())
    }

    pub fn get_move_sequence_settings(&self) -> &super::POGOProtos_Settings_Master::MoveSequenceSettings {
        self.move_sequence_settings.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master::MoveSequenceSettings::default_instance())
    }

    // optional .POGOProtos.Settings.Master.TypeEffectiveSettings type_effective = 8;

    pub fn clear_type_effective(&mut self) {
        self.type_effective.clear();
    }

    pub fn has_type_effective(&self) -> bool {
        self.type_effective.is_some()
    }

    // Param is passed by value, moved
    pub fn set_type_effective(&mut self, v: super::POGOProtos_Settings_Master::TypeEffectiveSettings) {
        self.type_effective = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_type_effective(&mut self) -> &mut super::POGOProtos_Settings_Master::TypeEffectiveSettings {
        if self.type_effective.is_none() {
            self.type_effective.set_default();
        };
        self.type_effective.as_mut().unwrap()
    }

    // Take field
    pub fn take_type_effective(&mut self) -> super::POGOProtos_Settings_Master::TypeEffectiveSettings {
        self.type_effective.take().unwrap_or_else(|| super::POGOProtos_Settings_Master::TypeEffectiveSettings::new())
    }

    pub fn get_type_effective(&self) -> &super::POGOProtos_Settings_Master::TypeEffectiveSettings {
        self.type_effective.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master::TypeEffectiveSettings::default_instance())
    }

    // optional .POGOProtos.Settings.Master.BadgeSettings badge_settings = 10;

    pub fn clear_badge_settings(&mut self) {
        self.badge_settings.clear();
    }

    pub fn has_badge_settings(&self) -> bool {
        self.badge_settings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_badge_settings(&mut self, v: super::POGOProtos_Settings_Master::BadgeSettings) {
        self.badge_settings = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_badge_settings(&mut self) -> &mut super::POGOProtos_Settings_Master::BadgeSettings {
        if self.badge_settings.is_none() {
            self.badge_settings.set_default();
        };
        self.badge_settings.as_mut().unwrap()
    }

    // Take field
    pub fn take_badge_settings(&mut self) -> super::POGOProtos_Settings_Master::BadgeSettings {
        self.badge_settings.take().unwrap_or_else(|| super::POGOProtos_Settings_Master::BadgeSettings::new())
    }

    pub fn get_badge_settings(&self) -> &super::POGOProtos_Settings_Master::BadgeSettings {
        self.badge_settings.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master::BadgeSettings::default_instance())
    }

    // optional .POGOProtos.Settings.Master.CameraSettings camera = 11;

    pub fn clear_camera(&mut self) {
        self.camera.clear();
    }

    pub fn has_camera(&self) -> bool {
        self.camera.is_some()
    }

    // Param is passed by value, moved
    pub fn set_camera(&mut self, v: super::POGOProtos_Settings_Master::CameraSettings) {
        self.camera = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_camera(&mut self) -> &mut super::POGOProtos_Settings_Master::CameraSettings {
        if self.camera.is_none() {
            self.camera.set_default();
        };
        self.camera.as_mut().unwrap()
    }

    // Take field
    pub fn take_camera(&mut self) -> super::POGOProtos_Settings_Master::CameraSettings {
        self.camera.take().unwrap_or_else(|| super::POGOProtos_Settings_Master::CameraSettings::new())
    }

    pub fn get_camera(&self) -> &super::POGOProtos_Settings_Master::CameraSettings {
        self.camera.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master::CameraSettings::default_instance())
    }

    // optional .POGOProtos.Settings.Master.PlayerLevelSettings player_level = 12;

    pub fn clear_player_level(&mut self) {
        self.player_level.clear();
    }

    pub fn has_player_level(&self) -> bool {
        self.player_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_level(&mut self, v: super::POGOProtos_Settings_Master::PlayerLevelSettings) {
        self.player_level = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_level(&mut self) -> &mut super::POGOProtos_Settings_Master::PlayerLevelSettings {
        if self.player_level.is_none() {
            self.player_level.set_default();
        };
        self.player_level.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_level(&mut self) -> super::POGOProtos_Settings_Master::PlayerLevelSettings {
        self.player_level.take().unwrap_or_else(|| super::POGOProtos_Settings_Master::PlayerLevelSettings::new())
    }

    pub fn get_player_level(&self) -> &super::POGOProtos_Settings_Master::PlayerLevelSettings {
        self.player_level.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master::PlayerLevelSettings::default_instance())
    }

    // optional .POGOProtos.Settings.Master.GymLevelSettings gym_level = 13;

    pub fn clear_gym_level(&mut self) {
        self.gym_level.clear();
    }

    pub fn has_gym_level(&self) -> bool {
        self.gym_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gym_level(&mut self, v: super::POGOProtos_Settings_Master::GymLevelSettings) {
        self.gym_level = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gym_level(&mut self) -> &mut super::POGOProtos_Settings_Master::GymLevelSettings {
        if self.gym_level.is_none() {
            self.gym_level.set_default();
        };
        self.gym_level.as_mut().unwrap()
    }

    // Take field
    pub fn take_gym_level(&mut self) -> super::POGOProtos_Settings_Master::GymLevelSettings {
        self.gym_level.take().unwrap_or_else(|| super::POGOProtos_Settings_Master::GymLevelSettings::new())
    }

    pub fn get_gym_level(&self) -> &super::POGOProtos_Settings_Master::GymLevelSettings {
        self.gym_level.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master::GymLevelSettings::default_instance())
    }

    // optional .POGOProtos.Settings.Master.GymBattleSettings battle_settings = 14;

    pub fn clear_battle_settings(&mut self) {
        self.battle_settings.clear();
    }

    pub fn has_battle_settings(&self) -> bool {
        self.battle_settings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battle_settings(&mut self, v: super::POGOProtos_Settings_Master::GymBattleSettings) {
        self.battle_settings = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_battle_settings(&mut self) -> &mut super::POGOProtos_Settings_Master::GymBattleSettings {
        if self.battle_settings.is_none() {
            self.battle_settings.set_default();
        };
        self.battle_settings.as_mut().unwrap()
    }

    // Take field
    pub fn take_battle_settings(&mut self) -> super::POGOProtos_Settings_Master::GymBattleSettings {
        self.battle_settings.take().unwrap_or_else(|| super::POGOProtos_Settings_Master::GymBattleSettings::new())
    }

    pub fn get_battle_settings(&self) -> &super::POGOProtos_Settings_Master::GymBattleSettings {
        self.battle_settings.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master::GymBattleSettings::default_instance())
    }

    // optional .POGOProtos.Settings.Master.EncounterSettings encounter_settings = 15;

    pub fn clear_encounter_settings(&mut self) {
        self.encounter_settings.clear();
    }

    pub fn has_encounter_settings(&self) -> bool {
        self.encounter_settings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encounter_settings(&mut self, v: super::POGOProtos_Settings_Master::EncounterSettings) {
        self.encounter_settings = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encounter_settings(&mut self) -> &mut super::POGOProtos_Settings_Master::EncounterSettings {
        if self.encounter_settings.is_none() {
            self.encounter_settings.set_default();
        };
        self.encounter_settings.as_mut().unwrap()
    }

    // Take field
    pub fn take_encounter_settings(&mut self) -> super::POGOProtos_Settings_Master::EncounterSettings {
        self.encounter_settings.take().unwrap_or_else(|| super::POGOProtos_Settings_Master::EncounterSettings::new())
    }

    pub fn get_encounter_settings(&self) -> &super::POGOProtos_Settings_Master::EncounterSettings {
        self.encounter_settings.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master::EncounterSettings::default_instance())
    }

    // optional .POGOProtos.Settings.Master.IapItemDisplay iap_item_display = 16;

    pub fn clear_iap_item_display(&mut self) {
        self.iap_item_display.clear();
    }

    pub fn has_iap_item_display(&self) -> bool {
        self.iap_item_display.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iap_item_display(&mut self, v: super::POGOProtos_Settings_Master::IapItemDisplay) {
        self.iap_item_display = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_iap_item_display(&mut self) -> &mut super::POGOProtos_Settings_Master::IapItemDisplay {
        if self.iap_item_display.is_none() {
            self.iap_item_display.set_default();
        };
        self.iap_item_display.as_mut().unwrap()
    }

    // Take field
    pub fn take_iap_item_display(&mut self) -> super::POGOProtos_Settings_Master::IapItemDisplay {
        self.iap_item_display.take().unwrap_or_else(|| super::POGOProtos_Settings_Master::IapItemDisplay::new())
    }

    pub fn get_iap_item_display(&self) -> &super::POGOProtos_Settings_Master::IapItemDisplay {
        self.iap_item_display.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master::IapItemDisplay::default_instance())
    }

    // optional .POGOProtos.Settings.Master.IapSettings iap_settings = 17;

    pub fn clear_iap_settings(&mut self) {
        self.iap_settings.clear();
    }

    pub fn has_iap_settings(&self) -> bool {
        self.iap_settings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iap_settings(&mut self, v: super::POGOProtos_Settings_Master::IapSettings) {
        self.iap_settings = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_iap_settings(&mut self) -> &mut super::POGOProtos_Settings_Master::IapSettings {
        if self.iap_settings.is_none() {
            self.iap_settings.set_default();
        };
        self.iap_settings.as_mut().unwrap()
    }

    // Take field
    pub fn take_iap_settings(&mut self) -> super::POGOProtos_Settings_Master::IapSettings {
        self.iap_settings.take().unwrap_or_else(|| super::POGOProtos_Settings_Master::IapSettings::new())
    }

    pub fn get_iap_settings(&self) -> &super::POGOProtos_Settings_Master::IapSettings {
        self.iap_settings.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master::IapSettings::default_instance())
    }

    // optional .POGOProtos.Settings.Master.PokemonUpgradeSettings pokemon_upgrades = 18;

    pub fn clear_pokemon_upgrades(&mut self) {
        self.pokemon_upgrades.clear();
    }

    pub fn has_pokemon_upgrades(&self) -> bool {
        self.pokemon_upgrades.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_upgrades(&mut self, v: super::POGOProtos_Settings_Master::PokemonUpgradeSettings) {
        self.pokemon_upgrades = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pokemon_upgrades(&mut self) -> &mut super::POGOProtos_Settings_Master::PokemonUpgradeSettings {
        if self.pokemon_upgrades.is_none() {
            self.pokemon_upgrades.set_default();
        };
        self.pokemon_upgrades.as_mut().unwrap()
    }

    // Take field
    pub fn take_pokemon_upgrades(&mut self) -> super::POGOProtos_Settings_Master::PokemonUpgradeSettings {
        self.pokemon_upgrades.take().unwrap_or_else(|| super::POGOProtos_Settings_Master::PokemonUpgradeSettings::new())
    }

    pub fn get_pokemon_upgrades(&self) -> &super::POGOProtos_Settings_Master::PokemonUpgradeSettings {
        self.pokemon_upgrades.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master::PokemonUpgradeSettings::default_instance())
    }

    // optional .POGOProtos.Settings.Master.EquippedBadgeSettings equipped_badges = 19;

    pub fn clear_equipped_badges(&mut self) {
        self.equipped_badges.clear();
    }

    pub fn has_equipped_badges(&self) -> bool {
        self.equipped_badges.is_some()
    }

    // Param is passed by value, moved
    pub fn set_equipped_badges(&mut self, v: super::POGOProtos_Settings_Master::EquippedBadgeSettings) {
        self.equipped_badges = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_equipped_badges(&mut self) -> &mut super::POGOProtos_Settings_Master::EquippedBadgeSettings {
        if self.equipped_badges.is_none() {
            self.equipped_badges.set_default();
        };
        self.equipped_badges.as_mut().unwrap()
    }

    // Take field
    pub fn take_equipped_badges(&mut self) -> super::POGOProtos_Settings_Master::EquippedBadgeSettings {
        self.equipped_badges.take().unwrap_or_else(|| super::POGOProtos_Settings_Master::EquippedBadgeSettings::new())
    }

    pub fn get_equipped_badges(&self) -> &super::POGOProtos_Settings_Master::EquippedBadgeSettings {
        self.equipped_badges.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master::EquippedBadgeSettings::default_instance())
    }
}

impl ::protobuf::Message for DownloadItemTemplatesResponse_ItemTemplate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.template_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pokemon_settings));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.item_settings));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.move_settings));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.move_sequence_settings));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.type_effective));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.badge_settings));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.camera));
                },
                12 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_level));
                },
                13 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.gym_level));
                },
                14 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.battle_settings));
                },
                15 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.encounter_settings));
                },
                16 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.iap_item_display));
                },
                17 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.iap_settings));
                },
                18 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pokemon_upgrades));
                },
                19 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.equipped_badges));
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
        for value in &self.template_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.pokemon_settings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.item_settings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.move_settings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.move_sequence_settings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.type_effective {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.badge_settings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.camera {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.player_level {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.gym_level {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.battle_settings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.encounter_settings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.iap_item_display {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.iap_settings {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.pokemon_upgrades {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.equipped_badges {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.template_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.pokemon_settings.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.item_settings.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.move_settings.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.move_sequence_settings.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.type_effective.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.badge_settings.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.camera.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.player_level.as_ref() {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.gym_level.as_ref() {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.battle_settings.as_ref() {
            try!(os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.encounter_settings.as_ref() {
            try!(os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.iap_item_display.as_ref() {
            try!(os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.iap_settings.as_ref() {
            try!(os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pokemon_upgrades.as_ref() {
            try!(os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.equipped_badges.as_ref() {
            try!(os.write_tag(19, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<DownloadItemTemplatesResponse_ItemTemplate>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DownloadItemTemplatesResponse_ItemTemplate {
    fn new() -> DownloadItemTemplatesResponse_ItemTemplate {
        DownloadItemTemplatesResponse_ItemTemplate::new()
    }

    fn descriptor_static(_: ::std::option::Option<DownloadItemTemplatesResponse_ItemTemplate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "template_id",
                    DownloadItemTemplatesResponse_ItemTemplate::has_template_id,
                    DownloadItemTemplatesResponse_ItemTemplate::get_template_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pokemon_settings",
                    DownloadItemTemplatesResponse_ItemTemplate::has_pokemon_settings,
                    DownloadItemTemplatesResponse_ItemTemplate::get_pokemon_settings,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "item_settings",
                    DownloadItemTemplatesResponse_ItemTemplate::has_item_settings,
                    DownloadItemTemplatesResponse_ItemTemplate::get_item_settings,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "move_settings",
                    DownloadItemTemplatesResponse_ItemTemplate::has_move_settings,
                    DownloadItemTemplatesResponse_ItemTemplate::get_move_settings,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "move_sequence_settings",
                    DownloadItemTemplatesResponse_ItemTemplate::has_move_sequence_settings,
                    DownloadItemTemplatesResponse_ItemTemplate::get_move_sequence_settings,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "type_effective",
                    DownloadItemTemplatesResponse_ItemTemplate::has_type_effective,
                    DownloadItemTemplatesResponse_ItemTemplate::get_type_effective,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "badge_settings",
                    DownloadItemTemplatesResponse_ItemTemplate::has_badge_settings,
                    DownloadItemTemplatesResponse_ItemTemplate::get_badge_settings,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "camera",
                    DownloadItemTemplatesResponse_ItemTemplate::has_camera,
                    DownloadItemTemplatesResponse_ItemTemplate::get_camera,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "player_level",
                    DownloadItemTemplatesResponse_ItemTemplate::has_player_level,
                    DownloadItemTemplatesResponse_ItemTemplate::get_player_level,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "gym_level",
                    DownloadItemTemplatesResponse_ItemTemplate::has_gym_level,
                    DownloadItemTemplatesResponse_ItemTemplate::get_gym_level,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "battle_settings",
                    DownloadItemTemplatesResponse_ItemTemplate::has_battle_settings,
                    DownloadItemTemplatesResponse_ItemTemplate::get_battle_settings,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "encounter_settings",
                    DownloadItemTemplatesResponse_ItemTemplate::has_encounter_settings,
                    DownloadItemTemplatesResponse_ItemTemplate::get_encounter_settings,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "iap_item_display",
                    DownloadItemTemplatesResponse_ItemTemplate::has_iap_item_display,
                    DownloadItemTemplatesResponse_ItemTemplate::get_iap_item_display,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "iap_settings",
                    DownloadItemTemplatesResponse_ItemTemplate::has_iap_settings,
                    DownloadItemTemplatesResponse_ItemTemplate::get_iap_settings,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pokemon_upgrades",
                    DownloadItemTemplatesResponse_ItemTemplate::has_pokemon_upgrades,
                    DownloadItemTemplatesResponse_ItemTemplate::get_pokemon_upgrades,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "equipped_badges",
                    DownloadItemTemplatesResponse_ItemTemplate::has_equipped_badges,
                    DownloadItemTemplatesResponse_ItemTemplate::get_equipped_badges,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DownloadItemTemplatesResponse_ItemTemplate>(
                    "DownloadItemTemplatesResponse_ItemTemplate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DownloadItemTemplatesResponse_ItemTemplate {
    fn clear(&mut self) {
        self.clear_template_id();
        self.clear_pokemon_settings();
        self.clear_item_settings();
        self.clear_move_settings();
        self.clear_move_sequence_settings();
        self.clear_type_effective();
        self.clear_badge_settings();
        self.clear_camera();
        self.clear_player_level();
        self.clear_gym_level();
        self.clear_battle_settings();
        self.clear_encounter_settings();
        self.clear_iap_item_display();
        self.clear_iap_settings();
        self.clear_pokemon_upgrades();
        self.clear_equipped_badges();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DownloadItemTemplatesResponse_ItemTemplate {
    fn eq(&self, other: &DownloadItemTemplatesResponse_ItemTemplate) -> bool {
        self.template_id == other.template_id &&
        self.pokemon_settings == other.pokemon_settings &&
        self.item_settings == other.item_settings &&
        self.move_settings == other.move_settings &&
        self.move_sequence_settings == other.move_sequence_settings &&
        self.type_effective == other.type_effective &&
        self.badge_settings == other.badge_settings &&
        self.camera == other.camera &&
        self.player_level == other.player_level &&
        self.gym_level == other.gym_level &&
        self.battle_settings == other.battle_settings &&
        self.encounter_settings == other.encounter_settings &&
        self.iap_item_display == other.iap_item_display &&
        self.iap_settings == other.iap_settings &&
        self.pokemon_upgrades == other.pokemon_upgrades &&
        self.equipped_badges == other.equipped_badges &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DownloadItemTemplatesResponse_ItemTemplate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AddFortModifierResponse {
    // message fields
    result: ::std::option::Option<AddFortModifierResponse_Result>,
    fort_details: ::protobuf::SingularPtrField<FortDetailsResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddFortModifierResponse {}

impl AddFortModifierResponse {
    pub fn new() -> AddFortModifierResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddFortModifierResponse {
        static mut instance: ::protobuf::lazy::Lazy<AddFortModifierResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddFortModifierResponse,
        };
        unsafe {
            instance.get(|| {
                AddFortModifierResponse {
                    result: ::std::option::Option::None,
                    fort_details: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.AddFortModifierResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: AddFortModifierResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> AddFortModifierResponse_Result {
        self.result.unwrap_or(AddFortModifierResponse_Result::NO_RESULT_SET)
    }

    // optional .POGOProtos.Networking.Responses.FortDetailsResponse fort_details = 2;

    pub fn clear_fort_details(&mut self) {
        self.fort_details.clear();
    }

    pub fn has_fort_details(&self) -> bool {
        self.fort_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fort_details(&mut self, v: FortDetailsResponse) {
        self.fort_details = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fort_details(&mut self) -> &mut FortDetailsResponse {
        if self.fort_details.is_none() {
            self.fort_details.set_default();
        };
        self.fort_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_fort_details(&mut self) -> FortDetailsResponse {
        self.fort_details.take().unwrap_or_else(|| FortDetailsResponse::new())
    }

    pub fn get_fort_details(&self) -> &FortDetailsResponse {
        self.fort_details.as_ref().unwrap_or_else(|| FortDetailsResponse::default_instance())
    }
}

impl ::protobuf::Message for AddFortModifierResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fort_details));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.fort_details {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.fort_details.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<AddFortModifierResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AddFortModifierResponse {
    fn new() -> AddFortModifierResponse {
        AddFortModifierResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddFortModifierResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    AddFortModifierResponse::has_result,
                    AddFortModifierResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fort_details",
                    AddFortModifierResponse::has_fort_details,
                    AddFortModifierResponse::get_fort_details,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddFortModifierResponse>(
                    "AddFortModifierResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddFortModifierResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_fort_details();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AddFortModifierResponse {
    fn eq(&self, other: &AddFortModifierResponse) -> bool {
        self.result == other.result &&
        self.fort_details == other.fort_details &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AddFortModifierResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AddFortModifierResponse_Result {
    NO_RESULT_SET = 0,
    SUCCESS = 1,
    FORT_ALREADY_HAS_MODIFIER = 2,
    TOO_FAR_AWAY = 3,
    NO_ITEM_IN_INVENTORY = 4,
}

impl ::protobuf::ProtobufEnum for AddFortModifierResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AddFortModifierResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(AddFortModifierResponse_Result::NO_RESULT_SET),
            1 => ::std::option::Option::Some(AddFortModifierResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(AddFortModifierResponse_Result::FORT_ALREADY_HAS_MODIFIER),
            3 => ::std::option::Option::Some(AddFortModifierResponse_Result::TOO_FAR_AWAY),
            4 => ::std::option::Option::Some(AddFortModifierResponse_Result::NO_ITEM_IN_INVENTORY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AddFortModifierResponse_Result] = &[
            AddFortModifierResponse_Result::NO_RESULT_SET,
            AddFortModifierResponse_Result::SUCCESS,
            AddFortModifierResponse_Result::FORT_ALREADY_HAS_MODIFIER,
            AddFortModifierResponse_Result::TOO_FAR_AWAY,
            AddFortModifierResponse_Result::NO_ITEM_IN_INVENTORY,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<AddFortModifierResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AddFortModifierResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AddFortModifierResponse_Result {
}

#[derive(Clone,Default)]
pub struct EncounterTutorialCompleteResponse {
    // message fields
    result: ::std::option::Option<EncounterTutorialCompleteResponse_Result>,
    pokemon_data: ::protobuf::SingularPtrField<super::POGOProtos_Data::PokemonData>,
    capture_award: ::protobuf::SingularPtrField<super::POGOProtos_Data_Capture::CaptureAward>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EncounterTutorialCompleteResponse {}

impl EncounterTutorialCompleteResponse {
    pub fn new() -> EncounterTutorialCompleteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EncounterTutorialCompleteResponse {
        static mut instance: ::protobuf::lazy::Lazy<EncounterTutorialCompleteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EncounterTutorialCompleteResponse,
        };
        unsafe {
            instance.get(|| {
                EncounterTutorialCompleteResponse {
                    result: ::std::option::Option::None,
                    pokemon_data: ::protobuf::SingularPtrField::none(),
                    capture_award: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.EncounterTutorialCompleteResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: EncounterTutorialCompleteResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> EncounterTutorialCompleteResponse_Result {
        self.result.unwrap_or(EncounterTutorialCompleteResponse_Result::UNSET)
    }

    // optional .POGOProtos.Data.PokemonData pokemon_data = 2;

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

    // optional .POGOProtos.Data.Capture.CaptureAward capture_award = 3;

    pub fn clear_capture_award(&mut self) {
        self.capture_award.clear();
    }

    pub fn has_capture_award(&self) -> bool {
        self.capture_award.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capture_award(&mut self, v: super::POGOProtos_Data_Capture::CaptureAward) {
        self.capture_award = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_capture_award(&mut self) -> &mut super::POGOProtos_Data_Capture::CaptureAward {
        if self.capture_award.is_none() {
            self.capture_award.set_default();
        };
        self.capture_award.as_mut().unwrap()
    }

    // Take field
    pub fn take_capture_award(&mut self) -> super::POGOProtos_Data_Capture::CaptureAward {
        self.capture_award.take().unwrap_or_else(|| super::POGOProtos_Data_Capture::CaptureAward::new())
    }

    pub fn get_capture_award(&self) -> &super::POGOProtos_Data_Capture::CaptureAward {
        self.capture_award.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Capture::CaptureAward::default_instance())
    }
}

impl ::protobuf::Message for EncounterTutorialCompleteResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pokemon_data));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.capture_award));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.pokemon_data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.capture_award {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.pokemon_data.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.capture_award.as_ref() {
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
        ::std::any::TypeId::of::<EncounterTutorialCompleteResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EncounterTutorialCompleteResponse {
    fn new() -> EncounterTutorialCompleteResponse {
        EncounterTutorialCompleteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<EncounterTutorialCompleteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    EncounterTutorialCompleteResponse::has_result,
                    EncounterTutorialCompleteResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pokemon_data",
                    EncounterTutorialCompleteResponse::has_pokemon_data,
                    EncounterTutorialCompleteResponse::get_pokemon_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "capture_award",
                    EncounterTutorialCompleteResponse::has_capture_award,
                    EncounterTutorialCompleteResponse::get_capture_award,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EncounterTutorialCompleteResponse>(
                    "EncounterTutorialCompleteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EncounterTutorialCompleteResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_pokemon_data();
        self.clear_capture_award();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EncounterTutorialCompleteResponse {
    fn eq(&self, other: &EncounterTutorialCompleteResponse) -> bool {
        self.result == other.result &&
        self.pokemon_data == other.pokemon_data &&
        self.capture_award == other.capture_award &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EncounterTutorialCompleteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EncounterTutorialCompleteResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    ERROR_INVALID_POKEMON = 2,
}

impl ::protobuf::ProtobufEnum for EncounterTutorialCompleteResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EncounterTutorialCompleteResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(EncounterTutorialCompleteResponse_Result::UNSET),
            1 => ::std::option::Option::Some(EncounterTutorialCompleteResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(EncounterTutorialCompleteResponse_Result::ERROR_INVALID_POKEMON),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EncounterTutorialCompleteResponse_Result] = &[
            EncounterTutorialCompleteResponse_Result::UNSET,
            EncounterTutorialCompleteResponse_Result::SUCCESS,
            EncounterTutorialCompleteResponse_Result::ERROR_INVALID_POKEMON,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EncounterTutorialCompleteResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EncounterTutorialCompleteResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EncounterTutorialCompleteResponse_Result {
}

#[derive(Clone,Default)]
pub struct GetInventoryResponse {
    // message fields
    success: ::std::option::Option<bool>,
    inventory_delta: ::protobuf::SingularPtrField<super::POGOProtos_Inventory::InventoryDelta>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetInventoryResponse {}

impl GetInventoryResponse {
    pub fn new() -> GetInventoryResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetInventoryResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetInventoryResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetInventoryResponse,
        };
        unsafe {
            instance.get(|| {
                GetInventoryResponse {
                    success: ::std::option::Option::None,
                    inventory_delta: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // optional .POGOProtos.Inventory.InventoryDelta inventory_delta = 2;

    pub fn clear_inventory_delta(&mut self) {
        self.inventory_delta.clear();
    }

    pub fn has_inventory_delta(&self) -> bool {
        self.inventory_delta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inventory_delta(&mut self, v: super::POGOProtos_Inventory::InventoryDelta) {
        self.inventory_delta = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inventory_delta(&mut self) -> &mut super::POGOProtos_Inventory::InventoryDelta {
        if self.inventory_delta.is_none() {
            self.inventory_delta.set_default();
        };
        self.inventory_delta.as_mut().unwrap()
    }

    // Take field
    pub fn take_inventory_delta(&mut self) -> super::POGOProtos_Inventory::InventoryDelta {
        self.inventory_delta.take().unwrap_or_else(|| super::POGOProtos_Inventory::InventoryDelta::new())
    }

    pub fn get_inventory_delta(&self) -> &super::POGOProtos_Inventory::InventoryDelta {
        self.inventory_delta.as_ref().unwrap_or_else(|| super::POGOProtos_Inventory::InventoryDelta::default_instance())
    }
}

impl ::protobuf::Message for GetInventoryResponse {
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
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.inventory_delta));
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
        if self.success.is_some() {
            my_size += 2;
        };
        for value in &self.inventory_delta {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.inventory_delta.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<GetInventoryResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetInventoryResponse {
    fn new() -> GetInventoryResponse {
        GetInventoryResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetInventoryResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    GetInventoryResponse::has_success,
                    GetInventoryResponse::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "inventory_delta",
                    GetInventoryResponse::has_inventory_delta,
                    GetInventoryResponse::get_inventory_delta,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetInventoryResponse>(
                    "GetInventoryResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetInventoryResponse {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_inventory_delta();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetInventoryResponse {
    fn eq(&self, other: &GetInventoryResponse) -> bool {
        self.success == other.success &&
        self.inventory_delta == other.inventory_delta &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetInventoryResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetMapObjectsResponse {
    // message fields
    map_cells: ::protobuf::RepeatedField<super::POGOProtos_Map::MapCell>,
    status: ::std::option::Option<super::POGOProtos_Map::MapObjectsStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetMapObjectsResponse {}

impl GetMapObjectsResponse {
    pub fn new() -> GetMapObjectsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetMapObjectsResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetMapObjectsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetMapObjectsResponse,
        };
        unsafe {
            instance.get(|| {
                GetMapObjectsResponse {
                    map_cells: ::protobuf::RepeatedField::new(),
                    status: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .POGOProtos.Map.MapCell map_cells = 1;

    pub fn clear_map_cells(&mut self) {
        self.map_cells.clear();
    }

    // Param is passed by value, moved
    pub fn set_map_cells(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Map::MapCell>) {
        self.map_cells = v;
    }

    // Mutable pointer to the field.
    pub fn mut_map_cells(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Map::MapCell> {
        &mut self.map_cells
    }

    // Take field
    pub fn take_map_cells(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Map::MapCell> {
        ::std::mem::replace(&mut self.map_cells, ::protobuf::RepeatedField::new())
    }

    pub fn get_map_cells(&self) -> &[super::POGOProtos_Map::MapCell] {
        &self.map_cells
    }

    // optional .POGOProtos.Map.MapObjectsStatus status = 2;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: super::POGOProtos_Map::MapObjectsStatus) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> super::POGOProtos_Map::MapObjectsStatus {
        self.status.unwrap_or(super::POGOProtos_Map::MapObjectsStatus::UNSET_STATUS)
    }
}

impl ::protobuf::Message for GetMapObjectsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.map_cells));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.status = ::std::option::Option::Some(tmp);
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
        for value in &self.map_cells {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.status {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.map_cells {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.status {
            try!(os.write_enum(2, v.value()));
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
        ::std::any::TypeId::of::<GetMapObjectsResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetMapObjectsResponse {
    fn new() -> GetMapObjectsResponse {
        GetMapObjectsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetMapObjectsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "map_cells",
                    GetMapObjectsResponse::get_map_cells,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    GetMapObjectsResponse::has_status,
                    GetMapObjectsResponse::get_status,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetMapObjectsResponse>(
                    "GetMapObjectsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetMapObjectsResponse {
    fn clear(&mut self) {
        self.clear_map_cells();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetMapObjectsResponse {
    fn eq(&self, other: &GetMapObjectsResponse) -> bool {
        self.map_cells == other.map_cells &&
        self.status == other.status &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetMapObjectsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetPlayerTeamResponse {
    // message fields
    status: ::std::option::Option<SetPlayerTeamResponse_Status>,
    player_data: ::protobuf::SingularPtrField<super::POGOProtos_Data::PlayerData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetPlayerTeamResponse {}

impl SetPlayerTeamResponse {
    pub fn new() -> SetPlayerTeamResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetPlayerTeamResponse {
        static mut instance: ::protobuf::lazy::Lazy<SetPlayerTeamResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetPlayerTeamResponse,
        };
        unsafe {
            instance.get(|| {
                SetPlayerTeamResponse {
                    status: ::std::option::Option::None,
                    player_data: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.SetPlayerTeamResponse.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: SetPlayerTeamResponse_Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> SetPlayerTeamResponse_Status {
        self.status.unwrap_or(SetPlayerTeamResponse_Status::UNSET)
    }

    // optional .POGOProtos.Data.PlayerData player_data = 2;

    pub fn clear_player_data(&mut self) {
        self.player_data.clear();
    }

    pub fn has_player_data(&self) -> bool {
        self.player_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_data(&mut self, v: super::POGOProtos_Data::PlayerData) {
        self.player_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_data(&mut self) -> &mut super::POGOProtos_Data::PlayerData {
        if self.player_data.is_none() {
            self.player_data.set_default();
        };
        self.player_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_data(&mut self) -> super::POGOProtos_Data::PlayerData {
        self.player_data.take().unwrap_or_else(|| super::POGOProtos_Data::PlayerData::new())
    }

    pub fn get_player_data(&self) -> &super::POGOProtos_Data::PlayerData {
        self.player_data.as_ref().unwrap_or_else(|| super::POGOProtos_Data::PlayerData::default_instance())
    }
}

impl ::protobuf::Message for SetPlayerTeamResponse {
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
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_data));
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
        for value in &self.status {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.player_data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.player_data.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<SetPlayerTeamResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetPlayerTeamResponse {
    fn new() -> SetPlayerTeamResponse {
        SetPlayerTeamResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetPlayerTeamResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    SetPlayerTeamResponse::has_status,
                    SetPlayerTeamResponse::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "player_data",
                    SetPlayerTeamResponse::has_player_data,
                    SetPlayerTeamResponse::get_player_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetPlayerTeamResponse>(
                    "SetPlayerTeamResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetPlayerTeamResponse {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_player_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetPlayerTeamResponse {
    fn eq(&self, other: &SetPlayerTeamResponse) -> bool {
        self.status == other.status &&
        self.player_data == other.player_data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetPlayerTeamResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SetPlayerTeamResponse_Status {
    UNSET = 0,
    SUCCESS = 1,
    TEAM_ALREADY_SET = 2,
    FAILURE = 3,
}

impl ::protobuf::ProtobufEnum for SetPlayerTeamResponse_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SetPlayerTeamResponse_Status> {
        match value {
            0 => ::std::option::Option::Some(SetPlayerTeamResponse_Status::UNSET),
            1 => ::std::option::Option::Some(SetPlayerTeamResponse_Status::SUCCESS),
            2 => ::std::option::Option::Some(SetPlayerTeamResponse_Status::TEAM_ALREADY_SET),
            3 => ::std::option::Option::Some(SetPlayerTeamResponse_Status::FAILURE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SetPlayerTeamResponse_Status] = &[
            SetPlayerTeamResponse_Status::UNSET,
            SetPlayerTeamResponse_Status::SUCCESS,
            SetPlayerTeamResponse_Status::TEAM_ALREADY_SET,
            SetPlayerTeamResponse_Status::FAILURE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<SetPlayerTeamResponse_Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SetPlayerTeamResponse_Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SetPlayerTeamResponse_Status {
}

#[derive(Clone,Default)]
pub struct GetIncensePokemonResponse {
    // message fields
    result: ::std::option::Option<GetIncensePokemonResponse_Result>,
    pokemon_id: ::std::option::Option<super::POGOProtos_Enums::PokemonId>,
    latitude: ::std::option::Option<f64>,
    longitude: ::std::option::Option<f64>,
    encounter_location: ::protobuf::SingularField<::std::string::String>,
    encounter_id: ::std::option::Option<u64>,
    disappear_timestamp_ms: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetIncensePokemonResponse {}

impl GetIncensePokemonResponse {
    pub fn new() -> GetIncensePokemonResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetIncensePokemonResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetIncensePokemonResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetIncensePokemonResponse,
        };
        unsafe {
            instance.get(|| {
                GetIncensePokemonResponse {
                    result: ::std::option::Option::None,
                    pokemon_id: ::std::option::Option::None,
                    latitude: ::std::option::Option::None,
                    longitude: ::std::option::Option::None,
                    encounter_location: ::protobuf::SingularField::none(),
                    encounter_id: ::std::option::Option::None,
                    disappear_timestamp_ms: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.GetIncensePokemonResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: GetIncensePokemonResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> GetIncensePokemonResponse_Result {
        self.result.unwrap_or(GetIncensePokemonResponse_Result::INCENSE_ENCOUNTER_UNKNOWN)
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

    // optional string encounter_location = 5;

    pub fn clear_encounter_location(&mut self) {
        self.encounter_location.clear();
    }

    pub fn has_encounter_location(&self) -> bool {
        self.encounter_location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encounter_location(&mut self, v: ::std::string::String) {
        self.encounter_location = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encounter_location(&mut self) -> &mut ::std::string::String {
        if self.encounter_location.is_none() {
            self.encounter_location.set_default();
        };
        self.encounter_location.as_mut().unwrap()
    }

    // Take field
    pub fn take_encounter_location(&mut self) -> ::std::string::String {
        self.encounter_location.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_encounter_location(&self) -> &str {
        match self.encounter_location.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional fixed64 encounter_id = 6;

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

    // optional int64 disappear_timestamp_ms = 7;

    pub fn clear_disappear_timestamp_ms(&mut self) {
        self.disappear_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_disappear_timestamp_ms(&self) -> bool {
        self.disappear_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_disappear_timestamp_ms(&mut self, v: i64) {
        self.disappear_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_disappear_timestamp_ms(&self) -> i64 {
        self.disappear_timestamp_ms.unwrap_or(0)
    }
}

impl ::protobuf::Message for GetIncensePokemonResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.pokemon_id = ::std::option::Option::Some(tmp);
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.encounter_location));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.encounter_id = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.disappear_timestamp_ms = ::std::option::Option::Some(tmp);
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.pokemon_id {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        if self.latitude.is_some() {
            my_size += 9;
        };
        if self.longitude.is_some() {
            my_size += 9;
        };
        for value in &self.encounter_location {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        if self.encounter_id.is_some() {
            my_size += 9;
        };
        for value in &self.disappear_timestamp_ms {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.pokemon_id {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.latitude {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.longitude {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.encounter_location.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.encounter_id {
            try!(os.write_fixed64(6, v));
        };
        if let Some(v) = self.disappear_timestamp_ms {
            try!(os.write_int64(7, v));
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
        ::std::any::TypeId::of::<GetIncensePokemonResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetIncensePokemonResponse {
    fn new() -> GetIncensePokemonResponse {
        GetIncensePokemonResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetIncensePokemonResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    GetIncensePokemonResponse::has_result,
                    GetIncensePokemonResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "pokemon_id",
                    GetIncensePokemonResponse::has_pokemon_id,
                    GetIncensePokemonResponse::get_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "latitude",
                    GetIncensePokemonResponse::has_latitude,
                    GetIncensePokemonResponse::get_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "longitude",
                    GetIncensePokemonResponse::has_longitude,
                    GetIncensePokemonResponse::get_longitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "encounter_location",
                    GetIncensePokemonResponse::has_encounter_location,
                    GetIncensePokemonResponse::get_encounter_location,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "encounter_id",
                    GetIncensePokemonResponse::has_encounter_id,
                    GetIncensePokemonResponse::get_encounter_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "disappear_timestamp_ms",
                    GetIncensePokemonResponse::has_disappear_timestamp_ms,
                    GetIncensePokemonResponse::get_disappear_timestamp_ms,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetIncensePokemonResponse>(
                    "GetIncensePokemonResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetIncensePokemonResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_pokemon_id();
        self.clear_latitude();
        self.clear_longitude();
        self.clear_encounter_location();
        self.clear_encounter_id();
        self.clear_disappear_timestamp_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetIncensePokemonResponse {
    fn eq(&self, other: &GetIncensePokemonResponse) -> bool {
        self.result == other.result &&
        self.pokemon_id == other.pokemon_id &&
        self.latitude == other.latitude &&
        self.longitude == other.longitude &&
        self.encounter_location == other.encounter_location &&
        self.encounter_id == other.encounter_id &&
        self.disappear_timestamp_ms == other.disappear_timestamp_ms &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetIncensePokemonResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum GetIncensePokemonResponse_Result {
    INCENSE_ENCOUNTER_UNKNOWN = 0,
    INCENSE_ENCOUNTER_AVAILABLE = 1,
    INCENSE_ENCOUNTER_NOT_AVAILABLE = 2,
}

impl ::protobuf::ProtobufEnum for GetIncensePokemonResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<GetIncensePokemonResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(GetIncensePokemonResponse_Result::INCENSE_ENCOUNTER_UNKNOWN),
            1 => ::std::option::Option::Some(GetIncensePokemonResponse_Result::INCENSE_ENCOUNTER_AVAILABLE),
            2 => ::std::option::Option::Some(GetIncensePokemonResponse_Result::INCENSE_ENCOUNTER_NOT_AVAILABLE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [GetIncensePokemonResponse_Result] = &[
            GetIncensePokemonResponse_Result::INCENSE_ENCOUNTER_UNKNOWN,
            GetIncensePokemonResponse_Result::INCENSE_ENCOUNTER_AVAILABLE,
            GetIncensePokemonResponse_Result::INCENSE_ENCOUNTER_NOT_AVAILABLE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<GetIncensePokemonResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("GetIncensePokemonResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for GetIncensePokemonResponse_Result {
}

#[derive(Clone,Default)]
pub struct GetAssetDigestResponse {
    // message fields
    digest: ::protobuf::RepeatedField<super::POGOProtos_Data::AssetDigestEntry>,
    timestamp_ms: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetAssetDigestResponse {}

impl GetAssetDigestResponse {
    pub fn new() -> GetAssetDigestResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetAssetDigestResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetAssetDigestResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetAssetDigestResponse,
        };
        unsafe {
            instance.get(|| {
                GetAssetDigestResponse {
                    digest: ::protobuf::RepeatedField::new(),
                    timestamp_ms: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .POGOProtos.Data.AssetDigestEntry digest = 1;

    pub fn clear_digest(&mut self) {
        self.digest.clear();
    }

    // Param is passed by value, moved
    pub fn set_digest(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Data::AssetDigestEntry>) {
        self.digest = v;
    }

    // Mutable pointer to the field.
    pub fn mut_digest(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Data::AssetDigestEntry> {
        &mut self.digest
    }

    // Take field
    pub fn take_digest(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Data::AssetDigestEntry> {
        ::std::mem::replace(&mut self.digest, ::protobuf::RepeatedField::new())
    }

    pub fn get_digest(&self) -> &[super::POGOProtos_Data::AssetDigestEntry] {
        &self.digest
    }

    // optional uint64 timestamp_ms = 2;

    pub fn clear_timestamp_ms(&mut self) {
        self.timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_timestamp_ms(&self) -> bool {
        self.timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp_ms(&mut self, v: u64) {
        self.timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp_ms(&self) -> u64 {
        self.timestamp_ms.unwrap_or(0)
    }
}

impl ::protobuf::Message for GetAssetDigestResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.digest));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.timestamp_ms = ::std::option::Option::Some(tmp);
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
        for value in &self.digest {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.timestamp_ms {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.digest {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.timestamp_ms {
            try!(os.write_uint64(2, v));
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
        ::std::any::TypeId::of::<GetAssetDigestResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetAssetDigestResponse {
    fn new() -> GetAssetDigestResponse {
        GetAssetDigestResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetAssetDigestResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "digest",
                    GetAssetDigestResponse::get_digest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "timestamp_ms",
                    GetAssetDigestResponse::has_timestamp_ms,
                    GetAssetDigestResponse::get_timestamp_ms,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetAssetDigestResponse>(
                    "GetAssetDigestResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetAssetDigestResponse {
    fn clear(&mut self) {
        self.clear_digest();
        self.clear_timestamp_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetAssetDigestResponse {
    fn eq(&self, other: &GetAssetDigestResponse) -> bool {
        self.digest == other.digest &&
        self.timestamp_ms == other.timestamp_ms &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetAssetDigestResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CheckCodenameAvailableResponse {
    // message fields
    codename: ::protobuf::SingularField<::std::string::String>,
    user_message: ::protobuf::SingularField<::std::string::String>,
    is_assignable: ::std::option::Option<bool>,
    status: ::std::option::Option<CheckCodenameAvailableResponse_Status>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CheckCodenameAvailableResponse {}

impl CheckCodenameAvailableResponse {
    pub fn new() -> CheckCodenameAvailableResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckCodenameAvailableResponse {
        static mut instance: ::protobuf::lazy::Lazy<CheckCodenameAvailableResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckCodenameAvailableResponse,
        };
        unsafe {
            instance.get(|| {
                CheckCodenameAvailableResponse {
                    codename: ::protobuf::SingularField::none(),
                    user_message: ::protobuf::SingularField::none(),
                    is_assignable: ::std::option::Option::None,
                    status: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string codename = 1;

    pub fn clear_codename(&mut self) {
        self.codename.clear();
    }

    pub fn has_codename(&self) -> bool {
        self.codename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_codename(&mut self, v: ::std::string::String) {
        self.codename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_codename(&mut self) -> &mut ::std::string::String {
        if self.codename.is_none() {
            self.codename.set_default();
        };
        self.codename.as_mut().unwrap()
    }

    // Take field
    pub fn take_codename(&mut self) -> ::std::string::String {
        self.codename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_codename(&self) -> &str {
        match self.codename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string user_message = 2;

    pub fn clear_user_message(&mut self) {
        self.user_message.clear();
    }

    pub fn has_user_message(&self) -> bool {
        self.user_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_message(&mut self, v: ::std::string::String) {
        self.user_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user_message(&mut self) -> &mut ::std::string::String {
        if self.user_message.is_none() {
            self.user_message.set_default();
        };
        self.user_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_user_message(&mut self) -> ::std::string::String {
        self.user_message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_user_message(&self) -> &str {
        match self.user_message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool is_assignable = 3;

    pub fn clear_is_assignable(&mut self) {
        self.is_assignable = ::std::option::Option::None;
    }

    pub fn has_is_assignable(&self) -> bool {
        self.is_assignable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_assignable(&mut self, v: bool) {
        self.is_assignable = ::std::option::Option::Some(v);
    }

    pub fn get_is_assignable(&self) -> bool {
        self.is_assignable.unwrap_or(false)
    }

    // optional .POGOProtos.Networking.Responses.CheckCodenameAvailableResponse.Status status = 4;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: CheckCodenameAvailableResponse_Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> CheckCodenameAvailableResponse_Status {
        self.status.unwrap_or(CheckCodenameAvailableResponse_Status::UNSET)
    }
}

impl ::protobuf::Message for CheckCodenameAvailableResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.codename));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.user_message));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_assignable = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.status = ::std::option::Option::Some(tmp);
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
        for value in &self.codename {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.user_message {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.is_assignable.is_some() {
            my_size += 2;
        };
        for value in &self.status {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.codename.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.user_message.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.is_assignable {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.status {
            try!(os.write_enum(4, v.value()));
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
        ::std::any::TypeId::of::<CheckCodenameAvailableResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CheckCodenameAvailableResponse {
    fn new() -> CheckCodenameAvailableResponse {
        CheckCodenameAvailableResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckCodenameAvailableResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "codename",
                    CheckCodenameAvailableResponse::has_codename,
                    CheckCodenameAvailableResponse::get_codename,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "user_message",
                    CheckCodenameAvailableResponse::has_user_message,
                    CheckCodenameAvailableResponse::get_user_message,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_assignable",
                    CheckCodenameAvailableResponse::has_is_assignable,
                    CheckCodenameAvailableResponse::get_is_assignable,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    CheckCodenameAvailableResponse::has_status,
                    CheckCodenameAvailableResponse::get_status,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CheckCodenameAvailableResponse>(
                    "CheckCodenameAvailableResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckCodenameAvailableResponse {
    fn clear(&mut self) {
        self.clear_codename();
        self.clear_user_message();
        self.clear_is_assignable();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CheckCodenameAvailableResponse {
    fn eq(&self, other: &CheckCodenameAvailableResponse) -> bool {
        self.codename == other.codename &&
        self.user_message == other.user_message &&
        self.is_assignable == other.is_assignable &&
        self.status == other.status &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CheckCodenameAvailableResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CheckCodenameAvailableResponse_Status {
    UNSET = 0,
    SUCCESS = 1,
    CODENAME_NOT_AVAILABLE = 2,
    CODENAME_NOT_VALID = 3,
    CURRENT_OWNER = 4,
    CODENAME_CHANGE_NOT_ALLOWED = 5,
}

impl ::protobuf::ProtobufEnum for CheckCodenameAvailableResponse_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CheckCodenameAvailableResponse_Status> {
        match value {
            0 => ::std::option::Option::Some(CheckCodenameAvailableResponse_Status::UNSET),
            1 => ::std::option::Option::Some(CheckCodenameAvailableResponse_Status::SUCCESS),
            2 => ::std::option::Option::Some(CheckCodenameAvailableResponse_Status::CODENAME_NOT_AVAILABLE),
            3 => ::std::option::Option::Some(CheckCodenameAvailableResponse_Status::CODENAME_NOT_VALID),
            4 => ::std::option::Option::Some(CheckCodenameAvailableResponse_Status::CURRENT_OWNER),
            5 => ::std::option::Option::Some(CheckCodenameAvailableResponse_Status::CODENAME_CHANGE_NOT_ALLOWED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CheckCodenameAvailableResponse_Status] = &[
            CheckCodenameAvailableResponse_Status::UNSET,
            CheckCodenameAvailableResponse_Status::SUCCESS,
            CheckCodenameAvailableResponse_Status::CODENAME_NOT_AVAILABLE,
            CheckCodenameAvailableResponse_Status::CODENAME_NOT_VALID,
            CheckCodenameAvailableResponse_Status::CURRENT_OWNER,
            CheckCodenameAvailableResponse_Status::CODENAME_CHANGE_NOT_ALLOWED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CheckCodenameAvailableResponse_Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CheckCodenameAvailableResponse_Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CheckCodenameAvailableResponse_Status {
}

#[derive(Clone,Default)]
pub struct DiskEncounterResponse {
    // message fields
    result: ::std::option::Option<DiskEncounterResponse_Result>,
    pokemon_data: ::protobuf::SingularPtrField<super::POGOProtos_Data::PokemonData>,
    capture_probability: ::protobuf::SingularPtrField<super::POGOProtos_Data_Capture::CaptureProbability>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DiskEncounterResponse {}

impl DiskEncounterResponse {
    pub fn new() -> DiskEncounterResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DiskEncounterResponse {
        static mut instance: ::protobuf::lazy::Lazy<DiskEncounterResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiskEncounterResponse,
        };
        unsafe {
            instance.get(|| {
                DiskEncounterResponse {
                    result: ::std::option::Option::None,
                    pokemon_data: ::protobuf::SingularPtrField::none(),
                    capture_probability: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.DiskEncounterResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: DiskEncounterResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> DiskEncounterResponse_Result {
        self.result.unwrap_or(DiskEncounterResponse_Result::UNKNOWN)
    }

    // optional .POGOProtos.Data.PokemonData pokemon_data = 2;

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

    // optional .POGOProtos.Data.Capture.CaptureProbability capture_probability = 3;

    pub fn clear_capture_probability(&mut self) {
        self.capture_probability.clear();
    }

    pub fn has_capture_probability(&self) -> bool {
        self.capture_probability.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capture_probability(&mut self, v: super::POGOProtos_Data_Capture::CaptureProbability) {
        self.capture_probability = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_capture_probability(&mut self) -> &mut super::POGOProtos_Data_Capture::CaptureProbability {
        if self.capture_probability.is_none() {
            self.capture_probability.set_default();
        };
        self.capture_probability.as_mut().unwrap()
    }

    // Take field
    pub fn take_capture_probability(&mut self) -> super::POGOProtos_Data_Capture::CaptureProbability {
        self.capture_probability.take().unwrap_or_else(|| super::POGOProtos_Data_Capture::CaptureProbability::new())
    }

    pub fn get_capture_probability(&self) -> &super::POGOProtos_Data_Capture::CaptureProbability {
        self.capture_probability.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Capture::CaptureProbability::default_instance())
    }
}

impl ::protobuf::Message for DiskEncounterResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pokemon_data));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.capture_probability));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.pokemon_data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.capture_probability {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.pokemon_data.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.capture_probability.as_ref() {
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
        ::std::any::TypeId::of::<DiskEncounterResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DiskEncounterResponse {
    fn new() -> DiskEncounterResponse {
        DiskEncounterResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DiskEncounterResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    DiskEncounterResponse::has_result,
                    DiskEncounterResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pokemon_data",
                    DiskEncounterResponse::has_pokemon_data,
                    DiskEncounterResponse::get_pokemon_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "capture_probability",
                    DiskEncounterResponse::has_capture_probability,
                    DiskEncounterResponse::get_capture_probability,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DiskEncounterResponse>(
                    "DiskEncounterResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DiskEncounterResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_pokemon_data();
        self.clear_capture_probability();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DiskEncounterResponse {
    fn eq(&self, other: &DiskEncounterResponse) -> bool {
        self.result == other.result &&
        self.pokemon_data == other.pokemon_data &&
        self.capture_probability == other.capture_probability &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DiskEncounterResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DiskEncounterResponse_Result {
    UNKNOWN = 0,
    SUCCESS = 1,
    NOT_AVAILABLE = 2,
    NOT_IN_RANGE = 3,
    ENCOUNTER_ALREADY_FINISHED = 4,
    POKEMON_INVENTORY_FULL = 5,
}

impl ::protobuf::ProtobufEnum for DiskEncounterResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DiskEncounterResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(DiskEncounterResponse_Result::UNKNOWN),
            1 => ::std::option::Option::Some(DiskEncounterResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(DiskEncounterResponse_Result::NOT_AVAILABLE),
            3 => ::std::option::Option::Some(DiskEncounterResponse_Result::NOT_IN_RANGE),
            4 => ::std::option::Option::Some(DiskEncounterResponse_Result::ENCOUNTER_ALREADY_FINISHED),
            5 => ::std::option::Option::Some(DiskEncounterResponse_Result::POKEMON_INVENTORY_FULL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DiskEncounterResponse_Result] = &[
            DiskEncounterResponse_Result::UNKNOWN,
            DiskEncounterResponse_Result::SUCCESS,
            DiskEncounterResponse_Result::NOT_AVAILABLE,
            DiskEncounterResponse_Result::NOT_IN_RANGE,
            DiskEncounterResponse_Result::ENCOUNTER_ALREADY_FINISHED,
            DiskEncounterResponse_Result::POKEMON_INVENTORY_FULL,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<DiskEncounterResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DiskEncounterResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DiskEncounterResponse_Result {
}

#[derive(Clone,Default)]
pub struct UseItemEggIncubatorResponse {
    // message fields
    result: ::std::option::Option<UseItemEggIncubatorResponse_Result>,
    egg_incubator: ::protobuf::SingularPtrField<super::POGOProtos_Inventory::EggIncubator>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UseItemEggIncubatorResponse {}

impl UseItemEggIncubatorResponse {
    pub fn new() -> UseItemEggIncubatorResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UseItemEggIncubatorResponse {
        static mut instance: ::protobuf::lazy::Lazy<UseItemEggIncubatorResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UseItemEggIncubatorResponse,
        };
        unsafe {
            instance.get(|| {
                UseItemEggIncubatorResponse {
                    result: ::std::option::Option::None,
                    egg_incubator: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.UseItemEggIncubatorResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: UseItemEggIncubatorResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> UseItemEggIncubatorResponse_Result {
        self.result.unwrap_or(UseItemEggIncubatorResponse_Result::UNSET)
    }

    // optional .POGOProtos.Inventory.EggIncubator egg_incubator = 2;

    pub fn clear_egg_incubator(&mut self) {
        self.egg_incubator.clear();
    }

    pub fn has_egg_incubator(&self) -> bool {
        self.egg_incubator.is_some()
    }

    // Param is passed by value, moved
    pub fn set_egg_incubator(&mut self, v: super::POGOProtos_Inventory::EggIncubator) {
        self.egg_incubator = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_egg_incubator(&mut self) -> &mut super::POGOProtos_Inventory::EggIncubator {
        if self.egg_incubator.is_none() {
            self.egg_incubator.set_default();
        };
        self.egg_incubator.as_mut().unwrap()
    }

    // Take field
    pub fn take_egg_incubator(&mut self) -> super::POGOProtos_Inventory::EggIncubator {
        self.egg_incubator.take().unwrap_or_else(|| super::POGOProtos_Inventory::EggIncubator::new())
    }

    pub fn get_egg_incubator(&self) -> &super::POGOProtos_Inventory::EggIncubator {
        self.egg_incubator.as_ref().unwrap_or_else(|| super::POGOProtos_Inventory::EggIncubator::default_instance())
    }
}

impl ::protobuf::Message for UseItemEggIncubatorResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.egg_incubator));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.egg_incubator {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.egg_incubator.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<UseItemEggIncubatorResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UseItemEggIncubatorResponse {
    fn new() -> UseItemEggIncubatorResponse {
        UseItemEggIncubatorResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<UseItemEggIncubatorResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    UseItemEggIncubatorResponse::has_result,
                    UseItemEggIncubatorResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "egg_incubator",
                    UseItemEggIncubatorResponse::has_egg_incubator,
                    UseItemEggIncubatorResponse::get_egg_incubator,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UseItemEggIncubatorResponse>(
                    "UseItemEggIncubatorResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UseItemEggIncubatorResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_egg_incubator();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UseItemEggIncubatorResponse {
    fn eq(&self, other: &UseItemEggIncubatorResponse) -> bool {
        self.result == other.result &&
        self.egg_incubator == other.egg_incubator &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UseItemEggIncubatorResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum UseItemEggIncubatorResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    ERROR_INCUBATOR_NOT_FOUND = 2,
    ERROR_POKEMON_EGG_NOT_FOUND = 3,
    ERROR_POKEMON_ID_NOT_EGG = 4,
    ERROR_INCUBATOR_ALREADY_IN_USE = 5,
    ERROR_POKEMON_ALREADY_INCUBATING = 6,
    ERROR_INCUBATOR_NO_USES_REMAINING = 7,
}

impl ::protobuf::ProtobufEnum for UseItemEggIncubatorResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UseItemEggIncubatorResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(UseItemEggIncubatorResponse_Result::UNSET),
            1 => ::std::option::Option::Some(UseItemEggIncubatorResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(UseItemEggIncubatorResponse_Result::ERROR_INCUBATOR_NOT_FOUND),
            3 => ::std::option::Option::Some(UseItemEggIncubatorResponse_Result::ERROR_POKEMON_EGG_NOT_FOUND),
            4 => ::std::option::Option::Some(UseItemEggIncubatorResponse_Result::ERROR_POKEMON_ID_NOT_EGG),
            5 => ::std::option::Option::Some(UseItemEggIncubatorResponse_Result::ERROR_INCUBATOR_ALREADY_IN_USE),
            6 => ::std::option::Option::Some(UseItemEggIncubatorResponse_Result::ERROR_POKEMON_ALREADY_INCUBATING),
            7 => ::std::option::Option::Some(UseItemEggIncubatorResponse_Result::ERROR_INCUBATOR_NO_USES_REMAINING),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [UseItemEggIncubatorResponse_Result] = &[
            UseItemEggIncubatorResponse_Result::UNSET,
            UseItemEggIncubatorResponse_Result::SUCCESS,
            UseItemEggIncubatorResponse_Result::ERROR_INCUBATOR_NOT_FOUND,
            UseItemEggIncubatorResponse_Result::ERROR_POKEMON_EGG_NOT_FOUND,
            UseItemEggIncubatorResponse_Result::ERROR_POKEMON_ID_NOT_EGG,
            UseItemEggIncubatorResponse_Result::ERROR_INCUBATOR_ALREADY_IN_USE,
            UseItemEggIncubatorResponse_Result::ERROR_POKEMON_ALREADY_INCUBATING,
            UseItemEggIncubatorResponse_Result::ERROR_INCUBATOR_NO_USES_REMAINING,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<UseItemEggIncubatorResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("UseItemEggIncubatorResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for UseItemEggIncubatorResponse_Result {
}

#[derive(Clone,Default)]
pub struct NicknamePokemonResponse {
    // message fields
    result: ::std::option::Option<NicknamePokemonResponse_Result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NicknamePokemonResponse {}

impl NicknamePokemonResponse {
    pub fn new() -> NicknamePokemonResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NicknamePokemonResponse {
        static mut instance: ::protobuf::lazy::Lazy<NicknamePokemonResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NicknamePokemonResponse,
        };
        unsafe {
            instance.get(|| {
                NicknamePokemonResponse {
                    result: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.NicknamePokemonResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: NicknamePokemonResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> NicknamePokemonResponse_Result {
        self.result.unwrap_or(NicknamePokemonResponse_Result::UNSET)
    }
}

impl ::protobuf::Message for NicknamePokemonResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
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
        ::std::any::TypeId::of::<NicknamePokemonResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NicknamePokemonResponse {
    fn new() -> NicknamePokemonResponse {
        NicknamePokemonResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<NicknamePokemonResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    NicknamePokemonResponse::has_result,
                    NicknamePokemonResponse::get_result,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NicknamePokemonResponse>(
                    "NicknamePokemonResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NicknamePokemonResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for NicknamePokemonResponse {
    fn eq(&self, other: &NicknamePokemonResponse) -> bool {
        self.result == other.result &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for NicknamePokemonResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NicknamePokemonResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    ERROR_INVALID_NICKNAME = 2,
    ERROR_POKEMON_NOT_FOUND = 3,
    ERROR_POKEMON_IS_EGG = 4,
}

impl ::protobuf::ProtobufEnum for NicknamePokemonResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NicknamePokemonResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(NicknamePokemonResponse_Result::UNSET),
            1 => ::std::option::Option::Some(NicknamePokemonResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(NicknamePokemonResponse_Result::ERROR_INVALID_NICKNAME),
            3 => ::std::option::Option::Some(NicknamePokemonResponse_Result::ERROR_POKEMON_NOT_FOUND),
            4 => ::std::option::Option::Some(NicknamePokemonResponse_Result::ERROR_POKEMON_IS_EGG),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NicknamePokemonResponse_Result] = &[
            NicknamePokemonResponse_Result::UNSET,
            NicknamePokemonResponse_Result::SUCCESS,
            NicknamePokemonResponse_Result::ERROR_INVALID_NICKNAME,
            NicknamePokemonResponse_Result::ERROR_POKEMON_NOT_FOUND,
            NicknamePokemonResponse_Result::ERROR_POKEMON_IS_EGG,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<NicknamePokemonResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NicknamePokemonResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NicknamePokemonResponse_Result {
}

#[derive(Clone,Default)]
pub struct FortRecallPokemonResponse {
    // message fields
    result: ::std::option::Option<FortRecallPokemonResponse_Result>,
    fort_details: ::protobuf::SingularPtrField<FortDetailsResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FortRecallPokemonResponse {}

impl FortRecallPokemonResponse {
    pub fn new() -> FortRecallPokemonResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FortRecallPokemonResponse {
        static mut instance: ::protobuf::lazy::Lazy<FortRecallPokemonResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FortRecallPokemonResponse,
        };
        unsafe {
            instance.get(|| {
                FortRecallPokemonResponse {
                    result: ::std::option::Option::None,
                    fort_details: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.FortRecallPokemonResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: FortRecallPokemonResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> FortRecallPokemonResponse_Result {
        self.result.unwrap_or(FortRecallPokemonResponse_Result::NO_RESULT_SET)
    }

    // optional .POGOProtos.Networking.Responses.FortDetailsResponse fort_details = 2;

    pub fn clear_fort_details(&mut self) {
        self.fort_details.clear();
    }

    pub fn has_fort_details(&self) -> bool {
        self.fort_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fort_details(&mut self, v: FortDetailsResponse) {
        self.fort_details = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fort_details(&mut self) -> &mut FortDetailsResponse {
        if self.fort_details.is_none() {
            self.fort_details.set_default();
        };
        self.fort_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_fort_details(&mut self) -> FortDetailsResponse {
        self.fort_details.take().unwrap_or_else(|| FortDetailsResponse::new())
    }

    pub fn get_fort_details(&self) -> &FortDetailsResponse {
        self.fort_details.as_ref().unwrap_or_else(|| FortDetailsResponse::default_instance())
    }
}

impl ::protobuf::Message for FortRecallPokemonResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fort_details));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.fort_details {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.fort_details.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<FortRecallPokemonResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FortRecallPokemonResponse {
    fn new() -> FortRecallPokemonResponse {
        FortRecallPokemonResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<FortRecallPokemonResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    FortRecallPokemonResponse::has_result,
                    FortRecallPokemonResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fort_details",
                    FortRecallPokemonResponse::has_fort_details,
                    FortRecallPokemonResponse::get_fort_details,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FortRecallPokemonResponse>(
                    "FortRecallPokemonResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FortRecallPokemonResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_fort_details();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FortRecallPokemonResponse {
    fn eq(&self, other: &FortRecallPokemonResponse) -> bool {
        self.result == other.result &&
        self.fort_details == other.fort_details &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FortRecallPokemonResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FortRecallPokemonResponse_Result {
    NO_RESULT_SET = 0,
    SUCCESS = 1,
    ERROR_NOT_IN_RANGE = 2,
    ERROR_POKEMON_NOT_ON_FORT = 3,
    ERROR_NO_PLAYER = 4,
}

impl ::protobuf::ProtobufEnum for FortRecallPokemonResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FortRecallPokemonResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(FortRecallPokemonResponse_Result::NO_RESULT_SET),
            1 => ::std::option::Option::Some(FortRecallPokemonResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(FortRecallPokemonResponse_Result::ERROR_NOT_IN_RANGE),
            3 => ::std::option::Option::Some(FortRecallPokemonResponse_Result::ERROR_POKEMON_NOT_ON_FORT),
            4 => ::std::option::Option::Some(FortRecallPokemonResponse_Result::ERROR_NO_PLAYER),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FortRecallPokemonResponse_Result] = &[
            FortRecallPokemonResponse_Result::NO_RESULT_SET,
            FortRecallPokemonResponse_Result::SUCCESS,
            FortRecallPokemonResponse_Result::ERROR_NOT_IN_RANGE,
            FortRecallPokemonResponse_Result::ERROR_POKEMON_NOT_ON_FORT,
            FortRecallPokemonResponse_Result::ERROR_NO_PLAYER,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<FortRecallPokemonResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("FortRecallPokemonResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for FortRecallPokemonResponse_Result {
}

#[derive(Clone,Default)]
pub struct LevelUpRewardsResponse {
    // message fields
    result: ::std::option::Option<LevelUpRewardsResponse_Result>,
    items_awarded: ::protobuf::RepeatedField<super::POGOProtos_Inventory_Item::ItemAward>,
    items_unlocked: ::std::vec::Vec<super::POGOProtos_Inventory_Item::ItemId>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LevelUpRewardsResponse {}

impl LevelUpRewardsResponse {
    pub fn new() -> LevelUpRewardsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LevelUpRewardsResponse {
        static mut instance: ::protobuf::lazy::Lazy<LevelUpRewardsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LevelUpRewardsResponse,
        };
        unsafe {
            instance.get(|| {
                LevelUpRewardsResponse {
                    result: ::std::option::Option::None,
                    items_awarded: ::protobuf::RepeatedField::new(),
                    items_unlocked: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.LevelUpRewardsResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: LevelUpRewardsResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> LevelUpRewardsResponse_Result {
        self.result.unwrap_or(LevelUpRewardsResponse_Result::UNSET)
    }

    // repeated .POGOProtos.Inventory.Item.ItemAward items_awarded = 2;

    pub fn clear_items_awarded(&mut self) {
        self.items_awarded.clear();
    }

    // Param is passed by value, moved
    pub fn set_items_awarded(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Inventory_Item::ItemAward>) {
        self.items_awarded = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items_awarded(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Inventory_Item::ItemAward> {
        &mut self.items_awarded
    }

    // Take field
    pub fn take_items_awarded(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Inventory_Item::ItemAward> {
        ::std::mem::replace(&mut self.items_awarded, ::protobuf::RepeatedField::new())
    }

    pub fn get_items_awarded(&self) -> &[super::POGOProtos_Inventory_Item::ItemAward] {
        &self.items_awarded
    }

    // repeated .POGOProtos.Inventory.Item.ItemId items_unlocked = 4;

    pub fn clear_items_unlocked(&mut self) {
        self.items_unlocked.clear();
    }

    // Param is passed by value, moved
    pub fn set_items_unlocked(&mut self, v: ::std::vec::Vec<super::POGOProtos_Inventory_Item::ItemId>) {
        self.items_unlocked = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items_unlocked(&mut self) -> &mut ::std::vec::Vec<super::POGOProtos_Inventory_Item::ItemId> {
        &mut self.items_unlocked
    }

    // Take field
    pub fn take_items_unlocked(&mut self) -> ::std::vec::Vec<super::POGOProtos_Inventory_Item::ItemId> {
        ::std::mem::replace(&mut self.items_unlocked, ::std::vec::Vec::new())
    }

    pub fn get_items_unlocked(&self) -> &[super::POGOProtos_Inventory_Item::ItemId] {
        &self.items_unlocked
    }
}

impl ::protobuf::Message for LevelUpRewardsResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items_awarded));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.items_unlocked));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.items_awarded {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.items_unlocked {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        for v in &self.items_awarded {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.items_unlocked {
            try!(os.write_enum(4, v.value()));
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
        ::std::any::TypeId::of::<LevelUpRewardsResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LevelUpRewardsResponse {
    fn new() -> LevelUpRewardsResponse {
        LevelUpRewardsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<LevelUpRewardsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    LevelUpRewardsResponse::has_result,
                    LevelUpRewardsResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "items_awarded",
                    LevelUpRewardsResponse::get_items_awarded,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "items_unlocked",
                    LevelUpRewardsResponse::get_items_unlocked,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LevelUpRewardsResponse>(
                    "LevelUpRewardsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LevelUpRewardsResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_items_awarded();
        self.clear_items_unlocked();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LevelUpRewardsResponse {
    fn eq(&self, other: &LevelUpRewardsResponse) -> bool {
        self.result == other.result &&
        self.items_awarded == other.items_awarded &&
        self.items_unlocked == other.items_unlocked &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LevelUpRewardsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum LevelUpRewardsResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    AWARDED_ALREADY = 2,
}

impl ::protobuf::ProtobufEnum for LevelUpRewardsResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<LevelUpRewardsResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(LevelUpRewardsResponse_Result::UNSET),
            1 => ::std::option::Option::Some(LevelUpRewardsResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(LevelUpRewardsResponse_Result::AWARDED_ALREADY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [LevelUpRewardsResponse_Result] = &[
            LevelUpRewardsResponse_Result::UNSET,
            LevelUpRewardsResponse_Result::SUCCESS,
            LevelUpRewardsResponse_Result::AWARDED_ALREADY,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<LevelUpRewardsResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("LevelUpRewardsResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for LevelUpRewardsResponse_Result {
}

#[derive(Clone,Default)]
pub struct RecycleInventoryItemResponse {
    // message fields
    result: ::std::option::Option<RecycleInventoryItemResponse_Result>,
    new_count: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RecycleInventoryItemResponse {}

impl RecycleInventoryItemResponse {
    pub fn new() -> RecycleInventoryItemResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RecycleInventoryItemResponse {
        static mut instance: ::protobuf::lazy::Lazy<RecycleInventoryItemResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RecycleInventoryItemResponse,
        };
        unsafe {
            instance.get(|| {
                RecycleInventoryItemResponse {
                    result: ::std::option::Option::None,
                    new_count: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.RecycleInventoryItemResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: RecycleInventoryItemResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> RecycleInventoryItemResponse_Result {
        self.result.unwrap_or(RecycleInventoryItemResponse_Result::UNSET)
    }

    // optional int32 new_count = 2;

    pub fn clear_new_count(&mut self) {
        self.new_count = ::std::option::Option::None;
    }

    pub fn has_new_count(&self) -> bool {
        self.new_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_count(&mut self, v: i32) {
        self.new_count = ::std::option::Option::Some(v);
    }

    pub fn get_new_count(&self) -> i32 {
        self.new_count.unwrap_or(0)
    }
}

impl ::protobuf::Message for RecycleInventoryItemResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.new_count = ::std::option::Option::Some(tmp);
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.new_count {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.new_count {
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
        ::std::any::TypeId::of::<RecycleInventoryItemResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RecycleInventoryItemResponse {
    fn new() -> RecycleInventoryItemResponse {
        RecycleInventoryItemResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RecycleInventoryItemResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    RecycleInventoryItemResponse::has_result,
                    RecycleInventoryItemResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "new_count",
                    RecycleInventoryItemResponse::has_new_count,
                    RecycleInventoryItemResponse::get_new_count,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RecycleInventoryItemResponse>(
                    "RecycleInventoryItemResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RecycleInventoryItemResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_new_count();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RecycleInventoryItemResponse {
    fn eq(&self, other: &RecycleInventoryItemResponse) -> bool {
        self.result == other.result &&
        self.new_count == other.new_count &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RecycleInventoryItemResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RecycleInventoryItemResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    ERROR_NOT_ENOUGH_COPIES = 2,
    ERROR_CANNOT_RECYCLE_INCUBATORS = 3,
}

impl ::protobuf::ProtobufEnum for RecycleInventoryItemResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RecycleInventoryItemResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(RecycleInventoryItemResponse_Result::UNSET),
            1 => ::std::option::Option::Some(RecycleInventoryItemResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(RecycleInventoryItemResponse_Result::ERROR_NOT_ENOUGH_COPIES),
            3 => ::std::option::Option::Some(RecycleInventoryItemResponse_Result::ERROR_CANNOT_RECYCLE_INCUBATORS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RecycleInventoryItemResponse_Result] = &[
            RecycleInventoryItemResponse_Result::UNSET,
            RecycleInventoryItemResponse_Result::SUCCESS,
            RecycleInventoryItemResponse_Result::ERROR_NOT_ENOUGH_COPIES,
            RecycleInventoryItemResponse_Result::ERROR_CANNOT_RECYCLE_INCUBATORS,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<RecycleInventoryItemResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RecycleInventoryItemResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RecycleInventoryItemResponse_Result {
}

#[derive(Clone,Default)]
pub struct SetAvatarResponse {
    // message fields
    status: ::std::option::Option<SetAvatarResponse_Status>,
    player_data: ::protobuf::SingularPtrField<super::POGOProtos_Data::PlayerData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetAvatarResponse {}

impl SetAvatarResponse {
    pub fn new() -> SetAvatarResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetAvatarResponse {
        static mut instance: ::protobuf::lazy::Lazy<SetAvatarResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetAvatarResponse,
        };
        unsafe {
            instance.get(|| {
                SetAvatarResponse {
                    status: ::std::option::Option::None,
                    player_data: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.SetAvatarResponse.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: SetAvatarResponse_Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> SetAvatarResponse_Status {
        self.status.unwrap_or(SetAvatarResponse_Status::UNSET)
    }

    // optional .POGOProtos.Data.PlayerData player_data = 2;

    pub fn clear_player_data(&mut self) {
        self.player_data.clear();
    }

    pub fn has_player_data(&self) -> bool {
        self.player_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_data(&mut self, v: super::POGOProtos_Data::PlayerData) {
        self.player_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_data(&mut self) -> &mut super::POGOProtos_Data::PlayerData {
        if self.player_data.is_none() {
            self.player_data.set_default();
        };
        self.player_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_data(&mut self) -> super::POGOProtos_Data::PlayerData {
        self.player_data.take().unwrap_or_else(|| super::POGOProtos_Data::PlayerData::new())
    }

    pub fn get_player_data(&self) -> &super::POGOProtos_Data::PlayerData {
        self.player_data.as_ref().unwrap_or_else(|| super::POGOProtos_Data::PlayerData::default_instance())
    }
}

impl ::protobuf::Message for SetAvatarResponse {
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
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_data));
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
        for value in &self.status {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.player_data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.player_data.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<SetAvatarResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetAvatarResponse {
    fn new() -> SetAvatarResponse {
        SetAvatarResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetAvatarResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    SetAvatarResponse::has_status,
                    SetAvatarResponse::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "player_data",
                    SetAvatarResponse::has_player_data,
                    SetAvatarResponse::get_player_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetAvatarResponse>(
                    "SetAvatarResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetAvatarResponse {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_player_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetAvatarResponse {
    fn eq(&self, other: &SetAvatarResponse) -> bool {
        self.status == other.status &&
        self.player_data == other.player_data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetAvatarResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SetAvatarResponse_Status {
    UNSET = 0,
    SUCCESS = 1,
    AVATAR_ALREADY_SET = 2,
    FAILURE = 3,
}

impl ::protobuf::ProtobufEnum for SetAvatarResponse_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SetAvatarResponse_Status> {
        match value {
            0 => ::std::option::Option::Some(SetAvatarResponse_Status::UNSET),
            1 => ::std::option::Option::Some(SetAvatarResponse_Status::SUCCESS),
            2 => ::std::option::Option::Some(SetAvatarResponse_Status::AVATAR_ALREADY_SET),
            3 => ::std::option::Option::Some(SetAvatarResponse_Status::FAILURE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SetAvatarResponse_Status] = &[
            SetAvatarResponse_Status::UNSET,
            SetAvatarResponse_Status::SUCCESS,
            SetAvatarResponse_Status::AVATAR_ALREADY_SET,
            SetAvatarResponse_Status::FAILURE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<SetAvatarResponse_Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SetAvatarResponse_Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SetAvatarResponse_Status {
}

#[derive(Clone,Default)]
pub struct CollectDailyDefenderBonusResponse {
    // message fields
    result: ::std::option::Option<CollectDailyDefenderBonusResponse_Result>,
    currency_type: ::protobuf::RepeatedField<::std::string::String>,
    currency_awarded: ::std::vec::Vec<i32>,
    defenders_count: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CollectDailyDefenderBonusResponse {}

impl CollectDailyDefenderBonusResponse {
    pub fn new() -> CollectDailyDefenderBonusResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CollectDailyDefenderBonusResponse {
        static mut instance: ::protobuf::lazy::Lazy<CollectDailyDefenderBonusResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CollectDailyDefenderBonusResponse,
        };
        unsafe {
            instance.get(|| {
                CollectDailyDefenderBonusResponse {
                    result: ::std::option::Option::None,
                    currency_type: ::protobuf::RepeatedField::new(),
                    currency_awarded: ::std::vec::Vec::new(),
                    defenders_count: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.CollectDailyDefenderBonusResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CollectDailyDefenderBonusResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CollectDailyDefenderBonusResponse_Result {
        self.result.unwrap_or(CollectDailyDefenderBonusResponse_Result::UNSET)
    }

    // repeated string currency_type = 2;

    pub fn clear_currency_type(&mut self) {
        self.currency_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_currency_type(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.currency_type = v;
    }

    // Mutable pointer to the field.
    pub fn mut_currency_type(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.currency_type
    }

    // Take field
    pub fn take_currency_type(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.currency_type, ::protobuf::RepeatedField::new())
    }

    pub fn get_currency_type(&self) -> &[::std::string::String] {
        &self.currency_type
    }

    // repeated int32 currency_awarded = 3;

    pub fn clear_currency_awarded(&mut self) {
        self.currency_awarded.clear();
    }

    // Param is passed by value, moved
    pub fn set_currency_awarded(&mut self, v: ::std::vec::Vec<i32>) {
        self.currency_awarded = v;
    }

    // Mutable pointer to the field.
    pub fn mut_currency_awarded(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.currency_awarded
    }

    // Take field
    pub fn take_currency_awarded(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.currency_awarded, ::std::vec::Vec::new())
    }

    pub fn get_currency_awarded(&self) -> &[i32] {
        &self.currency_awarded
    }

    // optional int32 defenders_count = 4;

    pub fn clear_defenders_count(&mut self) {
        self.defenders_count = ::std::option::Option::None;
    }

    pub fn has_defenders_count(&self) -> bool {
        self.defenders_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_defenders_count(&mut self, v: i32) {
        self.defenders_count = ::std::option::Option::Some(v);
    }

    pub fn get_defenders_count(&self) -> i32 {
        self.defenders_count.unwrap_or(0)
    }
}

impl ::protobuf::Message for CollectDailyDefenderBonusResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.currency_type));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.currency_awarded));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.defenders_count = ::std::option::Option::Some(tmp);
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.currency_type {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.currency_awarded {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.defenders_count {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        for v in &self.currency_type {
            try!(os.write_string(2, &v));
        };
        for v in &self.currency_awarded {
            try!(os.write_int32(3, *v));
        };
        if let Some(v) = self.defenders_count {
            try!(os.write_int32(4, v));
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
        ::std::any::TypeId::of::<CollectDailyDefenderBonusResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CollectDailyDefenderBonusResponse {
    fn new() -> CollectDailyDefenderBonusResponse {
        CollectDailyDefenderBonusResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CollectDailyDefenderBonusResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    CollectDailyDefenderBonusResponse::has_result,
                    CollectDailyDefenderBonusResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "currency_type",
                    CollectDailyDefenderBonusResponse::get_currency_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "currency_awarded",
                    CollectDailyDefenderBonusResponse::get_currency_awarded,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "defenders_count",
                    CollectDailyDefenderBonusResponse::has_defenders_count,
                    CollectDailyDefenderBonusResponse::get_defenders_count,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CollectDailyDefenderBonusResponse>(
                    "CollectDailyDefenderBonusResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CollectDailyDefenderBonusResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_currency_type();
        self.clear_currency_awarded();
        self.clear_defenders_count();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CollectDailyDefenderBonusResponse {
    fn eq(&self, other: &CollectDailyDefenderBonusResponse) -> bool {
        self.result == other.result &&
        self.currency_type == other.currency_type &&
        self.currency_awarded == other.currency_awarded &&
        self.defenders_count == other.defenders_count &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CollectDailyDefenderBonusResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CollectDailyDefenderBonusResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    FAILURE = 2,
    TOO_SOON = 3,
    NO_DEFENDERS = 4,
}

impl ::protobuf::ProtobufEnum for CollectDailyDefenderBonusResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CollectDailyDefenderBonusResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(CollectDailyDefenderBonusResponse_Result::UNSET),
            1 => ::std::option::Option::Some(CollectDailyDefenderBonusResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(CollectDailyDefenderBonusResponse_Result::FAILURE),
            3 => ::std::option::Option::Some(CollectDailyDefenderBonusResponse_Result::TOO_SOON),
            4 => ::std::option::Option::Some(CollectDailyDefenderBonusResponse_Result::NO_DEFENDERS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CollectDailyDefenderBonusResponse_Result] = &[
            CollectDailyDefenderBonusResponse_Result::UNSET,
            CollectDailyDefenderBonusResponse_Result::SUCCESS,
            CollectDailyDefenderBonusResponse_Result::FAILURE,
            CollectDailyDefenderBonusResponse_Result::TOO_SOON,
            CollectDailyDefenderBonusResponse_Result::NO_DEFENDERS,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CollectDailyDefenderBonusResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CollectDailyDefenderBonusResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CollectDailyDefenderBonusResponse_Result {
}

#[derive(Clone,Default)]
pub struct SetContactSettingsResponse {
    // message fields
    status: ::std::option::Option<SetContactSettingsResponse_Status>,
    player_data: ::protobuf::SingularPtrField<super::POGOProtos_Data::PlayerData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetContactSettingsResponse {}

impl SetContactSettingsResponse {
    pub fn new() -> SetContactSettingsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetContactSettingsResponse {
        static mut instance: ::protobuf::lazy::Lazy<SetContactSettingsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetContactSettingsResponse,
        };
        unsafe {
            instance.get(|| {
                SetContactSettingsResponse {
                    status: ::std::option::Option::None,
                    player_data: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.SetContactSettingsResponse.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: SetContactSettingsResponse_Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> SetContactSettingsResponse_Status {
        self.status.unwrap_or(SetContactSettingsResponse_Status::UNSET)
    }

    // optional .POGOProtos.Data.PlayerData player_data = 2;

    pub fn clear_player_data(&mut self) {
        self.player_data.clear();
    }

    pub fn has_player_data(&self) -> bool {
        self.player_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_data(&mut self, v: super::POGOProtos_Data::PlayerData) {
        self.player_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_data(&mut self) -> &mut super::POGOProtos_Data::PlayerData {
        if self.player_data.is_none() {
            self.player_data.set_default();
        };
        self.player_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_data(&mut self) -> super::POGOProtos_Data::PlayerData {
        self.player_data.take().unwrap_or_else(|| super::POGOProtos_Data::PlayerData::new())
    }

    pub fn get_player_data(&self) -> &super::POGOProtos_Data::PlayerData {
        self.player_data.as_ref().unwrap_or_else(|| super::POGOProtos_Data::PlayerData::default_instance())
    }
}

impl ::protobuf::Message for SetContactSettingsResponse {
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
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_data));
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
        for value in &self.status {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.player_data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.player_data.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<SetContactSettingsResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetContactSettingsResponse {
    fn new() -> SetContactSettingsResponse {
        SetContactSettingsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetContactSettingsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    SetContactSettingsResponse::has_status,
                    SetContactSettingsResponse::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "player_data",
                    SetContactSettingsResponse::has_player_data,
                    SetContactSettingsResponse::get_player_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetContactSettingsResponse>(
                    "SetContactSettingsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetContactSettingsResponse {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_player_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetContactSettingsResponse {
    fn eq(&self, other: &SetContactSettingsResponse) -> bool {
        self.status == other.status &&
        self.player_data == other.player_data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetContactSettingsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SetContactSettingsResponse_Status {
    UNSET = 0,
    SUCCESS = 1,
    FAILURE = 2,
}

impl ::protobuf::ProtobufEnum for SetContactSettingsResponse_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SetContactSettingsResponse_Status> {
        match value {
            0 => ::std::option::Option::Some(SetContactSettingsResponse_Status::UNSET),
            1 => ::std::option::Option::Some(SetContactSettingsResponse_Status::SUCCESS),
            2 => ::std::option::Option::Some(SetContactSettingsResponse_Status::FAILURE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SetContactSettingsResponse_Status] = &[
            SetContactSettingsResponse_Status::UNSET,
            SetContactSettingsResponse_Status::SUCCESS,
            SetContactSettingsResponse_Status::FAILURE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<SetContactSettingsResponse_Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SetContactSettingsResponse_Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SetContactSettingsResponse_Status {
}

#[derive(Clone,Default)]
pub struct StartGymBattleResponse {
    // message fields
    result: ::std::option::Option<StartGymBattleResponse_Result>,
    battle_start_timestamp_ms: ::std::option::Option<i64>,
    battle_end_timestamp_ms: ::std::option::Option<i64>,
    battle_id: ::protobuf::SingularField<::std::string::String>,
    defender: ::protobuf::SingularPtrField<super::POGOProtos_Data_Battle::BattleParticipant>,
    battle_log: ::protobuf::SingularPtrField<super::POGOProtos_Data_Battle::BattleLog>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StartGymBattleResponse {}

impl StartGymBattleResponse {
    pub fn new() -> StartGymBattleResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StartGymBattleResponse {
        static mut instance: ::protobuf::lazy::Lazy<StartGymBattleResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StartGymBattleResponse,
        };
        unsafe {
            instance.get(|| {
                StartGymBattleResponse {
                    result: ::std::option::Option::None,
                    battle_start_timestamp_ms: ::std::option::Option::None,
                    battle_end_timestamp_ms: ::std::option::Option::None,
                    battle_id: ::protobuf::SingularField::none(),
                    defender: ::protobuf::SingularPtrField::none(),
                    battle_log: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.StartGymBattleResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: StartGymBattleResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> StartGymBattleResponse_Result {
        self.result.unwrap_or(StartGymBattleResponse_Result::UNSET)
    }

    // optional int64 battle_start_timestamp_ms = 2;

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

    // optional int64 battle_end_timestamp_ms = 3;

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

    // optional string battle_id = 4;

    pub fn clear_battle_id(&mut self) {
        self.battle_id.clear();
    }

    pub fn has_battle_id(&self) -> bool {
        self.battle_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battle_id(&mut self, v: ::std::string::String) {
        self.battle_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_battle_id(&mut self) -> &mut ::std::string::String {
        if self.battle_id.is_none() {
            self.battle_id.set_default();
        };
        self.battle_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_battle_id(&mut self) -> ::std::string::String {
        self.battle_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_battle_id(&self) -> &str {
        match self.battle_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .POGOProtos.Data.Battle.BattleParticipant defender = 5;

    pub fn clear_defender(&mut self) {
        self.defender.clear();
    }

    pub fn has_defender(&self) -> bool {
        self.defender.is_some()
    }

    // Param is passed by value, moved
    pub fn set_defender(&mut self, v: super::POGOProtos_Data_Battle::BattleParticipant) {
        self.defender = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_defender(&mut self) -> &mut super::POGOProtos_Data_Battle::BattleParticipant {
        if self.defender.is_none() {
            self.defender.set_default();
        };
        self.defender.as_mut().unwrap()
    }

    // Take field
    pub fn take_defender(&mut self) -> super::POGOProtos_Data_Battle::BattleParticipant {
        self.defender.take().unwrap_or_else(|| super::POGOProtos_Data_Battle::BattleParticipant::new())
    }

    pub fn get_defender(&self) -> &super::POGOProtos_Data_Battle::BattleParticipant {
        self.defender.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Battle::BattleParticipant::default_instance())
    }

    // optional .POGOProtos.Data.Battle.BattleLog battle_log = 6;

    pub fn clear_battle_log(&mut self) {
        self.battle_log.clear();
    }

    pub fn has_battle_log(&self) -> bool {
        self.battle_log.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battle_log(&mut self, v: super::POGOProtos_Data_Battle::BattleLog) {
        self.battle_log = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_battle_log(&mut self) -> &mut super::POGOProtos_Data_Battle::BattleLog {
        if self.battle_log.is_none() {
            self.battle_log.set_default();
        };
        self.battle_log.as_mut().unwrap()
    }

    // Take field
    pub fn take_battle_log(&mut self) -> super::POGOProtos_Data_Battle::BattleLog {
        self.battle_log.take().unwrap_or_else(|| super::POGOProtos_Data_Battle::BattleLog::new())
    }

    pub fn get_battle_log(&self) -> &super::POGOProtos_Data_Battle::BattleLog {
        self.battle_log.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Battle::BattleLog::default_instance())
    }
}

impl ::protobuf::Message for StartGymBattleResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.battle_start_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.battle_end_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.battle_id));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.defender));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.battle_log));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.battle_start_timestamp_ms {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.battle_end_timestamp_ms {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.battle_id {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.defender {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.battle_log {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.battle_start_timestamp_ms {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.battle_end_timestamp_ms {
            try!(os.write_int64(3, v));
        };
        if let Some(v) = self.battle_id.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.defender.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.battle_log.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<StartGymBattleResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StartGymBattleResponse {
    fn new() -> StartGymBattleResponse {
        StartGymBattleResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<StartGymBattleResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    StartGymBattleResponse::has_result,
                    StartGymBattleResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "battle_start_timestamp_ms",
                    StartGymBattleResponse::has_battle_start_timestamp_ms,
                    StartGymBattleResponse::get_battle_start_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "battle_end_timestamp_ms",
                    StartGymBattleResponse::has_battle_end_timestamp_ms,
                    StartGymBattleResponse::get_battle_end_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "battle_id",
                    StartGymBattleResponse::has_battle_id,
                    StartGymBattleResponse::get_battle_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "defender",
                    StartGymBattleResponse::has_defender,
                    StartGymBattleResponse::get_defender,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "battle_log",
                    StartGymBattleResponse::has_battle_log,
                    StartGymBattleResponse::get_battle_log,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StartGymBattleResponse>(
                    "StartGymBattleResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StartGymBattleResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_battle_start_timestamp_ms();
        self.clear_battle_end_timestamp_ms();
        self.clear_battle_id();
        self.clear_defender();
        self.clear_battle_log();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StartGymBattleResponse {
    fn eq(&self, other: &StartGymBattleResponse) -> bool {
        self.result == other.result &&
        self.battle_start_timestamp_ms == other.battle_start_timestamp_ms &&
        self.battle_end_timestamp_ms == other.battle_end_timestamp_ms &&
        self.battle_id == other.battle_id &&
        self.defender == other.defender &&
        self.battle_log == other.battle_log &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StartGymBattleResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum StartGymBattleResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    ERROR_GYM_NOT_FOUND = 2,
    ERROR_GYM_NEUTRAL = 3,
    ERROR_GYM_WRONG_TEAM = 4,
    ERROR_GYM_EMPTY = 5,
    ERROR_INVALID_DEFENDER = 6,
    ERROR_TRAINING_INVALID_ATTACKER_COUNT = 7,
    ERROR_ALL_POKEMON_FAINTED = 8,
    ERROR_TOO_MANY_BATTLES = 9,
    ERROR_TOO_MANY_PLAYERS = 10,
    ERROR_GYM_BATTLE_LOCKOUT = 11,
    ERROR_PLAYER_BELOW_MINIMUM_LEVEL = 12,
    ERROR_NOT_IN_RANGE = 13,
}

impl ::protobuf::ProtobufEnum for StartGymBattleResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<StartGymBattleResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(StartGymBattleResponse_Result::UNSET),
            1 => ::std::option::Option::Some(StartGymBattleResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(StartGymBattleResponse_Result::ERROR_GYM_NOT_FOUND),
            3 => ::std::option::Option::Some(StartGymBattleResponse_Result::ERROR_GYM_NEUTRAL),
            4 => ::std::option::Option::Some(StartGymBattleResponse_Result::ERROR_GYM_WRONG_TEAM),
            5 => ::std::option::Option::Some(StartGymBattleResponse_Result::ERROR_GYM_EMPTY),
            6 => ::std::option::Option::Some(StartGymBattleResponse_Result::ERROR_INVALID_DEFENDER),
            7 => ::std::option::Option::Some(StartGymBattleResponse_Result::ERROR_TRAINING_INVALID_ATTACKER_COUNT),
            8 => ::std::option::Option::Some(StartGymBattleResponse_Result::ERROR_ALL_POKEMON_FAINTED),
            9 => ::std::option::Option::Some(StartGymBattleResponse_Result::ERROR_TOO_MANY_BATTLES),
            10 => ::std::option::Option::Some(StartGymBattleResponse_Result::ERROR_TOO_MANY_PLAYERS),
            11 => ::std::option::Option::Some(StartGymBattleResponse_Result::ERROR_GYM_BATTLE_LOCKOUT),
            12 => ::std::option::Option::Some(StartGymBattleResponse_Result::ERROR_PLAYER_BELOW_MINIMUM_LEVEL),
            13 => ::std::option::Option::Some(StartGymBattleResponse_Result::ERROR_NOT_IN_RANGE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [StartGymBattleResponse_Result] = &[
            StartGymBattleResponse_Result::UNSET,
            StartGymBattleResponse_Result::SUCCESS,
            StartGymBattleResponse_Result::ERROR_GYM_NOT_FOUND,
            StartGymBattleResponse_Result::ERROR_GYM_NEUTRAL,
            StartGymBattleResponse_Result::ERROR_GYM_WRONG_TEAM,
            StartGymBattleResponse_Result::ERROR_GYM_EMPTY,
            StartGymBattleResponse_Result::ERROR_INVALID_DEFENDER,
            StartGymBattleResponse_Result::ERROR_TRAINING_INVALID_ATTACKER_COUNT,
            StartGymBattleResponse_Result::ERROR_ALL_POKEMON_FAINTED,
            StartGymBattleResponse_Result::ERROR_TOO_MANY_BATTLES,
            StartGymBattleResponse_Result::ERROR_TOO_MANY_PLAYERS,
            StartGymBattleResponse_Result::ERROR_GYM_BATTLE_LOCKOUT,
            StartGymBattleResponse_Result::ERROR_PLAYER_BELOW_MINIMUM_LEVEL,
            StartGymBattleResponse_Result::ERROR_NOT_IN_RANGE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<StartGymBattleResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("StartGymBattleResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for StartGymBattleResponse_Result {
}

#[derive(Clone,Default)]
pub struct EvolvePokemonResponse {
    // message fields
    result: ::std::option::Option<EvolvePokemonResponse_Result>,
    evolved_pokemon_data: ::protobuf::SingularPtrField<super::POGOProtos_Data::PokemonData>,
    experience_awarded: ::std::option::Option<i32>,
    candy_awarded: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EvolvePokemonResponse {}

impl EvolvePokemonResponse {
    pub fn new() -> EvolvePokemonResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EvolvePokemonResponse {
        static mut instance: ::protobuf::lazy::Lazy<EvolvePokemonResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EvolvePokemonResponse,
        };
        unsafe {
            instance.get(|| {
                EvolvePokemonResponse {
                    result: ::std::option::Option::None,
                    evolved_pokemon_data: ::protobuf::SingularPtrField::none(),
                    experience_awarded: ::std::option::Option::None,
                    candy_awarded: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.EvolvePokemonResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: EvolvePokemonResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> EvolvePokemonResponse_Result {
        self.result.unwrap_or(EvolvePokemonResponse_Result::UNSET)
    }

    // optional .POGOProtos.Data.PokemonData evolved_pokemon_data = 2;

    pub fn clear_evolved_pokemon_data(&mut self) {
        self.evolved_pokemon_data.clear();
    }

    pub fn has_evolved_pokemon_data(&self) -> bool {
        self.evolved_pokemon_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_evolved_pokemon_data(&mut self, v: super::POGOProtos_Data::PokemonData) {
        self.evolved_pokemon_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_evolved_pokemon_data(&mut self) -> &mut super::POGOProtos_Data::PokemonData {
        if self.evolved_pokemon_data.is_none() {
            self.evolved_pokemon_data.set_default();
        };
        self.evolved_pokemon_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_evolved_pokemon_data(&mut self) -> super::POGOProtos_Data::PokemonData {
        self.evolved_pokemon_data.take().unwrap_or_else(|| super::POGOProtos_Data::PokemonData::new())
    }

    pub fn get_evolved_pokemon_data(&self) -> &super::POGOProtos_Data::PokemonData {
        self.evolved_pokemon_data.as_ref().unwrap_or_else(|| super::POGOProtos_Data::PokemonData::default_instance())
    }

    // optional int32 experience_awarded = 3;

    pub fn clear_experience_awarded(&mut self) {
        self.experience_awarded = ::std::option::Option::None;
    }

    pub fn has_experience_awarded(&self) -> bool {
        self.experience_awarded.is_some()
    }

    // Param is passed by value, moved
    pub fn set_experience_awarded(&mut self, v: i32) {
        self.experience_awarded = ::std::option::Option::Some(v);
    }

    pub fn get_experience_awarded(&self) -> i32 {
        self.experience_awarded.unwrap_or(0)
    }

    // optional int32 candy_awarded = 4;

    pub fn clear_candy_awarded(&mut self) {
        self.candy_awarded = ::std::option::Option::None;
    }

    pub fn has_candy_awarded(&self) -> bool {
        self.candy_awarded.is_some()
    }

    // Param is passed by value, moved
    pub fn set_candy_awarded(&mut self, v: i32) {
        self.candy_awarded = ::std::option::Option::Some(v);
    }

    pub fn get_candy_awarded(&self) -> i32 {
        self.candy_awarded.unwrap_or(0)
    }
}

impl ::protobuf::Message for EvolvePokemonResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.evolved_pokemon_data));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.experience_awarded = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.candy_awarded = ::std::option::Option::Some(tmp);
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.evolved_pokemon_data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.experience_awarded {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.candy_awarded {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.evolved_pokemon_data.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.experience_awarded {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.candy_awarded {
            try!(os.write_int32(4, v));
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
        ::std::any::TypeId::of::<EvolvePokemonResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EvolvePokemonResponse {
    fn new() -> EvolvePokemonResponse {
        EvolvePokemonResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<EvolvePokemonResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    EvolvePokemonResponse::has_result,
                    EvolvePokemonResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "evolved_pokemon_data",
                    EvolvePokemonResponse::has_evolved_pokemon_data,
                    EvolvePokemonResponse::get_evolved_pokemon_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "experience_awarded",
                    EvolvePokemonResponse::has_experience_awarded,
                    EvolvePokemonResponse::get_experience_awarded,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "candy_awarded",
                    EvolvePokemonResponse::has_candy_awarded,
                    EvolvePokemonResponse::get_candy_awarded,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EvolvePokemonResponse>(
                    "EvolvePokemonResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EvolvePokemonResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_evolved_pokemon_data();
        self.clear_experience_awarded();
        self.clear_candy_awarded();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EvolvePokemonResponse {
    fn eq(&self, other: &EvolvePokemonResponse) -> bool {
        self.result == other.result &&
        self.evolved_pokemon_data == other.evolved_pokemon_data &&
        self.experience_awarded == other.experience_awarded &&
        self.candy_awarded == other.candy_awarded &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EvolvePokemonResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EvolvePokemonResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    FAILED_POKEMON_MISSING = 2,
    FAILED_INSUFFICIENT_RESOURCES = 3,
    FAILED_POKEMON_CANNOT_EVOLVE = 4,
    FAILED_POKEMON_IS_DEPLOYED = 5,
}

impl ::protobuf::ProtobufEnum for EvolvePokemonResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EvolvePokemonResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(EvolvePokemonResponse_Result::UNSET),
            1 => ::std::option::Option::Some(EvolvePokemonResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(EvolvePokemonResponse_Result::FAILED_POKEMON_MISSING),
            3 => ::std::option::Option::Some(EvolvePokemonResponse_Result::FAILED_INSUFFICIENT_RESOURCES),
            4 => ::std::option::Option::Some(EvolvePokemonResponse_Result::FAILED_POKEMON_CANNOT_EVOLVE),
            5 => ::std::option::Option::Some(EvolvePokemonResponse_Result::FAILED_POKEMON_IS_DEPLOYED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EvolvePokemonResponse_Result] = &[
            EvolvePokemonResponse_Result::UNSET,
            EvolvePokemonResponse_Result::SUCCESS,
            EvolvePokemonResponse_Result::FAILED_POKEMON_MISSING,
            EvolvePokemonResponse_Result::FAILED_INSUFFICIENT_RESOURCES,
            EvolvePokemonResponse_Result::FAILED_POKEMON_CANNOT_EVOLVE,
            EvolvePokemonResponse_Result::FAILED_POKEMON_IS_DEPLOYED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EvolvePokemonResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EvolvePokemonResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EvolvePokemonResponse_Result {
}

#[derive(Clone,Default)]
pub struct ReleasePokemonResponse {
    // message fields
    result: ::std::option::Option<ReleasePokemonResponse_Result>,
    candy_awarded: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReleasePokemonResponse {}

impl ReleasePokemonResponse {
    pub fn new() -> ReleasePokemonResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReleasePokemonResponse {
        static mut instance: ::protobuf::lazy::Lazy<ReleasePokemonResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReleasePokemonResponse,
        };
        unsafe {
            instance.get(|| {
                ReleasePokemonResponse {
                    result: ::std::option::Option::None,
                    candy_awarded: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.ReleasePokemonResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ReleasePokemonResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> ReleasePokemonResponse_Result {
        self.result.unwrap_or(ReleasePokemonResponse_Result::UNSET)
    }

    // optional int32 candy_awarded = 2;

    pub fn clear_candy_awarded(&mut self) {
        self.candy_awarded = ::std::option::Option::None;
    }

    pub fn has_candy_awarded(&self) -> bool {
        self.candy_awarded.is_some()
    }

    // Param is passed by value, moved
    pub fn set_candy_awarded(&mut self, v: i32) {
        self.candy_awarded = ::std::option::Option::Some(v);
    }

    pub fn get_candy_awarded(&self) -> i32 {
        self.candy_awarded.unwrap_or(0)
    }
}

impl ::protobuf::Message for ReleasePokemonResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.candy_awarded = ::std::option::Option::Some(tmp);
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.candy_awarded {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.candy_awarded {
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
        ::std::any::TypeId::of::<ReleasePokemonResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReleasePokemonResponse {
    fn new() -> ReleasePokemonResponse {
        ReleasePokemonResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReleasePokemonResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    ReleasePokemonResponse::has_result,
                    ReleasePokemonResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "candy_awarded",
                    ReleasePokemonResponse::has_candy_awarded,
                    ReleasePokemonResponse::get_candy_awarded,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReleasePokemonResponse>(
                    "ReleasePokemonResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReleasePokemonResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_candy_awarded();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReleasePokemonResponse {
    fn eq(&self, other: &ReleasePokemonResponse) -> bool {
        self.result == other.result &&
        self.candy_awarded == other.candy_awarded &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReleasePokemonResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReleasePokemonResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
    POKEMON_DEPLOYED = 2,
    FAILED = 3,
    ERROR_POKEMON_IS_EGG = 4,
    ERROR_POKEMON_IS_BUDDY = 5,
}

impl ::protobuf::ProtobufEnum for ReleasePokemonResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReleasePokemonResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(ReleasePokemonResponse_Result::UNSET),
            1 => ::std::option::Option::Some(ReleasePokemonResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(ReleasePokemonResponse_Result::POKEMON_DEPLOYED),
            3 => ::std::option::Option::Some(ReleasePokemonResponse_Result::FAILED),
            4 => ::std::option::Option::Some(ReleasePokemonResponse_Result::ERROR_POKEMON_IS_EGG),
            5 => ::std::option::Option::Some(ReleasePokemonResponse_Result::ERROR_POKEMON_IS_BUDDY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReleasePokemonResponse_Result] = &[
            ReleasePokemonResponse_Result::UNSET,
            ReleasePokemonResponse_Result::SUCCESS,
            ReleasePokemonResponse_Result::POKEMON_DEPLOYED,
            ReleasePokemonResponse_Result::FAILED,
            ReleasePokemonResponse_Result::ERROR_POKEMON_IS_EGG,
            ReleasePokemonResponse_Result::ERROR_POKEMON_IS_BUDDY,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ReleasePokemonResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ReleasePokemonResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ReleasePokemonResponse_Result {
}

#[derive(Clone,Default)]
pub struct GetPlayerProfileResponse {
    // message fields
    result: ::std::option::Option<GetPlayerProfileResponse_Result>,
    start_time: ::std::option::Option<i64>,
    badges: ::protobuf::RepeatedField<super::POGOProtos_Data::PlayerBadge>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetPlayerProfileResponse {}

impl GetPlayerProfileResponse {
    pub fn new() -> GetPlayerProfileResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetPlayerProfileResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetPlayerProfileResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetPlayerProfileResponse,
        };
        unsafe {
            instance.get(|| {
                GetPlayerProfileResponse {
                    result: ::std::option::Option::None,
                    start_time: ::std::option::Option::None,
                    badges: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.GetPlayerProfileResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: GetPlayerProfileResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> GetPlayerProfileResponse_Result {
        self.result.unwrap_or(GetPlayerProfileResponse_Result::UNSET)
    }

    // optional int64 start_time = 2;

    pub fn clear_start_time(&mut self) {
        self.start_time = ::std::option::Option::None;
    }

    pub fn has_start_time(&self) -> bool {
        self.start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_time(&mut self, v: i64) {
        self.start_time = ::std::option::Option::Some(v);
    }

    pub fn get_start_time(&self) -> i64 {
        self.start_time.unwrap_or(0)
    }

    // repeated .POGOProtos.Data.PlayerBadge badges = 3;

    pub fn clear_badges(&mut self) {
        self.badges.clear();
    }

    // Param is passed by value, moved
    pub fn set_badges(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Data::PlayerBadge>) {
        self.badges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_badges(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Data::PlayerBadge> {
        &mut self.badges
    }

    // Take field
    pub fn take_badges(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Data::PlayerBadge> {
        ::std::mem::replace(&mut self.badges, ::protobuf::RepeatedField::new())
    }

    pub fn get_badges(&self) -> &[super::POGOProtos_Data::PlayerBadge] {
        &self.badges
    }
}

impl ::protobuf::Message for GetPlayerProfileResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.start_time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.badges));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.start_time {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.badges {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.start_time {
            try!(os.write_int64(2, v));
        };
        for v in &self.badges {
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
        ::std::any::TypeId::of::<GetPlayerProfileResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetPlayerProfileResponse {
    fn new() -> GetPlayerProfileResponse {
        GetPlayerProfileResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetPlayerProfileResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    GetPlayerProfileResponse::has_result,
                    GetPlayerProfileResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "start_time",
                    GetPlayerProfileResponse::has_start_time,
                    GetPlayerProfileResponse::get_start_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "badges",
                    GetPlayerProfileResponse::get_badges,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetPlayerProfileResponse>(
                    "GetPlayerProfileResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetPlayerProfileResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_start_time();
        self.clear_badges();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetPlayerProfileResponse {
    fn eq(&self, other: &GetPlayerProfileResponse) -> bool {
        self.result == other.result &&
        self.start_time == other.start_time &&
        self.badges == other.badges &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetPlayerProfileResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum GetPlayerProfileResponse_Result {
    UNSET = 0,
    SUCCESS = 1,
}

impl ::protobuf::ProtobufEnum for GetPlayerProfileResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<GetPlayerProfileResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(GetPlayerProfileResponse_Result::UNSET),
            1 => ::std::option::Option::Some(GetPlayerProfileResponse_Result::SUCCESS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [GetPlayerProfileResponse_Result] = &[
            GetPlayerProfileResponse_Result::UNSET,
            GetPlayerProfileResponse_Result::SUCCESS,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<GetPlayerProfileResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("GetPlayerProfileResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for GetPlayerProfileResponse_Result {
}

#[derive(Clone,Default)]
pub struct EchoResponse {
    // message fields
    context: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EchoResponse {}

impl EchoResponse {
    pub fn new() -> EchoResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EchoResponse {
        static mut instance: ::protobuf::lazy::Lazy<EchoResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EchoResponse,
        };
        unsafe {
            instance.get(|| {
                EchoResponse {
                    context: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: ::std::string::String) {
        self.context = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut ::std::string::String {
        if self.context.is_none() {
            self.context.set_default();
        };
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> ::std::string::String {
        self.context.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_context(&self) -> &str {
        match self.context.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for EchoResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.context));
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
        for value in &self.context {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.context.as_ref() {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<EchoResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EchoResponse {
    fn new() -> EchoResponse {
        EchoResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<EchoResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "context",
                    EchoResponse::has_context,
                    EchoResponse::get_context,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EchoResponse>(
                    "EchoResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EchoResponse {
    fn clear(&mut self) {
        self.clear_context();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EchoResponse {
    fn eq(&self, other: &EchoResponse) -> bool {
        self.context == other.context &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EchoResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UseIncenseResponse {
    // message fields
    result: ::std::option::Option<UseIncenseResponse_Result>,
    applied_incense: ::protobuf::SingularPtrField<super::POGOProtos_Inventory::AppliedItem>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UseIncenseResponse {}

impl UseIncenseResponse {
    pub fn new() -> UseIncenseResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UseIncenseResponse {
        static mut instance: ::protobuf::lazy::Lazy<UseIncenseResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UseIncenseResponse,
        };
        unsafe {
            instance.get(|| {
                UseIncenseResponse {
                    result: ::std::option::Option::None,
                    applied_incense: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Responses.UseIncenseResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: UseIncenseResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> UseIncenseResponse_Result {
        self.result.unwrap_or(UseIncenseResponse_Result::UNKNOWN)
    }

    // optional .POGOProtos.Inventory.AppliedItem applied_incense = 2;

    pub fn clear_applied_incense(&mut self) {
        self.applied_incense.clear();
    }

    pub fn has_applied_incense(&self) -> bool {
        self.applied_incense.is_some()
    }

    // Param is passed by value, moved
    pub fn set_applied_incense(&mut self, v: super::POGOProtos_Inventory::AppliedItem) {
        self.applied_incense = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_applied_incense(&mut self) -> &mut super::POGOProtos_Inventory::AppliedItem {
        if self.applied_incense.is_none() {
            self.applied_incense.set_default();
        };
        self.applied_incense.as_mut().unwrap()
    }

    // Take field
    pub fn take_applied_incense(&mut self) -> super::POGOProtos_Inventory::AppliedItem {
        self.applied_incense.take().unwrap_or_else(|| super::POGOProtos_Inventory::AppliedItem::new())
    }

    pub fn get_applied_incense(&self) -> &super::POGOProtos_Inventory::AppliedItem {
        self.applied_incense.as_ref().unwrap_or_else(|| super::POGOProtos_Inventory::AppliedItem::default_instance())
    }
}

impl ::protobuf::Message for UseIncenseResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.applied_incense));
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.applied_incense {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.applied_incense.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<UseIncenseResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UseIncenseResponse {
    fn new() -> UseIncenseResponse {
        UseIncenseResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<UseIncenseResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    UseIncenseResponse::has_result,
                    UseIncenseResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "applied_incense",
                    UseIncenseResponse::has_applied_incense,
                    UseIncenseResponse::get_applied_incense,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UseIncenseResponse>(
                    "UseIncenseResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UseIncenseResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_applied_incense();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UseIncenseResponse {
    fn eq(&self, other: &UseIncenseResponse) -> bool {
        self.result == other.result &&
        self.applied_incense == other.applied_incense &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UseIncenseResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum UseIncenseResponse_Result {
    UNKNOWN = 0,
    SUCCESS = 1,
    INCENSE_ALREADY_ACTIVE = 2,
    NONE_IN_INVENTORY = 3,
    LOCATION_UNSET = 4,
}

impl ::protobuf::ProtobufEnum for UseIncenseResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UseIncenseResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(UseIncenseResponse_Result::UNKNOWN),
            1 => ::std::option::Option::Some(UseIncenseResponse_Result::SUCCESS),
            2 => ::std::option::Option::Some(UseIncenseResponse_Result::INCENSE_ALREADY_ACTIVE),
            3 => ::std::option::Option::Some(UseIncenseResponse_Result::NONE_IN_INVENTORY),
            4 => ::std::option::Option::Some(UseIncenseResponse_Result::LOCATION_UNSET),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [UseIncenseResponse_Result] = &[
            UseIncenseResponse_Result::UNKNOWN,
            UseIncenseResponse_Result::SUCCESS,
            UseIncenseResponse_Result::INCENSE_ALREADY_ACTIVE,
            UseIncenseResponse_Result::NONE_IN_INVENTORY,
            UseIncenseResponse_Result::LOCATION_UNSET,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<UseIncenseResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("UseIncenseResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for UseIncenseResponse_Result {
}

#[derive(Clone,Default)]
pub struct EncounterResponse {
    // message fields
    wild_pokemon: ::protobuf::SingularPtrField<super::POGOProtos_Map_Pokemon::WildPokemon>,
    background: ::std::option::Option<EncounterResponse_Background>,
    status: ::std::option::Option<EncounterResponse_Status>,
    capture_probability: ::protobuf::SingularPtrField<super::POGOProtos_Data_Capture::CaptureProbability>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EncounterResponse {}

impl EncounterResponse {
    pub fn new() -> EncounterResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EncounterResponse {
        static mut instance: ::protobuf::lazy::Lazy<EncounterResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EncounterResponse,
        };
        unsafe {
            instance.get(|| {
                EncounterResponse {
                    wild_pokemon: ::protobuf::SingularPtrField::none(),
                    background: ::std::option::Option::None,
                    status: ::std::option::Option::None,
                    capture_probability: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Map.Pokemon.WildPokemon wild_pokemon = 1;

    pub fn clear_wild_pokemon(&mut self) {
        self.wild_pokemon.clear();
    }

    pub fn has_wild_pokemon(&self) -> bool {
        self.wild_pokemon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wild_pokemon(&mut self, v: super::POGOProtos_Map_Pokemon::WildPokemon) {
        self.wild_pokemon = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_wild_pokemon(&mut self) -> &mut super::POGOProtos_Map_Pokemon::WildPokemon {
        if self.wild_pokemon.is_none() {
            self.wild_pokemon.set_default();
        };
        self.wild_pokemon.as_mut().unwrap()
    }

    // Take field
    pub fn take_wild_pokemon(&mut self) -> super::POGOProtos_Map_Pokemon::WildPokemon {
        self.wild_pokemon.take().unwrap_or_else(|| super::POGOProtos_Map_Pokemon::WildPokemon::new())
    }

    pub fn get_wild_pokemon(&self) -> &super::POGOProtos_Map_Pokemon::WildPokemon {
        self.wild_pokemon.as_ref().unwrap_or_else(|| super::POGOProtos_Map_Pokemon::WildPokemon::default_instance())
    }

    // optional .POGOProtos.Networking.Responses.EncounterResponse.Background background = 2;

    pub fn clear_background(&mut self) {
        self.background = ::std::option::Option::None;
    }

    pub fn has_background(&self) -> bool {
        self.background.is_some()
    }

    // Param is passed by value, moved
    pub fn set_background(&mut self, v: EncounterResponse_Background) {
        self.background = ::std::option::Option::Some(v);
    }

    pub fn get_background(&self) -> EncounterResponse_Background {
        self.background.unwrap_or(EncounterResponse_Background::PARK)
    }

    // optional .POGOProtos.Networking.Responses.EncounterResponse.Status status = 3;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: EncounterResponse_Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> EncounterResponse_Status {
        self.status.unwrap_or(EncounterResponse_Status::ENCOUNTER_ERROR)
    }

    // optional .POGOProtos.Data.Capture.CaptureProbability capture_probability = 4;

    pub fn clear_capture_probability(&mut self) {
        self.capture_probability.clear();
    }

    pub fn has_capture_probability(&self) -> bool {
        self.capture_probability.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capture_probability(&mut self, v: super::POGOProtos_Data_Capture::CaptureProbability) {
        self.capture_probability = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_capture_probability(&mut self) -> &mut super::POGOProtos_Data_Capture::CaptureProbability {
        if self.capture_probability.is_none() {
            self.capture_probability.set_default();
        };
        self.capture_probability.as_mut().unwrap()
    }

    // Take field
    pub fn take_capture_probability(&mut self) -> super::POGOProtos_Data_Capture::CaptureProbability {
        self.capture_probability.take().unwrap_or_else(|| super::POGOProtos_Data_Capture::CaptureProbability::new())
    }

    pub fn get_capture_probability(&self) -> &super::POGOProtos_Data_Capture::CaptureProbability {
        self.capture_probability.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Capture::CaptureProbability::default_instance())
    }
}

impl ::protobuf::Message for EncounterResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.wild_pokemon));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.background = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.status = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.capture_probability));
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
        for value in &self.wild_pokemon {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.background {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.status {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in &self.capture_probability {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.wild_pokemon.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.background {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.status {
            try!(os.write_enum(3, v.value()));
        };
        if let Some(v) = self.capture_probability.as_ref() {
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
        ::std::any::TypeId::of::<EncounterResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EncounterResponse {
    fn new() -> EncounterResponse {
        EncounterResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<EncounterResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "wild_pokemon",
                    EncounterResponse::has_wild_pokemon,
                    EncounterResponse::get_wild_pokemon,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "background",
                    EncounterResponse::has_background,
                    EncounterResponse::get_background,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    EncounterResponse::has_status,
                    EncounterResponse::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "capture_probability",
                    EncounterResponse::has_capture_probability,
                    EncounterResponse::get_capture_probability,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EncounterResponse>(
                    "EncounterResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EncounterResponse {
    fn clear(&mut self) {
        self.clear_wild_pokemon();
        self.clear_background();
        self.clear_status();
        self.clear_capture_probability();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EncounterResponse {
    fn eq(&self, other: &EncounterResponse) -> bool {
        self.wild_pokemon == other.wild_pokemon &&
        self.background == other.background &&
        self.status == other.status &&
        self.capture_probability == other.capture_probability &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EncounterResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EncounterResponse_Background {
    PARK = 0,
    DESERT = 1,
}

impl ::protobuf::ProtobufEnum for EncounterResponse_Background {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EncounterResponse_Background> {
        match value {
            0 => ::std::option::Option::Some(EncounterResponse_Background::PARK),
            1 => ::std::option::Option::Some(EncounterResponse_Background::DESERT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EncounterResponse_Background] = &[
            EncounterResponse_Background::PARK,
            EncounterResponse_Background::DESERT,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EncounterResponse_Background>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EncounterResponse_Background", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EncounterResponse_Background {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EncounterResponse_Status {
    ENCOUNTER_ERROR = 0,
    ENCOUNTER_SUCCESS = 1,
    ENCOUNTER_NOT_FOUND = 2,
    ENCOUNTER_CLOSED = 3,
    ENCOUNTER_POKEMON_FLED = 4,
    ENCOUNTER_NOT_IN_RANGE = 5,
    ENCOUNTER_ALREADY_HAPPENED = 6,
    POKEMON_INVENTORY_FULL = 7,
}

impl ::protobuf::ProtobufEnum for EncounterResponse_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EncounterResponse_Status> {
        match value {
            0 => ::std::option::Option::Some(EncounterResponse_Status::ENCOUNTER_ERROR),
            1 => ::std::option::Option::Some(EncounterResponse_Status::ENCOUNTER_SUCCESS),
            2 => ::std::option::Option::Some(EncounterResponse_Status::ENCOUNTER_NOT_FOUND),
            3 => ::std::option::Option::Some(EncounterResponse_Status::ENCOUNTER_CLOSED),
            4 => ::std::option::Option::Some(EncounterResponse_Status::ENCOUNTER_POKEMON_FLED),
            5 => ::std::option::Option::Some(EncounterResponse_Status::ENCOUNTER_NOT_IN_RANGE),
            6 => ::std::option::Option::Some(EncounterResponse_Status::ENCOUNTER_ALREADY_HAPPENED),
            7 => ::std::option::Option::Some(EncounterResponse_Status::POKEMON_INVENTORY_FULL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EncounterResponse_Status] = &[
            EncounterResponse_Status::ENCOUNTER_ERROR,
            EncounterResponse_Status::ENCOUNTER_SUCCESS,
            EncounterResponse_Status::ENCOUNTER_NOT_FOUND,
            EncounterResponse_Status::ENCOUNTER_CLOSED,
            EncounterResponse_Status::ENCOUNTER_POKEMON_FLED,
            EncounterResponse_Status::ENCOUNTER_NOT_IN_RANGE,
            EncounterResponse_Status::ENCOUNTER_ALREADY_HAPPENED,
            EncounterResponse_Status::POKEMON_INVENTORY_FULL,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EncounterResponse_Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EncounterResponse_Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EncounterResponse_Status {
}

#[derive(Clone,Default)]
pub struct ClaimCodenameResponse {
    // message fields
    codename: ::protobuf::SingularField<::std::string::String>,
    user_message: ::protobuf::SingularField<::std::string::String>,
    is_assignable: ::std::option::Option<bool>,
    status: ::std::option::Option<ClaimCodenameResponse_Status>,
    updated_player: ::protobuf::SingularPtrField<super::POGOProtos_Data::PlayerData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClaimCodenameResponse {}

impl ClaimCodenameResponse {
    pub fn new() -> ClaimCodenameResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClaimCodenameResponse {
        static mut instance: ::protobuf::lazy::Lazy<ClaimCodenameResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClaimCodenameResponse,
        };
        unsafe {
            instance.get(|| {
                ClaimCodenameResponse {
                    codename: ::protobuf::SingularField::none(),
                    user_message: ::protobuf::SingularField::none(),
                    is_assignable: ::std::option::Option::None,
                    status: ::std::option::Option::None,
                    updated_player: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string codename = 1;

    pub fn clear_codename(&mut self) {
        self.codename.clear();
    }

    pub fn has_codename(&self) -> bool {
        self.codename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_codename(&mut self, v: ::std::string::String) {
        self.codename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_codename(&mut self) -> &mut ::std::string::String {
        if self.codename.is_none() {
            self.codename.set_default();
        };
        self.codename.as_mut().unwrap()
    }

    // Take field
    pub fn take_codename(&mut self) -> ::std::string::String {
        self.codename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_codename(&self) -> &str {
        match self.codename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string user_message = 2;

    pub fn clear_user_message(&mut self) {
        self.user_message.clear();
    }

    pub fn has_user_message(&self) -> bool {
        self.user_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_message(&mut self, v: ::std::string::String) {
        self.user_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user_message(&mut self) -> &mut ::std::string::String {
        if self.user_message.is_none() {
            self.user_message.set_default();
        };
        self.user_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_user_message(&mut self) -> ::std::string::String {
        self.user_message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_user_message(&self) -> &str {
        match self.user_message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool is_assignable = 3;

    pub fn clear_is_assignable(&mut self) {
        self.is_assignable = ::std::option::Option::None;
    }

    pub fn has_is_assignable(&self) -> bool {
        self.is_assignable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_assignable(&mut self, v: bool) {
        self.is_assignable = ::std::option::Option::Some(v);
    }

    pub fn get_is_assignable(&self) -> bool {
        self.is_assignable.unwrap_or(false)
    }

    // optional .POGOProtos.Networking.Responses.ClaimCodenameResponse.Status status = 4;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: ClaimCodenameResponse_Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> ClaimCodenameResponse_Status {
        self.status.unwrap_or(ClaimCodenameResponse_Status::UNSET)
    }

    // optional .POGOProtos.Data.PlayerData updated_player = 5;

    pub fn clear_updated_player(&mut self) {
        self.updated_player.clear();
    }

    pub fn has_updated_player(&self) -> bool {
        self.updated_player.is_some()
    }

    // Param is passed by value, moved
    pub fn set_updated_player(&mut self, v: super::POGOProtos_Data::PlayerData) {
        self.updated_player = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_updated_player(&mut self) -> &mut super::POGOProtos_Data::PlayerData {
        if self.updated_player.is_none() {
            self.updated_player.set_default();
        };
        self.updated_player.as_mut().unwrap()
    }

    // Take field
    pub fn take_updated_player(&mut self) -> super::POGOProtos_Data::PlayerData {
        self.updated_player.take().unwrap_or_else(|| super::POGOProtos_Data::PlayerData::new())
    }

    pub fn get_updated_player(&self) -> &super::POGOProtos_Data::PlayerData {
        self.updated_player.as_ref().unwrap_or_else(|| super::POGOProtos_Data::PlayerData::default_instance())
    }
}

impl ::protobuf::Message for ClaimCodenameResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.codename));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.user_message));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_assignable = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.status = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.updated_player));
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
        for value in &self.codename {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.user_message {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.is_assignable.is_some() {
            my_size += 2;
        };
        for value in &self.status {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        for value in &self.updated_player {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.codename.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.user_message.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.is_assignable {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.status {
            try!(os.write_enum(4, v.value()));
        };
        if let Some(v) = self.updated_player.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<ClaimCodenameResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClaimCodenameResponse {
    fn new() -> ClaimCodenameResponse {
        ClaimCodenameResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClaimCodenameResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "codename",
                    ClaimCodenameResponse::has_codename,
                    ClaimCodenameResponse::get_codename,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "user_message",
                    ClaimCodenameResponse::has_user_message,
                    ClaimCodenameResponse::get_user_message,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_assignable",
                    ClaimCodenameResponse::has_is_assignable,
                    ClaimCodenameResponse::get_is_assignable,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    ClaimCodenameResponse::has_status,
                    ClaimCodenameResponse::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "updated_player",
                    ClaimCodenameResponse::has_updated_player,
                    ClaimCodenameResponse::get_updated_player,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClaimCodenameResponse>(
                    "ClaimCodenameResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClaimCodenameResponse {
    fn clear(&mut self) {
        self.clear_codename();
        self.clear_user_message();
        self.clear_is_assignable();
        self.clear_status();
        self.clear_updated_player();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ClaimCodenameResponse {
    fn eq(&self, other: &ClaimCodenameResponse) -> bool {
        self.codename == other.codename &&
        self.user_message == other.user_message &&
        self.is_assignable == other.is_assignable &&
        self.status == other.status &&
        self.updated_player == other.updated_player &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ClaimCodenameResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ClaimCodenameResponse_Status {
    UNSET = 0,
    SUCCESS = 1,
    CODENAME_NOT_AVAILABLE = 2,
    CODENAME_NOT_VALID = 3,
    CURRENT_OWNER = 4,
    CODENAME_CHANGE_NOT_ALLOWED = 5,
}

impl ::protobuf::ProtobufEnum for ClaimCodenameResponse_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ClaimCodenameResponse_Status> {
        match value {
            0 => ::std::option::Option::Some(ClaimCodenameResponse_Status::UNSET),
            1 => ::std::option::Option::Some(ClaimCodenameResponse_Status::SUCCESS),
            2 => ::std::option::Option::Some(ClaimCodenameResponse_Status::CODENAME_NOT_AVAILABLE),
            3 => ::std::option::Option::Some(ClaimCodenameResponse_Status::CODENAME_NOT_VALID),
            4 => ::std::option::Option::Some(ClaimCodenameResponse_Status::CURRENT_OWNER),
            5 => ::std::option::Option::Some(ClaimCodenameResponse_Status::CODENAME_CHANGE_NOT_ALLOWED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ClaimCodenameResponse_Status] = &[
            ClaimCodenameResponse_Status::UNSET,
            ClaimCodenameResponse_Status::SUCCESS,
            ClaimCodenameResponse_Status::CODENAME_NOT_AVAILABLE,
            ClaimCodenameResponse_Status::CODENAME_NOT_VALID,
            ClaimCodenameResponse_Status::CURRENT_OWNER,
            ClaimCodenameResponse_Status::CODENAME_CHANGE_NOT_ALLOWED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ClaimCodenameResponse_Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ClaimCodenameResponse_Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ClaimCodenameResponse_Status {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x25, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1f, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x1a, 0x15, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x1c, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61,
    0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x16, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1a, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x19, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61,
    0x70, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x19, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x19, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x47, 0x79, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x1d, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x2e, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x1c, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61,
    0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74,
    0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1a,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e,
    0x4c, 0x6f, 0x67, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e,
    0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x14, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x50, 0x00, 0x50, 0x01, 0x50, 0x02, 0x50, 0x03, 0x50, 0x04, 0x50, 0x05, 0x50, 0x06,
    0x50, 0x07, 0x50, 0x08, 0x50, 0x09, 0x50, 0x0a, 0x50, 0x0b, 0x50, 0x0c, 0x50, 0x0d, 0x22, 0x6b,
    0x0a, 0x11, 0x47, 0x65, 0x74, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x12, 0x3c, 0x0a,
    0x0b, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x44, 0x61, 0x74, 0x61, 0x52,
    0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x44, 0x61, 0x74, 0x61, 0x22, 0xab, 0x02, 0x0a, 0x17,
    0x53, 0x65, 0x74, 0x42, 0x75, 0x64, 0x64, 0x79, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x57, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3f, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x42, 0x75, 0x64,
    0x64, 0x79, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x12, 0x42, 0x0a, 0x0d, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x62, 0x75, 0x64, 0x64,
    0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x42, 0x75, 0x64, 0x64, 0x79, 0x50,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x0c, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x42,
    0x75, 0x64, 0x64, 0x79, 0x22, 0x73, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09,
    0x0a, 0x05, 0x55, 0x4e, 0x45, 0x53, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43,
    0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x1a, 0x0a, 0x16, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f,
    0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x44, 0x45, 0x50, 0x4c, 0x4f, 0x59, 0x45, 0x44,
    0x10, 0x02, 0x12, 0x1b, 0x0a, 0x17, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4f, 0x4b, 0x45,
    0x4d, 0x4f, 0x4e, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x4f, 0x57, 0x4e, 0x45, 0x44, 0x10, 0x03, 0x12,
    0x18, 0x0a, 0x14, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e,
    0x5f, 0x49, 0x53, 0x5f, 0x45, 0x47, 0x47, 0x10, 0x04, 0x22, 0xd4, 0x01, 0x0a, 0x16, 0x47, 0x65,
    0x74, 0x48, 0x61, 0x74, 0x63, 0x68, 0x65, 0x64, 0x45, 0x67, 0x67, 0x73, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x12, 0x21,
    0x0a, 0x0a, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x06, 0x42, 0x02, 0x10, 0x01, 0x52, 0x09, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49,
    0x64, 0x12, 0x2d, 0x0a, 0x12, 0x65, 0x78, 0x70, 0x65, 0x72, 0x69, 0x65, 0x6e, 0x63, 0x65, 0x5f,
    0x61, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x18, 0x03, 0x20, 0x03, 0x28, 0x05, 0x52, 0x11, 0x65,
    0x78, 0x70, 0x65, 0x72, 0x69, 0x65, 0x6e, 0x63, 0x65, 0x41, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64,
    0x12, 0x23, 0x0a, 0x0d, 0x63, 0x61, 0x6e, 0x64, 0x79, 0x5f, 0x61, 0x77, 0x61, 0x72, 0x64, 0x65,
    0x64, 0x18, 0x04, 0x20, 0x03, 0x28, 0x05, 0x52, 0x0c, 0x63, 0x61, 0x6e, 0x64, 0x79, 0x41, 0x77,
    0x61, 0x72, 0x64, 0x65, 0x64, 0x12, 0x29, 0x0a, 0x10, 0x73, 0x74, 0x61, 0x72, 0x64, 0x75, 0x73,
    0x74, 0x5f, 0x61, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x18, 0x05, 0x20, 0x03, 0x28, 0x05, 0x52,
    0x0f, 0x73, 0x74, 0x61, 0x72, 0x64, 0x75, 0x73, 0x74, 0x41, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64,
    0x22, 0xc9, 0x03, 0x0a, 0x11, 0x41, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x47, 0x79, 0x6d, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x51, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x39, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x41, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x47,
    0x79, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x40, 0x0a, 0x0a, 0x62, 0x61, 0x74,
    0x74, 0x6c, 0x65, 0x5f, 0x6c, 0x6f, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e,
    0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x4c, 0x6f, 0x67,
    0x52, 0x09, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x4c, 0x6f, 0x67, 0x12, 0x1b, 0x0a, 0x09, 0x62,
    0x61, 0x74, 0x74, 0x6c, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08,
    0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x49, 0x64, 0x12, 0x52, 0x0a, 0x0f, 0x61, 0x63, 0x74, 0x69,
    0x76, 0x65, 0x5f, 0x64, 0x65, 0x66, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x29, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44,
    0x61, 0x74, 0x61, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c,
    0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x0e, 0x61, 0x63,
    0x74, 0x69, 0x76, 0x65, 0x44, 0x65, 0x66, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x12, 0x52, 0x0a, 0x0f,
    0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x29, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x42,
    0x61, 0x74, 0x74, 0x6c, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f,
    0x52, 0x0e, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x41, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x65, 0x72,
    0x22, 0x5a, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e,
    0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53,
    0x10, 0x01, 0x12, 0x20, 0x0a, 0x1c, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x49, 0x4e, 0x56, 0x41,
    0x4c, 0x49, 0x44, 0x5f, 0x41, 0x54, 0x54, 0x41, 0x43, 0x4b, 0x5f, 0x41, 0x43, 0x54, 0x49, 0x4f,
    0x4e, 0x53, 0x10, 0x02, 0x12, 0x16, 0x0a, 0x12, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x4e, 0x4f,
    0x54, 0x5f, 0x49, 0x4e, 0x5f, 0x52, 0x41, 0x4e, 0x47, 0x45, 0x10, 0x03, 0x22, 0xab, 0x01, 0x0a,
    0x16, 0x47, 0x65, 0x74, 0x42, 0x75, 0x64, 0x64, 0x79, 0x57, 0x61, 0x6c, 0x6b, 0x65, 0x64, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65,
    0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73,
    0x73, 0x12, 0x49, 0x0a, 0x0f, 0x66, 0x61, 0x6d, 0x69, 0x6c, 0x79, 0x5f, 0x63, 0x61, 0x6e, 0x64,
    0x79, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x46, 0x61, 0x6d, 0x69, 0x6c, 0x79, 0x49, 0x64, 0x52, 0x0d, 0x66,
    0x61, 0x6d, 0x69, 0x6c, 0x79, 0x43, 0x61, 0x6e, 0x64, 0x79, 0x49, 0x64, 0x12, 0x2c, 0x0a, 0x12,
    0x63, 0x61, 0x6e, 0x64, 0x79, 0x5f, 0x65, 0x61, 0x72, 0x6e, 0x65, 0x64, 0x5f, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x10, 0x63, 0x61, 0x6e, 0x64, 0x79, 0x45,
    0x61, 0x72, 0x6e, 0x65, 0x64, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x22, 0xd4, 0x02, 0x0a, 0x16, 0x55,
    0x73, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x58, 0x70, 0x42, 0x6f, 0x6f, 0x73, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x56, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3e, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x55, 0x73, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x58,
    0x70, 0x42, 0x6f, 0x6f, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x47, 0x0a,
    0x0d, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x5f, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x41, 0x70, 0x70, 0x6c,
    0x69, 0x65, 0x64, 0x49, 0x74, 0x65, 0x6d, 0x73, 0x52, 0x0c, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65,
    0x64, 0x49, 0x74, 0x65, 0x6d, 0x73, 0x22, 0x98, 0x01, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07,
    0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x1b, 0x0a, 0x17, 0x45, 0x52, 0x52,
    0x4f, 0x52, 0x5f, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x5f, 0x49, 0x54, 0x45, 0x4d, 0x5f,
    0x54, 0x59, 0x50, 0x45, 0x10, 0x02, 0x12, 0x21, 0x0a, 0x1d, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f,
    0x58, 0x50, 0x5f, 0x42, 0x4f, 0x4f, 0x53, 0x54, 0x5f, 0x41, 0x4c, 0x52, 0x45, 0x41, 0x44, 0x59,
    0x5f, 0x41, 0x43, 0x54, 0x49, 0x56, 0x45, 0x10, 0x03, 0x12, 0x1c, 0x0a, 0x18, 0x45, 0x52, 0x52,
    0x4f, 0x52, 0x5f, 0x4e, 0x4f, 0x5f, 0x49, 0x54, 0x45, 0x4d, 0x53, 0x5f, 0x52, 0x45, 0x4d, 0x41,
    0x49, 0x4e, 0x49, 0x4e, 0x47, 0x10, 0x04, 0x12, 0x18, 0x0a, 0x14, 0x45, 0x52, 0x52, 0x4f, 0x52,
    0x5f, 0x4c, 0x4f, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10,
    0x05, 0x22, 0xf9, 0x03, 0x0a, 0x13, 0x46, 0x6f, 0x72, 0x74, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c,
    0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x17, 0x0a, 0x07, 0x66, 0x6f, 0x72,
    0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x66, 0x6f, 0x72, 0x74,
    0x49, 0x64, 0x12, 0x3a, 0x0a, 0x0a, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x63, 0x6f, 0x6c, 0x6f, 0x72,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x54, 0x65, 0x61, 0x6d, 0x43, 0x6f,
    0x6c, 0x6f, 0x72, 0x52, 0x09, 0x74, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6c, 0x6f, 0x72, 0x12, 0x3f,
    0x0a, 0x0c, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61,
    0x74, 0x61, 0x52, 0x0b, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x12,
    0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x5f, 0x75, 0x72, 0x6c,
    0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x09, 0x52, 0x09, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x55, 0x72,
    0x6c, 0x73, 0x12, 0x0e, 0x0a, 0x02, 0x66, 0x70, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52, 0x02,
    0x66, 0x70, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x74, 0x61, 0x6d, 0x69, 0x6e, 0x61, 0x18, 0x07, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x07, 0x73, 0x74, 0x61, 0x6d, 0x69, 0x6e, 0x61, 0x12, 0x1f, 0x0a, 0x0b,
    0x6d, 0x61, 0x78, 0x5f, 0x73, 0x74, 0x61, 0x6d, 0x69, 0x6e, 0x61, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x0a, 0x6d, 0x61, 0x78, 0x53, 0x74, 0x61, 0x6d, 0x69, 0x6e, 0x61, 0x12, 0x31, 0x0a,
    0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x46, 0x6f, 0x72,
    0x74, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x12, 0x1a, 0x0a, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x0a, 0x20, 0x01,
    0x28, 0x01, 0x52, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x1c, 0x0a, 0x09,
    0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x01, 0x52,
    0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65,
    0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x3f, 0x0a, 0x09,
    0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x73, 0x18, 0x0d, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70,
    0x2e, 0x46, 0x6f, 0x72, 0x74, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69,
    0x65, 0x72, 0x52, 0x09, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x73, 0x22, 0x85, 0x01,
    0x0a, 0x18, 0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e,
    0x67, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x12, 0x12, 0x0a, 0x04, 0x68, 0x61, 0x73, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04,
    0x68, 0x61, 0x73, 0x68, 0x12, 0x3f, 0x0a, 0x08, 0x73, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x47, 0x6c, 0x6f,
    0x62, 0x61, 0x6c, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x52, 0x08, 0x73, 0x65, 0x74,
    0x74, 0x69, 0x6e, 0x67, 0x73, 0x22, 0xf2, 0x01, 0x0a, 0x15, 0x55, 0x73, 0x65, 0x49, 0x74, 0x65,
    0x6d, 0x52, 0x65, 0x76, 0x69, 0x76, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x55, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x3d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x73, 0x2e, 0x55, 0x73, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x52, 0x65, 0x76, 0x69, 0x76, 0x65, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06,
    0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x74, 0x61, 0x6d, 0x69, 0x6e,
    0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x73, 0x74, 0x61, 0x6d, 0x69, 0x6e, 0x61,
    0x22, 0x68, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e,
    0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53,
    0x10, 0x01, 0x12, 0x14, 0x0a, 0x10, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x4e, 0x4f, 0x5f, 0x50,
    0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x10, 0x02, 0x12, 0x14, 0x0a, 0x10, 0x45, 0x52, 0x52, 0x4f,
    0x52, 0x5f, 0x43, 0x41, 0x4e, 0x4e, 0x4f, 0x54, 0x5f, 0x55, 0x53, 0x45, 0x10, 0x03, 0x12, 0x1a,
    0x0a, 0x16, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x44, 0x45, 0x50, 0x4c, 0x4f, 0x59, 0x45, 0x44,
    0x5f, 0x54, 0x4f, 0x5f, 0x46, 0x4f, 0x52, 0x54, 0x10, 0x04, 0x22, 0x64, 0x0a, 0x16, 0x43, 0x68,
    0x65, 0x63, 0x6b, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x25, 0x0a, 0x0e, 0x73, 0x68, 0x6f, 0x77, 0x5f, 0x63, 0x68, 0x61,
    0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0d, 0x73, 0x68,
    0x6f, 0x77, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x12, 0x23, 0x0a, 0x0d, 0x63,
    0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x5f, 0x75, 0x72, 0x6c, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0c, 0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x55, 0x72, 0x6c,
    0x22, 0x76, 0x0a, 0x1c, 0x4d, 0x61, 0x72, 0x6b, 0x54, 0x75, 0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c,
    0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x12, 0x3c, 0x0a, 0x0b, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x44, 0x61, 0x74, 0x61, 0x52, 0x0a, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x44, 0x61, 0x74, 0x61, 0x22, 0xb8, 0x01, 0x0a, 0x14, 0x50, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x48, 0x0a, 0x0d, 0x77, 0x69, 0x6c, 0x64, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f,
    0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f,
    0x6e, 0x2e, 0x57, 0x69, 0x6c, 0x64, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x0c, 0x77,
    0x69, 0x6c, 0x64, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x73, 0x12, 0x33, 0x0a, 0x05, 0x66,
    0x6f, 0x72, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x46, 0x6f, 0x72, 0x74,
    0x2e, 0x46, 0x6f, 0x72, 0x74, 0x44, 0x61, 0x74, 0x61, 0x52, 0x05, 0x66, 0x6f, 0x72, 0x74, 0x73,
    0x12, 0x21, 0x0a, 0x0c, 0x66, 0x6f, 0x72, 0x74, 0x73, 0x5f, 0x6e, 0x65, 0x61, 0x72, 0x62, 0x79,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x66, 0x6f, 0x72, 0x74, 0x73, 0x4e, 0x65, 0x61,
    0x72, 0x62, 0x79, 0x22, 0xac, 0x01, 0x0a, 0x1a, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x41, 0x77, 0x61,
    0x72, 0x64, 0x65, 0x64, 0x42, 0x61, 0x64, 0x67, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x12, 0x42, 0x0a, 0x0e,
    0x61, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x5f, 0x62, 0x61, 0x64, 0x67, 0x65, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x42, 0x61, 0x64, 0x67, 0x65, 0x54, 0x79, 0x70,
    0x65, 0x52, 0x0d, 0x61, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x42, 0x61, 0x64, 0x67, 0x65, 0x73,
    0x12, 0x30, 0x0a, 0x14, 0x61, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x5f, 0x62, 0x61, 0x64, 0x67,
    0x65, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x05, 0x52, 0x12,
    0x61, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x42, 0x61, 0x64, 0x67, 0x65, 0x4c, 0x65, 0x76, 0x65,
    0x6c, 0x73, 0x22, 0xa6, 0x02, 0x0a, 0x23, 0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x52,
    0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x56, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x63, 0x0a, 0x06, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x4b, 0x2e, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69,
    0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x44, 0x6f, 0x77,
    0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12,
    0x3d, 0x0a, 0x1b, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x74, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65,
    0x73, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x18, 0x69, 0x74, 0x65, 0x6d, 0x54, 0x65, 0x6d, 0x70, 0x6c, 0x61,
    0x74, 0x65, 0x73, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12, 0x39,
    0x0a, 0x19, 0x61, 0x73, 0x73, 0x65, 0x74, 0x5f, 0x64, 0x69, 0x67, 0x65, 0x73, 0x74, 0x5f, 0x74,
    0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x16, 0x61, 0x73, 0x73, 0x65, 0x74, 0x44, 0x69, 0x67, 0x65, 0x73, 0x74, 0x54, 0x69,
    0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x22, 0x20, 0x0a, 0x06, 0x52, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b,
    0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x22, 0xb3, 0x01, 0x0a, 0x19,
    0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x44, 0x61, 0x69, 0x6c, 0x79, 0x42, 0x6f, 0x6e, 0x75,
    0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x59, 0x0a, 0x06, 0x72, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x41, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e,
    0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x43, 0x6f, 0x6c, 0x6c,
    0x65, 0x63, 0x74, 0x44, 0x61, 0x69, 0x6c, 0x79, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x22, 0x3b, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09,
    0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43,
    0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52,
    0x45, 0x10, 0x02, 0x12, 0x0c, 0x0a, 0x08, 0x54, 0x4f, 0x4f, 0x5f, 0x53, 0x4f, 0x4f, 0x4e, 0x10,
    0x03, 0x22, 0x61, 0x0a, 0x17, 0x47, 0x65, 0x74, 0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64,
    0x55, 0x72, 0x6c, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x46, 0x0a, 0x0d,
    0x64, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x75, 0x72, 0x6c, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x55, 0x72,
    0x6c, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x0c, 0x64, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64,
    0x55, 0x72, 0x6c, 0x73, 0x22, 0xae, 0x02, 0x0a, 0x15, 0x47, 0x65, 0x74, 0x47, 0x79, 0x6d, 0x44,
    0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x3a,
    0x0a, 0x09, 0x67, 0x79, 0x6d, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x1d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44,
    0x61, 0x74, 0x61, 0x2e, 0x47, 0x79, 0x6d, 0x2e, 0x47, 0x79, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x65,
    0x52, 0x08, 0x67, 0x79, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x12,
    0x0a, 0x04, 0x75, 0x72, 0x6c, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x09, 0x52, 0x04, 0x75, 0x72,
    0x6c, 0x73, 0x12, 0x55, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x3d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x47, 0x79, 0x6d, 0x44, 0x65, 0x74, 0x61, 0x69,
    0x6c, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b,
    0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x38, 0x0a, 0x06, 0x52,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00,
    0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x16, 0x0a,
    0x12, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x49, 0x4e, 0x5f, 0x52, 0x41,
    0x4e, 0x47, 0x45, 0x10, 0x02, 0x22, 0x57, 0x0a, 0x1d, 0x47, 0x65, 0x74, 0x53, 0x75, 0x67, 0x67,
    0x65, 0x73, 0x74, 0x65, 0x64, 0x43, 0x6f, 0x64, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x63, 0x6f, 0x64, 0x65, 0x6e, 0x61,
    0x6d, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x52, 0x09, 0x63, 0x6f, 0x64, 0x65, 0x6e,
    0x61, 0x6d, 0x65, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x22, 0xf0,
    0x04, 0x0a, 0x19, 0x46, 0x6f, 0x72, 0x74, 0x44, 0x65, 0x70, 0x6c, 0x6f, 0x79, 0x50, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x59, 0x0a, 0x06,
    0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x41, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72,
    0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x46,
    0x6f, 0x72, 0x74, 0x44, 0x65, 0x70, 0x6c, 0x6f, 0x79, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52,
    0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x57, 0x0a, 0x0c, 0x66, 0x6f, 0x72, 0x74, 0x5f,
    0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x34, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f,
    0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e,
    0x46, 0x6f, 0x72, 0x74, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x52, 0x0b, 0x66, 0x6f, 0x72, 0x74, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73,
    0x12, 0x3f, 0x0a, 0x0c, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x74, 0x61,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e,
    0x44, 0x61, 0x74, 0x61, 0x52, 0x0b, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74,
    0x61, 0x12, 0x3a, 0x0a, 0x09, 0x67, 0x79, 0x6d, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x47, 0x79, 0x6d, 0x2e, 0x47, 0x79, 0x6d, 0x53, 0x74,
    0x61, 0x74, 0x65, 0x52, 0x08, 0x67, 0x79, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x65, 0x22, 0xa1, 0x02,
    0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x11, 0x0a, 0x0d, 0x4e, 0x4f, 0x5f, 0x52,
    0x45, 0x53, 0x55, 0x4c, 0x54, 0x5f, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53,
    0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x25, 0x0a, 0x21, 0x45, 0x52, 0x52, 0x4f,
    0x52, 0x5f, 0x41, 0x4c, 0x52, 0x45, 0x41, 0x44, 0x59, 0x5f, 0x48, 0x41, 0x53, 0x5f, 0x50, 0x4f,
    0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x4f, 0x4e, 0x5f, 0x46, 0x4f, 0x52, 0x54, 0x10, 0x02, 0x12,
    0x21, 0x0a, 0x1d, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x4f, 0x50, 0x50, 0x4f, 0x53, 0x49, 0x4e,
    0x47, 0x5f, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x4f, 0x57, 0x4e, 0x53, 0x5f, 0x46, 0x4f, 0x52, 0x54,
    0x10, 0x03, 0x12, 0x16, 0x0a, 0x12, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x46, 0x4f, 0x52, 0x54,
    0x5f, 0x49, 0x53, 0x5f, 0x46, 0x55, 0x4c, 0x4c, 0x10, 0x04, 0x12, 0x16, 0x0a, 0x12, 0x45, 0x52,
    0x52, 0x4f, 0x52, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x49, 0x4e, 0x5f, 0x52, 0x41, 0x4e, 0x47, 0x45,
    0x10, 0x05, 0x12, 0x1c, 0x0a, 0x18, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4c, 0x41, 0x59,
    0x45, 0x52, 0x5f, 0x48, 0x41, 0x53, 0x5f, 0x4e, 0x4f, 0x5f, 0x54, 0x45, 0x41, 0x4d, 0x10, 0x06,
    0x12, 0x1d, 0x0a, 0x19, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f,
    0x4e, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x46, 0x55, 0x4c, 0x4c, 0x5f, 0x48, 0x50, 0x10, 0x07, 0x12,
    0x24, 0x0a, 0x20, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4c, 0x41, 0x59, 0x45, 0x52, 0x5f,
    0x42, 0x45, 0x4c, 0x4f, 0x57, 0x5f, 0x4d, 0x49, 0x4e, 0x49, 0x4d, 0x55, 0x4d, 0x5f, 0x4c, 0x45,
    0x56, 0x45, 0x4c, 0x10, 0x08, 0x12, 0x1a, 0x0a, 0x16, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50,
    0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x49, 0x53, 0x5f, 0x42, 0x55, 0x44, 0x44, 0x59, 0x10,
    0x09, 0x22, 0x9d, 0x03, 0x0a, 0x18, 0x49, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x45, 0x6e, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x58,
    0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x40,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77,
    0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73,
    0x2e, 0x49, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x45, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65,
    0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x3f, 0x0a, 0x0c, 0x70, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61,
    0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x52, 0x0b, 0x70, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x12, 0x5c, 0x0a, 0x13, 0x63, 0x61, 0x70,
    0x74, 0x75, 0x72, 0x65, 0x5f, 0x70, 0x72, 0x6f, 0x62, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65,
    0x2e, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x61, 0x62, 0x69, 0x6c,
    0x69, 0x74, 0x79, 0x52, 0x12, 0x63, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x50, 0x72, 0x6f, 0x62,
    0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x22, 0x87, 0x01, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x1d, 0x0a, 0x19, 0x49, 0x4e, 0x43, 0x45, 0x4e, 0x53, 0x45, 0x5f, 0x45, 0x4e,
    0x43, 0x4f, 0x55, 0x4e, 0x54, 0x45, 0x52, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10,
    0x00, 0x12, 0x1d, 0x0a, 0x19, 0x49, 0x4e, 0x43, 0x45, 0x4e, 0x53, 0x45, 0x5f, 0x45, 0x4e, 0x43,
    0x4f, 0x55, 0x4e, 0x54, 0x45, 0x52, 0x5f, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01,
    0x12, 0x23, 0x0a, 0x1f, 0x49, 0x4e, 0x43, 0x45, 0x4e, 0x53, 0x45, 0x5f, 0x45, 0x4e, 0x43, 0x4f,
    0x55, 0x4e, 0x54, 0x45, 0x52, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x41, 0x56, 0x41, 0x49, 0x4c, 0x41,
    0x42, 0x4c, 0x45, 0x10, 0x02, 0x12, 0x1a, 0x0a, 0x16, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e,
    0x5f, 0x49, 0x4e, 0x56, 0x45, 0x4e, 0x54, 0x4f, 0x52, 0x59, 0x5f, 0x46, 0x55, 0x4c, 0x4c, 0x10,
    0x03, 0x22, 0x33, 0x0a, 0x17, 0x56, 0x65, 0x72, 0x69, 0x66, 0x79, 0x43, 0x68, 0x61, 0x6c, 0x6c,
    0x65, 0x6e, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x18, 0x0a, 0x07,
    0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x73,
    0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x22, 0xf2, 0x01, 0x0a, 0x15, 0x55, 0x73, 0x65, 0x49, 0x74,
    0x65, 0x6d, 0x50, 0x6f, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x55, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x3d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65,
    0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x73, 0x2e, 0x55, 0x73, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x50, 0x6f, 0x74, 0x69, 0x6f, 0x6e,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52,
    0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x74, 0x61, 0x6d, 0x69,
    0x6e, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x73, 0x74, 0x61, 0x6d, 0x69, 0x6e,
    0x61, 0x22, 0x68, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x55,
    0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53,
    0x53, 0x10, 0x01, 0x12, 0x14, 0x0a, 0x10, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x4e, 0x4f, 0x5f,
    0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x10, 0x02, 0x12, 0x14, 0x0a, 0x10, 0x45, 0x52, 0x52,
    0x4f, 0x52, 0x5f, 0x43, 0x41, 0x4e, 0x4e, 0x4f, 0x54, 0x5f, 0x55, 0x53, 0x45, 0x10, 0x03, 0x12,
    0x1a, 0x0a, 0x16, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x44, 0x45, 0x50, 0x4c, 0x4f, 0x59, 0x45,
    0x44, 0x5f, 0x54, 0x4f, 0x5f, 0x46, 0x4f, 0x52, 0x54, 0x10, 0x04, 0x22, 0xf5, 0x01, 0x0a, 0x12,
    0x45, 0x71, 0x75, 0x69, 0x70, 0x42, 0x61, 0x64, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x52, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x3a, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x73, 0x2e, 0x45, 0x71, 0x75, 0x69, 0x70, 0x42, 0x61, 0x64, 0x67, 0x65, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06,
    0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x41, 0x0a, 0x08, 0x65, 0x71, 0x75, 0x69, 0x70, 0x70,
    0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x2e, 0x45, 0x71, 0x75, 0x69, 0x70, 0x70, 0x65, 0x64, 0x42, 0x61, 0x64, 0x67, 0x65, 0x52,
    0x08, 0x65, 0x71, 0x75, 0x69, 0x70, 0x70, 0x65, 0x64, 0x22, 0x48, 0x0a, 0x06, 0x52, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b,
    0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x13, 0x0a, 0x0f, 0x43,
    0x4f, 0x4f, 0x4c, 0x44, 0x4f, 0x57, 0x4e, 0x5f, 0x41, 0x43, 0x54, 0x49, 0x56, 0x45, 0x10, 0x02,
    0x12, 0x11, 0x0a, 0x0d, 0x4e, 0x4f, 0x54, 0x5f, 0x51, 0x55, 0x41, 0x4c, 0x49, 0x46, 0x49, 0x45,
    0x44, 0x10, 0x03, 0x22, 0xdb, 0x02, 0x0a, 0x16, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x50,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x56,
    0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3e,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77,
    0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73,
    0x2e, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06,
    0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x47, 0x0a, 0x10, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64,
    0x65, 0x64, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1c, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61,
    0x74, 0x61, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x52, 0x0f,
    0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x64, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x22,
    0x9f, 0x01, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e,
    0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53,
    0x10, 0x01, 0x12, 0x1b, 0x0a, 0x17, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4f, 0x4b, 0x45,
    0x4d, 0x4f, 0x4e, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x46, 0x4f, 0x55, 0x4e, 0x44, 0x10, 0x02, 0x12,
    0x20, 0x0a, 0x1c, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x49, 0x4e, 0x53, 0x55, 0x46, 0x46, 0x49,
    0x43, 0x49, 0x45, 0x4e, 0x54, 0x5f, 0x52, 0x45, 0x53, 0x4f, 0x55, 0x52, 0x43, 0x45, 0x53, 0x10,
    0x03, 0x12, 0x1f, 0x0a, 0x1b, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x55, 0x50, 0x47, 0x52, 0x41,
    0x44, 0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x41, 0x56, 0x41, 0x49, 0x4c, 0x41, 0x42, 0x4c, 0x45,
    0x10, 0x04, 0x12, 0x1d, 0x0a, 0x19, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4f, 0x4b, 0x45,
    0x4d, 0x4f, 0x4e, 0x5f, 0x49, 0x53, 0x5f, 0x44, 0x45, 0x50, 0x4c, 0x4f, 0x59, 0x45, 0x44, 0x10,
    0x05, 0x22, 0xd2, 0x04, 0x0a, 0x12, 0x46, 0x6f, 0x72, 0x74, 0x53, 0x65, 0x61, 0x72, 0x63, 0x68,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x52, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3a, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67,
    0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x53,
    0x65, 0x61, 0x72, 0x63, 0x68, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x49, 0x0a, 0x0d,
    0x69, 0x74, 0x65, 0x6d, 0x73, 0x5f, 0x61, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x18, 0x02, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e,
    0x49, 0x74, 0x65, 0x6d, 0x41, 0x77, 0x61, 0x72, 0x64, 0x52, 0x0c, 0x69, 0x74, 0x65, 0x6d, 0x73,
    0x41, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x67, 0x65, 0x6d, 0x73, 0x5f,
    0x61, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x67,
    0x65, 0x6d, 0x73, 0x41, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x12, 0x46, 0x0a, 0x10, 0x70, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x65, 0x67, 0x67, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61,
    0x74, 0x61, 0x52, 0x0e, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x45,
    0x67, 0x67, 0x12, 0x2d, 0x0a, 0x12, 0x65, 0x78, 0x70, 0x65, 0x72, 0x69, 0x65, 0x6e, 0x63, 0x65,
    0x5f, 0x61, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x52, 0x11,
    0x65, 0x78, 0x70, 0x65, 0x72, 0x69, 0x65, 0x6e, 0x63, 0x65, 0x41, 0x77, 0x61, 0x72, 0x64, 0x65,
    0x64, 0x12, 0x43, 0x0a, 0x1e, 0x63, 0x6f, 0x6f, 0x6c, 0x64, 0x6f, 0x77, 0x6e, 0x5f, 0x63, 0x6f,
    0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
    0x5f, 0x6d, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x03, 0x52, 0x1b, 0x63, 0x6f, 0x6f, 0x6c, 0x64,
    0x6f, 0x77, 0x6e, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x54, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12, 0x3b, 0x0a, 0x1a, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x5f,
    0x68, 0x61, 0x63, 0x6b, 0x5f, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x6e, 0x75,
    0x6d, 0x62, 0x65, 0x72, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x52, 0x17, 0x63, 0x68, 0x61, 0x69,
    0x6e, 0x48, 0x61, 0x63, 0x6b, 0x53, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x4e, 0x75, 0x6d,
    0x62, 0x65, 0x72, 0x22, 0x80, 0x01, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x11,
    0x0a, 0x0d, 0x4e, 0x4f, 0x5f, 0x52, 0x45, 0x53, 0x55, 0x4c, 0x54, 0x5f, 0x53, 0x45, 0x54, 0x10,
    0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x10,
    0x0a, 0x0c, 0x4f, 0x55, 0x54, 0x5f, 0x4f, 0x46, 0x5f, 0x52, 0x41, 0x4e, 0x47, 0x45, 0x10, 0x02,
    0x12, 0x16, 0x0a, 0x12, 0x49, 0x4e, 0x5f, 0x43, 0x4f, 0x4f, 0x4c, 0x44, 0x4f, 0x57, 0x4e, 0x5f,
    0x50, 0x45, 0x52, 0x49, 0x4f, 0x44, 0x10, 0x03, 0x12, 0x12, 0x0a, 0x0e, 0x49, 0x4e, 0x56, 0x45,
    0x4e, 0x54, 0x4f, 0x52, 0x59, 0x5f, 0x46, 0x55, 0x4c, 0x4c, 0x10, 0x04, 0x12, 0x18, 0x0a, 0x14,
    0x45, 0x58, 0x43, 0x45, 0x45, 0x44, 0x45, 0x44, 0x5f, 0x44, 0x41, 0x49, 0x4c, 0x59, 0x5f, 0x4c,
    0x49, 0x4d, 0x49, 0x54, 0x10, 0x05, 0x22, 0xd1, 0x01, 0x0a, 0x1a, 0x53, 0x65, 0x74, 0x46, 0x61,
    0x76, 0x6f, 0x72, 0x69, 0x74, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x5a, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x42, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x46, 0x61, 0x76, 0x6f, 0x72,
    0x69, 0x74, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x22, 0x57, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x55,
    0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53,
    0x53, 0x10, 0x01, 0x12, 0x1b, 0x0a, 0x17, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4f, 0x4b,
    0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x46, 0x4f, 0x55, 0x4e, 0x44, 0x10, 0x02,
    0x12, 0x18, 0x0a, 0x14, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f,
    0x4e, 0x5f, 0x49, 0x53, 0x5f, 0x45, 0x47, 0x47, 0x10, 0x03, 0x22, 0xd7, 0x01, 0x0a, 0x12, 0x55,
    0x73, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x47, 0x79, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x52, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x3a, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x73, 0x2e, 0x55, 0x73, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x47, 0x79, 0x6d, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64,
    0x5f, 0x67, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x09, 0x75, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x64, 0x47, 0x70, 0x22, 0x4e, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09,
    0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43,
    0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x14, 0x0a, 0x10, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f,
    0x43, 0x41, 0x4e, 0x4e, 0x4f, 0x54, 0x5f, 0x55, 0x53, 0x45, 0x10, 0x02, 0x12, 0x16, 0x0a, 0x12,
    0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x49, 0x4e, 0x5f, 0x52, 0x41, 0x4e,
    0x47, 0x45, 0x10, 0x03, 0x22, 0xf7, 0x02, 0x0a, 0x14, 0x43, 0x61, 0x74, 0x63, 0x68, 0x50, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x59, 0x0a,
    0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x41, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f,
    0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e,
    0x43, 0x61, 0x74, 0x63, 0x68, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x43, 0x61, 0x74, 0x63, 0x68, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x21, 0x0a, 0x0c, 0x6d, 0x69, 0x73, 0x73,
    0x5f, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0b,
    0x6d, 0x69, 0x73, 0x73, 0x50, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x12, 0x2e, 0x0a, 0x13, 0x63,
    0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x64, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f,
    0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x06, 0x52, 0x11, 0x63, 0x61, 0x70, 0x74, 0x75, 0x72,
    0x65, 0x64, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x4a, 0x0a, 0x0d, 0x63,
    0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x5f, 0x61, 0x77, 0x61, 0x72, 0x64, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x25, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x44, 0x61, 0x74, 0x61, 0x2e, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x43, 0x61, 0x70,
    0x74, 0x75, 0x72, 0x65, 0x41, 0x77, 0x61, 0x72, 0x64, 0x52, 0x0c, 0x63, 0x61, 0x70, 0x74, 0x75,
    0x72, 0x65, 0x41, 0x77, 0x61, 0x72, 0x64, 0x22, 0x65, 0x0a, 0x0b, 0x43, 0x61, 0x74, 0x63, 0x68,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x0f, 0x0a, 0x0b, 0x43, 0x41, 0x54, 0x43, 0x48, 0x5f,
    0x45, 0x52, 0x52, 0x4f, 0x52, 0x10, 0x00, 0x12, 0x11, 0x0a, 0x0d, 0x43, 0x41, 0x54, 0x43, 0x48,
    0x5f, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x10, 0x0a, 0x0c, 0x43, 0x41,
    0x54, 0x43, 0x48, 0x5f, 0x45, 0x53, 0x43, 0x41, 0x50, 0x45, 0x10, 0x02, 0x12, 0x0e, 0x0a, 0x0a,
    0x43, 0x41, 0x54, 0x43, 0x48, 0x5f, 0x46, 0x4c, 0x45, 0x45, 0x10, 0x03, 0x12, 0x10, 0x0a, 0x0c,
    0x43, 0x41, 0x54, 0x43, 0x48, 0x5f, 0x4d, 0x49, 0x53, 0x53, 0x45, 0x44, 0x10, 0x04, 0x22, 0xd9,
    0x01, 0x0a, 0x16, 0x53, 0x66, 0x69, 0x64, 0x61, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x4c, 0x6f,
    0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x56, 0x0a, 0x06, 0x72, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3e, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e,
    0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x53, 0x66, 0x69, 0x64,
    0x61, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x4c, 0x6f, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x12, 0x45, 0x0a, 0x0b, 0x6c, 0x6f, 0x67, 0x5f, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x4c, 0x6f, 0x67, 0x73, 0x2e, 0x41, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x4c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x0a, 0x6c, 0x6f,
    0x67, 0x45, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x22, 0x20, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a,
    0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x22, 0x8a, 0x02, 0x0a, 0x16, 0x55,
    0x73, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x12,
    0x2a, 0x0a, 0x11, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x63, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x5f,
    0x6d, 0x75, 0x6c, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0f, 0x69, 0x74, 0x65, 0x6d,
    0x43, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x4d, 0x75, 0x6c, 0x74, 0x12, 0x24, 0x0a, 0x0e, 0x69,
    0x74, 0x65, 0x6d, 0x5f, 0x66, 0x6c, 0x65, 0x65, 0x5f, 0x6d, 0x75, 0x6c, 0x74, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x01, 0x52, 0x0c, 0x69, 0x74, 0x65, 0x6d, 0x46, 0x6c, 0x65, 0x65, 0x4d, 0x75, 0x6c,
    0x74, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x74, 0x6f, 0x70, 0x5f, 0x6d, 0x6f, 0x76, 0x65, 0x6d, 0x65,
    0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0c, 0x73, 0x74, 0x6f, 0x70, 0x4d, 0x6f,
    0x76, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x74, 0x6f, 0x70, 0x5f, 0x61,
    0x74, 0x74, 0x61, 0x63, 0x6b, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x73, 0x74, 0x6f,
    0x70, 0x41, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x12, 0x1d, 0x0a, 0x0a, 0x74, 0x61, 0x72, 0x67, 0x65,
    0x74, 0x5f, 0x6d, 0x61, 0x78, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x52, 0x09, 0x74, 0x61, 0x72,
    0x67, 0x65, 0x74, 0x4d, 0x61, 0x78, 0x12, 0x1f, 0x0a, 0x0b, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74,
    0x5f, 0x73, 0x6c, 0x6f, 0x77, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x74, 0x61, 0x72,
    0x67, 0x65, 0x74, 0x53, 0x6c, 0x6f, 0x77, 0x22, 0x82, 0x0c, 0x0a, 0x1d, 0x44, 0x6f, 0x77, 0x6e,
    0x6c, 0x6f, 0x61, 0x64, 0x49, 0x74, 0x65, 0x6d, 0x54, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65,
    0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63,
    0x65, 0x73, 0x73, 0x12, 0x72, 0x0a, 0x0e, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x74, 0x65, 0x6d, 0x70,
    0x6c, 0x61, 0x74, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x4b, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x44, 0x6f,
    0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x49, 0x74, 0x65, 0x6d, 0x54, 0x65, 0x6d, 0x70, 0x6c, 0x61,
    0x74, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x49, 0x74, 0x65, 0x6d,
    0x54, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x52, 0x0d, 0x69, 0x74, 0x65, 0x6d, 0x54, 0x65,
    0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x12, 0x21, 0x0a, 0x0c, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x74,
    0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x1a, 0xaf, 0x0a, 0x0a, 0x0c, 0x49,
    0x74, 0x65, 0x6d, 0x54, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x74,
    0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x0a, 0x74, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x49, 0x64, 0x12, 0x56, 0x0a, 0x10,
    0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x73, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x53, 0x65, 0x74, 0x74, 0x69,
    0x6e, 0x67, 0x73, 0x52, 0x0f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x53, 0x65, 0x74, 0x74,
    0x69, 0x6e, 0x67, 0x73, 0x12, 0x4d, 0x0a, 0x0d, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x73, 0x65, 0x74,
    0x74, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67,
    0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x53, 0x65, 0x74,
    0x74, 0x69, 0x6e, 0x67, 0x73, 0x52, 0x0c, 0x69, 0x74, 0x65, 0x6d, 0x53, 0x65, 0x74, 0x74, 0x69,
    0x6e, 0x67, 0x73, 0x12, 0x4d, 0x0a, 0x0d, 0x6d, 0x6f, 0x76, 0x65, 0x5f, 0x73, 0x65, 0x74, 0x74,
    0x69, 0x6e, 0x67, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73,
    0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x4d, 0x6f, 0x76, 0x65, 0x53, 0x65, 0x74, 0x74,
    0x69, 0x6e, 0x67, 0x73, 0x52, 0x0c, 0x6d, 0x6f, 0x76, 0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e,
    0x67, 0x73, 0x12, 0x66, 0x0a, 0x16, 0x6d, 0x6f, 0x76, 0x65, 0x5f, 0x73, 0x65, 0x71, 0x75, 0x65,
    0x6e, 0x63, 0x65, 0x5f, 0x73, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x30, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e,
    0x4d, 0x6f, 0x76, 0x65, 0x53, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x53, 0x65, 0x74, 0x74,
    0x69, 0x6e, 0x67, 0x73, 0x52, 0x14, 0x6d, 0x6f, 0x76, 0x65, 0x53, 0x65, 0x71, 0x75, 0x65, 0x6e,
    0x63, 0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x58, 0x0a, 0x0e, 0x74, 0x79,
    0x70, 0x65, 0x5f, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x31, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e,
    0x54, 0x79, 0x70, 0x65, 0x45, 0x66, 0x66, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x53, 0x65, 0x74,
    0x74, 0x69, 0x6e, 0x67, 0x73, 0x52, 0x0d, 0x74, 0x79, 0x70, 0x65, 0x45, 0x66, 0x66, 0x65, 0x63,
    0x74, 0x69, 0x76, 0x65, 0x12, 0x50, 0x0a, 0x0e, 0x62, 0x61, 0x64, 0x67, 0x65, 0x5f, 0x73, 0x65,
    0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x29, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e,
    0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x42, 0x61, 0x64, 0x67, 0x65, 0x53,
    0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x52, 0x0d, 0x62, 0x61, 0x64, 0x67, 0x65, 0x53, 0x65,
    0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x42, 0x0a, 0x06, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61,
    0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e,
    0x67, 0x73, 0x52, 0x06, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x12, 0x52, 0x0a, 0x0c, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x2f, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65,
    0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x50, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67,
    0x73, 0x52, 0x0b, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x49,
    0x0a, 0x09, 0x67, 0x79, 0x6d, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x0d, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x2c, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53,
    0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x47,
    0x79, 0x6d, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x52,
    0x08, 0x67, 0x79, 0x6d, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x56, 0x0a, 0x0f, 0x62, 0x61, 0x74,
    0x74, 0x6c, 0x65, 0x5f, 0x73, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x0e, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x2d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e,
    0x47, 0x79, 0x6d, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67,
    0x73, 0x52, 0x0e, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67,
    0x73, 0x12, 0x5c, 0x0a, 0x12, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x5f, 0x73,
    0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2d, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69,
    0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x45, 0x6e, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x65, 0x72, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x52, 0x11, 0x65, 0x6e,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12,
    0x54, 0x0a, 0x10, 0x69, 0x61, 0x70, 0x5f, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x64, 0x69, 0x73, 0x70,
    0x6c, 0x61, 0x79, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e,
    0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x49, 0x61, 0x70, 0x49, 0x74, 0x65, 0x6d, 0x44, 0x69,
    0x73, 0x70, 0x6c, 0x61, 0x79, 0x52, 0x0e, 0x69, 0x61, 0x70, 0x49, 0x74, 0x65, 0x6d, 0x44, 0x69,
    0x73, 0x70, 0x6c, 0x61, 0x79, 0x12, 0x4a, 0x0a, 0x0c, 0x69, 0x61, 0x70, 0x5f, 0x73, 0x65, 0x74,
    0x74, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67,
    0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x49, 0x61, 0x70, 0x53, 0x65, 0x74, 0x74,
    0x69, 0x6e, 0x67, 0x73, 0x52, 0x0b, 0x69, 0x61, 0x70, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67,
    0x73, 0x12, 0x5d, 0x0a, 0x10, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x75, 0x70, 0x67,
    0x72, 0x61, 0x64, 0x65, 0x73, 0x18, 0x12, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67,
    0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e,
    0x55, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x52,
    0x0f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x73,
    0x12, 0x5a, 0x0a, 0x0f, 0x65, 0x71, 0x75, 0x69, 0x70, 0x70, 0x65, 0x64, 0x5f, 0x62, 0x61, 0x64,
    0x67, 0x65, 0x73, 0x18, 0x13, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x31, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e,
    0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x45, 0x71, 0x75, 0x69, 0x70, 0x70, 0x65, 0x64, 0x42,
    0x61, 0x64, 0x67, 0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x52, 0x0e, 0x65, 0x71,
    0x75, 0x69, 0x70, 0x70, 0x65, 0x64, 0x42, 0x61, 0x64, 0x67, 0x65, 0x73, 0x22, 0xc0, 0x02, 0x0a,
    0x17, 0x41, 0x64, 0x64, 0x46, 0x6f, 0x72, 0x74, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x57, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3f, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67,
    0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x41, 0x64, 0x64, 0x46, 0x6f,
    0x72, 0x74, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x12, 0x57, 0x0a, 0x0c, 0x66, 0x6f, 0x72, 0x74, 0x5f, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c,
    0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x34, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x44, 0x65,
    0x74, 0x61, 0x69, 0x6c, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0b, 0x66,
    0x6f, 0x72, 0x74, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x22, 0x73, 0x0a, 0x06, 0x52, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x12, 0x11, 0x0a, 0x0d, 0x4e, 0x4f, 0x5f, 0x52, 0x45, 0x53, 0x55, 0x4c,
    0x54, 0x5f, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45,
    0x53, 0x53, 0x10, 0x01, 0x12, 0x1d, 0x0a, 0x19, 0x46, 0x4f, 0x52, 0x54, 0x5f, 0x41, 0x4c, 0x52,
    0x45, 0x41, 0x44, 0x59, 0x5f, 0x48, 0x41, 0x53, 0x5f, 0x4d, 0x4f, 0x44, 0x49, 0x46, 0x49, 0x45,
    0x52, 0x10, 0x02, 0x12, 0x10, 0x0a, 0x0c, 0x54, 0x4f, 0x4f, 0x5f, 0x46, 0x41, 0x52, 0x5f, 0x41,
    0x57, 0x41, 0x59, 0x10, 0x03, 0x12, 0x18, 0x0a, 0x14, 0x4e, 0x4f, 0x5f, 0x49, 0x54, 0x45, 0x4d,
    0x5f, 0x49, 0x4e, 0x5f, 0x49, 0x4e, 0x56, 0x45, 0x4e, 0x54, 0x4f, 0x52, 0x59, 0x10, 0x04, 0x22,
    0xd0, 0x02, 0x0a, 0x21, 0x45, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x54, 0x75, 0x74,
    0x6f, 0x72, 0x69, 0x61, 0x6c, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x61, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x49, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x45, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65,
    0x72, 0x54, 0x75, 0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74,
    0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x3f, 0x0a, 0x0c, 0x70, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61,
    0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x52, 0x0b, 0x70, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x12, 0x4a, 0x0a, 0x0d, 0x63, 0x61, 0x70,
    0x74, 0x75, 0x72, 0x65, 0x5f, 0x61, 0x77, 0x61, 0x72, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x25, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61,
    0x74, 0x61, 0x2e, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x43, 0x61, 0x70, 0x74, 0x75,
    0x72, 0x65, 0x41, 0x77, 0x61, 0x72, 0x64, 0x52, 0x0c, 0x63, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65,
    0x41, 0x77, 0x61, 0x72, 0x64, 0x22, 0x3b, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12,
    0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55,
    0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x19, 0x0a, 0x15, 0x45, 0x52, 0x52, 0x4f, 0x52,
    0x5f, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x5f, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e,
    0x10, 0x02, 0x22, 0x7f, 0x0a, 0x14, 0x47, 0x65, 0x74, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f,
    0x72, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75,
    0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x12, 0x4d, 0x0a, 0x0f, 0x69, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72,
    0x79, 0x5f, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e,
    0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x44, 0x65,
    0x6c, 0x74, 0x61, 0x52, 0x0e, 0x69, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x44, 0x65,
    0x6c, 0x74, 0x61, 0x22, 0x87, 0x01, 0x0a, 0x15, 0x47, 0x65, 0x74, 0x4d, 0x61, 0x70, 0x4f, 0x62,
    0x6a, 0x65, 0x63, 0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x34, 0x0a,
    0x09, 0x6d, 0x61, 0x70, 0x5f, 0x63, 0x65, 0x6c, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x17, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61,
    0x70, 0x2e, 0x4d, 0x61, 0x70, 0x43, 0x65, 0x6c, 0x6c, 0x52, 0x08, 0x6d, 0x61, 0x70, 0x43, 0x65,
    0x6c, 0x6c, 0x73, 0x12, 0x38, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x20, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x4d, 0x61, 0x70, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x53,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x22, 0xf1, 0x01,
    0x0a, 0x15, 0x53, 0x65, 0x74, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x54, 0x65, 0x61, 0x6d, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x55, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x50, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x54, 0x65, 0x61, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x3c,
    0x0a, 0x0b, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x44, 0x61, 0x74, 0x61,
    0x52, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x44, 0x61, 0x74, 0x61, 0x22, 0x43, 0x0a, 0x06,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10,
    0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x14,
    0x0a, 0x10, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x41, 0x4c, 0x52, 0x45, 0x41, 0x44, 0x59, 0x5f, 0x53,
    0x45, 0x54, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x10,
    0x03, 0x22, 0xe3, 0x03, 0x0a, 0x19, 0x47, 0x65, 0x74, 0x49, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65,
    0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x59, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x41, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x73, 0x2e, 0x47, 0x65, 0x74, 0x49, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x50, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x3a, 0x0a, 0x0a, 0x70, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d,
    0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x52, 0x09, 0x70, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75,
    0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75,
    0x64, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x01, 0x52, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65,
    0x12, 0x2d, 0x0a, 0x12, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x5f, 0x6c, 0x6f,
    0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x11, 0x65, 0x6e,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12,
    0x21, 0x0a, 0x0c, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0b, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72,
    0x49, 0x64, 0x12, 0x34, 0x0a, 0x16, 0x64, 0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x5f,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x03, 0x52, 0x14, 0x64, 0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x54, 0x69, 0x6d,
    0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x22, 0x6d, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x1d, 0x0a, 0x19, 0x49, 0x4e, 0x43, 0x45, 0x4e, 0x53, 0x45, 0x5f, 0x45, 0x4e,
    0x43, 0x4f, 0x55, 0x4e, 0x54, 0x45, 0x52, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10,
    0x00, 0x12, 0x1f, 0x0a, 0x1b, 0x49, 0x4e, 0x43, 0x45, 0x4e, 0x53, 0x45, 0x5f, 0x45, 0x4e, 0x43,
    0x4f, 0x55, 0x4e, 0x54, 0x45, 0x52, 0x5f, 0x41, 0x56, 0x41, 0x49, 0x4c, 0x41, 0x42, 0x4c, 0x45,
    0x10, 0x01, 0x12, 0x23, 0x0a, 0x1f, 0x49, 0x4e, 0x43, 0x45, 0x4e, 0x53, 0x45, 0x5f, 0x45, 0x4e,
    0x43, 0x4f, 0x55, 0x4e, 0x54, 0x45, 0x52, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x41, 0x56, 0x41, 0x49,
    0x4c, 0x41, 0x42, 0x4c, 0x45, 0x10, 0x02, 0x22, 0x76, 0x0a, 0x16, 0x47, 0x65, 0x74, 0x41, 0x73,
    0x73, 0x65, 0x74, 0x44, 0x69, 0x67, 0x65, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x39, 0x0a, 0x06, 0x64, 0x69, 0x67, 0x65, 0x73, 0x74, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44,
    0x61, 0x74, 0x61, 0x2e, 0x41, 0x73, 0x73, 0x65, 0x74, 0x44, 0x69, 0x67, 0x65, 0x73, 0x74, 0x45,
    0x6e, 0x74, 0x72, 0x79, 0x52, 0x06, 0x64, 0x69, 0x67, 0x65, 0x73, 0x74, 0x12, 0x21, 0x0a, 0x0c,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x0b, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x22,
    0xef, 0x02, 0x0a, 0x1e, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x43, 0x6f, 0x64, 0x65, 0x6e, 0x61, 0x6d,
    0x65, 0x41, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x63, 0x6f, 0x64, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63, 0x6f, 0x64, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x21,
    0x0a, 0x0c, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x75, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x12, 0x23, 0x0a, 0x0d, 0x69, 0x73, 0x5f, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x62,
    0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0c, 0x69, 0x73, 0x41, 0x73, 0x73, 0x69,
    0x67, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x12, 0x5e, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x46, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x43, 0x6f,
    0x64, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x41, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x06,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x22, 0x88, 0x01, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07,
    0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x1a, 0x0a, 0x16, 0x43, 0x4f, 0x44,
    0x45, 0x4e, 0x41, 0x4d, 0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x41, 0x56, 0x41, 0x49, 0x4c, 0x41,
    0x42, 0x4c, 0x45, 0x10, 0x02, 0x12, 0x16, 0x0a, 0x12, 0x43, 0x4f, 0x44, 0x45, 0x4e, 0x41, 0x4d,
    0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x10, 0x03, 0x12, 0x11, 0x0a,
    0x0d, 0x43, 0x55, 0x52, 0x52, 0x45, 0x4e, 0x54, 0x5f, 0x4f, 0x57, 0x4e, 0x45, 0x52, 0x10, 0x04,
    0x12, 0x1f, 0x0a, 0x1b, 0x43, 0x4f, 0x44, 0x45, 0x4e, 0x41, 0x4d, 0x45, 0x5f, 0x43, 0x48, 0x41,
    0x4e, 0x47, 0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x41, 0x4c, 0x4c, 0x4f, 0x57, 0x45, 0x44, 0x10,
    0x05, 0x22, 0x93, 0x03, 0x0a, 0x15, 0x44, 0x69, 0x73, 0x6b, 0x45, 0x6e, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x55, 0x0a, 0x06, 0x72,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3d, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x44, 0x69,
    0x73, 0x6b, 0x45, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x3f, 0x0a, 0x0c, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x64, 0x61,
    0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x52, 0x0b, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44,
    0x61, 0x74, 0x61, 0x12, 0x5c, 0x0a, 0x13, 0x63, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x5f, 0x70,
    0x72, 0x6f, 0x62, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x2b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61,
    0x74, 0x61, 0x2e, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x43, 0x61, 0x70, 0x74, 0x75,
    0x72, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x52, 0x12, 0x63,
    0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74,
    0x79, 0x22, 0x83, 0x01, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x0b, 0x0a, 0x07,
    0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43,
    0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x11, 0x0a, 0x0d, 0x4e, 0x4f, 0x54, 0x5f, 0x41, 0x56,
    0x41, 0x49, 0x4c, 0x41, 0x42, 0x4c, 0x45, 0x10, 0x02, 0x12, 0x10, 0x0a, 0x0c, 0x4e, 0x4f, 0x54,
    0x5f, 0x49, 0x4e, 0x5f, 0x52, 0x41, 0x4e, 0x47, 0x45, 0x10, 0x03, 0x12, 0x1e, 0x0a, 0x1a, 0x45,
    0x4e, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x45, 0x52, 0x5f, 0x41, 0x4c, 0x52, 0x45, 0x41, 0x44, 0x59,
    0x5f, 0x46, 0x49, 0x4e, 0x49, 0x53, 0x48, 0x45, 0x44, 0x10, 0x04, 0x12, 0x1a, 0x0a, 0x16, 0x50,
    0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x49, 0x4e, 0x56, 0x45, 0x4e, 0x54, 0x4f, 0x52, 0x59,
    0x5f, 0x46, 0x55, 0x4c, 0x4c, 0x10, 0x05, 0x22, 0xb5, 0x03, 0x0a, 0x1b, 0x55, 0x73, 0x65, 0x49,
    0x74, 0x65, 0x6d, 0x45, 0x67, 0x67, 0x49, 0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x5b, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x43, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x55, 0x73, 0x65, 0x49, 0x74, 0x65,
    0x6d, 0x45, 0x67, 0x67, 0x49, 0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x12, 0x47, 0x0a, 0x0d, 0x65, 0x67, 0x67, 0x5f, 0x69, 0x6e, 0x63, 0x75,
    0x62, 0x61, 0x74, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f,
    0x72, 0x79, 0x2e, 0x45, 0x67, 0x67, 0x49, 0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x52,
    0x0c, 0x65, 0x67, 0x67, 0x49, 0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x22, 0xef, 0x01,
    0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45,
    0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01,
    0x12, 0x1d, 0x0a, 0x19, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x49, 0x4e, 0x43, 0x55, 0x42, 0x41,
    0x54, 0x4f, 0x52, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x46, 0x4f, 0x55, 0x4e, 0x44, 0x10, 0x02, 0x12,
    0x1f, 0x0a, 0x1b, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e,
    0x5f, 0x45, 0x47, 0x47, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x46, 0x4f, 0x55, 0x4e, 0x44, 0x10, 0x03,
    0x12, 0x1c, 0x0a, 0x18, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f,
    0x4e, 0x5f, 0x49, 0x44, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x45, 0x47, 0x47, 0x10, 0x04, 0x12, 0x22,
    0x0a, 0x1e, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x49, 0x4e, 0x43, 0x55, 0x42, 0x41, 0x54, 0x4f,
    0x52, 0x5f, 0x41, 0x4c, 0x52, 0x45, 0x41, 0x44, 0x59, 0x5f, 0x49, 0x4e, 0x5f, 0x55, 0x53, 0x45,
    0x10, 0x05, 0x12, 0x24, 0x0a, 0x20, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4f, 0x4b, 0x45,
    0x4d, 0x4f, 0x4e, 0x5f, 0x41, 0x4c, 0x52, 0x45, 0x41, 0x44, 0x59, 0x5f, 0x49, 0x4e, 0x43, 0x55,
    0x42, 0x41, 0x54, 0x49, 0x4e, 0x47, 0x10, 0x06, 0x12, 0x25, 0x0a, 0x21, 0x45, 0x52, 0x52, 0x4f,
    0x52, 0x5f, 0x49, 0x4e, 0x43, 0x55, 0x42, 0x41, 0x54, 0x4f, 0x52, 0x5f, 0x4e, 0x4f, 0x5f, 0x55,
    0x53, 0x45, 0x53, 0x5f, 0x52, 0x45, 0x4d, 0x41, 0x49, 0x4e, 0x49, 0x4e, 0x47, 0x10, 0x07, 0x22,
    0xe7, 0x01, 0x0a, 0x17, 0x4e, 0x69, 0x63, 0x6b, 0x6e, 0x61, 0x6d, 0x65, 0x50, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x57, 0x0a, 0x06, 0x72,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3f, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x4e, 0x69,
    0x63, 0x6b, 0x6e, 0x61, 0x6d, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x22, 0x73, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09,
    0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43,
    0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x1a, 0x0a, 0x16, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f,
    0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x5f, 0x4e, 0x49, 0x43, 0x4b, 0x4e, 0x41, 0x4d, 0x45,
    0x10, 0x02, 0x12, 0x1b, 0x0a, 0x17, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4f, 0x4b, 0x45,
    0x4d, 0x4f, 0x4e, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x46, 0x4f, 0x55, 0x4e, 0x44, 0x10, 0x03, 0x12,
    0x18, 0x0a, 0x14, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e,
    0x5f, 0x49, 0x53, 0x5f, 0x45, 0x47, 0x47, 0x10, 0x04, 0x22, 0xc5, 0x02, 0x0a, 0x19, 0x46, 0x6f,
    0x72, 0x74, 0x52, 0x65, 0x63, 0x61, 0x6c, 0x6c, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x59, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x41, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x52, 0x65,
    0x63, 0x61, 0x6c, 0x6c, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x57, 0x0a, 0x0c, 0x66, 0x6f, 0x72, 0x74, 0x5f, 0x64, 0x65, 0x74, 0x61, 0x69,
    0x6c, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x34, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67,
    0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x44,
    0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0b,
    0x66, 0x6f, 0x72, 0x74, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x22, 0x74, 0x0a, 0x06, 0x52,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x11, 0x0a, 0x0d, 0x4e, 0x4f, 0x5f, 0x52, 0x45, 0x53, 0x55,
    0x4c, 0x54, 0x5f, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43,
    0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x16, 0x0a, 0x12, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x4e,
    0x4f, 0x54, 0x5f, 0x49, 0x4e, 0x5f, 0x52, 0x41, 0x4e, 0x47, 0x45, 0x10, 0x02, 0x12, 0x1d, 0x0a,
    0x19, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x4e,
    0x4f, 0x54, 0x5f, 0x4f, 0x4e, 0x5f, 0x46, 0x4f, 0x52, 0x54, 0x10, 0x03, 0x12, 0x13, 0x0a, 0x0f,
    0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x4e, 0x4f, 0x5f, 0x50, 0x4c, 0x41, 0x59, 0x45, 0x52, 0x10,
    0x04, 0x22, 0xbc, 0x02, 0x0a, 0x16, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x55, 0x70, 0x52, 0x65, 0x77,
    0x61, 0x72, 0x64, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x56, 0x0a, 0x06,
    0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3e, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72,
    0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x4c,
    0x65, 0x76, 0x65, 0x6c, 0x55, 0x70, 0x52, 0x65, 0x77, 0x61, 0x72, 0x64, 0x73, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x12, 0x49, 0x0a, 0x0d, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x5f, 0x61, 0x77,
    0x61, 0x72, 0x64, 0x65, 0x64, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f,
    0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x77, 0x61, 0x72,
    0x64, 0x52, 0x0c, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x41, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x12,
    0x48, 0x0a, 0x0e, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x5f, 0x75, 0x6e, 0x6c, 0x6f, 0x63, 0x6b, 0x65,
    0x64, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49,
    0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x52, 0x0d, 0x69, 0x74, 0x65, 0x6d,
    0x73, 0x55, 0x6e, 0x6c, 0x6f, 0x63, 0x6b, 0x65, 0x64, 0x22, 0x35, 0x0a, 0x06, 0x52, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b,
    0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x13, 0x0a, 0x0f, 0x41,
    0x57, 0x41, 0x52, 0x44, 0x45, 0x44, 0x5f, 0x41, 0x4c, 0x52, 0x45, 0x41, 0x44, 0x59, 0x10, 0x02,
    0x22, 0xfd, 0x01, 0x0a, 0x1c, 0x52, 0x65, 0x63, 0x79, 0x63, 0x6c, 0x65, 0x49, 0x6e, 0x76, 0x65,
    0x6e, 0x74, 0x6f, 0x72, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x5c, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x44, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x63, 0x79, 0x63, 0x6c, 0x65, 0x49, 0x6e, 0x76, 0x65, 0x6e,
    0x74, 0x6f, 0x72, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12,
    0x1b, 0x0a, 0x09, 0x6e, 0x65, 0x77, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x08, 0x6e, 0x65, 0x77, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x22, 0x62, 0x0a, 0x06,
    0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10,
    0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x1b,
    0x0a, 0x17, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x45, 0x4e, 0x4f, 0x55,
    0x47, 0x48, 0x5f, 0x43, 0x4f, 0x50, 0x49, 0x45, 0x53, 0x10, 0x02, 0x12, 0x23, 0x0a, 0x1f, 0x45,
    0x52, 0x52, 0x4f, 0x52, 0x5f, 0x43, 0x41, 0x4e, 0x4e, 0x4f, 0x54, 0x5f, 0x52, 0x45, 0x43, 0x59,
    0x43, 0x4c, 0x45, 0x5f, 0x49, 0x4e, 0x43, 0x55, 0x42, 0x41, 0x54, 0x4f, 0x52, 0x53, 0x10, 0x03,
    0x22, 0xeb, 0x01, 0x0a, 0x11, 0x53, 0x65, 0x74, 0x41, 0x76, 0x61, 0x74, 0x61, 0x72, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x51, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x39, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x41, 0x76, 0x61, 0x74,
    0x61, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x3c, 0x0a, 0x0b, 0x70, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61,
    0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x44, 0x61, 0x74, 0x61, 0x52, 0x0a, 0x70, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x44, 0x61, 0x74, 0x61, 0x22, 0x45, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07,
    0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x16, 0x0a, 0x12, 0x41, 0x56, 0x41,
    0x54, 0x41, 0x52, 0x5f, 0x41, 0x4c, 0x52, 0x45, 0x41, 0x44, 0x59, 0x5f, 0x53, 0x45, 0x54, 0x10,
    0x02, 0x12, 0x0b, 0x0a, 0x07, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x10, 0x03, 0x22, 0xce,
    0x02, 0x0a, 0x21, 0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x44, 0x61, 0x69, 0x6c, 0x79, 0x44,
    0x65, 0x66, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x61, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x49, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x44, 0x61,
    0x69, 0x6c, 0x79, 0x44, 0x65, 0x66, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x42, 0x6f, 0x6e, 0x75, 0x73,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52,
    0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x75, 0x72, 0x72, 0x65,
    0x6e, 0x63, 0x79, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0c,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x29, 0x0a, 0x10,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x5f, 0x61, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64,
    0x18, 0x03, 0x20, 0x03, 0x28, 0x05, 0x52, 0x0f, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79,
    0x41, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x12, 0x27, 0x0a, 0x0f, 0x64, 0x65, 0x66, 0x65, 0x6e,
    0x64, 0x65, 0x72, 0x73, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x0e, 0x64, 0x65, 0x66, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x73, 0x43, 0x6f, 0x75, 0x6e, 0x74,
    0x22, 0x4d, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e,
    0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53,
    0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x10, 0x02, 0x12,
    0x0c, 0x0a, 0x08, 0x54, 0x4f, 0x4f, 0x5f, 0x53, 0x4f, 0x4f, 0x4e, 0x10, 0x03, 0x12, 0x10, 0x0a,
    0x0c, 0x4e, 0x4f, 0x5f, 0x44, 0x45, 0x46, 0x45, 0x4e, 0x44, 0x45, 0x52, 0x53, 0x10, 0x04, 0x22,
    0xe5, 0x01, 0x0a, 0x1a, 0x53, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x53, 0x65,
    0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x5a,
    0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x42,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77,
    0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73,
    0x2e, 0x53, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x53, 0x65, 0x74, 0x74, 0x69,
    0x6e, 0x67, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x3c, 0x0a, 0x0b, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x44, 0x61, 0x74, 0x61, 0x52, 0x0a, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x44, 0x61, 0x74, 0x61, 0x22, 0x2d, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a,
    0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x46, 0x41,
    0x49, 0x4c, 0x55, 0x52, 0x45, 0x10, 0x02, 0x22, 0x84, 0x06, 0x0a, 0x16, 0x53, 0x74, 0x61, 0x72,
    0x74, 0x47, 0x79, 0x6d, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x56, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x3e, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x73, 0x2e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x47, 0x79, 0x6d, 0x42, 0x61, 0x74,
    0x74, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x39, 0x0a, 0x19, 0x62, 0x61,
    0x74, 0x74, 0x6c, 0x65, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x16, 0x62,
    0x61, 0x74, 0x74, 0x6c, 0x65, 0x53, 0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12, 0x35, 0x0a, 0x17, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x5f,
    0x65, 0x6e, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x52, 0x14, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x45, 0x6e,
    0x64, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12, 0x1b, 0x0a, 0x09,
    0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x08, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x49, 0x64, 0x12, 0x45, 0x0a, 0x08, 0x64, 0x65, 0x66,
    0x65, 0x6e, 0x64, 0x65, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x29, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x42, 0x61,
    0x74, 0x74, 0x6c, 0x65, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69,
    0x63, 0x69, 0x70, 0x61, 0x6e, 0x74, 0x52, 0x08, 0x64, 0x65, 0x66, 0x65, 0x6e, 0x64, 0x65, 0x72,
    0x12, 0x40, 0x0a, 0x0a, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x5f, 0x6c, 0x6f, 0x67, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x42, 0x61,
    0x74, 0x74, 0x6c, 0x65, 0x4c, 0x6f, 0x67, 0x52, 0x09, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x4c,
    0x6f, 0x67, 0x22, 0xf9, 0x02, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09, 0x0a,
    0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43,
    0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x13, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x47,
    0x59, 0x4d, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x46, 0x4f, 0x55, 0x4e, 0x44, 0x10, 0x02, 0x12, 0x15,
    0x0a, 0x11, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x47, 0x59, 0x4d, 0x5f, 0x4e, 0x45, 0x55, 0x54,
    0x52, 0x41, 0x4c, 0x10, 0x03, 0x12, 0x18, 0x0a, 0x14, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x47,
    0x59, 0x4d, 0x5f, 0x57, 0x52, 0x4f, 0x4e, 0x47, 0x5f, 0x54, 0x45, 0x41, 0x4d, 0x10, 0x04, 0x12,
    0x13, 0x0a, 0x0f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x47, 0x59, 0x4d, 0x5f, 0x45, 0x4d, 0x50,
    0x54, 0x59, 0x10, 0x05, 0x12, 0x1a, 0x0a, 0x16, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x49, 0x4e,
    0x56, 0x41, 0x4c, 0x49, 0x44, 0x5f, 0x44, 0x45, 0x46, 0x45, 0x4e, 0x44, 0x45, 0x52, 0x10, 0x06,
    0x12, 0x29, 0x0a, 0x25, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x54, 0x52, 0x41, 0x49, 0x4e, 0x49,
    0x4e, 0x47, 0x5f, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x5f, 0x41, 0x54, 0x54, 0x41, 0x43,
    0x4b, 0x45, 0x52, 0x5f, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x10, 0x07, 0x12, 0x1d, 0x0a, 0x19, 0x45,
    0x52, 0x52, 0x4f, 0x52, 0x5f, 0x41, 0x4c, 0x4c, 0x5f, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e,
    0x5f, 0x46, 0x41, 0x49, 0x4e, 0x54, 0x45, 0x44, 0x10, 0x08, 0x12, 0x1a, 0x0a, 0x16, 0x45, 0x52,
    0x52, 0x4f, 0x52, 0x5f, 0x54, 0x4f, 0x4f, 0x5f, 0x4d, 0x41, 0x4e, 0x59, 0x5f, 0x42, 0x41, 0x54,
    0x54, 0x4c, 0x45, 0x53, 0x10, 0x09, 0x12, 0x1a, 0x0a, 0x16, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f,
    0x54, 0x4f, 0x4f, 0x5f, 0x4d, 0x41, 0x4e, 0x59, 0x5f, 0x50, 0x4c, 0x41, 0x59, 0x45, 0x52, 0x53,
    0x10, 0x0a, 0x12, 0x1c, 0x0a, 0x18, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x47, 0x59, 0x4d, 0x5f,
    0x42, 0x41, 0x54, 0x54, 0x4c, 0x45, 0x5f, 0x4c, 0x4f, 0x43, 0x4b, 0x4f, 0x55, 0x54, 0x10, 0x0b,
    0x12, 0x24, 0x0a, 0x20, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4c, 0x41, 0x59, 0x45, 0x52,
    0x5f, 0x42, 0x45, 0x4c, 0x4f, 0x57, 0x5f, 0x4d, 0x49, 0x4e, 0x49, 0x4d, 0x55, 0x4d, 0x5f, 0x4c,
    0x45, 0x56, 0x45, 0x4c, 0x10, 0x0c, 0x12, 0x16, 0x0a, 0x12, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f,
    0x4e, 0x4f, 0x54, 0x5f, 0x49, 0x4e, 0x5f, 0x52, 0x41, 0x4e, 0x47, 0x45, 0x10, 0x0d, 0x22, 0xb6,
    0x03, 0x0a, 0x15, 0x45, 0x76, 0x6f, 0x6c, 0x76, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x55, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67,
    0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x45, 0x76, 0x6f, 0x6c, 0x76,
    0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12,
    0x4e, 0x0a, 0x14, 0x65, 0x76, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e,
    0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x52, 0x12, 0x65, 0x76, 0x6f,
    0x6c, 0x76, 0x65, 0x64, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x12,
    0x2d, 0x0a, 0x12, 0x65, 0x78, 0x70, 0x65, 0x72, 0x69, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x61, 0x77,
    0x61, 0x72, 0x64, 0x65, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x11, 0x65, 0x78, 0x70,
    0x65, 0x72, 0x69, 0x65, 0x6e, 0x63, 0x65, 0x41, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x12, 0x23,
    0x0a, 0x0d, 0x63, 0x61, 0x6e, 0x64, 0x79, 0x5f, 0x61, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x63, 0x61, 0x6e, 0x64, 0x79, 0x41, 0x77, 0x61, 0x72,
    0x64, 0x65, 0x64, 0x22, 0xa1, 0x01, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09,
    0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43,
    0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x1a, 0x0a, 0x16, 0x46, 0x41, 0x49, 0x4c, 0x45, 0x44,
    0x5f, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x4d, 0x49, 0x53, 0x53, 0x49, 0x4e, 0x47,
    0x10, 0x02, 0x12, 0x21, 0x0a, 0x1d, 0x46, 0x41, 0x49, 0x4c, 0x45, 0x44, 0x5f, 0x49, 0x4e, 0x53,
    0x55, 0x46, 0x46, 0x49, 0x43, 0x49, 0x45, 0x4e, 0x54, 0x5f, 0x52, 0x45, 0x53, 0x4f, 0x55, 0x52,
    0x43, 0x45, 0x53, 0x10, 0x03, 0x12, 0x20, 0x0a, 0x1c, 0x46, 0x41, 0x49, 0x4c, 0x45, 0x44, 0x5f,
    0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x43, 0x41, 0x4e, 0x4e, 0x4f, 0x54, 0x5f, 0x45,
    0x56, 0x4f, 0x4c, 0x56, 0x45, 0x10, 0x04, 0x12, 0x1e, 0x0a, 0x1a, 0x46, 0x41, 0x49, 0x4c, 0x45,
    0x44, 0x5f, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x49, 0x53, 0x5f, 0x44, 0x45, 0x50,
    0x4c, 0x4f, 0x59, 0x45, 0x44, 0x10, 0x05, 0x22, 0x8f, 0x02, 0x0a, 0x16, 0x52, 0x65, 0x6c, 0x65,
    0x61, 0x73, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x56, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x3e, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x50, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x61,
    0x6e, 0x64, 0x79, 0x5f, 0x61, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x0c, 0x63, 0x61, 0x6e, 0x64, 0x79, 0x41, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x22,
    0x78, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53,
    0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10,
    0x01, 0x12, 0x14, 0x0a, 0x10, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x44, 0x45, 0x50,
    0x4c, 0x4f, 0x59, 0x45, 0x44, 0x10, 0x02, 0x12, 0x0a, 0x0a, 0x06, 0x46, 0x41, 0x49, 0x4c, 0x45,
    0x44, 0x10, 0x03, 0x12, 0x18, 0x0a, 0x14, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4f, 0x4b,
    0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x49, 0x53, 0x5f, 0x45, 0x47, 0x47, 0x10, 0x04, 0x12, 0x1a, 0x0a,
    0x16, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x49,
    0x53, 0x5f, 0x42, 0x55, 0x44, 0x44, 0x59, 0x10, 0x05, 0x22, 0xeb, 0x01, 0x0a, 0x18, 0x47, 0x65,
    0x74, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x58, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x40, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x50, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x50, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x03, 0x52, 0x09, 0x73, 0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x12,
    0x34, 0x0a, 0x06, 0x62, 0x61, 0x64, 0x67, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x1c, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x42, 0x61, 0x64, 0x67, 0x65, 0x52, 0x06, 0x62,
    0x61, 0x64, 0x67, 0x65, 0x73, 0x22, 0x20, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12,
    0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55,
    0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x22, 0x28, 0x0a, 0x0c, 0x45, 0x63, 0x68, 0x6f, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65,
    0x78, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78,
    0x74, 0x22, 0x9f, 0x02, 0x0a, 0x12, 0x55, 0x73, 0x65, 0x49, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x52, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3a, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67,
    0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x55, 0x73, 0x65, 0x49, 0x6e,
    0x63, 0x65, 0x6e, 0x73, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x4a, 0x0a, 0x0f,
    0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x5f, 0x69, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x41, 0x70, 0x70,
    0x6c, 0x69, 0x65, 0x64, 0x49, 0x74, 0x65, 0x6d, 0x52, 0x0e, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65,
    0x64, 0x49, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x22, 0x69, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x0b, 0x0a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12,
    0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x1a, 0x0a, 0x16,
    0x49, 0x4e, 0x43, 0x45, 0x4e, 0x53, 0x45, 0x5f, 0x41, 0x4c, 0x52, 0x45, 0x41, 0x44, 0x59, 0x5f,
    0x41, 0x43, 0x54, 0x49, 0x56, 0x45, 0x10, 0x02, 0x12, 0x15, 0x0a, 0x11, 0x4e, 0x4f, 0x4e, 0x45,
    0x5f, 0x49, 0x4e, 0x5f, 0x49, 0x4e, 0x56, 0x45, 0x4e, 0x54, 0x4f, 0x52, 0x59, 0x10, 0x03, 0x12,
    0x12, 0x0a, 0x0e, 0x4c, 0x4f, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x55, 0x4e, 0x53, 0x45,
    0x54, 0x10, 0x04, 0x22, 0xe9, 0x04, 0x0a, 0x11, 0x45, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65,
    0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x46, 0x0a, 0x0c, 0x77, 0x69, 0x6c,
    0x64, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x23, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70,
    0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x2e, 0x57, 0x69, 0x6c, 0x64, 0x50, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x0b, 0x77, 0x69, 0x6c, 0x64, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f,
    0x6e, 0x12, 0x5d, 0x0a, 0x0a, 0x62, 0x61, 0x63, 0x6b, 0x67, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x45, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65,
    0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x42, 0x61, 0x63, 0x6b, 0x67, 0x72,
    0x6f, 0x75, 0x6e, 0x64, 0x52, 0x0a, 0x62, 0x61, 0x63, 0x6b, 0x67, 0x72, 0x6f, 0x75, 0x6e, 0x64,
    0x12, 0x51, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x39, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65,
    0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x73, 0x2e, 0x45, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x06, 0x73, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x12, 0x5c, 0x0a, 0x13, 0x63, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x5f, 0x70,
    0x72, 0x6f, 0x62, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x2b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61,
    0x74, 0x61, 0x2e, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x43, 0x61, 0x70, 0x74, 0x75,
    0x72, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x52, 0x12, 0x63,
    0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74,
    0x79, 0x22, 0x22, 0x0a, 0x0a, 0x42, 0x61, 0x63, 0x6b, 0x67, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x12,
    0x08, 0x0a, 0x04, 0x50, 0x41, 0x52, 0x4b, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06, 0x44, 0x45, 0x53,
    0x45, 0x52, 0x54, 0x10, 0x01, 0x22, 0xd7, 0x01, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x12, 0x13, 0x0a, 0x0f, 0x45, 0x4e, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x45, 0x52, 0x5f, 0x45, 0x52,
    0x52, 0x4f, 0x52, 0x10, 0x00, 0x12, 0x15, 0x0a, 0x11, 0x45, 0x4e, 0x43, 0x4f, 0x55, 0x4e, 0x54,
    0x45, 0x52, 0x5f, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x13,
    0x45, 0x4e, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x45, 0x52, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x46, 0x4f,
    0x55, 0x4e, 0x44, 0x10, 0x02, 0x12, 0x14, 0x0a, 0x10, 0x45, 0x4e, 0x43, 0x4f, 0x55, 0x4e, 0x54,
    0x45, 0x52, 0x5f, 0x43, 0x4c, 0x4f, 0x53, 0x45, 0x44, 0x10, 0x03, 0x12, 0x1a, 0x0a, 0x16, 0x45,
    0x4e, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x45, 0x52, 0x5f, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e,
    0x5f, 0x46, 0x4c, 0x45, 0x44, 0x10, 0x04, 0x12, 0x1a, 0x0a, 0x16, 0x45, 0x4e, 0x43, 0x4f, 0x55,
    0x4e, 0x54, 0x45, 0x52, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x49, 0x4e, 0x5f, 0x52, 0x41, 0x4e, 0x47,
    0x45, 0x10, 0x05, 0x12, 0x1e, 0x0a, 0x1a, 0x45, 0x4e, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x45, 0x52,
    0x5f, 0x41, 0x4c, 0x52, 0x45, 0x41, 0x44, 0x59, 0x5f, 0x48, 0x41, 0x50, 0x50, 0x45, 0x4e, 0x45,
    0x44, 0x10, 0x06, 0x12, 0x1a, 0x0a, 0x16, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x49,
    0x4e, 0x56, 0x45, 0x4e, 0x54, 0x4f, 0x52, 0x59, 0x5f, 0x46, 0x55, 0x4c, 0x4c, 0x10, 0x07, 0x22,
    0xa1, 0x03, 0x0a, 0x15, 0x43, 0x6c, 0x61, 0x69, 0x6d, 0x43, 0x6f, 0x64, 0x65, 0x6e, 0x61, 0x6d,
    0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x63, 0x6f, 0x64,
    0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63, 0x6f, 0x64,
    0x65, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x75, 0x73, 0x65,
    0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x23, 0x0a, 0x0d, 0x69, 0x73, 0x5f, 0x61,
    0x73, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x0c, 0x69, 0x73, 0x41, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x12, 0x55, 0x0a,
    0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x3d, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f,
    0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e,
    0x43, 0x6c, 0x61, 0x69, 0x6d, 0x43, 0x6f, 0x64, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x06, 0x73, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x12, 0x42, 0x0a, 0x0e, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x5f,
    0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x44, 0x61, 0x74, 0x61, 0x52, 0x0d, 0x75, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x64, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x22, 0x88, 0x01, 0x0a, 0x06, 0x53, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b,
    0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x1a, 0x0a, 0x16, 0x43,
    0x4f, 0x44, 0x45, 0x4e, 0x41, 0x4d, 0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x41, 0x56, 0x41, 0x49,
    0x4c, 0x41, 0x42, 0x4c, 0x45, 0x10, 0x02, 0x12, 0x16, 0x0a, 0x12, 0x43, 0x4f, 0x44, 0x45, 0x4e,
    0x41, 0x4d, 0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x10, 0x03, 0x12,
    0x11, 0x0a, 0x0d, 0x43, 0x55, 0x52, 0x52, 0x45, 0x4e, 0x54, 0x5f, 0x4f, 0x57, 0x4e, 0x45, 0x52,
    0x10, 0x04, 0x12, 0x1f, 0x0a, 0x1b, 0x43, 0x4f, 0x44, 0x45, 0x4e, 0x41, 0x4d, 0x45, 0x5f, 0x43,
    0x48, 0x41, 0x4e, 0x47, 0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x41, 0x4c, 0x4c, 0x4f, 0x57, 0x45,
    0x44, 0x10, 0x05, 0x4a, 0x8a, 0xd1, 0x01, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xf6, 0x04, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x01, 0x08, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x00, 0x12, 0x03, 0x03, 0x07, 0x0d, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x0e, 0x25, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x01,
    0x12, 0x03, 0x04, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x0e, 0x2c,
    0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x05, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x05, 0x0e, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x06, 0x07,
    0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x06, 0x0e, 0x2a, 0x0a, 0x09, 0x0a, 0x02,
    0x0a, 0x04, 0x12, 0x03, 0x07, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x07,
    0x0e, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x08, 0x07, 0x0d, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x08, 0x0e, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x06, 0x12, 0x03,
    0x09, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x06, 0x12, 0x03, 0x09, 0x0e, 0x2c, 0x0a, 0x09,
    0x0a, 0x02, 0x0a, 0x07, 0x12, 0x03, 0x0a, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x07, 0x12,
    0x03, 0x0a, 0x0e, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x08, 0x12, 0x03, 0x0b, 0x07, 0x0d, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x08, 0x12, 0x03, 0x0b, 0x0e, 0x2d, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x09,
    0x12, 0x03, 0x0c, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x09, 0x12, 0x03, 0x0c, 0x0e, 0x2c,
    0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x0a, 0x12, 0x03, 0x0d, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x0a, 0x12, 0x03, 0x0d, 0x0e, 0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x0b, 0x12, 0x03, 0x0e, 0x07,
    0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x0b, 0x12, 0x03, 0x0e, 0x0e, 0x2a, 0x0a, 0x09, 0x0a, 0x02,
    0x0a, 0x0c, 0x12, 0x03, 0x0f, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x0c, 0x12, 0x03, 0x0f,
    0x0e, 0x30, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x0d, 0x12, 0x03, 0x10, 0x07, 0x0d, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x0d, 0x12, 0x03, 0x10, 0x0e, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x12, 0x00, 0x15, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x12, 0x08, 0x19,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x13, 0x08, 0x19, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x13, 0x08, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x13, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x13, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x14, 0x08, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x14, 0x08,
    0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x14, 0x08, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x24, 0x2f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x32, 0x33, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x01, 0x12, 0x04, 0x16, 0x00, 0x22, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12,
    0x03, 0x16, 0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x18, 0x08,
    0x53, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x18, 0x08, 0x17, 0x01,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x18, 0x08, 0x47, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x48, 0x4e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x51, 0x52, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x01, 0x12, 0x03, 0x19, 0x08, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x19, 0x08, 0x18, 0x53, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x19, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x19,
    0x26, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x19, 0x36, 0x37,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x04, 0x00, 0x12, 0x04, 0x1b, 0x08, 0x21, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x10, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x10, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1c, 0x18, 0x19, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x01, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x10, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1d, 0x10, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x1d, 0x1a, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1e, 0x10, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x10, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1e, 0x29, 0x2a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x1f, 0x10, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x1f, 0x10, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00,
    0x02, 0x03, 0x02, 0x12, 0x03, 0x1f, 0x2a, 0x2b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x20, 0x10, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x20, 0x10, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02,
    0x04, 0x02, 0x12, 0x03, 0x20, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x23,
    0x00, 0x29, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x23, 0x08, 0x1e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x24, 0x08, 0x19, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x24, 0x08, 0x23, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x24, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x24, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x24, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x25,
    0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x25, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x25, 0x11, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x25, 0x19, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x25, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x08, 0x12, 0x03, 0x25, 0x28, 0x35, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x01,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x25, 0x29, 0x34, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02,
    0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x25, 0x29, 0x2f, 0x0a, 0x11, 0x0a, 0x0a, 0x04,
    0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x25, 0x29, 0x2f, 0x0a, 0x12,
    0x0a, 0x0b, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x25,
    0x29, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x03, 0x25, 0x30, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x26, 0x08,
    0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x26, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x26, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x17, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x26, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x27, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x27, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x27,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x27, 0x17, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x27, 0x27, 0x28, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x28, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x28, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x28, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x28, 0x17, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x28, 0x2a, 0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x2a, 0x00, 0x37, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x08, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x2b, 0x08, 0x2a, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x2b, 0x08, 0x41, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x2b, 0x42, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x4b,
    0x4c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x39, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x04, 0x2c, 0x08, 0x2b, 0x4d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x2c, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2c, 0x2a, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x2c, 0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12,
    0x03, 0x2d, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x04, 0x2d,
    0x08, 0x2c, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2d, 0x08,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x0f, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2d, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x2e, 0x08, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x03, 0x04, 0x12, 0x04, 0x2e, 0x08, 0x2d, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x2e, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x2e, 0x32, 0x41, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x2e, 0x44, 0x45, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x2f, 0x08, 0x46,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2f, 0x08, 0x2e, 0x46, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x06, 0x12, 0x03, 0x2f, 0x08, 0x31, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2f, 0x32, 0x41, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2f, 0x44, 0x45, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x04,
    0x00, 0x12, 0x04, 0x31, 0x04, 0x36, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x31, 0x09, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x32, 0x10, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x32, 0x10, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x32, 0x18, 0x19, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x33, 0x10, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x33, 0x10, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x33, 0x1a, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x34,
    0x10, 0x31, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x34,
    0x10, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x34,
    0x2f, 0x30, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x35, 0x10,
    0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x35, 0x10,
    0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x35, 0x25,
    0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x38, 0x00, 0x3d, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x38, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x00, 0x12, 0x03, 0x3a, 0x08, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x3a, 0x08, 0x39, 0x01, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x3a, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3a, 0x0d,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3a, 0x17, 0x18, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x3b, 0x08, 0x3e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x04, 0x3b, 0x08, 0x3a, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x3b, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x3b, 0x2a, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x3b, 0x3c, 0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x3c,
    0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x04, 0x3c, 0x08, 0x3b,
    0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3c, 0x08, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3c, 0x0e, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3c, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x05, 0x12, 0x04, 0x3e, 0x00, 0x4a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03,
    0x3e, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x08, 0x52,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x3f, 0x08, 0x3e, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3f, 0x08, 0x46, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3f, 0x47, 0x4d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3f, 0x50, 0x51, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x01, 0x12, 0x03, 0x40, 0x08, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x40, 0x08, 0x3f, 0x52, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x40, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x40, 0x2b,
    0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x40, 0x3b, 0x3c, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x05, 0x04, 0x00, 0x12, 0x04, 0x42, 0x08, 0x49, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x04, 0x00, 0x01, 0x12, 0x03, 0x42, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x43, 0x10, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x10, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x43, 0x18, 0x19, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x44, 0x10, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x44, 0x10, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x44, 0x1a, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x45, 0x10, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x45, 0x10, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x45, 0x2a, 0x2b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x46, 0x10, 0x32, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x46, 0x10, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x02, 0x12, 0x03, 0x46, 0x30, 0x31, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x12, 0x03, 0x47, 0x10, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x47, 0x10, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x02, 0x12, 0x03, 0x47, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x12, 0x03, 0x48, 0x10, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x48, 0x10, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00, 0x02, 0x05, 0x02,
    0x12, 0x03, 0x48, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x4b, 0x00, 0x59,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x4b, 0x08, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x4c, 0x08, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x4c, 0x08, 0x4b, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x4c, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x4c, 0x0f, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x4c, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x4d, 0x08, 0x33,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x04, 0x4d, 0x08, 0x4c, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x4d, 0x08, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4d, 0x24, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4d, 0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x02, 0x12, 0x03, 0x4e, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x4e, 0x08, 0x4d, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x4e, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4e, 0x25,
    0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4e, 0x34, 0x35, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x4f, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x04, 0x4f, 0x08, 0x4e, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x03, 0x05, 0x12, 0x03, 0x4f, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x4f, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x4f, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03, 0x50,
    0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x03, 0x50, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x03, 0x50, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x50, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03, 0x50, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x05, 0x12, 0x03, 0x51, 0x08, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x04,
    0x12, 0x04, 0x51, 0x08, 0x50, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x05, 0x12,
    0x03, 0x51, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12, 0x03, 0x51,
    0x0e, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03, 0x51, 0x13, 0x14,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x06, 0x12, 0x03, 0x52, 0x08, 0x1a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x06, 0x04, 0x12, 0x04, 0x52, 0x08, 0x51, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x06, 0x05, 0x12, 0x03, 0x52, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x52, 0x0e, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06,
    0x03, 0x12, 0x03, 0x52, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x07, 0x12, 0x03,
    0x53, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x04, 0x12, 0x04, 0x53, 0x08,
    0x52, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x05, 0x12, 0x03, 0x53, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x01, 0x12, 0x03, 0x53, 0x0e, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x03, 0x12, 0x03, 0x53, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x08, 0x12, 0x03, 0x54, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x08, 0x04, 0x12, 0x04, 0x54, 0x08, 0x53, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08,
    0x06, 0x12, 0x03, 0x54, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x54, 0x26, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x03, 0x12, 0x03, 0x54,
    0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x09, 0x12, 0x03, 0x55, 0x08, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x04, 0x12, 0x04, 0x55, 0x08, 0x54, 0x2f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x05, 0x12, 0x03, 0x55, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x09, 0x01, 0x12, 0x03, 0x55, 0x0f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x09, 0x03, 0x12, 0x03, 0x55, 0x1a, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x0a,
    0x12, 0x03, 0x56, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x04, 0x12, 0x04,
    0x56, 0x08, 0x55, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x56,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x56, 0x0f, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x56, 0x1b, 0x1d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x0b, 0x12, 0x03, 0x57, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x0b, 0x04, 0x12, 0x04, 0x57, 0x08, 0x56, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x0b, 0x05, 0x12, 0x03, 0x57, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b,
    0x01, 0x12, 0x03, 0x57, 0x0f, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b, 0x03, 0x12,
    0x03, 0x57, 0x1d, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x0c, 0x12, 0x03, 0x58, 0x08,
    0x42, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x58, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0c, 0x06, 0x12, 0x03, 0x58, 0x11, 0x32, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x58, 0x33, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x58, 0x3f, 0x41, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12,
    0x04, 0x5a, 0x00, 0x5e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x5a, 0x08,
    0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x5b, 0x08, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x5b, 0x08, 0x5a, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5b, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5b, 0x0f, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x5b, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12,
    0x03, 0x5c, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04, 0x5c,
    0x08, 0x5b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5c, 0x08,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5c, 0x0f, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5c, 0x16, 0x17, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x5d, 0x08, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x5d, 0x08, 0x5c, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x02, 0x06, 0x12, 0x03, 0x5d, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x5d, 0x2c, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x5d, 0x37, 0x38, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x5f, 0x00, 0x6a, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x5f, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x00, 0x12, 0x03, 0x60, 0x08, 0x51, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x60, 0x08, 0x5f, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x60, 0x08, 0x45, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x60, 0x46, 0x4c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x60, 0x4f,
    0x50, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x61, 0x08, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x04, 0x61, 0x08, 0x60, 0x51, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x61, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x61, 0x0e, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x61, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x04, 0x00, 0x12,
    0x04, 0x63, 0x08, 0x69, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x63, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x64,
    0x10, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x64,
    0x10, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x64,
    0x18, 0x19, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x65, 0x10,
    0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x65, 0x10,
    0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x65, 0x1a,
    0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x66, 0x10, 0x25,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x66, 0x10, 0x20,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x66, 0x23, 0x24,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x67, 0x10, 0x25, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x67, 0x10, 0x20, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x67, 0x23, 0x24, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x68, 0x10, 0x2b, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x68, 0x10, 0x26, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x68, 0x29, 0x2a, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x6b, 0x00, 0x6e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09,
    0x01, 0x12, 0x03, 0x6b, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03,
    0x6c, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0x6c, 0x08,
    0x6b, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6c, 0x08, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6c, 0x0d, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6c, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x6d, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x6d, 0x08, 0x6c, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x6d, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x6d, 0x0f, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6d,
    0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x6f, 0x00, 0x72, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x6f, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a,
    0x02, 0x00, 0x12, 0x03, 0x70, 0x08, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x70, 0x08, 0x6f, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x70, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x70,
    0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x70, 0x17, 0x18,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03, 0x71, 0x08, 0x34, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x04, 0x71, 0x08, 0x70, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x01, 0x06, 0x12, 0x03, 0x71, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x71, 0x24, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x71, 0x32, 0x33, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x73, 0x00,
    0x77, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x73, 0x08, 0x1c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x74, 0x08, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x74, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x74, 0x11, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x74, 0x35, 0x42, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x74, 0x45, 0x46, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x75, 0x08, 0x39,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03, 0x75, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x06, 0x12, 0x03, 0x75, 0x11, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x75, 0x2f, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x75, 0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02,
    0x12, 0x03, 0x76, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x76, 0x08, 0x75, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x05, 0x12, 0x03, 0x76,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12, 0x03, 0x76, 0x0e, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x03, 0x76, 0x1d, 0x1e, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x78, 0x00, 0x7c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c,
    0x01, 0x12, 0x03, 0x78, 0x08, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03,
    0x79, 0x08, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0x79, 0x08,
    0x78, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x03, 0x79, 0x08, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x79, 0x0d, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x79, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x7a, 0x08, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x7a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x7a, 0x11, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x7a, 0x2d, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7a, 0x3e,
    0x3f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x03, 0x7b, 0x08, 0x30, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x03, 0x7b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x03, 0x7b, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x7b, 0x17, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x7b, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x05, 0x7d, 0x00,
    0x86, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x03, 0x7d, 0x08, 0x2b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x7e, 0x08, 0x5f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04, 0x7e, 0x08, 0x7d, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x00, 0x06, 0x12, 0x03, 0x7e, 0x08, 0x53, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x7e, 0x54, 0x5a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x7e, 0x5d, 0x5e, 0x0a, 0x20, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x03, 0x7f,
    0x08, 0x2f, 0x22, 0x13, 0x20, 0x4c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x20, 0x61, 0x76, 0x61, 0x69,
    0x6c, 0x61, 0x62, 0x6c, 0x65, 0x3f, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x7f, 0x08, 0x7e, 0x5f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x7f, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7f,
    0x0f, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7f, 0x2d, 0x2e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x02, 0x12, 0x04, 0x80, 0x01, 0x08, 0x2d, 0x0a, 0x0e,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x04, 0x12, 0x05, 0x80, 0x01, 0x08, 0x7f, 0x2f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x05, 0x12, 0x04, 0x80, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x02, 0x01, 0x12, 0x04, 0x80, 0x01, 0x0f, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x02, 0x03, 0x12, 0x04, 0x80, 0x01, 0x2b, 0x2c, 0x0a, 0x0e, 0x0a, 0x04, 0x04,
    0x0d, 0x04, 0x00, 0x12, 0x06, 0x82, 0x01, 0x08, 0x85, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x04, 0x00, 0x01, 0x12, 0x04, 0x82, 0x01, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0x83, 0x01, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x83, 0x01, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0d, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0x83, 0x01, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x0d, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0x84, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x0d, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x84, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0x84, 0x01, 0x1a, 0x1b, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0x87, 0x01, 0x00, 0x90, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x0e, 0x01, 0x12, 0x04, 0x87, 0x01, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02,
    0x00, 0x12, 0x04, 0x88, 0x01, 0x08, 0x55, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04,
    0x12, 0x06, 0x88, 0x01, 0x08, 0x87, 0x01, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00,
    0x06, 0x12, 0x04, 0x88, 0x01, 0x08, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x88, 0x01, 0x4a, 0x50, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x88, 0x01, 0x53, 0x54, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0e, 0x04, 0x00, 0x12, 0x06, 0x8a,
    0x01, 0x08, 0x8f, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x04, 0x00, 0x01, 0x12, 0x04,
    0x8a, 0x01, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0e, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04,
    0x8b, 0x01, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x8b, 0x01, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x04, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x04, 0x8b, 0x01, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0e, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x04, 0x8c, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x8c, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x04, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x04, 0x8c, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0e, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x04, 0x8d, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x04,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x8d, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0e,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x8e, 0x01, 0x10, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0e, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0x8e, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x0f, 0x12, 0x06, 0x91, 0x01, 0x00, 0x93, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f,
    0x01, 0x12, 0x04, 0x91, 0x01, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12,
    0x04, 0x92, 0x01, 0x08, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x92, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x06, 0x12, 0x04, 0x92,
    0x01, 0x11, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0x92, 0x01,
    0x33, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0x92, 0x01, 0x43,
    0x44, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0x94, 0x01, 0x00, 0xa0, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x94, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0x95, 0x01, 0x08, 0x34, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x00, 0x04, 0x12, 0x06, 0x95, 0x01, 0x08, 0x94, 0x01, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x00, 0x06, 0x12, 0x04, 0x95, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x95, 0x01, 0x26, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x00, 0x03, 0x12, 0x04, 0x95, 0x01, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01,
    0x12, 0x04, 0x96, 0x01, 0x08, 0x18, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12,
    0x06, 0x96, 0x01, 0x08, 0x95, 0x01, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05,
    0x12, 0x04, 0x96, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x96, 0x01, 0x0f, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04,
    0x96, 0x01, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x02, 0x12, 0x04, 0x97, 0x01,
    0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x04, 0x12, 0x04, 0x97, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x05, 0x12, 0x04, 0x97, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x01, 0x12, 0x04, 0x97, 0x01, 0x18, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x03, 0x12, 0x04, 0x97, 0x01, 0x1f, 0x20, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x10, 0x02, 0x03, 0x12, 0x04, 0x98, 0x01, 0x08, 0x51, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x03, 0x04, 0x12, 0x06, 0x98, 0x01, 0x08, 0x97, 0x01, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x03, 0x06, 0x12, 0x04, 0x98, 0x01, 0x08, 0x45, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x03, 0x01, 0x12, 0x04, 0x98, 0x01, 0x46, 0x4c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x03, 0x03, 0x12, 0x04, 0x98, 0x01, 0x4f, 0x50, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10,
    0x02, 0x04, 0x12, 0x04, 0x99, 0x01, 0x08, 0x1f, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04,
    0x04, 0x12, 0x06, 0x99, 0x01, 0x08, 0x98, 0x01, 0x51, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x04, 0x05, 0x12, 0x04, 0x99, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x99, 0x01, 0x0f, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x99, 0x01, 0x1d, 0x1e, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x10, 0x04, 0x00, 0x12, 0x06,
    0x9b, 0x01, 0x08, 0x9f, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x04, 0x00, 0x01, 0x12,
    0x04, 0x9b, 0x01, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x04, 0x9c, 0x01, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x9c, 0x01, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x04, 0x9c, 0x01, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x04, 0x9d, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x04, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x04, 0x9d, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x04, 0x9e, 0x01, 0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10,
    0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x9e, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x11, 0x12, 0x06, 0xa1, 0x01, 0x00, 0xa4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01,
    0x12, 0x04, 0xa1, 0x01, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04,
    0xa2, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa2,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa2, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x18,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa2, 0x01, 0x24, 0x25,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x19, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x06, 0xa3, 0x01, 0x08, 0xa2, 0x01, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x0d, 0x14, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa3, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x12, 0x12, 0x06, 0xa5, 0x01, 0x00, 0xb7, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12,
    0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12,
    0x04, 0xa6, 0x01, 0x08, 0x55, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x06,
    0xa6, 0x01, 0x08, 0xa5, 0x01, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x06, 0x12,
    0x04, 0xa6, 0x01, 0x08, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xa6, 0x01, 0x4a, 0x50, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa6,
    0x01, 0x53, 0x54, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x08,
    0x4e, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x06, 0xa7, 0x01, 0x08, 0xa6,
    0x01, 0x55, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x06, 0x12, 0x04, 0xa7, 0x01, 0x08,
    0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x3d, 0x49,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa7, 0x01, 0x4c, 0x4d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x02, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x36, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x02, 0x04, 0x12, 0x06, 0xa8, 0x01, 0x08, 0xa7, 0x01, 0x4e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x06, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x25, 0x31, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa8, 0x01, 0x34, 0x35, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x12, 0x02, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x34, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x03, 0x04, 0x12, 0x06, 0xa9, 0x01, 0x08, 0xa8, 0x01, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x03, 0x06, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x03, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x26, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x03,
    0x03, 0x12, 0x04, 0xa9, 0x01, 0x32, 0x33, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x12, 0x04, 0x00, 0x12,
    0x06, 0xab, 0x01, 0x08, 0xb6, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x04, 0x00, 0x01,
    0x12, 0x04, 0xab, 0x01, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x12, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x04, 0xac, 0x01, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xac, 0x01, 0x10, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x04, 0xac, 0x01, 0x20, 0x21, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x12, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x04, 0xad, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xad, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04,
    0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xad, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x12,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xae, 0x01, 0x10, 0x36, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xae, 0x01, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x12, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xae, 0x01, 0x34, 0x35, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x12, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xaf, 0x01, 0x10, 0x32, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x12, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x10, 0x2d, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x12, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xaf, 0x01, 0x30, 0x31, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x12, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xb0, 0x01, 0x10, 0x27, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x12, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x10, 0x22, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xb0, 0x01, 0x25, 0x26,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x12, 0x04, 0x00, 0x02, 0x05, 0x12, 0x04, 0xb1, 0x01, 0x10, 0x27,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0xb1, 0x01, 0x10,
    0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04, 0xb1, 0x01,
    0x25, 0x26, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x12, 0x04, 0x00, 0x02, 0x06, 0x12, 0x04, 0xb2, 0x01,
    0x10, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04, 0xb2,
    0x01, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x04,
    0xb2, 0x01, 0x2b, 0x2c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x12, 0x04, 0x00, 0x02, 0x07, 0x12, 0x04,
    0xb3, 0x01, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12,
    0x04, 0xb3, 0x01, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04, 0x00, 0x02, 0x07, 0x02,
    0x12, 0x04, 0xb3, 0x01, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x12, 0x04, 0x00, 0x02, 0x08,
    0x12, 0x04, 0xb4, 0x01, 0x10, 0x35, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04, 0x00, 0x02, 0x08,
    0x01, 0x12, 0x04, 0xb4, 0x01, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04, 0x00, 0x02,
    0x08, 0x02, 0x12, 0x04, 0xb4, 0x01, 0x33, 0x34, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x12, 0x04, 0x00,
    0x02, 0x09, 0x12, 0x04, 0xb5, 0x01, 0x10, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04, 0x00,
    0x02, 0x09, 0x01, 0x12, 0x04, 0xb5, 0x01, 0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04,
    0x00, 0x02, 0x09, 0x02, 0x12, 0x04, 0xb5, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13,
    0x12, 0x06, 0xb8, 0x01, 0x00, 0xc3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12,
    0x04, 0xb8, 0x01, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xb9,
    0x01, 0x08, 0x54, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x06, 0xb9, 0x01,
    0x08, 0xb8, 0x01, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb9,
    0x01, 0x08, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb9, 0x01,
    0x49, 0x4f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb9, 0x01, 0x52,
    0x53, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0xba, 0x01, 0x08, 0x36, 0x0a,
    0x0f, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x04, 0x12, 0x06, 0xba, 0x01, 0x08, 0xb9, 0x01, 0x54,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x06, 0x12, 0x04, 0xba, 0x01, 0x08, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x01, 0x12, 0x04, 0xba, 0x01, 0x25, 0x31, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03, 0x12, 0x04, 0xba, 0x01, 0x34, 0x35, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x13, 0x02, 0x02, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x4c, 0x0a, 0x0f, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x02, 0x04, 0x12, 0x06, 0xbb, 0x01, 0x08, 0xba, 0x01, 0x36, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x02, 0x06, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x34, 0x47, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xbb, 0x01, 0x4a, 0x4b, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x13, 0x04,
    0x00, 0x12, 0x06, 0xbd, 0x01, 0x08, 0xc2, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x04,
    0x00, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x13, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x04, 0xbe, 0x01, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x04,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xbe, 0x01, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x13,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x13, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xbf, 0x01, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x13, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xc0, 0x01, 0x10, 0x34, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x13, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc0, 0x01, 0x10, 0x2f, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x13, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xc0, 0x01, 0x32, 0x33, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x13, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xc1, 0x01, 0x10, 0x2b, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x13, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc1, 0x01, 0x10, 0x26, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x13, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xc1, 0x01, 0x29, 0x2a,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xc4, 0x01, 0x00, 0xc6, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x14, 0x02, 0x00, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x19, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x00, 0x04, 0x12, 0x06, 0xc5, 0x01, 0x08, 0xc4, 0x01, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xc5, 0x01, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xc5, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xc7,
    0x01, 0x00, 0xd2, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xc7, 0x01,
    0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0xc8, 0x01, 0x08, 0x51,
    0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x06, 0xc8, 0x01, 0x08, 0xc7, 0x01,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc8, 0x01, 0x08, 0x45,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc8, 0x01, 0x46, 0x4c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc8, 0x01, 0x4f, 0x50, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0xc9, 0x01, 0x08, 0x1a, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x01, 0x04, 0x12, 0x06, 0xc9, 0x01, 0x08, 0xc8, 0x01, 0x51, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc9, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc9, 0x01, 0x0e, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc9, 0x01, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x15,
    0x04, 0x00, 0x12, 0x06, 0xcb, 0x01, 0x08, 0xd1, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x04, 0x00, 0x01, 0x12, 0x04, 0xcb, 0x01, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x04, 0xcc, 0x01, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15,
    0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xcc, 0x01, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x15, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xcd, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x15, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcd, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x15, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xcd, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x15, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xce, 0x01, 0x10, 0x25, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xce, 0x01, 0x10, 0x20, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xce, 0x01, 0x23, 0x24, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xcf, 0x01, 0x10, 0x25, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x10, 0x20,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xcf, 0x01, 0x23,
    0x24, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xd0, 0x01, 0x10,
    0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xd0, 0x01,
    0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xd0,
    0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0xd3, 0x01, 0x00, 0xdd, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0xd3, 0x01, 0x08, 0x1a, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x4e, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x06, 0xd4, 0x01, 0x08, 0xd3, 0x01, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x42, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x43, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd4, 0x01, 0x4c, 0x4d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16,
    0x02, 0x01, 0x12, 0x04, 0xd5, 0x01, 0x08, 0x3b, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01,
    0x04, 0x12, 0x06, 0xd5, 0x01, 0x08, 0xd4, 0x01, 0x4e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x01, 0x06, 0x12, 0x04, 0xd5, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xd5, 0x01, 0x2e, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xd5, 0x01, 0x39, 0x3a, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x16, 0x04, 0x00, 0x12, 0x06,
    0xd7, 0x01, 0x08, 0xdc, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x04, 0x00, 0x01, 0x12,
    0x04, 0xd7, 0x01, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x16, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x04, 0xd8, 0x01, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x16, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xd8, 0x01, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x16, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x04, 0xd8, 0x01, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x16, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x04, 0xd9, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x16, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x16, 0x04, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x04, 0xd9, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x16, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x04, 0xda, 0x01, 0x10, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x16, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xda, 0x01, 0x10, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x16,
    0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xda, 0x01, 0x22, 0x23, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x16, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xdb, 0x01, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x16, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x10, 0x1d, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x16, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xdb, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x17, 0x12, 0x06, 0xde, 0x01, 0x00, 0xea, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x17, 0x01, 0x12, 0x04, 0xde, 0x01, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00,
    0x12, 0x04, 0xdf, 0x01, 0x08, 0x52, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12,
    0x06, 0xdf, 0x01, 0x08, 0xde, 0x01, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xdf, 0x01, 0x08, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xdf, 0x01, 0x47, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xdf, 0x01, 0x50, 0x51, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x01, 0x12, 0x04, 0xe0, 0x01,
    0x08, 0x3a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x04, 0x12, 0x06, 0xe0, 0x01, 0x08,
    0xdf, 0x01, 0x52, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x06, 0x12, 0x04, 0xe0, 0x01,
    0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x25,
    0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe0, 0x01, 0x38, 0x39,
    0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x17, 0x04, 0x00, 0x12, 0x06, 0xe2, 0x01, 0x08, 0xe9, 0x01, 0x09,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x04, 0x00, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x0d, 0x13, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x17, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xe3, 0x01, 0x10, 0x1a, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe3, 0x01, 0x10, 0x15,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xe3, 0x01, 0x18,
    0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x17, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xe4, 0x01, 0x10,
    0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe4, 0x01,
    0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xe4,
    0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x17, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xe5,
    0x01, 0x10, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04,
    0xe5, 0x01, 0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x04, 0xe5, 0x01, 0x2a, 0x2b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x17, 0x04, 0x00, 0x02, 0x03, 0x12,
    0x04, 0xe6, 0x01, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x04, 0xe6, 0x01, 0x10, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x04, 0xe6, 0x01, 0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x17, 0x04, 0x00, 0x02,
    0x04, 0x12, 0x04, 0xe7, 0x01, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02,
    0x04, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x10, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00,
    0x02, 0x04, 0x02, 0x12, 0x04, 0xe7, 0x01, 0x2e, 0x2f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x17, 0x04,
    0x00, 0x02, 0x05, 0x12, 0x04, 0xe8, 0x01, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04,
    0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0xe8, 0x01, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17,
    0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04, 0xe8, 0x01, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x18, 0x12, 0x06, 0xeb, 0x01, 0x00, 0xfc, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01,
    0x12, 0x04, 0xeb, 0x01, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04,
    0xec, 0x01, 0x08, 0x4e, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x06, 0xec,
    0x01, 0x08, 0xeb, 0x01, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xec, 0x01, 0x08, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12, 0x04, 0xec,
    0x01, 0x43, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x04, 0xec, 0x01,
    0x4c, 0x4d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x01, 0x12, 0x04, 0xed, 0x01, 0x08, 0x48,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x04, 0x12, 0x04, 0xed, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x06, 0x12, 0x04, 0xed, 0x01, 0x11, 0x35, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12, 0x04, 0xed, 0x01, 0x36, 0x43, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x04, 0xed, 0x01, 0x46, 0x47, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x18, 0x02, 0x02, 0x12, 0x04, 0xee, 0x01, 0x08, 0x1f, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x02, 0x04, 0x12, 0x06, 0xee, 0x01, 0x08, 0xed, 0x01, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x02, 0x05, 0x12, 0x04, 0xee, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xee, 0x01, 0x0e, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xee, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x03,
    0x12, 0x04, 0xef, 0x01, 0x08, 0x3a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x03, 0x04, 0x12,
    0x06, 0xef, 0x01, 0x08, 0xee, 0x01, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x03, 0x06,
    0x12, 0x04, 0xef, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xef, 0x01, 0x25, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x03, 0x03, 0x12, 0x04,
    0xef, 0x01, 0x38, 0x39, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x04, 0x12, 0x04, 0xf0, 0x01,
    0x08, 0x25, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x04, 0x12, 0x06, 0xf0, 0x01, 0x08,
    0xef, 0x01, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x05, 0x12, 0x04, 0xf0, 0x01,
    0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x01, 0x12, 0x04, 0xf0, 0x01, 0x0e,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x03, 0x12, 0x04, 0xf0, 0x01, 0x23, 0x24,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x05, 0x12, 0x04, 0xf1, 0x01, 0x08, 0x31, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x18, 0x02, 0x05, 0x04, 0x12, 0x06, 0xf1, 0x01, 0x08, 0xf0, 0x01, 0x25, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x05, 0x05, 0x12, 0x04, 0xf1, 0x01, 0x08, 0x0d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x18, 0x02, 0x05, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x0e, 0x2c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x05, 0x03, 0x12, 0x04, 0xf1, 0x01, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x18, 0x02, 0x06, 0x12, 0x04, 0xf2, 0x01, 0x08, 0x2d, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x06, 0x04, 0x12, 0x06, 0xf2, 0x01, 0x08, 0xf1, 0x01, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x06, 0x05, 0x12, 0x04, 0xf2, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x06, 0x01, 0x12, 0x04, 0xf2, 0x01, 0x0e, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x06, 0x03, 0x12, 0x04, 0xf2, 0x01, 0x2b, 0x2c, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x18, 0x04, 0x00,
    0x12, 0x06, 0xf4, 0x01, 0x08, 0xfb, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x04, 0x00,
    0x01, 0x12, 0x04, 0xf4, 0x01, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x18, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x04, 0xf5, 0x01, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xf5, 0x01, 0x10, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x04, 0xf5, 0x01, 0x20, 0x21, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x18, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xf6, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x18, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xf7, 0x01, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x18, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x18, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xf7, 0x01, 0x1f, 0x20, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x18, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xf8, 0x01, 0x10, 0x27, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x18, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x10, 0x22, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x18, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xf8, 0x01, 0x25, 0x26, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x18, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xf9, 0x01, 0x10, 0x23, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x18, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xf9, 0x01, 0x10, 0x1e,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xf9, 0x01, 0x21,
    0x22, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x18, 0x04, 0x00, 0x02, 0x05, 0x12, 0x04, 0xfa, 0x01, 0x10,
    0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0xfa, 0x01,
    0x10, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04, 0xfa,
    0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xfd, 0x01, 0x00, 0x86, 0x02,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x08, 0x22, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0xfe, 0x01, 0x08, 0x56, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x06, 0xfe, 0x01, 0x08, 0xfd, 0x01, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x00, 0x06, 0x12, 0x04, 0xfe, 0x01, 0x08, 0x4a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfe, 0x01, 0x4b, 0x51, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfe, 0x01, 0x54, 0x55, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x19,
    0x04, 0x00, 0x12, 0x06, 0x80, 0x02, 0x08, 0x85, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x04, 0x00, 0x01, 0x12, 0x04, 0x80, 0x02, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x19, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x04, 0x81, 0x02, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x81, 0x02, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19,
    0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0x81, 0x02, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x19, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0x82, 0x02, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x19, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x82, 0x02, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x19, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0x82, 0x02, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x19, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x83, 0x02, 0x10, 0x2c, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x19, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x83, 0x02, 0x10, 0x27, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x19, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x83, 0x02, 0x2a, 0x2b, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x19, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x84, 0x02, 0x10, 0x29, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x19, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x84, 0x02, 0x10, 0x24,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0x84, 0x02, 0x27,
    0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06, 0x87, 0x02, 0x00, 0x91, 0x02, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0x87, 0x02, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0x88, 0x02, 0x08, 0x4e, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x00, 0x04, 0x12, 0x06, 0x88, 0x02, 0x08, 0x87, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x00, 0x06, 0x12, 0x04, 0x88, 0x02, 0x08, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x88, 0x02, 0x43, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x00, 0x03, 0x12, 0x04, 0x88, 0x02, 0x4c, 0x4d, 0x0a, 0x1e, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x01,
    0x12, 0x04, 0x89, 0x02, 0x08, 0x1d, 0x22, 0x10, 0x20, 0x47, 0x79, 0x6d, 0x20, 0x50, 0x6f, 0x69,
    0x6e, 0x74, 0x73, 0x20, 0x28, 0x3f, 0x29, 0x0a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01,
    0x04, 0x12, 0x06, 0x89, 0x02, 0x08, 0x88, 0x02, 0x4e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x01, 0x05, 0x12, 0x04, 0x89, 0x02, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x89, 0x02, 0x0e, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x89, 0x02, 0x1b, 0x1c, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1a, 0x04, 0x00, 0x12, 0x06,
    0x8b, 0x02, 0x08, 0x90, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x04, 0x00, 0x01, 0x12,
    0x04, 0x8b, 0x02, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x04, 0x8c, 0x02, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x8c, 0x02, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x04, 0x8c, 0x02, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x04, 0x8d, 0x02, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x8d, 0x02, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x04, 0x8d, 0x02, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x04, 0x8e, 0x02, 0x10, 0x25, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8e, 0x02, 0x10, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a,
    0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x8e, 0x02, 0x23, 0x24, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x1a, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x8f, 0x02, 0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1a, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8f, 0x02, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1a, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0x8f, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x1b, 0x12, 0x06, 0x92, 0x02, 0x00, 0x9f, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x1b, 0x01, 0x12, 0x04, 0x92, 0x02, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00,
    0x12, 0x04, 0x93, 0x02, 0x08, 0x55, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x04, 0x12,
    0x06, 0x93, 0x02, 0x08, 0x92, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x06,
    0x12, 0x04, 0x93, 0x02, 0x08, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x93, 0x02, 0x4a, 0x50, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x93, 0x02, 0x53, 0x54, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x01, 0x12, 0x04, 0x94, 0x02,
    0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x04, 0x12, 0x06, 0x94, 0x02, 0x08,
    0x93, 0x02, 0x55, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x05, 0x12, 0x04, 0x94, 0x02,
    0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x01, 0x12, 0x04, 0x94, 0x02, 0x0f,
    0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x03, 0x12, 0x04, 0x94, 0x02, 0x1e, 0x1f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x02, 0x12, 0x04, 0x95, 0x02, 0x08, 0x28, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x04, 0x12, 0x06, 0x95, 0x02, 0x08, 0x94, 0x02, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x05, 0x12, 0x04, 0x95, 0x02, 0x08, 0x0f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x01, 0x12, 0x04, 0x95, 0x02, 0x10, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x02, 0x03, 0x12, 0x04, 0x95, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1b, 0x02, 0x03, 0x12, 0x04, 0x96, 0x02, 0x08, 0x40, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x03, 0x04, 0x12, 0x06, 0x96, 0x02, 0x08, 0x95, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x03, 0x06, 0x12, 0x04, 0x96, 0x02, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x03, 0x01, 0x12, 0x04, 0x96, 0x02, 0x2e, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02,
    0x03, 0x03, 0x12, 0x04, 0x96, 0x02, 0x3e, 0x3f, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1b, 0x04, 0x00,
    0x12, 0x06, 0x98, 0x02, 0x08, 0x9e, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x04, 0x00,
    0x01, 0x12, 0x04, 0x98, 0x02, 0x0d, 0x18, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1b, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x04, 0x99, 0x02, 0x10, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x99, 0x02, 0x10, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x04, 0x99, 0x02, 0x1e, 0x1f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1b, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x04, 0x9a, 0x02, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9a, 0x02, 0x10, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0x9a, 0x02, 0x20, 0x21, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x1b, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x9b, 0x02, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1b, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x9b, 0x02, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1b, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x9b, 0x02, 0x1f, 0x20, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x9c, 0x02, 0x10, 0x1f, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x9c, 0x02, 0x10, 0x1a, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0x9c, 0x02, 0x1d, 0x1e, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0x9d, 0x02, 0x10, 0x21, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0x9d, 0x02, 0x10, 0x1c,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0x9d, 0x02, 0x1f,
    0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1c, 0x12, 0x06, 0xa0, 0x02, 0x00, 0xa8, 0x02, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0xa0, 0x02, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0xa1, 0x02, 0x08, 0x52, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x00, 0x04, 0x12, 0x06, 0xa1, 0x02, 0x08, 0xa0, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa1, 0x02, 0x08, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xa1, 0x02, 0x47, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xa1, 0x02, 0x50, 0x51, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x01,
    0x12, 0x04, 0xa2, 0x02, 0x08, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xa2, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x06, 0x12, 0x04,
    0xa2, 0x02, 0x11, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa2,
    0x02, 0x36, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa2, 0x02,
    0x44, 0x45, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1c, 0x04, 0x00, 0x12, 0x06, 0xa4, 0x02, 0x08, 0xa7,
    0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x04, 0x00, 0x01, 0x12, 0x04, 0xa4, 0x02, 0x0d,
    0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1c, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xa5, 0x02, 0x10,
    0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa5, 0x02,
    0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xa5,
    0x02, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1c, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xa6,
    0x02, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xa6, 0x02, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x04, 0xa6, 0x02, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1d, 0x12, 0x06, 0xa9, 0x02, 0x00,
    0xb1, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04, 0xa9, 0x02, 0x08, 0x1e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x00, 0x12, 0x04, 0xaa, 0x02, 0x08, 0x19, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12, 0x06, 0xaa, 0x02, 0x08, 0xa9, 0x02, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x05, 0x12, 0x04, 0xaa, 0x02, 0x08, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xaa, 0x02, 0x0d, 0x14, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xaa, 0x02, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1d, 0x02, 0x01, 0x12, 0x04, 0xab, 0x02, 0x08, 0x25, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x01, 0x04, 0x12, 0x06, 0xab, 0x02, 0x08, 0xaa, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x01, 0x05, 0x12, 0x04, 0xab, 0x02, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xab, 0x02, 0x0f, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xab, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x02,
    0x12, 0x04, 0xac, 0x02, 0x08, 0x22, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x02, 0x04, 0x12,
    0x06, 0xac, 0x02, 0x08, 0xab, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xac, 0x02, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xac, 0x02, 0x0f, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xac, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x03, 0x12, 0x04, 0xad, 0x02,
    0x08, 0x1f, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x03, 0x04, 0x12, 0x06, 0xad, 0x02, 0x08,
    0xac, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x03, 0x05, 0x12, 0x04, 0xad, 0x02,
    0x08, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x03, 0x01, 0x12, 0x04, 0xad, 0x02, 0x0d,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x03, 0x03, 0x12, 0x04, 0xad, 0x02, 0x1d, 0x1e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x04, 0x12, 0x04, 0xae, 0x02, 0x08, 0x1d, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x04, 0x04, 0x12, 0x06, 0xae, 0x02, 0x08, 0xad, 0x02, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x04, 0x05, 0x12, 0x04, 0xae, 0x02, 0x08, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x04, 0x01, 0x12, 0x04, 0xae, 0x02, 0x0d, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x04, 0x03, 0x12, 0x04, 0xae, 0x02, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1d, 0x02, 0x05, 0x12, 0x04, 0xaf, 0x02, 0x08, 0x1c, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x05, 0x04, 0x12, 0x06, 0xaf, 0x02, 0x08, 0xae, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x05, 0x05, 0x12, 0x04, 0xaf, 0x02, 0x08, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x05, 0x01, 0x12, 0x04, 0xaf, 0x02, 0x0d, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x05, 0x03, 0x12, 0x04, 0xaf, 0x02, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x06,
    0x12, 0x04, 0xb0, 0x02, 0x08, 0x1d, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x06, 0x04, 0x12,
    0x06, 0xb0, 0x02, 0x08, 0xaf, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x06, 0x05,
    0x12, 0x04, 0xb0, 0x02, 0x08, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x06, 0x01, 0x12,
    0x04, 0xb0, 0x02, 0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x06, 0x03, 0x12, 0x04,
    0xb0, 0x02, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0xb2, 0x02, 0x00, 0xc9,
    0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12, 0x04, 0xb2, 0x02, 0x08, 0x25, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x00, 0x12, 0x04, 0xb3, 0x02, 0x08, 0x19, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x1e, 0x02, 0x00, 0x04, 0x12, 0x06, 0xb3, 0x02, 0x08, 0xb2, 0x02, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb3, 0x02, 0x08, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb3, 0x02, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb3, 0x02, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1e, 0x02, 0x01, 0x12, 0x04, 0xb4, 0x02, 0x08, 0x70, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xb4, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01,
    0x06, 0x12, 0x04, 0xb4, 0x02, 0x11, 0x5c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xb4, 0x02, 0x5d, 0x6b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xb4, 0x02, 0x6e, 0x6f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x02, 0x12, 0x04, 0xb5,
    0x02, 0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x04, 0x12, 0x06, 0xb5, 0x02,
    0x08, 0xb4, 0x02, 0x70, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x05, 0x12, 0x04, 0xb5,
    0x02, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb5, 0x02,
    0x0f, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb5, 0x02, 0x1e,
    0x1f, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1e, 0x03, 0x00, 0x12, 0x06, 0xb7, 0x02, 0x08, 0xc8, 0x02,
    0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x03, 0x00, 0x01, 0x12, 0x04, 0xb7, 0x02, 0x10, 0x1c,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xb8, 0x02, 0x10, 0x27,
    0x0a, 0x11, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x06, 0xb8, 0x02, 0x10,
    0xb7, 0x02, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xb8, 0x02, 0x10, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xb8, 0x02, 0x17, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xb8, 0x02, 0x25, 0x26, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x01,
    0x12, 0x04, 0xb9, 0x02, 0x10, 0x51, 0x0a, 0x11, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x06, 0xb9, 0x02, 0x10, 0xb8, 0x02, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03,
    0x00, 0x02, 0x01, 0x06, 0x12, 0x04, 0xb9, 0x02, 0x10, 0x3b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e,
    0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb9, 0x02, 0x3c, 0x4c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1e, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb9, 0x02, 0x4f, 0x50, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x1e, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0xba, 0x02, 0x10, 0x4b, 0x0a, 0x11, 0x0a, 0x07,
    0x04, 0x1e, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x06, 0xba, 0x02, 0x10, 0xb9, 0x02, 0x51, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x02, 0x06, 0x12, 0x04, 0xba, 0x02, 0x10, 0x38,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xba, 0x02, 0x39,
    0x46, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0xba, 0x02,
    0x49, 0x4a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x03, 0x12, 0x04, 0xbb, 0x02,
    0x10, 0x4b, 0x0a, 0x11, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x06, 0xbb,
    0x02, 0x10, 0xba, 0x02, 0x4b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x03, 0x06,
    0x12, 0x04, 0xbb, 0x02, 0x10, 0x38, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xbb, 0x02, 0x39, 0x46, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02,
    0x03, 0x03, 0x12, 0x04, 0xbb, 0x02, 0x49, 0x4a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00,
    0x02, 0x04, 0x12, 0x04, 0xbc, 0x02, 0x10, 0x5c, 0x0a, 0x11, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00,
    0x02, 0x04, 0x04, 0x12, 0x06, 0xbc, 0x02, 0x10, 0xbb, 0x02, 0x4b, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1e, 0x03, 0x00, 0x02, 0x04, 0x06, 0x12, 0x04, 0xbc, 0x02, 0x10, 0x40, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1e, 0x03, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xbc, 0x02, 0x41, 0x57, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x04, 0x03, 0x12, 0x04, 0xbc, 0x02, 0x5a, 0x5b, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x05, 0x12, 0x04, 0xbd, 0x02, 0x10, 0x55, 0x0a, 0x11,
    0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x05, 0x04, 0x12, 0x06, 0xbd, 0x02, 0x10, 0xbc, 0x02,
    0x5c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x05, 0x06, 0x12, 0x04, 0xbd, 0x02,
    0x10, 0x41, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0xbd,
    0x02, 0x42, 0x50, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x05, 0x03, 0x12, 0x04,
    0xbd, 0x02, 0x53, 0x54, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x06, 0x12, 0x04,
    0xbe, 0x02, 0x10, 0x4e, 0x0a, 0x11, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x06, 0x04, 0x12,
    0x06, 0xbe, 0x02, 0x10, 0xbd, 0x02, 0x55, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02,
    0x06, 0x06, 0x12, 0x04, 0xbe, 0x02, 0x10, 0x39, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00,
    0x02, 0x06, 0x01, 0x12, 0x04, 0xbe, 0x02, 0x3a, 0x48, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03,
    0x00, 0x02, 0x06, 0x03, 0x12, 0x04, 0xbe, 0x02, 0x4b, 0x4d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e,
    0x03, 0x00, 0x02, 0x07, 0x12, 0x04, 0xbf, 0x02, 0x10, 0x47, 0x0a, 0x11, 0x0a, 0x07, 0x04, 0x1e,
    0x03, 0x00, 0x02, 0x07, 0x04, 0x12, 0x06, 0xbf, 0x02, 0x10, 0xbe, 0x02, 0x4e, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x07, 0x06, 0x12, 0x04, 0xbf, 0x02, 0x10, 0x3a, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x07, 0x01, 0x12, 0x04, 0xbf, 0x02, 0x3b, 0x41, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x07, 0x03, 0x12, 0x04, 0xbf, 0x02, 0x44, 0x46,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x08, 0x12, 0x04, 0xc0, 0x02, 0x10, 0x52,
    0x0a, 0x11, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x08, 0x04, 0x12, 0x06, 0xc0, 0x02, 0x10,
    0xbf, 0x02, 0x47, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x08, 0x06, 0x12, 0x04,
    0xc0, 0x02, 0x10, 0x3f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x08, 0x01, 0x12,
    0x04, 0xc0, 0x02, 0x40, 0x4c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x08, 0x03,
    0x12, 0x04, 0xc0, 0x02, 0x4f, 0x51, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x09,
    0x12, 0x04, 0xc1, 0x02, 0x10, 0x4c, 0x0a, 0x11, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x09,
    0x04, 0x12, 0x06, 0xc1, 0x02, 0x10, 0xc0, 0x02, 0x52, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03,
    0x00, 0x02, 0x09, 0x06, 0x12, 0x04, 0xc1, 0x02, 0x10, 0x3c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e,
    0x03, 0x00, 0x02, 0x09, 0x01, 0x12, 0x04, 0xc1, 0x02, 0x3d, 0x46, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1e, 0x03, 0x00, 0x02, 0x09, 0x03, 0x12, 0x04, 0xc1, 0x02, 0x49, 0x4b, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x1e, 0x03, 0x00, 0x02, 0x0a, 0x12, 0x04, 0xc2, 0x02, 0x10, 0x53, 0x0a, 0x11, 0x0a, 0x07,
    0x04, 0x1e, 0x03, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x06, 0xc2, 0x02, 0x10, 0xc1, 0x02, 0x4c, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0a, 0x06, 0x12, 0x04, 0xc2, 0x02, 0x10, 0x3d,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xc2, 0x02, 0x3e,
    0x4d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xc2, 0x02,
    0x50, 0x52, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0b, 0x12, 0x04, 0xc3, 0x02,
    0x10, 0x56, 0x0a, 0x11, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x06, 0xc3,
    0x02, 0x10, 0xc2, 0x02, 0x53, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0b, 0x06,
    0x12, 0x04, 0xc3, 0x02, 0x10, 0x3d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0b,
    0x01, 0x12, 0x04, 0xc3, 0x02, 0x3e, 0x50, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02,
    0x0b, 0x03, 0x12, 0x04, 0xc3, 0x02, 0x53, 0x55, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00,
    0x02, 0x0c, 0x12, 0x04, 0xc4, 0x02, 0x10, 0x51, 0x0a, 0x11, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00,
    0x02, 0x0c, 0x04, 0x12, 0x06, 0xc4, 0x02, 0x10, 0xc3, 0x02, 0x56, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1e, 0x03, 0x00, 0x02, 0x0c, 0x06, 0x12, 0x04, 0xc4, 0x02, 0x10, 0x3a, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1e, 0x03, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x04, 0xc4, 0x02, 0x3b, 0x4b, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0c, 0x03, 0x12, 0x04, 0xc4, 0x02, 0x4e, 0x50, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0d, 0x12, 0x04, 0xc5, 0x02, 0x10, 0x4a, 0x0a, 0x11,
    0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0d, 0x04, 0x12, 0x06, 0xc5, 0x02, 0x10, 0xc4, 0x02,
    0x51, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0d, 0x06, 0x12, 0x04, 0xc5, 0x02,
    0x10, 0x37, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xc5,
    0x02, 0x38, 0x44, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0d, 0x03, 0x12, 0x04,
    0xc5, 0x02, 0x47, 0x49, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0e, 0x12, 0x04,
    0xc6, 0x02, 0x10, 0x59, 0x0a, 0x11, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0e, 0x04, 0x12,
    0x06, 0xc6, 0x02, 0x10, 0xc5, 0x02, 0x4a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02,
    0x0e, 0x06, 0x12, 0x04, 0xc6, 0x02, 0x10, 0x42, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00,
    0x02, 0x0e, 0x01, 0x12, 0x04, 0xc6, 0x02, 0x43, 0x53, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03,
    0x00, 0x02, 0x0e, 0x03, 0x12, 0x04, 0xc6, 0x02, 0x56, 0x58, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e,
    0x03, 0x00, 0x02, 0x0f, 0x12, 0x04, 0xc7, 0x02, 0x10, 0x57, 0x0a, 0x11, 0x0a, 0x07, 0x04, 0x1e,
    0x03, 0x00, 0x02, 0x0f, 0x04, 0x12, 0x06, 0xc7, 0x02, 0x10, 0xc6, 0x02, 0x59, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0f, 0x06, 0x12, 0x04, 0xc7, 0x02, 0x10, 0x41, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x04, 0xc7, 0x02, 0x42, 0x51, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x0f, 0x03, 0x12, 0x04, 0xc7, 0x02, 0x54, 0x56,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1f, 0x12, 0x06, 0xca, 0x02, 0x00, 0xd5, 0x02, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x1f, 0x01, 0x12, 0x04, 0xca, 0x02, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1f, 0x02, 0x00, 0x12, 0x04, 0xcb, 0x02, 0x08, 0x53, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x00, 0x04, 0x12, 0x06, 0xcb, 0x02, 0x08, 0xca, 0x02, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xcb, 0x02, 0x08, 0x47, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xcb, 0x02, 0x48, 0x4e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xcb, 0x02, 0x51, 0x52, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x01, 0x12,
    0x04, 0xcc, 0x02, 0x08, 0x4e, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x04, 0x12, 0x06,
    0xcc, 0x02, 0x08, 0xcb, 0x02, 0x53, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x06, 0x12,
    0x04, 0xcc, 0x02, 0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xcc, 0x02, 0x3d, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xcc,
    0x02, 0x4c, 0x4d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1f, 0x04, 0x00, 0x12, 0x06, 0xce, 0x02, 0x08,
    0xd4, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x04, 0x00, 0x01, 0x12, 0x04, 0xce, 0x02,
    0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1f, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xcf, 0x02,
    0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcf,
    0x02, 0x10, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04,
    0xcf, 0x02, 0x20, 0x21, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1f, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04,
    0xd0, 0x02, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xd0, 0x02, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x04, 0x00, 0x02, 0x01, 0x02,
    0x12, 0x04, 0xd0, 0x02, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1f, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x04, 0xd1, 0x02, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x04, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xd1, 0x02, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x04, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x04, 0xd1, 0x02, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1f, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x04, 0xd2, 0x02, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x04, 0xd2, 0x02, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x04,
    0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xd2, 0x02, 0x1f, 0x20, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1f,
    0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xd3, 0x02, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f,
    0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xd3, 0x02, 0x10, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1f, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xd3, 0x02, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x20, 0x12, 0x06, 0xd6, 0x02, 0x00, 0xe0, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x20,
    0x01, 0x12, 0x04, 0xd6, 0x02, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x00, 0x12,
    0x04, 0xd7, 0x02, 0x08, 0x5d, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x04, 0x12, 0x06,
    0xd7, 0x02, 0x08, 0xd6, 0x02, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x06, 0x12,
    0x04, 0xd7, 0x02, 0x08, 0x51, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xd7, 0x02, 0x52, 0x58, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd7,
    0x02, 0x5b, 0x5c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x01, 0x12, 0x04, 0xd8, 0x02, 0x08,
    0x36, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x04, 0x12, 0x06, 0xd8, 0x02, 0x08, 0xd7,
    0x02, 0x5d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x06, 0x12, 0x04, 0xd8, 0x02, 0x08,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd8, 0x02, 0x25, 0x31,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd8, 0x02, 0x34, 0x35, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x02, 0x12, 0x04, 0xd9, 0x02, 0x08, 0x40, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x02, 0x04, 0x12, 0x06, 0xd9, 0x02, 0x08, 0xd8, 0x02, 0x36, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x20, 0x02, 0x02, 0x06, 0x12, 0x04, 0xd9, 0x02, 0x08, 0x2d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd9, 0x02, 0x2e, 0x3b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd9, 0x02, 0x3e, 0x3f, 0x0a, 0x0e, 0x0a, 0x04, 0x04,
    0x20, 0x04, 0x00, 0x12, 0x06, 0xdb, 0x02, 0x08, 0xdf, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x04, 0x00, 0x01, 0x12, 0x04, 0xdb, 0x02, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x20,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xdc, 0x02, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x20,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdc, 0x02, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x20, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xdc, 0x02, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x20, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xdd, 0x02, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x20, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xdd, 0x02, 0x10, 0x17, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x20, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xdd, 0x02, 0x1a, 0x1b, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x20, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xde, 0x02, 0x10, 0x2a, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x20, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xde, 0x02, 0x10, 0x25, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x20, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xde, 0x02, 0x28, 0x29,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x21, 0x12, 0x06, 0xe1, 0x02, 0x00, 0xe4, 0x02, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x21, 0x01, 0x12, 0x04, 0xe1, 0x02, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x21, 0x02, 0x00, 0x12, 0x04, 0xe2, 0x02, 0x08, 0x19, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x21, 0x02,
    0x00, 0x04, 0x12, 0x06, 0xe2, 0x02, 0x08, 0xe1, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xe2, 0x02, 0x08, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xe2, 0x02, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xe2, 0x02, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x01, 0x12,
    0x04, 0xe3, 0x02, 0x08, 0x41, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x04, 0x12, 0x06,
    0xe3, 0x02, 0x08, 0xe2, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x06, 0x12,
    0x04, 0xe3, 0x02, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xe3, 0x02, 0x2d, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe3,
    0x02, 0x3f, 0x40, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x22, 0x12, 0x06, 0xe5, 0x02, 0x00, 0xe8, 0x02,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x22, 0x01, 0x12, 0x04, 0xe5, 0x02, 0x08, 0x1d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x22, 0x02, 0x00, 0x12, 0x04, 0xe6, 0x02, 0x08, 0x37, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x22, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe6, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x22, 0x02, 0x00, 0x06, 0x12, 0x04, 0xe6, 0x02, 0x11, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xe6, 0x02, 0x29, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xe6, 0x02, 0x35, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x01,
    0x12, 0x04, 0xe7, 0x02, 0x08, 0x34, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x04, 0x12,
    0x06, 0xe7, 0x02, 0x08, 0xe6, 0x02, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x06,
    0x12, 0x04, 0xe7, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xe7, 0x02, 0x29, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xe7, 0x02, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x23, 0x12, 0x06, 0xe9, 0x02, 0x00, 0xf3,
    0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x23, 0x01, 0x12, 0x04, 0xe9, 0x02, 0x08, 0x1d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x00, 0x12, 0x04, 0xea, 0x02, 0x08, 0x51, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x23, 0x02, 0x00, 0x04, 0x12, 0x06, 0xea, 0x02, 0x08, 0xe9, 0x02, 0x1f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x06, 0x12, 0x04, 0xea, 0x02, 0x08, 0x45, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x23, 0x02, 0x00, 0x01, 0x12, 0x04, 0xea, 0x02, 0x46, 0x4c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x23, 0x02, 0x00, 0x03, 0x12, 0x04, 0xea, 0x02, 0x4f, 0x50, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x23, 0x02, 0x01, 0x12, 0x04, 0xeb, 0x02, 0x08, 0x34, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x23, 0x02,
    0x01, 0x04, 0x12, 0x06, 0xeb, 0x02, 0x08, 0xea, 0x02, 0x51, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23,
    0x02, 0x01, 0x06, 0x12, 0x04, 0xeb, 0x02, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xeb, 0x02, 0x24, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xeb, 0x02, 0x32, 0x33, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x23, 0x04, 0x00, 0x12,
    0x06, 0xed, 0x02, 0x08, 0xf2, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x04, 0x00, 0x01,
    0x12, 0x04, 0xed, 0x02, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x23, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x04, 0xee, 0x02, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xee, 0x02, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x04, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x04, 0xee, 0x02, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x23, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x04, 0xef, 0x02, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xef, 0x02, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x04,
    0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xef, 0x02, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x23,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xf0, 0x02, 0x10, 0x25, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf0, 0x02, 0x10, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x23, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xf0, 0x02, 0x23, 0x24, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x23, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xf1, 0x02, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x23, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf1, 0x02, 0x10, 0x17, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x23, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xf1, 0x02, 0x1a, 0x1b, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x24, 0x12, 0x06, 0xf4, 0x02, 0x00, 0x82, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x24, 0x01, 0x12, 0x04, 0xf4, 0x02, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02,
    0x00, 0x12, 0x04, 0xf5, 0x02, 0x08, 0x55, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x04,
    0x12, 0x06, 0xf5, 0x02, 0x08, 0xf4, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00,
    0x06, 0x12, 0x04, 0xf5, 0x02, 0x08, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xf5, 0x02, 0x4a, 0x50, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xf5, 0x02, 0x53, 0x54, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x01, 0x12, 0x04, 0xf6,
    0x02, 0x08, 0x33, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x04, 0x12, 0x06, 0xf6, 0x02,
    0x08, 0xf5, 0x02, 0x55, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x06, 0x12, 0x04, 0xf6,
    0x02, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf6, 0x02,
    0x24, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf6, 0x02, 0x31,
    0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x02, 0x12, 0x04, 0xf7, 0x02, 0x08, 0x1c, 0x0a,
    0x0f, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x02, 0x04, 0x12, 0x06, 0xf7, 0x02, 0x08, 0xf6, 0x02, 0x33,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x02, 0x05, 0x12, 0x04, 0xf7, 0x02, 0x08, 0x0e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf7, 0x02, 0x0f, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x24, 0x02, 0x02, 0x03, 0x12, 0x04, 0xf7, 0x02, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x24, 0x02, 0x03, 0x12, 0x04, 0xf8, 0x02, 0x08, 0x1d, 0x0a, 0x0f, 0x0a, 0x05, 0x04,
    0x24, 0x02, 0x03, 0x04, 0x12, 0x06, 0xf8, 0x02, 0x08, 0xf7, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x24, 0x02, 0x03, 0x05, 0x12, 0x04, 0xf8, 0x02, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x24, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf8, 0x02, 0x0f, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xf8, 0x02, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02,
    0x04, 0x12, 0x04, 0xf9, 0x02, 0x08, 0x26, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x04, 0x04,
    0x12, 0x06, 0xf9, 0x02, 0x08, 0xf8, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x04,
    0x05, 0x12, 0x04, 0xf9, 0x02, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x04, 0x01,
    0x12, 0x04, 0xf9, 0x02, 0x0f, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x04, 0x03, 0x12,
    0x04, 0xf9, 0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x05, 0x12, 0x04, 0xfa,
    0x02, 0x08, 0x21, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x05, 0x04, 0x12, 0x06, 0xfa, 0x02,
    0x08, 0xf9, 0x02, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x05, 0x05, 0x12, 0x04, 0xfa,
    0x02, 0x08, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x05, 0x01, 0x12, 0x04, 0xfa, 0x02,
    0x10, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x05, 0x03, 0x12, 0x04, 0xfa, 0x02, 0x1f,
    0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x06, 0x12, 0x04, 0xfb, 0x02, 0x08, 0x29, 0x0a,
    0x0f, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x06, 0x04, 0x12, 0x06, 0xfb, 0x02, 0x08, 0xfa, 0x02, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x06, 0x05, 0x12, 0x04, 0xfb, 0x02, 0x08, 0x0d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x06, 0x01, 0x12, 0x04, 0xfb, 0x02, 0x0e, 0x24, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x24, 0x02, 0x06, 0x03, 0x12, 0x04, 0xfb, 0x02, 0x27, 0x28, 0x0a, 0x0e, 0x0a,
    0x04, 0x04, 0x24, 0x04, 0x00, 0x12, 0x06, 0xfd, 0x02, 0x08, 0x81, 0x03, 0x09, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x24, 0x04, 0x00, 0x01, 0x12, 0x04, 0xfd, 0x02, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x24, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xfe, 0x02, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x24, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfe, 0x02, 0x10, 0x29, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x24, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xfe, 0x02, 0x2c, 0x2d, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x24, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xff, 0x02, 0x10, 0x30, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x24, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xff, 0x02, 0x10, 0x2b, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x24, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xff, 0x02, 0x2e, 0x2f,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x24, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x80, 0x03, 0x10, 0x34,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x24, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x80, 0x03, 0x10,
    0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x24, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x80, 0x03,
    0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x25, 0x12, 0x06, 0x83, 0x03, 0x00, 0x86, 0x03, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x25, 0x01, 0x12, 0x04, 0x83, 0x03, 0x08, 0x1e, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x25, 0x02, 0x00, 0x12, 0x04, 0x84, 0x03, 0x08, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x25, 0x02, 0x00, 0x04, 0x12, 0x04, 0x84, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x02, 0x00, 0x06, 0x12, 0x04, 0x84, 0x03, 0x11, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x84, 0x03, 0x33, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x84, 0x03, 0x3c, 0x3d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x01, 0x12,
    0x04, 0x85, 0x03, 0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x04, 0x12, 0x06,
    0x85, 0x03, 0x08, 0x84, 0x03, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x05, 0x12,
    0x04, 0x85, 0x03, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x01, 0x12, 0x04,
    0x85, 0x03, 0x0f, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x03, 0x12, 0x04, 0x85,
    0x03, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x26, 0x12, 0x06, 0x87, 0x03, 0x00, 0x95, 0x03,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x26, 0x01, 0x12, 0x04, 0x87, 0x03, 0x08, 0x26, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x26, 0x02, 0x00, 0x12, 0x04, 0x88, 0x03, 0x08, 0x1c, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x00, 0x04, 0x12, 0x06, 0x88, 0x03, 0x08, 0x87, 0x03, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x26, 0x02, 0x00, 0x05, 0x12, 0x04, 0x88, 0x03, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x00, 0x01, 0x12, 0x04, 0x88, 0x03, 0x0f, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x00, 0x03, 0x12, 0x04, 0x88, 0x03, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26,
    0x02, 0x01, 0x12, 0x04, 0x89, 0x03, 0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01,
    0x04, 0x12, 0x06, 0x89, 0x03, 0x08, 0x88, 0x03, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02,
    0x01, 0x05, 0x12, 0x04, 0x89, 0x03, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x89, 0x03, 0x0f, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x89, 0x03, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x02, 0x12, 0x04,
    0x8a, 0x03, 0x08, 0x1f, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x04, 0x12, 0x06, 0x8a,
    0x03, 0x08, 0x89, 0x03, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x05, 0x12, 0x04,
    0x8a, 0x03, 0x08, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8a,
    0x03, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8a, 0x03,
    0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x03, 0x12, 0x04, 0x8b, 0x03, 0x08, 0x5a,
    0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x04, 0x12, 0x06, 0x8b, 0x03, 0x08, 0x8a, 0x03,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x06, 0x12, 0x04, 0x8b, 0x03, 0x08, 0x4e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8b, 0x03, 0x4f, 0x55, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8b, 0x03, 0x58, 0x59, 0x0a, 0x0e,
    0x0a, 0x04, 0x04, 0x26, 0x04, 0x00, 0x12, 0x06, 0x8d, 0x03, 0x08, 0x94, 0x03, 0x09, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x26, 0x04, 0x00, 0x01, 0x12, 0x04, 0x8d, 0x03, 0x0d, 0x13, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x26, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0x8e, 0x03, 0x10, 0x1a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x26, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8e, 0x03, 0x10, 0x15, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x26, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0x8e, 0x03, 0x18, 0x19, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x26, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0x8f, 0x03, 0x10, 0x1c, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x26, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8f, 0x03, 0x10, 0x17,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0x8f, 0x03, 0x1a,
    0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x26, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x90, 0x03, 0x10,
    0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x90, 0x03,
    0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x90,
    0x03, 0x29, 0x2a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x26, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x91,
    0x03, 0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04,
    0x91, 0x03, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12,
    0x04, 0x91, 0x03, 0x25, 0x26, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x26, 0x04, 0x00, 0x02, 0x04, 0x12,
    0x04, 0x92, 0x03, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x04, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x92, 0x03, 0x10, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x04, 0x00, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x92, 0x03, 0x20, 0x21, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x26, 0x04, 0x00, 0x02,
    0x05, 0x12, 0x04, 0x93, 0x03, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x04, 0x00, 0x02,
    0x05, 0x01, 0x12, 0x04, 0x93, 0x03, 0x10, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x04, 0x00,
    0x02, 0x05, 0x02, 0x12, 0x04, 0x93, 0x03, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x27, 0x12,
    0x06, 0x96, 0x03, 0x00, 0xa3, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x27, 0x01, 0x12, 0x04,
    0x96, 0x03, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x00, 0x12, 0x04, 0x97, 0x03,
    0x08, 0x1a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x04, 0x12, 0x06, 0x97, 0x03, 0x08,
    0x96, 0x03, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x06, 0x12, 0x04, 0x97, 0x03,
    0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x01, 0x12, 0x04, 0x97, 0x03, 0x0f,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x03, 0x12, 0x04, 0x97, 0x03, 0x18, 0x19,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x01, 0x12, 0x04, 0x98, 0x03, 0x08, 0x36, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x04, 0x12, 0x06, 0x98, 0x03, 0x08, 0x97, 0x03, 0x1a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x06, 0x12, 0x04, 0x98, 0x03, 0x08, 0x24, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x01, 0x12, 0x04, 0x98, 0x03, 0x25, 0x31, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x01, 0x03, 0x12, 0x04, 0x98, 0x03, 0x34, 0x35, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x27, 0x02, 0x02, 0x12, 0x04, 0x99, 0x03, 0x08, 0x4c, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x02, 0x04, 0x12, 0x06, 0x99, 0x03, 0x08, 0x98, 0x03, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x02, 0x06, 0x12, 0x04, 0x99, 0x03, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x02, 0x01, 0x12, 0x04, 0x99, 0x03, 0x34, 0x47, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02,
    0x02, 0x03, 0x12, 0x04, 0x99, 0x03, 0x4a, 0x4b, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x27, 0x04, 0x00,
    0x12, 0x06, 0x9b, 0x03, 0x08, 0xa2, 0x03, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x04, 0x00,
    0x01, 0x12, 0x04, 0x9b, 0x03, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x27, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x04, 0x9c, 0x03, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x27, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x9c, 0x03, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x27, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x04, 0x9c, 0x03, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x27, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x04, 0x9d, 0x03, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x27, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9d, 0x03, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x27,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0x9d, 0x03, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x27, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x9e, 0x03, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x27, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x9e, 0x03, 0x10, 0x1d, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x27, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x9e, 0x03, 0x20, 0x21, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x27, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x9f, 0x03, 0x10, 0x21, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x27, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x9f, 0x03, 0x10, 0x1c, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x27, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0x9f, 0x03, 0x1f, 0x20, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x27, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xa0, 0x03, 0x10, 0x2f, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x27, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xa0, 0x03, 0x10, 0x2a,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x27, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xa0, 0x03, 0x2d,
    0x2e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x27, 0x04, 0x00, 0x02, 0x05, 0x12, 0x04, 0xa1, 0x03, 0x10,
    0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x27, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0xa1, 0x03,
    0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x27, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04, 0xa1,
    0x03, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x28, 0x12, 0x06, 0xa4, 0x03, 0x00, 0xb2, 0x03,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x28, 0x01, 0x12, 0x04, 0xa4, 0x03, 0x08, 0x23, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x28, 0x02, 0x00, 0x12, 0x04, 0xa5, 0x03, 0x08, 0x57, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x28, 0x02, 0x00, 0x04, 0x12, 0x06, 0xa5, 0x03, 0x08, 0xa4, 0x03, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x28, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa5, 0x03, 0x08, 0x4b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x28, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa5, 0x03, 0x4c, 0x52, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x28, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa5, 0x03, 0x55, 0x56, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28,
    0x02, 0x01, 0x12, 0x04, 0xa6, 0x03, 0x08, 0x3d, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01,
    0x04, 0x12, 0x06, 0xa6, 0x03, 0x08, 0xa5, 0x03, 0x57, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02,
    0x01, 0x06, 0x12, 0x04, 0xa6, 0x03, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xa6, 0x03, 0x2b, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xa6, 0x03, 0x3b, 0x3c, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x28, 0x04, 0x00, 0x12, 0x06,
    0xa8, 0x03, 0x08, 0xb1, 0x03, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x04, 0x00, 0x01, 0x12,
    0x04, 0xa8, 0x03, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x28, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x04, 0xa9, 0x03, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xa9, 0x03, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x04, 0xa9, 0x03, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x28, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x04, 0xaa, 0x03, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xaa, 0x03, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x04, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x04, 0xaa, 0x03, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x28, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x04, 0xab, 0x03, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xab, 0x03, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28,
    0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xab, 0x03, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x28, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xac, 0x03, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x28, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xac, 0x03, 0x10, 0x2b, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x28, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xac, 0x03, 0x2e, 0x2f, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x28, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xad, 0x03, 0x10, 0x2d, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x28, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xad, 0x03, 0x10, 0x28, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x28, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xad, 0x03, 0x2b, 0x2c, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x28, 0x04, 0x00, 0x02, 0x05, 0x12, 0x04, 0xae, 0x03, 0x10, 0x33, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x28, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0xae, 0x03, 0x10, 0x2e,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04, 0xae, 0x03, 0x31,
    0x32, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x28, 0x04, 0x00, 0x02, 0x06, 0x12, 0x04, 0xaf, 0x03, 0x10,
    0x35, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04, 0xaf, 0x03,
    0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x04, 0xaf,
    0x03, 0x33, 0x34, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x28, 0x04, 0x00, 0x02, 0x07, 0x12, 0x04, 0xb0,
    0x03, 0x10, 0x36, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x04,
    0xb0, 0x03, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x04, 0x00, 0x02, 0x07, 0x02, 0x12,
    0x04, 0xb0, 0x03, 0x34, 0x35, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x29, 0x12, 0x06, 0xb3, 0x03, 0x00,
    0xbd, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x29, 0x01, 0x12, 0x04, 0xb3, 0x03, 0x08, 0x1f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x00, 0x12, 0x04, 0xb4, 0x03, 0x08, 0x53, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x04, 0x12, 0x06, 0xb4, 0x03, 0x08, 0xb3, 0x03, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb4, 0x03, 0x08, 0x47, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb4, 0x03, 0x48, 0x4e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x29, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb4, 0x03, 0x51, 0x52, 0x0a, 0x0e, 0x0a, 0x04,
    0x04, 0x29, 0x04, 0x00, 0x12, 0x06, 0xb6, 0x03, 0x08, 0xbc, 0x03, 0x09, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x29, 0x04, 0x00, 0x01, 0x12, 0x04, 0xb6, 0x03, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x29, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xb7, 0x03, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x29, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb7, 0x03, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x29, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xb7, 0x03, 0x18, 0x19, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x29, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xb8, 0x03, 0x10, 0x1c, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x29, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb8, 0x03, 0x10, 0x17, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x29, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xb8, 0x03, 0x1a, 0x1b, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x29, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xb9, 0x03, 0x10, 0x2b, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x29, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb9, 0x03, 0x10, 0x26,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xb9, 0x03, 0x29,
    0x2a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xba, 0x03, 0x10,
    0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xba, 0x03,
    0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xba,
    0x03, 0x2a, 0x2b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xbb,
    0x03, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04,
    0xbb, 0x03, 0x10, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12,
    0x04, 0xbb, 0x03, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2a, 0x12, 0x06, 0xbe, 0x03, 0x00,
    0xc9, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2a, 0x01, 0x12, 0x04, 0xbe, 0x03, 0x08, 0x21,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2a, 0x02, 0x00, 0x12, 0x04, 0xbf, 0x03, 0x08, 0x55, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x04, 0x12, 0x06, 0xbf, 0x03, 0x08, 0xbe, 0x03, 0x23, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x06, 0x12, 0x04, 0xbf, 0x03, 0x08, 0x49, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbf, 0x03, 0x4a, 0x50, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbf, 0x03, 0x53, 0x54, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x2a, 0x02, 0x01, 0x12, 0x04, 0xc0, 0x03, 0x08, 0x4e, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2a,
    0x02, 0x01, 0x04, 0x12, 0x06, 0xc0, 0x03, 0x08, 0xbf, 0x03, 0x55, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2a, 0x02, 0x01, 0x06, 0x12, 0x04, 0xc0, 0x03, 0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xc0, 0x03, 0x3d, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xc0, 0x03, 0x4c, 0x4d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x2a, 0x04, 0x00,
    0x12, 0x06, 0xc2, 0x03, 0x08, 0xc8, 0x03, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x04, 0x00,
    0x01, 0x12, 0x04, 0xc2, 0x03, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x2a, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x04, 0xc3, 0x03, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2a, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xc3, 0x03, 0x10, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2a, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x04, 0xc3, 0x03, 0x20, 0x21, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x2a, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x04, 0xc4, 0x03, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2a, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc4, 0x03, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2a,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xc4, 0x03, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x2a, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xc5, 0x03, 0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x2a, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc5, 0x03, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x2a, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xc5, 0x03, 0x25, 0x26, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x2a, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xc6, 0x03, 0x10, 0x2e, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x2a, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc6, 0x03, 0x10, 0x29, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x2a, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xc6, 0x03, 0x2c, 0x2d, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x2a, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xc7, 0x03, 0x10, 0x24, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x2a, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xc7, 0x03, 0x10, 0x1f,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2a, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xc7, 0x03, 0x22,
    0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2b, 0x12, 0x06, 0xca, 0x03, 0x00, 0xd4, 0x03, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x2b, 0x01, 0x12, 0x04, 0xca, 0x03, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x2b, 0x02, 0x00, 0x12, 0x04, 0xcb, 0x03, 0x08, 0x52, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2b,
    0x02, 0x00, 0x04, 0x12, 0x06, 0xcb, 0x03, 0x08, 0xca, 0x03, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2b, 0x02, 0x00, 0x06, 0x12, 0x04, 0xcb, 0x03, 0x08, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xcb, 0x03, 0x47, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xcb, 0x03, 0x50, 0x51, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02, 0x01,
    0x12, 0x04, 0xcc, 0x03, 0x08, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xcc, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x01, 0x06, 0x12, 0x04,
    0xcc, 0x03, 0x11, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcc,
    0x03, 0x36, 0x43, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x01, 0x03, 0x12, 0x04, 0xcc, 0x03,
    0x46, 0x47, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02, 0x02, 0x12, 0x04, 0xcd, 0x03, 0x08, 0x46,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x02, 0x04, 0x12, 0x04, 0xcd, 0x03, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x02, 0x06, 0x12, 0x04, 0xcd, 0x03, 0x11, 0x32, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2b, 0x02, 0x02, 0x01, 0x12, 0x04, 0xcd, 0x03, 0x33, 0x41, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2b, 0x02, 0x02, 0x03, 0x12, 0x04, 0xcd, 0x03, 0x44, 0x45, 0x0a, 0x0e, 0x0a, 0x04,
    0x04, 0x2b, 0x04, 0x00, 0x12, 0x06, 0xcf, 0x03, 0x08, 0xd3, 0x03, 0x09, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2b, 0x04, 0x00, 0x01, 0x12, 0x04, 0xcf, 0x03, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x2b, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xd0, 0x03, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x2b, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd0, 0x03, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x2b, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xd0, 0x03, 0x18, 0x19, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x2b, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xd1, 0x03, 0x10, 0x1c, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x2b, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd1, 0x03, 0x10, 0x17, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x2b, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xd1, 0x03, 0x1a, 0x1b, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x2b, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xd2, 0x03, 0x10, 0x24, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x2b, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd2, 0x03, 0x10, 0x1f,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2b, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xd2, 0x03, 0x22,
    0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2c, 0x12, 0x06, 0xd5, 0x03, 0x00, 0xdf, 0x03, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x2c, 0x01, 0x12, 0x04, 0xd5, 0x03, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x2c, 0x02, 0x00, 0x12, 0x04, 0xd6, 0x03, 0x08, 0x58, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2c,
    0x02, 0x00, 0x04, 0x12, 0x06, 0xd6, 0x03, 0x08, 0xd5, 0x03, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2c, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd6, 0x03, 0x08, 0x4c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xd6, 0x03, 0x4d, 0x53, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xd6, 0x03, 0x56, 0x57, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2c, 0x02, 0x01,
    0x12, 0x04, 0xd7, 0x03, 0x08, 0x1c, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x04, 0x12,
    0x06, 0xd7, 0x03, 0x08, 0xd6, 0x03, 0x58, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xd7, 0x03, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xd7, 0x03, 0x0e, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xd7, 0x03, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x2c, 0x04, 0x00, 0x12, 0x06, 0xd9, 0x03,
    0x08, 0xde, 0x03, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x04, 0x00, 0x01, 0x12, 0x04, 0xd9,
    0x03, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x2c, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xda,
    0x03, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2c, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xda, 0x03, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2c, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x04, 0xda, 0x03, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x2c, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x04, 0xdb, 0x03, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2c, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xdb, 0x03, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2c, 0x04, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x04, 0xdb, 0x03, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x2c, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x04, 0xdc, 0x03, 0x10, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2c, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xdc, 0x03, 0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2c, 0x04, 0x00,
    0x02, 0x02, 0x02, 0x12, 0x04, 0xdc, 0x03, 0x2a, 0x2b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x2c, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x04, 0xdd, 0x03, 0x10, 0x34, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2c, 0x04,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xdd, 0x03, 0x10, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2c,
    0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xdd, 0x03, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x2d, 0x12, 0x06, 0xe0, 0x03, 0x00, 0xea, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2d, 0x01,
    0x12, 0x04, 0xe0, 0x03, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x00, 0x12, 0x04,
    0xe1, 0x03, 0x08, 0x4d, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x04, 0x12, 0x06, 0xe1,
    0x03, 0x08, 0xe0, 0x03, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xe1, 0x03, 0x08, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe1,
    0x03, 0x42, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe1, 0x03,
    0x4b, 0x4c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x01, 0x12, 0x04, 0xe2, 0x03, 0x08, 0x34,
    0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x04, 0x12, 0x06, 0xe2, 0x03, 0x08, 0xe1, 0x03,
    0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x06, 0x12, 0x04, 0xe2, 0x03, 0x08, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe2, 0x03, 0x24, 0x2f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe2, 0x03, 0x32, 0x33, 0x0a, 0x0e,
    0x0a, 0x04, 0x04, 0x2d, 0x04, 0x00, 0x12, 0x06, 0xe4, 0x03, 0x08, 0xe9, 0x03, 0x09, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2d, 0x04, 0x00, 0x01, 0x12, 0x04, 0xe4, 0x03, 0x0d, 0x13, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x2d, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xe5, 0x03, 0x10, 0x1a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x2d, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe5, 0x03, 0x10, 0x15, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x2d, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xe5, 0x03, 0x18, 0x19, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x2d, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xe6, 0x03, 0x10, 0x1c, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x2d, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe6, 0x03, 0x10, 0x17,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2d, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xe6, 0x03, 0x1a,
    0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x2d, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xe7, 0x03, 0x10,
    0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2d, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe7, 0x03,
    0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2d, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xe7,
    0x03, 0x25, 0x26, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x2d, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xe8,
    0x03, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2d, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04,
    0xe8, 0x03, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2d, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12,
    0x04, 0xe8, 0x03, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2e, 0x12, 0x06, 0xeb, 0x03, 0x00,
    0xf8, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2e, 0x01, 0x12, 0x04, 0xeb, 0x03, 0x08, 0x29,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x00, 0x12, 0x04, 0xec, 0x03, 0x08, 0x5d, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x04, 0x12, 0x06, 0xec, 0x03, 0x08, 0xeb, 0x03, 0x2b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x06, 0x12, 0x04, 0xec, 0x03, 0x08, 0x51, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xec, 0x03, 0x52, 0x58, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xec, 0x03, 0x5b, 0x5c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x2e, 0x02, 0x01, 0x12, 0x04, 0xed, 0x03, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xed, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02,
    0x01, 0x05, 0x12, 0x04, 0xed, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xed, 0x03, 0x18, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xed, 0x03, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x02, 0x12, 0x04,
    0xee, 0x03, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x02, 0x04, 0x12, 0x04, 0xee,
    0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x02, 0x05, 0x12, 0x04, 0xee, 0x03,
    0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x02, 0x01, 0x12, 0x04, 0xee, 0x03, 0x17,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x02, 0x03, 0x12, 0x04, 0xee, 0x03, 0x2a, 0x2b,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x03, 0x12, 0x04, 0xef, 0x03, 0x08, 0x22, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x2e, 0x02, 0x03, 0x04, 0x12, 0x06, 0xef, 0x03, 0x08, 0xee, 0x03, 0x2c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x03, 0x05, 0x12, 0x04, 0xef, 0x03, 0x08, 0x0d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2e, 0x02, 0x03, 0x01, 0x12, 0x04, 0xef, 0x03, 0x0e, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2e, 0x02, 0x03, 0x03, 0x12, 0x04, 0xef, 0x03, 0x20, 0x21, 0x0a, 0x0e, 0x0a, 0x04,
    0x04, 0x2e, 0x04, 0x00, 0x12, 0x06, 0xf1, 0x03, 0x08, 0xf7, 0x03, 0x09, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2e, 0x04, 0x00, 0x01, 0x12, 0x04, 0xf1, 0x03, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x2e, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xf2, 0x03, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x2e, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf2, 0x03, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x2e, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xf2, 0x03, 0x18, 0x19, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x2e, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xf3, 0x03, 0x10, 0x1c, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x2e, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf3, 0x03, 0x10, 0x17, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x2e, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xf3, 0x03, 0x1a, 0x1b, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x2e, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xf4, 0x03, 0x10, 0x1c, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x2e, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf4, 0x03, 0x10, 0x17,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2e, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xf4, 0x03, 0x1a,
    0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x2e, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xf5, 0x03, 0x10,
    0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2e, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf5, 0x03,
    0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2e, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xf5,
    0x03, 0x1b, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x2e, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xf6,
    0x03, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2e, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04,
    0xf6, 0x03, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2e, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12,
    0x04, 0xf6, 0x03, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2f, 0x12, 0x06, 0xf9, 0x03, 0x00,
    0x82, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2f, 0x01, 0x12, 0x04, 0xf9, 0x03, 0x08, 0x22,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2f, 0x02, 0x00, 0x12, 0x04, 0xfa, 0x03, 0x08, 0x56, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00, 0x04, 0x12, 0x06, 0xfa, 0x03, 0x08, 0xf9, 0x03, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00, 0x06, 0x12, 0x04, 0xfa, 0x03, 0x08, 0x4a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfa, 0x03, 0x4b, 0x51, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfa, 0x03, 0x54, 0x55, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x2f, 0x02, 0x01, 0x12, 0x04, 0xfb, 0x03, 0x08, 0x34, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2f,
    0x02, 0x01, 0x04, 0x12, 0x06, 0xfb, 0x03, 0x08, 0xfa, 0x03, 0x56, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2f, 0x02, 0x01, 0x06, 0x12, 0x04, 0xfb, 0x03, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xfb, 0x03, 0x24, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xfb, 0x03, 0x32, 0x33, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x2f, 0x04, 0x00,
    0x12, 0x06, 0xfd, 0x03, 0x08, 0x81, 0x04, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x04, 0x00,
    0x01, 0x12, 0x04, 0xfd, 0x03, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x2f, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x04, 0xfe, 0x03, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2f, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xfe, 0x03, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2f, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x04, 0xfe, 0x03, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x2f, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x04, 0xff, 0x03, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2f, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xff, 0x03, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2f,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xff, 0x03, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x2f, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x80, 0x04, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x2f, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x80, 0x04, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x2f, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x80, 0x04, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x30, 0x12, 0x06, 0x83, 0x04, 0x00, 0x9b, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x30, 0x01, 0x12, 0x04, 0x83, 0x04, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x00,
    0x12, 0x04, 0x84, 0x04, 0x08, 0x52, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x04, 0x12,
    0x06, 0x84, 0x04, 0x08, 0x83, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x06,
    0x12, 0x04, 0x84, 0x04, 0x08, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x84, 0x04, 0x47, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x84, 0x04, 0x50, 0x51, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x01, 0x12, 0x04, 0x85, 0x04,
    0x08, 0x2c, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x01, 0x04, 0x12, 0x06, 0x85, 0x04, 0x08,
    0x84, 0x04, 0x52, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x01, 0x05, 0x12, 0x04, 0x85, 0x04,
    0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x01, 0x01, 0x12, 0x04, 0x85, 0x04, 0x0e,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x01, 0x03, 0x12, 0x04, 0x85, 0x04, 0x2a, 0x2b,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x02, 0x12, 0x04, 0x86, 0x04, 0x08, 0x2a, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x04, 0x12, 0x06, 0x86, 0x04, 0x08, 0x85, 0x04, 0x2c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x05, 0x12, 0x04, 0x86, 0x04, 0x08, 0x0d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x01, 0x12, 0x04, 0x86, 0x04, 0x0e, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x30, 0x02, 0x02, 0x03, 0x12, 0x04, 0x86, 0x04, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x30, 0x02, 0x03, 0x12, 0x04, 0x87, 0x04, 0x08, 0x1d, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x30,
    0x02, 0x03, 0x04, 0x12, 0x06, 0x87, 0x04, 0x08, 0x86, 0x04, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x30, 0x02, 0x03, 0x05, 0x12, 0x04, 0x87, 0x04, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30,
    0x02, 0x03, 0x01, 0x12, 0x04, 0x87, 0x04, 0x0f, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02,
    0x03, 0x03, 0x12, 0x04, 0x87, 0x04, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x04,
    0x12, 0x04, 0x88, 0x04, 0x08, 0x3f, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x04, 0x04, 0x12,
    0x06, 0x88, 0x04, 0x08, 0x87, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x04, 0x06,
    0x12, 0x04, 0x88, 0x04, 0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x88, 0x04, 0x32, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x88, 0x04, 0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x05, 0x12, 0x04, 0x89, 0x04,
    0x08, 0x39, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x05, 0x04, 0x12, 0x06, 0x89, 0x04, 0x08,
    0x88, 0x04, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x05, 0x06, 0x12, 0x04, 0x89, 0x04,
    0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x05, 0x01, 0x12, 0x04, 0x89, 0x04, 0x2a,
    0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x05, 0x03, 0x12, 0x04, 0x89, 0x04, 0x37, 0x38,
    0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x30, 0x04, 0x00, 0x12, 0x06, 0x8b, 0x04, 0x08, 0x9a, 0x04, 0x09,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x04, 0x00, 0x01, 0x12, 0x04, 0x8b, 0x04, 0x0d, 0x13, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x30, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0x8c, 0x04, 0x10, 0x1a, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8c, 0x04, 0x10, 0x15,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0x8c, 0x04, 0x18,
    0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x30, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0x8d, 0x04, 0x10,
    0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8d, 0x04,
    0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0x8d,
    0x04, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x30, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x8e,
    0x04, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04,
    0x8e, 0x04, 0x10, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x04, 0x8e, 0x04, 0x26, 0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x30, 0x04, 0x00, 0x02, 0x03, 0x12,
    0x04, 0x8f, 0x04, 0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x04, 0x8f, 0x04, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x04, 0x8f, 0x04, 0x24, 0x25, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x30, 0x04, 0x00, 0x02,
    0x04, 0x12, 0x04, 0x90, 0x04, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02,
    0x04, 0x01, 0x12, 0x04, 0x90, 0x04, 0x10, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x90, 0x04, 0x27, 0x28, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x30, 0x04,
    0x00, 0x02, 0x05, 0x12, 0x04, 0x91, 0x04, 0x10, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04,
    0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0x91, 0x04, 0x10, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30,
    0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04, 0x91, 0x04, 0x22, 0x23, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x30, 0x04, 0x00, 0x02, 0x06, 0x12, 0x04, 0x92, 0x04, 0x10, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x30, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04, 0x92, 0x04, 0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x30, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x04, 0x92, 0x04, 0x29, 0x2a, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x30, 0x04, 0x00, 0x02, 0x07, 0x12, 0x04, 0x93, 0x04, 0x10, 0x3a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x04, 0x93, 0x04, 0x10, 0x35, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x07, 0x02, 0x12, 0x04, 0x93, 0x04, 0x38, 0x39, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x30, 0x04, 0x00, 0x02, 0x08, 0x12, 0x04, 0x94, 0x04, 0x10, 0x2e, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x04, 0x94, 0x04, 0x10, 0x29,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x08, 0x02, 0x12, 0x04, 0x94, 0x04, 0x2c,
    0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x30, 0x04, 0x00, 0x02, 0x09, 0x12, 0x04, 0x95, 0x04, 0x10,
    0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x04, 0x95, 0x04,
    0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x09, 0x02, 0x12, 0x04, 0x95,
    0x04, 0x29, 0x2a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x30, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x04, 0x96,
    0x04, 0x10, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x04,
    0x96, 0x04, 0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x0a, 0x02, 0x12,
    0x04, 0x96, 0x04, 0x29, 0x2b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x30, 0x04, 0x00, 0x02, 0x0b, 0x12,
    0x04, 0x97, 0x04, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x0b, 0x01,
    0x12, 0x04, 0x97, 0x04, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02, 0x0b,
    0x02, 0x12, 0x04, 0x97, 0x04, 0x2b, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x30, 0x04, 0x00, 0x02,
    0x0c, 0x12, 0x04, 0x98, 0x04, 0x10, 0x36, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00, 0x02,
    0x0c, 0x01, 0x12, 0x04, 0x98, 0x04, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04, 0x00,
    0x02, 0x0c, 0x02, 0x12, 0x04, 0x98, 0x04, 0x33, 0x35, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x30, 0x04,
    0x00, 0x02, 0x0d, 0x12, 0x04, 0x99, 0x04, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30, 0x04,
    0x00, 0x02, 0x0d, 0x01, 0x12, 0x04, 0x99, 0x04, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x30,
    0x04, 0x00, 0x02, 0x0d, 0x02, 0x12, 0x04, 0x99, 0x04, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x31, 0x12, 0x06, 0x9c, 0x04, 0x00, 0xaa, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x31, 0x01,
    0x12, 0x04, 0x9c, 0x04, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x00, 0x12, 0x04,
    0x9d, 0x04, 0x08, 0x51, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x04, 0x12, 0x06, 0x9d,
    0x04, 0x08, 0x9c, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x06, 0x12, 0x04,
    0x9d, 0x04, 0x08, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9d,
    0x04, 0x46, 0x4c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9d, 0x04,
    0x4f, 0x50, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x01, 0x12, 0x04, 0x9e, 0x04, 0x08, 0x3e,
    0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x01, 0x04, 0x12, 0x06, 0x9e, 0x04, 0x08, 0x9d, 0x04,
    0x51, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x01, 0x06, 0x12, 0x04, 0x9e, 0x04, 0x08, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9e, 0x04, 0x25, 0x39, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9e, 0x04, 0x3c, 0x3d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x31, 0x02, 0x02, 0x12, 0x04, 0x9f, 0x04, 0x08, 0x25, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x31, 0x02, 0x02, 0x04, 0x12, 0x06, 0x9f, 0x04, 0x08, 0x9e, 0x04, 0x3e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x31, 0x02, 0x02, 0x05, 0x12, 0x04, 0x9f, 0x04, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x31, 0x02, 0x02, 0x01, 0x12, 0x04, 0x9f, 0x04, 0x0e, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x31, 0x02, 0x02, 0x03, 0x12, 0x04, 0x9f, 0x04, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31,
    0x02, 0x03, 0x12, 0x04, 0xa0, 0x04, 0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x03,
    0x04, 0x12, 0x06, 0xa0, 0x04, 0x08, 0x9f, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02,
    0x03, 0x05, 0x12, 0x04, 0xa0, 0x04, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xa0, 0x04, 0x0e, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x03, 0x03,
    0x12, 0x04, 0xa0, 0x04, 0x1e, 0x1f, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x31, 0x04, 0x00, 0x12, 0x06,
    0xa2, 0x04, 0x08, 0xa9, 0x04, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x04, 0x00, 0x01, 0x12,
    0x04, 0xa2, 0x04, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x31, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x04, 0xa3, 0x04, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x31, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xa3, 0x04, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x31, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x04, 0xa3, 0x04, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x31, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x04, 0xa4, 0x04, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x31, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xa4, 0x04, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x31, 0x04, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x04, 0xa4, 0x04, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x31, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x04, 0xa5, 0x04, 0x10, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x31, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa5, 0x04, 0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x31,
    0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xa5, 0x04, 0x29, 0x2a, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x31, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xa6, 0x04, 0x10, 0x32, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x31, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa6, 0x04, 0x10, 0x2d, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x31, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xa6, 0x04, 0x30, 0x31, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x31, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xa7, 0x04, 0x10, 0x31, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x31, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xa7, 0x04, 0x10, 0x2c, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x31, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xa7, 0x04, 0x2f, 0x30, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x31, 0x04, 0x00, 0x02, 0x05, 0x12, 0x04, 0xa8, 0x04, 0x10, 0x2f, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x31, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0xa8, 0x04, 0x10, 0x2a,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x31, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04, 0xa8, 0x04, 0x2d,
    0x2e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x32, 0x12, 0x06, 0xab, 0x04, 0x00, 0xb7, 0x04, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x32, 0x01, 0x12, 0x04, 0xab, 0x04, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x32, 0x02, 0x00, 0x12, 0x04, 0xac, 0x04, 0x08, 0x52, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x32,
    0x02, 0x00, 0x04, 0x12, 0x06, 0xac, 0x04, 0x08, 0xab, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x32, 0x02, 0x00, 0x06, 0x12, 0x04, 0xac, 0x04, 0x08, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xac, 0x04, 0x47, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xac, 0x04, 0x50, 0x51, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x32, 0x02, 0x01,
    0x12, 0x04, 0xad, 0x04, 0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x01, 0x04, 0x12,
    0x06, 0xad, 0x04, 0x08, 0xac, 0x04, 0x52, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xad, 0x04, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xad, 0x04, 0x0e, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xad, 0x04, 0x1e, 0x1f, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x32, 0x04, 0x00, 0x12, 0x06, 0xaf, 0x04,
    0x08, 0xb6, 0x04, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x04, 0x00, 0x01, 0x12, 0x04, 0xaf,
    0x04, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x32, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xb0,
    0x04, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x32, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xb0, 0x04, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x32, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x04, 0xb0, 0x04, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x32, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x04, 0xb1, 0x04, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x32, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xb1, 0x04, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x32, 0x04, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x04, 0xb1, 0x04, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x32, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x04, 0xb2, 0x04, 0x10, 0x25, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x32, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xb2, 0x04, 0x10, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x32, 0x04, 0x00,
    0x02, 0x02, 0x02, 0x12, 0x04, 0xb2, 0x04, 0x23, 0x24, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x32, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x04, 0xb3, 0x04, 0x10, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x32, 0x04,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xb3, 0x04, 0x10, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x32,
    0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xb3, 0x04, 0x19, 0x1a, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x32, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xb4, 0x04, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x32, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xb4, 0x04, 0x10, 0x24, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x32, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xb4, 0x04, 0x27, 0x28, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x32, 0x04, 0x00, 0x02, 0x05, 0x12, 0x04, 0xb5, 0x04, 0x10, 0x2b, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x32, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0xb5, 0x04, 0x10, 0x26, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x32, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04, 0xb5, 0x04, 0x29, 0x2a, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x33, 0x12, 0x06, 0xb8, 0x04, 0x00, 0xc1, 0x04, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x33, 0x01, 0x12, 0x04, 0xb8, 0x04, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x33,
    0x02, 0x00, 0x12, 0x04, 0xb9, 0x04, 0x08, 0x54, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00,
    0x04, 0x12, 0x06, 0xb9, 0x04, 0x08, 0xb8, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02,
    0x00, 0x06, 0x12, 0x04, 0xb9, 0x04, 0x08, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xb9, 0x04, 0x49, 0x4f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xb9, 0x04, 0x52, 0x53, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x33, 0x02, 0x01, 0x12, 0x04,
    0xba, 0x04, 0x08, 0x1d, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x01, 0x04, 0x12, 0x06, 0xba,
    0x04, 0x08, 0xb9, 0x04, 0x54, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xba, 0x04, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x01, 0x01, 0x12, 0x04, 0xba,
    0x04, 0x0e, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x01, 0x03, 0x12, 0x04, 0xba, 0x04,
    0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x33, 0x02, 0x02, 0x12, 0x04, 0xbb, 0x04, 0x08, 0x39,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x02, 0x04, 0x12, 0x04, 0xbb, 0x04, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x02, 0x06, 0x12, 0x04, 0xbb, 0x04, 0x11, 0x2d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x33, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbb, 0x04, 0x2e, 0x34, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x33, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbb, 0x04, 0x37, 0x38, 0x0a, 0x0e, 0x0a, 0x04,
    0x04, 0x33, 0x04, 0x00, 0x12, 0x06, 0xbd, 0x04, 0x08, 0xc0, 0x04, 0x09, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x33, 0x04, 0x00, 0x01, 0x12, 0x04, 0xbd, 0x04, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x33, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xbe, 0x04, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x33, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbe, 0x04, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x33, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xbe, 0x04, 0x18, 0x19, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x33, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xbf, 0x04, 0x10, 0x1c, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x33, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xbf, 0x04, 0x10, 0x17, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x33, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xbf, 0x04, 0x1a, 0x1b, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x34, 0x12, 0x06, 0xc2, 0x04, 0x00, 0xc4, 0x04, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x34, 0x01, 0x12, 0x04, 0xc2, 0x04, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x34,
    0x02, 0x00, 0x12, 0x04, 0xc3, 0x04, 0x08, 0x1b, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x00,
    0x04, 0x12, 0x06, 0xc3, 0x04, 0x08, 0xc2, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xc3, 0x04, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xc3, 0x04, 0x0f, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xc3, 0x04, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x35, 0x12, 0x06, 0xc5, 0x04,
    0x00, 0xd0, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x35, 0x01, 0x12, 0x04, 0xc5, 0x04, 0x08,
    0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x35, 0x02, 0x00, 0x12, 0x04, 0xc6, 0x04, 0x08, 0x4e, 0x0a,
    0x0f, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x04, 0x12, 0x06, 0xc6, 0x04, 0x08, 0xc5, 0x04, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc6, 0x04, 0x08, 0x42, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc6, 0x04, 0x43, 0x49, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc6, 0x04, 0x4c, 0x4d, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x35, 0x02, 0x01, 0x12, 0x04, 0xc7, 0x04, 0x08, 0x3e, 0x0a, 0x0f, 0x0a, 0x05, 0x04,
    0x35, 0x02, 0x01, 0x04, 0x12, 0x06, 0xc7, 0x04, 0x08, 0xc6, 0x04, 0x4e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x35, 0x02, 0x01, 0x06, 0x12, 0x04, 0xc7, 0x04, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x35, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc7, 0x04, 0x2a, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xc7, 0x04, 0x3c, 0x3d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x35, 0x04,
    0x00, 0x12, 0x06, 0xc9, 0x04, 0x08, 0xcf, 0x04, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x04,
    0x00, 0x01, 0x12, 0x04, 0xc9, 0x04, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x35, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x04, 0xca, 0x04, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xca, 0x04, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x04,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xca, 0x04, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x35,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xcb, 0x04, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcb, 0x04, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x35, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xcb, 0x04, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x35, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xcc, 0x04, 0x10, 0x2b, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x35, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xcc, 0x04, 0x10, 0x26, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x35, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xcc, 0x04, 0x29, 0x2a, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x35, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xcd, 0x04, 0x10, 0x26, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x35, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xcd, 0x04, 0x10, 0x21, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x35, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xcd, 0x04, 0x24, 0x25,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x35, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xce, 0x04, 0x10, 0x23,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xce, 0x04, 0x10,
    0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xce, 0x04,
    0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x36, 0x12, 0x06, 0xd1, 0x04, 0x00, 0xe6, 0x04, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x36, 0x01, 0x12, 0x04, 0xd1, 0x04, 0x08, 0x19, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x36, 0x02, 0x00, 0x12, 0x04, 0xd2, 0x04, 0x08, 0x3d, 0x0a, 0x0f, 0x0a, 0x05, 0x04,
    0x36, 0x02, 0x00, 0x04, 0x12, 0x06, 0xd2, 0x04, 0x08, 0xd1, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x36, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd2, 0x04, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x36, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd2, 0x04, 0x2c, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xd2, 0x04, 0x3b, 0x3c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36, 0x02,
    0x01, 0x12, 0x04, 0xd3, 0x04, 0x08, 0x22, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x01, 0x04,
    0x12, 0x06, 0xd3, 0x04, 0x08, 0xd2, 0x04, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x01,
    0x06, 0x12, 0x04, 0xd3, 0x04, 0x08, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xd3, 0x04, 0x13, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xd3, 0x04, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36, 0x02, 0x02, 0x12, 0x04, 0xd4,
    0x04, 0x08, 0x1a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x02, 0x04, 0x12, 0x06, 0xd4, 0x04,
    0x08, 0xd3, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x02, 0x06, 0x12, 0x04, 0xd4,
    0x04, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd4, 0x04,
    0x0f, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd4, 0x04, 0x18,
    0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36, 0x02, 0x03, 0x12, 0x04, 0xd5, 0x04, 0x08, 0x4c, 0x0a,
    0x0f, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x03, 0x04, 0x12, 0x06, 0xd5, 0x04, 0x08, 0xd4, 0x04, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x03, 0x06, 0x12, 0x04, 0xd5, 0x04, 0x08, 0x33, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd5, 0x04, 0x34, 0x47, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x36, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd5, 0x04, 0x4a, 0x4b, 0x0a, 0x0e, 0x0a,
    0x04, 0x04, 0x36, 0x04, 0x00, 0x12, 0x06, 0xd7, 0x04, 0x08, 0xda, 0x04, 0x09, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x36, 0x04, 0x00, 0x01, 0x12, 0x04, 0xd7, 0x04, 0x0d, 0x17, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x36, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xd8, 0x04, 0x10, 0x19, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x36, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd8, 0x04, 0x10, 0x14, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x36, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xd8, 0x04, 0x17, 0x18, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x36, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xd9, 0x04, 0x10, 0x1b, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x36, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd9, 0x04, 0x10, 0x16, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x36, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xd9, 0x04, 0x19, 0x1a,
    0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x36, 0x04, 0x01, 0x12, 0x06, 0xdc, 0x04, 0x08, 0xe5, 0x04, 0x09,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x04, 0x01, 0x01, 0x12, 0x04, 0xdc, 0x04, 0x0d, 0x13, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x36, 0x04, 0x01, 0x02, 0x00, 0x12, 0x04, 0xdd, 0x04, 0x10, 0x24, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x36, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdd, 0x04, 0x10, 0x1f,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x36, 0x04, 0x01, 0x02, 0x00, 0x02, 0x12, 0x04, 0xdd, 0x04, 0x22,
    0x23, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x36, 0x04, 0x01, 0x02, 0x01, 0x12, 0x04, 0xde, 0x04, 0x10,
    0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x36, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x04, 0xde, 0x04,
    0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x36, 0x04, 0x01, 0x02, 0x01, 0x02, 0x12, 0x04, 0xde,
    0x04, 0x24, 0x25, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x36, 0x04, 0x01, 0x02, 0x02, 0x12, 0x04, 0xdf,
    0x04, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x36, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x04,
    0xdf, 0x04, 0x10, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x36, 0x04, 0x01, 0x02, 0x02, 0x02, 0x12,
    0x04, 0xdf, 0x04, 0x26, 0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x36, 0x04, 0x01, 0x02, 0x03, 0x12,
    0x04, 0xe0, 0x04, 0x10, 0x25, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x36, 0x04, 0x01, 0x02, 0x03, 0x01,
    0x12, 0x04, 0xe0, 0x04, 0x10, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x36, 0x04, 0x01, 0x02, 0x03,
    0x02, 0x12, 0x04, 0xe0, 0x04, 0x23, 0x24, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x36, 0x04, 0x01, 0x02,
    0x04, 0x12, 0x04, 0xe1, 0x04, 0x10, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x36, 0x04, 0x01, 0x02,
    0x04, 0x01, 0x12, 0x04, 0xe1, 0x04, 0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x36, 0x04, 0x01,
    0x02, 0x04, 0x02, 0x12, 0x04, 0xe1, 0x04, 0x29, 0x2a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x36, 0x04,
    0x01, 0x02, 0x05, 0x12, 0x04, 0xe2, 0x04, 0x10, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x36, 0x04,
    0x01, 0x02, 0x05, 0x01, 0x12, 0x04, 0xe2, 0x04, 0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x36,
    0x04, 0x01, 0x02, 0x05, 0x02, 0x12, 0x04, 0xe2, 0x04, 0x29, 0x2a, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x36, 0x04, 0x01, 0x02, 0x06, 0x12, 0x04, 0xe3, 0x04, 0x10, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x36, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x04, 0xe3, 0x04, 0x10, 0x2a, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x36, 0x04, 0x01, 0x02, 0x06, 0x02, 0x12, 0x04, 0xe3, 0x04, 0x2d, 0x2e, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x36, 0x04, 0x01, 0x02, 0x07, 0x12, 0x04, 0xe4, 0x04, 0x10, 0x2b, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x36, 0x04, 0x01, 0x02, 0x07, 0x01, 0x12, 0x04, 0xe4, 0x04, 0x10, 0x26, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x36, 0x04, 0x01, 0x02, 0x07, 0x02, 0x12, 0x04, 0xe4, 0x04, 0x29, 0x2a, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x37, 0x12, 0x06, 0xe7, 0x04, 0x00, 0xf6, 0x04, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x37, 0x01, 0x12, 0x04, 0xe7, 0x04, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x37,
    0x02, 0x00, 0x12, 0x04, 0xe8, 0x04, 0x08, 0x1c, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x00,
    0x04, 0x12, 0x06, 0xe8, 0x04, 0x08, 0xe7, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xe8, 0x04, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xe8, 0x04, 0x0f, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xe8, 0x04, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x37, 0x02, 0x01, 0x12, 0x04,
    0xe9, 0x04, 0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x01, 0x04, 0x12, 0x06, 0xe9,
    0x04, 0x08, 0xe8, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xe9, 0x04, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe9,
    0x04, 0x0f, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe9, 0x04,
    0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x37, 0x02, 0x02, 0x12, 0x04, 0xea, 0x04, 0x08, 0x1f,
    0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x02, 0x04, 0x12, 0x06, 0xea, 0x04, 0x08, 0xe9, 0x04,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x02, 0x05, 0x12, 0x04, 0xea, 0x04, 0x08, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x02, 0x01, 0x12, 0x04, 0xea, 0x04, 0x0d, 0x1a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x02, 0x03, 0x12, 0x04, 0xea, 0x04, 0x1d, 0x1e, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x37, 0x02, 0x03, 0x12, 0x04, 0xeb, 0x04, 0x08, 0x51, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x37, 0x02, 0x03, 0x04, 0x12, 0x06, 0xeb, 0x04, 0x08, 0xea, 0x04, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x37, 0x02, 0x03, 0x06, 0x12, 0x04, 0xeb, 0x04, 0x08, 0x45, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x37, 0x02, 0x03, 0x01, 0x12, 0x04, 0xeb, 0x04, 0x46, 0x4c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x37, 0x02, 0x03, 0x03, 0x12, 0x04, 0xeb, 0x04, 0x4f, 0x50, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x37,
    0x02, 0x04, 0x12, 0x04, 0xec, 0x04, 0x08, 0x37, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x04,
    0x04, 0x12, 0x06, 0xec, 0x04, 0x08, 0xeb, 0x04, 0x51, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02,
    0x04, 0x06, 0x12, 0x04, 0xec, 0x04, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x04,
    0x01, 0x12, 0x04, 0xec, 0x04, 0x24, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x04, 0x03,
    0x12, 0x04, 0xec, 0x04, 0x35, 0x36, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x37, 0x04, 0x00, 0x12, 0x06,
    0xee, 0x04, 0x08, 0xf5, 0x04, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x04, 0x00, 0x01, 0x12,
    0x04, 0xee, 0x04, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x37, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x04, 0xef, 0x04, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x37, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xef, 0x04, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x37, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x04, 0xef, 0x04, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x37, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x04, 0xf0, 0x04, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x37, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xf0, 0x04, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x37, 0x04, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x04, 0xf0, 0x04, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x37, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x04, 0xf1, 0x04, 0x10, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x37, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf1, 0x04, 0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x37,
    0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xf1, 0x04, 0x29, 0x2a, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x37, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xf2, 0x04, 0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x37, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf2, 0x04, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x37, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xf2, 0x04, 0x25, 0x26, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x37, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xf3, 0x04, 0x10, 0x22, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x37, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xf3, 0x04, 0x10, 0x1d, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x37, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xf3, 0x04, 0x20, 0x21, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x37, 0x04, 0x00, 0x02, 0x05, 0x12, 0x04, 0xf4, 0x04, 0x10, 0x30, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x37, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0xf4, 0x04, 0x10, 0x2b,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x37, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04, 0xf4, 0x04, 0x2e,
    0x2f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
