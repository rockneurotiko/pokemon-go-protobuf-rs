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
pub struct PlayerUpdateMessage {
    // message fields
    latitude: ::std::option::Option<f64>,
    longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayerUpdateMessage {}

impl PlayerUpdateMessage {
    pub fn new() -> PlayerUpdateMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayerUpdateMessage {
        static mut instance: ::protobuf::lazy::Lazy<PlayerUpdateMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayerUpdateMessage,
        };
        unsafe {
            instance.get(|| {
                PlayerUpdateMessage {
                    latitude: ::std::option::Option::None,
                    longitude: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional double latitude = 1;

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

    // optional double longitude = 2;

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

impl ::protobuf::Message for PlayerUpdateMessage {
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
                    self.latitude = ::std::option::Option::Some(tmp);
                },
                2 => {
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
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.longitude {
            try!(os.write_double(2, v));
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
        ::std::any::TypeId::of::<PlayerUpdateMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PlayerUpdateMessage {
    fn new() -> PlayerUpdateMessage {
        PlayerUpdateMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayerUpdateMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "latitude",
                    PlayerUpdateMessage::has_latitude,
                    PlayerUpdateMessage::get_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "longitude",
                    PlayerUpdateMessage::has_longitude,
                    PlayerUpdateMessage::get_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlayerUpdateMessage>(
                    "PlayerUpdateMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayerUpdateMessage {
    fn clear(&mut self) {
        self.clear_latitude();
        self.clear_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PlayerUpdateMessage {
    fn eq(&self, other: &PlayerUpdateMessage) -> bool {
        self.latitude == other.latitude &&
        self.longitude == other.longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PlayerUpdateMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DownloadSettingsMessage {
    // message fields
    hash: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DownloadSettingsMessage {}

impl DownloadSettingsMessage {
    pub fn new() -> DownloadSettingsMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DownloadSettingsMessage {
        static mut instance: ::protobuf::lazy::Lazy<DownloadSettingsMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DownloadSettingsMessage,
        };
        unsafe {
            instance.get(|| {
                DownloadSettingsMessage {
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

impl ::protobuf::Message for DownloadSettingsMessage {
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
        ::std::any::TypeId::of::<DownloadSettingsMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DownloadSettingsMessage {
    fn new() -> DownloadSettingsMessage {
        DownloadSettingsMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<DownloadSettingsMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "hash",
                    DownloadSettingsMessage::has_hash,
                    DownloadSettingsMessage::get_hash,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DownloadSettingsMessage>(
                    "DownloadSettingsMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DownloadSettingsMessage {
    fn clear(&mut self) {
        self.clear_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DownloadSettingsMessage {
    fn eq(&self, other: &DownloadSettingsMessage) -> bool {
        self.hash == other.hash &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DownloadSettingsMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DownloadRemoteConfigVersionMessage {
    // message fields
    platform: ::std::option::Option<super::POGOProtos_Enums::Platform>,
    device_manufacturer: ::protobuf::SingularField<::std::string::String>,
    device_model: ::protobuf::SingularField<::std::string::String>,
    locale: ::protobuf::SingularField<::std::string::String>,
    app_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DownloadRemoteConfigVersionMessage {}

impl DownloadRemoteConfigVersionMessage {
    pub fn new() -> DownloadRemoteConfigVersionMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DownloadRemoteConfigVersionMessage {
        static mut instance: ::protobuf::lazy::Lazy<DownloadRemoteConfigVersionMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DownloadRemoteConfigVersionMessage,
        };
        unsafe {
            instance.get(|| {
                DownloadRemoteConfigVersionMessage {
                    platform: ::std::option::Option::None,
                    device_manufacturer: ::protobuf::SingularField::none(),
                    device_model: ::protobuf::SingularField::none(),
                    locale: ::protobuf::SingularField::none(),
                    app_version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Enums.Platform platform = 1;

    pub fn clear_platform(&mut self) {
        self.platform = ::std::option::Option::None;
    }

    pub fn has_platform(&self) -> bool {
        self.platform.is_some()
    }

    // Param is passed by value, moved
    pub fn set_platform(&mut self, v: super::POGOProtos_Enums::Platform) {
        self.platform = ::std::option::Option::Some(v);
    }

    pub fn get_platform(&self) -> super::POGOProtos_Enums::Platform {
        self.platform.unwrap_or(super::POGOProtos_Enums::Platform::UNSET)
    }

    // optional string device_manufacturer = 2;

    pub fn clear_device_manufacturer(&mut self) {
        self.device_manufacturer.clear();
    }

    pub fn has_device_manufacturer(&self) -> bool {
        self.device_manufacturer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_manufacturer(&mut self, v: ::std::string::String) {
        self.device_manufacturer = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_manufacturer(&mut self) -> &mut ::std::string::String {
        if self.device_manufacturer.is_none() {
            self.device_manufacturer.set_default();
        };
        self.device_manufacturer.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_manufacturer(&mut self) -> ::std::string::String {
        self.device_manufacturer.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_device_manufacturer(&self) -> &str {
        match self.device_manufacturer.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string device_model = 3;

    pub fn clear_device_model(&mut self) {
        self.device_model.clear();
    }

    pub fn has_device_model(&self) -> bool {
        self.device_model.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_model(&mut self, v: ::std::string::String) {
        self.device_model = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_model(&mut self) -> &mut ::std::string::String {
        if self.device_model.is_none() {
            self.device_model.set_default();
        };
        self.device_model.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_model(&mut self) -> ::std::string::String {
        self.device_model.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_device_model(&self) -> &str {
        match self.device_model.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string locale = 4;

    pub fn clear_locale(&mut self) {
        self.locale.clear();
    }

    pub fn has_locale(&self) -> bool {
        self.locale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_locale(&mut self, v: ::std::string::String) {
        self.locale = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_locale(&mut self) -> &mut ::std::string::String {
        if self.locale.is_none() {
            self.locale.set_default();
        };
        self.locale.as_mut().unwrap()
    }

    // Take field
    pub fn take_locale(&mut self) -> ::std::string::String {
        self.locale.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_locale(&self) -> &str {
        match self.locale.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint32 app_version = 5;

    pub fn clear_app_version(&mut self) {
        self.app_version = ::std::option::Option::None;
    }

    pub fn has_app_version(&self) -> bool {
        self.app_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_version(&mut self, v: u32) {
        self.app_version = ::std::option::Option::Some(v);
    }

    pub fn get_app_version(&self) -> u32 {
        self.app_version.unwrap_or(0)
    }
}

impl ::protobuf::Message for DownloadRemoteConfigVersionMessage {
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
                    self.platform = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.device_manufacturer));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.device_model));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.locale));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.app_version = ::std::option::Option::Some(tmp);
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
        for value in &self.platform {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.device_manufacturer {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.device_model {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.locale {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.app_version {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.platform {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.device_manufacturer.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.device_model.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.locale.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.app_version {
            try!(os.write_uint32(5, v));
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
        ::std::any::TypeId::of::<DownloadRemoteConfigVersionMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DownloadRemoteConfigVersionMessage {
    fn new() -> DownloadRemoteConfigVersionMessage {
        DownloadRemoteConfigVersionMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<DownloadRemoteConfigVersionMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "platform",
                    DownloadRemoteConfigVersionMessage::has_platform,
                    DownloadRemoteConfigVersionMessage::get_platform,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "device_manufacturer",
                    DownloadRemoteConfigVersionMessage::has_device_manufacturer,
                    DownloadRemoteConfigVersionMessage::get_device_manufacturer,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "device_model",
                    DownloadRemoteConfigVersionMessage::has_device_model,
                    DownloadRemoteConfigVersionMessage::get_device_model,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "locale",
                    DownloadRemoteConfigVersionMessage::has_locale,
                    DownloadRemoteConfigVersionMessage::get_locale,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "app_version",
                    DownloadRemoteConfigVersionMessage::has_app_version,
                    DownloadRemoteConfigVersionMessage::get_app_version,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DownloadRemoteConfigVersionMessage>(
                    "DownloadRemoteConfigVersionMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DownloadRemoteConfigVersionMessage {
    fn clear(&mut self) {
        self.clear_platform();
        self.clear_device_manufacturer();
        self.clear_device_model();
        self.clear_locale();
        self.clear_app_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DownloadRemoteConfigVersionMessage {
    fn eq(&self, other: &DownloadRemoteConfigVersionMessage) -> bool {
        self.platform == other.platform &&
        self.device_manufacturer == other.device_manufacturer &&
        self.device_model == other.device_model &&
        self.locale == other.locale &&
        self.app_version == other.app_version &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DownloadRemoteConfigVersionMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UseItemPotionMessage {
    // message fields
    item_id: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemId>,
    pokemon_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UseItemPotionMessage {}

impl UseItemPotionMessage {
    pub fn new() -> UseItemPotionMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UseItemPotionMessage {
        static mut instance: ::protobuf::lazy::Lazy<UseItemPotionMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UseItemPotionMessage,
        };
        unsafe {
            instance.get(|| {
                UseItemPotionMessage {
                    item_id: ::std::option::Option::None,
                    pokemon_id: ::std::option::Option::None,
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

    // optional fixed64 pokemon_id = 2;

    pub fn clear_pokemon_id(&mut self) {
        self.pokemon_id = ::std::option::Option::None;
    }

    pub fn has_pokemon_id(&self) -> bool {
        self.pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_id(&mut self, v: u64) {
        self.pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_id(&self) -> u64 {
        self.pokemon_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for UseItemPotionMessage {
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.pokemon_id = ::std::option::Option::Some(tmp);
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
        if self.pokemon_id.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_id {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.pokemon_id {
            try!(os.write_fixed64(2, v));
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
        ::std::any::TypeId::of::<UseItemPotionMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UseItemPotionMessage {
    fn new() -> UseItemPotionMessage {
        UseItemPotionMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<UseItemPotionMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "item_id",
                    UseItemPotionMessage::has_item_id,
                    UseItemPotionMessage::get_item_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "pokemon_id",
                    UseItemPotionMessage::has_pokemon_id,
                    UseItemPotionMessage::get_pokemon_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UseItemPotionMessage>(
                    "UseItemPotionMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UseItemPotionMessage {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_pokemon_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UseItemPotionMessage {
    fn eq(&self, other: &UseItemPotionMessage) -> bool {
        self.item_id == other.item_id &&
        self.pokemon_id == other.pokemon_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UseItemPotionMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetContactSettingsMessage {
    // message fields
    contact_settings: ::protobuf::SingularPtrField<super::POGOProtos_Data_Player::ContactSettings>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetContactSettingsMessage {}

impl SetContactSettingsMessage {
    pub fn new() -> SetContactSettingsMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetContactSettingsMessage {
        static mut instance: ::protobuf::lazy::Lazy<SetContactSettingsMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetContactSettingsMessage,
        };
        unsafe {
            instance.get(|| {
                SetContactSettingsMessage {
                    contact_settings: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Data.Player.ContactSettings contact_settings = 1;

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
}

impl ::protobuf::Message for SetContactSettingsMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.contact_settings));
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
        for value in &self.contact_settings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.contact_settings.as_ref() {
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
        ::std::any::TypeId::of::<SetContactSettingsMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetContactSettingsMessage {
    fn new() -> SetContactSettingsMessage {
        SetContactSettingsMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetContactSettingsMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "contact_settings",
                    SetContactSettingsMessage::has_contact_settings,
                    SetContactSettingsMessage::get_contact_settings,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetContactSettingsMessage>(
                    "SetContactSettingsMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetContactSettingsMessage {
    fn clear(&mut self) {
        self.clear_contact_settings();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetContactSettingsMessage {
    fn eq(&self, other: &SetContactSettingsMessage) -> bool {
        self.contact_settings == other.contact_settings &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetContactSettingsMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct LevelUpRewardsMessage {
    // message fields
    level: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LevelUpRewardsMessage {}

impl LevelUpRewardsMessage {
    pub fn new() -> LevelUpRewardsMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LevelUpRewardsMessage {
        static mut instance: ::protobuf::lazy::Lazy<LevelUpRewardsMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LevelUpRewardsMessage,
        };
        unsafe {
            instance.get(|| {
                LevelUpRewardsMessage {
                    level: ::std::option::Option::None,
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
}

impl ::protobuf::Message for LevelUpRewardsMessage {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.level {
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
        ::std::any::TypeId::of::<LevelUpRewardsMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LevelUpRewardsMessage {
    fn new() -> LevelUpRewardsMessage {
        LevelUpRewardsMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<LevelUpRewardsMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "level",
                    LevelUpRewardsMessage::has_level,
                    LevelUpRewardsMessage::get_level,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LevelUpRewardsMessage>(
                    "LevelUpRewardsMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LevelUpRewardsMessage {
    fn clear(&mut self) {
        self.clear_level();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LevelUpRewardsMessage {
    fn eq(&self, other: &LevelUpRewardsMessage) -> bool {
        self.level == other.level &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LevelUpRewardsMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FortDetailsMessage {
    // message fields
    fort_id: ::protobuf::SingularField<::std::string::String>,
    latitude: ::std::option::Option<f64>,
    longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FortDetailsMessage {}

impl FortDetailsMessage {
    pub fn new() -> FortDetailsMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FortDetailsMessage {
        static mut instance: ::protobuf::lazy::Lazy<FortDetailsMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FortDetailsMessage,
        };
        unsafe {
            instance.get(|| {
                FortDetailsMessage {
                    fort_id: ::protobuf::SingularField::none(),
                    latitude: ::std::option::Option::None,
                    longitude: ::std::option::Option::None,
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

impl ::protobuf::Message for FortDetailsMessage {
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
        for value in &self.fort_id {
            my_size += ::protobuf::rt::string_size(1, &value);
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
        if let Some(v) = self.fort_id.as_ref() {
            try!(os.write_string(1, &v));
        };
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
        ::std::any::TypeId::of::<FortDetailsMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FortDetailsMessage {
    fn new() -> FortDetailsMessage {
        FortDetailsMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<FortDetailsMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "fort_id",
                    FortDetailsMessage::has_fort_id,
                    FortDetailsMessage::get_fort_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "latitude",
                    FortDetailsMessage::has_latitude,
                    FortDetailsMessage::get_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "longitude",
                    FortDetailsMessage::has_longitude,
                    FortDetailsMessage::get_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FortDetailsMessage>(
                    "FortDetailsMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FortDetailsMessage {
    fn clear(&mut self) {
        self.clear_fort_id();
        self.clear_latitude();
        self.clear_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FortDetailsMessage {
    fn eq(&self, other: &FortDetailsMessage) -> bool {
        self.fort_id == other.fort_id &&
        self.latitude == other.latitude &&
        self.longitude == other.longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FortDetailsMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetPlayerMessage {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetPlayerMessage {}

impl GetPlayerMessage {
    pub fn new() -> GetPlayerMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetPlayerMessage {
        static mut instance: ::protobuf::lazy::Lazy<GetPlayerMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetPlayerMessage,
        };
        unsafe {
            instance.get(|| {
                GetPlayerMessage {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for GetPlayerMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<GetPlayerMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetPlayerMessage {
    fn new() -> GetPlayerMessage {
        GetPlayerMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetPlayerMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetPlayerMessage>(
                    "GetPlayerMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetPlayerMessage {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetPlayerMessage {
    fn eq(&self, other: &GetPlayerMessage) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetPlayerMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetGymDetailsMessage {
    // message fields
    gym_id: ::protobuf::SingularField<::std::string::String>,
    player_latitude: ::std::option::Option<f64>,
    player_longitude: ::std::option::Option<f64>,
    gym_latitude: ::std::option::Option<f64>,
    gym_longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetGymDetailsMessage {}

impl GetGymDetailsMessage {
    pub fn new() -> GetGymDetailsMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetGymDetailsMessage {
        static mut instance: ::protobuf::lazy::Lazy<GetGymDetailsMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetGymDetailsMessage,
        };
        unsafe {
            instance.get(|| {
                GetGymDetailsMessage {
                    gym_id: ::protobuf::SingularField::none(),
                    player_latitude: ::std::option::Option::None,
                    player_longitude: ::std::option::Option::None,
                    gym_latitude: ::std::option::Option::None,
                    gym_longitude: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string gym_id = 1;

    pub fn clear_gym_id(&mut self) {
        self.gym_id.clear();
    }

    pub fn has_gym_id(&self) -> bool {
        self.gym_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gym_id(&mut self, v: ::std::string::String) {
        self.gym_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gym_id(&mut self) -> &mut ::std::string::String {
        if self.gym_id.is_none() {
            self.gym_id.set_default();
        };
        self.gym_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_gym_id(&mut self) -> ::std::string::String {
        self.gym_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_gym_id(&self) -> &str {
        match self.gym_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional double player_latitude = 2;

    pub fn clear_player_latitude(&mut self) {
        self.player_latitude = ::std::option::Option::None;
    }

    pub fn has_player_latitude(&self) -> bool {
        self.player_latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_latitude(&mut self, v: f64) {
        self.player_latitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_latitude(&self) -> f64 {
        self.player_latitude.unwrap_or(0.)
    }

    // optional double player_longitude = 3;

    pub fn clear_player_longitude(&mut self) {
        self.player_longitude = ::std::option::Option::None;
    }

    pub fn has_player_longitude(&self) -> bool {
        self.player_longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_longitude(&mut self, v: f64) {
        self.player_longitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_longitude(&self) -> f64 {
        self.player_longitude.unwrap_or(0.)
    }

    // optional double gym_latitude = 4;

    pub fn clear_gym_latitude(&mut self) {
        self.gym_latitude = ::std::option::Option::None;
    }

    pub fn has_gym_latitude(&self) -> bool {
        self.gym_latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gym_latitude(&mut self, v: f64) {
        self.gym_latitude = ::std::option::Option::Some(v);
    }

    pub fn get_gym_latitude(&self) -> f64 {
        self.gym_latitude.unwrap_or(0.)
    }

    // optional double gym_longitude = 5;

    pub fn clear_gym_longitude(&mut self) {
        self.gym_longitude = ::std::option::Option::None;
    }

    pub fn has_gym_longitude(&self) -> bool {
        self.gym_longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gym_longitude(&mut self, v: f64) {
        self.gym_longitude = ::std::option::Option::Some(v);
    }

    pub fn get_gym_longitude(&self) -> f64 {
        self.gym_longitude.unwrap_or(0.)
    }
}

impl ::protobuf::Message for GetGymDetailsMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.gym_id));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_latitude = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_longitude = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.gym_latitude = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.gym_longitude = ::std::option::Option::Some(tmp);
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
        for value in &self.gym_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.player_latitude.is_some() {
            my_size += 9;
        };
        if self.player_longitude.is_some() {
            my_size += 9;
        };
        if self.gym_latitude.is_some() {
            my_size += 9;
        };
        if self.gym_longitude.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gym_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.player_latitude {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.player_longitude {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.gym_latitude {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.gym_longitude {
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
        ::std::any::TypeId::of::<GetGymDetailsMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetGymDetailsMessage {
    fn new() -> GetGymDetailsMessage {
        GetGymDetailsMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetGymDetailsMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "gym_id",
                    GetGymDetailsMessage::has_gym_id,
                    GetGymDetailsMessage::get_gym_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_latitude",
                    GetGymDetailsMessage::has_player_latitude,
                    GetGymDetailsMessage::get_player_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_longitude",
                    GetGymDetailsMessage::has_player_longitude,
                    GetGymDetailsMessage::get_player_longitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "gym_latitude",
                    GetGymDetailsMessage::has_gym_latitude,
                    GetGymDetailsMessage::get_gym_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "gym_longitude",
                    GetGymDetailsMessage::has_gym_longitude,
                    GetGymDetailsMessage::get_gym_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetGymDetailsMessage>(
                    "GetGymDetailsMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetGymDetailsMessage {
    fn clear(&mut self) {
        self.clear_gym_id();
        self.clear_player_latitude();
        self.clear_player_longitude();
        self.clear_gym_latitude();
        self.clear_gym_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetGymDetailsMessage {
    fn eq(&self, other: &GetGymDetailsMessage) -> bool {
        self.gym_id == other.gym_id &&
        self.player_latitude == other.player_latitude &&
        self.player_longitude == other.player_longitude &&
        self.gym_latitude == other.gym_latitude &&
        self.gym_longitude == other.gym_longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetGymDetailsMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SfidaActionLogMessage {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SfidaActionLogMessage {}

impl SfidaActionLogMessage {
    pub fn new() -> SfidaActionLogMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SfidaActionLogMessage {
        static mut instance: ::protobuf::lazy::Lazy<SfidaActionLogMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SfidaActionLogMessage,
        };
        unsafe {
            instance.get(|| {
                SfidaActionLogMessage {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for SfidaActionLogMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<SfidaActionLogMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SfidaActionLogMessage {
    fn new() -> SfidaActionLogMessage {
        SfidaActionLogMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<SfidaActionLogMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<SfidaActionLogMessage>(
                    "SfidaActionLogMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SfidaActionLogMessage {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SfidaActionLogMessage {
    fn eq(&self, other: &SfidaActionLogMessage) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SfidaActionLogMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetDownloadUrlsMessage {
    // message fields
    asset_id: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetDownloadUrlsMessage {}

impl GetDownloadUrlsMessage {
    pub fn new() -> GetDownloadUrlsMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetDownloadUrlsMessage {
        static mut instance: ::protobuf::lazy::Lazy<GetDownloadUrlsMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetDownloadUrlsMessage,
        };
        unsafe {
            instance.get(|| {
                GetDownloadUrlsMessage {
                    asset_id: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string asset_id = 1;

    pub fn clear_asset_id(&mut self) {
        self.asset_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_asset_id(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.asset_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_asset_id(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.asset_id
    }

    // Take field
    pub fn take_asset_id(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.asset_id, ::protobuf::RepeatedField::new())
    }

    pub fn get_asset_id(&self) -> &[::std::string::String] {
        &self.asset_id
    }
}

impl ::protobuf::Message for GetDownloadUrlsMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.asset_id));
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.asset_id {
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
        ::std::any::TypeId::of::<GetDownloadUrlsMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetDownloadUrlsMessage {
    fn new() -> GetDownloadUrlsMessage {
        GetDownloadUrlsMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetDownloadUrlsMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "asset_id",
                    GetDownloadUrlsMessage::get_asset_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetDownloadUrlsMessage>(
                    "GetDownloadUrlsMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetDownloadUrlsMessage {
    fn clear(&mut self) {
        self.clear_asset_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetDownloadUrlsMessage {
    fn eq(&self, other: &GetDownloadUrlsMessage) -> bool {
        self.asset_id == other.asset_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetDownloadUrlsMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AttackGymMessage {
    // message fields
    gym_id: ::protobuf::SingularField<::std::string::String>,
    battle_id: ::protobuf::SingularField<::std::string::String>,
    attack_actions: ::protobuf::RepeatedField<super::POGOProtos_Data_Battle::BattleAction>,
    last_retrieved_actions: ::protobuf::SingularPtrField<super::POGOProtos_Data_Battle::BattleAction>,
    player_latitude: ::std::option::Option<f64>,
    player_longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AttackGymMessage {}

impl AttackGymMessage {
    pub fn new() -> AttackGymMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AttackGymMessage {
        static mut instance: ::protobuf::lazy::Lazy<AttackGymMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AttackGymMessage,
        };
        unsafe {
            instance.get(|| {
                AttackGymMessage {
                    gym_id: ::protobuf::SingularField::none(),
                    battle_id: ::protobuf::SingularField::none(),
                    attack_actions: ::protobuf::RepeatedField::new(),
                    last_retrieved_actions: ::protobuf::SingularPtrField::none(),
                    player_latitude: ::std::option::Option::None,
                    player_longitude: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string gym_id = 1;

    pub fn clear_gym_id(&mut self) {
        self.gym_id.clear();
    }

    pub fn has_gym_id(&self) -> bool {
        self.gym_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gym_id(&mut self, v: ::std::string::String) {
        self.gym_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gym_id(&mut self) -> &mut ::std::string::String {
        if self.gym_id.is_none() {
            self.gym_id.set_default();
        };
        self.gym_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_gym_id(&mut self) -> ::std::string::String {
        self.gym_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_gym_id(&self) -> &str {
        match self.gym_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string battle_id = 2;

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

    // repeated .POGOProtos.Data.Battle.BattleAction attack_actions = 3;

    pub fn clear_attack_actions(&mut self) {
        self.attack_actions.clear();
    }

    // Param is passed by value, moved
    pub fn set_attack_actions(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Data_Battle::BattleAction>) {
        self.attack_actions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attack_actions(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Data_Battle::BattleAction> {
        &mut self.attack_actions
    }

    // Take field
    pub fn take_attack_actions(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Data_Battle::BattleAction> {
        ::std::mem::replace(&mut self.attack_actions, ::protobuf::RepeatedField::new())
    }

    pub fn get_attack_actions(&self) -> &[super::POGOProtos_Data_Battle::BattleAction] {
        &self.attack_actions
    }

    // optional .POGOProtos.Data.Battle.BattleAction last_retrieved_actions = 4;

    pub fn clear_last_retrieved_actions(&mut self) {
        self.last_retrieved_actions.clear();
    }

    pub fn has_last_retrieved_actions(&self) -> bool {
        self.last_retrieved_actions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_retrieved_actions(&mut self, v: super::POGOProtos_Data_Battle::BattleAction) {
        self.last_retrieved_actions = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_retrieved_actions(&mut self) -> &mut super::POGOProtos_Data_Battle::BattleAction {
        if self.last_retrieved_actions.is_none() {
            self.last_retrieved_actions.set_default();
        };
        self.last_retrieved_actions.as_mut().unwrap()
    }

    // Take field
    pub fn take_last_retrieved_actions(&mut self) -> super::POGOProtos_Data_Battle::BattleAction {
        self.last_retrieved_actions.take().unwrap_or_else(|| super::POGOProtos_Data_Battle::BattleAction::new())
    }

    pub fn get_last_retrieved_actions(&self) -> &super::POGOProtos_Data_Battle::BattleAction {
        self.last_retrieved_actions.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Battle::BattleAction::default_instance())
    }

    // optional double player_latitude = 5;

    pub fn clear_player_latitude(&mut self) {
        self.player_latitude = ::std::option::Option::None;
    }

    pub fn has_player_latitude(&self) -> bool {
        self.player_latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_latitude(&mut self, v: f64) {
        self.player_latitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_latitude(&self) -> f64 {
        self.player_latitude.unwrap_or(0.)
    }

    // optional double player_longitude = 6;

    pub fn clear_player_longitude(&mut self) {
        self.player_longitude = ::std::option::Option::None;
    }

    pub fn has_player_longitude(&self) -> bool {
        self.player_longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_longitude(&mut self, v: f64) {
        self.player_longitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_longitude(&self) -> f64 {
        self.player_longitude.unwrap_or(0.)
    }
}

impl ::protobuf::Message for AttackGymMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.gym_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.battle_id));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.attack_actions));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.last_retrieved_actions));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_latitude = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_longitude = ::std::option::Option::Some(tmp);
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
        for value in &self.gym_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.battle_id {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.attack_actions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.last_retrieved_actions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.player_latitude.is_some() {
            my_size += 9;
        };
        if self.player_longitude.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gym_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.battle_id.as_ref() {
            try!(os.write_string(2, &v));
        };
        for v in &self.attack_actions {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.last_retrieved_actions.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.player_latitude {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.player_longitude {
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
        ::std::any::TypeId::of::<AttackGymMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AttackGymMessage {
    fn new() -> AttackGymMessage {
        AttackGymMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<AttackGymMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "gym_id",
                    AttackGymMessage::has_gym_id,
                    AttackGymMessage::get_gym_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "battle_id",
                    AttackGymMessage::has_battle_id,
                    AttackGymMessage::get_battle_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "attack_actions",
                    AttackGymMessage::get_attack_actions,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "last_retrieved_actions",
                    AttackGymMessage::has_last_retrieved_actions,
                    AttackGymMessage::get_last_retrieved_actions,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_latitude",
                    AttackGymMessage::has_player_latitude,
                    AttackGymMessage::get_player_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_longitude",
                    AttackGymMessage::has_player_longitude,
                    AttackGymMessage::get_player_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AttackGymMessage>(
                    "AttackGymMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AttackGymMessage {
    fn clear(&mut self) {
        self.clear_gym_id();
        self.clear_battle_id();
        self.clear_attack_actions();
        self.clear_last_retrieved_actions();
        self.clear_player_latitude();
        self.clear_player_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AttackGymMessage {
    fn eq(&self, other: &AttackGymMessage) -> bool {
        self.gym_id == other.gym_id &&
        self.battle_id == other.battle_id &&
        self.attack_actions == other.attack_actions &&
        self.last_retrieved_actions == other.last_retrieved_actions &&
        self.player_latitude == other.player_latitude &&
        self.player_longitude == other.player_longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AttackGymMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UseItemCaptureMessage {
    // message fields
    item_id: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemId>,
    encounter_id: ::std::option::Option<u64>,
    spawn_point_id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UseItemCaptureMessage {}

impl UseItemCaptureMessage {
    pub fn new() -> UseItemCaptureMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UseItemCaptureMessage {
        static mut instance: ::protobuf::lazy::Lazy<UseItemCaptureMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UseItemCaptureMessage,
        };
        unsafe {
            instance.get(|| {
                UseItemCaptureMessage {
                    item_id: ::std::option::Option::None,
                    encounter_id: ::std::option::Option::None,
                    spawn_point_id: ::protobuf::SingularField::none(),
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

    // optional string spawn_point_id = 3;

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
}

impl ::protobuf::Message for UseItemCaptureMessage {
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.encounter_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.spawn_point_id));
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
        if self.encounter_id.is_some() {
            my_size += 9;
        };
        for value in &self.spawn_point_id {
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
        if let Some(v) = self.encounter_id {
            try!(os.write_fixed64(2, v));
        };
        if let Some(v) = self.spawn_point_id.as_ref() {
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
        ::std::any::TypeId::of::<UseItemCaptureMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UseItemCaptureMessage {
    fn new() -> UseItemCaptureMessage {
        UseItemCaptureMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<UseItemCaptureMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "item_id",
                    UseItemCaptureMessage::has_item_id,
                    UseItemCaptureMessage::get_item_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "encounter_id",
                    UseItemCaptureMessage::has_encounter_id,
                    UseItemCaptureMessage::get_encounter_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "spawn_point_id",
                    UseItemCaptureMessage::has_spawn_point_id,
                    UseItemCaptureMessage::get_spawn_point_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UseItemCaptureMessage>(
                    "UseItemCaptureMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UseItemCaptureMessage {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_encounter_id();
        self.clear_spawn_point_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UseItemCaptureMessage {
    fn eq(&self, other: &UseItemCaptureMessage) -> bool {
        self.item_id == other.item_id &&
        self.encounter_id == other.encounter_id &&
        self.spawn_point_id == other.spawn_point_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UseItemCaptureMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct NicknamePokemonMessage {
    // message fields
    pokemon_id: ::std::option::Option<u64>,
    nickname: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NicknamePokemonMessage {}

impl NicknamePokemonMessage {
    pub fn new() -> NicknamePokemonMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NicknamePokemonMessage {
        static mut instance: ::protobuf::lazy::Lazy<NicknamePokemonMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NicknamePokemonMessage,
        };
        unsafe {
            instance.get(|| {
                NicknamePokemonMessage {
                    pokemon_id: ::std::option::Option::None,
                    nickname: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional fixed64 pokemon_id = 1;

    pub fn clear_pokemon_id(&mut self) {
        self.pokemon_id = ::std::option::Option::None;
    }

    pub fn has_pokemon_id(&self) -> bool {
        self.pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_id(&mut self, v: u64) {
        self.pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_id(&self) -> u64 {
        self.pokemon_id.unwrap_or(0)
    }

    // optional string nickname = 2;

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
}

impl ::protobuf::Message for NicknamePokemonMessage {
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
                    self.pokemon_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.nickname));
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
        if self.pokemon_id.is_some() {
            my_size += 9;
        };
        for value in &self.nickname {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pokemon_id {
            try!(os.write_fixed64(1, v));
        };
        if let Some(v) = self.nickname.as_ref() {
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
        ::std::any::TypeId::of::<NicknamePokemonMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NicknamePokemonMessage {
    fn new() -> NicknamePokemonMessage {
        NicknamePokemonMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<NicknamePokemonMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "pokemon_id",
                    NicknamePokemonMessage::has_pokemon_id,
                    NicknamePokemonMessage::get_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "nickname",
                    NicknamePokemonMessage::has_nickname,
                    NicknamePokemonMessage::get_nickname,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NicknamePokemonMessage>(
                    "NicknamePokemonMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NicknamePokemonMessage {
    fn clear(&mut self) {
        self.clear_pokemon_id();
        self.clear_nickname();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for NicknamePokemonMessage {
    fn eq(&self, other: &NicknamePokemonMessage) -> bool {
        self.pokemon_id == other.pokemon_id &&
        self.nickname == other.nickname &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for NicknamePokemonMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RecycleInventoryItemMessage {
    // message fields
    item_id: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemId>,
    count: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RecycleInventoryItemMessage {}

impl RecycleInventoryItemMessage {
    pub fn new() -> RecycleInventoryItemMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RecycleInventoryItemMessage {
        static mut instance: ::protobuf::lazy::Lazy<RecycleInventoryItemMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RecycleInventoryItemMessage,
        };
        unsafe {
            instance.get(|| {
                RecycleInventoryItemMessage {
                    item_id: ::std::option::Option::None,
                    count: ::std::option::Option::None,
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

    // optional int32 count = 2;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: i32) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count(&self) -> i32 {
        self.count.unwrap_or(0)
    }
}

impl ::protobuf::Message for RecycleInventoryItemMessage {
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
                    let tmp = try!(is.read_int32());
                    self.count = ::std::option::Option::Some(tmp);
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
        for value in &self.count {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_id {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.count {
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
        ::std::any::TypeId::of::<RecycleInventoryItemMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RecycleInventoryItemMessage {
    fn new() -> RecycleInventoryItemMessage {
        RecycleInventoryItemMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<RecycleInventoryItemMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "item_id",
                    RecycleInventoryItemMessage::has_item_id,
                    RecycleInventoryItemMessage::get_item_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "count",
                    RecycleInventoryItemMessage::has_count,
                    RecycleInventoryItemMessage::get_count,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RecycleInventoryItemMessage>(
                    "RecycleInventoryItemMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RecycleInventoryItemMessage {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_count();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RecycleInventoryItemMessage {
    fn eq(&self, other: &RecycleInventoryItemMessage) -> bool {
        self.item_id == other.item_id &&
        self.count == other.count &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RecycleInventoryItemMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UseItemGymMessage {
    // message fields
    item_id: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemId>,
    gym_id: ::protobuf::SingularField<::std::string::String>,
    player_latitude: ::std::option::Option<f64>,
    player_longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UseItemGymMessage {}

impl UseItemGymMessage {
    pub fn new() -> UseItemGymMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UseItemGymMessage {
        static mut instance: ::protobuf::lazy::Lazy<UseItemGymMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UseItemGymMessage,
        };
        unsafe {
            instance.get(|| {
                UseItemGymMessage {
                    item_id: ::std::option::Option::None,
                    gym_id: ::protobuf::SingularField::none(),
                    player_latitude: ::std::option::Option::None,
                    player_longitude: ::std::option::Option::None,
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

    // optional string gym_id = 2;

    pub fn clear_gym_id(&mut self) {
        self.gym_id.clear();
    }

    pub fn has_gym_id(&self) -> bool {
        self.gym_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gym_id(&mut self, v: ::std::string::String) {
        self.gym_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gym_id(&mut self) -> &mut ::std::string::String {
        if self.gym_id.is_none() {
            self.gym_id.set_default();
        };
        self.gym_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_gym_id(&mut self) -> ::std::string::String {
        self.gym_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_gym_id(&self) -> &str {
        match self.gym_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional double player_latitude = 3;

    pub fn clear_player_latitude(&mut self) {
        self.player_latitude = ::std::option::Option::None;
    }

    pub fn has_player_latitude(&self) -> bool {
        self.player_latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_latitude(&mut self, v: f64) {
        self.player_latitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_latitude(&self) -> f64 {
        self.player_latitude.unwrap_or(0.)
    }

    // optional double player_longitude = 4;

    pub fn clear_player_longitude(&mut self) {
        self.player_longitude = ::std::option::Option::None;
    }

    pub fn has_player_longitude(&self) -> bool {
        self.player_longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_longitude(&mut self, v: f64) {
        self.player_longitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_longitude(&self) -> f64 {
        self.player_longitude.unwrap_or(0.)
    }
}

impl ::protobuf::Message for UseItemGymMessage {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.gym_id));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_latitude = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_longitude = ::std::option::Option::Some(tmp);
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
        for value in &self.gym_id {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.player_latitude.is_some() {
            my_size += 9;
        };
        if self.player_longitude.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_id {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.gym_id.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.player_latitude {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.player_longitude {
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
        ::std::any::TypeId::of::<UseItemGymMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UseItemGymMessage {
    fn new() -> UseItemGymMessage {
        UseItemGymMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<UseItemGymMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "item_id",
                    UseItemGymMessage::has_item_id,
                    UseItemGymMessage::get_item_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "gym_id",
                    UseItemGymMessage::has_gym_id,
                    UseItemGymMessage::get_gym_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_latitude",
                    UseItemGymMessage::has_player_latitude,
                    UseItemGymMessage::get_player_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_longitude",
                    UseItemGymMessage::has_player_longitude,
                    UseItemGymMessage::get_player_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UseItemGymMessage>(
                    "UseItemGymMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UseItemGymMessage {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_gym_id();
        self.clear_player_latitude();
        self.clear_player_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UseItemGymMessage {
    fn eq(&self, other: &UseItemGymMessage) -> bool {
        self.item_id == other.item_id &&
        self.gym_id == other.gym_id &&
        self.player_latitude == other.player_latitude &&
        self.player_longitude == other.player_longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UseItemGymMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UseItemReviveMessage {
    // message fields
    item_id: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemId>,
    pokemon_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UseItemReviveMessage {}

impl UseItemReviveMessage {
    pub fn new() -> UseItemReviveMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UseItemReviveMessage {
        static mut instance: ::protobuf::lazy::Lazy<UseItemReviveMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UseItemReviveMessage,
        };
        unsafe {
            instance.get(|| {
                UseItemReviveMessage {
                    item_id: ::std::option::Option::None,
                    pokemon_id: ::std::option::Option::None,
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

    // optional fixed64 pokemon_id = 2;

    pub fn clear_pokemon_id(&mut self) {
        self.pokemon_id = ::std::option::Option::None;
    }

    pub fn has_pokemon_id(&self) -> bool {
        self.pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_id(&mut self, v: u64) {
        self.pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_id(&self) -> u64 {
        self.pokemon_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for UseItemReviveMessage {
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.pokemon_id = ::std::option::Option::Some(tmp);
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
        if self.pokemon_id.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_id {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.pokemon_id {
            try!(os.write_fixed64(2, v));
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
        ::std::any::TypeId::of::<UseItemReviveMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UseItemReviveMessage {
    fn new() -> UseItemReviveMessage {
        UseItemReviveMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<UseItemReviveMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "item_id",
                    UseItemReviveMessage::has_item_id,
                    UseItemReviveMessage::get_item_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "pokemon_id",
                    UseItemReviveMessage::has_pokemon_id,
                    UseItemReviveMessage::get_pokemon_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UseItemReviveMessage>(
                    "UseItemReviveMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UseItemReviveMessage {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_pokemon_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UseItemReviveMessage {
    fn eq(&self, other: &UseItemReviveMessage) -> bool {
        self.item_id == other.item_id &&
        self.pokemon_id == other.pokemon_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UseItemReviveMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UseIncenseMessage {
    // message fields
    incense_type: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemId>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UseIncenseMessage {}

impl UseIncenseMessage {
    pub fn new() -> UseIncenseMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UseIncenseMessage {
        static mut instance: ::protobuf::lazy::Lazy<UseIncenseMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UseIncenseMessage,
        };
        unsafe {
            instance.get(|| {
                UseIncenseMessage {
                    incense_type: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Inventory.Item.ItemId incense_type = 1;

    pub fn clear_incense_type(&mut self) {
        self.incense_type = ::std::option::Option::None;
    }

    pub fn has_incense_type(&self) -> bool {
        self.incense_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_incense_type(&mut self, v: super::POGOProtos_Inventory_Item::ItemId) {
        self.incense_type = ::std::option::Option::Some(v);
    }

    pub fn get_incense_type(&self) -> super::POGOProtos_Inventory_Item::ItemId {
        self.incense_type.unwrap_or(super::POGOProtos_Inventory_Item::ItemId::ITEM_UNKNOWN)
    }
}

impl ::protobuf::Message for UseIncenseMessage {
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
                    self.incense_type = ::std::option::Option::Some(tmp);
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
        for value in &self.incense_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.incense_type {
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
        ::std::any::TypeId::of::<UseIncenseMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UseIncenseMessage {
    fn new() -> UseIncenseMessage {
        UseIncenseMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<UseIncenseMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "incense_type",
                    UseIncenseMessage::has_incense_type,
                    UseIncenseMessage::get_incense_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UseIncenseMessage>(
                    "UseIncenseMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UseIncenseMessage {
    fn clear(&mut self) {
        self.clear_incense_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UseIncenseMessage {
    fn eq(&self, other: &UseIncenseMessage) -> bool {
        self.incense_type == other.incense_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UseIncenseMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetPlayerTeamMessage {
    // message fields
    team: ::std::option::Option<super::POGOProtos_Enums::TeamColor>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetPlayerTeamMessage {}

impl SetPlayerTeamMessage {
    pub fn new() -> SetPlayerTeamMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetPlayerTeamMessage {
        static mut instance: ::protobuf::lazy::Lazy<SetPlayerTeamMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetPlayerTeamMessage,
        };
        unsafe {
            instance.get(|| {
                SetPlayerTeamMessage {
                    team: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Enums.TeamColor team = 1;

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
}

impl ::protobuf::Message for SetPlayerTeamMessage {
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
                    self.team = ::std::option::Option::Some(tmp);
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
        for value in &self.team {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team {
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
        ::std::any::TypeId::of::<SetPlayerTeamMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetPlayerTeamMessage {
    fn new() -> SetPlayerTeamMessage {
        SetPlayerTeamMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetPlayerTeamMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "team",
                    SetPlayerTeamMessage::has_team,
                    SetPlayerTeamMessage::get_team,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetPlayerTeamMessage>(
                    "SetPlayerTeamMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetPlayerTeamMessage {
    fn clear(&mut self) {
        self.clear_team();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetPlayerTeamMessage {
    fn eq(&self, other: &SetPlayerTeamMessage) -> bool {
        self.team == other.team &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetPlayerTeamMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CollectDailyDefenderBonusMessage {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CollectDailyDefenderBonusMessage {}

impl CollectDailyDefenderBonusMessage {
    pub fn new() -> CollectDailyDefenderBonusMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CollectDailyDefenderBonusMessage {
        static mut instance: ::protobuf::lazy::Lazy<CollectDailyDefenderBonusMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CollectDailyDefenderBonusMessage,
        };
        unsafe {
            instance.get(|| {
                CollectDailyDefenderBonusMessage {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CollectDailyDefenderBonusMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<CollectDailyDefenderBonusMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CollectDailyDefenderBonusMessage {
    fn new() -> CollectDailyDefenderBonusMessage {
        CollectDailyDefenderBonusMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CollectDailyDefenderBonusMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CollectDailyDefenderBonusMessage>(
                    "CollectDailyDefenderBonusMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CollectDailyDefenderBonusMessage {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CollectDailyDefenderBonusMessage {
    fn eq(&self, other: &CollectDailyDefenderBonusMessage) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CollectDailyDefenderBonusMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DiskEncounterMessage {
    // message fields
    encounter_id: ::std::option::Option<u64>,
    fort_id: ::protobuf::SingularField<::std::string::String>,
    player_latitude: ::std::option::Option<f64>,
    player_longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DiskEncounterMessage {}

impl DiskEncounterMessage {
    pub fn new() -> DiskEncounterMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DiskEncounterMessage {
        static mut instance: ::protobuf::lazy::Lazy<DiskEncounterMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiskEncounterMessage,
        };
        unsafe {
            instance.get(|| {
                DiskEncounterMessage {
                    encounter_id: ::std::option::Option::None,
                    fort_id: ::protobuf::SingularField::none(),
                    player_latitude: ::std::option::Option::None,
                    player_longitude: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 encounter_id = 1;

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

    // optional string fort_id = 2;

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

    // optional double player_latitude = 3;

    pub fn clear_player_latitude(&mut self) {
        self.player_latitude = ::std::option::Option::None;
    }

    pub fn has_player_latitude(&self) -> bool {
        self.player_latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_latitude(&mut self, v: f64) {
        self.player_latitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_latitude(&self) -> f64 {
        self.player_latitude.unwrap_or(0.)
    }

    // optional double player_longitude = 4;

    pub fn clear_player_longitude(&mut self) {
        self.player_longitude = ::std::option::Option::None;
    }

    pub fn has_player_longitude(&self) -> bool {
        self.player_longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_longitude(&mut self, v: f64) {
        self.player_longitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_longitude(&self) -> f64 {
        self.player_longitude.unwrap_or(0.)
    }
}

impl ::protobuf::Message for DiskEncounterMessage {
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
                    self.encounter_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fort_id));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_latitude = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_longitude = ::std::option::Option::Some(tmp);
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
        for value in &self.encounter_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.fort_id {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.player_latitude.is_some() {
            my_size += 9;
        };
        if self.player_longitude.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.encounter_id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.fort_id.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.player_latitude {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.player_longitude {
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
        ::std::any::TypeId::of::<DiskEncounterMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DiskEncounterMessage {
    fn new() -> DiskEncounterMessage {
        DiskEncounterMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<DiskEncounterMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "encounter_id",
                    DiskEncounterMessage::has_encounter_id,
                    DiskEncounterMessage::get_encounter_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "fort_id",
                    DiskEncounterMessage::has_fort_id,
                    DiskEncounterMessage::get_fort_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_latitude",
                    DiskEncounterMessage::has_player_latitude,
                    DiskEncounterMessage::get_player_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_longitude",
                    DiskEncounterMessage::has_player_longitude,
                    DiskEncounterMessage::get_player_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DiskEncounterMessage>(
                    "DiskEncounterMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DiskEncounterMessage {
    fn clear(&mut self) {
        self.clear_encounter_id();
        self.clear_fort_id();
        self.clear_player_latitude();
        self.clear_player_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DiskEncounterMessage {
    fn eq(&self, other: &DiskEncounterMessage) -> bool {
        self.encounter_id == other.encounter_id &&
        self.fort_id == other.fort_id &&
        self.player_latitude == other.player_latitude &&
        self.player_longitude == other.player_longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DiskEncounterMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UseItemXpBoostMessage {
    // message fields
    item_id: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemId>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UseItemXpBoostMessage {}

impl UseItemXpBoostMessage {
    pub fn new() -> UseItemXpBoostMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UseItemXpBoostMessage {
        static mut instance: ::protobuf::lazy::Lazy<UseItemXpBoostMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UseItemXpBoostMessage,
        };
        unsafe {
            instance.get(|| {
                UseItemXpBoostMessage {
                    item_id: ::std::option::Option::None,
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
}

impl ::protobuf::Message for UseItemXpBoostMessage {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_id {
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
        ::std::any::TypeId::of::<UseItemXpBoostMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UseItemXpBoostMessage {
    fn new() -> UseItemXpBoostMessage {
        UseItemXpBoostMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<UseItemXpBoostMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "item_id",
                    UseItemXpBoostMessage::has_item_id,
                    UseItemXpBoostMessage::get_item_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UseItemXpBoostMessage>(
                    "UseItemXpBoostMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UseItemXpBoostMessage {
    fn clear(&mut self) {
        self.clear_item_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UseItemXpBoostMessage {
    fn eq(&self, other: &UseItemXpBoostMessage) -> bool {
        self.item_id == other.item_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UseItemXpBoostMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EvolvePokemonMessage {
    // message fields
    pokemon_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EvolvePokemonMessage {}

impl EvolvePokemonMessage {
    pub fn new() -> EvolvePokemonMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EvolvePokemonMessage {
        static mut instance: ::protobuf::lazy::Lazy<EvolvePokemonMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EvolvePokemonMessage,
        };
        unsafe {
            instance.get(|| {
                EvolvePokemonMessage {
                    pokemon_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional fixed64 pokemon_id = 1;

    pub fn clear_pokemon_id(&mut self) {
        self.pokemon_id = ::std::option::Option::None;
    }

    pub fn has_pokemon_id(&self) -> bool {
        self.pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_id(&mut self, v: u64) {
        self.pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_id(&self) -> u64 {
        self.pokemon_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for EvolvePokemonMessage {
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
                    self.pokemon_id = ::std::option::Option::Some(tmp);
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
        if self.pokemon_id.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pokemon_id {
            try!(os.write_fixed64(1, v));
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
        ::std::any::TypeId::of::<EvolvePokemonMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EvolvePokemonMessage {
    fn new() -> EvolvePokemonMessage {
        EvolvePokemonMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<EvolvePokemonMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "pokemon_id",
                    EvolvePokemonMessage::has_pokemon_id,
                    EvolvePokemonMessage::get_pokemon_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EvolvePokemonMessage>(
                    "EvolvePokemonMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EvolvePokemonMessage {
    fn clear(&mut self) {
        self.clear_pokemon_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EvolvePokemonMessage {
    fn eq(&self, other: &EvolvePokemonMessage) -> bool {
        self.pokemon_id == other.pokemon_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EvolvePokemonMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetMapObjectsMessage {
    // message fields
    cell_id: ::std::vec::Vec<u64>,
    since_timestamp_ms: ::std::vec::Vec<i64>,
    latitude: ::std::option::Option<f64>,
    longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetMapObjectsMessage {}

impl GetMapObjectsMessage {
    pub fn new() -> GetMapObjectsMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetMapObjectsMessage {
        static mut instance: ::protobuf::lazy::Lazy<GetMapObjectsMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetMapObjectsMessage,
        };
        unsafe {
            instance.get(|| {
                GetMapObjectsMessage {
                    cell_id: ::std::vec::Vec::new(),
                    since_timestamp_ms: ::std::vec::Vec::new(),
                    latitude: ::std::option::Option::None,
                    longitude: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated uint64 cell_id = 1;

    pub fn clear_cell_id(&mut self) {
        self.cell_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_cell_id(&mut self, v: ::std::vec::Vec<u64>) {
        self.cell_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cell_id(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.cell_id
    }

    // Take field
    pub fn take_cell_id(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.cell_id, ::std::vec::Vec::new())
    }

    pub fn get_cell_id(&self) -> &[u64] {
        &self.cell_id
    }

    // repeated int64 since_timestamp_ms = 2;

    pub fn clear_since_timestamp_ms(&mut self) {
        self.since_timestamp_ms.clear();
    }

    // Param is passed by value, moved
    pub fn set_since_timestamp_ms(&mut self, v: ::std::vec::Vec<i64>) {
        self.since_timestamp_ms = v;
    }

    // Mutable pointer to the field.
    pub fn mut_since_timestamp_ms(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.since_timestamp_ms
    }

    // Take field
    pub fn take_since_timestamp_ms(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.since_timestamp_ms, ::std::vec::Vec::new())
    }

    pub fn get_since_timestamp_ms(&self) -> &[i64] {
        &self.since_timestamp_ms
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

impl ::protobuf::Message for GetMapObjectsMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.cell_id));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.since_timestamp_ms));
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
        if !self.cell_id.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(1, &self.cell_id);
        };
        if !self.since_timestamp_ms.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(2, &self.since_timestamp_ms);
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
        if !self.cell_id.is_empty() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.cell_id)));
            for v in &self.cell_id {
                try!(os.write_uint64_no_tag(*v));
            };
        };
        if !self.since_timestamp_ms.is_empty() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.since_timestamp_ms)));
            for v in &self.since_timestamp_ms {
                try!(os.write_int64_no_tag(*v));
            };
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
        ::std::any::TypeId::of::<GetMapObjectsMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetMapObjectsMessage {
    fn new() -> GetMapObjectsMessage {
        GetMapObjectsMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetMapObjectsMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "cell_id",
                    GetMapObjectsMessage::get_cell_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "since_timestamp_ms",
                    GetMapObjectsMessage::get_since_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "latitude",
                    GetMapObjectsMessage::has_latitude,
                    GetMapObjectsMessage::get_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "longitude",
                    GetMapObjectsMessage::has_longitude,
                    GetMapObjectsMessage::get_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetMapObjectsMessage>(
                    "GetMapObjectsMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetMapObjectsMessage {
    fn clear(&mut self) {
        self.clear_cell_id();
        self.clear_since_timestamp_ms();
        self.clear_latitude();
        self.clear_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetMapObjectsMessage {
    fn eq(&self, other: &GetMapObjectsMessage) -> bool {
        self.cell_id == other.cell_id &&
        self.since_timestamp_ms == other.since_timestamp_ms &&
        self.latitude == other.latitude &&
        self.longitude == other.longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetMapObjectsMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StartGymBattleMessage {
    // message fields
    gym_id: ::protobuf::SingularField<::std::string::String>,
    attacking_pokemon_ids: ::std::vec::Vec<u64>,
    defending_pokemon_id: ::std::option::Option<u64>,
    player_latitude: ::std::option::Option<f64>,
    player_longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StartGymBattleMessage {}

impl StartGymBattleMessage {
    pub fn new() -> StartGymBattleMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StartGymBattleMessage {
        static mut instance: ::protobuf::lazy::Lazy<StartGymBattleMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StartGymBattleMessage,
        };
        unsafe {
            instance.get(|| {
                StartGymBattleMessage {
                    gym_id: ::protobuf::SingularField::none(),
                    attacking_pokemon_ids: ::std::vec::Vec::new(),
                    defending_pokemon_id: ::std::option::Option::None,
                    player_latitude: ::std::option::Option::None,
                    player_longitude: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string gym_id = 1;

    pub fn clear_gym_id(&mut self) {
        self.gym_id.clear();
    }

    pub fn has_gym_id(&self) -> bool {
        self.gym_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gym_id(&mut self, v: ::std::string::String) {
        self.gym_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gym_id(&mut self) -> &mut ::std::string::String {
        if self.gym_id.is_none() {
            self.gym_id.set_default();
        };
        self.gym_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_gym_id(&mut self) -> ::std::string::String {
        self.gym_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_gym_id(&self) -> &str {
        match self.gym_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated fixed64 attacking_pokemon_ids = 2;

    pub fn clear_attacking_pokemon_ids(&mut self) {
        self.attacking_pokemon_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_attacking_pokemon_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.attacking_pokemon_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attacking_pokemon_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.attacking_pokemon_ids
    }

    // Take field
    pub fn take_attacking_pokemon_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.attacking_pokemon_ids, ::std::vec::Vec::new())
    }

    pub fn get_attacking_pokemon_ids(&self) -> &[u64] {
        &self.attacking_pokemon_ids
    }

    // optional fixed64 defending_pokemon_id = 3;

    pub fn clear_defending_pokemon_id(&mut self) {
        self.defending_pokemon_id = ::std::option::Option::None;
    }

    pub fn has_defending_pokemon_id(&self) -> bool {
        self.defending_pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_defending_pokemon_id(&mut self, v: u64) {
        self.defending_pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_defending_pokemon_id(&self) -> u64 {
        self.defending_pokemon_id.unwrap_or(0)
    }

    // optional double player_latitude = 4;

    pub fn clear_player_latitude(&mut self) {
        self.player_latitude = ::std::option::Option::None;
    }

    pub fn has_player_latitude(&self) -> bool {
        self.player_latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_latitude(&mut self, v: f64) {
        self.player_latitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_latitude(&self) -> f64 {
        self.player_latitude.unwrap_or(0.)
    }

    // optional double player_longitude = 5;

    pub fn clear_player_longitude(&mut self) {
        self.player_longitude = ::std::option::Option::None;
    }

    pub fn has_player_longitude(&self) -> bool {
        self.player_longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_longitude(&mut self, v: f64) {
        self.player_longitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_longitude(&self) -> f64 {
        self.player_longitude.unwrap_or(0.)
    }
}

impl ::protobuf::Message for StartGymBattleMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.gym_id));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.attacking_pokemon_ids));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.defending_pokemon_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_latitude = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_longitude = ::std::option::Option::Some(tmp);
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
        for value in &self.gym_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += 9 * self.attacking_pokemon_ids.len() as u32;
        if self.defending_pokemon_id.is_some() {
            my_size += 9;
        };
        if self.player_latitude.is_some() {
            my_size += 9;
        };
        if self.player_longitude.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gym_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in &self.attacking_pokemon_ids {
            try!(os.write_fixed64(2, *v));
        };
        if let Some(v) = self.defending_pokemon_id {
            try!(os.write_fixed64(3, v));
        };
        if let Some(v) = self.player_latitude {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.player_longitude {
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
        ::std::any::TypeId::of::<StartGymBattleMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StartGymBattleMessage {
    fn new() -> StartGymBattleMessage {
        StartGymBattleMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<StartGymBattleMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "gym_id",
                    StartGymBattleMessage::has_gym_id,
                    StartGymBattleMessage::get_gym_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "attacking_pokemon_ids",
                    StartGymBattleMessage::get_attacking_pokemon_ids,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "defending_pokemon_id",
                    StartGymBattleMessage::has_defending_pokemon_id,
                    StartGymBattleMessage::get_defending_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_latitude",
                    StartGymBattleMessage::has_player_latitude,
                    StartGymBattleMessage::get_player_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_longitude",
                    StartGymBattleMessage::has_player_longitude,
                    StartGymBattleMessage::get_player_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StartGymBattleMessage>(
                    "StartGymBattleMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StartGymBattleMessage {
    fn clear(&mut self) {
        self.clear_gym_id();
        self.clear_attacking_pokemon_ids();
        self.clear_defending_pokemon_id();
        self.clear_player_latitude();
        self.clear_player_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StartGymBattleMessage {
    fn eq(&self, other: &StartGymBattleMessage) -> bool {
        self.gym_id == other.gym_id &&
        self.attacking_pokemon_ids == other.attacking_pokemon_ids &&
        self.defending_pokemon_id == other.defending_pokemon_id &&
        self.player_latitude == other.player_latitude &&
        self.player_longitude == other.player_longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StartGymBattleMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EncounterMessage {
    // message fields
    encounter_id: ::std::option::Option<u64>,
    spawn_point_id: ::protobuf::SingularField<::std::string::String>,
    player_latitude: ::std::option::Option<f64>,
    player_longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EncounterMessage {}

impl EncounterMessage {
    pub fn new() -> EncounterMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EncounterMessage {
        static mut instance: ::protobuf::lazy::Lazy<EncounterMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EncounterMessage,
        };
        unsafe {
            instance.get(|| {
                EncounterMessage {
                    encounter_id: ::std::option::Option::None,
                    spawn_point_id: ::protobuf::SingularField::none(),
                    player_latitude: ::std::option::Option::None,
                    player_longitude: ::std::option::Option::None,
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

    // optional string spawn_point_id = 2;

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

    // optional double player_latitude = 3;

    pub fn clear_player_latitude(&mut self) {
        self.player_latitude = ::std::option::Option::None;
    }

    pub fn has_player_latitude(&self) -> bool {
        self.player_latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_latitude(&mut self, v: f64) {
        self.player_latitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_latitude(&self) -> f64 {
        self.player_latitude.unwrap_or(0.)
    }

    // optional double player_longitude = 4;

    pub fn clear_player_longitude(&mut self) {
        self.player_longitude = ::std::option::Option::None;
    }

    pub fn has_player_longitude(&self) -> bool {
        self.player_longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_longitude(&mut self, v: f64) {
        self.player_longitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_longitude(&self) -> f64 {
        self.player_longitude.unwrap_or(0.)
    }
}

impl ::protobuf::Message for EncounterMessage {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.spawn_point_id));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_latitude = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_longitude = ::std::option::Option::Some(tmp);
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
        for value in &self.spawn_point_id {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.player_latitude.is_some() {
            my_size += 9;
        };
        if self.player_longitude.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.encounter_id {
            try!(os.write_fixed64(1, v));
        };
        if let Some(v) = self.spawn_point_id.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.player_latitude {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.player_longitude {
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
        ::std::any::TypeId::of::<EncounterMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EncounterMessage {
    fn new() -> EncounterMessage {
        EncounterMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<EncounterMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "encounter_id",
                    EncounterMessage::has_encounter_id,
                    EncounterMessage::get_encounter_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "spawn_point_id",
                    EncounterMessage::has_spawn_point_id,
                    EncounterMessage::get_spawn_point_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_latitude",
                    EncounterMessage::has_player_latitude,
                    EncounterMessage::get_player_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_longitude",
                    EncounterMessage::has_player_longitude,
                    EncounterMessage::get_player_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EncounterMessage>(
                    "EncounterMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EncounterMessage {
    fn clear(&mut self) {
        self.clear_encounter_id();
        self.clear_spawn_point_id();
        self.clear_player_latitude();
        self.clear_player_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EncounterMessage {
    fn eq(&self, other: &EncounterMessage) -> bool {
        self.encounter_id == other.encounter_id &&
        self.spawn_point_id == other.spawn_point_id &&
        self.player_latitude == other.player_latitude &&
        self.player_longitude == other.player_longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EncounterMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReleasePokemonMessage {
    // message fields
    pokemon_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReleasePokemonMessage {}

impl ReleasePokemonMessage {
    pub fn new() -> ReleasePokemonMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReleasePokemonMessage {
        static mut instance: ::protobuf::lazy::Lazy<ReleasePokemonMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReleasePokemonMessage,
        };
        unsafe {
            instance.get(|| {
                ReleasePokemonMessage {
                    pokemon_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional fixed64 pokemon_id = 1;

    pub fn clear_pokemon_id(&mut self) {
        self.pokemon_id = ::std::option::Option::None;
    }

    pub fn has_pokemon_id(&self) -> bool {
        self.pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_id(&mut self, v: u64) {
        self.pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_id(&self) -> u64 {
        self.pokemon_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for ReleasePokemonMessage {
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
                    self.pokemon_id = ::std::option::Option::Some(tmp);
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
        if self.pokemon_id.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pokemon_id {
            try!(os.write_fixed64(1, v));
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
        ::std::any::TypeId::of::<ReleasePokemonMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReleasePokemonMessage {
    fn new() -> ReleasePokemonMessage {
        ReleasePokemonMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReleasePokemonMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "pokemon_id",
                    ReleasePokemonMessage::has_pokemon_id,
                    ReleasePokemonMessage::get_pokemon_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReleasePokemonMessage>(
                    "ReleasePokemonMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReleasePokemonMessage {
    fn clear(&mut self) {
        self.clear_pokemon_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReleasePokemonMessage {
    fn eq(&self, other: &ReleasePokemonMessage) -> bool {
        self.pokemon_id == other.pokemon_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReleasePokemonMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EncounterTutorialCompleteMessage {
    // message fields
    pokemon_id: ::std::option::Option<super::POGOProtos_Enums::PokemonId>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EncounterTutorialCompleteMessage {}

impl EncounterTutorialCompleteMessage {
    pub fn new() -> EncounterTutorialCompleteMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EncounterTutorialCompleteMessage {
        static mut instance: ::protobuf::lazy::Lazy<EncounterTutorialCompleteMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EncounterTutorialCompleteMessage,
        };
        unsafe {
            instance.get(|| {
                EncounterTutorialCompleteMessage {
                    pokemon_id: ::std::option::Option::None,
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
}

impl ::protobuf::Message for EncounterTutorialCompleteMessage {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pokemon_id {
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
        ::std::any::TypeId::of::<EncounterTutorialCompleteMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EncounterTutorialCompleteMessage {
    fn new() -> EncounterTutorialCompleteMessage {
        EncounterTutorialCompleteMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<EncounterTutorialCompleteMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "pokemon_id",
                    EncounterTutorialCompleteMessage::has_pokemon_id,
                    EncounterTutorialCompleteMessage::get_pokemon_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EncounterTutorialCompleteMessage>(
                    "EncounterTutorialCompleteMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EncounterTutorialCompleteMessage {
    fn clear(&mut self) {
        self.clear_pokemon_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EncounterTutorialCompleteMessage {
    fn eq(&self, other: &EncounterTutorialCompleteMessage) -> bool {
        self.pokemon_id == other.pokemon_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EncounterTutorialCompleteMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ClaimCodenameMessage {
    // message fields
    codename: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClaimCodenameMessage {}

impl ClaimCodenameMessage {
    pub fn new() -> ClaimCodenameMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClaimCodenameMessage {
        static mut instance: ::protobuf::lazy::Lazy<ClaimCodenameMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClaimCodenameMessage,
        };
        unsafe {
            instance.get(|| {
                ClaimCodenameMessage {
                    codename: ::protobuf::SingularField::none(),
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
}

impl ::protobuf::Message for ClaimCodenameMessage {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.codename.as_ref() {
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
        ::std::any::TypeId::of::<ClaimCodenameMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClaimCodenameMessage {
    fn new() -> ClaimCodenameMessage {
        ClaimCodenameMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClaimCodenameMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "codename",
                    ClaimCodenameMessage::has_codename,
                    ClaimCodenameMessage::get_codename,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClaimCodenameMessage>(
                    "ClaimCodenameMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClaimCodenameMessage {
    fn clear(&mut self) {
        self.clear_codename();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ClaimCodenameMessage {
    fn eq(&self, other: &ClaimCodenameMessage) -> bool {
        self.codename == other.codename &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ClaimCodenameMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetIncensePokemonMessage {
    // message fields
    player_latitude: ::std::option::Option<f64>,
    player_longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetIncensePokemonMessage {}

impl GetIncensePokemonMessage {
    pub fn new() -> GetIncensePokemonMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetIncensePokemonMessage {
        static mut instance: ::protobuf::lazy::Lazy<GetIncensePokemonMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetIncensePokemonMessage,
        };
        unsafe {
            instance.get(|| {
                GetIncensePokemonMessage {
                    player_latitude: ::std::option::Option::None,
                    player_longitude: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional double player_latitude = 1;

    pub fn clear_player_latitude(&mut self) {
        self.player_latitude = ::std::option::Option::None;
    }

    pub fn has_player_latitude(&self) -> bool {
        self.player_latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_latitude(&mut self, v: f64) {
        self.player_latitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_latitude(&self) -> f64 {
        self.player_latitude.unwrap_or(0.)
    }

    // optional double player_longitude = 2;

    pub fn clear_player_longitude(&mut self) {
        self.player_longitude = ::std::option::Option::None;
    }

    pub fn has_player_longitude(&self) -> bool {
        self.player_longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_longitude(&mut self, v: f64) {
        self.player_longitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_longitude(&self) -> f64 {
        self.player_longitude.unwrap_or(0.)
    }
}

impl ::protobuf::Message for GetIncensePokemonMessage {
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
                    self.player_latitude = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_longitude = ::std::option::Option::Some(tmp);
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
        if self.player_latitude.is_some() {
            my_size += 9;
        };
        if self.player_longitude.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_latitude {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.player_longitude {
            try!(os.write_double(2, v));
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
        ::std::any::TypeId::of::<GetIncensePokemonMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetIncensePokemonMessage {
    fn new() -> GetIncensePokemonMessage {
        GetIncensePokemonMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetIncensePokemonMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_latitude",
                    GetIncensePokemonMessage::has_player_latitude,
                    GetIncensePokemonMessage::get_player_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_longitude",
                    GetIncensePokemonMessage::has_player_longitude,
                    GetIncensePokemonMessage::get_player_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetIncensePokemonMessage>(
                    "GetIncensePokemonMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetIncensePokemonMessage {
    fn clear(&mut self) {
        self.clear_player_latitude();
        self.clear_player_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetIncensePokemonMessage {
    fn eq(&self, other: &GetIncensePokemonMessage) -> bool {
        self.player_latitude == other.player_latitude &&
        self.player_longitude == other.player_longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetIncensePokemonMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct MarkTutorialCompleteMessage {
    // message fields
    tutorials_completed: ::std::vec::Vec<super::POGOProtos_Enums::TutorialState>,
    send_marketing_emails: ::std::option::Option<bool>,
    send_push_notifications: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MarkTutorialCompleteMessage {}

impl MarkTutorialCompleteMessage {
    pub fn new() -> MarkTutorialCompleteMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MarkTutorialCompleteMessage {
        static mut instance: ::protobuf::lazy::Lazy<MarkTutorialCompleteMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MarkTutorialCompleteMessage,
        };
        unsafe {
            instance.get(|| {
                MarkTutorialCompleteMessage {
                    tutorials_completed: ::std::vec::Vec::new(),
                    send_marketing_emails: ::std::option::Option::None,
                    send_push_notifications: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .POGOProtos.Enums.TutorialState tutorials_completed = 1;

    pub fn clear_tutorials_completed(&mut self) {
        self.tutorials_completed.clear();
    }

    // Param is passed by value, moved
    pub fn set_tutorials_completed(&mut self, v: ::std::vec::Vec<super::POGOProtos_Enums::TutorialState>) {
        self.tutorials_completed = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tutorials_completed(&mut self) -> &mut ::std::vec::Vec<super::POGOProtos_Enums::TutorialState> {
        &mut self.tutorials_completed
    }

    // Take field
    pub fn take_tutorials_completed(&mut self) -> ::std::vec::Vec<super::POGOProtos_Enums::TutorialState> {
        ::std::mem::replace(&mut self.tutorials_completed, ::std::vec::Vec::new())
    }

    pub fn get_tutorials_completed(&self) -> &[super::POGOProtos_Enums::TutorialState] {
        &self.tutorials_completed
    }

    // optional bool send_marketing_emails = 2;

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

    // optional bool send_push_notifications = 3;

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

impl ::protobuf::Message for MarkTutorialCompleteMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.tutorials_completed));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.send_marketing_emails = ::std::option::Option::Some(tmp);
                },
                3 => {
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
        for value in &self.tutorials_completed {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
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
        for v in &self.tutorials_completed {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.send_marketing_emails {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.send_push_notifications {
            try!(os.write_bool(3, v));
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
        ::std::any::TypeId::of::<MarkTutorialCompleteMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MarkTutorialCompleteMessage {
    fn new() -> MarkTutorialCompleteMessage {
        MarkTutorialCompleteMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<MarkTutorialCompleteMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "tutorials_completed",
                    MarkTutorialCompleteMessage::get_tutorials_completed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "send_marketing_emails",
                    MarkTutorialCompleteMessage::has_send_marketing_emails,
                    MarkTutorialCompleteMessage::get_send_marketing_emails,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "send_push_notifications",
                    MarkTutorialCompleteMessage::has_send_push_notifications,
                    MarkTutorialCompleteMessage::get_send_push_notifications,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MarkTutorialCompleteMessage>(
                    "MarkTutorialCompleteMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MarkTutorialCompleteMessage {
    fn clear(&mut self) {
        self.clear_tutorials_completed();
        self.clear_send_marketing_emails();
        self.clear_send_push_notifications();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for MarkTutorialCompleteMessage {
    fn eq(&self, other: &MarkTutorialCompleteMessage) -> bool {
        self.tutorials_completed == other.tutorials_completed &&
        self.send_marketing_emails == other.send_marketing_emails &&
        self.send_push_notifications == other.send_push_notifications &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for MarkTutorialCompleteMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FortSearchMessage {
    // message fields
    fort_id: ::protobuf::SingularField<::std::string::String>,
    player_latitude: ::std::option::Option<f64>,
    player_longitude: ::std::option::Option<f64>,
    fort_latitude: ::std::option::Option<f64>,
    fort_longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FortSearchMessage {}

impl FortSearchMessage {
    pub fn new() -> FortSearchMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FortSearchMessage {
        static mut instance: ::protobuf::lazy::Lazy<FortSearchMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FortSearchMessage,
        };
        unsafe {
            instance.get(|| {
                FortSearchMessage {
                    fort_id: ::protobuf::SingularField::none(),
                    player_latitude: ::std::option::Option::None,
                    player_longitude: ::std::option::Option::None,
                    fort_latitude: ::std::option::Option::None,
                    fort_longitude: ::std::option::Option::None,
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

    // optional double player_latitude = 2;

    pub fn clear_player_latitude(&mut self) {
        self.player_latitude = ::std::option::Option::None;
    }

    pub fn has_player_latitude(&self) -> bool {
        self.player_latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_latitude(&mut self, v: f64) {
        self.player_latitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_latitude(&self) -> f64 {
        self.player_latitude.unwrap_or(0.)
    }

    // optional double player_longitude = 3;

    pub fn clear_player_longitude(&mut self) {
        self.player_longitude = ::std::option::Option::None;
    }

    pub fn has_player_longitude(&self) -> bool {
        self.player_longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_longitude(&mut self, v: f64) {
        self.player_longitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_longitude(&self) -> f64 {
        self.player_longitude.unwrap_or(0.)
    }

    // optional double fort_latitude = 4;

    pub fn clear_fort_latitude(&mut self) {
        self.fort_latitude = ::std::option::Option::None;
    }

    pub fn has_fort_latitude(&self) -> bool {
        self.fort_latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fort_latitude(&mut self, v: f64) {
        self.fort_latitude = ::std::option::Option::Some(v);
    }

    pub fn get_fort_latitude(&self) -> f64 {
        self.fort_latitude.unwrap_or(0.)
    }

    // optional double fort_longitude = 5;

    pub fn clear_fort_longitude(&mut self) {
        self.fort_longitude = ::std::option::Option::None;
    }

    pub fn has_fort_longitude(&self) -> bool {
        self.fort_longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fort_longitude(&mut self, v: f64) {
        self.fort_longitude = ::std::option::Option::Some(v);
    }

    pub fn get_fort_longitude(&self) -> f64 {
        self.fort_longitude.unwrap_or(0.)
    }
}

impl ::protobuf::Message for FortSearchMessage {
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
                    let tmp = try!(is.read_double());
                    self.player_latitude = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_longitude = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.fort_latitude = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.fort_longitude = ::std::option::Option::Some(tmp);
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
        if self.player_latitude.is_some() {
            my_size += 9;
        };
        if self.player_longitude.is_some() {
            my_size += 9;
        };
        if self.fort_latitude.is_some() {
            my_size += 9;
        };
        if self.fort_longitude.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fort_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.player_latitude {
            try!(os.write_double(2, v));
        };
        if let Some(v) = self.player_longitude {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.fort_latitude {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.fort_longitude {
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
        ::std::any::TypeId::of::<FortSearchMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FortSearchMessage {
    fn new() -> FortSearchMessage {
        FortSearchMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<FortSearchMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "fort_id",
                    FortSearchMessage::has_fort_id,
                    FortSearchMessage::get_fort_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_latitude",
                    FortSearchMessage::has_player_latitude,
                    FortSearchMessage::get_player_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_longitude",
                    FortSearchMessage::has_player_longitude,
                    FortSearchMessage::get_player_longitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "fort_latitude",
                    FortSearchMessage::has_fort_latitude,
                    FortSearchMessage::get_fort_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "fort_longitude",
                    FortSearchMessage::has_fort_longitude,
                    FortSearchMessage::get_fort_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FortSearchMessage>(
                    "FortSearchMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FortSearchMessage {
    fn clear(&mut self) {
        self.clear_fort_id();
        self.clear_player_latitude();
        self.clear_player_longitude();
        self.clear_fort_latitude();
        self.clear_fort_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FortSearchMessage {
    fn eq(&self, other: &FortSearchMessage) -> bool {
        self.fort_id == other.fort_id &&
        self.player_latitude == other.player_latitude &&
        self.player_longitude == other.player_longitude &&
        self.fort_latitude == other.fort_latitude &&
        self.fort_longitude == other.fort_longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FortSearchMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AddFortModifierMessage {
    // message fields
    modifier_type: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemId>,
    fort_id: ::protobuf::SingularField<::std::string::String>,
    player_latitude: ::std::option::Option<f64>,
    player_longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddFortModifierMessage {}

impl AddFortModifierMessage {
    pub fn new() -> AddFortModifierMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddFortModifierMessage {
        static mut instance: ::protobuf::lazy::Lazy<AddFortModifierMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddFortModifierMessage,
        };
        unsafe {
            instance.get(|| {
                AddFortModifierMessage {
                    modifier_type: ::std::option::Option::None,
                    fort_id: ::protobuf::SingularField::none(),
                    player_latitude: ::std::option::Option::None,
                    player_longitude: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Inventory.Item.ItemId modifier_type = 1;

    pub fn clear_modifier_type(&mut self) {
        self.modifier_type = ::std::option::Option::None;
    }

    pub fn has_modifier_type(&self) -> bool {
        self.modifier_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modifier_type(&mut self, v: super::POGOProtos_Inventory_Item::ItemId) {
        self.modifier_type = ::std::option::Option::Some(v);
    }

    pub fn get_modifier_type(&self) -> super::POGOProtos_Inventory_Item::ItemId {
        self.modifier_type.unwrap_or(super::POGOProtos_Inventory_Item::ItemId::ITEM_UNKNOWN)
    }

    // optional string fort_id = 2;

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

    // optional double player_latitude = 3;

    pub fn clear_player_latitude(&mut self) {
        self.player_latitude = ::std::option::Option::None;
    }

    pub fn has_player_latitude(&self) -> bool {
        self.player_latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_latitude(&mut self, v: f64) {
        self.player_latitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_latitude(&self) -> f64 {
        self.player_latitude.unwrap_or(0.)
    }

    // optional double player_longitude = 4;

    pub fn clear_player_longitude(&mut self) {
        self.player_longitude = ::std::option::Option::None;
    }

    pub fn has_player_longitude(&self) -> bool {
        self.player_longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_longitude(&mut self, v: f64) {
        self.player_longitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_longitude(&self) -> f64 {
        self.player_longitude.unwrap_or(0.)
    }
}

impl ::protobuf::Message for AddFortModifierMessage {
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
                    self.modifier_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fort_id));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_latitude = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_longitude = ::std::option::Option::Some(tmp);
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
        for value in &self.modifier_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.fort_id {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.player_latitude.is_some() {
            my_size += 9;
        };
        if self.player_longitude.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.modifier_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.fort_id.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.player_latitude {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.player_longitude {
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
        ::std::any::TypeId::of::<AddFortModifierMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AddFortModifierMessage {
    fn new() -> AddFortModifierMessage {
        AddFortModifierMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddFortModifierMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "modifier_type",
                    AddFortModifierMessage::has_modifier_type,
                    AddFortModifierMessage::get_modifier_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "fort_id",
                    AddFortModifierMessage::has_fort_id,
                    AddFortModifierMessage::get_fort_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_latitude",
                    AddFortModifierMessage::has_player_latitude,
                    AddFortModifierMessage::get_player_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_longitude",
                    AddFortModifierMessage::has_player_longitude,
                    AddFortModifierMessage::get_player_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddFortModifierMessage>(
                    "AddFortModifierMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddFortModifierMessage {
    fn clear(&mut self) {
        self.clear_modifier_type();
        self.clear_fort_id();
        self.clear_player_latitude();
        self.clear_player_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AddFortModifierMessage {
    fn eq(&self, other: &AddFortModifierMessage) -> bool {
        self.modifier_type == other.modifier_type &&
        self.fort_id == other.fort_id &&
        self.player_latitude == other.player_latitude &&
        self.player_longitude == other.player_longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AddFortModifierMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetHatchedEggsMessage {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetHatchedEggsMessage {}

impl GetHatchedEggsMessage {
    pub fn new() -> GetHatchedEggsMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetHatchedEggsMessage {
        static mut instance: ::protobuf::lazy::Lazy<GetHatchedEggsMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetHatchedEggsMessage,
        };
        unsafe {
            instance.get(|| {
                GetHatchedEggsMessage {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for GetHatchedEggsMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<GetHatchedEggsMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetHatchedEggsMessage {
    fn new() -> GetHatchedEggsMessage {
        GetHatchedEggsMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetHatchedEggsMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetHatchedEggsMessage>(
                    "GetHatchedEggsMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetHatchedEggsMessage {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetHatchedEggsMessage {
    fn eq(&self, other: &GetHatchedEggsMessage) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetHatchedEggsMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetAvatarMessage {
    // message fields
    player_avatar: ::protobuf::SingularPtrField<super::POGOProtos_Data_Player::PlayerAvatar>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetAvatarMessage {}

impl SetAvatarMessage {
    pub fn new() -> SetAvatarMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetAvatarMessage {
        static mut instance: ::protobuf::lazy::Lazy<SetAvatarMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetAvatarMessage,
        };
        unsafe {
            instance.get(|| {
                SetAvatarMessage {
                    player_avatar: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Data.Player.PlayerAvatar player_avatar = 2;

    pub fn clear_player_avatar(&mut self) {
        self.player_avatar.clear();
    }

    pub fn has_player_avatar(&self) -> bool {
        self.player_avatar.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_avatar(&mut self, v: super::POGOProtos_Data_Player::PlayerAvatar) {
        self.player_avatar = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_avatar(&mut self) -> &mut super::POGOProtos_Data_Player::PlayerAvatar {
        if self.player_avatar.is_none() {
            self.player_avatar.set_default();
        };
        self.player_avatar.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_avatar(&mut self) -> super::POGOProtos_Data_Player::PlayerAvatar {
        self.player_avatar.take().unwrap_or_else(|| super::POGOProtos_Data_Player::PlayerAvatar::new())
    }

    pub fn get_player_avatar(&self) -> &super::POGOProtos_Data_Player::PlayerAvatar {
        self.player_avatar.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Player::PlayerAvatar::default_instance())
    }
}

impl ::protobuf::Message for SetAvatarMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_avatar));
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
        for value in &self.player_avatar {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_avatar.as_ref() {
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
        ::std::any::TypeId::of::<SetAvatarMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetAvatarMessage {
    fn new() -> SetAvatarMessage {
        SetAvatarMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetAvatarMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "player_avatar",
                    SetAvatarMessage::has_player_avatar,
                    SetAvatarMessage::get_player_avatar,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetAvatarMessage>(
                    "SetAvatarMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetAvatarMessage {
    fn clear(&mut self) {
        self.clear_player_avatar();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetAvatarMessage {
    fn eq(&self, other: &SetAvatarMessage) -> bool {
        self.player_avatar == other.player_avatar &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetAvatarMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UseItemEggIncubatorMessage {
    // message fields
    item_id: ::protobuf::SingularField<::std::string::String>,
    pokemon_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UseItemEggIncubatorMessage {}

impl UseItemEggIncubatorMessage {
    pub fn new() -> UseItemEggIncubatorMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UseItemEggIncubatorMessage {
        static mut instance: ::protobuf::lazy::Lazy<UseItemEggIncubatorMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UseItemEggIncubatorMessage,
        };
        unsafe {
            instance.get(|| {
                UseItemEggIncubatorMessage {
                    item_id: ::protobuf::SingularField::none(),
                    pokemon_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string item_id = 1;

    pub fn clear_item_id(&mut self) {
        self.item_id.clear();
    }

    pub fn has_item_id(&self) -> bool {
        self.item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_id(&mut self, v: ::std::string::String) {
        self.item_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_item_id(&mut self) -> &mut ::std::string::String {
        if self.item_id.is_none() {
            self.item_id.set_default();
        };
        self.item_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_item_id(&mut self) -> ::std::string::String {
        self.item_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_item_id(&self) -> &str {
        match self.item_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint64 pokemon_id = 2;

    pub fn clear_pokemon_id(&mut self) {
        self.pokemon_id = ::std::option::Option::None;
    }

    pub fn has_pokemon_id(&self) -> bool {
        self.pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_id(&mut self, v: u64) {
        self.pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_id(&self) -> u64 {
        self.pokemon_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for UseItemEggIncubatorMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.item_id));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.pokemon_id = ::std::option::Option::Some(tmp);
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
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.pokemon_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.pokemon_id {
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
        ::std::any::TypeId::of::<UseItemEggIncubatorMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UseItemEggIncubatorMessage {
    fn new() -> UseItemEggIncubatorMessage {
        UseItemEggIncubatorMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<UseItemEggIncubatorMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "item_id",
                    UseItemEggIncubatorMessage::has_item_id,
                    UseItemEggIncubatorMessage::get_item_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "pokemon_id",
                    UseItemEggIncubatorMessage::has_pokemon_id,
                    UseItemEggIncubatorMessage::get_pokemon_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UseItemEggIncubatorMessage>(
                    "UseItemEggIncubatorMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UseItemEggIncubatorMessage {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_pokemon_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UseItemEggIncubatorMessage {
    fn eq(&self, other: &UseItemEggIncubatorMessage) -> bool {
        self.item_id == other.item_id &&
        self.pokemon_id == other.pokemon_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UseItemEggIncubatorMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CheckCodenameAvailableMessage {
    // message fields
    codename: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CheckCodenameAvailableMessage {}

impl CheckCodenameAvailableMessage {
    pub fn new() -> CheckCodenameAvailableMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckCodenameAvailableMessage {
        static mut instance: ::protobuf::lazy::Lazy<CheckCodenameAvailableMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckCodenameAvailableMessage,
        };
        unsafe {
            instance.get(|| {
                CheckCodenameAvailableMessage {
                    codename: ::protobuf::SingularField::none(),
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
}

impl ::protobuf::Message for CheckCodenameAvailableMessage {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.codename.as_ref() {
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
        ::std::any::TypeId::of::<CheckCodenameAvailableMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CheckCodenameAvailableMessage {
    fn new() -> CheckCodenameAvailableMessage {
        CheckCodenameAvailableMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckCodenameAvailableMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "codename",
                    CheckCodenameAvailableMessage::has_codename,
                    CheckCodenameAvailableMessage::get_codename,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CheckCodenameAvailableMessage>(
                    "CheckCodenameAvailableMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckCodenameAvailableMessage {
    fn clear(&mut self) {
        self.clear_codename();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CheckCodenameAvailableMessage {
    fn eq(&self, other: &CheckCodenameAvailableMessage) -> bool {
        self.codename == other.codename &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CheckCodenameAvailableMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetInventoryMessage {
    // message fields
    last_timestamp_ms: ::std::option::Option<i64>,
    item_been_seen: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetInventoryMessage {}

impl GetInventoryMessage {
    pub fn new() -> GetInventoryMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetInventoryMessage {
        static mut instance: ::protobuf::lazy::Lazy<GetInventoryMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetInventoryMessage,
        };
        unsafe {
            instance.get(|| {
                GetInventoryMessage {
                    last_timestamp_ms: ::std::option::Option::None,
                    item_been_seen: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 last_timestamp_ms = 1;

    pub fn clear_last_timestamp_ms(&mut self) {
        self.last_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_last_timestamp_ms(&self) -> bool {
        self.last_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_timestamp_ms(&mut self, v: i64) {
        self.last_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_last_timestamp_ms(&self) -> i64 {
        self.last_timestamp_ms.unwrap_or(0)
    }

    // optional int32 item_been_seen = 2;

    pub fn clear_item_been_seen(&mut self) {
        self.item_been_seen = ::std::option::Option::None;
    }

    pub fn has_item_been_seen(&self) -> bool {
        self.item_been_seen.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_been_seen(&mut self, v: i32) {
        self.item_been_seen = ::std::option::Option::Some(v);
    }

    pub fn get_item_been_seen(&self) -> i32 {
        self.item_been_seen.unwrap_or(0)
    }
}

impl ::protobuf::Message for GetInventoryMessage {
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
                    self.last_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.item_been_seen = ::std::option::Option::Some(tmp);
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
        for value in &self.last_timestamp_ms {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.item_been_seen {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.last_timestamp_ms {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.item_been_seen {
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
        ::std::any::TypeId::of::<GetInventoryMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetInventoryMessage {
    fn new() -> GetInventoryMessage {
        GetInventoryMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetInventoryMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "last_timestamp_ms",
                    GetInventoryMessage::has_last_timestamp_ms,
                    GetInventoryMessage::get_last_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "item_been_seen",
                    GetInventoryMessage::has_item_been_seen,
                    GetInventoryMessage::get_item_been_seen,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetInventoryMessage>(
                    "GetInventoryMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetInventoryMessage {
    fn clear(&mut self) {
        self.clear_last_timestamp_ms();
        self.clear_item_been_seen();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetInventoryMessage {
    fn eq(&self, other: &GetInventoryMessage) -> bool {
        self.last_timestamp_ms == other.last_timestamp_ms &&
        self.item_been_seen == other.item_been_seen &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetInventoryMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UpgradePokemonMessage {
    // message fields
    pokemon_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpgradePokemonMessage {}

impl UpgradePokemonMessage {
    pub fn new() -> UpgradePokemonMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpgradePokemonMessage {
        static mut instance: ::protobuf::lazy::Lazy<UpgradePokemonMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpgradePokemonMessage,
        };
        unsafe {
            instance.get(|| {
                UpgradePokemonMessage {
                    pokemon_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional fixed64 pokemon_id = 1;

    pub fn clear_pokemon_id(&mut self) {
        self.pokemon_id = ::std::option::Option::None;
    }

    pub fn has_pokemon_id(&self) -> bool {
        self.pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_id(&mut self, v: u64) {
        self.pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_id(&self) -> u64 {
        self.pokemon_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for UpgradePokemonMessage {
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
                    self.pokemon_id = ::std::option::Option::Some(tmp);
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
        if self.pokemon_id.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pokemon_id {
            try!(os.write_fixed64(1, v));
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
        ::std::any::TypeId::of::<UpgradePokemonMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UpgradePokemonMessage {
    fn new() -> UpgradePokemonMessage {
        UpgradePokemonMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpgradePokemonMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "pokemon_id",
                    UpgradePokemonMessage::has_pokemon_id,
                    UpgradePokemonMessage::get_pokemon_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpgradePokemonMessage>(
                    "UpgradePokemonMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpgradePokemonMessage {
    fn clear(&mut self) {
        self.clear_pokemon_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UpgradePokemonMessage {
    fn eq(&self, other: &UpgradePokemonMessage) -> bool {
        self.pokemon_id == other.pokemon_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UpgradePokemonMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EchoMessage {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EchoMessage {}

impl EchoMessage {
    pub fn new() -> EchoMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EchoMessage {
        static mut instance: ::protobuf::lazy::Lazy<EchoMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EchoMessage,
        };
        unsafe {
            instance.get(|| {
                EchoMessage {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for EchoMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<EchoMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EchoMessage {
    fn new() -> EchoMessage {
        EchoMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<EchoMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<EchoMessage>(
                    "EchoMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EchoMessage {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EchoMessage {
    fn eq(&self, other: &EchoMessage) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EchoMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EquipBadgeMessage {
    // message fields
    badge_type: ::std::option::Option<super::POGOProtos_Enums::BadgeType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EquipBadgeMessage {}

impl EquipBadgeMessage {
    pub fn new() -> EquipBadgeMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EquipBadgeMessage {
        static mut instance: ::protobuf::lazy::Lazy<EquipBadgeMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EquipBadgeMessage,
        };
        unsafe {
            instance.get(|| {
                EquipBadgeMessage {
                    badge_type: ::std::option::Option::None,
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
}

impl ::protobuf::Message for EquipBadgeMessage {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.badge_type {
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
        ::std::any::TypeId::of::<EquipBadgeMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EquipBadgeMessage {
    fn new() -> EquipBadgeMessage {
        EquipBadgeMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<EquipBadgeMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "badge_type",
                    EquipBadgeMessage::has_badge_type,
                    EquipBadgeMessage::get_badge_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EquipBadgeMessage>(
                    "EquipBadgeMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EquipBadgeMessage {
    fn clear(&mut self) {
        self.clear_badge_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EquipBadgeMessage {
    fn eq(&self, other: &EquipBadgeMessage) -> bool {
        self.badge_type == other.badge_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EquipBadgeMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetPlayerProfileMessage {
    // message fields
    player_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetPlayerProfileMessage {}

impl GetPlayerProfileMessage {
    pub fn new() -> GetPlayerProfileMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetPlayerProfileMessage {
        static mut instance: ::protobuf::lazy::Lazy<GetPlayerProfileMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetPlayerProfileMessage,
        };
        unsafe {
            instance.get(|| {
                GetPlayerProfileMessage {
                    player_name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string player_name = 1;

    pub fn clear_player_name(&mut self) {
        self.player_name.clear();
    }

    pub fn has_player_name(&self) -> bool {
        self.player_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_name(&mut self, v: ::std::string::String) {
        self.player_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_name(&mut self) -> &mut ::std::string::String {
        if self.player_name.is_none() {
            self.player_name.set_default();
        };
        self.player_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_name(&mut self) -> ::std::string::String {
        self.player_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_player_name(&self) -> &str {
        match self.player_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for GetPlayerProfileMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.player_name));
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
        for value in &self.player_name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_name.as_ref() {
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
        ::std::any::TypeId::of::<GetPlayerProfileMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetPlayerProfileMessage {
    fn new() -> GetPlayerProfileMessage {
        GetPlayerProfileMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetPlayerProfileMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "player_name",
                    GetPlayerProfileMessage::has_player_name,
                    GetPlayerProfileMessage::get_player_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetPlayerProfileMessage>(
                    "GetPlayerProfileMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetPlayerProfileMessage {
    fn clear(&mut self) {
        self.clear_player_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetPlayerProfileMessage {
    fn eq(&self, other: &GetPlayerProfileMessage) -> bool {
        self.player_name == other.player_name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetPlayerProfileMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetSuggestedCodenamesMessage {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetSuggestedCodenamesMessage {}

impl GetSuggestedCodenamesMessage {
    pub fn new() -> GetSuggestedCodenamesMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetSuggestedCodenamesMessage {
        static mut instance: ::protobuf::lazy::Lazy<GetSuggestedCodenamesMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetSuggestedCodenamesMessage,
        };
        unsafe {
            instance.get(|| {
                GetSuggestedCodenamesMessage {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for GetSuggestedCodenamesMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<GetSuggestedCodenamesMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetSuggestedCodenamesMessage {
    fn new() -> GetSuggestedCodenamesMessage {
        GetSuggestedCodenamesMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetSuggestedCodenamesMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetSuggestedCodenamesMessage>(
                    "GetSuggestedCodenamesMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetSuggestedCodenamesMessage {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetSuggestedCodenamesMessage {
    fn eq(&self, other: &GetSuggestedCodenamesMessage) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetSuggestedCodenamesMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FortDeployPokemonMessage {
    // message fields
    fort_id: ::protobuf::SingularField<::std::string::String>,
    pokemon_id: ::std::option::Option<u64>,
    player_latitude: ::std::option::Option<f64>,
    player_longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FortDeployPokemonMessage {}

impl FortDeployPokemonMessage {
    pub fn new() -> FortDeployPokemonMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FortDeployPokemonMessage {
        static mut instance: ::protobuf::lazy::Lazy<FortDeployPokemonMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FortDeployPokemonMessage,
        };
        unsafe {
            instance.get(|| {
                FortDeployPokemonMessage {
                    fort_id: ::protobuf::SingularField::none(),
                    pokemon_id: ::std::option::Option::None,
                    player_latitude: ::std::option::Option::None,
                    player_longitude: ::std::option::Option::None,
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

    // optional fixed64 pokemon_id = 2;

    pub fn clear_pokemon_id(&mut self) {
        self.pokemon_id = ::std::option::Option::None;
    }

    pub fn has_pokemon_id(&self) -> bool {
        self.pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_id(&mut self, v: u64) {
        self.pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_id(&self) -> u64 {
        self.pokemon_id.unwrap_or(0)
    }

    // optional double player_latitude = 3;

    pub fn clear_player_latitude(&mut self) {
        self.player_latitude = ::std::option::Option::None;
    }

    pub fn has_player_latitude(&self) -> bool {
        self.player_latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_latitude(&mut self, v: f64) {
        self.player_latitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_latitude(&self) -> f64 {
        self.player_latitude.unwrap_or(0.)
    }

    // optional double player_longitude = 4;

    pub fn clear_player_longitude(&mut self) {
        self.player_longitude = ::std::option::Option::None;
    }

    pub fn has_player_longitude(&self) -> bool {
        self.player_longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_longitude(&mut self, v: f64) {
        self.player_longitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_longitude(&self) -> f64 {
        self.player_longitude.unwrap_or(0.)
    }
}

impl ::protobuf::Message for FortDeployPokemonMessage {
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
                    self.pokemon_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_latitude = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_longitude = ::std::option::Option::Some(tmp);
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
        if self.pokemon_id.is_some() {
            my_size += 9;
        };
        if self.player_latitude.is_some() {
            my_size += 9;
        };
        if self.player_longitude.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fort_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.pokemon_id {
            try!(os.write_fixed64(2, v));
        };
        if let Some(v) = self.player_latitude {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.player_longitude {
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
        ::std::any::TypeId::of::<FortDeployPokemonMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FortDeployPokemonMessage {
    fn new() -> FortDeployPokemonMessage {
        FortDeployPokemonMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<FortDeployPokemonMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "fort_id",
                    FortDeployPokemonMessage::has_fort_id,
                    FortDeployPokemonMessage::get_fort_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "pokemon_id",
                    FortDeployPokemonMessage::has_pokemon_id,
                    FortDeployPokemonMessage::get_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_latitude",
                    FortDeployPokemonMessage::has_player_latitude,
                    FortDeployPokemonMessage::get_player_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_longitude",
                    FortDeployPokemonMessage::has_player_longitude,
                    FortDeployPokemonMessage::get_player_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FortDeployPokemonMessage>(
                    "FortDeployPokemonMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FortDeployPokemonMessage {
    fn clear(&mut self) {
        self.clear_fort_id();
        self.clear_pokemon_id();
        self.clear_player_latitude();
        self.clear_player_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FortDeployPokemonMessage {
    fn eq(&self, other: &FortDeployPokemonMessage) -> bool {
        self.fort_id == other.fort_id &&
        self.pokemon_id == other.pokemon_id &&
        self.player_latitude == other.player_latitude &&
        self.player_longitude == other.player_longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FortDeployPokemonMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CollectDailyBonusMessage {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CollectDailyBonusMessage {}

impl CollectDailyBonusMessage {
    pub fn new() -> CollectDailyBonusMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CollectDailyBonusMessage {
        static mut instance: ::protobuf::lazy::Lazy<CollectDailyBonusMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CollectDailyBonusMessage,
        };
        unsafe {
            instance.get(|| {
                CollectDailyBonusMessage {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CollectDailyBonusMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<CollectDailyBonusMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CollectDailyBonusMessage {
    fn new() -> CollectDailyBonusMessage {
        CollectDailyBonusMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CollectDailyBonusMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CollectDailyBonusMessage>(
                    "CollectDailyBonusMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CollectDailyBonusMessage {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CollectDailyBonusMessage {
    fn eq(&self, other: &CollectDailyBonusMessage) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CollectDailyBonusMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetFavoritePokemonMessage {
    // message fields
    pokemon_id: ::std::option::Option<u64>,
    is_favorite: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetFavoritePokemonMessage {}

impl SetFavoritePokemonMessage {
    pub fn new() -> SetFavoritePokemonMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetFavoritePokemonMessage {
        static mut instance: ::protobuf::lazy::Lazy<SetFavoritePokemonMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetFavoritePokemonMessage,
        };
        unsafe {
            instance.get(|| {
                SetFavoritePokemonMessage {
                    pokemon_id: ::std::option::Option::None,
                    is_favorite: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 pokemon_id = 1;

    pub fn clear_pokemon_id(&mut self) {
        self.pokemon_id = ::std::option::Option::None;
    }

    pub fn has_pokemon_id(&self) -> bool {
        self.pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_id(&mut self, v: u64) {
        self.pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_id(&self) -> u64 {
        self.pokemon_id.unwrap_or(0)
    }

    // optional bool is_favorite = 2;

    pub fn clear_is_favorite(&mut self) {
        self.is_favorite = ::std::option::Option::None;
    }

    pub fn has_is_favorite(&self) -> bool {
        self.is_favorite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_favorite(&mut self, v: bool) {
        self.is_favorite = ::std::option::Option::Some(v);
    }

    pub fn get_is_favorite(&self) -> bool {
        self.is_favorite.unwrap_or(false)
    }
}

impl ::protobuf::Message for SetFavoritePokemonMessage {
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
                    self.pokemon_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_favorite = ::std::option::Option::Some(tmp);
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
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.is_favorite.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pokemon_id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.is_favorite {
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
        ::std::any::TypeId::of::<SetFavoritePokemonMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetFavoritePokemonMessage {
    fn new() -> SetFavoritePokemonMessage {
        SetFavoritePokemonMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetFavoritePokemonMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "pokemon_id",
                    SetFavoritePokemonMessage::has_pokemon_id,
                    SetFavoritePokemonMessage::get_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_favorite",
                    SetFavoritePokemonMessage::has_is_favorite,
                    SetFavoritePokemonMessage::get_is_favorite,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetFavoritePokemonMessage>(
                    "SetFavoritePokemonMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetFavoritePokemonMessage {
    fn clear(&mut self) {
        self.clear_pokemon_id();
        self.clear_is_favorite();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetFavoritePokemonMessage {
    fn eq(&self, other: &SetFavoritePokemonMessage) -> bool {
        self.pokemon_id == other.pokemon_id &&
        self.is_favorite == other.is_favorite &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetFavoritePokemonMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetAssetDigestMessage {
    // message fields
    platform: ::std::option::Option<super::POGOProtos_Enums::Platform>,
    device_manufacturer: ::protobuf::SingularField<::std::string::String>,
    device_model: ::protobuf::SingularField<::std::string::String>,
    locale: ::protobuf::SingularField<::std::string::String>,
    app_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetAssetDigestMessage {}

impl GetAssetDigestMessage {
    pub fn new() -> GetAssetDigestMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetAssetDigestMessage {
        static mut instance: ::protobuf::lazy::Lazy<GetAssetDigestMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetAssetDigestMessage,
        };
        unsafe {
            instance.get(|| {
                GetAssetDigestMessage {
                    platform: ::std::option::Option::None,
                    device_manufacturer: ::protobuf::SingularField::none(),
                    device_model: ::protobuf::SingularField::none(),
                    locale: ::protobuf::SingularField::none(),
                    app_version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Enums.Platform platform = 1;

    pub fn clear_platform(&mut self) {
        self.platform = ::std::option::Option::None;
    }

    pub fn has_platform(&self) -> bool {
        self.platform.is_some()
    }

    // Param is passed by value, moved
    pub fn set_platform(&mut self, v: super::POGOProtos_Enums::Platform) {
        self.platform = ::std::option::Option::Some(v);
    }

    pub fn get_platform(&self) -> super::POGOProtos_Enums::Platform {
        self.platform.unwrap_or(super::POGOProtos_Enums::Platform::UNSET)
    }

    // optional string device_manufacturer = 2;

    pub fn clear_device_manufacturer(&mut self) {
        self.device_manufacturer.clear();
    }

    pub fn has_device_manufacturer(&self) -> bool {
        self.device_manufacturer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_manufacturer(&mut self, v: ::std::string::String) {
        self.device_manufacturer = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_manufacturer(&mut self) -> &mut ::std::string::String {
        if self.device_manufacturer.is_none() {
            self.device_manufacturer.set_default();
        };
        self.device_manufacturer.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_manufacturer(&mut self) -> ::std::string::String {
        self.device_manufacturer.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_device_manufacturer(&self) -> &str {
        match self.device_manufacturer.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string device_model = 3;

    pub fn clear_device_model(&mut self) {
        self.device_model.clear();
    }

    pub fn has_device_model(&self) -> bool {
        self.device_model.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_model(&mut self, v: ::std::string::String) {
        self.device_model = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_model(&mut self) -> &mut ::std::string::String {
        if self.device_model.is_none() {
            self.device_model.set_default();
        };
        self.device_model.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_model(&mut self) -> ::std::string::String {
        self.device_model.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_device_model(&self) -> &str {
        match self.device_model.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string locale = 4;

    pub fn clear_locale(&mut self) {
        self.locale.clear();
    }

    pub fn has_locale(&self) -> bool {
        self.locale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_locale(&mut self, v: ::std::string::String) {
        self.locale = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_locale(&mut self) -> &mut ::std::string::String {
        if self.locale.is_none() {
            self.locale.set_default();
        };
        self.locale.as_mut().unwrap()
    }

    // Take field
    pub fn take_locale(&mut self) -> ::std::string::String {
        self.locale.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_locale(&self) -> &str {
        match self.locale.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint32 app_version = 5;

    pub fn clear_app_version(&mut self) {
        self.app_version = ::std::option::Option::None;
    }

    pub fn has_app_version(&self) -> bool {
        self.app_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_version(&mut self, v: u32) {
        self.app_version = ::std::option::Option::Some(v);
    }

    pub fn get_app_version(&self) -> u32 {
        self.app_version.unwrap_or(0)
    }
}

impl ::protobuf::Message for GetAssetDigestMessage {
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
                    self.platform = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.device_manufacturer));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.device_model));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.locale));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.app_version = ::std::option::Option::Some(tmp);
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
        for value in &self.platform {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.device_manufacturer {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.device_model {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.locale {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.app_version {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.platform {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.device_manufacturer.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.device_model.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.locale.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.app_version {
            try!(os.write_uint32(5, v));
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
        ::std::any::TypeId::of::<GetAssetDigestMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetAssetDigestMessage {
    fn new() -> GetAssetDigestMessage {
        GetAssetDigestMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetAssetDigestMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "platform",
                    GetAssetDigestMessage::has_platform,
                    GetAssetDigestMessage::get_platform,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "device_manufacturer",
                    GetAssetDigestMessage::has_device_manufacturer,
                    GetAssetDigestMessage::get_device_manufacturer,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "device_model",
                    GetAssetDigestMessage::has_device_model,
                    GetAssetDigestMessage::get_device_model,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "locale",
                    GetAssetDigestMessage::has_locale,
                    GetAssetDigestMessage::get_locale,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "app_version",
                    GetAssetDigestMessage::has_app_version,
                    GetAssetDigestMessage::get_app_version,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetAssetDigestMessage>(
                    "GetAssetDigestMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetAssetDigestMessage {
    fn clear(&mut self) {
        self.clear_platform();
        self.clear_device_manufacturer();
        self.clear_device_model();
        self.clear_locale();
        self.clear_app_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetAssetDigestMessage {
    fn eq(&self, other: &GetAssetDigestMessage) -> bool {
        self.platform == other.platform &&
        self.device_manufacturer == other.device_manufacturer &&
        self.device_model == other.device_model &&
        self.locale == other.locale &&
        self.app_version == other.app_version &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetAssetDigestMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DownloadItemTemplatesMessage {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DownloadItemTemplatesMessage {}

impl DownloadItemTemplatesMessage {
    pub fn new() -> DownloadItemTemplatesMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DownloadItemTemplatesMessage {
        static mut instance: ::protobuf::lazy::Lazy<DownloadItemTemplatesMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DownloadItemTemplatesMessage,
        };
        unsafe {
            instance.get(|| {
                DownloadItemTemplatesMessage {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for DownloadItemTemplatesMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<DownloadItemTemplatesMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DownloadItemTemplatesMessage {
    fn new() -> DownloadItemTemplatesMessage {
        DownloadItemTemplatesMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<DownloadItemTemplatesMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DownloadItemTemplatesMessage>(
                    "DownloadItemTemplatesMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DownloadItemTemplatesMessage {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DownloadItemTemplatesMessage {
    fn eq(&self, other: &DownloadItemTemplatesMessage) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DownloadItemTemplatesMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct IncenseEncounterMessage {
    // message fields
    encounter_id: ::std::option::Option<u64>,
    encounter_location: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IncenseEncounterMessage {}

impl IncenseEncounterMessage {
    pub fn new() -> IncenseEncounterMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IncenseEncounterMessage {
        static mut instance: ::protobuf::lazy::Lazy<IncenseEncounterMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IncenseEncounterMessage,
        };
        unsafe {
            instance.get(|| {
                IncenseEncounterMessage {
                    encounter_id: ::std::option::Option::None,
                    encounter_location: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 encounter_id = 1;

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

    // optional string encounter_location = 2;

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
}

impl ::protobuf::Message for IncenseEncounterMessage {
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
                    self.encounter_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.encounter_location));
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
        for value in &self.encounter_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.encounter_location {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.encounter_id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.encounter_location.as_ref() {
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
        ::std::any::TypeId::of::<IncenseEncounterMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IncenseEncounterMessage {
    fn new() -> IncenseEncounterMessage {
        IncenseEncounterMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<IncenseEncounterMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "encounter_id",
                    IncenseEncounterMessage::has_encounter_id,
                    IncenseEncounterMessage::get_encounter_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "encounter_location",
                    IncenseEncounterMessage::has_encounter_location,
                    IncenseEncounterMessage::get_encounter_location,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IncenseEncounterMessage>(
                    "IncenseEncounterMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IncenseEncounterMessage {
    fn clear(&mut self) {
        self.clear_encounter_id();
        self.clear_encounter_location();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IncenseEncounterMessage {
    fn eq(&self, other: &IncenseEncounterMessage) -> bool {
        self.encounter_id == other.encounter_id &&
        self.encounter_location == other.encounter_location &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IncenseEncounterMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CatchPokemonMessage {
    // message fields
    encounter_id: ::std::option::Option<u64>,
    pokeball: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemId>,
    normalized_reticle_size: ::std::option::Option<f64>,
    spawn_point_id: ::protobuf::SingularField<::std::string::String>,
    hit_pokemon: ::std::option::Option<bool>,
    spin_modifier: ::std::option::Option<f64>,
    normalized_hit_position: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CatchPokemonMessage {}

impl CatchPokemonMessage {
    pub fn new() -> CatchPokemonMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CatchPokemonMessage {
        static mut instance: ::protobuf::lazy::Lazy<CatchPokemonMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CatchPokemonMessage,
        };
        unsafe {
            instance.get(|| {
                CatchPokemonMessage {
                    encounter_id: ::std::option::Option::None,
                    pokeball: ::std::option::Option::None,
                    normalized_reticle_size: ::std::option::Option::None,
                    spawn_point_id: ::protobuf::SingularField::none(),
                    hit_pokemon: ::std::option::Option::None,
                    spin_modifier: ::std::option::Option::None,
                    normalized_hit_position: ::std::option::Option::None,
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

    // optional .POGOProtos.Inventory.Item.ItemId pokeball = 2;

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

    // optional double normalized_reticle_size = 3;

    pub fn clear_normalized_reticle_size(&mut self) {
        self.normalized_reticle_size = ::std::option::Option::None;
    }

    pub fn has_normalized_reticle_size(&self) -> bool {
        self.normalized_reticle_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_normalized_reticle_size(&mut self, v: f64) {
        self.normalized_reticle_size = ::std::option::Option::Some(v);
    }

    pub fn get_normalized_reticle_size(&self) -> f64 {
        self.normalized_reticle_size.unwrap_or(0.)
    }

    // optional string spawn_point_id = 4;

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

    // optional bool hit_pokemon = 5;

    pub fn clear_hit_pokemon(&mut self) {
        self.hit_pokemon = ::std::option::Option::None;
    }

    pub fn has_hit_pokemon(&self) -> bool {
        self.hit_pokemon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hit_pokemon(&mut self, v: bool) {
        self.hit_pokemon = ::std::option::Option::Some(v);
    }

    pub fn get_hit_pokemon(&self) -> bool {
        self.hit_pokemon.unwrap_or(false)
    }

    // optional double spin_modifier = 6;

    pub fn clear_spin_modifier(&mut self) {
        self.spin_modifier = ::std::option::Option::None;
    }

    pub fn has_spin_modifier(&self) -> bool {
        self.spin_modifier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spin_modifier(&mut self, v: f64) {
        self.spin_modifier = ::std::option::Option::Some(v);
    }

    pub fn get_spin_modifier(&self) -> f64 {
        self.spin_modifier.unwrap_or(0.)
    }

    // optional double normalized_hit_position = 7;

    pub fn clear_normalized_hit_position(&mut self) {
        self.normalized_hit_position = ::std::option::Option::None;
    }

    pub fn has_normalized_hit_position(&self) -> bool {
        self.normalized_hit_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_normalized_hit_position(&mut self, v: f64) {
        self.normalized_hit_position = ::std::option::Option::Some(v);
    }

    pub fn get_normalized_hit_position(&self) -> f64 {
        self.normalized_hit_position.unwrap_or(0.)
    }
}

impl ::protobuf::Message for CatchPokemonMessage {
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
                    let tmp = try!(is.read_enum());
                    self.pokeball = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.normalized_reticle_size = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.spawn_point_id));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.hit_pokemon = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.spin_modifier = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.normalized_hit_position = ::std::option::Option::Some(tmp);
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
        for value in &self.pokeball {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        if self.normalized_reticle_size.is_some() {
            my_size += 9;
        };
        for value in &self.spawn_point_id {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if self.hit_pokemon.is_some() {
            my_size += 2;
        };
        if self.spin_modifier.is_some() {
            my_size += 9;
        };
        if self.normalized_hit_position.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.encounter_id {
            try!(os.write_fixed64(1, v));
        };
        if let Some(v) = self.pokeball {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.normalized_reticle_size {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.spawn_point_id.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.hit_pokemon {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.spin_modifier {
            try!(os.write_double(6, v));
        };
        if let Some(v) = self.normalized_hit_position {
            try!(os.write_double(7, v));
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
        ::std::any::TypeId::of::<CatchPokemonMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CatchPokemonMessage {
    fn new() -> CatchPokemonMessage {
        CatchPokemonMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CatchPokemonMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "encounter_id",
                    CatchPokemonMessage::has_encounter_id,
                    CatchPokemonMessage::get_encounter_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "pokeball",
                    CatchPokemonMessage::has_pokeball,
                    CatchPokemonMessage::get_pokeball,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "normalized_reticle_size",
                    CatchPokemonMessage::has_normalized_reticle_size,
                    CatchPokemonMessage::get_normalized_reticle_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "spawn_point_id",
                    CatchPokemonMessage::has_spawn_point_id,
                    CatchPokemonMessage::get_spawn_point_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "hit_pokemon",
                    CatchPokemonMessage::has_hit_pokemon,
                    CatchPokemonMessage::get_hit_pokemon,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "spin_modifier",
                    CatchPokemonMessage::has_spin_modifier,
                    CatchPokemonMessage::get_spin_modifier,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "normalized_hit_position",
                    CatchPokemonMessage::has_normalized_hit_position,
                    CatchPokemonMessage::get_normalized_hit_position,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CatchPokemonMessage>(
                    "CatchPokemonMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CatchPokemonMessage {
    fn clear(&mut self) {
        self.clear_encounter_id();
        self.clear_pokeball();
        self.clear_normalized_reticle_size();
        self.clear_spawn_point_id();
        self.clear_hit_pokemon();
        self.clear_spin_modifier();
        self.clear_normalized_hit_position();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CatchPokemonMessage {
    fn eq(&self, other: &CatchPokemonMessage) -> bool {
        self.encounter_id == other.encounter_id &&
        self.pokeball == other.pokeball &&
        self.normalized_reticle_size == other.normalized_reticle_size &&
        self.spawn_point_id == other.spawn_point_id &&
        self.hit_pokemon == other.hit_pokemon &&
        self.spin_modifier == other.spin_modifier &&
        self.normalized_hit_position == other.normalized_hit_position &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CatchPokemonMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FortRecallPokemonMessage {
    // message fields
    fort_id: ::protobuf::SingularField<::std::string::String>,
    pokemon_id: ::std::option::Option<u64>,
    player_latitude: ::std::option::Option<f64>,
    player_longitude: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FortRecallPokemonMessage {}

impl FortRecallPokemonMessage {
    pub fn new() -> FortRecallPokemonMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FortRecallPokemonMessage {
        static mut instance: ::protobuf::lazy::Lazy<FortRecallPokemonMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FortRecallPokemonMessage,
        };
        unsafe {
            instance.get(|| {
                FortRecallPokemonMessage {
                    fort_id: ::protobuf::SingularField::none(),
                    pokemon_id: ::std::option::Option::None,
                    player_latitude: ::std::option::Option::None,
                    player_longitude: ::std::option::Option::None,
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

    // optional fixed64 pokemon_id = 2;

    pub fn clear_pokemon_id(&mut self) {
        self.pokemon_id = ::std::option::Option::None;
    }

    pub fn has_pokemon_id(&self) -> bool {
        self.pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_id(&mut self, v: u64) {
        self.pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_id(&self) -> u64 {
        self.pokemon_id.unwrap_or(0)
    }

    // optional double player_latitude = 3;

    pub fn clear_player_latitude(&mut self) {
        self.player_latitude = ::std::option::Option::None;
    }

    pub fn has_player_latitude(&self) -> bool {
        self.player_latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_latitude(&mut self, v: f64) {
        self.player_latitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_latitude(&self) -> f64 {
        self.player_latitude.unwrap_or(0.)
    }

    // optional double player_longitude = 4;

    pub fn clear_player_longitude(&mut self) {
        self.player_longitude = ::std::option::Option::None;
    }

    pub fn has_player_longitude(&self) -> bool {
        self.player_longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_longitude(&mut self, v: f64) {
        self.player_longitude = ::std::option::Option::Some(v);
    }

    pub fn get_player_longitude(&self) -> f64 {
        self.player_longitude.unwrap_or(0.)
    }
}

impl ::protobuf::Message for FortRecallPokemonMessage {
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
                    self.pokemon_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_latitude = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.player_longitude = ::std::option::Option::Some(tmp);
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
        if self.pokemon_id.is_some() {
            my_size += 9;
        };
        if self.player_latitude.is_some() {
            my_size += 9;
        };
        if self.player_longitude.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fort_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.pokemon_id {
            try!(os.write_fixed64(2, v));
        };
        if let Some(v) = self.player_latitude {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.player_longitude {
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
        ::std::any::TypeId::of::<FortRecallPokemonMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FortRecallPokemonMessage {
    fn new() -> FortRecallPokemonMessage {
        FortRecallPokemonMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<FortRecallPokemonMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "fort_id",
                    FortRecallPokemonMessage::has_fort_id,
                    FortRecallPokemonMessage::get_fort_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "pokemon_id",
                    FortRecallPokemonMessage::has_pokemon_id,
                    FortRecallPokemonMessage::get_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_latitude",
                    FortRecallPokemonMessage::has_player_latitude,
                    FortRecallPokemonMessage::get_player_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "player_longitude",
                    FortRecallPokemonMessage::has_player_longitude,
                    FortRecallPokemonMessage::get_player_longitude,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FortRecallPokemonMessage>(
                    "FortRecallPokemonMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FortRecallPokemonMessage {
    fn clear(&mut self) {
        self.clear_fort_id();
        self.clear_pokemon_id();
        self.clear_player_latitude();
        self.clear_player_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FortRecallPokemonMessage {
    fn eq(&self, other: &FortRecallPokemonMessage) -> bool {
        self.fort_id == other.fort_id &&
        self.pokemon_id == other.pokemon_id &&
        self.player_latitude == other.player_latitude &&
        self.player_longitude == other.player_longitude &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FortRecallPokemonMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CheckAwardedBadgesMessage {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CheckAwardedBadgesMessage {}

impl CheckAwardedBadgesMessage {
    pub fn new() -> CheckAwardedBadgesMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckAwardedBadgesMessage {
        static mut instance: ::protobuf::lazy::Lazy<CheckAwardedBadgesMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckAwardedBadgesMessage,
        };
        unsafe {
            instance.get(|| {
                CheckAwardedBadgesMessage {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CheckAwardedBadgesMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<CheckAwardedBadgesMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CheckAwardedBadgesMessage {
    fn new() -> CheckAwardedBadgesMessage {
        CheckAwardedBadgesMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckAwardedBadgesMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CheckAwardedBadgesMessage>(
                    "CheckAwardedBadgesMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckAwardedBadgesMessage {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CheckAwardedBadgesMessage {
    fn eq(&self, other: &CheckAwardedBadgesMessage) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CheckAwardedBadgesMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x2d, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73,
    0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x27, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77,
    0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x2e,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x1a, 0x16, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x1f, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76,
    0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x1c, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61,
    0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x1c, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61,
    0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x00, 0x50,
    0x01, 0x50, 0x02, 0x50, 0x03, 0x22, 0x4f, 0x0a, 0x13, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x55,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x1a, 0x0a, 0x08,
    0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x52, 0x08,
    0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x6c, 0x6f, 0x6e, 0x67,
    0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x52, 0x09, 0x6c, 0x6f, 0x6e,
    0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x22, 0x2d, 0x0a, 0x17, 0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f,
    0x61, 0x64, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x12, 0x12, 0x0a, 0x04, 0x68, 0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x04, 0x68, 0x61, 0x73, 0x68, 0x22, 0xe9, 0x01, 0x0a, 0x22, 0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f,
    0x61, 0x64, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x36, 0x0a, 0x08,
    0x70, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1a,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d,
    0x73, 0x2e, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x52, 0x08, 0x70, 0x6c, 0x61, 0x74,
    0x66, 0x6f, 0x72, 0x6d, 0x12, 0x2f, 0x0a, 0x13, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x6d,
    0x61, 0x6e, 0x75, 0x66, 0x61, 0x63, 0x74, 0x75, 0x72, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x12, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x4d, 0x61, 0x6e, 0x75, 0x66, 0x61, 0x63,
    0x74, 0x75, 0x72, 0x65, 0x72, 0x12, 0x21, 0x0a, 0x0c, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f,
    0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x76,
    0x69, 0x63, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x6c, 0x12, 0x16, 0x0a, 0x06, 0x6c, 0x6f, 0x63, 0x61,
    0x6c, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x65,
    0x12, 0x1f, 0x0a, 0x0b, 0x61, 0x70, 0x70, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x61, 0x70, 0x70, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x22, 0x71, 0x0a, 0x14, 0x55, 0x73, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x50, 0x6f, 0x74, 0x69,
    0x6f, 0x6e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x3a, 0x0a, 0x07, 0x69, 0x74, 0x65,
    0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72,
    0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x52, 0x06, 0x69,
    0x74, 0x65, 0x6d, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e,
    0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09, 0x70, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x49, 0x64, 0x22, 0x6f, 0x0a, 0x19, 0x53, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x61,
    0x63, 0x74, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x12, 0x52, 0x0a, 0x10, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x5f, 0x73, 0x65, 0x74,
    0x74, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x53, 0x65, 0x74, 0x74,
    0x69, 0x6e, 0x67, 0x73, 0x52, 0x0f, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x53, 0x65, 0x74,
    0x74, 0x69, 0x6e, 0x67, 0x73, 0x22, 0x2d, 0x0a, 0x15, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x55, 0x70,
    0x52, 0x65, 0x77, 0x61, 0x72, 0x64, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x14,
    0x0a, 0x05, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x05, 0x6c,
    0x65, 0x76, 0x65, 0x6c, 0x22, 0x67, 0x0a, 0x12, 0x46, 0x6f, 0x72, 0x74, 0x44, 0x65, 0x74, 0x61,
    0x69, 0x6c, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x17, 0x0a, 0x07, 0x66, 0x6f,
    0x72, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x66, 0x6f, 0x72,
    0x74, 0x49, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x01, 0x52, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12,
    0x1c, 0x0a, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x01, 0x52, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x22, 0x12, 0x0a,
    0x10, 0x47, 0x65, 0x74, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x22, 0xc9, 0x01, 0x0a, 0x14, 0x47, 0x65, 0x74, 0x47, 0x79, 0x6d, 0x44, 0x65, 0x74, 0x61,
    0x69, 0x6c, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x15, 0x0a, 0x06, 0x67, 0x79,
    0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x67, 0x79, 0x6d, 0x49,
    0x64, 0x12, 0x27, 0x0a, 0x0f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x61, 0x74, 0x69,
    0x74, 0x75, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0e, 0x70, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x4c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x29, 0x0a, 0x10, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x01, 0x52, 0x0f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x6f, 0x6e, 0x67,
    0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x67, 0x79, 0x6d, 0x5f, 0x6c, 0x61, 0x74,
    0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0b, 0x67, 0x79, 0x6d,
    0x4c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x23, 0x0a, 0x0d, 0x67, 0x79, 0x6d, 0x5f,
    0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x01, 0x52,
    0x0c, 0x67, 0x79, 0x6d, 0x4c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x22, 0x17, 0x0a,
    0x15, 0x53, 0x66, 0x69, 0x64, 0x61, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x4c, 0x6f, 0x67, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x33, 0x0a, 0x16, 0x47, 0x65, 0x74, 0x44, 0x6f, 0x77,
    0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x55, 0x72, 0x6c, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x12, 0x19, 0x0a, 0x08, 0x61, 0x73, 0x73, 0x65, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x09, 0x52, 0x07, 0x61, 0x73, 0x73, 0x65, 0x74, 0x49, 0x64, 0x22, 0xc3, 0x02, 0x0a, 0x10,
    0x41, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x47, 0x79, 0x6d, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x12, 0x15, 0x0a, 0x06, 0x67, 0x79, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x05, 0x67, 0x79, 0x6d, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x62, 0x61, 0x74, 0x74, 0x6c,
    0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x62, 0x61, 0x74, 0x74,
    0x6c, 0x65, 0x49, 0x64, 0x12, 0x4b, 0x0a, 0x0e, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x5f, 0x61,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x42,
    0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x41, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x52, 0x0d, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x12, 0x5a, 0x0a, 0x16, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x72, 0x65, 0x74, 0x72, 0x69, 0x65,
    0x76, 0x65, 0x64, 0x5f, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x24, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44,
    0x61, 0x74, 0x61, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c,
    0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x14, 0x6c, 0x61, 0x73, 0x74, 0x52, 0x65, 0x74,
    0x72, 0x69, 0x65, 0x76, 0x65, 0x64, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x27, 0x0a,
    0x0f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0e, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x61,
    0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x29, 0x0a, 0x10, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72,
    0x5f, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x01,
    0x52, 0x0f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64,
    0x65, 0x22, 0x9c, 0x01, 0x0a, 0x15, 0x55, 0x73, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x43, 0x61, 0x70,
    0x74, 0x75, 0x72, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x3a, 0x0a, 0x07, 0x69,
    0x74, 0x65, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x21, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74,
    0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x52,
    0x06, 0x69, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x65, 0x6e, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0b, 0x65,
    0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x49, 0x64, 0x12, 0x24, 0x0a, 0x0e, 0x73, 0x70,
    0x61, 0x77, 0x6e, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0c, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x49, 0x64,
    0x22, 0x53, 0x0a, 0x16, 0x4e, 0x69, 0x63, 0x6b, 0x6e, 0x61, 0x6d, 0x65, 0x50, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09,
    0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x6e, 0x69, 0x63,
    0x6b, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x6e, 0x69, 0x63,
    0x6b, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x6f, 0x0a, 0x1b, 0x52, 0x65, 0x63, 0x79, 0x63, 0x6c, 0x65,
    0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x12, 0x3a, 0x0a, 0x07, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65,
    0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x52, 0x06, 0x69, 0x74, 0x65, 0x6d, 0x49, 0x64,
    0x12, 0x14, 0x0a, 0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x22, 0xba, 0x01, 0x0a, 0x11, 0x55, 0x73, 0x65, 0x49, 0x74,
    0x65, 0x6d, 0x47, 0x79, 0x6d, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x3a, 0x0a, 0x07,
    0x69, 0x74, 0x65, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x21, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e,
    0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x49, 0x64,
    0x52, 0x06, 0x69, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x12, 0x15, 0x0a, 0x06, 0x67, 0x79, 0x6d, 0x5f,
    0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x67, 0x79, 0x6d, 0x49, 0x64, 0x12,
    0x27, 0x0a, 0x0f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75,
    0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0e, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72,
    0x4c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x29, 0x0a, 0x10, 0x70, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x5f, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x01, 0x52, 0x0f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x6f, 0x6e, 0x67, 0x69, 0x74,
    0x75, 0x64, 0x65, 0x22, 0x71, 0x0a, 0x14, 0x55, 0x73, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x52, 0x65,
    0x76, 0x69, 0x76, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x3a, 0x0a, 0x07, 0x69,
    0x74, 0x65, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x21, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74,
    0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x52,
    0x06, 0x69, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09, 0x70, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x22, 0x59, 0x0a, 0x11, 0x55, 0x73, 0x65, 0x49, 0x6e, 0x63,
    0x65, 0x6e, 0x73, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x44, 0x0a, 0x0c, 0x69,
    0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49,
    0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74,
    0x65, 0x6d, 0x49, 0x64, 0x52, 0x0b, 0x69, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x54, 0x79, 0x70,
    0x65, 0x22, 0x47, 0x0a, 0x14, 0x53, 0x65, 0x74, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x54, 0x65,
    0x61, 0x6d, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x2f, 0x0a, 0x04, 0x74, 0x65, 0x61,
    0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x54, 0x65, 0x61, 0x6d, 0x43,
    0x6f, 0x6c, 0x6f, 0x72, 0x52, 0x04, 0x74, 0x65, 0x61, 0x6d, 0x22, 0x22, 0x0a, 0x20, 0x43, 0x6f,
    0x6c, 0x6c, 0x65, 0x63, 0x74, 0x44, 0x61, 0x69, 0x6c, 0x79, 0x44, 0x65, 0x66, 0x65, 0x6e, 0x64,
    0x65, 0x72, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0xa6,
    0x01, 0x0a, 0x14, 0x44, 0x69, 0x73, 0x6b, 0x45, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x65, 0x6e, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x65,
    0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x49, 0x64, 0x12, 0x17, 0x0a, 0x07, 0x66, 0x6f,
    0x72, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x66, 0x6f, 0x72,
    0x74, 0x49, 0x64, 0x12, 0x27, 0x0a, 0x0f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x61,
    0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0e, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x4c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x29, 0x0a, 0x10,
    0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x6f,
    0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x22, 0x53, 0x0a, 0x15, 0x55, 0x73, 0x65, 0x49, 0x74,
    0x65, 0x6d, 0x58, 0x70, 0x42, 0x6f, 0x6f, 0x73, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x12, 0x3a, 0x0a, 0x07, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49,
    0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74,
    0x65, 0x6d, 0x49, 0x64, 0x52, 0x06, 0x69, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x22, 0x35, 0x0a, 0x14,
    0x45, 0x76, 0x6f, 0x6c, 0x76, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f,
    0x6e, 0x49, 0x64, 0x22, 0x9f, 0x01, 0x0a, 0x14, 0x47, 0x65, 0x74, 0x4d, 0x61, 0x70, 0x4f, 0x62,
    0x6a, 0x65, 0x63, 0x74, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x1b, 0x0a, 0x07,
    0x63, 0x65, 0x6c, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x04, 0x42, 0x02, 0x10,
    0x01, 0x52, 0x06, 0x63, 0x65, 0x6c, 0x6c, 0x49, 0x64, 0x12, 0x30, 0x0a, 0x12, 0x73, 0x69, 0x6e,
    0x63, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x03, 0x42, 0x02, 0x10, 0x01, 0x52, 0x10, 0x73, 0x69, 0x6e, 0x63, 0x65,
    0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x6c,
    0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x08, 0x6c,
    0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69,
    0x74, 0x75, 0x64, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x52, 0x09, 0x6c, 0x6f, 0x6e, 0x67,
    0x69, 0x74, 0x75, 0x64, 0x65, 0x22, 0xe8, 0x01, 0x0a, 0x15, 0x53, 0x74, 0x61, 0x72, 0x74, 0x47,
    0x79, 0x6d, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12,
    0x15, 0x0a, 0x06, 0x67, 0x79, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x05, 0x67, 0x79, 0x6d, 0x49, 0x64, 0x12, 0x32, 0x0a, 0x15, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b,
    0x69, 0x6e, 0x67, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x73, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x06, 0x52, 0x13, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x69, 0x6e, 0x67,
    0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x73, 0x12, 0x30, 0x0a, 0x14, 0x64, 0x65,
    0x66, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f,
    0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x06, 0x52, 0x12, 0x64, 0x65, 0x66, 0x65, 0x6e, 0x64,
    0x69, 0x6e, 0x67, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x27, 0x0a, 0x0f,
    0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0e, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x61, 0x74,
    0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x29, 0x0a, 0x10, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f,
    0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x01, 0x52,
    0x0f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65,
    0x22, 0xaf, 0x01, 0x0a, 0x10, 0x45, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0b, 0x65, 0x6e, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x49, 0x64, 0x12, 0x24, 0x0a, 0x0e, 0x73, 0x70, 0x61, 0x77,
    0x6e, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x0c, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x27,
    0x0a, 0x0f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64,
    0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0e, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c,
    0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x29, 0x0a, 0x10, 0x70, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x5f, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x01, 0x52, 0x0f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75,
    0x64, 0x65, 0x22, 0x36, 0x0a, 0x15, 0x52, 0x65, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x50, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x70,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52,
    0x09, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x22, 0x5e, 0x0a, 0x20, 0x45, 0x6e,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x54, 0x75, 0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c, 0x43,
    0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x3a,
    0x0a, 0x0a, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x52,
    0x09, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x22, 0x32, 0x0a, 0x14, 0x43, 0x6c,
    0x61, 0x69, 0x6d, 0x43, 0x6f, 0x64, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x63, 0x6f, 0x64, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63, 0x6f, 0x64, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x6e,
    0x0a, 0x18, 0x47, 0x65, 0x74, 0x49, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x50, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x27, 0x0a, 0x0f, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x01, 0x52, 0x0e, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x61, 0x74, 0x69, 0x74,
    0x75, 0x64, 0x65, 0x12, 0x29, 0x0a, 0x10, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x6f,
    0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0f, 0x70,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x22, 0xdb,
    0x01, 0x0a, 0x1b, 0x4d, 0x61, 0x72, 0x6b, 0x54, 0x75, 0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c, 0x43,
    0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x50,
    0x0a, 0x13, 0x74, 0x75, 0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c, 0x73, 0x5f, 0x63, 0x6f, 0x6d, 0x70,
    0x6c, 0x65, 0x74, 0x65, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x1f, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x54,
    0x75, 0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c, 0x53, 0x74, 0x61, 0x74, 0x65, 0x52, 0x12, 0x74, 0x75,
    0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c, 0x73, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64,
    0x12, 0x32, 0x0a, 0x15, 0x73, 0x65, 0x6e, 0x64, 0x5f, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x69,
    0x6e, 0x67, 0x5f, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x13, 0x73, 0x65, 0x6e, 0x64, 0x4d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x69, 0x6e, 0x67, 0x45, 0x6d,
    0x61, 0x69, 0x6c, 0x73, 0x12, 0x36, 0x0a, 0x17, 0x73, 0x65, 0x6e, 0x64, 0x5f, 0x70, 0x75, 0x73,
    0x68, 0x5f, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x15, 0x73, 0x65, 0x6e, 0x64, 0x50, 0x75, 0x73, 0x68, 0x4e,
    0x6f, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x22, 0xcc, 0x01, 0x0a,
    0x11, 0x46, 0x6f, 0x72, 0x74, 0x53, 0x65, 0x61, 0x72, 0x63, 0x68, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x17, 0x0a, 0x07, 0x66, 0x6f, 0x72, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x06, 0x66, 0x6f, 0x72, 0x74, 0x49, 0x64, 0x12, 0x27, 0x0a, 0x0f, 0x70,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x01, 0x52, 0x0e, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x61, 0x74, 0x69,
    0x74, 0x75, 0x64, 0x65, 0x12, 0x29, 0x0a, 0x10, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c,
    0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0f,
    0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12,
    0x23, 0x0a, 0x0d, 0x66, 0x6f, 0x72, 0x74, 0x5f, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0c, 0x66, 0x6f, 0x72, 0x74, 0x4c, 0x61, 0x74, 0x69,
    0x74, 0x75, 0x64, 0x65, 0x12, 0x25, 0x0a, 0x0e, 0x66, 0x6f, 0x72, 0x74, 0x5f, 0x6c, 0x6f, 0x6e,
    0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0d, 0x66, 0x6f,
    0x72, 0x74, 0x4c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x22, 0xcd, 0x01, 0x0a, 0x16,
    0x41, 0x64, 0x64, 0x46, 0x6f, 0x72, 0x74, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x46, 0x0a, 0x0d, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69,
    0x65, 0x72, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x21, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e,
    0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x49, 0x64,
    0x52, 0x0c, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x54, 0x79, 0x70, 0x65, 0x12, 0x17,
    0x0a, 0x07, 0x66, 0x6f, 0x72, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x06, 0x66, 0x6f, 0x72, 0x74, 0x49, 0x64, 0x12, 0x27, 0x0a, 0x0f, 0x70, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x5f, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01,
    0x52, 0x0e, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65,
    0x12, 0x29, 0x0a, 0x10, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x6f, 0x6e, 0x67, 0x69,
    0x74, 0x75, 0x64, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0f, 0x70, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x4c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x22, 0x17, 0x0a, 0x15, 0x47,
    0x65, 0x74, 0x48, 0x61, 0x74, 0x63, 0x68, 0x65, 0x64, 0x45, 0x67, 0x67, 0x73, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x22, 0x5d, 0x0a, 0x10, 0x53, 0x65, 0x74, 0x41, 0x76, 0x61, 0x74, 0x61,
    0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x49, 0x0a, 0x0d, 0x70, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x5f, 0x61, 0x76, 0x61, 0x74, 0x61, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x24, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x41,
    0x76, 0x61, 0x74, 0x61, 0x72, 0x52, 0x0c, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x41, 0x76, 0x61,
    0x74, 0x61, 0x72, 0x22, 0x54, 0x0a, 0x1a, 0x55, 0x73, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x45, 0x67,
    0x67, 0x49, 0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x12, 0x17, 0x0a, 0x07, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x06, 0x69, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09,
    0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x22, 0x3b, 0x0a, 0x1d, 0x43, 0x68, 0x65,
    0x63, 0x6b, 0x43, 0x6f, 0x64, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x41, 0x76, 0x61, 0x69, 0x6c, 0x61,
    0x62, 0x6c, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x63, 0x6f,
    0x64, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63, 0x6f,
    0x64, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x67, 0x0a, 0x13, 0x47, 0x65, 0x74, 0x49, 0x6e, 0x76,
    0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x2a, 0x0a,
    0x11, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f,
    0x6d, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0f, 0x6c, 0x61, 0x73, 0x74, 0x54, 0x69,
    0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12, 0x24, 0x0a, 0x0e, 0x69, 0x74, 0x65,
    0x6d, 0x5f, 0x62, 0x65, 0x65, 0x6e, 0x5f, 0x73, 0x65, 0x65, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x0c, 0x69, 0x74, 0x65, 0x6d, 0x42, 0x65, 0x65, 0x6e, 0x53, 0x65, 0x65, 0x6e, 0x22,
    0x36, 0x0a, 0x15, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f,
    0x6e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09, 0x70, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x22, 0x0d, 0x0a, 0x0b, 0x45, 0x63, 0x68, 0x6f, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x4f, 0x0a, 0x11, 0x45, 0x71, 0x75, 0x69, 0x70, 0x42,
    0x61, 0x64, 0x67, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x3a, 0x0a, 0x0a, 0x62,
    0x61, 0x64, 0x67, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75,
    0x6d, 0x73, 0x2e, 0x42, 0x61, 0x64, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x09, 0x62, 0x61,
    0x64, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x22, 0x3a, 0x0a, 0x17, 0x47, 0x65, 0x74, 0x50, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4e,
    0x61, 0x6d, 0x65, 0x22, 0x1e, 0x0a, 0x1c, 0x47, 0x65, 0x74, 0x53, 0x75, 0x67, 0x67, 0x65, 0x73,
    0x74, 0x65, 0x64, 0x43, 0x6f, 0x64, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x22, 0xa6, 0x01, 0x0a, 0x18, 0x46, 0x6f, 0x72, 0x74, 0x44, 0x65, 0x70, 0x6c,
    0x6f, 0x79, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x12, 0x17, 0x0a, 0x07, 0x66, 0x6f, 0x72, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x06, 0x66, 0x6f, 0x72, 0x74, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09, 0x70,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x27, 0x0a, 0x0f, 0x70, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x5f, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x01, 0x52, 0x0e, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64,
    0x65, 0x12, 0x29, 0x0a, 0x10, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x6f, 0x6e, 0x67,
    0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0f, 0x70, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x4c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x22, 0x1a, 0x0a, 0x18,
    0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x44, 0x61, 0x69, 0x6c, 0x79, 0x42, 0x6f, 0x6e, 0x75,
    0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x5b, 0x0a, 0x19, 0x53, 0x65, 0x74, 0x46,
    0x61, 0x76, 0x6f, 0x72, 0x69, 0x74, 0x65, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x70, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x69, 0x73, 0x5f, 0x66, 0x61, 0x76, 0x6f, 0x72,
    0x69, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x69, 0x73, 0x46, 0x61, 0x76,
    0x6f, 0x72, 0x69, 0x74, 0x65, 0x22, 0xdc, 0x01, 0x0a, 0x15, 0x47, 0x65, 0x74, 0x41, 0x73, 0x73,
    0x65, 0x74, 0x44, 0x69, 0x67, 0x65, 0x73, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12,
    0x36, 0x0a, 0x08, 0x70, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x1a, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45,
    0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x52, 0x08, 0x70,
    0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x12, 0x2f, 0x0a, 0x13, 0x64, 0x65, 0x76, 0x69, 0x63,
    0x65, 0x5f, 0x6d, 0x61, 0x6e, 0x75, 0x66, 0x61, 0x63, 0x74, 0x75, 0x72, 0x65, 0x72, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x12, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x4d, 0x61, 0x6e, 0x75,
    0x66, 0x61, 0x63, 0x74, 0x75, 0x72, 0x65, 0x72, 0x12, 0x21, 0x0a, 0x0c, 0x64, 0x65, 0x76, 0x69,
    0x63, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b,
    0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x6c, 0x12, 0x16, 0x0a, 0x06, 0x6c,
    0x6f, 0x63, 0x61, 0x6c, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6c, 0x6f, 0x63,
    0x61, 0x6c, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x61, 0x70, 0x70, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x61, 0x70, 0x70, 0x56, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x22, 0x1e, 0x0a, 0x1c, 0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64,
    0x49, 0x74, 0x65, 0x6d, 0x54, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x22, 0x6b, 0x0a, 0x17, 0x49, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x45,
    0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12,
    0x21, 0x0a, 0x0c, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72,
    0x49, 0x64, 0x12, 0x2d, 0x0a, 0x12, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x5f,
    0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x11,
    0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x22, 0xd3, 0x02, 0x0a, 0x13, 0x43, 0x61, 0x74, 0x63, 0x68, 0x50, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x65, 0x6e, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52,
    0x0b, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x49, 0x64, 0x12, 0x3d, 0x0a, 0x08,
    0x70, 0x6f, 0x6b, 0x65, 0x62, 0x61, 0x6c, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x21,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65,
    0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x49,
    0x64, 0x52, 0x08, 0x70, 0x6f, 0x6b, 0x65, 0x62, 0x61, 0x6c, 0x6c, 0x12, 0x36, 0x0a, 0x17, 0x6e,
    0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x5f, 0x72, 0x65, 0x74, 0x69, 0x63, 0x6c,
    0x65, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x15, 0x6e, 0x6f,
    0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x52, 0x65, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x53,
    0x69, 0x7a, 0x65, 0x12, 0x24, 0x0a, 0x0e, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x5f, 0x70, 0x6f, 0x69,
    0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x73, 0x70, 0x61,
    0x77, 0x6e, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x68, 0x69, 0x74,
    0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a,
    0x68, 0x69, 0x74, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x70,
    0x69, 0x6e, 0x5f, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x01, 0x52, 0x0c, 0x73, 0x70, 0x69, 0x6e, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12,
    0x36, 0x0a, 0x17, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x5f, 0x68, 0x69,
    0x74, 0x5f, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x07, 0x20, 0x01, 0x28, 0x01,
    0x52, 0x15, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x48, 0x69, 0x74, 0x50,
    0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0xa6, 0x01, 0x0a, 0x18, 0x46, 0x6f, 0x72, 0x74,
    0x52, 0x65, 0x63, 0x61, 0x6c, 0x6c, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x12, 0x17, 0x0a, 0x07, 0x66, 0x6f, 0x72, 0x74, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x66, 0x6f, 0x72, 0x74, 0x49, 0x64, 0x12, 0x1d, 0x0a,
    0x0a, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x06, 0x52, 0x09, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x27, 0x0a, 0x0f,
    0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0e, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x61, 0x74,
    0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x29, 0x0a, 0x10, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f,
    0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x52,
    0x0f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65,
    0x22, 0x1b, 0x0a, 0x19, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x41, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64,
    0x42, 0x61, 0x64, 0x67, 0x65, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x4a, 0xa1, 0x4e,
    0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xe7, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x2f, 0x0a, 0x09, 0x0a,
    0x02, 0x0a, 0x00, 0x12, 0x03, 0x03, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x03, 0x0e, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x04, 0x07, 0x0d, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x0e, 0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x02, 0x12,
    0x03, 0x05, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x05, 0x0e, 0x2c, 0x0a,
    0x09, 0x0a, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x06, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x06, 0x0e, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x0b,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x09, 0x08, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x09, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x09, 0x0f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x09, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0a, 0x08, 0x09, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a, 0x08, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x0f, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x0c, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x08,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x08, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0d, 0x08, 0x0c, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x0d, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0f,
    0x00, 0x15, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x2a, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x10, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x10, 0x08, 0x0f, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x10, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x10, 0x23, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x10, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x11,
    0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x11, 0x08, 0x10,
    0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x11, 0x08, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x11, 0x0f, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x11, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x12, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x04, 0x12, 0x04, 0x12, 0x08, 0x11, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x12, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x12, 0x0f, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x12, 0x1e,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x13, 0x08, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x04, 0x13, 0x08, 0x12, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x13, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x13, 0x0f, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x13, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x14, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x04, 0x14,
    0x08, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x14, 0x08,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x14, 0x0f, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x14, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x16, 0x00, 0x19, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x16, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x17,
    0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x17, 0x08, 0x16,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x17, 0x08, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17, 0x2a, 0x31, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x17, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x01, 0x12, 0x03, 0x18, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x18, 0x08, 0x17, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x18, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x18, 0x10, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x18, 0x1d,
    0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1a, 0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x00, 0x12, 0x03, 0x1b, 0x08, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x1b, 0x08, 0x1a, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x1b, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x30,
    0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x43, 0x44, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x1d, 0x00, 0x1f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x05, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12,
    0x03, 0x1e, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x1e,
    0x08, 0x1d, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1e, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x0e, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x16, 0x17, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x06, 0x12, 0x04, 0x20, 0x00, 0x24, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01,
    0x12, 0x03, 0x20, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x21,
    0x08, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04, 0x21, 0x08, 0x20,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x21, 0x08, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x0f, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x01, 0x12, 0x03, 0x22, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x22, 0x08, 0x21, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x22, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x22, 0x0f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x22, 0x1a,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x23, 0x08, 0x1d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x04, 0x23, 0x08, 0x22, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x03, 0x23, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x23, 0x0f, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x23, 0x1b, 0x1c, 0x0a, 0x20, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x25,
    0x00, 0x27, 0x01, 0x22, 0x14, 0x20, 0x4e, 0x6f, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x20, 0x6e, 0x65, 0x65, 0x64, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01,
    0x12, 0x03, 0x25, 0x08, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x28, 0x00, 0x2e,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x28, 0x08, 0x1c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x29, 0x08, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x29, 0x08, 0x28, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x29, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x29, 0x0f, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x29, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x04, 0x2a, 0x08, 0x29, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2a, 0x08, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x0f, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02,
    0x02, 0x12, 0x03, 0x2b, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x2b, 0x08, 0x2a, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x2b, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x0f,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2b, 0x22, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x03, 0x2c, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x04, 0x2c, 0x08, 0x2b, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x03, 0x05, 0x12, 0x03, 0x2c, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x2c, 0x0f, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x2c, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12, 0x03, 0x2d,
    0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2d, 0x08, 0x2c,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x05, 0x12, 0x03, 0x2d, 0x08, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2d, 0x0f, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2d, 0x1f, 0x20, 0x0a, 0x20, 0x0a, 0x02, 0x04,
    0x09, 0x12, 0x04, 0x2f, 0x00, 0x31, 0x01, 0x22, 0x14, 0x20, 0x4e, 0x6f, 0x20, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12,
    0x04, 0x32, 0x00, 0x34, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x32, 0x08,
    0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x33, 0x08, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x33, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x33, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x33, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x33, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x35, 0x00,
    0x3c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x35, 0x08, 0x18, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x36, 0x08, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x36, 0x08, 0x35, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x36, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x36, 0x0f, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x36, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x37, 0x08,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x04, 0x37, 0x08, 0x36, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x03, 0x37, 0x08, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x37, 0x0f, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x37, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x02, 0x12, 0x03, 0x38, 0x08, 0x49, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x38, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x38, 0x11, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12, 0x03, 0x38, 0x36,
    0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x03, 0x38, 0x47, 0x48, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x03, 0x12, 0x03, 0x39, 0x08, 0x48, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x03, 0x04, 0x12, 0x04, 0x39, 0x08, 0x38, 0x49, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x03, 0x06, 0x12, 0x03, 0x39, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x39, 0x2d, 0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x39, 0x46, 0x47, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x04, 0x12, 0x03, 0x3a,
    0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x04, 0x12, 0x04, 0x3a, 0x08, 0x39,
    0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x05, 0x12, 0x03, 0x3a, 0x08, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x01, 0x12, 0x03, 0x3a, 0x0f, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x04, 0x03, 0x12, 0x03, 0x3a, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x05, 0x12, 0x03, 0x3b, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05,
    0x04, 0x12, 0x04, 0x3b, 0x08, 0x3a, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x3b, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x3b, 0x0f, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x03, 0x12, 0x03, 0x3b, 0x22,
    0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x3d, 0x00, 0x41, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02,
    0x00, 0x12, 0x03, 0x3e, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x3e, 0x08, 0x3d, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x3e, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x2a,
    0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3e, 0x34, 0x35, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x04, 0x3f, 0x08, 0x3e, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3f, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x3f, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x3f, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x03, 0x40,
    0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x04, 0x40, 0x08, 0x3f,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x03, 0x40, 0x08, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x03, 0x40, 0x0f, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x03, 0x40, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x0d, 0x12, 0x04, 0x42, 0x00, 0x45, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x03,
    0x42, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x43, 0x08, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04, 0x43, 0x08, 0x42, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x03, 0x43, 0x08, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x10, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x43, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02,
    0x01, 0x12, 0x03, 0x44, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x44, 0x08, 0x43, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x44, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x03, 0x44, 0x0f,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x03, 0x44, 0x1a, 0x1b, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x04, 0x46, 0x00, 0x49, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x0e, 0x01, 0x12, 0x03, 0x46, 0x08, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12,
    0x03, 0x47, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0x47,
    0x08, 0x46, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x06, 0x12, 0x03, 0x47, 0x08,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x47, 0x2a, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x03, 0x47, 0x34, 0x35, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x03, 0x48, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x48, 0x08, 0x47, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x48, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x48, 0x0e, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x48, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x04, 0x4a, 0x00, 0x4f, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x03, 0x4a, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x00, 0x12, 0x03, 0x4b, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x4b, 0x08, 0x4a, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x4b, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x4b, 0x2a, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4b, 0x34,
    0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x03, 0x4c, 0x08, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x04, 0x4c, 0x08, 0x4b, 0x36, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4c, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4c, 0x0f, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x4c, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x02, 0x12,
    0x03, 0x4d, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x04, 0x12, 0x04, 0x4d,
    0x08, 0x4c, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4d, 0x08,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4d, 0x0f, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4d, 0x21, 0x22, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0f, 0x02, 0x03, 0x12, 0x03, 0x4e, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x03, 0x04, 0x12, 0x04, 0x4e, 0x08, 0x4d, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x4e, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x4e, 0x0f, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x4e, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x04, 0x50, 0x00, 0x53, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x03, 0x50, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x10, 0x02, 0x00, 0x12, 0x03, 0x51, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x51, 0x08, 0x50, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x51, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x51, 0x2a, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x03, 0x51, 0x34,
    0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x03, 0x52, 0x08, 0x1f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x04, 0x52, 0x08, 0x51, 0x36, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x03, 0x52, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x01, 0x01, 0x12, 0x03, 0x52, 0x10, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x52, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x04, 0x54,
    0x00, 0x56, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x03, 0x54, 0x08, 0x19, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x03, 0x55, 0x08, 0x3b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0x55, 0x08, 0x54, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x00, 0x06, 0x12, 0x03, 0x55, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x55, 0x2a, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x55, 0x39, 0x3a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x04, 0x57, 0x00, 0x59,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x03, 0x57, 0x08, 0x1c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x03, 0x58, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x58, 0x08, 0x57, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x58, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x58, 0x24, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x58, 0x2b, 0x2c, 0x0a, 0x20, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x04, 0x5a, 0x00, 0x5c, 0x01, 0x22,
    0x14, 0x20, 0x4e, 0x6f, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x6e, 0x65, 0x65,
    0x64, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x03, 0x5a, 0x08,
    0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x04, 0x5d, 0x00, 0x62, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x14, 0x01, 0x12, 0x03, 0x5d, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x14, 0x02,
    0x00, 0x12, 0x03, 0x5e, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x5e, 0x08, 0x5d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x5e, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5e, 0x0f,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5e, 0x1e, 0x1f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x03, 0x5f, 0x08, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x04, 0x5f, 0x08, 0x5e, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5f, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x5f, 0x0f, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x5f, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x02, 0x12, 0x03, 0x60,
    0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x04, 0x12, 0x04, 0x60, 0x08, 0x5f,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x05, 0x12, 0x03, 0x60, 0x08, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x01, 0x12, 0x03, 0x60, 0x0f, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x02, 0x03, 0x12, 0x03, 0x60, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x14, 0x02, 0x03, 0x12, 0x03, 0x61, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03,
    0x04, 0x12, 0x04, 0x61, 0x08, 0x60, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x61, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x61, 0x0f, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x03, 0x12, 0x03, 0x61, 0x22,
    0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x04, 0x63, 0x00, 0x65, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x15, 0x01, 0x12, 0x03, 0x63, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x15, 0x02,
    0x00, 0x12, 0x03, 0x64, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x64, 0x08, 0x63, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x64, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x03, 0x64, 0x2a,
    0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x03, 0x64, 0x34, 0x35, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x04, 0x66, 0x00, 0x68, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x16, 0x01, 0x12, 0x03, 0x66, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12,
    0x03, 0x67, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x04, 0x67,
    0x08, 0x66, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x05, 0x12, 0x03, 0x67, 0x08,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x03, 0x67, 0x10, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x03, 0x67, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x17, 0x12, 0x04, 0x69, 0x00, 0x6e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x17, 0x01,
    0x12, 0x03, 0x69, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x03, 0x6a,
    0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6a, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6a, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6a, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6a, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x00, 0x08, 0x12, 0x03, 0x6a, 0x24, 0x31, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x17, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x6a, 0x25, 0x30, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x17, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x6a, 0x25, 0x2b, 0x0a, 0x11, 0x0a, 0x0a, 0x04,
    0x17, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x6a, 0x25, 0x2b, 0x0a, 0x12,
    0x0a, 0x0b, 0x04, 0x17, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6a,
    0x25, 0x2b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x17, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x03, 0x6a, 0x2c, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x01, 0x12, 0x03, 0x6b, 0x08,
    0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6b, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6b, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6b, 0x17, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6b, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x01, 0x08, 0x12, 0x03, 0x6b, 0x2e, 0x3b, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x17, 0x02, 0x01, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x6b, 0x2f, 0x3a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x17, 0x02, 0x01,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x6b, 0x2f, 0x35, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x17,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x2f, 0x35, 0x0a, 0x12, 0x0a,
    0x0b, 0x04, 0x17, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6b, 0x2f,
    0x35, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x17, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03,
    0x6b, 0x36, 0x3a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x02, 0x12, 0x03, 0x6c, 0x08, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x04, 0x12, 0x04, 0x6c, 0x08, 0x6b, 0x3c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x05, 0x12, 0x03, 0x6c, 0x08, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6c, 0x0f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6c, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x17, 0x02,
    0x03, 0x12, 0x03, 0x6d, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x04, 0x12,
    0x04, 0x6d, 0x08, 0x6c, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x6d, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x01, 0x12, 0x03, 0x6d, 0x0f,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x03, 0x12, 0x03, 0x6d, 0x1b, 0x1c, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x04, 0x6f, 0x00, 0x75, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x18, 0x01, 0x12, 0x03, 0x6f, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12,
    0x03, 0x70, 0x08, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0x70,
    0x08, 0x6f, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x05, 0x12, 0x03, 0x70, 0x08,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12, 0x03, 0x70, 0x0f, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x03, 0x70, 0x18, 0x19, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x18, 0x02, 0x01, 0x12, 0x03, 0x71, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x71, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x71, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x71, 0x19, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x03, 0x71,
    0x31, 0x32, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x02, 0x12, 0x03, 0x72, 0x08, 0x29, 0x22,
    0x27, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x66, 0x69,
    0x78, 0x65, 0x64, 0x36, 0x34, 0x2c, 0x20, 0x74, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79,
    0x20, 0x47, 0x72, 0x6f, 0x76, 0x65, 0x72, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02,
    0x04, 0x12, 0x04, 0x72, 0x08, 0x71, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x72, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x72, 0x10, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x03, 0x12, 0x03, 0x72, 0x27,
    0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x03, 0x12, 0x03, 0x73, 0x08, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x18, 0x02, 0x03, 0x04, 0x12, 0x04, 0x73, 0x08, 0x72, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x03, 0x05, 0x12, 0x03, 0x73, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x03, 0x01, 0x12, 0x03, 0x73, 0x0f, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x73, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x04, 0x12,
    0x03, 0x74, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x04, 0x12, 0x04, 0x74,
    0x08, 0x73, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x05, 0x12, 0x03, 0x74, 0x08,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x01, 0x12, 0x03, 0x74, 0x0f, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x03, 0x12, 0x03, 0x74, 0x22, 0x23, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x19, 0x12, 0x04, 0x76, 0x00, 0x7b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x19, 0x01,
    0x12, 0x03, 0x76, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x03, 0x77,
    0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0x77, 0x08, 0x76,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x05, 0x12, 0x03, 0x77, 0x08, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x03, 0x77, 0x10, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x03, 0x77, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x19, 0x02, 0x01, 0x12, 0x03, 0x78, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x78, 0x08, 0x77, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x78, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x78, 0x0f, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x03, 0x78, 0x20,
    0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x02, 0x12, 0x03, 0x79, 0x08, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x04, 0x12, 0x04, 0x79, 0x08, 0x78, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x02, 0x05, 0x12, 0x03, 0x79, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x02, 0x01, 0x12, 0x03, 0x79, 0x0f, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x79, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x03, 0x12,
    0x03, 0x7a, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x04, 0x12, 0x04, 0x7a,
    0x08, 0x79, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x05, 0x12, 0x03, 0x7a, 0x08,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x01, 0x12, 0x03, 0x7a, 0x0f, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x03, 0x12, 0x03, 0x7a, 0x22, 0x23, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x1a, 0x12, 0x04, 0x7c, 0x00, 0x7e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x1a, 0x01,
    0x12, 0x03, 0x7c, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00, 0x12, 0x03, 0x7d,
    0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12, 0x04, 0x7d, 0x08, 0x7c,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7d, 0x08, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7d, 0x10, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7d, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x02, 0x04,
    0x1b, 0x12, 0x05, 0x7f, 0x00, 0x81, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12,
    0x03, 0x7f, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0x80, 0x01,
    0x08, 0x33, 0x0a, 0x0e, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x05, 0x80, 0x01, 0x08,
    0x7f, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x06, 0x12, 0x04, 0x80, 0x01, 0x08,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x01, 0x12, 0x04, 0x80, 0x01, 0x24, 0x2e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x03, 0x12, 0x04, 0x80, 0x01, 0x31, 0x32, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x1c, 0x12, 0x06, 0x82, 0x01, 0x00, 0x84, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0x82, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c,
    0x02, 0x00, 0x12, 0x04, 0x83, 0x01, 0x08, 0x1c, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00,
    0x04, 0x12, 0x06, 0x83, 0x01, 0x08, 0x82, 0x01, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x83, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x83, 0x01, 0x0f, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x83, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1d, 0x12, 0x06, 0x85, 0x01,
    0x00, 0x88, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04, 0x85, 0x01, 0x08,
    0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x00, 0x12, 0x04, 0x86, 0x01, 0x08, 0x23, 0x0a,
    0x0f, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12, 0x06, 0x86, 0x01, 0x08, 0x85, 0x01, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x05, 0x12, 0x04, 0x86, 0x01, 0x08, 0x0e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0x86, 0x01, 0x0f, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x86, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1d, 0x02, 0x01, 0x12, 0x04, 0x87, 0x01, 0x08, 0x24, 0x0a, 0x0f, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x01, 0x04, 0x12, 0x06, 0x87, 0x01, 0x08, 0x86, 0x01, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1d, 0x02, 0x01, 0x05, 0x12, 0x04, 0x87, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x01, 0x01, 0x12, 0x04, 0x87, 0x01, 0x0f, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x87, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1e, 0x12,
    0x06, 0x89, 0x01, 0x00, 0x8d, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12, 0x04,
    0x89, 0x01, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x00, 0x12, 0x04, 0x8a, 0x01,
    0x08, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x06, 0x12, 0x04, 0x8a, 0x01, 0x11, 0x30,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x31, 0x44, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8a, 0x01, 0x47, 0x48, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1e, 0x02, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x08, 0x27, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x1e, 0x02, 0x01, 0x04, 0x12, 0x06, 0x8b, 0x01, 0x08, 0x8a, 0x01, 0x49, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1e, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8b, 0x01, 0x08, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1e, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x0d, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1e, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e,
    0x02, 0x02, 0x12, 0x04, 0x8c, 0x01, 0x08, 0x29, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02,
    0x04, 0x12, 0x06, 0x8c, 0x01, 0x08, 0x8b, 0x01, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02,
    0x02, 0x05, 0x12, 0x04, 0x8c, 0x01, 0x08, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x8c, 0x01, 0x0d, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x03,
    0x12, 0x04, 0x8c, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1f, 0x12, 0x06, 0x8e, 0x01,
    0x00, 0x94, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x08,
    0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x00, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x1b, 0x0a,
    0x0f, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x04, 0x12, 0x06, 0x8f, 0x01, 0x08, 0x8e, 0x01, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x0e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x0f, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1f, 0x02, 0x01, 0x12, 0x04, 0x90, 0x01, 0x08, 0x23, 0x0a, 0x0f, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x01, 0x04, 0x12, 0x06, 0x90, 0x01, 0x08, 0x8f, 0x01, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1f, 0x02, 0x01, 0x05, 0x12, 0x04, 0x90, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x01, 0x01, 0x12, 0x04, 0x90, 0x01, 0x0f, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x90, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02,
    0x02, 0x12, 0x04, 0x91, 0x01, 0x08, 0x24, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x04,
    0x12, 0x06, 0x91, 0x01, 0x08, 0x90, 0x01, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02,
    0x05, 0x12, 0x04, 0x91, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x01,
    0x12, 0x04, 0x91, 0x01, 0x0f, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x03, 0x12,
    0x04, 0x91, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x03, 0x12, 0x04, 0x92,
    0x01, 0x08, 0x21, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x03, 0x04, 0x12, 0x06, 0x92, 0x01,
    0x08, 0x91, 0x01, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x03, 0x05, 0x12, 0x04, 0x92,
    0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x03, 0x01, 0x12, 0x04, 0x92, 0x01,
    0x0f, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x03, 0x03, 0x12, 0x04, 0x92, 0x01, 0x1f,
    0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x04, 0x12, 0x04, 0x93, 0x01, 0x08, 0x22, 0x0a,
    0x0f, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x04, 0x04, 0x12, 0x06, 0x93, 0x01, 0x08, 0x92, 0x01, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x04, 0x05, 0x12, 0x04, 0x93, 0x01, 0x08, 0x0e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x04, 0x01, 0x12, 0x04, 0x93, 0x01, 0x0f, 0x1d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x02, 0x04, 0x03, 0x12, 0x04, 0x93, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x20, 0x12, 0x06, 0x95, 0x01, 0x00, 0x9a, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x20, 0x01, 0x12, 0x04, 0x95, 0x01, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x00,
    0x12, 0x04, 0x96, 0x01, 0x08, 0x3c, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x04, 0x12,
    0x06, 0x96, 0x01, 0x08, 0x95, 0x01, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x06,
    0x12, 0x04, 0x96, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x96, 0x01, 0x2a, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x96, 0x01, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x01, 0x12, 0x04, 0x97, 0x01,
    0x08, 0x1b, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x04, 0x12, 0x06, 0x97, 0x01, 0x08,
    0x96, 0x01, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x05, 0x12, 0x04, 0x97, 0x01,
    0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x01, 0x12, 0x04, 0x97, 0x01, 0x0f,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x03, 0x12, 0x04, 0x97, 0x01, 0x19, 0x1a,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x02, 0x12, 0x04, 0x98, 0x01, 0x08, 0x23, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x20, 0x02, 0x02, 0x04, 0x12, 0x06, 0x98, 0x01, 0x08, 0x97, 0x01, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x02, 0x05, 0x12, 0x04, 0x98, 0x01, 0x08, 0x0e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x20, 0x02, 0x02, 0x01, 0x12, 0x04, 0x98, 0x01, 0x0f, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x02, 0x03, 0x12, 0x04, 0x98, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x20, 0x02, 0x03, 0x12, 0x04, 0x99, 0x01, 0x08, 0x24, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x03, 0x04, 0x12, 0x06, 0x99, 0x01, 0x08, 0x98, 0x01, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x03, 0x05, 0x12, 0x04, 0x99, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x03, 0x01, 0x12, 0x04, 0x99, 0x01, 0x0f, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x03, 0x03, 0x12, 0x04, 0x99, 0x01, 0x22, 0x23, 0x0a, 0x22, 0x0a, 0x02, 0x04, 0x21, 0x12, 0x06,
    0x9b, 0x01, 0x00, 0x9d, 0x01, 0x01, 0x22, 0x14, 0x20, 0x4e, 0x6f, 0x20, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x21, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x22, 0x12,
    0x06, 0x9e, 0x01, 0x00, 0xa0, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x22, 0x01, 0x12, 0x04,
    0x9e, 0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x00, 0x12, 0x04, 0x9f, 0x01,
    0x08, 0x3f, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x04, 0x12, 0x06, 0x9f, 0x01, 0x08,
    0x9e, 0x01, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9f, 0x01,
    0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x2d,
    0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x3d, 0x3e,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x23, 0x12, 0x06, 0xa1, 0x01, 0x00, 0xa4, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x23, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x23, 0x02, 0x00, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x1b, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x23, 0x02,
    0x00, 0x04, 0x12, 0x06, 0xa2, 0x01, 0x08, 0xa1, 0x01, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x0f, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xa2, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x01, 0x12,
    0x04, 0xa3, 0x01, 0x08, 0x1e, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x04, 0x12, 0x06,
    0xa3, 0x01, 0x08, 0xa2, 0x01, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xa3, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xa3, 0x01, 0x0f, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa3,
    0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x24, 0x12, 0x06, 0xa5, 0x01, 0x00, 0xa7, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x24, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x25, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x24, 0x02, 0x00, 0x12, 0x04, 0xa6, 0x01, 0x08, 0x1c, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x24, 0x02, 0x00, 0x04, 0x12, 0x06, 0xa6, 0x01, 0x08, 0xa5, 0x01, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x24, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa6, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x24, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x0f, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x24, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa6, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x25,
    0x12, 0x06, 0xa8, 0x01, 0x00, 0xab, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x25, 0x01, 0x12,
    0x04, 0xa8, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x00, 0x12, 0x04, 0xa9,
    0x01, 0x08, 0x24, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x04, 0x12, 0x06, 0xa9, 0x01,
    0x08, 0xa8, 0x01, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa9,
    0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa9, 0x01,
    0x0e, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x22,
    0x23, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x21, 0x22,
    0x1e, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x3a, 0x20, 0x46, 0x69, 0x6e, 0x64, 0x20, 0x6f, 0x75, 0x74,
    0x20, 0x77, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x2e, 0x0a, 0x0a,
    0x0f, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x04, 0x12, 0x06, 0xaa, 0x01, 0x08, 0xa9, 0x01, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x05, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x0d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x0e, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x03, 0x12, 0x04, 0xaa, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x26, 0x12, 0x06, 0xac, 0x01, 0x00, 0xae, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x26, 0x01, 0x12, 0x04, 0xac, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x00,
    0x12, 0x04, 0xad, 0x01, 0x08, 0x1f, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x04, 0x12,
    0x06, 0xad, 0x01, 0x08, 0xac, 0x01, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xad, 0x01, 0x08, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xad, 0x01, 0x10, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xad, 0x01, 0x1d, 0x1e, 0x0a, 0x22, 0x0a, 0x02, 0x04, 0x27, 0x12, 0x06, 0xaf, 0x01, 0x00, 0xb1,
    0x01, 0x01, 0x22, 0x14, 0x20, 0x4e, 0x6f, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20,
    0x6e, 0x65, 0x65, 0x64, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x27, 0x01, 0x12,
    0x04, 0xaf, 0x01, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x28, 0x12, 0x06, 0xb2, 0x01, 0x00,
    0xb4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x28, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x08, 0x19,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x00, 0x12, 0x04, 0xb3, 0x01, 0x08, 0x33, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x04, 0x12, 0x06, 0xb3, 0x01, 0x08, 0xb2, 0x01, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb3, 0x01, 0x08, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x24, 0x2e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x28, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb3, 0x01, 0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x29, 0x12, 0x06, 0xb5, 0x01, 0x00, 0xb7, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x29,
    0x01, 0x12, 0x04, 0xb5, 0x01, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x00, 0x12,
    0x04, 0xb6, 0x01, 0x08, 0x1f, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x04, 0x12, 0x06,
    0xb6, 0x01, 0x08, 0xb5, 0x01, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xb6, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xb6, 0x01, 0x0f, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb6,
    0x01, 0x1d, 0x1e, 0x0a, 0x22, 0x0a, 0x02, 0x04, 0x2a, 0x12, 0x06, 0xb8, 0x01, 0x00, 0xba, 0x01,
    0x01, 0x22, 0x14, 0x20, 0x4e, 0x6f, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x6e,
    0x65, 0x65, 0x64, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2a, 0x01, 0x12, 0x04,
    0xb8, 0x01, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2b, 0x12, 0x06, 0xbb, 0x01, 0x00, 0xc0,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2b, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x20, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02, 0x00, 0x12, 0x04, 0xbc, 0x01, 0x08, 0x1b, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x2b, 0x02, 0x00, 0x04, 0x12, 0x06, 0xbc, 0x01, 0x08, 0xbb, 0x01, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbc, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbc, 0x01, 0x0f, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbc, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x2b, 0x02, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x08, 0x1f, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2b, 0x02,
    0x01, 0x04, 0x12, 0x06, 0xbd, 0x01, 0x08, 0xbc, 0x01, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xbd, 0x01, 0x08, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x10, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xbd, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02, 0x02, 0x12,
    0x04, 0xbe, 0x01, 0x08, 0x23, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x02, 0x04, 0x12, 0x06,
    0xbe, 0x01, 0x08, 0xbd, 0x01, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x02, 0x05, 0x12,
    0x04, 0xbe, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x02, 0x01, 0x12, 0x04,
    0xbe, 0x01, 0x0f, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbe,
    0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02, 0x03, 0x12, 0x04, 0xbf, 0x01, 0x08,
    0x24, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x03, 0x04, 0x12, 0x06, 0xbf, 0x01, 0x08, 0xbe,
    0x01, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x03, 0x05, 0x12, 0x04, 0xbf, 0x01, 0x08,
    0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x03, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x0f, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x03, 0x03, 0x12, 0x04, 0xbf, 0x01, 0x22, 0x23, 0x0a,
    0x22, 0x0a, 0x02, 0x04, 0x2c, 0x12, 0x06, 0xc1, 0x01, 0x00, 0xc3, 0x01, 0x01, 0x22, 0x14, 0x20,
    0x4e, 0x6f, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x65,
    0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2c, 0x01, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x20,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2d, 0x12, 0x06, 0xc4, 0x01, 0x00, 0xc7, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x2d, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x08, 0x21, 0x0a, 0x34, 0x0a, 0x04, 0x04,
    0x2d, 0x02, 0x00, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x1e, 0x22, 0x26, 0x20, 0x6e, 0x65, 0x65, 0x64,
    0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x2c, 0x20,
    0x74, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x47, 0x72, 0x6f, 0x76, 0x65, 0x72,
    0x0a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x04, 0x12, 0x06, 0xc5, 0x01, 0x08, 0xc4,
    0x01, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc5, 0x01, 0x08,
    0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc5, 0x01, 0x0f, 0x19,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc5, 0x01, 0x1c, 0x1d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x01, 0x12, 0x04, 0xc6, 0x01, 0x08, 0x1d, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x2d, 0x02, 0x01, 0x04, 0x12, 0x06, 0xc6, 0x01, 0x08, 0xc5, 0x01, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc6, 0x01, 0x08, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2d, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc6, 0x01, 0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2d, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc6, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x2e, 0x12, 0x06, 0xc8, 0x01, 0x00, 0xce, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2e, 0x01,
    0x12, 0x04, 0xc8, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x00, 0x12, 0x04,
    0xc9, 0x01, 0x08, 0x30, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x04, 0x12, 0x06, 0xc9,
    0x01, 0x08, 0xc8, 0x01, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xc9, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc9,
    0x01, 0x23, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc9, 0x01,
    0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x01, 0x12, 0x04, 0xca, 0x01, 0x08, 0x27,
    0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x04, 0x12, 0x06, 0xca, 0x01, 0x08, 0xc9, 0x01,
    0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x05, 0x12, 0x04, 0xca, 0x01, 0x08, 0x0e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x01, 0x12, 0x04, 0xca, 0x01, 0x0f, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xca, 0x01, 0x25, 0x26, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x2e, 0x02, 0x02, 0x12, 0x04, 0xcb, 0x01, 0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x2e, 0x02, 0x02, 0x04, 0x12, 0x06, 0xcb, 0x01, 0x08, 0xca, 0x01, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2e, 0x02, 0x02, 0x05, 0x12, 0x04, 0xcb, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2e, 0x02, 0x02, 0x01, 0x12, 0x04, 0xcb, 0x01, 0x0f, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2e, 0x02, 0x02, 0x03, 0x12, 0x04, 0xcb, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e,
    0x02, 0x03, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x1a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x03,
    0x04, 0x12, 0x06, 0xcc, 0x01, 0x08, 0xcb, 0x01, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02,
    0x03, 0x05, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xcc, 0x01, 0x0f, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x03, 0x03,
    0x12, 0x04, 0xcc, 0x01, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x04, 0x12, 0x04,
    0xcd, 0x01, 0x08, 0x1f, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x04, 0x04, 0x12, 0x06, 0xcd,
    0x01, 0x08, 0xcc, 0x01, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x04, 0x05, 0x12, 0x04,
    0xcd, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x04, 0x01, 0x12, 0x04, 0xcd,
    0x01, 0x0f, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x04, 0x03, 0x12, 0x04, 0xcd, 0x01,
    0x1d, 0x1e, 0x0a, 0x22, 0x0a, 0x02, 0x04, 0x2f, 0x12, 0x06, 0xcf, 0x01, 0x00, 0xd1, 0x01, 0x01,
    0x22, 0x14, 0x20, 0x4e, 0x6f, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x6e, 0x65,
    0x65, 0x64, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2f, 0x01, 0x12, 0x04, 0xcf,
    0x01, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x30, 0x12, 0x06, 0xd2, 0x01, 0x00, 0xd5, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x30, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x08, 0x1f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x30, 0x02, 0x00, 0x12, 0x04, 0xd3, 0x01, 0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x30, 0x02, 0x00, 0x04, 0x12, 0x06, 0xd3, 0x01, 0x08, 0xd2, 0x01, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x30, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd3, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x30, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd3, 0x01, 0x0f, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x30, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd3, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30,
    0x02, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x26, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x01,
    0x04, 0x12, 0x06, 0xd4, 0x01, 0x08, 0xd3, 0x01, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02,
    0x01, 0x05, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xd4, 0x01, 0x0f, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xd4, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x31, 0x12, 0x06, 0xd6, 0x01,
    0x00, 0xde, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x31, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x08,
    0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x00, 0x12, 0x04, 0xd7, 0x01, 0x08, 0x21, 0x0a,
    0x0f, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x04, 0x12, 0x06, 0xd7, 0x01, 0x08, 0xd6, 0x01, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd7, 0x01, 0x08, 0x0f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x10, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd7, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x31, 0x02, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x08, 0x37, 0x0a, 0x0f, 0x0a, 0x05, 0x04,
    0x31, 0x02, 0x01, 0x04, 0x12, 0x06, 0xd8, 0x01, 0x08, 0xd7, 0x01, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x31, 0x02, 0x01, 0x06, 0x12, 0x04, 0xd8, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x31, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x2a, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xd8, 0x01, 0x35, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02,
    0x02, 0x12, 0x04, 0xd9, 0x01, 0x08, 0x2b, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x02, 0x04,
    0x12, 0x06, 0xd9, 0x01, 0x08, 0xd8, 0x01, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x02,
    0x05, 0x12, 0x04, 0xd9, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xd9, 0x01, 0x0f, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xd9, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x03, 0x12, 0x04, 0xda,
    0x01, 0x08, 0x22, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x03, 0x04, 0x12, 0x06, 0xda, 0x01,
    0x08, 0xd9, 0x01, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x03, 0x05, 0x12, 0x04, 0xda,
    0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x03, 0x01, 0x12, 0x04, 0xda, 0x01,
    0x0f, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x03, 0x03, 0x12, 0x04, 0xda, 0x01, 0x20,
    0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x04, 0x12, 0x04, 0xdb, 0x01, 0x08, 0x1d, 0x0a,
    0x0f, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x04, 0x04, 0x12, 0x06, 0xdb, 0x01, 0x08, 0xda, 0x01, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x04, 0x05, 0x12, 0x04, 0xdb, 0x01, 0x08, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x04, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x0d, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x31, 0x02, 0x04, 0x03, 0x12, 0x04, 0xdb, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x31, 0x02, 0x05, 0x12, 0x04, 0xdc, 0x01, 0x08, 0x21, 0x0a, 0x0f, 0x0a, 0x05, 0x04,
    0x31, 0x02, 0x05, 0x04, 0x12, 0x06, 0xdc, 0x01, 0x08, 0xdb, 0x01, 0x1d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x31, 0x02, 0x05, 0x05, 0x12, 0x04, 0xdc, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x31, 0x02, 0x05, 0x01, 0x12, 0x04, 0xdc, 0x01, 0x0f, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31,
    0x02, 0x05, 0x03, 0x12, 0x04, 0xdc, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02,
    0x06, 0x12, 0x04, 0xdd, 0x01, 0x08, 0x2b, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x06, 0x04,
    0x12, 0x06, 0xdd, 0x01, 0x08, 0xdc, 0x01, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x06,
    0x05, 0x12, 0x04, 0xdd, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x06, 0x01,
    0x12, 0x04, 0xdd, 0x01, 0x0f, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x06, 0x03, 0x12,
    0x04, 0xdd, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x32, 0x12, 0x06, 0xdf, 0x01, 0x00,
    0xe4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x32, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x08, 0x20,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x32, 0x02, 0x00, 0x12, 0x04, 0xe0, 0x01, 0x08, 0x1b, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x04, 0x12, 0x06, 0xe0, 0x01, 0x08, 0xdf, 0x01, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x05, 0x12, 0x04, 0xe0, 0x01, 0x08, 0x0e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x0f, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x32, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe0, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x32, 0x02, 0x01, 0x12, 0x04, 0xe1, 0x01, 0x08, 0x1f, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x32,
    0x02, 0x01, 0x04, 0x12, 0x06, 0xe1, 0x01, 0x08, 0xe0, 0x01, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x32, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe1, 0x01, 0x08, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xe1, 0x01, 0x10, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xe1, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x32, 0x02, 0x02,
    0x12, 0x04, 0xe2, 0x01, 0x08, 0x23, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x02, 0x04, 0x12,
    0x06, 0xe2, 0x01, 0x08, 0xe1, 0x01, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xe2, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xe2, 0x01, 0x0f, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xe2, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x32, 0x02, 0x03, 0x12, 0x04, 0xe3, 0x01,
    0x08, 0x24, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x03, 0x04, 0x12, 0x06, 0xe3, 0x01, 0x08,
    0xe2, 0x01, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x03, 0x05, 0x12, 0x04, 0xe3, 0x01,
    0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x03, 0x01, 0x12, 0x04, 0xe3, 0x01, 0x0f,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x03, 0x03, 0x12, 0x04, 0xe3, 0x01, 0x22, 0x23,
    0x0a, 0x22, 0x0a, 0x02, 0x04, 0x33, 0x12, 0x06, 0xe5, 0x01, 0x00, 0xe7, 0x01, 0x01, 0x22, 0x14,
    0x20, 0x4e, 0x6f, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x6e, 0x65, 0x65, 0x64,
    0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x33, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x08,
    0x21, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
