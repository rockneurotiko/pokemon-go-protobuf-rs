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
pub struct GpsSettings {
    // message fields
    driving_warning_speed_meters_per_second: ::std::option::Option<f32>,
    driving_warning_cooldown_minutes: ::std::option::Option<f32>,
    driving_speed_sample_interval_seconds: ::std::option::Option<f32>,
    driving_speed_sample_count: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GpsSettings {}

impl GpsSettings {
    pub fn new() -> GpsSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GpsSettings {
        static mut instance: ::protobuf::lazy::Lazy<GpsSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GpsSettings,
        };
        unsafe {
            instance.get(|| {
                GpsSettings {
                    driving_warning_speed_meters_per_second: ::std::option::Option::None,
                    driving_warning_cooldown_minutes: ::std::option::Option::None,
                    driving_speed_sample_interval_seconds: ::std::option::Option::None,
                    driving_speed_sample_count: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float driving_warning_speed_meters_per_second = 1;

    pub fn clear_driving_warning_speed_meters_per_second(&mut self) {
        self.driving_warning_speed_meters_per_second = ::std::option::Option::None;
    }

    pub fn has_driving_warning_speed_meters_per_second(&self) -> bool {
        self.driving_warning_speed_meters_per_second.is_some()
    }

    // Param is passed by value, moved
    pub fn set_driving_warning_speed_meters_per_second(&mut self, v: f32) {
        self.driving_warning_speed_meters_per_second = ::std::option::Option::Some(v);
    }

    pub fn get_driving_warning_speed_meters_per_second(&self) -> f32 {
        self.driving_warning_speed_meters_per_second.unwrap_or(0.)
    }

    // optional float driving_warning_cooldown_minutes = 2;

    pub fn clear_driving_warning_cooldown_minutes(&mut self) {
        self.driving_warning_cooldown_minutes = ::std::option::Option::None;
    }

    pub fn has_driving_warning_cooldown_minutes(&self) -> bool {
        self.driving_warning_cooldown_minutes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_driving_warning_cooldown_minutes(&mut self, v: f32) {
        self.driving_warning_cooldown_minutes = ::std::option::Option::Some(v);
    }

    pub fn get_driving_warning_cooldown_minutes(&self) -> f32 {
        self.driving_warning_cooldown_minutes.unwrap_or(0.)
    }

    // optional float driving_speed_sample_interval_seconds = 3;

    pub fn clear_driving_speed_sample_interval_seconds(&mut self) {
        self.driving_speed_sample_interval_seconds = ::std::option::Option::None;
    }

    pub fn has_driving_speed_sample_interval_seconds(&self) -> bool {
        self.driving_speed_sample_interval_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_driving_speed_sample_interval_seconds(&mut self, v: f32) {
        self.driving_speed_sample_interval_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_driving_speed_sample_interval_seconds(&self) -> f32 {
        self.driving_speed_sample_interval_seconds.unwrap_or(0.)
    }

    // optional int32 driving_speed_sample_count = 4;

    pub fn clear_driving_speed_sample_count(&mut self) {
        self.driving_speed_sample_count = ::std::option::Option::None;
    }

    pub fn has_driving_speed_sample_count(&self) -> bool {
        self.driving_speed_sample_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_driving_speed_sample_count(&mut self, v: i32) {
        self.driving_speed_sample_count = ::std::option::Option::Some(v);
    }

    pub fn get_driving_speed_sample_count(&self) -> i32 {
        self.driving_speed_sample_count.unwrap_or(0)
    }
}

impl ::protobuf::Message for GpsSettings {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.driving_warning_speed_meters_per_second = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.driving_warning_cooldown_minutes = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.driving_speed_sample_interval_seconds = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.driving_speed_sample_count = ::std::option::Option::Some(tmp);
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
        if self.driving_warning_speed_meters_per_second.is_some() {
            my_size += 5;
        };
        if self.driving_warning_cooldown_minutes.is_some() {
            my_size += 5;
        };
        if self.driving_speed_sample_interval_seconds.is_some() {
            my_size += 5;
        };
        for value in &self.driving_speed_sample_count {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.driving_warning_speed_meters_per_second {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.driving_warning_cooldown_minutes {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.driving_speed_sample_interval_seconds {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.driving_speed_sample_count {
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
        ::std::any::TypeId::of::<GpsSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GpsSettings {
    fn new() -> GpsSettings {
        GpsSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<GpsSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "driving_warning_speed_meters_per_second",
                    GpsSettings::has_driving_warning_speed_meters_per_second,
                    GpsSettings::get_driving_warning_speed_meters_per_second,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "driving_warning_cooldown_minutes",
                    GpsSettings::has_driving_warning_cooldown_minutes,
                    GpsSettings::get_driving_warning_cooldown_minutes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "driving_speed_sample_interval_seconds",
                    GpsSettings::has_driving_speed_sample_interval_seconds,
                    GpsSettings::get_driving_speed_sample_interval_seconds,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "driving_speed_sample_count",
                    GpsSettings::has_driving_speed_sample_count,
                    GpsSettings::get_driving_speed_sample_count,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GpsSettings>(
                    "GpsSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GpsSettings {
    fn clear(&mut self) {
        self.clear_driving_warning_speed_meters_per_second();
        self.clear_driving_warning_cooldown_minutes();
        self.clear_driving_speed_sample_interval_seconds();
        self.clear_driving_speed_sample_count();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GpsSettings {
    fn eq(&self, other: &GpsSettings) -> bool {
        self.driving_warning_speed_meters_per_second == other.driving_warning_speed_meters_per_second &&
        self.driving_warning_cooldown_minutes == other.driving_warning_cooldown_minutes &&
        self.driving_speed_sample_interval_seconds == other.driving_speed_sample_interval_seconds &&
        self.driving_speed_sample_count == other.driving_speed_sample_count &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GpsSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GlobalSettings {
    // message fields
    fort_settings: ::protobuf::SingularPtrField<FortSettings>,
    map_settings: ::protobuf::SingularPtrField<MapSettings>,
    level_settings: ::protobuf::SingularPtrField<LevelSettings>,
    inventory_settings: ::protobuf::SingularPtrField<InventorySettings>,
    minimum_client_version: ::protobuf::SingularField<::std::string::String>,
    gps_settings: ::protobuf::SingularPtrField<GpsSettings>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GlobalSettings {}

impl GlobalSettings {
    pub fn new() -> GlobalSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GlobalSettings {
        static mut instance: ::protobuf::lazy::Lazy<GlobalSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GlobalSettings,
        };
        unsafe {
            instance.get(|| {
                GlobalSettings {
                    fort_settings: ::protobuf::SingularPtrField::none(),
                    map_settings: ::protobuf::SingularPtrField::none(),
                    level_settings: ::protobuf::SingularPtrField::none(),
                    inventory_settings: ::protobuf::SingularPtrField::none(),
                    minimum_client_version: ::protobuf::SingularField::none(),
                    gps_settings: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Settings.FortSettings fort_settings = 2;

    pub fn clear_fort_settings(&mut self) {
        self.fort_settings.clear();
    }

    pub fn has_fort_settings(&self) -> bool {
        self.fort_settings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fort_settings(&mut self, v: FortSettings) {
        self.fort_settings = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fort_settings(&mut self) -> &mut FortSettings {
        if self.fort_settings.is_none() {
            self.fort_settings.set_default();
        };
        self.fort_settings.as_mut().unwrap()
    }

    // Take field
    pub fn take_fort_settings(&mut self) -> FortSettings {
        self.fort_settings.take().unwrap_or_else(|| FortSettings::new())
    }

    pub fn get_fort_settings(&self) -> &FortSettings {
        self.fort_settings.as_ref().unwrap_or_else(|| FortSettings::default_instance())
    }

    // optional .POGOProtos.Settings.MapSettings map_settings = 3;

    pub fn clear_map_settings(&mut self) {
        self.map_settings.clear();
    }

    pub fn has_map_settings(&self) -> bool {
        self.map_settings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_settings(&mut self, v: MapSettings) {
        self.map_settings = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_settings(&mut self) -> &mut MapSettings {
        if self.map_settings.is_none() {
            self.map_settings.set_default();
        };
        self.map_settings.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_settings(&mut self) -> MapSettings {
        self.map_settings.take().unwrap_or_else(|| MapSettings::new())
    }

    pub fn get_map_settings(&self) -> &MapSettings {
        self.map_settings.as_ref().unwrap_or_else(|| MapSettings::default_instance())
    }

    // optional .POGOProtos.Settings.LevelSettings level_settings = 4;

    pub fn clear_level_settings(&mut self) {
        self.level_settings.clear();
    }

    pub fn has_level_settings(&self) -> bool {
        self.level_settings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_level_settings(&mut self, v: LevelSettings) {
        self.level_settings = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_level_settings(&mut self) -> &mut LevelSettings {
        if self.level_settings.is_none() {
            self.level_settings.set_default();
        };
        self.level_settings.as_mut().unwrap()
    }

    // Take field
    pub fn take_level_settings(&mut self) -> LevelSettings {
        self.level_settings.take().unwrap_or_else(|| LevelSettings::new())
    }

    pub fn get_level_settings(&self) -> &LevelSettings {
        self.level_settings.as_ref().unwrap_or_else(|| LevelSettings::default_instance())
    }

    // optional .POGOProtos.Settings.InventorySettings inventory_settings = 5;

    pub fn clear_inventory_settings(&mut self) {
        self.inventory_settings.clear();
    }

    pub fn has_inventory_settings(&self) -> bool {
        self.inventory_settings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inventory_settings(&mut self, v: InventorySettings) {
        self.inventory_settings = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inventory_settings(&mut self) -> &mut InventorySettings {
        if self.inventory_settings.is_none() {
            self.inventory_settings.set_default();
        };
        self.inventory_settings.as_mut().unwrap()
    }

    // Take field
    pub fn take_inventory_settings(&mut self) -> InventorySettings {
        self.inventory_settings.take().unwrap_or_else(|| InventorySettings::new())
    }

    pub fn get_inventory_settings(&self) -> &InventorySettings {
        self.inventory_settings.as_ref().unwrap_or_else(|| InventorySettings::default_instance())
    }

    // optional string minimum_client_version = 6;

    pub fn clear_minimum_client_version(&mut self) {
        self.minimum_client_version.clear();
    }

    pub fn has_minimum_client_version(&self) -> bool {
        self.minimum_client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minimum_client_version(&mut self, v: ::std::string::String) {
        self.minimum_client_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_minimum_client_version(&mut self) -> &mut ::std::string::String {
        if self.minimum_client_version.is_none() {
            self.minimum_client_version.set_default();
        };
        self.minimum_client_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_minimum_client_version(&mut self) -> ::std::string::String {
        self.minimum_client_version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_minimum_client_version(&self) -> &str {
        match self.minimum_client_version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .POGOProtos.Settings.GpsSettings gps_settings = 7;

    pub fn clear_gps_settings(&mut self) {
        self.gps_settings.clear();
    }

    pub fn has_gps_settings(&self) -> bool {
        self.gps_settings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gps_settings(&mut self, v: GpsSettings) {
        self.gps_settings = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gps_settings(&mut self) -> &mut GpsSettings {
        if self.gps_settings.is_none() {
            self.gps_settings.set_default();
        };
        self.gps_settings.as_mut().unwrap()
    }

    // Take field
    pub fn take_gps_settings(&mut self) -> GpsSettings {
        self.gps_settings.take().unwrap_or_else(|| GpsSettings::new())
    }

    pub fn get_gps_settings(&self) -> &GpsSettings {
        self.gps_settings.as_ref().unwrap_or_else(|| GpsSettings::default_instance())
    }
}

impl ::protobuf::Message for GlobalSettings {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fort_settings));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.map_settings));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.level_settings));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.inventory_settings));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.minimum_client_version));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.gps_settings));
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
        for value in &self.fort_settings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.map_settings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.level_settings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.inventory_settings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.minimum_client_version {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        for value in &self.gps_settings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fort_settings.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.map_settings.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.level_settings.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.inventory_settings.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.minimum_client_version.as_ref() {
            try!(os.write_string(6, &v));
        };
        if let Some(v) = self.gps_settings.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<GlobalSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GlobalSettings {
    fn new() -> GlobalSettings {
        GlobalSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<GlobalSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fort_settings",
                    GlobalSettings::has_fort_settings,
                    GlobalSettings::get_fort_settings,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "map_settings",
                    GlobalSettings::has_map_settings,
                    GlobalSettings::get_map_settings,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "level_settings",
                    GlobalSettings::has_level_settings,
                    GlobalSettings::get_level_settings,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "inventory_settings",
                    GlobalSettings::has_inventory_settings,
                    GlobalSettings::get_inventory_settings,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "minimum_client_version",
                    GlobalSettings::has_minimum_client_version,
                    GlobalSettings::get_minimum_client_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "gps_settings",
                    GlobalSettings::has_gps_settings,
                    GlobalSettings::get_gps_settings,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GlobalSettings>(
                    "GlobalSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GlobalSettings {
    fn clear(&mut self) {
        self.clear_fort_settings();
        self.clear_map_settings();
        self.clear_level_settings();
        self.clear_inventory_settings();
        self.clear_minimum_client_version();
        self.clear_gps_settings();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GlobalSettings {
    fn eq(&self, other: &GlobalSettings) -> bool {
        self.fort_settings == other.fort_settings &&
        self.map_settings == other.map_settings &&
        self.level_settings == other.level_settings &&
        self.inventory_settings == other.inventory_settings &&
        self.minimum_client_version == other.minimum_client_version &&
        self.gps_settings == other.gps_settings &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GlobalSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct MapSettings {
    // message fields
    pokemon_visible_range: ::std::option::Option<f64>,
    poke_nav_range_meters: ::std::option::Option<f64>,
    encounter_range_meters: ::std::option::Option<f64>,
    get_map_objects_min_refresh_seconds: ::std::option::Option<f32>,
    get_map_objects_max_refresh_seconds: ::std::option::Option<f32>,
    get_map_objects_min_distance_meters: ::std::option::Option<f32>,
    google_maps_api_key: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MapSettings {}

impl MapSettings {
    pub fn new() -> MapSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MapSettings {
        static mut instance: ::protobuf::lazy::Lazy<MapSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MapSettings,
        };
        unsafe {
            instance.get(|| {
                MapSettings {
                    pokemon_visible_range: ::std::option::Option::None,
                    poke_nav_range_meters: ::std::option::Option::None,
                    encounter_range_meters: ::std::option::Option::None,
                    get_map_objects_min_refresh_seconds: ::std::option::Option::None,
                    get_map_objects_max_refresh_seconds: ::std::option::Option::None,
                    get_map_objects_min_distance_meters: ::std::option::Option::None,
                    google_maps_api_key: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional double pokemon_visible_range = 1;

    pub fn clear_pokemon_visible_range(&mut self) {
        self.pokemon_visible_range = ::std::option::Option::None;
    }

    pub fn has_pokemon_visible_range(&self) -> bool {
        self.pokemon_visible_range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_visible_range(&mut self, v: f64) {
        self.pokemon_visible_range = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_visible_range(&self) -> f64 {
        self.pokemon_visible_range.unwrap_or(0.)
    }

    // optional double poke_nav_range_meters = 2;

    pub fn clear_poke_nav_range_meters(&mut self) {
        self.poke_nav_range_meters = ::std::option::Option::None;
    }

    pub fn has_poke_nav_range_meters(&self) -> bool {
        self.poke_nav_range_meters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_poke_nav_range_meters(&mut self, v: f64) {
        self.poke_nav_range_meters = ::std::option::Option::Some(v);
    }

    pub fn get_poke_nav_range_meters(&self) -> f64 {
        self.poke_nav_range_meters.unwrap_or(0.)
    }

    // optional double encounter_range_meters = 3;

    pub fn clear_encounter_range_meters(&mut self) {
        self.encounter_range_meters = ::std::option::Option::None;
    }

    pub fn has_encounter_range_meters(&self) -> bool {
        self.encounter_range_meters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encounter_range_meters(&mut self, v: f64) {
        self.encounter_range_meters = ::std::option::Option::Some(v);
    }

    pub fn get_encounter_range_meters(&self) -> f64 {
        self.encounter_range_meters.unwrap_or(0.)
    }

    // optional float get_map_objects_min_refresh_seconds = 4;

    pub fn clear_get_map_objects_min_refresh_seconds(&mut self) {
        self.get_map_objects_min_refresh_seconds = ::std::option::Option::None;
    }

    pub fn has_get_map_objects_min_refresh_seconds(&self) -> bool {
        self.get_map_objects_min_refresh_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_map_objects_min_refresh_seconds(&mut self, v: f32) {
        self.get_map_objects_min_refresh_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_get_map_objects_min_refresh_seconds(&self) -> f32 {
        self.get_map_objects_min_refresh_seconds.unwrap_or(0.)
    }

    // optional float get_map_objects_max_refresh_seconds = 5;

    pub fn clear_get_map_objects_max_refresh_seconds(&mut self) {
        self.get_map_objects_max_refresh_seconds = ::std::option::Option::None;
    }

    pub fn has_get_map_objects_max_refresh_seconds(&self) -> bool {
        self.get_map_objects_max_refresh_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_map_objects_max_refresh_seconds(&mut self, v: f32) {
        self.get_map_objects_max_refresh_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_get_map_objects_max_refresh_seconds(&self) -> f32 {
        self.get_map_objects_max_refresh_seconds.unwrap_or(0.)
    }

    // optional float get_map_objects_min_distance_meters = 6;

    pub fn clear_get_map_objects_min_distance_meters(&mut self) {
        self.get_map_objects_min_distance_meters = ::std::option::Option::None;
    }

    pub fn has_get_map_objects_min_distance_meters(&self) -> bool {
        self.get_map_objects_min_distance_meters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_map_objects_min_distance_meters(&mut self, v: f32) {
        self.get_map_objects_min_distance_meters = ::std::option::Option::Some(v);
    }

    pub fn get_get_map_objects_min_distance_meters(&self) -> f32 {
        self.get_map_objects_min_distance_meters.unwrap_or(0.)
    }

    // optional string google_maps_api_key = 7;

    pub fn clear_google_maps_api_key(&mut self) {
        self.google_maps_api_key.clear();
    }

    pub fn has_google_maps_api_key(&self) -> bool {
        self.google_maps_api_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_google_maps_api_key(&mut self, v: ::std::string::String) {
        self.google_maps_api_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_google_maps_api_key(&mut self) -> &mut ::std::string::String {
        if self.google_maps_api_key.is_none() {
            self.google_maps_api_key.set_default();
        };
        self.google_maps_api_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_google_maps_api_key(&mut self) -> ::std::string::String {
        self.google_maps_api_key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_google_maps_api_key(&self) -> &str {
        match self.google_maps_api_key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for MapSettings {
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
                    let tmp = try!(is.read_double());
                    self.pokemon_visible_range = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.poke_nav_range_meters = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.encounter_range_meters = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.get_map_objects_min_refresh_seconds = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.get_map_objects_max_refresh_seconds = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.get_map_objects_min_distance_meters = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.google_maps_api_key));
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
        if self.pokemon_visible_range.is_some() {
            my_size += 9;
        };
        if self.poke_nav_range_meters.is_some() {
            my_size += 9;
        };
        if self.encounter_range_meters.is_some() {
            my_size += 9;
        };
        if self.get_map_objects_min_refresh_seconds.is_some() {
            my_size += 5;
        };
        if self.get_map_objects_max_refresh_seconds.is_some() {
            my_size += 5;
        };
        if self.get_map_objects_min_distance_meters.is_some() {
            my_size += 5;
        };
        for value in &self.google_maps_api_key {
            my_size += ::protobuf::rt::string_size(7, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pokemon_visible_range {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.poke_nav_range_meters {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.encounter_range_meters {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.get_map_objects_min_refresh_seconds {
            try!(os.write_float(4, v));
        };
        if let Some(v) = self.get_map_objects_max_refresh_seconds {
            try!(os.write_float(5, v));
        };
        if let Some(v) = self.get_map_objects_min_distance_meters {
            try!(os.write_float(6, v));
        };
        if let Some(v) = self.google_maps_api_key.as_ref() {
            try!(os.write_string(7, &v));
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
        ::std::any::TypeId::of::<MapSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MapSettings {
    fn new() -> MapSettings {
        MapSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<MapSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "pokemon_visible_range",
                    MapSettings::has_pokemon_visible_range,
                    MapSettings::get_pokemon_visible_range,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "poke_nav_range_meters",
                    MapSettings::has_poke_nav_range_meters,
                    MapSettings::get_poke_nav_range_meters,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "encounter_range_meters",
                    MapSettings::has_encounter_range_meters,
                    MapSettings::get_encounter_range_meters,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "get_map_objects_min_refresh_seconds",
                    MapSettings::has_get_map_objects_min_refresh_seconds,
                    MapSettings::get_get_map_objects_min_refresh_seconds,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "get_map_objects_max_refresh_seconds",
                    MapSettings::has_get_map_objects_max_refresh_seconds,
                    MapSettings::get_get_map_objects_max_refresh_seconds,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "get_map_objects_min_distance_meters",
                    MapSettings::has_get_map_objects_min_distance_meters,
                    MapSettings::get_get_map_objects_min_distance_meters,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "google_maps_api_key",
                    MapSettings::has_google_maps_api_key,
                    MapSettings::get_google_maps_api_key,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MapSettings>(
                    "MapSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MapSettings {
    fn clear(&mut self) {
        self.clear_pokemon_visible_range();
        self.clear_poke_nav_range_meters();
        self.clear_encounter_range_meters();
        self.clear_get_map_objects_min_refresh_seconds();
        self.clear_get_map_objects_max_refresh_seconds();
        self.clear_get_map_objects_min_distance_meters();
        self.clear_google_maps_api_key();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for MapSettings {
    fn eq(&self, other: &MapSettings) -> bool {
        self.pokemon_visible_range == other.pokemon_visible_range &&
        self.poke_nav_range_meters == other.poke_nav_range_meters &&
        self.encounter_range_meters == other.encounter_range_meters &&
        self.get_map_objects_min_refresh_seconds == other.get_map_objects_min_refresh_seconds &&
        self.get_map_objects_max_refresh_seconds == other.get_map_objects_max_refresh_seconds &&
        self.get_map_objects_min_distance_meters == other.get_map_objects_min_distance_meters &&
        self.google_maps_api_key == other.google_maps_api_key &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for MapSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct LevelSettings {
    // message fields
    trainer_cp_modifier: ::std::option::Option<f64>,
    trainer_difficulty_modifier: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LevelSettings {}

impl LevelSettings {
    pub fn new() -> LevelSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LevelSettings {
        static mut instance: ::protobuf::lazy::Lazy<LevelSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LevelSettings,
        };
        unsafe {
            instance.get(|| {
                LevelSettings {
                    trainer_cp_modifier: ::std::option::Option::None,
                    trainer_difficulty_modifier: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional double trainer_cp_modifier = 2;

    pub fn clear_trainer_cp_modifier(&mut self) {
        self.trainer_cp_modifier = ::std::option::Option::None;
    }

    pub fn has_trainer_cp_modifier(&self) -> bool {
        self.trainer_cp_modifier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trainer_cp_modifier(&mut self, v: f64) {
        self.trainer_cp_modifier = ::std::option::Option::Some(v);
    }

    pub fn get_trainer_cp_modifier(&self) -> f64 {
        self.trainer_cp_modifier.unwrap_or(0.)
    }

    // optional double trainer_difficulty_modifier = 3;

    pub fn clear_trainer_difficulty_modifier(&mut self) {
        self.trainer_difficulty_modifier = ::std::option::Option::None;
    }

    pub fn has_trainer_difficulty_modifier(&self) -> bool {
        self.trainer_difficulty_modifier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trainer_difficulty_modifier(&mut self, v: f64) {
        self.trainer_difficulty_modifier = ::std::option::Option::Some(v);
    }

    pub fn get_trainer_difficulty_modifier(&self) -> f64 {
        self.trainer_difficulty_modifier.unwrap_or(0.)
    }
}

impl ::protobuf::Message for LevelSettings {
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
                    self.trainer_cp_modifier = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.trainer_difficulty_modifier = ::std::option::Option::Some(tmp);
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
        if self.trainer_cp_modifier.is_some() {
            my_size += 9;
        };
        if self.trainer_difficulty_modifier.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.trainer_cp_modifier {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.trainer_difficulty_modifier {
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
        ::std::any::TypeId::of::<LevelSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LevelSettings {
    fn new() -> LevelSettings {
        LevelSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<LevelSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "trainer_cp_modifier",
                    LevelSettings::has_trainer_cp_modifier,
                    LevelSettings::get_trainer_cp_modifier,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "trainer_difficulty_modifier",
                    LevelSettings::has_trainer_difficulty_modifier,
                    LevelSettings::get_trainer_difficulty_modifier,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LevelSettings>(
                    "LevelSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LevelSettings {
    fn clear(&mut self) {
        self.clear_trainer_cp_modifier();
        self.clear_trainer_difficulty_modifier();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LevelSettings {
    fn eq(&self, other: &LevelSettings) -> bool {
        self.trainer_cp_modifier == other.trainer_cp_modifier &&
        self.trainer_difficulty_modifier == other.trainer_difficulty_modifier &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LevelSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InventorySettings {
    // message fields
    max_pokemon: ::std::option::Option<i32>,
    max_bag_items: ::std::option::Option<i32>,
    base_pokemon: ::std::option::Option<i32>,
    base_bag_items: ::std::option::Option<i32>,
    base_eggs: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InventorySettings {}

impl InventorySettings {
    pub fn new() -> InventorySettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InventorySettings {
        static mut instance: ::protobuf::lazy::Lazy<InventorySettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InventorySettings,
        };
        unsafe {
            instance.get(|| {
                InventorySettings {
                    max_pokemon: ::std::option::Option::None,
                    max_bag_items: ::std::option::Option::None,
                    base_pokemon: ::std::option::Option::None,
                    base_bag_items: ::std::option::Option::None,
                    base_eggs: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 max_pokemon = 1;

    pub fn clear_max_pokemon(&mut self) {
        self.max_pokemon = ::std::option::Option::None;
    }

    pub fn has_max_pokemon(&self) -> bool {
        self.max_pokemon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_pokemon(&mut self, v: i32) {
        self.max_pokemon = ::std::option::Option::Some(v);
    }

    pub fn get_max_pokemon(&self) -> i32 {
        self.max_pokemon.unwrap_or(0)
    }

    // optional int32 max_bag_items = 2;

    pub fn clear_max_bag_items(&mut self) {
        self.max_bag_items = ::std::option::Option::None;
    }

    pub fn has_max_bag_items(&self) -> bool {
        self.max_bag_items.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_bag_items(&mut self, v: i32) {
        self.max_bag_items = ::std::option::Option::Some(v);
    }

    pub fn get_max_bag_items(&self) -> i32 {
        self.max_bag_items.unwrap_or(0)
    }

    // optional int32 base_pokemon = 3;

    pub fn clear_base_pokemon(&mut self) {
        self.base_pokemon = ::std::option::Option::None;
    }

    pub fn has_base_pokemon(&self) -> bool {
        self.base_pokemon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_pokemon(&mut self, v: i32) {
        self.base_pokemon = ::std::option::Option::Some(v);
    }

    pub fn get_base_pokemon(&self) -> i32 {
        self.base_pokemon.unwrap_or(0)
    }

    // optional int32 base_bag_items = 4;

    pub fn clear_base_bag_items(&mut self) {
        self.base_bag_items = ::std::option::Option::None;
    }

    pub fn has_base_bag_items(&self) -> bool {
        self.base_bag_items.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_bag_items(&mut self, v: i32) {
        self.base_bag_items = ::std::option::Option::Some(v);
    }

    pub fn get_base_bag_items(&self) -> i32 {
        self.base_bag_items.unwrap_or(0)
    }

    // optional int32 base_eggs = 5;

    pub fn clear_base_eggs(&mut self) {
        self.base_eggs = ::std::option::Option::None;
    }

    pub fn has_base_eggs(&self) -> bool {
        self.base_eggs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_eggs(&mut self, v: i32) {
        self.base_eggs = ::std::option::Option::Some(v);
    }

    pub fn get_base_eggs(&self) -> i32 {
        self.base_eggs.unwrap_or(0)
    }
}

impl ::protobuf::Message for InventorySettings {
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
                    self.max_pokemon = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.max_bag_items = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.base_pokemon = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.base_bag_items = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.base_eggs = ::std::option::Option::Some(tmp);
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
        for value in &self.max_pokemon {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.max_bag_items {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.base_pokemon {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.base_bag_items {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.base_eggs {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.max_pokemon {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.max_bag_items {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.base_pokemon {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.base_bag_items {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.base_eggs {
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
        ::std::any::TypeId::of::<InventorySettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InventorySettings {
    fn new() -> InventorySettings {
        InventorySettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<InventorySettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "max_pokemon",
                    InventorySettings::has_max_pokemon,
                    InventorySettings::get_max_pokemon,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "max_bag_items",
                    InventorySettings::has_max_bag_items,
                    InventorySettings::get_max_bag_items,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "base_pokemon",
                    InventorySettings::has_base_pokemon,
                    InventorySettings::get_base_pokemon,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "base_bag_items",
                    InventorySettings::has_base_bag_items,
                    InventorySettings::get_base_bag_items,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "base_eggs",
                    InventorySettings::has_base_eggs,
                    InventorySettings::get_base_eggs,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InventorySettings>(
                    "InventorySettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InventorySettings {
    fn clear(&mut self) {
        self.clear_max_pokemon();
        self.clear_max_bag_items();
        self.clear_base_pokemon();
        self.clear_base_bag_items();
        self.clear_base_eggs();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InventorySettings {
    fn eq(&self, other: &InventorySettings) -> bool {
        self.max_pokemon == other.max_pokemon &&
        self.max_bag_items == other.max_bag_items &&
        self.base_pokemon == other.base_pokemon &&
        self.base_bag_items == other.base_bag_items &&
        self.base_eggs == other.base_eggs &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InventorySettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FortSettings {
    // message fields
    interaction_range_meters: ::std::option::Option<f64>,
    max_total_deployed_pokemon: ::std::option::Option<i32>,
    max_player_deployed_pokemon: ::std::option::Option<i32>,
    deploy_stamina_multiplier: ::std::option::Option<f64>,
    deploy_attack_multiplier: ::std::option::Option<f64>,
    far_interaction_range_meters: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FortSettings {}

impl FortSettings {
    pub fn new() -> FortSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FortSettings {
        static mut instance: ::protobuf::lazy::Lazy<FortSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FortSettings,
        };
        unsafe {
            instance.get(|| {
                FortSettings {
                    interaction_range_meters: ::std::option::Option::None,
                    max_total_deployed_pokemon: ::std::option::Option::None,
                    max_player_deployed_pokemon: ::std::option::Option::None,
                    deploy_stamina_multiplier: ::std::option::Option::None,
                    deploy_attack_multiplier: ::std::option::Option::None,
                    far_interaction_range_meters: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional double interaction_range_meters = 1;

    pub fn clear_interaction_range_meters(&mut self) {
        self.interaction_range_meters = ::std::option::Option::None;
    }

    pub fn has_interaction_range_meters(&self) -> bool {
        self.interaction_range_meters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_interaction_range_meters(&mut self, v: f64) {
        self.interaction_range_meters = ::std::option::Option::Some(v);
    }

    pub fn get_interaction_range_meters(&self) -> f64 {
        self.interaction_range_meters.unwrap_or(0.)
    }

    // optional int32 max_total_deployed_pokemon = 2;

    pub fn clear_max_total_deployed_pokemon(&mut self) {
        self.max_total_deployed_pokemon = ::std::option::Option::None;
    }

    pub fn has_max_total_deployed_pokemon(&self) -> bool {
        self.max_total_deployed_pokemon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_total_deployed_pokemon(&mut self, v: i32) {
        self.max_total_deployed_pokemon = ::std::option::Option::Some(v);
    }

    pub fn get_max_total_deployed_pokemon(&self) -> i32 {
        self.max_total_deployed_pokemon.unwrap_or(0)
    }

    // optional int32 max_player_deployed_pokemon = 3;

    pub fn clear_max_player_deployed_pokemon(&mut self) {
        self.max_player_deployed_pokemon = ::std::option::Option::None;
    }

    pub fn has_max_player_deployed_pokemon(&self) -> bool {
        self.max_player_deployed_pokemon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_player_deployed_pokemon(&mut self, v: i32) {
        self.max_player_deployed_pokemon = ::std::option::Option::Some(v);
    }

    pub fn get_max_player_deployed_pokemon(&self) -> i32 {
        self.max_player_deployed_pokemon.unwrap_or(0)
    }

    // optional double deploy_stamina_multiplier = 4;

    pub fn clear_deploy_stamina_multiplier(&mut self) {
        self.deploy_stamina_multiplier = ::std::option::Option::None;
    }

    pub fn has_deploy_stamina_multiplier(&self) -> bool {
        self.deploy_stamina_multiplier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deploy_stamina_multiplier(&mut self, v: f64) {
        self.deploy_stamina_multiplier = ::std::option::Option::Some(v);
    }

    pub fn get_deploy_stamina_multiplier(&self) -> f64 {
        self.deploy_stamina_multiplier.unwrap_or(0.)
    }

    // optional double deploy_attack_multiplier = 5;

    pub fn clear_deploy_attack_multiplier(&mut self) {
        self.deploy_attack_multiplier = ::std::option::Option::None;
    }

    pub fn has_deploy_attack_multiplier(&self) -> bool {
        self.deploy_attack_multiplier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deploy_attack_multiplier(&mut self, v: f64) {
        self.deploy_attack_multiplier = ::std::option::Option::Some(v);
    }

    pub fn get_deploy_attack_multiplier(&self) -> f64 {
        self.deploy_attack_multiplier.unwrap_or(0.)
    }

    // optional double far_interaction_range_meters = 6;

    pub fn clear_far_interaction_range_meters(&mut self) {
        self.far_interaction_range_meters = ::std::option::Option::None;
    }

    pub fn has_far_interaction_range_meters(&self) -> bool {
        self.far_interaction_range_meters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_far_interaction_range_meters(&mut self, v: f64) {
        self.far_interaction_range_meters = ::std::option::Option::Some(v);
    }

    pub fn get_far_interaction_range_meters(&self) -> f64 {
        self.far_interaction_range_meters.unwrap_or(0.)
    }
}

impl ::protobuf::Message for FortSettings {
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
                    let tmp = try!(is.read_double());
                    self.interaction_range_meters = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.max_total_deployed_pokemon = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.max_player_deployed_pokemon = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.deploy_stamina_multiplier = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.deploy_attack_multiplier = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.far_interaction_range_meters = ::std::option::Option::Some(tmp);
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
        if self.interaction_range_meters.is_some() {
            my_size += 9;
        };
        for value in &self.max_total_deployed_pokemon {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.max_player_deployed_pokemon {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.deploy_stamina_multiplier.is_some() {
            my_size += 9;
        };
        if self.deploy_attack_multiplier.is_some() {
            my_size += 9;
        };
        if self.far_interaction_range_meters.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.interaction_range_meters {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.max_total_deployed_pokemon {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.max_player_deployed_pokemon {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.deploy_stamina_multiplier {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.deploy_attack_multiplier {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.far_interaction_range_meters {
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
        ::std::any::TypeId::of::<FortSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FortSettings {
    fn new() -> FortSettings {
        FortSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<FortSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "interaction_range_meters",
                    FortSettings::has_interaction_range_meters,
                    FortSettings::get_interaction_range_meters,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "max_total_deployed_pokemon",
                    FortSettings::has_max_total_deployed_pokemon,
                    FortSettings::get_max_total_deployed_pokemon,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "max_player_deployed_pokemon",
                    FortSettings::has_max_player_deployed_pokemon,
                    FortSettings::get_max_player_deployed_pokemon,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "deploy_stamina_multiplier",
                    FortSettings::has_deploy_stamina_multiplier,
                    FortSettings::get_deploy_stamina_multiplier,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "deploy_attack_multiplier",
                    FortSettings::has_deploy_attack_multiplier,
                    FortSettings::get_deploy_attack_multiplier,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "far_interaction_range_meters",
                    FortSettings::has_far_interaction_range_meters,
                    FortSettings::get_far_interaction_range_meters,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FortSettings>(
                    "FortSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FortSettings {
    fn clear(&mut self) {
        self.clear_interaction_range_meters();
        self.clear_max_total_deployed_pokemon();
        self.clear_max_player_deployed_pokemon();
        self.clear_deploy_stamina_multiplier();
        self.clear_deploy_attack_multiplier();
        self.clear_far_interaction_range_meters();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FortSettings {
    fn eq(&self, other: &FortSettings) -> bool {
        self.interaction_range_meters == other.interaction_range_meters &&
        self.max_total_deployed_pokemon == other.max_total_deployed_pokemon &&
        self.max_player_deployed_pokemon == other.max_player_deployed_pokemon &&
        self.deploy_stamina_multiplier == other.deploy_stamina_multiplier &&
        self.deploy_attack_multiplier == other.deploy_attack_multiplier &&
        self.far_interaction_range_meters == other.far_interaction_range_meters &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FortSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DownloadSettingsAction {
    // message fields
    hash: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DownloadSettingsAction {}

impl DownloadSettingsAction {
    pub fn new() -> DownloadSettingsAction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DownloadSettingsAction {
        static mut instance: ::protobuf::lazy::Lazy<DownloadSettingsAction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DownloadSettingsAction,
        };
        unsafe {
            instance.get(|| {
                DownloadSettingsAction {
                    hash: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string hash = 1;

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
}

impl ::protobuf::Message for DownloadSettingsAction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hash));
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
        for value in &self.hash {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hash.as_ref() {
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
        ::std::any::TypeId::of::<DownloadSettingsAction>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DownloadSettingsAction {
    fn new() -> DownloadSettingsAction {
        DownloadSettingsAction::new()
    }

    fn descriptor_static(_: ::std::option::Option<DownloadSettingsAction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "hash",
                    DownloadSettingsAction::has_hash,
                    DownloadSettingsAction::get_hash,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DownloadSettingsAction>(
                    "DownloadSettingsAction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DownloadSettingsAction {
    fn clear(&mut self) {
        self.clear_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DownloadSettingsAction {
    fn eq(&self, other: &DownloadSettingsAction) -> bool {
        self.hash == other.hash &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DownloadSettingsAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x19, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74,
    0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73,
    0x22, 0xba, 0x02, 0x0a, 0x0b, 0x47, 0x70, 0x73, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73,
    0x12, 0x53, 0x0a, 0x27, 0x64, 0x72, 0x69, 0x76, 0x69, 0x6e, 0x67, 0x5f, 0x77, 0x61, 0x72, 0x6e,
    0x69, 0x6e, 0x67, 0x5f, 0x73, 0x70, 0x65, 0x65, 0x64, 0x5f, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73,
    0x5f, 0x70, 0x65, 0x72, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x02, 0x52, 0x22, 0x64, 0x72, 0x69, 0x76, 0x69, 0x6e, 0x67, 0x57, 0x61, 0x72, 0x6e, 0x69, 0x6e,
    0x67, 0x53, 0x70, 0x65, 0x65, 0x64, 0x4d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x50, 0x65, 0x72, 0x53,
    0x65, 0x63, 0x6f, 0x6e, 0x64, 0x12, 0x47, 0x0a, 0x20, 0x64, 0x72, 0x69, 0x76, 0x69, 0x6e, 0x67,
    0x5f, 0x77, 0x61, 0x72, 0x6e, 0x69, 0x6e, 0x67, 0x5f, 0x63, 0x6f, 0x6f, 0x6c, 0x64, 0x6f, 0x77,
    0x6e, 0x5f, 0x6d, 0x69, 0x6e, 0x75, 0x74, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52,
    0x1d, 0x64, 0x72, 0x69, 0x76, 0x69, 0x6e, 0x67, 0x57, 0x61, 0x72, 0x6e, 0x69, 0x6e, 0x67, 0x43,
    0x6f, 0x6f, 0x6c, 0x64, 0x6f, 0x77, 0x6e, 0x4d, 0x69, 0x6e, 0x75, 0x74, 0x65, 0x73, 0x12, 0x50,
    0x0a, 0x25, 0x64, 0x72, 0x69, 0x76, 0x69, 0x6e, 0x67, 0x5f, 0x73, 0x70, 0x65, 0x65, 0x64, 0x5f,
    0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x5f,
    0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x21, 0x64,
    0x72, 0x69, 0x76, 0x69, 0x6e, 0x67, 0x53, 0x70, 0x65, 0x65, 0x64, 0x53, 0x61, 0x6d, 0x70, 0x6c,
    0x65, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73,
    0x12, 0x3b, 0x0a, 0x1a, 0x64, 0x72, 0x69, 0x76, 0x69, 0x6e, 0x67, 0x5f, 0x73, 0x70, 0x65, 0x65,
    0x64, 0x5f, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x17, 0x64, 0x72, 0x69, 0x76, 0x69, 0x6e, 0x67, 0x53, 0x70, 0x65,
    0x65, 0x64, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x22, 0xba, 0x03,
    0x0a, 0x0e, 0x47, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73,
    0x12, 0x46, 0x0a, 0x0d, 0x66, 0x6f, 0x72, 0x74, 0x5f, 0x73, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67,
    0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x46, 0x6f,
    0x72, 0x74, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x52, 0x0c, 0x66, 0x6f, 0x72, 0x74,
    0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x43, 0x0a, 0x0c, 0x6d, 0x61, 0x70, 0x5f,
    0x73, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74,
    0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x70, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73,
    0x52, 0x0b, 0x6d, 0x61, 0x70, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x49, 0x0a,
    0x0e, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x5f, 0x73, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4c, 0x65, 0x76, 0x65,
    0x6c, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x52, 0x0d, 0x6c, 0x65, 0x76, 0x65, 0x6c,
    0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x55, 0x0a, 0x12, 0x69, 0x6e, 0x76, 0x65,
    0x6e, 0x74, 0x6f, 0x72, 0x79, 0x5f, 0x73, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e,
    0x74, 0x6f, 0x72, 0x79, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x52, 0x11, 0x69, 0x6e,
    0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12,
    0x34, 0x0a, 0x16, 0x6d, 0x69, 0x6e, 0x69, 0x6d, 0x75, 0x6d, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x14, 0x6d, 0x69, 0x6e, 0x69, 0x6d, 0x75, 0x6d, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x43, 0x0a, 0x0c, 0x67, 0x70, 0x73, 0x5f, 0x73, 0x65, 0x74,
    0x74, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67,
    0x73, 0x2e, 0x47, 0x70, 0x73, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x52, 0x0b, 0x67,
    0x70, 0x73, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x22, 0xc0, 0x03, 0x0a, 0x0b, 0x4d,
    0x61, 0x70, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x32, 0x0a, 0x15, 0x70, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x76, 0x69, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x5f, 0x72, 0x61,
    0x6e, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x52, 0x13, 0x70, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x56, 0x69, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x12, 0x31,
    0x0a, 0x15, 0x70, 0x6f, 0x6b, 0x65, 0x5f, 0x6e, 0x61, 0x76, 0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65,
    0x5f, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x52, 0x12, 0x70,
    0x6f, 0x6b, 0x65, 0x4e, 0x61, 0x76, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x4d, 0x65, 0x74, 0x65, 0x72,
    0x73, 0x12, 0x34, 0x0a, 0x16, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x5f, 0x72,
    0x61, 0x6e, 0x67, 0x65, 0x5f, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x01, 0x52, 0x14, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x52, 0x61, 0x6e, 0x67,
    0x65, 0x4d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x12, 0x4b, 0x0a, 0x23, 0x67, 0x65, 0x74, 0x5f, 0x6d,
    0x61, 0x70, 0x5f, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x5f, 0x6d, 0x69, 0x6e, 0x5f, 0x72,
    0x65, 0x66, 0x72, 0x65, 0x73, 0x68, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x02, 0x52, 0x1e, 0x67, 0x65, 0x74, 0x4d, 0x61, 0x70, 0x4f, 0x62, 0x6a, 0x65,
    0x63, 0x74, 0x73, 0x4d, 0x69, 0x6e, 0x52, 0x65, 0x66, 0x72, 0x65, 0x73, 0x68, 0x53, 0x65, 0x63,
    0x6f, 0x6e, 0x64, 0x73, 0x12, 0x4b, 0x0a, 0x23, 0x67, 0x65, 0x74, 0x5f, 0x6d, 0x61, 0x70, 0x5f,
    0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x5f, 0x6d, 0x61, 0x78, 0x5f, 0x72, 0x65, 0x66, 0x72,
    0x65, 0x73, 0x68, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x02, 0x52, 0x1e, 0x67, 0x65, 0x74, 0x4d, 0x61, 0x70, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73,
    0x4d, 0x61, 0x78, 0x52, 0x65, 0x66, 0x72, 0x65, 0x73, 0x68, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64,
    0x73, 0x12, 0x4b, 0x0a, 0x23, 0x67, 0x65, 0x74, 0x5f, 0x6d, 0x61, 0x70, 0x5f, 0x6f, 0x62, 0x6a,
    0x65, 0x63, 0x74, 0x73, 0x5f, 0x6d, 0x69, 0x6e, 0x5f, 0x64, 0x69, 0x73, 0x74, 0x61, 0x6e, 0x63,
    0x65, 0x5f, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x02, 0x52, 0x1e,
    0x67, 0x65, 0x74, 0x4d, 0x61, 0x70, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x4d, 0x69, 0x6e,
    0x44, 0x69, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x4d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x12, 0x2d,
    0x0a, 0x13, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x5f, 0x6d, 0x61, 0x70, 0x73, 0x5f, 0x61, 0x70,
    0x69, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x4d, 0x61, 0x70, 0x73, 0x41, 0x70, 0x69, 0x4b, 0x65, 0x79, 0x22, 0x7f, 0x0a,
    0x0d, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x2e,
    0x0a, 0x13, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x5f, 0x63, 0x70, 0x5f, 0x6d, 0x6f, 0x64,
    0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x52, 0x11, 0x74, 0x72, 0x61,
    0x69, 0x6e, 0x65, 0x72, 0x43, 0x70, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x3e,
    0x0a, 0x1b, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x5f, 0x64, 0x69, 0x66, 0x66, 0x69, 0x63,
    0x75, 0x6c, 0x74, 0x79, 0x5f, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x01, 0x52, 0x19, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x44, 0x69, 0x66, 0x66,
    0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x22, 0xbe,
    0x01, 0x0a, 0x11, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x53, 0x65, 0x74, 0x74,
    0x69, 0x6e, 0x67, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x6d, 0x61, 0x78, 0x5f, 0x70, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x6d, 0x61, 0x78, 0x50, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x12, 0x22, 0x0a, 0x0d, 0x6d, 0x61, 0x78, 0x5f, 0x62, 0x61, 0x67,
    0x5f, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x6d, 0x61,
    0x78, 0x42, 0x61, 0x67, 0x49, 0x74, 0x65, 0x6d, 0x73, 0x12, 0x21, 0x0a, 0x0c, 0x62, 0x61, 0x73,
    0x65, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x0b, 0x62, 0x61, 0x73, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x12, 0x24, 0x0a, 0x0e,
    0x62, 0x61, 0x73, 0x65, 0x5f, 0x62, 0x61, 0x67, 0x5f, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x62, 0x61, 0x73, 0x65, 0x42, 0x61, 0x67, 0x49, 0x74, 0x65,
    0x6d, 0x73, 0x12, 0x1b, 0x0a, 0x09, 0x62, 0x61, 0x73, 0x65, 0x5f, 0x65, 0x67, 0x67, 0x73, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x62, 0x61, 0x73, 0x65, 0x45, 0x67, 0x67, 0x73, 0x22,
    0xfb, 0x02, 0x0a, 0x0c, 0x46, 0x6f, 0x72, 0x74, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73,
    0x12, 0x38, 0x0a, 0x18, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f,
    0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x01, 0x52, 0x16, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52,
    0x61, 0x6e, 0x67, 0x65, 0x4d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x12, 0x3b, 0x0a, 0x1a, 0x6d, 0x61,
    0x78, 0x5f, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x5f, 0x64, 0x65, 0x70, 0x6c, 0x6f, 0x79, 0x65, 0x64,
    0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x17,
    0x6d, 0x61, 0x78, 0x54, 0x6f, 0x74, 0x61, 0x6c, 0x44, 0x65, 0x70, 0x6c, 0x6f, 0x79, 0x65, 0x64,
    0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x12, 0x3d, 0x0a, 0x1b, 0x6d, 0x61, 0x78, 0x5f, 0x70,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x64, 0x65, 0x70, 0x6c, 0x6f, 0x79, 0x65, 0x64, 0x5f, 0x70,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x18, 0x6d, 0x61,
    0x78, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x44, 0x65, 0x70, 0x6c, 0x6f, 0x79, 0x65, 0x64, 0x50,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x12, 0x3a, 0x0a, 0x19, 0x64, 0x65, 0x70, 0x6c, 0x6f, 0x79,
    0x5f, 0x73, 0x74, 0x61, 0x6d, 0x69, 0x6e, 0x61, 0x5f, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c,
    0x69, 0x65, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x52, 0x17, 0x64, 0x65, 0x70, 0x6c, 0x6f,
    0x79, 0x53, 0x74, 0x61, 0x6d, 0x69, 0x6e, 0x61, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x69,
    0x65, 0x72, 0x12, 0x38, 0x0a, 0x18, 0x64, 0x65, 0x70, 0x6c, 0x6f, 0x79, 0x5f, 0x61, 0x74, 0x74,
    0x61, 0x63, 0x6b, 0x5f, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x69, 0x65, 0x72, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x01, 0x52, 0x16, 0x64, 0x65, 0x70, 0x6c, 0x6f, 0x79, 0x41, 0x74, 0x74, 0x61,
    0x63, 0x6b, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x69, 0x65, 0x72, 0x12, 0x3f, 0x0a, 0x1c,
    0x66, 0x61, 0x72, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f,
    0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x01, 0x52, 0x19, 0x66, 0x61, 0x72, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x4d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x22, 0x2c, 0x0a,
    0x16, 0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67,
    0x73, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x68, 0x61, 0x73, 0x68, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x68, 0x61, 0x73, 0x68, 0x4a, 0xbe, 0x12, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x30, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12,
    0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x04, 0x00, 0x09, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04,
    0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x08, 0x3a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x05, 0x08, 0x04, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x05, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x0e, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x38, 0x39, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x06, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x06, 0x08, 0x05, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x0e, 0x2e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x31, 0x32, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x07, 0x08, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x07, 0x08, 0x06, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x07, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x07, 0x0e, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x07, 0x36, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x08, 0x08,
    0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x08, 0x08, 0x07, 0x38,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x08, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x08, 0x0e, 0x28, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08, 0x2b, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x0a, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0a,
    0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x08, 0x3c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0b, 0x08, 0x0a, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0b, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x2a, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x3a, 0x3b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x0c, 0x08, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x0c, 0x08, 0x0b, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0c,
    0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x29, 0x35,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x38, 0x39, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x08, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0x0d, 0x08, 0x0c, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x06, 0x12, 0x03, 0x0d, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x0d, 0x2b, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x0d, 0x3c, 0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x08,
    0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x04, 0x0e, 0x08, 0x0d, 0x3e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x06, 0x12, 0x03, 0x0e, 0x08, 0x2e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x2f, 0x41, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0e, 0x44, 0x45, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x04, 0x12, 0x03, 0x0f, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x0f, 0x08, 0x0e, 0x46, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x0f, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0f,
    0x0f, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0f, 0x28, 0x29,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x10, 0x08, 0x3a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x04, 0x10, 0x08, 0x0f, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x05, 0x06, 0x12, 0x03, 0x10, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x10, 0x29, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x10, 0x38, 0x39, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x12, 0x00,
    0x1a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x12, 0x08, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x13, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x13, 0x08, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x13, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x13, 0x0f, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x13, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x14, 0x08,
    0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x14, 0x08, 0x13, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x14, 0x08, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x0f, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x15, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x15, 0x08, 0x14, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x15, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x15,
    0x0f, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x15, 0x28, 0x29,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x16, 0x08, 0x36, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x04, 0x16, 0x08, 0x15, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x16, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x16, 0x0e, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x16, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x17, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x04, 0x17, 0x08,
    0x16, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x17, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x17, 0x0e, 0x31, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x17, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x18, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x05, 0x04, 0x12, 0x04, 0x18, 0x08, 0x17, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x18, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x18, 0x0e, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x18,
    0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x19, 0x08, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x04, 0x12, 0x04, 0x19, 0x08, 0x18, 0x36, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x05, 0x12, 0x03, 0x19, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x19, 0x0f, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x06, 0x03, 0x12, 0x03, 0x19, 0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x1b, 0x00, 0x1e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x15,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x08, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x1c, 0x08, 0x1b, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1c, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x0f, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x1c, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03,
    0x1d, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x04, 0x1d, 0x08,
    0x1c, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1d, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1d, 0x0f, 0x2a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1d, 0x2d, 0x2e, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x1f, 0x00, 0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12,
    0x03, 0x1f, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x20, 0x08,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x20, 0x08, 0x1f, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x20, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x20, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x21, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x21, 0x08, 0x20, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x21, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x21,
    0x0e, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x21, 0x1e, 0x1f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x22, 0x08, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x04, 0x22, 0x08, 0x21, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x22, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x22, 0x0e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x22, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03,
    0x23, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12, 0x04, 0x23, 0x08,
    0x22, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05, 0x12, 0x03, 0x23, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x23, 0x0e, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x23, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x24, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x24, 0x08, 0x23, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x24, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x24, 0x0e, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12, 0x03, 0x24,
    0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x26, 0x00, 0x2d, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x26, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x00, 0x12, 0x03, 0x27, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x27, 0x08, 0x26, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x27, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x27,
    0x0f, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x27, 0x2a, 0x2b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x28, 0x08, 0x2d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x04, 0x28, 0x08, 0x27, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x28, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x28, 0x0e, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x28, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03,
    0x29, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x04, 0x29, 0x08,
    0x28, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x29, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x29, 0x0e, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x29, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x2a, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x03, 0x04, 0x12, 0x04, 0x2a, 0x08, 0x29, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x2a, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x2a, 0x0f, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2a,
    0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x04, 0x12, 0x03, 0x2b, 0x08, 0x2c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2b, 0x08, 0x2a, 0x2d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x05, 0x12, 0x03, 0x2b, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2b, 0x0f, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x2b, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x05,
    0x12, 0x03, 0x2c, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x04, 0x12, 0x04,
    0x2c, 0x08, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x05, 0x12, 0x03, 0x2c,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x01, 0x12, 0x03, 0x2c, 0x0f, 0x2b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x03, 0x12, 0x03, 0x2c, 0x2e, 0x2f, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x2e, 0x00, 0x30, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06,
    0x01, 0x12, 0x03, 0x2e, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03,
    0x2f, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04, 0x2f, 0x08,
    0x2e, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2f, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x0f, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2f, 0x16, 0x17, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
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
