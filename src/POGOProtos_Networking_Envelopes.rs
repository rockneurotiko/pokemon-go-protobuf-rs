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
pub struct AuthTicket {
    // message fields
    start: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    expire_timestamp_ms: ::std::option::Option<u64>,
    end: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AuthTicket {}

impl AuthTicket {
    pub fn new() -> AuthTicket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AuthTicket {
        static mut instance: ::protobuf::lazy::Lazy<AuthTicket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthTicket,
        };
        unsafe {
            instance.get(|| {
                AuthTicket {
                    start: ::protobuf::SingularField::none(),
                    expire_timestamp_ms: ::std::option::Option::None,
                    end: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes start = 1;

    pub fn clear_start(&mut self) {
        self.start.clear();
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: ::std::vec::Vec<u8>) {
        self.start = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.start.is_none() {
            self.start.set_default();
        };
        self.start.as_mut().unwrap()
    }

    // Take field
    pub fn take_start(&mut self) -> ::std::vec::Vec<u8> {
        self.start.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_start(&self) -> &[u8] {
        match self.start.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint64 expire_timestamp_ms = 2;

    pub fn clear_expire_timestamp_ms(&mut self) {
        self.expire_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_expire_timestamp_ms(&self) -> bool {
        self.expire_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expire_timestamp_ms(&mut self, v: u64) {
        self.expire_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_expire_timestamp_ms(&self) -> u64 {
        self.expire_timestamp_ms.unwrap_or(0)
    }

    // optional bytes end = 3;

    pub fn clear_end(&mut self) {
        self.end.clear();
    }

    pub fn has_end(&self) -> bool {
        self.end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: ::std::vec::Vec<u8>) {
        self.end = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.end.is_none() {
            self.end.set_default();
        };
        self.end.as_mut().unwrap()
    }

    // Take field
    pub fn take_end(&mut self) -> ::std::vec::Vec<u8> {
        self.end.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_end(&self) -> &[u8] {
        match self.end.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for AuthTicket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.start));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.expire_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.end));
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
        for value in &self.start {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.expire_timestamp_ms {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.end {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.expire_timestamp_ms {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.end.as_ref() {
            try!(os.write_bytes(3, &v));
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
        ::std::any::TypeId::of::<AuthTicket>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AuthTicket {
    fn new() -> AuthTicket {
        AuthTicket::new()
    }

    fn descriptor_static(_: ::std::option::Option<AuthTicket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "start",
                    AuthTicket::has_start,
                    AuthTicket::get_start,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "expire_timestamp_ms",
                    AuthTicket::has_expire_timestamp_ms,
                    AuthTicket::get_expire_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "end",
                    AuthTicket::has_end,
                    AuthTicket::get_end,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthTicket>(
                    "AuthTicket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AuthTicket {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_expire_timestamp_ms();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AuthTicket {
    fn eq(&self, other: &AuthTicket) -> bool {
        self.start == other.start &&
        self.expire_timestamp_ms == other.expire_timestamp_ms &&
        self.end == other.end &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AuthTicket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Unknown6Response {
    // message fields
    response_type: ::std::option::Option<i32>,
    unknown2: ::protobuf::SingularPtrField<Unknown6Response_Unknown2>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Unknown6Response {}

impl Unknown6Response {
    pub fn new() -> Unknown6Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Unknown6Response {
        static mut instance: ::protobuf::lazy::Lazy<Unknown6Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Unknown6Response,
        };
        unsafe {
            instance.get(|| {
                Unknown6Response {
                    response_type: ::std::option::Option::None,
                    unknown2: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 response_type = 1;

    pub fn clear_response_type(&mut self) {
        self.response_type = ::std::option::Option::None;
    }

    pub fn has_response_type(&self) -> bool {
        self.response_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response_type(&mut self, v: i32) {
        self.response_type = ::std::option::Option::Some(v);
    }

    pub fn get_response_type(&self) -> i32 {
        self.response_type.unwrap_or(0)
    }

    // optional .POGOProtos.Networking.Envelopes.Unknown6Response.Unknown2 unknown2 = 2;

    pub fn clear_unknown2(&mut self) {
        self.unknown2.clear();
    }

    pub fn has_unknown2(&self) -> bool {
        self.unknown2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unknown2(&mut self, v: Unknown6Response_Unknown2) {
        self.unknown2 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unknown2(&mut self) -> &mut Unknown6Response_Unknown2 {
        if self.unknown2.is_none() {
            self.unknown2.set_default();
        };
        self.unknown2.as_mut().unwrap()
    }

    // Take field
    pub fn take_unknown2(&mut self) -> Unknown6Response_Unknown2 {
        self.unknown2.take().unwrap_or_else(|| Unknown6Response_Unknown2::new())
    }

    pub fn get_unknown2(&self) -> &Unknown6Response_Unknown2 {
        self.unknown2.as_ref().unwrap_or_else(|| Unknown6Response_Unknown2::default_instance())
    }
}

impl ::protobuf::Message for Unknown6Response {
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
                    self.response_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unknown2));
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
        for value in &self.response_type {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.unknown2 {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.response_type {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.unknown2.as_ref() {
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
        ::std::any::TypeId::of::<Unknown6Response>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Unknown6Response {
    fn new() -> Unknown6Response {
        Unknown6Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Unknown6Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "response_type",
                    Unknown6Response::has_response_type,
                    Unknown6Response::get_response_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "unknown2",
                    Unknown6Response::has_unknown2,
                    Unknown6Response::get_unknown2,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Unknown6Response>(
                    "Unknown6Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Unknown6Response {
    fn clear(&mut self) {
        self.clear_response_type();
        self.clear_unknown2();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Unknown6Response {
    fn eq(&self, other: &Unknown6Response) -> bool {
        self.response_type == other.response_type &&
        self.unknown2 == other.unknown2 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Unknown6Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Unknown6Response_Unknown2 {
    // message fields
    unknown1: ::std::option::Option<u64>,
    items: ::protobuf::RepeatedField<Unknown6Response_Unknown2_StoreItem>,
    player_currencies: ::protobuf::RepeatedField<super::POGOProtos_Data_Player::Currency>,
    unknown4: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Unknown6Response_Unknown2 {}

impl Unknown6Response_Unknown2 {
    pub fn new() -> Unknown6Response_Unknown2 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Unknown6Response_Unknown2 {
        static mut instance: ::protobuf::lazy::Lazy<Unknown6Response_Unknown2> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Unknown6Response_Unknown2,
        };
        unsafe {
            instance.get(|| {
                Unknown6Response_Unknown2 {
                    unknown1: ::std::option::Option::None,
                    items: ::protobuf::RepeatedField::new(),
                    player_currencies: ::protobuf::RepeatedField::new(),
                    unknown4: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 unknown1 = 1;

    pub fn clear_unknown1(&mut self) {
        self.unknown1 = ::std::option::Option::None;
    }

    pub fn has_unknown1(&self) -> bool {
        self.unknown1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unknown1(&mut self, v: u64) {
        self.unknown1 = ::std::option::Option::Some(v);
    }

    pub fn get_unknown1(&self) -> u64 {
        self.unknown1.unwrap_or(0)
    }

    // repeated .POGOProtos.Networking.Envelopes.Unknown6Response.Unknown2.StoreItem items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<Unknown6Response_Unknown2_StoreItem>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<Unknown6Response_Unknown2_StoreItem> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<Unknown6Response_Unknown2_StoreItem> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[Unknown6Response_Unknown2_StoreItem] {
        &self.items
    }

    // repeated .POGOProtos.Data.Player.Currency player_currencies = 3;

    pub fn clear_player_currencies(&mut self) {
        self.player_currencies.clear();
    }

    // Param is passed by value, moved
    pub fn set_player_currencies(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Data_Player::Currency>) {
        self.player_currencies = v;
    }

    // Mutable pointer to the field.
    pub fn mut_player_currencies(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Data_Player::Currency> {
        &mut self.player_currencies
    }

    // Take field
    pub fn take_player_currencies(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Data_Player::Currency> {
        ::std::mem::replace(&mut self.player_currencies, ::protobuf::RepeatedField::new())
    }

    pub fn get_player_currencies(&self) -> &[super::POGOProtos_Data_Player::Currency] {
        &self.player_currencies
    }

    // optional string unknown4 = 4;

    pub fn clear_unknown4(&mut self) {
        self.unknown4.clear();
    }

    pub fn has_unknown4(&self) -> bool {
        self.unknown4.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unknown4(&mut self, v: ::std::string::String) {
        self.unknown4 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unknown4(&mut self) -> &mut ::std::string::String {
        if self.unknown4.is_none() {
            self.unknown4.set_default();
        };
        self.unknown4.as_mut().unwrap()
    }

    // Take field
    pub fn take_unknown4(&mut self) -> ::std::string::String {
        self.unknown4.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_unknown4(&self) -> &str {
        match self.unknown4.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Unknown6Response_Unknown2 {
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
                    self.unknown1 = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.player_currencies));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.unknown4));
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
        for value in &self.unknown1 {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.player_currencies {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.unknown4 {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.unknown1 {
            try!(os.write_uint64(1, v));
        };
        for v in &self.items {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.player_currencies {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.unknown4.as_ref() {
            try!(os.write_string(4, &v));
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
        ::std::any::TypeId::of::<Unknown6Response_Unknown2>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Unknown6Response_Unknown2 {
    fn new() -> Unknown6Response_Unknown2 {
        Unknown6Response_Unknown2::new()
    }

    fn descriptor_static(_: ::std::option::Option<Unknown6Response_Unknown2>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "unknown1",
                    Unknown6Response_Unknown2::has_unknown1,
                    Unknown6Response_Unknown2::get_unknown1,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "items",
                    Unknown6Response_Unknown2::get_items,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "player_currencies",
                    Unknown6Response_Unknown2::get_player_currencies,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "unknown4",
                    Unknown6Response_Unknown2::has_unknown4,
                    Unknown6Response_Unknown2::get_unknown4,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Unknown6Response_Unknown2>(
                    "Unknown6Response_Unknown2",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Unknown6Response_Unknown2 {
    fn clear(&mut self) {
        self.clear_unknown1();
        self.clear_items();
        self.clear_player_currencies();
        self.clear_unknown4();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Unknown6Response_Unknown2 {
    fn eq(&self, other: &Unknown6Response_Unknown2) -> bool {
        self.unknown1 == other.unknown1 &&
        self.items == other.items &&
        self.player_currencies == other.player_currencies &&
        self.unknown4 == other.unknown4 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Unknown6Response_Unknown2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Unknown6Response_Unknown2_StoreItem {
    // message fields
    item_id: ::protobuf::SingularField<::std::string::String>,
    is_iap: ::std::option::Option<bool>,
    currency_to_buy: ::protobuf::SingularPtrField<super::POGOProtos_Data_Player::Currency>,
    yields_currency: ::protobuf::SingularPtrField<super::POGOProtos_Data_Player::Currency>,
    yields_item: ::protobuf::SingularPtrField<super::POGOProtos_Inventory_Item::ItemData>,
    tags: ::protobuf::RepeatedField<Unknown6Response_Unknown2_StoreItem_Tag>,
    unknown7: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Unknown6Response_Unknown2_StoreItem {}

impl Unknown6Response_Unknown2_StoreItem {
    pub fn new() -> Unknown6Response_Unknown2_StoreItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Unknown6Response_Unknown2_StoreItem {
        static mut instance: ::protobuf::lazy::Lazy<Unknown6Response_Unknown2_StoreItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Unknown6Response_Unknown2_StoreItem,
        };
        unsafe {
            instance.get(|| {
                Unknown6Response_Unknown2_StoreItem {
                    item_id: ::protobuf::SingularField::none(),
                    is_iap: ::std::option::Option::None,
                    currency_to_buy: ::protobuf::SingularPtrField::none(),
                    yields_currency: ::protobuf::SingularPtrField::none(),
                    yields_item: ::protobuf::SingularPtrField::none(),
                    tags: ::protobuf::RepeatedField::new(),
                    unknown7: ::std::option::Option::None,
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

    // optional bool is_iap = 2;

    pub fn clear_is_iap(&mut self) {
        self.is_iap = ::std::option::Option::None;
    }

    pub fn has_is_iap(&self) -> bool {
        self.is_iap.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_iap(&mut self, v: bool) {
        self.is_iap = ::std::option::Option::Some(v);
    }

    pub fn get_is_iap(&self) -> bool {
        self.is_iap.unwrap_or(false)
    }

    // optional .POGOProtos.Data.Player.Currency currency_to_buy = 3;

    pub fn clear_currency_to_buy(&mut self) {
        self.currency_to_buy.clear();
    }

    pub fn has_currency_to_buy(&self) -> bool {
        self.currency_to_buy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currency_to_buy(&mut self, v: super::POGOProtos_Data_Player::Currency) {
        self.currency_to_buy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currency_to_buy(&mut self) -> &mut super::POGOProtos_Data_Player::Currency {
        if self.currency_to_buy.is_none() {
            self.currency_to_buy.set_default();
        };
        self.currency_to_buy.as_mut().unwrap()
    }

    // Take field
    pub fn take_currency_to_buy(&mut self) -> super::POGOProtos_Data_Player::Currency {
        self.currency_to_buy.take().unwrap_or_else(|| super::POGOProtos_Data_Player::Currency::new())
    }

    pub fn get_currency_to_buy(&self) -> &super::POGOProtos_Data_Player::Currency {
        self.currency_to_buy.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Player::Currency::default_instance())
    }

    // optional .POGOProtos.Data.Player.Currency yields_currency = 4;

    pub fn clear_yields_currency(&mut self) {
        self.yields_currency.clear();
    }

    pub fn has_yields_currency(&self) -> bool {
        self.yields_currency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_yields_currency(&mut self, v: super::POGOProtos_Data_Player::Currency) {
        self.yields_currency = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_yields_currency(&mut self) -> &mut super::POGOProtos_Data_Player::Currency {
        if self.yields_currency.is_none() {
            self.yields_currency.set_default();
        };
        self.yields_currency.as_mut().unwrap()
    }

    // Take field
    pub fn take_yields_currency(&mut self) -> super::POGOProtos_Data_Player::Currency {
        self.yields_currency.take().unwrap_or_else(|| super::POGOProtos_Data_Player::Currency::new())
    }

    pub fn get_yields_currency(&self) -> &super::POGOProtos_Data_Player::Currency {
        self.yields_currency.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Player::Currency::default_instance())
    }

    // optional .POGOProtos.Inventory.Item.ItemData yields_item = 5;

    pub fn clear_yields_item(&mut self) {
        self.yields_item.clear();
    }

    pub fn has_yields_item(&self) -> bool {
        self.yields_item.is_some()
    }

    // Param is passed by value, moved
    pub fn set_yields_item(&mut self, v: super::POGOProtos_Inventory_Item::ItemData) {
        self.yields_item = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_yields_item(&mut self) -> &mut super::POGOProtos_Inventory_Item::ItemData {
        if self.yields_item.is_none() {
            self.yields_item.set_default();
        };
        self.yields_item.as_mut().unwrap()
    }

    // Take field
    pub fn take_yields_item(&mut self) -> super::POGOProtos_Inventory_Item::ItemData {
        self.yields_item.take().unwrap_or_else(|| super::POGOProtos_Inventory_Item::ItemData::new())
    }

    pub fn get_yields_item(&self) -> &super::POGOProtos_Inventory_Item::ItemData {
        self.yields_item.as_ref().unwrap_or_else(|| super::POGOProtos_Inventory_Item::ItemData::default_instance())
    }

    // repeated .POGOProtos.Networking.Envelopes.Unknown6Response.Unknown2.StoreItem.Tag tags = 6;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::protobuf::RepeatedField<Unknown6Response_Unknown2_StoreItem_Tag>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags(&mut self) -> &mut ::protobuf::RepeatedField<Unknown6Response_Unknown2_StoreItem_Tag> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::protobuf::RepeatedField<Unknown6Response_Unknown2_StoreItem_Tag> {
        ::std::mem::replace(&mut self.tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_tags(&self) -> &[Unknown6Response_Unknown2_StoreItem_Tag] {
        &self.tags
    }

    // optional int32 unknown7 = 7;

    pub fn clear_unknown7(&mut self) {
        self.unknown7 = ::std::option::Option::None;
    }

    pub fn has_unknown7(&self) -> bool {
        self.unknown7.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unknown7(&mut self, v: i32) {
        self.unknown7 = ::std::option::Option::Some(v);
    }

    pub fn get_unknown7(&self) -> i32 {
        self.unknown7.unwrap_or(0)
    }
}

impl ::protobuf::Message for Unknown6Response_Unknown2_StoreItem {
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
                    let tmp = try!(is.read_bool());
                    self.is_iap = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currency_to_buy));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.yields_currency));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.yields_item));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tags));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.unknown7 = ::std::option::Option::Some(tmp);
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
        if self.is_iap.is_some() {
            my_size += 2;
        };
        for value in &self.currency_to_buy {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.yields_currency {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.yields_item {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.tags {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.unknown7 {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.is_iap {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.currency_to_buy.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.yields_currency.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.yields_item.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.tags {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.unknown7 {
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
        ::std::any::TypeId::of::<Unknown6Response_Unknown2_StoreItem>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Unknown6Response_Unknown2_StoreItem {
    fn new() -> Unknown6Response_Unknown2_StoreItem {
        Unknown6Response_Unknown2_StoreItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<Unknown6Response_Unknown2_StoreItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "item_id",
                    Unknown6Response_Unknown2_StoreItem::has_item_id,
                    Unknown6Response_Unknown2_StoreItem::get_item_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_iap",
                    Unknown6Response_Unknown2_StoreItem::has_is_iap,
                    Unknown6Response_Unknown2_StoreItem::get_is_iap,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "currency_to_buy",
                    Unknown6Response_Unknown2_StoreItem::has_currency_to_buy,
                    Unknown6Response_Unknown2_StoreItem::get_currency_to_buy,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "yields_currency",
                    Unknown6Response_Unknown2_StoreItem::has_yields_currency,
                    Unknown6Response_Unknown2_StoreItem::get_yields_currency,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "yields_item",
                    Unknown6Response_Unknown2_StoreItem::has_yields_item,
                    Unknown6Response_Unknown2_StoreItem::get_yields_item,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "tags",
                    Unknown6Response_Unknown2_StoreItem::get_tags,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "unknown7",
                    Unknown6Response_Unknown2_StoreItem::has_unknown7,
                    Unknown6Response_Unknown2_StoreItem::get_unknown7,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Unknown6Response_Unknown2_StoreItem>(
                    "Unknown6Response_Unknown2_StoreItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Unknown6Response_Unknown2_StoreItem {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_is_iap();
        self.clear_currency_to_buy();
        self.clear_yields_currency();
        self.clear_yields_item();
        self.clear_tags();
        self.clear_unknown7();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Unknown6Response_Unknown2_StoreItem {
    fn eq(&self, other: &Unknown6Response_Unknown2_StoreItem) -> bool {
        self.item_id == other.item_id &&
        self.is_iap == other.is_iap &&
        self.currency_to_buy == other.currency_to_buy &&
        self.yields_currency == other.yields_currency &&
        self.yields_item == other.yields_item &&
        self.tags == other.tags &&
        self.unknown7 == other.unknown7 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Unknown6Response_Unknown2_StoreItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Unknown6Response_Unknown2_StoreItem_Tag {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Unknown6Response_Unknown2_StoreItem_Tag {}

impl Unknown6Response_Unknown2_StoreItem_Tag {
    pub fn new() -> Unknown6Response_Unknown2_StoreItem_Tag {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Unknown6Response_Unknown2_StoreItem_Tag {
        static mut instance: ::protobuf::lazy::Lazy<Unknown6Response_Unknown2_StoreItem_Tag> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Unknown6Response_Unknown2_StoreItem_Tag,
        };
        unsafe {
            instance.get(|| {
                Unknown6Response_Unknown2_StoreItem_Tag {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Unknown6Response_Unknown2_StoreItem_Tag {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value));
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
        for value in &self.key {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.value {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
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
        ::std::any::TypeId::of::<Unknown6Response_Unknown2_StoreItem_Tag>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Unknown6Response_Unknown2_StoreItem_Tag {
    fn new() -> Unknown6Response_Unknown2_StoreItem_Tag {
        Unknown6Response_Unknown2_StoreItem_Tag::new()
    }

    fn descriptor_static(_: ::std::option::Option<Unknown6Response_Unknown2_StoreItem_Tag>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    Unknown6Response_Unknown2_StoreItem_Tag::has_key,
                    Unknown6Response_Unknown2_StoreItem_Tag::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "value",
                    Unknown6Response_Unknown2_StoreItem_Tag::has_value,
                    Unknown6Response_Unknown2_StoreItem_Tag::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Unknown6Response_Unknown2_StoreItem_Tag>(
                    "Unknown6Response_Unknown2_StoreItem_Tag",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Unknown6Response_Unknown2_StoreItem_Tag {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Unknown6Response_Unknown2_StoreItem_Tag {
    fn eq(&self, other: &Unknown6Response_Unknown2_StoreItem_Tag) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Unknown6Response_Unknown2_StoreItem_Tag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RequestEnvelope {
    // message fields
    status_code: ::std::option::Option<i32>,
    request_id: ::std::option::Option<u64>,
    requests: ::protobuf::RepeatedField<super::POGOProtos_Networking_Requests::Request>,
    unknown6: ::protobuf::RepeatedField<Unknown6>,
    latitude: ::std::option::Option<f64>,
    longitude: ::std::option::Option<f64>,
    altitude: ::std::option::Option<f64>,
    auth_info: ::protobuf::SingularPtrField<RequestEnvelope_AuthInfo>,
    auth_ticket: ::protobuf::SingularPtrField<AuthTicket>,
    unknown12: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestEnvelope {}

impl RequestEnvelope {
    pub fn new() -> RequestEnvelope {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestEnvelope {
        static mut instance: ::protobuf::lazy::Lazy<RequestEnvelope> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestEnvelope,
        };
        unsafe {
            instance.get(|| {
                RequestEnvelope {
                    status_code: ::std::option::Option::None,
                    request_id: ::std::option::Option::None,
                    requests: ::protobuf::RepeatedField::new(),
                    unknown6: ::protobuf::RepeatedField::new(),
                    latitude: ::std::option::Option::None,
                    longitude: ::std::option::Option::None,
                    altitude: ::std::option::Option::None,
                    auth_info: ::protobuf::SingularPtrField::none(),
                    auth_ticket: ::protobuf::SingularPtrField::none(),
                    unknown12: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 status_code = 1;

    pub fn clear_status_code(&mut self) {
        self.status_code = ::std::option::Option::None;
    }

    pub fn has_status_code(&self) -> bool {
        self.status_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status_code(&mut self, v: i32) {
        self.status_code = ::std::option::Option::Some(v);
    }

    pub fn get_status_code(&self) -> i32 {
        self.status_code.unwrap_or(0)
    }

    // optional uint64 request_id = 3;

    pub fn clear_request_id(&mut self) {
        self.request_id = ::std::option::Option::None;
    }

    pub fn has_request_id(&self) -> bool {
        self.request_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_id(&mut self, v: u64) {
        self.request_id = ::std::option::Option::Some(v);
    }

    pub fn get_request_id(&self) -> u64 {
        self.request_id.unwrap_or(0)
    }

    // repeated .POGOProtos.Networking.Requests.Request requests = 4;

    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }

    // Param is passed by value, moved
    pub fn set_requests(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Networking_Requests::Request>) {
        self.requests = v;
    }

    // Mutable pointer to the field.
    pub fn mut_requests(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Networking_Requests::Request> {
        &mut self.requests
    }

    // Take field
    pub fn take_requests(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Networking_Requests::Request> {
        ::std::mem::replace(&mut self.requests, ::protobuf::RepeatedField::new())
    }

    pub fn get_requests(&self) -> &[super::POGOProtos_Networking_Requests::Request] {
        &self.requests
    }

    // repeated .POGOProtos.Networking.Envelopes.Unknown6 unknown6 = 6;

    pub fn clear_unknown6(&mut self) {
        self.unknown6.clear();
    }

    // Param is passed by value, moved
    pub fn set_unknown6(&mut self, v: ::protobuf::RepeatedField<Unknown6>) {
        self.unknown6 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_unknown6(&mut self) -> &mut ::protobuf::RepeatedField<Unknown6> {
        &mut self.unknown6
    }

    // Take field
    pub fn take_unknown6(&mut self) -> ::protobuf::RepeatedField<Unknown6> {
        ::std::mem::replace(&mut self.unknown6, ::protobuf::RepeatedField::new())
    }

    pub fn get_unknown6(&self) -> &[Unknown6] {
        &self.unknown6
    }

    // optional double latitude = 7;

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

    // optional double longitude = 8;

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

    // optional double altitude = 9;

    pub fn clear_altitude(&mut self) {
        self.altitude = ::std::option::Option::None;
    }

    pub fn has_altitude(&self) -> bool {
        self.altitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_altitude(&mut self, v: f64) {
        self.altitude = ::std::option::Option::Some(v);
    }

    pub fn get_altitude(&self) -> f64 {
        self.altitude.unwrap_or(0.)
    }

    // optional .POGOProtos.Networking.Envelopes.RequestEnvelope.AuthInfo auth_info = 10;

    pub fn clear_auth_info(&mut self) {
        self.auth_info.clear();
    }

    pub fn has_auth_info(&self) -> bool {
        self.auth_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_info(&mut self, v: RequestEnvelope_AuthInfo) {
        self.auth_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_info(&mut self) -> &mut RequestEnvelope_AuthInfo {
        if self.auth_info.is_none() {
            self.auth_info.set_default();
        };
        self.auth_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_info(&mut self) -> RequestEnvelope_AuthInfo {
        self.auth_info.take().unwrap_or_else(|| RequestEnvelope_AuthInfo::new())
    }

    pub fn get_auth_info(&self) -> &RequestEnvelope_AuthInfo {
        self.auth_info.as_ref().unwrap_or_else(|| RequestEnvelope_AuthInfo::default_instance())
    }

    // optional .POGOProtos.Networking.Envelopes.AuthTicket auth_ticket = 11;

    pub fn clear_auth_ticket(&mut self) {
        self.auth_ticket.clear();
    }

    pub fn has_auth_ticket(&self) -> bool {
        self.auth_ticket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_ticket(&mut self, v: AuthTicket) {
        self.auth_ticket = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_ticket(&mut self) -> &mut AuthTicket {
        if self.auth_ticket.is_none() {
            self.auth_ticket.set_default();
        };
        self.auth_ticket.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_ticket(&mut self) -> AuthTicket {
        self.auth_ticket.take().unwrap_or_else(|| AuthTicket::new())
    }

    pub fn get_auth_ticket(&self) -> &AuthTicket {
        self.auth_ticket.as_ref().unwrap_or_else(|| AuthTicket::default_instance())
    }

    // optional int64 unknown12 = 12;

    pub fn clear_unknown12(&mut self) {
        self.unknown12 = ::std::option::Option::None;
    }

    pub fn has_unknown12(&self) -> bool {
        self.unknown12.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unknown12(&mut self, v: i64) {
        self.unknown12 = ::std::option::Option::Some(v);
    }

    pub fn get_unknown12(&self) -> i64 {
        self.unknown12.unwrap_or(0)
    }
}

impl ::protobuf::Message for RequestEnvelope {
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
                    self.status_code = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.request_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.requests));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.unknown6));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.latitude = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.longitude = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.altitude = ::std::option::Option::Some(tmp);
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_info));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_ticket));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.unknown12 = ::std::option::Option::Some(tmp);
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
        for value in &self.status_code {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.request_id {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.requests {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.unknown6 {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.latitude.is_some() {
            my_size += 9;
        };
        if self.longitude.is_some() {
            my_size += 9;
        };
        if self.altitude.is_some() {
            my_size += 9;
        };
        for value in &self.auth_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.auth_ticket {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.unknown12 {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status_code {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.request_id {
            try!(os.write_uint64(3, v));
        };
        for v in &self.requests {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.unknown6 {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.latitude {
            try!(os.write_double(7, v));
        };
        if let Some(v) = self.longitude {
            try!(os.write_double(8, v));
        };
        if let Some(v) = self.altitude {
            try!(os.write_double(9, v));
        };
        if let Some(v) = self.auth_info.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.auth_ticket.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.unknown12 {
            try!(os.write_int64(12, v));
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
        ::std::any::TypeId::of::<RequestEnvelope>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestEnvelope {
    fn new() -> RequestEnvelope {
        RequestEnvelope::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestEnvelope>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "status_code",
                    RequestEnvelope::has_status_code,
                    RequestEnvelope::get_status_code,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "request_id",
                    RequestEnvelope::has_request_id,
                    RequestEnvelope::get_request_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "requests",
                    RequestEnvelope::get_requests,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "unknown6",
                    RequestEnvelope::get_unknown6,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "latitude",
                    RequestEnvelope::has_latitude,
                    RequestEnvelope::get_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "longitude",
                    RequestEnvelope::has_longitude,
                    RequestEnvelope::get_longitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "altitude",
                    RequestEnvelope::has_altitude,
                    RequestEnvelope::get_altitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "auth_info",
                    RequestEnvelope::has_auth_info,
                    RequestEnvelope::get_auth_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "auth_ticket",
                    RequestEnvelope::has_auth_ticket,
                    RequestEnvelope::get_auth_ticket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "unknown12",
                    RequestEnvelope::has_unknown12,
                    RequestEnvelope::get_unknown12,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestEnvelope>(
                    "RequestEnvelope",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestEnvelope {
    fn clear(&mut self) {
        self.clear_status_code();
        self.clear_request_id();
        self.clear_requests();
        self.clear_unknown6();
        self.clear_latitude();
        self.clear_longitude();
        self.clear_altitude();
        self.clear_auth_info();
        self.clear_auth_ticket();
        self.clear_unknown12();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RequestEnvelope {
    fn eq(&self, other: &RequestEnvelope) -> bool {
        self.status_code == other.status_code &&
        self.request_id == other.request_id &&
        self.requests == other.requests &&
        self.unknown6 == other.unknown6 &&
        self.latitude == other.latitude &&
        self.longitude == other.longitude &&
        self.altitude == other.altitude &&
        self.auth_info == other.auth_info &&
        self.auth_ticket == other.auth_ticket &&
        self.unknown12 == other.unknown12 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RequestEnvelope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RequestEnvelope_AuthInfo {
    // message fields
    provider: ::protobuf::SingularField<::std::string::String>,
    token: ::protobuf::SingularPtrField<RequestEnvelope_AuthInfo_JWT>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestEnvelope_AuthInfo {}

impl RequestEnvelope_AuthInfo {
    pub fn new() -> RequestEnvelope_AuthInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestEnvelope_AuthInfo {
        static mut instance: ::protobuf::lazy::Lazy<RequestEnvelope_AuthInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestEnvelope_AuthInfo,
        };
        unsafe {
            instance.get(|| {
                RequestEnvelope_AuthInfo {
                    provider: ::protobuf::SingularField::none(),
                    token: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string provider = 1;

    pub fn clear_provider(&mut self) {
        self.provider.clear();
    }

    pub fn has_provider(&self) -> bool {
        self.provider.is_some()
    }

    // Param is passed by value, moved
    pub fn set_provider(&mut self, v: ::std::string::String) {
        self.provider = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_provider(&mut self) -> &mut ::std::string::String {
        if self.provider.is_none() {
            self.provider.set_default();
        };
        self.provider.as_mut().unwrap()
    }

    // Take field
    pub fn take_provider(&mut self) -> ::std::string::String {
        self.provider.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_provider(&self) -> &str {
        match self.provider.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .POGOProtos.Networking.Envelopes.RequestEnvelope.AuthInfo.JWT token = 2;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: RequestEnvelope_AuthInfo_JWT) {
        self.token = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut RequestEnvelope_AuthInfo_JWT {
        if self.token.is_none() {
            self.token.set_default();
        };
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> RequestEnvelope_AuthInfo_JWT {
        self.token.take().unwrap_or_else(|| RequestEnvelope_AuthInfo_JWT::new())
    }

    pub fn get_token(&self) -> &RequestEnvelope_AuthInfo_JWT {
        self.token.as_ref().unwrap_or_else(|| RequestEnvelope_AuthInfo_JWT::default_instance())
    }
}

impl ::protobuf::Message for RequestEnvelope_AuthInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.provider));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.token));
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
        for value in &self.provider {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.token {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.provider.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.token.as_ref() {
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
        ::std::any::TypeId::of::<RequestEnvelope_AuthInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestEnvelope_AuthInfo {
    fn new() -> RequestEnvelope_AuthInfo {
        RequestEnvelope_AuthInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestEnvelope_AuthInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "provider",
                    RequestEnvelope_AuthInfo::has_provider,
                    RequestEnvelope_AuthInfo::get_provider,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "token",
                    RequestEnvelope_AuthInfo::has_token,
                    RequestEnvelope_AuthInfo::get_token,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestEnvelope_AuthInfo>(
                    "RequestEnvelope_AuthInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestEnvelope_AuthInfo {
    fn clear(&mut self) {
        self.clear_provider();
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RequestEnvelope_AuthInfo {
    fn eq(&self, other: &RequestEnvelope_AuthInfo) -> bool {
        self.provider == other.provider &&
        self.token == other.token &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RequestEnvelope_AuthInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RequestEnvelope_AuthInfo_JWT {
    // message fields
    contents: ::protobuf::SingularField<::std::string::String>,
    unknown2: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestEnvelope_AuthInfo_JWT {}

impl RequestEnvelope_AuthInfo_JWT {
    pub fn new() -> RequestEnvelope_AuthInfo_JWT {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestEnvelope_AuthInfo_JWT {
        static mut instance: ::protobuf::lazy::Lazy<RequestEnvelope_AuthInfo_JWT> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestEnvelope_AuthInfo_JWT,
        };
        unsafe {
            instance.get(|| {
                RequestEnvelope_AuthInfo_JWT {
                    contents: ::protobuf::SingularField::none(),
                    unknown2: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string contents = 1;

    pub fn clear_contents(&mut self) {
        self.contents.clear();
    }

    pub fn has_contents(&self) -> bool {
        self.contents.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contents(&mut self, v: ::std::string::String) {
        self.contents = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contents(&mut self) -> &mut ::std::string::String {
        if self.contents.is_none() {
            self.contents.set_default();
        };
        self.contents.as_mut().unwrap()
    }

    // Take field
    pub fn take_contents(&mut self) -> ::std::string::String {
        self.contents.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_contents(&self) -> &str {
        match self.contents.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 unknown2 = 2;

    pub fn clear_unknown2(&mut self) {
        self.unknown2 = ::std::option::Option::None;
    }

    pub fn has_unknown2(&self) -> bool {
        self.unknown2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unknown2(&mut self, v: i32) {
        self.unknown2 = ::std::option::Option::Some(v);
    }

    pub fn get_unknown2(&self) -> i32 {
        self.unknown2.unwrap_or(0)
    }
}

impl ::protobuf::Message for RequestEnvelope_AuthInfo_JWT {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.contents));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.unknown2 = ::std::option::Option::Some(tmp);
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
        for value in &self.contents {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.unknown2 {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.contents.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.unknown2 {
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
        ::std::any::TypeId::of::<RequestEnvelope_AuthInfo_JWT>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestEnvelope_AuthInfo_JWT {
    fn new() -> RequestEnvelope_AuthInfo_JWT {
        RequestEnvelope_AuthInfo_JWT::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestEnvelope_AuthInfo_JWT>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "contents",
                    RequestEnvelope_AuthInfo_JWT::has_contents,
                    RequestEnvelope_AuthInfo_JWT::get_contents,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "unknown2",
                    RequestEnvelope_AuthInfo_JWT::has_unknown2,
                    RequestEnvelope_AuthInfo_JWT::get_unknown2,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestEnvelope_AuthInfo_JWT>(
                    "RequestEnvelope_AuthInfo_JWT",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestEnvelope_AuthInfo_JWT {
    fn clear(&mut self) {
        self.clear_contents();
        self.clear_unknown2();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RequestEnvelope_AuthInfo_JWT {
    fn eq(&self, other: &RequestEnvelope_AuthInfo_JWT) -> bool {
        self.contents == other.contents &&
        self.unknown2 == other.unknown2 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RequestEnvelope_AuthInfo_JWT {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Unknown6 {
    // message fields
    request_type: ::std::option::Option<i32>,
    unknown2: ::protobuf::SingularPtrField<Unknown6_Unknown2>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Unknown6 {}

impl Unknown6 {
    pub fn new() -> Unknown6 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Unknown6 {
        static mut instance: ::protobuf::lazy::Lazy<Unknown6> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Unknown6,
        };
        unsafe {
            instance.get(|| {
                Unknown6 {
                    request_type: ::std::option::Option::None,
                    unknown2: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 request_type = 1;

    pub fn clear_request_type(&mut self) {
        self.request_type = ::std::option::Option::None;
    }

    pub fn has_request_type(&self) -> bool {
        self.request_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_type(&mut self, v: i32) {
        self.request_type = ::std::option::Option::Some(v);
    }

    pub fn get_request_type(&self) -> i32 {
        self.request_type.unwrap_or(0)
    }

    // optional .POGOProtos.Networking.Envelopes.Unknown6.Unknown2 unknown2 = 2;

    pub fn clear_unknown2(&mut self) {
        self.unknown2.clear();
    }

    pub fn has_unknown2(&self) -> bool {
        self.unknown2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unknown2(&mut self, v: Unknown6_Unknown2) {
        self.unknown2 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unknown2(&mut self) -> &mut Unknown6_Unknown2 {
        if self.unknown2.is_none() {
            self.unknown2.set_default();
        };
        self.unknown2.as_mut().unwrap()
    }

    // Take field
    pub fn take_unknown2(&mut self) -> Unknown6_Unknown2 {
        self.unknown2.take().unwrap_or_else(|| Unknown6_Unknown2::new())
    }

    pub fn get_unknown2(&self) -> &Unknown6_Unknown2 {
        self.unknown2.as_ref().unwrap_or_else(|| Unknown6_Unknown2::default_instance())
    }
}

impl ::protobuf::Message for Unknown6 {
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
                    self.request_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unknown2));
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
        for value in &self.request_type {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.unknown2 {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.request_type {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.unknown2.as_ref() {
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
        ::std::any::TypeId::of::<Unknown6>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Unknown6 {
    fn new() -> Unknown6 {
        Unknown6::new()
    }

    fn descriptor_static(_: ::std::option::Option<Unknown6>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "request_type",
                    Unknown6::has_request_type,
                    Unknown6::get_request_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "unknown2",
                    Unknown6::has_unknown2,
                    Unknown6::get_unknown2,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Unknown6>(
                    "Unknown6",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Unknown6 {
    fn clear(&mut self) {
        self.clear_request_type();
        self.clear_unknown2();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Unknown6 {
    fn eq(&self, other: &Unknown6) -> bool {
        self.request_type == other.request_type &&
        self.unknown2 == other.unknown2 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Unknown6 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Unknown6_Unknown2 {
    // message fields
    unknown1: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Unknown6_Unknown2 {}

impl Unknown6_Unknown2 {
    pub fn new() -> Unknown6_Unknown2 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Unknown6_Unknown2 {
        static mut instance: ::protobuf::lazy::Lazy<Unknown6_Unknown2> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Unknown6_Unknown2,
        };
        unsafe {
            instance.get(|| {
                Unknown6_Unknown2 {
                    unknown1: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes unknown1 = 1;

    pub fn clear_unknown1(&mut self) {
        self.unknown1.clear();
    }

    pub fn has_unknown1(&self) -> bool {
        self.unknown1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unknown1(&mut self, v: ::std::vec::Vec<u8>) {
        self.unknown1 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unknown1(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.unknown1.is_none() {
            self.unknown1.set_default();
        };
        self.unknown1.as_mut().unwrap()
    }

    // Take field
    pub fn take_unknown1(&mut self) -> ::std::vec::Vec<u8> {
        self.unknown1.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_unknown1(&self) -> &[u8] {
        match self.unknown1.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for Unknown6_Unknown2 {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.unknown1));
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
        for value in &self.unknown1 {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.unknown1.as_ref() {
            try!(os.write_bytes(1, &v));
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
        ::std::any::TypeId::of::<Unknown6_Unknown2>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Unknown6_Unknown2 {
    fn new() -> Unknown6_Unknown2 {
        Unknown6_Unknown2::new()
    }

    fn descriptor_static(_: ::std::option::Option<Unknown6_Unknown2>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "unknown1",
                    Unknown6_Unknown2::has_unknown1,
                    Unknown6_Unknown2::get_unknown1,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Unknown6_Unknown2>(
                    "Unknown6_Unknown2",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Unknown6_Unknown2 {
    fn clear(&mut self) {
        self.clear_unknown1();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Unknown6_Unknown2 {
    fn eq(&self, other: &Unknown6_Unknown2) -> bool {
        self.unknown1 == other.unknown1 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Unknown6_Unknown2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ResponseEnvelope {
    // message fields
    status_code: ::std::option::Option<i32>,
    request_id: ::std::option::Option<u64>,
    api_url: ::protobuf::SingularField<::std::string::String>,
    unknown6: ::protobuf::RepeatedField<Unknown6Response>,
    auth_ticket: ::protobuf::SingularPtrField<AuthTicket>,
    returns: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    error: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseEnvelope {}

impl ResponseEnvelope {
    pub fn new() -> ResponseEnvelope {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseEnvelope {
        static mut instance: ::protobuf::lazy::Lazy<ResponseEnvelope> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseEnvelope,
        };
        unsafe {
            instance.get(|| {
                ResponseEnvelope {
                    status_code: ::std::option::Option::None,
                    request_id: ::std::option::Option::None,
                    api_url: ::protobuf::SingularField::none(),
                    unknown6: ::protobuf::RepeatedField::new(),
                    auth_ticket: ::protobuf::SingularPtrField::none(),
                    returns: ::protobuf::RepeatedField::new(),
                    error: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 status_code = 1;

    pub fn clear_status_code(&mut self) {
        self.status_code = ::std::option::Option::None;
    }

    pub fn has_status_code(&self) -> bool {
        self.status_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status_code(&mut self, v: i32) {
        self.status_code = ::std::option::Option::Some(v);
    }

    pub fn get_status_code(&self) -> i32 {
        self.status_code.unwrap_or(0)
    }

    // optional uint64 request_id = 2;

    pub fn clear_request_id(&mut self) {
        self.request_id = ::std::option::Option::None;
    }

    pub fn has_request_id(&self) -> bool {
        self.request_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_id(&mut self, v: u64) {
        self.request_id = ::std::option::Option::Some(v);
    }

    pub fn get_request_id(&self) -> u64 {
        self.request_id.unwrap_or(0)
    }

    // optional string api_url = 3;

    pub fn clear_api_url(&mut self) {
        self.api_url.clear();
    }

    pub fn has_api_url(&self) -> bool {
        self.api_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_api_url(&mut self, v: ::std::string::String) {
        self.api_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_api_url(&mut self) -> &mut ::std::string::String {
        if self.api_url.is_none() {
            self.api_url.set_default();
        };
        self.api_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_api_url(&mut self) -> ::std::string::String {
        self.api_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_api_url(&self) -> &str {
        match self.api_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .POGOProtos.Networking.Envelopes.Unknown6Response unknown6 = 6;

    pub fn clear_unknown6(&mut self) {
        self.unknown6.clear();
    }

    // Param is passed by value, moved
    pub fn set_unknown6(&mut self, v: ::protobuf::RepeatedField<Unknown6Response>) {
        self.unknown6 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_unknown6(&mut self) -> &mut ::protobuf::RepeatedField<Unknown6Response> {
        &mut self.unknown6
    }

    // Take field
    pub fn take_unknown6(&mut self) -> ::protobuf::RepeatedField<Unknown6Response> {
        ::std::mem::replace(&mut self.unknown6, ::protobuf::RepeatedField::new())
    }

    pub fn get_unknown6(&self) -> &[Unknown6Response] {
        &self.unknown6
    }

    // optional .POGOProtos.Networking.Envelopes.AuthTicket auth_ticket = 7;

    pub fn clear_auth_ticket(&mut self) {
        self.auth_ticket.clear();
    }

    pub fn has_auth_ticket(&self) -> bool {
        self.auth_ticket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_ticket(&mut self, v: AuthTicket) {
        self.auth_ticket = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_ticket(&mut self) -> &mut AuthTicket {
        if self.auth_ticket.is_none() {
            self.auth_ticket.set_default();
        };
        self.auth_ticket.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_ticket(&mut self) -> AuthTicket {
        self.auth_ticket.take().unwrap_or_else(|| AuthTicket::new())
    }

    pub fn get_auth_ticket(&self) -> &AuthTicket {
        self.auth_ticket.as_ref().unwrap_or_else(|| AuthTicket::default_instance())
    }

    // repeated bytes returns = 100;

    pub fn clear_returns(&mut self) {
        self.returns.clear();
    }

    // Param is passed by value, moved
    pub fn set_returns(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.returns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_returns(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.returns
    }

    // Take field
    pub fn take_returns(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.returns, ::protobuf::RepeatedField::new())
    }

    pub fn get_returns(&self) -> &[::std::vec::Vec<u8>] {
        &self.returns
    }

    // optional string error = 101;

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
}

impl ::protobuf::Message for ResponseEnvelope {
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
                    self.status_code = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.request_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.api_url));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.unknown6));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auth_ticket));
                },
                100 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.returns));
                },
                101 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error));
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
        for value in &self.status_code {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.request_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.api_url {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.unknown6 {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.auth_ticket {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.returns {
            my_size += ::protobuf::rt::bytes_size(100, &value);
        };
        for value in &self.error {
            my_size += ::protobuf::rt::string_size(101, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status_code {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.request_id {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.api_url.as_ref() {
            try!(os.write_string(3, &v));
        };
        for v in &self.unknown6 {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.auth_ticket.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.returns {
            try!(os.write_bytes(100, &v));
        };
        if let Some(v) = self.error.as_ref() {
            try!(os.write_string(101, &v));
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
        ::std::any::TypeId::of::<ResponseEnvelope>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseEnvelope {
    fn new() -> ResponseEnvelope {
        ResponseEnvelope::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseEnvelope>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "status_code",
                    ResponseEnvelope::has_status_code,
                    ResponseEnvelope::get_status_code,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "request_id",
                    ResponseEnvelope::has_request_id,
                    ResponseEnvelope::get_request_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "api_url",
                    ResponseEnvelope::has_api_url,
                    ResponseEnvelope::get_api_url,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "unknown6",
                    ResponseEnvelope::get_unknown6,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "auth_ticket",
                    ResponseEnvelope::has_auth_ticket,
                    ResponseEnvelope::get_auth_ticket,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "returns",
                    ResponseEnvelope::get_returns,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "error",
                    ResponseEnvelope::has_error,
                    ResponseEnvelope::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseEnvelope>(
                    "ResponseEnvelope",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseEnvelope {
    fn clear(&mut self) {
        self.clear_status_code();
        self.clear_request_id();
        self.clear_api_url();
        self.clear_unknown6();
        self.clear_auth_ticket();
        self.clear_returns();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ResponseEnvelope {
    fn eq(&self, other: &ResponseEnvelope) -> bool {
        self.status_code == other.status_code &&
        self.request_id == other.request_id &&
        self.api_url == other.api_url &&
        self.unknown6 == other.unknown6 &&
        self.auth_ticket == other.auth_ticket &&
        self.returns == other.returns &&
        self.error == other.error &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ResponseEnvelope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ResponseEnvelope_Unknown7 {
    // message fields
    unknown71: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown72: ::std::option::Option<i64>,
    unknown73: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseEnvelope_Unknown7 {}

impl ResponseEnvelope_Unknown7 {
    pub fn new() -> ResponseEnvelope_Unknown7 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseEnvelope_Unknown7 {
        static mut instance: ::protobuf::lazy::Lazy<ResponseEnvelope_Unknown7> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseEnvelope_Unknown7,
        };
        unsafe {
            instance.get(|| {
                ResponseEnvelope_Unknown7 {
                    unknown71: ::protobuf::SingularField::none(),
                    unknown72: ::std::option::Option::None,
                    unknown73: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes unknown71 = 1;

    pub fn clear_unknown71(&mut self) {
        self.unknown71.clear();
    }

    pub fn has_unknown71(&self) -> bool {
        self.unknown71.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unknown71(&mut self, v: ::std::vec::Vec<u8>) {
        self.unknown71 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unknown71(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.unknown71.is_none() {
            self.unknown71.set_default();
        };
        self.unknown71.as_mut().unwrap()
    }

    // Take field
    pub fn take_unknown71(&mut self) -> ::std::vec::Vec<u8> {
        self.unknown71.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_unknown71(&self) -> &[u8] {
        match self.unknown71.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional int64 unknown72 = 2;

    pub fn clear_unknown72(&mut self) {
        self.unknown72 = ::std::option::Option::None;
    }

    pub fn has_unknown72(&self) -> bool {
        self.unknown72.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unknown72(&mut self, v: i64) {
        self.unknown72 = ::std::option::Option::Some(v);
    }

    pub fn get_unknown72(&self) -> i64 {
        self.unknown72.unwrap_or(0)
    }

    // optional bytes unknown73 = 3;

    pub fn clear_unknown73(&mut self) {
        self.unknown73.clear();
    }

    pub fn has_unknown73(&self) -> bool {
        self.unknown73.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unknown73(&mut self, v: ::std::vec::Vec<u8>) {
        self.unknown73 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unknown73(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.unknown73.is_none() {
            self.unknown73.set_default();
        };
        self.unknown73.as_mut().unwrap()
    }

    // Take field
    pub fn take_unknown73(&mut self) -> ::std::vec::Vec<u8> {
        self.unknown73.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_unknown73(&self) -> &[u8] {
        match self.unknown73.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for ResponseEnvelope_Unknown7 {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.unknown71));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.unknown72 = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.unknown73));
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
        for value in &self.unknown71 {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.unknown72 {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.unknown73 {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.unknown71.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.unknown72 {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.unknown73.as_ref() {
            try!(os.write_bytes(3, &v));
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
        ::std::any::TypeId::of::<ResponseEnvelope_Unknown7>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseEnvelope_Unknown7 {
    fn new() -> ResponseEnvelope_Unknown7 {
        ResponseEnvelope_Unknown7::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseEnvelope_Unknown7>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "unknown71",
                    ResponseEnvelope_Unknown7::has_unknown71,
                    ResponseEnvelope_Unknown7::get_unknown71,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "unknown72",
                    ResponseEnvelope_Unknown7::has_unknown72,
                    ResponseEnvelope_Unknown7::get_unknown72,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "unknown73",
                    ResponseEnvelope_Unknown7::has_unknown73,
                    ResponseEnvelope_Unknown7::get_unknown73,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseEnvelope_Unknown7>(
                    "ResponseEnvelope_Unknown7",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseEnvelope_Unknown7 {
    fn clear(&mut self) {
        self.clear_unknown71();
        self.clear_unknown72();
        self.clear_unknown73();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ResponseEnvelope_Unknown7 {
    fn eq(&self, other: &ResponseEnvelope_Unknown7) -> bool {
        self.unknown71 == other.unknown71 &&
        self.unknown72 == other.unknown72 &&
        self.unknown73 == other.unknown73 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ResponseEnvelope_Unknown7 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x25, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1f, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45,
    0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x1a, 0x1c, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65,
    0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x24, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x00, 0x50,
    0x01, 0x50, 0x02, 0x22, 0x64, 0x0a, 0x0a, 0x41, 0x75, 0x74, 0x68, 0x54, 0x69, 0x63, 0x6b, 0x65,
    0x74, 0x12, 0x14, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x72, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x05, 0x73, 0x74, 0x61, 0x72, 0x74, 0x12, 0x2e, 0x0a, 0x13, 0x65, 0x78, 0x70, 0x69, 0x72,
    0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x11, 0x65, 0x78, 0x70, 0x69, 0x72, 0x65, 0x54, 0x69, 0x6d, 0x65,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12, 0x10, 0x0a, 0x03, 0x65, 0x6e, 0x64, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x65, 0x6e, 0x64, 0x22, 0xc1, 0x06, 0x0a, 0x10, 0x55, 0x6e,
    0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x36, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x23,
    0x0a, 0x0d, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x56, 0x0a, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x32, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x3a, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e,
    0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x36,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e,
    0x32, 0x52, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x32, 0x1a, 0xaf, 0x05, 0x0a, 0x08,
    0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x32, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x6e, 0x6b, 0x6e,
    0x6f, 0x77, 0x6e, 0x31, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x75, 0x6e, 0x6b, 0x6e,
    0x6f, 0x77, 0x6e, 0x31, 0x12, 0x5a, 0x0a, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x02, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x44, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65,
    0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x36, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x32, 0x2e,
    0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x52, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73,
    0x12, 0x4d, 0x0a, 0x11, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x63, 0x75, 0x72, 0x72, 0x65,
    0x6e, 0x63, 0x69, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x2e, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x52, 0x10, 0x70,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x69, 0x65, 0x73, 0x12,
    0x1a, 0x0a, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x34, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x34, 0x1a, 0xbf, 0x03, 0x0a, 0x09,
    0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x12, 0x17, 0x0a, 0x07, 0x69, 0x74, 0x65,
    0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x69, 0x74, 0x65, 0x6d,
    0x49, 0x64, 0x12, 0x15, 0x0a, 0x06, 0x69, 0x73, 0x5f, 0x69, 0x61, 0x70, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x05, 0x69, 0x73, 0x49, 0x61, 0x70, 0x12, 0x48, 0x0a, 0x0f, 0x63, 0x75, 0x72,
    0x72, 0x65, 0x6e, 0x63, 0x79, 0x5f, 0x74, 0x6f, 0x5f, 0x62, 0x75, 0x79, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x20, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x43, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x63, 0x79, 0x52, 0x0d, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x54, 0x6f,
    0x42, 0x75, 0x79, 0x12, 0x49, 0x0a, 0x0f, 0x79, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x5f, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x52, 0x0e,
    0x79, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x12, 0x44,
    0x0a, 0x0b, 0x79, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x5f, 0x69, 0x74, 0x65, 0x6d, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e,
    0x49, 0x74, 0x65, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x52, 0x0a, 0x79, 0x69, 0x65, 0x6c, 0x64, 0x73,
    0x49, 0x74, 0x65, 0x6d, 0x12, 0x5c, 0x0a, 0x04, 0x74, 0x61, 0x67, 0x73, 0x18, 0x06, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x48, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c,
    0x6f, 0x70, 0x65, 0x73, 0x2e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x36, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x32, 0x2e, 0x53,
    0x74, 0x6f, 0x72, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x54, 0x61, 0x67, 0x52, 0x04, 0x74, 0x61,
    0x67, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x37, 0x18, 0x07,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x37, 0x1a, 0x2d,
    0x0a, 0x03, 0x54, 0x61, 0x67, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xb4, 0x05,
    0x0a, 0x0f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70,
    0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x63, 0x6f, 0x64, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x43, 0x6f,
    0x64, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x69, 0x64,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x49,
    0x64, 0x12, 0x43, 0x0a, 0x08, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x18, 0x04, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x08, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x12, 0x45, 0x0a, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77,
    0x6e, 0x36, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x29, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67,
    0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f,
    0x77, 0x6e, 0x36, 0x52, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x36, 0x12, 0x1a, 0x0a,
    0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x01, 0x52,
    0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x6c, 0x6f, 0x6e,
    0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x01, 0x52, 0x09, 0x6c, 0x6f,
    0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x61, 0x6c, 0x74, 0x69, 0x74,
    0x75, 0x64, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x01, 0x52, 0x08, 0x61, 0x6c, 0x74, 0x69, 0x74,
    0x75, 0x64, 0x65, 0x12, 0x56, 0x0a, 0x09, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x69, 0x6e, 0x66, 0x6f,
    0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x39, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45,
    0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x49, 0x6e, 0x66,
    0x6f, 0x52, 0x08, 0x61, 0x75, 0x74, 0x68, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x4c, 0x0a, 0x0b, 0x61,
    0x75, 0x74, 0x68, 0x5f, 0x74, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x2b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65,
    0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70,
    0x65, 0x73, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x0a, 0x61,
    0x75, 0x74, 0x68, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x1c, 0x0a, 0x09, 0x75, 0x6e, 0x6b,
    0x6e, 0x6f, 0x77, 0x6e, 0x31, 0x32, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x03, 0x52, 0x09, 0x75, 0x6e,
    0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x31, 0x32, 0x1a, 0xba, 0x01, 0x0a, 0x08, 0x41, 0x75, 0x74, 0x68,
    0x49, 0x6e, 0x66, 0x6f, 0x12, 0x1a, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x72,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x72,
    0x12, 0x53, 0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x3d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65,
    0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70,
    0x65, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x4a, 0x57, 0x54, 0x52, 0x05,
    0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x1a, 0x3d, 0x0a, 0x03, 0x4a, 0x57, 0x54, 0x12, 0x1a, 0x0a, 0x08,
    0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08,
    0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x6e, 0x6b, 0x6e,
    0x6f, 0x77, 0x6e, 0x32, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x75, 0x6e, 0x6b, 0x6e,
    0x6f, 0x77, 0x6e, 0x32, 0x22, 0xa5, 0x01, 0x0a, 0x08, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e,
    0x36, 0x12, 0x21, 0x0a, 0x0c, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x4e, 0x0a, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x32,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45,
    0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e,
    0x36, 0x2e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x32, 0x52, 0x08, 0x75, 0x6e, 0x6b, 0x6e,
    0x6f, 0x77, 0x6e, 0x32, 0x1a, 0x26, 0x0a, 0x08, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x32,
    0x12, 0x1a, 0x0a, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x31, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x31, 0x22, 0x9e, 0x03, 0x0a,
    0x10, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70,
    0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x63, 0x6f, 0x64, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x43, 0x6f,
    0x64, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x49,
    0x64, 0x12, 0x17, 0x0a, 0x07, 0x61, 0x70, 0x69, 0x5f, 0x75, 0x72, 0x6c, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x06, 0x61, 0x70, 0x69, 0x55, 0x72, 0x6c, 0x12, 0x4d, 0x0a, 0x08, 0x75, 0x6e,
    0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x36, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x31, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72,
    0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x55,
    0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x36, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52,
    0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x36, 0x12, 0x4c, 0x0a, 0x0b, 0x61, 0x75, 0x74,
    0x68, 0x5f, 0x74, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77,
    0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73,
    0x2e, 0x41, 0x75, 0x74, 0x68, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x0a, 0x61, 0x75, 0x74,
    0x68, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x72, 0x65, 0x74, 0x75, 0x72,
    0x6e, 0x73, 0x18, 0x64, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x07, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e,
    0x73, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x65, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x1a, 0x64, 0x0a, 0x08, 0x55, 0x6e, 0x6b, 0x6e, 0x6f,
    0x77, 0x6e, 0x37, 0x12, 0x1c, 0x0a, 0x09, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x37, 0x31,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x37,
    0x31, 0x12, 0x1c, 0x0a, 0x09, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x37, 0x32, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x03, 0x52, 0x09, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x37, 0x32, 0x12,
    0x1c, 0x0a, 0x09, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x37, 0x33, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x09, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x37, 0x33, 0x4a, 0xa8, 0x25,
    0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x56, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x27, 0x0a, 0x09, 0x0a, 0x02,
    0x0a, 0x00, 0x12, 0x03, 0x03, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03,
    0x0e, 0x2c, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x04, 0x07, 0x0d, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x0e, 0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x02, 0x12, 0x03,
    0x05, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x05, 0x0e, 0x34, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x07, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x08, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x08, 0x08,
    0x07, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x08, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x0e, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x09, 0x08, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x09, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x09, 0x0f, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x09,
    0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0a, 0x08, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x0a, 0x08, 0x09, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0a, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x0e, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x0c, 0x00, 0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x18,
    0x0a, 0x51, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x08, 0x20, 0x22, 0x44, 0x20,
    0x53, 0x74, 0x69, 0x6c, 0x6c, 0x20, 0x64, 0x6f, 0x6e, 0x27, 0x74, 0x20, 0x6b, 0x6e, 0x6f, 0x77,
    0x20, 0x77, 0x68, 0x61, 0x74, 0x20, 0x36, 0x20, 0x69, 0x73, 0x2c, 0x20, 0x62, 0x75, 0x74, 0x20,
    0x35, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x73, 0x20, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x20, 0x61, 0x76,
    0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x76, 0x69, 0x61, 0x20, 0x49, 0x41, 0x50, 0x73,
    0x2e, 0x20, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0d, 0x08,
    0x0c, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x0e, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x1e, 0x1f, 0x0a, 0x1c, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x1e, 0x22, 0x0f, 0x20, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x20, 0x64, 0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x0e, 0x08, 0x0d, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x0e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0e, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0e, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x03, 0x00, 0x12, 0x04, 0x10, 0x08, 0x24,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x01, 0x12, 0x03, 0x10, 0x10, 0x18, 0x0a,
    0x4e, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x11, 0x10, 0x24, 0x22, 0x3f,
    0x20, 0x4d, 0x61, 0x79, 0x62, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x3f, 0x20, 0x49,
    0x74, 0x27, 0x73, 0x20, 0x61, 0x6c, 0x77, 0x61, 0x79, 0x73, 0x20, 0x31, 0x20, 0x28, 0x73, 0x75,
    0x63, 0x63, 0x65, 0x73, 0x73, 0x29, 0x2c, 0x20, 0x73, 0x6f, 0x20, 0x69, 0x74, 0x27, 0x73, 0x20,
    0x70, 0x72, 0x6f, 0x62, 0x61, 0x62, 0x6c, 0x79, 0x20, 0x74, 0x68, 0x61, 0x74, 0x2e, 0x0a, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x11, 0x10, 0x10, 0x1a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x11, 0x10, 0x16,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x17, 0x1f,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x11, 0x22, 0x23,
    0x0a, 0x2a, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x12, 0x10, 0x2d, 0x22,
    0x1b, 0x20, 0x49, 0x74, 0x65, 0x6d, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x68, 0x6f, 0x77, 0x20,
    0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x68, 0x6f, 0x70, 0x0a, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x12, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x12, 0x19, 0x22, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x12, 0x23, 0x28, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x12, 0x2b, 0x2c, 0x0a, 0x39, 0x0a, 0x06,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x13, 0x10, 0x50, 0x22, 0x2a, 0x20, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x63, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x20, 0x68, 0x61, 0x73, 0x20, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6d, 0x6f, 0x6d, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x13, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x02, 0x06, 0x12, 0x03, 0x13, 0x19, 0x39, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x13, 0x3a, 0x4b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x13, 0x4e, 0x4f, 0x0a, 0x2d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x14, 0x10, 0x24, 0x22, 0x1e, 0x20, 0x53, 0x6f, 0x6d, 0x65, 0x20, 0x62, 0x61,
    0x73, 0x65, 0x36, 0x34, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x64, 0x20, 0x73, 0x74, 0x75,
    0x66, 0x66, 0x2e, 0x2e, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x03,
    0x04, 0x12, 0x04, 0x14, 0x10, 0x13, 0x50, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x14, 0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x14, 0x17, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x14, 0x22, 0x23, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x00, 0x12, 0x04, 0x16, 0x10, 0x23, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x00, 0x01, 0x12, 0x03, 0x16, 0x18, 0x21, 0x0a, 0x6b, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x17, 0x18, 0x2b, 0x22, 0x5a, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72,
    0x6e, 0x61, 0x6c, 0x20, 0x49, 0x44, 0x20, 0x28, 0x70, 0x72, 0x6f, 0x62, 0x61, 0x62, 0x6c, 0x79,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x47, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x20, 0x50, 0x6c, 0x61, 0x79,
    0x2f, 0x41, 0x70, 0x70, 0x20, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x29, 0x20, 0x65, 0x78, 0x61, 0x6d,
    0x70, 0x6c, 0x65, 0x3a, 0x20, 0x22, 0x70, 0x67, 0x6f, 0x72, 0x65, 0x6c, 0x65, 0x61, 0x73, 0x65,
    0x2e, 0x69, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x72, 0x79,
    0x2e, 0x31, 0x22, 0x0a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x17, 0x18, 0x16, 0x23, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x17, 0x18, 0x1e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17, 0x1f, 0x26, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x17, 0x29, 0x2a, 0x0a, 0x7e, 0x0a,
    0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x18, 0x18, 0x28, 0x22, 0x6d,
    0x20, 0x49, 0x66, 0x20, 0x74, 0x72, 0x75, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69,
    0x74, 0x65, 0x6d, 0x20, 0x69, 0x73, 0x20, 0x62, 0x6f, 0x75, 0x67, 0x68, 0x74, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x72, 0x65, 0x61, 0x6c, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79,
    0x20, 0x28, 0x55, 0x53, 0x44, 0x2c, 0x20, 0x65, 0x74, 0x63, 0x2e, 0x29, 0x20, 0x74, 0x68, 0x72,
    0x6f, 0x75, 0x67, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x50, 0x6c, 0x61, 0x79, 0x2f, 0x41, 0x70,
    0x70, 0x20, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x65, 0x61, 0x64, 0x20,
    0x6f, 0x66, 0x20, 0x50, 0x6f, 0x6b, 0x65, 0x63, 0x6f, 0x69, 0x6e, 0x73, 0x0a, 0x0a, 0x11, 0x0a,
    0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x18, 0x18, 0x17, 0x2b,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x18,
    0x18, 0x1c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x18, 0x1d, 0x23, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x18, 0x26, 0x27, 0x0a, 0xa2, 0x01, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x19, 0x18, 0x4d, 0x22, 0x90, 0x01, 0x20, 0x54, 0x68, 0x69, 0x73,
    0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x68, 0x6f, 0x77, 0x20, 0x6d, 0x75, 0x63,
    0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x74, 0x65, 0x6d, 0x20, 0x63, 0x6f, 0x73, 0x74, 0x73,
    0x20, 0x28, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x63, 0x65, 0x70,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x63, 0x6f, 0x73, 0x74, 0x20, 0x72, 0x65, 0x61, 0x6c, 0x20, 0x6d, 0x6f, 0x6e,
    0x65, 0x79, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x50, 0x6f, 0x6b, 0x65, 0x63, 0x6f, 0x69, 0x6e,
    0x73, 0x2c, 0x20, 0x74, 0x68, 0x61, 0x74, 0x27, 0x73, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65,
    0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x65, 0x63, 0x74,
    0x69, 0x76, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x29, 0x0a, 0x0a, 0x11, 0x0a, 0x09, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x19, 0x18, 0x18, 0x28, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x19, 0x18, 0x38,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x19,
    0x39, 0x48, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x19, 0x4b, 0x4c, 0x0a, 0x46, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x1a, 0x18, 0x4d, 0x22, 0x35, 0x20, 0x57, 0x68, 0x65, 0x6e, 0x20, 0x62, 0x6f, 0x75,
    0x67, 0x68, 0x74, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x49, 0x41, 0x50, 0x20, 0x77, 0x69,
    0x6c, 0x6c, 0x20, 0x79, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x75,
    0x63, 0x68, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x0a, 0x0a, 0x11, 0x0a, 0x09,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x1a, 0x18, 0x19, 0x4d, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x1a, 0x18,
    0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x1a, 0x39, 0x48, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x1a, 0x4b, 0x4c, 0x0a, 0x4b, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02,
    0x04, 0x12, 0x03, 0x1b, 0x18, 0x4c, 0x22, 0x3a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x69, 0x74, 0x65,
    0x6d, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x73,
    0x75, 0x63, 0x68, 0x20, 0x69, 0x74, 0x65, 0x6d, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x49, 0x41, 0x50, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x79, 0x69, 0x65, 0x6c,
    0x64, 0x0a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x1b, 0x18, 0x1a, 0x4d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02,
    0x04, 0x06, 0x12, 0x03, 0x1b, 0x18, 0x3b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1b, 0x3c, 0x47, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1b, 0x4a, 0x4b, 0x0a, 0x35, 0x0a, 0x08, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x05, 0x12, 0x03, 0x1c, 0x18, 0x2e, 0x22, 0x24, 0x20, 0x53,
    0x74, 0x75, 0x66, 0x66, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x53, 0x4f, 0x52, 0x54, 0x3a, 0x31,
    0x32, 0x2c, 0x20, 0x43, 0x41, 0x54, 0x45, 0x47, 0x4f, 0x52, 0x59, 0x3a, 0x49, 0x54, 0x45, 0x4d,
    0x53, 0x0a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x05, 0x04, 0x12,
    0x03, 0x1c, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x05,
    0x06, 0x12, 0x03, 0x1c, 0x21, 0x24, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x1c, 0x25, 0x29, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1c, 0x2c, 0x2d, 0x0a, 0x57, 0x0a, 0x08, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x00, 0x02, 0x06, 0x12, 0x03, 0x1d, 0x18, 0x2b, 0x22, 0x46, 0x20, 0x50, 0x6f,
    0x73, 0x73, 0x69, 0x62, 0x6c, 0x79, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x74, 0x68, 0x69, 0x6e, 0x67,
    0x20, 0x74, 0x6f, 0x20, 0x74, 0x6f, 0x67, 0x67, 0x6c, 0x65, 0x20, 0x76, 0x69, 0x73, 0x69, 0x62,
    0x69, 0x6c, 0x69, 0x74, 0x79, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x6f,
    0x72, 0x65, 0x2f, 0x70, 0x75, 0x72, 0x63, 0x68, 0x61, 0x73, 0x69, 0x62, 0x69, 0x6c, 0x69, 0x74,
    0x79, 0x3f, 0x0a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x06, 0x04,
    0x12, 0x04, 0x1d, 0x18, 0x1c, 0x2e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x06, 0x05, 0x12, 0x03, 0x1d, 0x18, 0x1d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1d, 0x1e, 0x26, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x1d, 0x29, 0x2a, 0x0a, 0x10, 0x0a, 0x08,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x12, 0x04, 0x1f, 0x18, 0x22, 0x19, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x20, 0x23,
    0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x20, 0x20, 0x2f, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x20, 0x20, 0x1f, 0x25, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x20, 0x20, 0x26, 0x0a, 0x12, 0x0a, 0x0b,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x27, 0x2a,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x20, 0x2d, 0x2e, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x21, 0x20, 0x31, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x00, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x21, 0x20, 0x20, 0x2f, 0x0a, 0x12, 0x0a, 0x0b,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x21, 0x20, 0x26,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x21, 0x27, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x21, 0x2f, 0x30, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04,
    0x26, 0x00, 0x3d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x26, 0x08, 0x17,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x27, 0x08, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x27, 0x08, 0x26, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x27, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x27, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x27, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x29, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x29, 0x08,
    0x27, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x29, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x0f, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x29, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x2a, 0x08, 0x46, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x2a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06,
    0x12, 0x03, 0x2a, 0x11, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x2a, 0x39, 0x41, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2a, 0x44,
    0x45, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2c, 0x08, 0x48, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x03, 0x06, 0x12, 0x03, 0x2c, 0x11, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x2c, 0x3b, 0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x2c, 0x46, 0x47, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x2d, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2d, 0x08,
    0x2c, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x2d, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2d, 0x0f, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2d, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2e, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x05, 0x04, 0x12, 0x04, 0x2e, 0x08, 0x2d, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x2e, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x2e, 0x0f, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x2e,
    0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2f, 0x08, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x04, 0x12, 0x04, 0x2f, 0x08, 0x2e, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x05, 0x12, 0x03, 0x2f, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x2f, 0x0f, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x06, 0x03, 0x12, 0x03, 0x2f, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x07,
    0x12, 0x03, 0x30, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x04, 0x12, 0x04,
    0x30, 0x08, 0x2f, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x06, 0x12, 0x03, 0x30,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x30, 0x11, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x30, 0x1d, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x08, 0x12, 0x03, 0x31, 0x08, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x08, 0x04, 0x12, 0x04, 0x31, 0x08, 0x30, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x08, 0x06, 0x12, 0x03, 0x31, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08,
    0x01, 0x12, 0x03, 0x31, 0x34, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x03, 0x12,
    0x03, 0x31, 0x42, 0x44, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x09, 0x12, 0x03, 0x32, 0x08,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x04, 0x12, 0x04, 0x32, 0x08, 0x31, 0x45,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x05, 0x12, 0x03, 0x32, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x01, 0x12, 0x03, 0x32, 0x0e, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x09, 0x03, 0x12, 0x03, 0x32, 0x1a, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02,
    0x03, 0x00, 0x12, 0x04, 0x34, 0x08, 0x3c, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x03, 0x00,
    0x01, 0x12, 0x03, 0x34, 0x10, 0x18, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x35, 0x10, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x35, 0x10, 0x34, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x35, 0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x35, 0x17, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x35, 0x22, 0x23, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x36, 0x10, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x36, 0x10, 0x35, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x36, 0x10, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x36, 0x14, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x36, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00,
    0x12, 0x04, 0x38, 0x10, 0x3b, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00,
    0x01, 0x12, 0x03, 0x38, 0x18, 0x1b, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x39, 0x18, 0x2c, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x39, 0x18, 0x38, 0x1d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02,
    0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x39, 0x18, 0x1e, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x39, 0x1f, 0x27, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x39, 0x2a, 0x2b,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x18,
    0x2c, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x3a, 0x18, 0x39, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x3a, 0x18, 0x1d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x3a, 0x1e, 0x26, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00,
    0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3a, 0x2a, 0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x3e, 0x00, 0x45, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x3e,
    0x08, 0x10, 0x0a, 0x2d, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x08, 0x1f, 0x22,
    0x20, 0x20, 0x35, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x49, 0x41, 0x50, 0x73, 0x2c, 0x20, 0x36, 0x20,
    0x69, 0x73, 0x20, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x20, 0x73, 0x74, 0x69, 0x6c, 0x6c,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x3f, 0x08, 0x3e, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3f, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3f, 0x0e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3f, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x01, 0x12, 0x03, 0x40, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x40, 0x08, 0x3f, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x40, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x40,
    0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x40, 0x1c, 0x1d,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x03, 0x00, 0x12, 0x04, 0x42, 0x08, 0x44, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x03, 0x00, 0x01, 0x12, 0x03, 0x42, 0x10, 0x18, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x43, 0x10, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x03, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x43, 0x10, 0x42, 0x1a, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x43, 0x10, 0x15, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x16, 0x1e, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x43, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x46, 0x00, 0x56, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12,
    0x03, 0x46, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x47, 0x08,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x47, 0x08, 0x46, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x47, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x47, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x47, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x48, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x48, 0x08, 0x47, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x48, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x48,
    0x0f, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x48, 0x1c, 0x1d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x49, 0x08, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x04, 0x49, 0x08, 0x48, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x49, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x49, 0x0f, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x49, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03,
    0x4b, 0x08, 0x50, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x4b, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x06, 0x12, 0x03, 0x4b, 0x11, 0x42, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4b, 0x43, 0x4b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4b, 0x4e, 0x4f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x04, 0x12, 0x03, 0x4c, 0x08, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04,
    0x04, 0x12, 0x04, 0x4c, 0x08, 0x4b, 0x50, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x06,
    0x12, 0x03, 0x4c, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x4c, 0x34, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12, 0x03, 0x4c, 0x42,
    0x43, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x03, 0x4e, 0x08, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x04, 0x12, 0x03, 0x4e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x05, 0x05, 0x12, 0x03, 0x4e, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x4e, 0x17, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x4e, 0x21, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x06, 0x12, 0x03,
    0x4f, 0x08, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x04, 0x12, 0x04, 0x4f, 0x08,
    0x4e, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x05, 0x12, 0x03, 0x4f, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x01, 0x12, 0x03, 0x4f, 0x0f, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x03, 0x12, 0x03, 0x4f, 0x17, 0x1a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x04, 0x03, 0x00, 0x12, 0x04, 0x51, 0x08, 0x55, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x03, 0x00, 0x01, 0x12, 0x03, 0x51, 0x10, 0x18, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x52, 0x10, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x52, 0x10, 0x51, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x52, 0x10, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x52, 0x16, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x52, 0x22, 0x23, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x53, 0x10, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x53, 0x10, 0x52, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x53, 0x10, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x53, 0x16, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x53, 0x22, 0x23, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x54, 0x10, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x04, 0x54, 0x10, 0x53, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x54, 0x10, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x54, 0x16, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x54, 0x22, 0x23, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
