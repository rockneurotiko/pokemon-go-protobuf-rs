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
pub struct FortLureInfo {
    // message fields
    fort_id: ::protobuf::SingularField<::std::string::String>,
    encounter_id: ::std::option::Option<u64>,
    active_pokemon_id: ::std::option::Option<super::POGOProtos_Enums::PokemonId>,
    lure_expires_timestamp_ms: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FortLureInfo {}

impl FortLureInfo {
    pub fn new() -> FortLureInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FortLureInfo {
        static mut instance: ::protobuf::lazy::Lazy<FortLureInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FortLureInfo,
        };
        unsafe {
            instance.get(|| {
                FortLureInfo {
                    fort_id: ::protobuf::SingularField::none(),
                    encounter_id: ::std::option::Option::None,
                    active_pokemon_id: ::std::option::Option::None,
                    lure_expires_timestamp_ms: ::std::option::Option::None,
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

    // optional .POGOProtos.Enums.PokemonId active_pokemon_id = 3;

    pub fn clear_active_pokemon_id(&mut self) {
        self.active_pokemon_id = ::std::option::Option::None;
    }

    pub fn has_active_pokemon_id(&self) -> bool {
        self.active_pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_active_pokemon_id(&mut self, v: super::POGOProtos_Enums::PokemonId) {
        self.active_pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_active_pokemon_id(&self) -> super::POGOProtos_Enums::PokemonId {
        self.active_pokemon_id.unwrap_or(super::POGOProtos_Enums::PokemonId::MISSINGNO)
    }

    // optional int64 lure_expires_timestamp_ms = 4;

    pub fn clear_lure_expires_timestamp_ms(&mut self) {
        self.lure_expires_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_lure_expires_timestamp_ms(&self) -> bool {
        self.lure_expires_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lure_expires_timestamp_ms(&mut self, v: i64) {
        self.lure_expires_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_lure_expires_timestamp_ms(&self) -> i64 {
        self.lure_expires_timestamp_ms.unwrap_or(0)
    }
}

impl ::protobuf::Message for FortLureInfo {
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
                    self.active_pokemon_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.lure_expires_timestamp_ms = ::std::option::Option::Some(tmp);
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
        if self.encounter_id.is_some() {
            my_size += 9;
        };
        for value in &self.active_pokemon_id {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in &self.lure_expires_timestamp_ms {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fort_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.encounter_id {
            try!(os.write_fixed64(2, v));
        };
        if let Some(v) = self.active_pokemon_id {
            try!(os.write_enum(3, v.value()));
        };
        if let Some(v) = self.lure_expires_timestamp_ms {
            try!(os.write_int64(4, v));
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
        ::std::any::TypeId::of::<FortLureInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FortLureInfo {
    fn new() -> FortLureInfo {
        FortLureInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<FortLureInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "fort_id",
                    FortLureInfo::has_fort_id,
                    FortLureInfo::get_fort_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "encounter_id",
                    FortLureInfo::has_encounter_id,
                    FortLureInfo::get_encounter_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "active_pokemon_id",
                    FortLureInfo::has_active_pokemon_id,
                    FortLureInfo::get_active_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "lure_expires_timestamp_ms",
                    FortLureInfo::has_lure_expires_timestamp_ms,
                    FortLureInfo::get_lure_expires_timestamp_ms,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FortLureInfo>(
                    "FortLureInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FortLureInfo {
    fn clear(&mut self) {
        self.clear_fort_id();
        self.clear_encounter_id();
        self.clear_active_pokemon_id();
        self.clear_lure_expires_timestamp_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FortLureInfo {
    fn eq(&self, other: &FortLureInfo) -> bool {
        self.fort_id == other.fort_id &&
        self.encounter_id == other.encounter_id &&
        self.active_pokemon_id == other.active_pokemon_id &&
        self.lure_expires_timestamp_ms == other.lure_expires_timestamp_ms &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FortLureInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FortSummary {
    // message fields
    fort_summary_id: ::protobuf::SingularField<::std::string::String>,
    last_modified_timestamp_ms: ::std::option::Option<i64>,
    latitude: ::std::option::Option<f64>,
    longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FortSummary {}

impl FortSummary {
    pub fn new() -> FortSummary {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FortSummary {
        static mut instance: ::protobuf::lazy::Lazy<FortSummary> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FortSummary,
        };
        unsafe {
            instance.get(|| {
                FortSummary {
                    fort_summary_id: ::protobuf::SingularField::none(),
                    last_modified_timestamp_ms: ::std::option::Option::None,
                    latitude: ::std::option::Option::None,
                    longitude: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string fort_summary_id = 1;

    pub fn clear_fort_summary_id(&mut self) {
        self.fort_summary_id.clear();
    }

    pub fn has_fort_summary_id(&self) -> bool {
        self.fort_summary_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fort_summary_id(&mut self, v: ::std::string::String) {
        self.fort_summary_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fort_summary_id(&mut self) -> &mut ::std::string::String {
        if self.fort_summary_id.is_none() {
            self.fort_summary_id.set_default();
        };
        self.fort_summary_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_fort_summary_id(&mut self) -> ::std::string::String {
        self.fort_summary_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_fort_summary_id(&self) -> &str {
        match self.fort_summary_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
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
}

impl ::protobuf::Message for FortSummary {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fort_summary_id));
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
        for value in &self.fort_summary_id {
            my_size += ::protobuf::rt::string_size(1, &value);
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fort_summary_id.as_ref() {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<FortSummary>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FortSummary {
    fn new() -> FortSummary {
        FortSummary::new()
    }

    fn descriptor_static(_: ::std::option::Option<FortSummary>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "fort_summary_id",
                    FortSummary::has_fort_summary_id,
                    FortSummary::get_fort_summary_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "last_modified_timestamp_ms",
                    FortSummary::has_last_modified_timestamp_ms,
                    FortSummary::get_last_modified_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "latitude",
                    FortSummary::has_latitude,
                    FortSummary::get_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "longitude",
                    FortSummary::has_longitude,
                    FortSummary::get_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FortSummary>(
                    "FortSummary",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FortSummary {
    fn clear(&mut self) {
        self.clear_fort_summary_id();
        self.clear_last_modified_timestamp_ms();
        self.clear_latitude();
        self.clear_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FortSummary {
    fn eq(&self, other: &FortSummary) -> bool {
        self.fort_summary_id == other.fort_summary_id &&
        self.last_modified_timestamp_ms == other.last_modified_timestamp_ms &&
        self.latitude == other.latitude &&
        self.longitude == other.longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FortSummary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FortData {
    // message fields
    id: ::protobuf::SingularField<::std::string::String>,
    last_modified_timestamp_ms: ::std::option::Option<i64>,
    latitude: ::std::option::Option<f64>,
    longitude: ::std::option::Option<f64>,
    enabled: ::std::option::Option<bool>,
    field_type: ::std::option::Option<FortType>,
    owned_by_team: ::std::option::Option<super::POGOProtos_Enums::TeamColor>,
    guard_pokemon_id: ::std::option::Option<super::POGOProtos_Enums::PokemonId>,
    guard_pokemon_cp: ::std::option::Option<i32>,
    gym_points: ::std::option::Option<i64>,
    is_in_battle: ::std::option::Option<bool>,
    active_fort_modifier: ::std::vec::Vec<super::POGOProtos_Inventory_Item::ItemId>,
    lure_info: ::protobuf::SingularPtrField<FortLureInfo>,
    cooldown_complete_timestamp_ms: ::std::option::Option<i64>,
    sponsor: ::std::option::Option<FortSponsor>,
    rendering_type: ::std::option::Option<FortRenderingType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FortData {}

impl FortData {
    pub fn new() -> FortData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FortData {
        static mut instance: ::protobuf::lazy::Lazy<FortData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FortData,
        };
        unsafe {
            instance.get(|| {
                FortData {
                    id: ::protobuf::SingularField::none(),
                    last_modified_timestamp_ms: ::std::option::Option::None,
                    latitude: ::std::option::Option::None,
                    longitude: ::std::option::Option::None,
                    enabled: ::std::option::Option::None,
                    field_type: ::std::option::Option::None,
                    owned_by_team: ::std::option::Option::None,
                    guard_pokemon_id: ::std::option::Option::None,
                    guard_pokemon_cp: ::std::option::Option::None,
                    gym_points: ::std::option::Option::None,
                    is_in_battle: ::std::option::Option::None,
                    active_fort_modifier: ::std::vec::Vec::new(),
                    lure_info: ::protobuf::SingularPtrField::none(),
                    cooldown_complete_timestamp_ms: ::std::option::Option::None,
                    sponsor: ::std::option::Option::None,
                    rendering_type: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        self.id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        match self.id.as_ref() {
            Some(v) => &v,
            None => "",
        }
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

    // optional bool enabled = 8;

    pub fn clear_enabled(&mut self) {
        self.enabled = ::std::option::Option::None;
    }

    pub fn has_enabled(&self) -> bool {
        self.enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enabled(&mut self, v: bool) {
        self.enabled = ::std::option::Option::Some(v);
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled.unwrap_or(false)
    }

    // optional .POGOProtos.Map.Fort.FortType type = 9;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: FortType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> FortType {
        self.field_type.unwrap_or(FortType::GYM)
    }

    // optional .POGOProtos.Enums.TeamColor owned_by_team = 5;

    pub fn clear_owned_by_team(&mut self) {
        self.owned_by_team = ::std::option::Option::None;
    }

    pub fn has_owned_by_team(&self) -> bool {
        self.owned_by_team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owned_by_team(&mut self, v: super::POGOProtos_Enums::TeamColor) {
        self.owned_by_team = ::std::option::Option::Some(v);
    }

    pub fn get_owned_by_team(&self) -> super::POGOProtos_Enums::TeamColor {
        self.owned_by_team.unwrap_or(super::POGOProtos_Enums::TeamColor::NEUTRAL)
    }

    // optional .POGOProtos.Enums.PokemonId guard_pokemon_id = 6;

    pub fn clear_guard_pokemon_id(&mut self) {
        self.guard_pokemon_id = ::std::option::Option::None;
    }

    pub fn has_guard_pokemon_id(&self) -> bool {
        self.guard_pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guard_pokemon_id(&mut self, v: super::POGOProtos_Enums::PokemonId) {
        self.guard_pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_guard_pokemon_id(&self) -> super::POGOProtos_Enums::PokemonId {
        self.guard_pokemon_id.unwrap_or(super::POGOProtos_Enums::PokemonId::MISSINGNO)
    }

    // optional int32 guard_pokemon_cp = 7;

    pub fn clear_guard_pokemon_cp(&mut self) {
        self.guard_pokemon_cp = ::std::option::Option::None;
    }

    pub fn has_guard_pokemon_cp(&self) -> bool {
        self.guard_pokemon_cp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guard_pokemon_cp(&mut self, v: i32) {
        self.guard_pokemon_cp = ::std::option::Option::Some(v);
    }

    pub fn get_guard_pokemon_cp(&self) -> i32 {
        self.guard_pokemon_cp.unwrap_or(0)
    }

    // optional int64 gym_points = 10;

    pub fn clear_gym_points(&mut self) {
        self.gym_points = ::std::option::Option::None;
    }

    pub fn has_gym_points(&self) -> bool {
        self.gym_points.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gym_points(&mut self, v: i64) {
        self.gym_points = ::std::option::Option::Some(v);
    }

    pub fn get_gym_points(&self) -> i64 {
        self.gym_points.unwrap_or(0)
    }

    // optional bool is_in_battle = 11;

    pub fn clear_is_in_battle(&mut self) {
        self.is_in_battle = ::std::option::Option::None;
    }

    pub fn has_is_in_battle(&self) -> bool {
        self.is_in_battle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_in_battle(&mut self, v: bool) {
        self.is_in_battle = ::std::option::Option::Some(v);
    }

    pub fn get_is_in_battle(&self) -> bool {
        self.is_in_battle.unwrap_or(false)
    }

    // repeated .POGOProtos.Inventory.Item.ItemId active_fort_modifier = 12;

    pub fn clear_active_fort_modifier(&mut self) {
        self.active_fort_modifier.clear();
    }

    // Param is passed by value, moved
    pub fn set_active_fort_modifier(&mut self, v: ::std::vec::Vec<super::POGOProtos_Inventory_Item::ItemId>) {
        self.active_fort_modifier = v;
    }

    // Mutable pointer to the field.
    pub fn mut_active_fort_modifier(&mut self) -> &mut ::std::vec::Vec<super::POGOProtos_Inventory_Item::ItemId> {
        &mut self.active_fort_modifier
    }

    // Take field
    pub fn take_active_fort_modifier(&mut self) -> ::std::vec::Vec<super::POGOProtos_Inventory_Item::ItemId> {
        ::std::mem::replace(&mut self.active_fort_modifier, ::std::vec::Vec::new())
    }

    pub fn get_active_fort_modifier(&self) -> &[super::POGOProtos_Inventory_Item::ItemId] {
        &self.active_fort_modifier
    }

    // optional .POGOProtos.Map.Fort.FortLureInfo lure_info = 13;

    pub fn clear_lure_info(&mut self) {
        self.lure_info.clear();
    }

    pub fn has_lure_info(&self) -> bool {
        self.lure_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lure_info(&mut self, v: FortLureInfo) {
        self.lure_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lure_info(&mut self) -> &mut FortLureInfo {
        if self.lure_info.is_none() {
            self.lure_info.set_default();
        };
        self.lure_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_lure_info(&mut self) -> FortLureInfo {
        self.lure_info.take().unwrap_or_else(|| FortLureInfo::new())
    }

    pub fn get_lure_info(&self) -> &FortLureInfo {
        self.lure_info.as_ref().unwrap_or_else(|| FortLureInfo::default_instance())
    }

    // optional int64 cooldown_complete_timestamp_ms = 14;

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

    // optional .POGOProtos.Map.Fort.FortSponsor sponsor = 15;

    pub fn clear_sponsor(&mut self) {
        self.sponsor = ::std::option::Option::None;
    }

    pub fn has_sponsor(&self) -> bool {
        self.sponsor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sponsor(&mut self, v: FortSponsor) {
        self.sponsor = ::std::option::Option::Some(v);
    }

    pub fn get_sponsor(&self) -> FortSponsor {
        self.sponsor.unwrap_or(FortSponsor::UNSET_SPONSOR)
    }

    // optional .POGOProtos.Map.Fort.FortRenderingType rendering_type = 16;

    pub fn clear_rendering_type(&mut self) {
        self.rendering_type = ::std::option::Option::None;
    }

    pub fn has_rendering_type(&self) -> bool {
        self.rendering_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rendering_type(&mut self, v: FortRenderingType) {
        self.rendering_type = ::std::option::Option::Some(v);
    }

    pub fn get_rendering_type(&self) -> FortRenderingType {
        self.rendering_type.unwrap_or(FortRenderingType::DEFAULT)
    }
}

impl ::protobuf::Message for FortData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.id));
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
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.enabled = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.owned_by_team = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.guard_pokemon_id = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.guard_pokemon_cp = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.gym_points = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_in_battle = ::std::option::Option::Some(tmp);
                },
                12 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.active_fort_modifier));
                },
                13 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lure_info));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.cooldown_complete_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.sponsor = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.rendering_type = ::std::option::Option::Some(tmp);
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
        for value in &self.id {
            my_size += ::protobuf::rt::string_size(1, &value);
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
        if self.enabled.is_some() {
            my_size += 2;
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(9, *value);
        };
        for value in &self.owned_by_team {
            my_size += ::protobuf::rt::enum_size(5, *value);
        };
        for value in &self.guard_pokemon_id {
            my_size += ::protobuf::rt::enum_size(6, *value);
        };
        for value in &self.guard_pokemon_cp {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.gym_points {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.is_in_battle.is_some() {
            my_size += 2;
        };
        for value in &self.active_fort_modifier {
            my_size += ::protobuf::rt::enum_size(12, *value);
        };
        for value in &self.lure_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.cooldown_complete_timestamp_ms {
            my_size += ::protobuf::rt::value_size(14, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.sponsor {
            my_size += ::protobuf::rt::enum_size(15, *value);
        };
        for value in &self.rendering_type {
            my_size += ::protobuf::rt::enum_size(16, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id.as_ref() {
            try!(os.write_string(1, &v));
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
        if let Some(v) = self.enabled {
            try!(os.write_bool(8, v));
        };
        if let Some(v) = self.field_type {
            try!(os.write_enum(9, v.value()));
        };
        if let Some(v) = self.owned_by_team {
            try!(os.write_enum(5, v.value()));
        };
        if let Some(v) = self.guard_pokemon_id {
            try!(os.write_enum(6, v.value()));
        };
        if let Some(v) = self.guard_pokemon_cp {
            try!(os.write_int32(7, v));
        };
        if let Some(v) = self.gym_points {
            try!(os.write_int64(10, v));
        };
        if let Some(v) = self.is_in_battle {
            try!(os.write_bool(11, v));
        };
        for v in &self.active_fort_modifier {
            try!(os.write_enum(12, v.value()));
        };
        if let Some(v) = self.lure_info.as_ref() {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cooldown_complete_timestamp_ms {
            try!(os.write_int64(14, v));
        };
        if let Some(v) = self.sponsor {
            try!(os.write_enum(15, v.value()));
        };
        if let Some(v) = self.rendering_type {
            try!(os.write_enum(16, v.value()));
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
        ::std::any::TypeId::of::<FortData>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FortData {
    fn new() -> FortData {
        FortData::new()
    }

    fn descriptor_static(_: ::std::option::Option<FortData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "id",
                    FortData::has_id,
                    FortData::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "last_modified_timestamp_ms",
                    FortData::has_last_modified_timestamp_ms,
                    FortData::get_last_modified_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "latitude",
                    FortData::has_latitude,
                    FortData::get_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "longitude",
                    FortData::has_longitude,
                    FortData::get_longitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "enabled",
                    FortData::has_enabled,
                    FortData::get_enabled,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    FortData::has_field_type,
                    FortData::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "owned_by_team",
                    FortData::has_owned_by_team,
                    FortData::get_owned_by_team,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "guard_pokemon_id",
                    FortData::has_guard_pokemon_id,
                    FortData::get_guard_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "guard_pokemon_cp",
                    FortData::has_guard_pokemon_cp,
                    FortData::get_guard_pokemon_cp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "gym_points",
                    FortData::has_gym_points,
                    FortData::get_gym_points,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_in_battle",
                    FortData::has_is_in_battle,
                    FortData::get_is_in_battle,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "active_fort_modifier",
                    FortData::get_active_fort_modifier,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "lure_info",
                    FortData::has_lure_info,
                    FortData::get_lure_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "cooldown_complete_timestamp_ms",
                    FortData::has_cooldown_complete_timestamp_ms,
                    FortData::get_cooldown_complete_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "sponsor",
                    FortData::has_sponsor,
                    FortData::get_sponsor,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "rendering_type",
                    FortData::has_rendering_type,
                    FortData::get_rendering_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FortData>(
                    "FortData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FortData {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_last_modified_timestamp_ms();
        self.clear_latitude();
        self.clear_longitude();
        self.clear_enabled();
        self.clear_field_type();
        self.clear_owned_by_team();
        self.clear_guard_pokemon_id();
        self.clear_guard_pokemon_cp();
        self.clear_gym_points();
        self.clear_is_in_battle();
        self.clear_active_fort_modifier();
        self.clear_lure_info();
        self.clear_cooldown_complete_timestamp_ms();
        self.clear_sponsor();
        self.clear_rendering_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FortData {
    fn eq(&self, other: &FortData) -> bool {
        self.id == other.id &&
        self.last_modified_timestamp_ms == other.last_modified_timestamp_ms &&
        self.latitude == other.latitude &&
        self.longitude == other.longitude &&
        self.enabled == other.enabled &&
        self.field_type == other.field_type &&
        self.owned_by_team == other.owned_by_team &&
        self.guard_pokemon_id == other.guard_pokemon_id &&
        self.guard_pokemon_cp == other.guard_pokemon_cp &&
        self.gym_points == other.gym_points &&
        self.is_in_battle == other.is_in_battle &&
        self.active_fort_modifier == other.active_fort_modifier &&
        self.lure_info == other.lure_info &&
        self.cooldown_complete_timestamp_ms == other.cooldown_complete_timestamp_ms &&
        self.sponsor == other.sponsor &&
        self.rendering_type == other.rendering_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FortData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FortModifier {
    // message fields
    item_id: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemId>,
    expiration_timestamp_ms: ::std::option::Option<i64>,
    deployer_player_codename: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FortModifier {}

impl FortModifier {
    pub fn new() -> FortModifier {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FortModifier {
        static mut instance: ::protobuf::lazy::Lazy<FortModifier> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FortModifier,
        };
        unsafe {
            instance.get(|| {
                FortModifier {
                    item_id: ::std::option::Option::None,
                    expiration_timestamp_ms: ::std::option::Option::None,
                    deployer_player_codename: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Inventory.Item.ItemId item_id = 1;

    pub fn clear_item_id(&mut self) {
        self.item_id = ::std::option::Option::None;
    }

    pub fn has_item_id(&self) -> bool {
        self.item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_id(&mut self, v: super::POGOProtos_Inventory_Item::ItemId) {
        self.item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_id(&self) -> super::POGOProtos_Inventory_Item::ItemId {
        self.item_id.unwrap_or(super::POGOProtos_Inventory_Item::ItemId::ITEM_UNKNOWN)
    }

    // optional int64 expiration_timestamp_ms = 2;

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

    // optional string deployer_player_codename = 3;

    pub fn clear_deployer_player_codename(&mut self) {
        self.deployer_player_codename.clear();
    }

    pub fn has_deployer_player_codename(&self) -> bool {
        self.deployer_player_codename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deployer_player_codename(&mut self, v: ::std::string::String) {
        self.deployer_player_codename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deployer_player_codename(&mut self) -> &mut ::std::string::String {
        if self.deployer_player_codename.is_none() {
            self.deployer_player_codename.set_default();
        };
        self.deployer_player_codename.as_mut().unwrap()
    }

    // Take field
    pub fn take_deployer_player_codename(&mut self) -> ::std::string::String {
        self.deployer_player_codename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_deployer_player_codename(&self) -> &str {
        match self.deployer_player_codename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for FortModifier {
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
                    self.item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.expiration_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.deployer_player_codename));
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
        for value in &self.item_id {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.expiration_timestamp_ms {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.deployer_player_codename {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_id {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.expiration_timestamp_ms {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.deployer_player_codename.as_ref() {
            try!(os.write_string(3, &v));
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
        ::std::any::TypeId::of::<FortModifier>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FortModifier {
    fn new() -> FortModifier {
        FortModifier::new()
    }

    fn descriptor_static(_: ::std::option::Option<FortModifier>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "item_id",
                    FortModifier::has_item_id,
                    FortModifier::get_item_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "expiration_timestamp_ms",
                    FortModifier::has_expiration_timestamp_ms,
                    FortModifier::get_expiration_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "deployer_player_codename",
                    FortModifier::has_deployer_player_codename,
                    FortModifier::get_deployer_player_codename,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FortModifier>(
                    "FortModifier",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FortModifier {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_expiration_timestamp_ms();
        self.clear_deployer_player_codename();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FortModifier {
    fn eq(&self, other: &FortModifier) -> bool {
        self.item_id == other.item_id &&
        self.expiration_timestamp_ms == other.expiration_timestamp_ms &&
        self.deployer_player_codename == other.deployer_player_codename &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FortModifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FortType {
    GYM = 0,
    CHECKPOINT = 1,
}

impl ::protobuf::ProtobufEnum for FortType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FortType> {
        match value {
            0 => ::std::option::Option::Some(FortType::GYM),
            1 => ::std::option::Option::Some(FortType::CHECKPOINT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FortType] = &[
            FortType::GYM,
            FortType::CHECKPOINT,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<FortType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("FortType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for FortType {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FortSponsor {
    UNSET_SPONSOR = 0,
    MCDONALDS = 1,
    POKEMON_STORE = 2,
}

impl ::protobuf::ProtobufEnum for FortSponsor {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FortSponsor> {
        match value {
            0 => ::std::option::Option::Some(FortSponsor::UNSET_SPONSOR),
            1 => ::std::option::Option::Some(FortSponsor::MCDONALDS),
            2 => ::std::option::Option::Some(FortSponsor::POKEMON_STORE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FortSponsor] = &[
            FortSponsor::UNSET_SPONSOR,
            FortSponsor::MCDONALDS,
            FortSponsor::POKEMON_STORE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<FortSponsor>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("FortSponsor", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for FortSponsor {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FortRenderingType {
    DEFAULT = 0,
    INTERNAL_TEST = 1,
}

impl ::protobuf::ProtobufEnum for FortRenderingType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FortRenderingType> {
        match value {
            0 => ::std::option::Option::Some(FortRenderingType::DEFAULT),
            1 => ::std::option::Option::Some(FortRenderingType::INTERNAL_TEST),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FortRenderingType] = &[
            FortRenderingType::DEFAULT,
            FortRenderingType::INTERNAL_TEST,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<FortRenderingType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("FortRenderingType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for FortRenderingType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x19, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70,
    0x2e, 0x46, 0x6f, 0x72, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x46, 0x6f, 0x72, 0x74,
    0x1a, 0x16, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75,
    0x6d, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49,
    0x74, 0x65, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x00, 0x50, 0x01, 0x22, 0xce, 0x01,
    0x0a, 0x0c, 0x46, 0x6f, 0x72, 0x74, 0x4c, 0x75, 0x72, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x17,
    0x0a, 0x07, 0x66, 0x6f, 0x72, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x06, 0x66, 0x6f, 0x72, 0x74, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x65, 0x6e, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0b, 0x65,
    0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x49, 0x64, 0x12, 0x47, 0x0a, 0x11, 0x61, 0x63,
    0x74, 0x69, 0x76, 0x65, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e,
    0x49, 0x64, 0x52, 0x0f, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f,
    0x6e, 0x49, 0x64, 0x12, 0x39, 0x0a, 0x19, 0x6c, 0x75, 0x72, 0x65, 0x5f, 0x65, 0x78, 0x70, 0x69,
    0x72, 0x65, 0x73, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x52, 0x16, 0x6c, 0x75, 0x72, 0x65, 0x45, 0x78, 0x70, 0x69,
    0x72, 0x65, 0x73, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x22, 0xac,
    0x01, 0x0a, 0x0b, 0x46, 0x6f, 0x72, 0x74, 0x53, 0x75, 0x6d, 0x6d, 0x61, 0x72, 0x79, 0x12, 0x26,
    0x0a, 0x0f, 0x66, 0x6f, 0x72, 0x74, 0x5f, 0x73, 0x75, 0x6d, 0x6d, 0x61, 0x72, 0x79, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x66, 0x6f, 0x72, 0x74, 0x53, 0x75, 0x6d,
    0x6d, 0x61, 0x72, 0x79, 0x49, 0x64, 0x12, 0x3b, 0x0a, 0x1a, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x6d,
    0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d,
    0x70, 0x5f, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x17, 0x6c, 0x61, 0x73, 0x74,
    0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d,
    0x70, 0x4d, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12,
    0x1c, 0x0a, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x01, 0x52, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x22, 0xb6, 0x06,
    0x0a, 0x08, 0x46, 0x6f, 0x72, 0x74, 0x44, 0x61, 0x74, 0x61, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x3b, 0x0a, 0x1a, 0x6c, 0x61,
    0x73, 0x74, 0x5f, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x17,
    0x6c, 0x61, 0x73, 0x74, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x54, 0x69, 0x6d, 0x65,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74,
    0x75, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74,
    0x75, 0x64, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x52, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64,
    0x65, 0x12, 0x18, 0x0a, 0x07, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x07, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x12, 0x31, 0x0a, 0x04, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x2e,
    0x46, 0x6f, 0x72, 0x74, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x3f,
    0x0a, 0x0d, 0x6f, 0x77, 0x6e, 0x65, 0x64, 0x5f, 0x62, 0x79, 0x5f, 0x74, 0x65, 0x61, 0x6d, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x54, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6c,
    0x6f, 0x72, 0x52, 0x0b, 0x6f, 0x77, 0x6e, 0x65, 0x64, 0x42, 0x79, 0x54, 0x65, 0x61, 0x6d, 0x12,
    0x45, 0x0a, 0x10, 0x67, 0x75, 0x61, 0x72, 0x64, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e,
    0x5f, 0x69, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x52, 0x0e, 0x67, 0x75, 0x61, 0x72, 0x64, 0x50, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x28, 0x0a, 0x10, 0x67, 0x75, 0x61, 0x72, 0x64, 0x5f,
    0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x63, 0x70, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x0e, 0x67, 0x75, 0x61, 0x72, 0x64, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x43, 0x70,
    0x12, 0x1d, 0x0a, 0x0a, 0x67, 0x79, 0x6d, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x73, 0x18, 0x0a,
    0x20, 0x01, 0x28, 0x03, 0x52, 0x09, 0x67, 0x79, 0x6d, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x73, 0x12,
    0x20, 0x0a, 0x0c, 0x69, 0x73, 0x5f, 0x69, 0x6e, 0x5f, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x18,
    0x0b, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x69, 0x73, 0x49, 0x6e, 0x42, 0x61, 0x74, 0x74, 0x6c,
    0x65, 0x12, 0x53, 0x0a, 0x14, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x66, 0x6f, 0x72, 0x74,
    0x5f, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x0c, 0x20, 0x03, 0x28, 0x0e, 0x32,
    0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76,
    0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d,
    0x49, 0x64, 0x52, 0x12, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x46, 0x6f, 0x72, 0x74, 0x4d, 0x6f,
    0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x3e, 0x0a, 0x09, 0x6c, 0x75, 0x72, 0x65, 0x5f, 0x69,
    0x6e, 0x66, 0x6f, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x2e,
    0x46, 0x6f, 0x72, 0x74, 0x4c, 0x75, 0x72, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x08, 0x6c, 0x75,
    0x72, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x43, 0x0a, 0x1e, 0x63, 0x6f, 0x6f, 0x6c, 0x64, 0x6f,
    0x77, 0x6e, 0x5f, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x03, 0x52, 0x1b,
    0x63, 0x6f, 0x6f, 0x6c, 0x64, 0x6f, 0x77, 0x6e, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65,
    0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12, 0x3a, 0x0a, 0x07, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x6f, 0x72, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x20, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x2e, 0x46, 0x6f,
    0x72, 0x74, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x53, 0x70, 0x6f, 0x6e, 0x73, 0x6f, 0x72, 0x52, 0x07,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x6f, 0x72, 0x12, 0x4d, 0x0a, 0x0e, 0x72, 0x65, 0x6e, 0x64, 0x65,
    0x72, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x26, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x70,
    0x2e, 0x46, 0x6f, 0x72, 0x74, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x52, 0x65, 0x6e, 0x64, 0x65, 0x72,
    0x69, 0x6e, 0x67, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0d, 0x72, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x69,
    0x6e, 0x67, 0x54, 0x79, 0x70, 0x65, 0x22, 0xbc, 0x01, 0x0a, 0x0c, 0x46, 0x6f, 0x72, 0x74, 0x4d,
    0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x3a, 0x0a, 0x07, 0x69, 0x74, 0x65, 0x6d, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e,
    0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x52, 0x06, 0x69, 0x74, 0x65,
    0x6d, 0x49, 0x64, 0x12, 0x36, 0x0a, 0x17, 0x65, 0x78, 0x70, 0x69, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x03, 0x52, 0x15, 0x65, 0x78, 0x70, 0x69, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12, 0x38, 0x0a, 0x18, 0x64,
    0x65, 0x70, 0x6c, 0x6f, 0x79, 0x65, 0x72, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x63,
    0x6f, 0x64, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x16, 0x64,
    0x65, 0x70, 0x6c, 0x6f, 0x79, 0x65, 0x72, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x6f, 0x64,
    0x65, 0x6e, 0x61, 0x6d, 0x65, 0x2a, 0x23, 0x0a, 0x08, 0x46, 0x6f, 0x72, 0x74, 0x54, 0x79, 0x70,
    0x65, 0x12, 0x07, 0x0a, 0x03, 0x47, 0x59, 0x4d, 0x10, 0x00, 0x12, 0x0e, 0x0a, 0x0a, 0x43, 0x48,
    0x45, 0x43, 0x4b, 0x50, 0x4f, 0x49, 0x4e, 0x54, 0x10, 0x01, 0x2a, 0x42, 0x0a, 0x0b, 0x46, 0x6f,
    0x72, 0x74, 0x53, 0x70, 0x6f, 0x6e, 0x73, 0x6f, 0x72, 0x12, 0x11, 0x0a, 0x0d, 0x55, 0x4e, 0x53,
    0x45, 0x54, 0x5f, 0x53, 0x50, 0x4f, 0x4e, 0x53, 0x4f, 0x52, 0x10, 0x00, 0x12, 0x0d, 0x0a, 0x09,
    0x4d, 0x43, 0x44, 0x4f, 0x4e, 0x41, 0x4c, 0x44, 0x53, 0x10, 0x01, 0x12, 0x11, 0x0a, 0x0d, 0x50,
    0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x53, 0x54, 0x4f, 0x52, 0x45, 0x10, 0x02, 0x2a, 0x33,
    0x0a, 0x11, 0x46, 0x6f, 0x72, 0x74, 0x52, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x69, 0x6e, 0x67, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x44, 0x45, 0x46, 0x41, 0x55, 0x4c, 0x54, 0x10, 0x00,
    0x12, 0x11, 0x0a, 0x0d, 0x49, 0x4e, 0x54, 0x45, 0x52, 0x4e, 0x41, 0x4c, 0x5f, 0x54, 0x45, 0x53,
    0x54, 0x10, 0x01, 0x4a, 0x94, 0x15, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x44, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01,
    0x08, 0x1b, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x00, 0x12, 0x03, 0x03, 0x07, 0x0d, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x0e, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x01, 0x12, 0x03,
    0x04, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x0e, 0x2f, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x06, 0x00, 0x0b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x06, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x07, 0x08, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x07, 0x08,
    0x06, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x07, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x0f, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x08, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x08, 0x08, 0x07, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x08, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x08, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x08,
    0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x09, 0x08, 0x3a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x09, 0x08, 0x08, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x09, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x09, 0x24, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x09, 0x38, 0x39, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x0a, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04,
    0x0a, 0x08, 0x09, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0a,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0a, 0x0e, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0a, 0x2a, 0x2b, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0c, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x0c, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x0d, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0d, 0x08,
    0x0c, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x0f, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x0e, 0x08, 0x0d, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x0e, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x0e, 0x0e, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e,
    0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x08, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0x0f, 0x08, 0x0e, 0x2d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x0f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03,
    0x12, 0x03, 0x10, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x04,
    0x10, 0x08, 0x0f, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x10,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x10, 0x0f, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x10, 0x1b, 0x1c, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x12, 0x00, 0x32, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x12, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03,
    0x13, 0x08, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x13, 0x08,
    0x12, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x13, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x0f, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x13, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x14, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x14, 0x08, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x14, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x14, 0x0e, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14,
    0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x15, 0x08, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x04, 0x15, 0x08, 0x14, 0x2d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x15, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x15, 0x0f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x15, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x16, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x04,
    0x16, 0x08, 0x15, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x16,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x16, 0x0f, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x16, 0x1b, 0x1c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x17, 0x08, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x04, 0x04, 0x12, 0x04, 0x17, 0x08, 0x16, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x17, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x17, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x17, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x18, 0x08,
    0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04, 0x12, 0x04, 0x18, 0x08, 0x17, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x06, 0x12, 0x03, 0x18, 0x08, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x18, 0x26, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x18, 0x2d, 0x2e, 0x0a, 0x46, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x06, 0x12, 0x03, 0x1d, 0x08, 0x36, 0x1a, 0x18, 0x20, 0x54, 0x65, 0x61, 0x6d, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x6f, 0x77, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x79, 0x6d,
    0x0a, 0x32, 0x1f, 0x2f, 0x2f, 0x20, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x72, 0x65, 0x6c,
    0x61, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x79, 0x6d, 0x73, 0x20, 0x6f, 0x6e, 0x6c,
    0x79, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x04, 0x12, 0x04, 0x1d, 0x08, 0x18,
    0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x06, 0x12, 0x03, 0x1d, 0x08, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1d, 0x24, 0x31, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x06, 0x03, 0x12, 0x03, 0x1d, 0x34, 0x35, 0x0a, 0x2c, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x07, 0x12, 0x03, 0x20, 0x08, 0x39, 0x1a, 0x1f, 0x20, 0x48, 0x69, 0x67, 0x68, 0x65,
    0x73, 0x74, 0x20, 0x43, 0x50, 0x20, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x20, 0x61, 0x74,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x79, 0x6d, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x07, 0x04, 0x12, 0x04, 0x20, 0x08, 0x1d, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07,
    0x06, 0x12, 0x03, 0x20, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x20, 0x24, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x20,
    0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x08, 0x12, 0x03, 0x21, 0x08, 0x23, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x04, 0x12, 0x04, 0x21, 0x08, 0x20, 0x39, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x05, 0x12, 0x03, 0x21, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x08, 0x01, 0x12, 0x03, 0x21, 0x0e, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x08, 0x03, 0x12, 0x03, 0x21, 0x21, 0x22, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x09,
    0x12, 0x03, 0x24, 0x08, 0x1e, 0x1a, 0x24, 0x20, 0x50, 0x72, 0x65, 0x73, 0x74, 0x69, 0x67, 0x61,
    0x74, 0x65, 0x20, 0x2f, 0x20, 0x65, 0x78, 0x70, 0x65, 0x72, 0x69, 0x65, 0x6e, 0x63, 0x65, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x79, 0x6d, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x09, 0x04, 0x12, 0x04, 0x24, 0x08, 0x21, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x09, 0x05, 0x12, 0x03, 0x24, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09,
    0x01, 0x12, 0x03, 0x24, 0x0e, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x03, 0x12,
    0x03, 0x24, 0x1b, 0x1d, 0x0a, 0x3f, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x0a, 0x12, 0x03, 0x27, 0x08,
    0x1f, 0x1a, 0x32, 0x20, 0x57, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x73, 0x6f, 0x6d, 0x65,
    0x6f, 0x6e, 0x65, 0x20, 0x69, 0x73, 0x20, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x69, 0x6e, 0x67, 0x20,
    0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x79, 0x6d, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65,
    0x6e, 0x74, 0x6c, 0x79, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x04, 0x12, 0x04,
    0x27, 0x08, 0x24, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x27,
    0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x27, 0x0d, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x27, 0x1c, 0x1e, 0x0a, 0x31,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x0b, 0x12, 0x03, 0x2a, 0x08, 0x4d, 0x1a, 0x24, 0x2f, 0x2f, 0x20,
    0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x72, 0x65, 0x6c, 0x61, 0x74, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x70, 0x6f, 0x6b, 0x65, 0x73, 0x74, 0x6f, 0x70, 0x73, 0x20, 0x6f, 0x6e, 0x6c, 0x79,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x2a, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x06, 0x12, 0x03, 0x2a, 0x11, 0x32, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x2a, 0x33, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x2a, 0x4a, 0x4c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x0c, 0x12, 0x03, 0x2b, 0x08, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0c, 0x04, 0x12,
    0x04, 0x2b, 0x08, 0x2a, 0x4d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0c, 0x06, 0x12, 0x03,
    0x2b, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x2b, 0x2a,
    0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x2b, 0x36, 0x38, 0x0a,
    0x53, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x0d, 0x12, 0x03, 0x2e, 0x08, 0x32, 0x1a, 0x46, 0x20, 0x54,
    0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x70, 0x6f, 0x6b, 0x65, 0x73, 0x74, 0x6f, 0x70, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62,
    0x65, 0x20, 0x61, 0x63, 0x74, 0x69, 0x76, 0x61, 0x74, 0x65, 0x64, 0x20, 0x61, 0x67, 0x61, 0x69,
    0x6e, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x65, 0x74, 0x20, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x20, 0x2f,
    0x20, 0x78, 0x70, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0d, 0x04, 0x12, 0x04, 0x2e,
    0x08, 0x2b, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x2e, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x2e, 0x0e, 0x2c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x2e, 0x2f, 0x31, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x0e, 0x12, 0x03, 0x30, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x0e, 0x04, 0x12, 0x04, 0x30, 0x08, 0x2e, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x0e, 0x06, 0x12, 0x03, 0x30, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0e, 0x01,
    0x12, 0x03, 0x30, 0x29, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0e, 0x03, 0x12, 0x03,
    0x30, 0x33, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x0f, 0x12, 0x03, 0x31, 0x08, 0x43,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0f, 0x04, 0x12, 0x04, 0x31, 0x08, 0x30, 0x36, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0f, 0x06, 0x12, 0x03, 0x31, 0x08, 0x2e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x31, 0x2f, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x31, 0x40, 0x42, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x33, 0x00, 0x37, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x33, 0x08,
    0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x34, 0x08, 0x36, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x34, 0x08, 0x33, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x34, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x34, 0x2a, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x34, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12,
    0x03, 0x35, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x04, 0x35,
    0x08, 0x34, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x35, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x35, 0x0e, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x35, 0x28, 0x29, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x36, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x36, 0x08, 0x35, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x36, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x36, 0x0f, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x36, 0x2a, 0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x38, 0x00, 0x3b, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x38, 0x05, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x39, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x39, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x39, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x08,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x3a, 0x15, 0x16, 0x0a, 0x0a, 0x0a,
    0x02, 0x05, 0x01, 0x12, 0x04, 0x3c, 0x00, 0x40, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01,
    0x12, 0x03, 0x3c, 0x05, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x3d,
    0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x3d, 0x18, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x3e, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12,
    0x03, 0x3f, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3f,
    0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x3f, 0x18, 0x19,
    0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x02, 0x12, 0x04, 0x41, 0x00, 0x44, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x02, 0x01, 0x12, 0x03, 0x41, 0x05, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x00,
    0x12, 0x03, 0x42, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x42, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x02, 0x12, 0x03, 0x42, 0x12,
    0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x43, 0x08, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x43, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x02, 0x02, 0x01, 0x02, 0x12, 0x03, 0x43, 0x18, 0x19, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x33,
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
