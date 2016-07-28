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
pub struct GymState {
    // message fields
    fort_data: ::protobuf::SingularPtrField<super::POGOProtos_Map_Fort::FortData>,
    memberships: ::protobuf::RepeatedField<GymMembership>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GymState {}

impl GymState {
    pub fn new() -> GymState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GymState {
        static mut instance: ::protobuf::lazy::Lazy<GymState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GymState,
        };
        unsafe {
            instance.get(|| {
                GymState {
                    fort_data: ::protobuf::SingularPtrField::none(),
                    memberships: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Map.Fort.FortData fort_data = 1;

    pub fn clear_fort_data(&mut self) {
        self.fort_data.clear();
    }

    pub fn has_fort_data(&self) -> bool {
        self.fort_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fort_data(&mut self, v: super::POGOProtos_Map_Fort::FortData) {
        self.fort_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fort_data(&mut self) -> &mut super::POGOProtos_Map_Fort::FortData {
        if self.fort_data.is_none() {
            self.fort_data.set_default();
        };
        self.fort_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_fort_data(&mut self) -> super::POGOProtos_Map_Fort::FortData {
        self.fort_data.take().unwrap_or_else(|| super::POGOProtos_Map_Fort::FortData::new())
    }

    pub fn get_fort_data(&self) -> &super::POGOProtos_Map_Fort::FortData {
        self.fort_data.as_ref().unwrap_or_else(|| super::POGOProtos_Map_Fort::FortData::default_instance())
    }

    // repeated .POGOProtos.Data.Gym.GymMembership memberships = 2;

    pub fn clear_memberships(&mut self) {
        self.memberships.clear();
    }

    // Param is passed by value, moved
    pub fn set_memberships(&mut self, v: ::protobuf::RepeatedField<GymMembership>) {
        self.memberships = v;
    }

    // Mutable pointer to the field.
    pub fn mut_memberships(&mut self) -> &mut ::protobuf::RepeatedField<GymMembership> {
        &mut self.memberships
    }

    // Take field
    pub fn take_memberships(&mut self) -> ::protobuf::RepeatedField<GymMembership> {
        ::std::mem::replace(&mut self.memberships, ::protobuf::RepeatedField::new())
    }

    pub fn get_memberships(&self) -> &[GymMembership] {
        &self.memberships
    }
}

impl ::protobuf::Message for GymState {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fort_data));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.memberships));
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
        for value in &self.fort_data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.memberships {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fort_data.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.memberships {
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
        ::std::any::TypeId::of::<GymState>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GymState {
    fn new() -> GymState {
        GymState::new()
    }

    fn descriptor_static(_: ::std::option::Option<GymState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fort_data",
                    GymState::has_fort_data,
                    GymState::get_fort_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "memberships",
                    GymState::get_memberships,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GymState>(
                    "GymState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GymState {
    fn clear(&mut self) {
        self.clear_fort_data();
        self.clear_memberships();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GymState {
    fn eq(&self, other: &GymState) -> bool {
        self.fort_data == other.fort_data &&
        self.memberships == other.memberships &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GymState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GymMembership {
    // message fields
    pokemon_data: ::protobuf::SingularPtrField<super::POGOProtos_Data::PokemonData>,
    trainer_public_profile: ::protobuf::SingularPtrField<super::POGOProtos_Data_Player::PlayerPublicProfile>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GymMembership {}

impl GymMembership {
    pub fn new() -> GymMembership {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GymMembership {
        static mut instance: ::protobuf::lazy::Lazy<GymMembership> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GymMembership,
        };
        unsafe {
            instance.get(|| {
                GymMembership {
                    pokemon_data: ::protobuf::SingularPtrField::none(),
                    trainer_public_profile: ::protobuf::SingularPtrField::none(),
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
}

impl ::protobuf::Message for GymMembership {
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.trainer_public_profile));
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
        for value in &self.trainer_public_profile {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(v) = self.trainer_public_profile.as_ref() {
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
        ::std::any::TypeId::of::<GymMembership>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GymMembership {
    fn new() -> GymMembership {
        GymMembership::new()
    }

    fn descriptor_static(_: ::std::option::Option<GymMembership>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pokemon_data",
                    GymMembership::has_pokemon_data,
                    GymMembership::get_pokemon_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "trainer_public_profile",
                    GymMembership::has_trainer_public_profile,
                    GymMembership::get_trainer_public_profile,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GymMembership>(
                    "GymMembership",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GymMembership {
    fn clear(&mut self) {
        self.clear_pokemon_data();
        self.clear_trainer_public_profile();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GymMembership {
    fn eq(&self, other: &GymMembership) -> bool {
        self.pokemon_data == other.pokemon_data &&
        self.trainer_public_profile == other.trainer_public_profile &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GymMembership {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x19, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x2e, 0x47, 0x79, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x47, 0x79, 0x6d,
    0x1a, 0x19, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70,
    0x2e, 0x46, 0x6f, 0x72, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x15, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x1a, 0x1c, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44,
    0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x50, 0x00, 0x50, 0x01, 0x50, 0x02, 0x22, 0x8c, 0x01, 0x0a, 0x08, 0x47, 0x79, 0x6d, 0x53, 0x74,
    0x61, 0x74, 0x65, 0x12, 0x3a, 0x0a, 0x09, 0x66, 0x6f, 0x72, 0x74, 0x5f, 0x64, 0x61, 0x74, 0x61,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x2e, 0x46, 0x6f, 0x72,
    0x74, 0x44, 0x61, 0x74, 0x61, 0x52, 0x08, 0x66, 0x6f, 0x72, 0x74, 0x44, 0x61, 0x74, 0x61, 0x12,
    0x44, 0x0a, 0x0b, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x47, 0x79, 0x6d, 0x2e, 0x47, 0x79, 0x6d, 0x4d, 0x65,
    0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x52, 0x0b, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x73, 0x68, 0x69, 0x70, 0x73, 0x22, 0xb3, 0x01, 0x0a, 0x0d, 0x47, 0x79, 0x6d, 0x4d, 0x65, 0x6d,
    0x62, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x12, 0x3f, 0x0a, 0x0c, 0x70, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e,
    0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x52, 0x0b, 0x70, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x12, 0x61, 0x0a, 0x16, 0x74, 0x72, 0x61, 0x69,
    0x6e, 0x65, 0x72, 0x5f, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x70, 0x72, 0x6f, 0x66, 0x69,
    0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x50, 0x72,
    0x6f, 0x66, 0x69, 0x6c, 0x65, 0x52, 0x14, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x50, 0x75,
    0x62, 0x6c, 0x69, 0x63, 0x50, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x4a, 0xa5, 0x03, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x0e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12,
    0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x1b, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x00,
    0x12, 0x03, 0x03, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x0e, 0x29,
    0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x04, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x04, 0x0e, 0x25, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x05, 0x07,
    0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x05, 0x0e, 0x2c, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x07, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x08,
    0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x08, 0x08, 0x07, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x08, 0x08, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x26, 0x2f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x09, 0x08, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x09, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x09, 0x11, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x34,
    0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x09, 0x42, 0x43, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0b, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x0c, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0c,
    0x08, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0c, 0x08,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x25, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x34, 0x35, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x4f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x0d, 0x08, 0x0c, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x0d, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0d, 0x34, 0x4a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0d, 0x4d, 0x4e, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
