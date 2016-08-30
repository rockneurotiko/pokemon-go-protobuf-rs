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
pub struct SendEncryptedSignatureResponse {
    // message fields
    received: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SendEncryptedSignatureResponse {}

impl SendEncryptedSignatureResponse {
    pub fn new() -> SendEncryptedSignatureResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SendEncryptedSignatureResponse {
        static mut instance: ::protobuf::lazy::Lazy<SendEncryptedSignatureResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SendEncryptedSignatureResponse,
        };
        unsafe {
            instance.get(|| {
                SendEncryptedSignatureResponse {
                    received: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool received = 1;

    pub fn clear_received(&mut self) {
        self.received = ::std::option::Option::None;
    }

    pub fn has_received(&self) -> bool {
        self.received.is_some()
    }

    // Param is passed by value, moved
    pub fn set_received(&mut self, v: bool) {
        self.received = ::std::option::Option::Some(v);
    }

    pub fn get_received(&self) -> bool {
        self.received.unwrap_or(false)
    }
}

impl ::protobuf::Message for SendEncryptedSignatureResponse {
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
                    self.received = ::std::option::Option::Some(tmp);
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
        if self.received.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.received {
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
        ::std::any::TypeId::of::<SendEncryptedSignatureResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SendEncryptedSignatureResponse {
    fn new() -> SendEncryptedSignatureResponse {
        SendEncryptedSignatureResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SendEncryptedSignatureResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "received",
                    SendEncryptedSignatureResponse::has_received,
                    SendEncryptedSignatureResponse::get_received,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SendEncryptedSignatureResponse>(
                    "SendEncryptedSignatureResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SendEncryptedSignatureResponse {
    fn clear(&mut self) {
        self.clear_received();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SendEncryptedSignatureResponse {
    fn eq(&self, other: &SendEncryptedSignatureResponse) -> bool {
        self.received == other.received &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SendEncryptedSignatureResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BuyItemAndroidResponse {
    // message fields
    result: ::std::option::Option<BuyItemAndroidResponse_Status>,
    purchase_token: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BuyItemAndroidResponse {}

impl BuyItemAndroidResponse {
    pub fn new() -> BuyItemAndroidResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BuyItemAndroidResponse {
        static mut instance: ::protobuf::lazy::Lazy<BuyItemAndroidResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BuyItemAndroidResponse,
        };
        unsafe {
            instance.get(|| {
                BuyItemAndroidResponse {
                    result: ::std::option::Option::None,
                    purchase_token: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Platform.Responses.BuyItemAndroidResponse.Status result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: BuyItemAndroidResponse_Status) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> BuyItemAndroidResponse_Status {
        self.result.unwrap_or(BuyItemAndroidResponse_Status::UNKNOWN)
    }

    // optional string purchase_token = 2;

    pub fn clear_purchase_token(&mut self) {
        self.purchase_token.clear();
    }

    pub fn has_purchase_token(&self) -> bool {
        self.purchase_token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_purchase_token(&mut self, v: ::std::string::String) {
        self.purchase_token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_purchase_token(&mut self) -> &mut ::std::string::String {
        if self.purchase_token.is_none() {
            self.purchase_token.set_default();
        };
        self.purchase_token.as_mut().unwrap()
    }

    // Take field
    pub fn take_purchase_token(&mut self) -> ::std::string::String {
        self.purchase_token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_purchase_token(&self) -> &str {
        match self.purchase_token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for BuyItemAndroidResponse {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.purchase_token));
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
        for value in &self.purchase_token {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.purchase_token.as_ref() {
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
        ::std::any::TypeId::of::<BuyItemAndroidResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BuyItemAndroidResponse {
    fn new() -> BuyItemAndroidResponse {
        BuyItemAndroidResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<BuyItemAndroidResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    BuyItemAndroidResponse::has_result,
                    BuyItemAndroidResponse::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "purchase_token",
                    BuyItemAndroidResponse::has_purchase_token,
                    BuyItemAndroidResponse::get_purchase_token,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BuyItemAndroidResponse>(
                    "BuyItemAndroidResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BuyItemAndroidResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_purchase_token();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BuyItemAndroidResponse {
    fn eq(&self, other: &BuyItemAndroidResponse) -> bool {
        self.result == other.result &&
        self.purchase_token == other.purchase_token &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BuyItemAndroidResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum BuyItemAndroidResponse_Status {
    UNKNOWN = 0,
    SUCCESS = 1,
}

impl ::protobuf::ProtobufEnum for BuyItemAndroidResponse_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<BuyItemAndroidResponse_Status> {
        match value {
            0 => ::std::option::Option::Some(BuyItemAndroidResponse_Status::UNKNOWN),
            1 => ::std::option::Option::Some(BuyItemAndroidResponse_Status::SUCCESS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [BuyItemAndroidResponse_Status] = &[
            BuyItemAndroidResponse_Status::UNKNOWN,
            BuyItemAndroidResponse_Status::SUCCESS,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<BuyItemAndroidResponse_Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("BuyItemAndroidResponse_Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for BuyItemAndroidResponse_Status {
}

#[derive(Clone,Default)]
pub struct GetStoreItemsResponse {
    // message fields
    status: ::std::option::Option<GetStoreItemsResponse_Status>,
    items: ::protobuf::RepeatedField<GetStoreItemsResponse_StoreItem>,
    player_currencies: ::protobuf::RepeatedField<super::POGOProtos_Data_Player::Currency>,
    unknown4: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetStoreItemsResponse {}

impl GetStoreItemsResponse {
    pub fn new() -> GetStoreItemsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetStoreItemsResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetStoreItemsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetStoreItemsResponse,
        };
        unsafe {
            instance.get(|| {
                GetStoreItemsResponse {
                    status: ::std::option::Option::None,
                    items: ::protobuf::RepeatedField::new(),
                    player_currencies: ::protobuf::RepeatedField::new(),
                    unknown4: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Platform.Responses.GetStoreItemsResponse.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: GetStoreItemsResponse_Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> GetStoreItemsResponse_Status {
        self.status.unwrap_or(GetStoreItemsResponse_Status::UNDEFINED)
    }

    // repeated .POGOProtos.Networking.Platform.Responses.GetStoreItemsResponse.StoreItem items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<GetStoreItemsResponse_StoreItem>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<GetStoreItemsResponse_StoreItem> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<GetStoreItemsResponse_StoreItem> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[GetStoreItemsResponse_StoreItem] {
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

impl ::protobuf::Message for GetStoreItemsResponse {
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
        for value in &self.status {
            my_size += ::protobuf::rt::enum_size(1, *value);
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
        if let Some(v) = self.status {
            try!(os.write_enum(1, v.value()));
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
        ::std::any::TypeId::of::<GetStoreItemsResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetStoreItemsResponse {
    fn new() -> GetStoreItemsResponse {
        GetStoreItemsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetStoreItemsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    GetStoreItemsResponse::has_status,
                    GetStoreItemsResponse::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "items",
                    GetStoreItemsResponse::get_items,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "player_currencies",
                    GetStoreItemsResponse::get_player_currencies,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "unknown4",
                    GetStoreItemsResponse::has_unknown4,
                    GetStoreItemsResponse::get_unknown4,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetStoreItemsResponse>(
                    "GetStoreItemsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetStoreItemsResponse {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_items();
        self.clear_player_currencies();
        self.clear_unknown4();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetStoreItemsResponse {
    fn eq(&self, other: &GetStoreItemsResponse) -> bool {
        self.status == other.status &&
        self.items == other.items &&
        self.player_currencies == other.player_currencies &&
        self.unknown4 == other.unknown4 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetStoreItemsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetStoreItemsResponse_StoreItem {
    // message fields
    item_id: ::protobuf::SingularField<::std::string::String>,
    is_iap: ::std::option::Option<bool>,
    currency_to_buy: ::protobuf::SingularPtrField<super::POGOProtos_Data_Player::Currency>,
    yields_currency: ::protobuf::SingularPtrField<super::POGOProtos_Data_Player::Currency>,
    yields_item: ::protobuf::SingularPtrField<super::POGOProtos_Inventory_Item::ItemData>,
    tags: ::protobuf::RepeatedField<GetStoreItemsResponse_StoreItem_TagsEntry>,
    unknown7: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetStoreItemsResponse_StoreItem {}

impl GetStoreItemsResponse_StoreItem {
    pub fn new() -> GetStoreItemsResponse_StoreItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetStoreItemsResponse_StoreItem {
        static mut instance: ::protobuf::lazy::Lazy<GetStoreItemsResponse_StoreItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetStoreItemsResponse_StoreItem,
        };
        unsafe {
            instance.get(|| {
                GetStoreItemsResponse_StoreItem {
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

    // repeated .POGOProtos.Networking.Platform.Responses.GetStoreItemsResponse.StoreItem.TagsEntry tags = 6;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::protobuf::RepeatedField<GetStoreItemsResponse_StoreItem_TagsEntry>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags(&mut self) -> &mut ::protobuf::RepeatedField<GetStoreItemsResponse_StoreItem_TagsEntry> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::protobuf::RepeatedField<GetStoreItemsResponse_StoreItem_TagsEntry> {
        ::std::mem::replace(&mut self.tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_tags(&self) -> &[GetStoreItemsResponse_StoreItem_TagsEntry] {
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

impl ::protobuf::Message for GetStoreItemsResponse_StoreItem {
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
        ::std::any::TypeId::of::<GetStoreItemsResponse_StoreItem>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetStoreItemsResponse_StoreItem {
    fn new() -> GetStoreItemsResponse_StoreItem {
        GetStoreItemsResponse_StoreItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetStoreItemsResponse_StoreItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "item_id",
                    GetStoreItemsResponse_StoreItem::has_item_id,
                    GetStoreItemsResponse_StoreItem::get_item_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_iap",
                    GetStoreItemsResponse_StoreItem::has_is_iap,
                    GetStoreItemsResponse_StoreItem::get_is_iap,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "currency_to_buy",
                    GetStoreItemsResponse_StoreItem::has_currency_to_buy,
                    GetStoreItemsResponse_StoreItem::get_currency_to_buy,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "yields_currency",
                    GetStoreItemsResponse_StoreItem::has_yields_currency,
                    GetStoreItemsResponse_StoreItem::get_yields_currency,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "yields_item",
                    GetStoreItemsResponse_StoreItem::has_yields_item,
                    GetStoreItemsResponse_StoreItem::get_yields_item,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "tags",
                    GetStoreItemsResponse_StoreItem::get_tags,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "unknown7",
                    GetStoreItemsResponse_StoreItem::has_unknown7,
                    GetStoreItemsResponse_StoreItem::get_unknown7,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetStoreItemsResponse_StoreItem>(
                    "GetStoreItemsResponse_StoreItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetStoreItemsResponse_StoreItem {
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

impl ::std::cmp::PartialEq for GetStoreItemsResponse_StoreItem {
    fn eq(&self, other: &GetStoreItemsResponse_StoreItem) -> bool {
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

impl ::std::fmt::Debug for GetStoreItemsResponse_StoreItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetStoreItemsResponse_StoreItem_TagsEntry {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetStoreItemsResponse_StoreItem_TagsEntry {}

impl GetStoreItemsResponse_StoreItem_TagsEntry {
    pub fn new() -> GetStoreItemsResponse_StoreItem_TagsEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetStoreItemsResponse_StoreItem_TagsEntry {
        static mut instance: ::protobuf::lazy::Lazy<GetStoreItemsResponse_StoreItem_TagsEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetStoreItemsResponse_StoreItem_TagsEntry,
        };
        unsafe {
            instance.get(|| {
                GetStoreItemsResponse_StoreItem_TagsEntry {
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

impl ::protobuf::Message for GetStoreItemsResponse_StoreItem_TagsEntry {
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
        ::std::any::TypeId::of::<GetStoreItemsResponse_StoreItem_TagsEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetStoreItemsResponse_StoreItem_TagsEntry {
    fn new() -> GetStoreItemsResponse_StoreItem_TagsEntry {
        GetStoreItemsResponse_StoreItem_TagsEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetStoreItemsResponse_StoreItem_TagsEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    GetStoreItemsResponse_StoreItem_TagsEntry::has_key,
                    GetStoreItemsResponse_StoreItem_TagsEntry::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "value",
                    GetStoreItemsResponse_StoreItem_TagsEntry::has_value,
                    GetStoreItemsResponse_StoreItem_TagsEntry::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetStoreItemsResponse_StoreItem_TagsEntry>(
                    "GetStoreItemsResponse_StoreItem_TagsEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetStoreItemsResponse_StoreItem_TagsEntry {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetStoreItemsResponse_StoreItem_TagsEntry {
    fn eq(&self, other: &GetStoreItemsResponse_StoreItem_TagsEntry) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetStoreItemsResponse_StoreItem_TagsEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum GetStoreItemsResponse_Status {
    UNDEFINED = 0,
    OKAY = 1,
}

impl ::protobuf::ProtobufEnum for GetStoreItemsResponse_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<GetStoreItemsResponse_Status> {
        match value {
            0 => ::std::option::Option::Some(GetStoreItemsResponse_Status::UNDEFINED),
            1 => ::std::option::Option::Some(GetStoreItemsResponse_Status::OKAY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [GetStoreItemsResponse_Status] = &[
            GetStoreItemsResponse_Status::UNDEFINED,
            GetStoreItemsResponse_Status::OKAY,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<GetStoreItemsResponse_Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("GetStoreItemsResponse_Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for GetStoreItemsResponse_Status {
}

#[derive(Clone,Default)]
pub struct BuyItemPokeCoinsResponse {
    // message fields
    result: ::std::option::Option<BuyItemPokeCoinsResponse_Status>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BuyItemPokeCoinsResponse {}

impl BuyItemPokeCoinsResponse {
    pub fn new() -> BuyItemPokeCoinsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BuyItemPokeCoinsResponse {
        static mut instance: ::protobuf::lazy::Lazy<BuyItemPokeCoinsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BuyItemPokeCoinsResponse,
        };
        unsafe {
            instance.get(|| {
                BuyItemPokeCoinsResponse {
                    result: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Networking.Platform.Responses.BuyItemPokeCoinsResponse.Status result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: BuyItemPokeCoinsResponse_Status) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> BuyItemPokeCoinsResponse_Status {
        self.result.unwrap_or(BuyItemPokeCoinsResponse_Status::UNKNOWN)
    }
}

impl ::protobuf::Message for BuyItemPokeCoinsResponse {
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
        ::std::any::TypeId::of::<BuyItemPokeCoinsResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BuyItemPokeCoinsResponse {
    fn new() -> BuyItemPokeCoinsResponse {
        BuyItemPokeCoinsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<BuyItemPokeCoinsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    BuyItemPokeCoinsResponse::has_result,
                    BuyItemPokeCoinsResponse::get_result,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BuyItemPokeCoinsResponse>(
                    "BuyItemPokeCoinsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BuyItemPokeCoinsResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BuyItemPokeCoinsResponse {
    fn eq(&self, other: &BuyItemPokeCoinsResponse) -> bool {
        self.result == other.result &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BuyItemPokeCoinsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum BuyItemPokeCoinsResponse_Status {
    UNKNOWN = 0,
    SUCCESS = 1,
    NOT_ENOUGH_POKECOINS = 3,
}

impl ::protobuf::ProtobufEnum for BuyItemPokeCoinsResponse_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<BuyItemPokeCoinsResponse_Status> {
        match value {
            0 => ::std::option::Option::Some(BuyItemPokeCoinsResponse_Status::UNKNOWN),
            1 => ::std::option::Option::Some(BuyItemPokeCoinsResponse_Status::SUCCESS),
            3 => ::std::option::Option::Some(BuyItemPokeCoinsResponse_Status::NOT_ENOUGH_POKECOINS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [BuyItemPokeCoinsResponse_Status] = &[
            BuyItemPokeCoinsResponse_Status::UNKNOWN,
            BuyItemPokeCoinsResponse_Status::SUCCESS,
            BuyItemPokeCoinsResponse_Status::NOT_ENOUGH_POKECOINS,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<BuyItemPokeCoinsResponse_Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("BuyItemPokeCoinsResponse_Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for BuyItemPokeCoinsResponse_Status {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d,
    0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x28, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d,
    0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x1a, 0x1c, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49,
    0x74, 0x65, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x00, 0x50, 0x01, 0x22, 0x3c, 0x0a,
    0x1e, 0x53, 0x65, 0x6e, 0x64, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x53, 0x69,
    0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x1a, 0x0a, 0x08, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x08, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x64, 0x22, 0xc4, 0x01, 0x0a, 0x16,
    0x42, 0x75, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x6e, 0x64, 0x72, 0x6f, 0x69, 0x64, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x5f, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x47, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x50,
    0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x73, 0x2e, 0x42, 0x75, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x6e, 0x64, 0x72, 0x6f, 0x69, 0x64,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52,
    0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x25, 0x0a, 0x0e, 0x70, 0x75, 0x72, 0x63, 0x68,
    0x61, 0x73, 0x65, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0d, 0x70, 0x75, 0x72, 0x63, 0x68, 0x61, 0x73, 0x65, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x22, 0x22,
    0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x0b, 0x0a, 0x07, 0x55, 0x4e, 0x4b, 0x4e,
    0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53,
    0x10, 0x01, 0x22, 0xbd, 0x06, 0x0a, 0x15, 0x47, 0x65, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49,
    0x74, 0x65, 0x6d, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x5e, 0x0a, 0x06,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x46, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72,
    0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x2e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65,
    0x49, 0x74, 0x65, 0x6d, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x53, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x5f, 0x0a, 0x05,
    0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x49, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x69, 0x6e, 0x67, 0x2e, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x2e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49,
    0x74, 0x65, 0x6d, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x53, 0x74, 0x6f,
    0x72, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x52, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x12, 0x4d, 0x0a,
    0x11, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x69,
    0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x2e, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x52, 0x10, 0x70, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x69, 0x65, 0x73, 0x12, 0x1a, 0x0a, 0x08,
    0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x34, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08,
    0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x34, 0x1a, 0xd4, 0x03, 0x0a, 0x09, 0x53, 0x74, 0x6f,
    0x72, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x12, 0x17, 0x0a, 0x07, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x69, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x12,
    0x15, 0x0a, 0x06, 0x69, 0x73, 0x5f, 0x69, 0x61, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x05, 0x69, 0x73, 0x49, 0x61, 0x70, 0x12, 0x48, 0x0a, 0x0f, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e,
    0x63, 0x79, 0x5f, 0x74, 0x6f, 0x5f, 0x62, 0x75, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x20, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63,
    0x79, 0x52, 0x0d, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x54, 0x6f, 0x42, 0x75, 0x79,
    0x12, 0x49, 0x0a, 0x0f, 0x79, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x5f, 0x63, 0x75, 0x72, 0x72, 0x65,
    0x6e, 0x63, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x2e, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x52, 0x0e, 0x79, 0x69, 0x65,
    0x6c, 0x64, 0x73, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x12, 0x44, 0x0a, 0x0b, 0x79,
    0x69, 0x65, 0x6c, 0x64, 0x73, 0x5f, 0x69, 0x74, 0x65, 0x6d, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x23, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e,
    0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65,
    0x6d, 0x44, 0x61, 0x74, 0x61, 0x52, 0x0a, 0x79, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x49, 0x74, 0x65,
    0x6d, 0x12, 0x67, 0x0a, 0x04, 0x74, 0x61, 0x67, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x53, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d,
    0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x53, 0x74,
    0x6f, 0x72, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x2e, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x54, 0x61, 0x67, 0x73, 0x45,
    0x6e, 0x74, 0x72, 0x79, 0x52, 0x04, 0x74, 0x61, 0x67, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x6e,
    0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x37, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x75, 0x6e,
    0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x37, 0x1a, 0x37, 0x0a, 0x09, 0x54, 0x61, 0x67, 0x73, 0x45, 0x6e,
    0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x22,
    0x21, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x0d, 0x0a, 0x09, 0x55, 0x4e, 0x44,
    0x45, 0x46, 0x49, 0x4e, 0x45, 0x44, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x4f, 0x4b, 0x41, 0x59,
    0x10, 0x01, 0x22, 0xbb, 0x01, 0x0a, 0x18, 0x42, 0x75, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x50, 0x6f,
    0x6b, 0x65, 0x43, 0x6f, 0x69, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x61, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x49, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d,
    0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x42, 0x75, 0x79, 0x49, 0x74,
    0x65, 0x6d, 0x50, 0x6f, 0x6b, 0x65, 0x43, 0x6f, 0x69, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x22, 0x3c, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x0b, 0x0a, 0x07,
    0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43,
    0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x18, 0x0a, 0x14, 0x4e, 0x4f, 0x54, 0x5f, 0x45, 0x4e,
    0x4f, 0x55, 0x47, 0x48, 0x5f, 0x50, 0x4f, 0x4b, 0x45, 0x43, 0x4f, 0x49, 0x4e, 0x53, 0x10, 0x03,
    0x4a, 0xd8, 0x14, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x36, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c,
    0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x30, 0x0a,
    0x09, 0x0a, 0x02, 0x0a, 0x00, 0x12, 0x03, 0x03, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00,
    0x12, 0x03, 0x03, 0x0e, 0x2c, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x04, 0x07, 0x0d,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x0e, 0x2f, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x06, 0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x06, 0x08, 0x26, 0x0a, 0x2b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x08, 0x1a,
    0x22, 0x1e, 0x20, 0x73, 0x70, 0x65, 0x63, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x64, 0x2c, 0x20, 0x63,
    0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x75, 0x6d, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x07, 0x08, 0x06, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x07, 0x08, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x0b, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08,
    0x1e, 0x0a, 0x15, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x04, 0x16, 0x22, 0x08,
    0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x0c, 0x04, 0x0b, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x0c, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x14,
    0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x04, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0d, 0x04, 0x0c, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0d, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x0b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x0d, 0x1c, 0x1d, 0x0a, 0x3e, 0x0a, 0x04, 0x04, 0x01, 0x04, 0x00, 0x12,
    0x04, 0x10, 0x04, 0x13, 0x05, 0x1a, 0x30, 0x20, 0x54, 0x48, 0x45, 0x53, 0x45, 0x20, 0x41, 0x52,
    0x45, 0x20, 0x53, 0x4f, 0x4d, 0x45, 0x57, 0x48, 0x41, 0x54, 0x20, 0x53, 0x50, 0x45, 0x43, 0x55,
    0x4c, 0x41, 0x54, 0x45, 0x44, 0x2c, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x20, 0x6d, 0x61,
    0x79, 0x20, 0x62, 0x65, 0x20, 0x32, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x10, 0x09, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x11, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x11, 0x08, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x11, 0x12, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x12, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x12, 0x08, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x12, 0x12, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x17, 0x00, 0x2b, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x17, 0x08, 0x1d, 0x0a, 0x19, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x00, 0x12, 0x03, 0x18, 0x08, 0x1a, 0x22, 0x0c, 0x75, 0x6e, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x72, 0x6d, 0x65, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x18, 0x08, 0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x18, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x0f,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x18, 0x19, 0x0a,
    0x28, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x19, 0x08, 0x25, 0x22, 0x1b, 0x20, 0x49,
    0x74, 0x65, 0x6d, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x68, 0x6f, 0x77, 0x20, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x68, 0x6f, 0x70, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x19, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x19, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x19, 0x1b, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x19, 0x23,
    0x24, 0x0a, 0x37, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x08, 0x48, 0x22, 0x2a,
    0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x20, 0x68, 0x61, 0x73, 0x20, 0x61, 0x74, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6d, 0x6f, 0x6d, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x1a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x1a, 0x11, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x1a, 0x32, 0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1a,
    0x46, 0x47, 0x0a, 0x40, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x08, 0x1c, 0x22,
    0x33, 0x20, 0x53, 0x6f, 0x6d, 0x65, 0x20, 0x62, 0x61, 0x73, 0x65, 0x36, 0x34, 0x20, 0x65, 0x6e,
    0x63, 0x6f, 0x64, 0x65, 0x64, 0x20, 0x73, 0x74, 0x75, 0x66, 0x66, 0x2e, 0x2e, 0x2e, 0x20, 0x28,
    0x44, 0x65, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x72, 0x20, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61,
    0x64, 0x3f, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x04, 0x1b,
    0x08, 0x1a, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x1b, 0x08,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1b, 0x0f, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1b, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x02, 0x03, 0x00, 0x12, 0x04, 0x1d, 0x08, 0x25, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x03, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x10, 0x19, 0x0a, 0x69, 0x0a, 0x06, 0x04, 0x02, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x10, 0x23, 0x22, 0x5a, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72,
    0x6e, 0x61, 0x6c, 0x20, 0x49, 0x44, 0x20, 0x28, 0x70, 0x72, 0x6f, 0x62, 0x61, 0x62, 0x6c, 0x79,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x47, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x20, 0x50, 0x6c, 0x61, 0x79,
    0x2f, 0x41, 0x70, 0x70, 0x20, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x29, 0x20, 0x65, 0x78, 0x61, 0x6d,
    0x70, 0x6c, 0x65, 0x3a, 0x20, 0x22, 0x70, 0x67, 0x6f, 0x72, 0x65, 0x6c, 0x65, 0x61, 0x73, 0x65,
    0x2e, 0x69, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x72, 0x79,
    0x2e, 0x31, 0x22, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x1e, 0x10, 0x1d, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x1e, 0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x1e, 0x17, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x1e, 0x21, 0x22, 0x0a, 0x7c, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x1f, 0x10, 0x20, 0x22, 0x6d, 0x20, 0x49, 0x66, 0x20, 0x74, 0x72, 0x75, 0x65, 0x2c, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x74, 0x65, 0x6d, 0x20, 0x69, 0x73, 0x20, 0x62, 0x6f, 0x75,
    0x67, 0x68, 0x74, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x72, 0x65, 0x61, 0x6c, 0x20, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x20, 0x28, 0x55, 0x53, 0x44, 0x2c, 0x20, 0x65, 0x74, 0x63,
    0x2e, 0x29, 0x20, 0x74, 0x68, 0x72, 0x6f, 0x75, 0x67, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x50,
    0x6c, 0x61, 0x79, 0x2f, 0x41, 0x70, 0x70, 0x20, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x6e,
    0x73, 0x74, 0x65, 0x61, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x50, 0x6f, 0x6b, 0x65, 0x63, 0x6f, 0x69,
    0x6e, 0x73, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x1f, 0x10, 0x1e, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x1f, 0x10, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x1f, 0x15, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x1f, 0x1e, 0x1f, 0x0a, 0xa0, 0x01, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x20, 0x10, 0x45, 0x22, 0x90, 0x01, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x64, 0x65, 0x66,
    0x69, 0x6e, 0x65, 0x73, 0x20, 0x68, 0x6f, 0x77, 0x20, 0x6d, 0x75, 0x63, 0x68, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x69, 0x74, 0x65, 0x6d, 0x20, 0x63, 0x6f, 0x73, 0x74, 0x73, 0x20, 0x28, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x63, 0x65, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x6f, 0x66, 0x20, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x63,
    0x6f, 0x73, 0x74, 0x20, 0x72, 0x65, 0x61, 0x6c, 0x20, 0x6d, 0x6f, 0x6e, 0x65, 0x79, 0x20, 0x6c,
    0x69, 0x6b, 0x65, 0x20, 0x50, 0x6f, 0x6b, 0x65, 0x63, 0x6f, 0x69, 0x6e, 0x73, 0x2c, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x27, 0x73, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x69, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x20,
    0x73, 0x74, 0x6f, 0x72, 0x65, 0x29, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x04, 0x20, 0x10, 0x1f, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00,
    0x02, 0x02, 0x06, 0x12, 0x03, 0x20, 0x10, 0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x20, 0x31, 0x40, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x20, 0x43, 0x44, 0x0a, 0x44, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x21, 0x10, 0x45, 0x22, 0x35, 0x20, 0x57, 0x68, 0x65, 0x6e, 0x20, 0x62,
    0x6f, 0x75, 0x67, 0x68, 0x74, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x49, 0x41, 0x50, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x79, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
    0x6d, 0x75, 0x63, 0x68, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x0a, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x21, 0x10, 0x20, 0x45, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x21, 0x10, 0x30, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x21, 0x31, 0x40, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x21, 0x43, 0x44, 0x0a,
    0x49, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x04, 0x12, 0x03, 0x22, 0x10, 0x44, 0x22, 0x3a,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x69, 0x74, 0x65, 0x6d, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x75, 0x63, 0x68, 0x20, 0x69, 0x74, 0x65, 0x6d,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x49, 0x41, 0x50, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x79, 0x69, 0x65, 0x6c, 0x64, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x02,
    0x03, 0x00, 0x02, 0x04, 0x04, 0x12, 0x04, 0x22, 0x10, 0x21, 0x45, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x22, 0x10, 0x33, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x22, 0x34, 0x3f, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x22, 0x42, 0x43, 0x0a, 0x33, 0x0a, 0x06, 0x04,
    0x02, 0x03, 0x00, 0x02, 0x05, 0x12, 0x03, 0x23, 0x10, 0x2d, 0x22, 0x24, 0x20, 0x53, 0x74, 0x75,
    0x66, 0x66, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x53, 0x4f, 0x52, 0x54, 0x3a, 0x31, 0x32, 0x2c,
    0x20, 0x43, 0x41, 0x54, 0x45, 0x47, 0x4f, 0x52, 0x59, 0x3a, 0x49, 0x54, 0x45, 0x4d, 0x53, 0x0a,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x05, 0x04, 0x12, 0x04, 0x23, 0x10, 0x22,
    0x44, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x23, 0x10,
    0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x23, 0x24,
    0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x23, 0x2b,
    0x2c, 0x0a, 0x55, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x06, 0x12, 0x03, 0x24, 0x10, 0x23,
    0x22, 0x46, 0x20, 0x50, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x79, 0x20, 0x73, 0x6f, 0x6d, 0x65,
    0x74, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x6f, 0x67, 0x67, 0x6c, 0x65, 0x20,
    0x76, 0x69, 0x73, 0x69, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2f, 0x70, 0x75, 0x72, 0x63, 0x68, 0x61, 0x73, 0x69,
    0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x3f, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00,
    0x02, 0x06, 0x04, 0x12, 0x04, 0x24, 0x10, 0x23, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03,
    0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x24, 0x10, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03,
    0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x24, 0x16, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03,
    0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x24, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x27, 0x08, 0x2a, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x27, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x28, 0x10, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x28, 0x10, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x28, 0x1c, 0x1d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x29, 0x10, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x29, 0x10, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x29, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x2d, 0x00, 0x36, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x2d, 0x08, 0x20, 0x0a, 0x15, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x2e, 0x04, 0x16, 0x22, 0x08, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x2e, 0x04, 0x2d,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2e, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x14, 0x15, 0x0a, 0x73, 0x0a, 0x04, 0x04,
    0x03, 0x04, 0x00, 0x12, 0x04, 0x31, 0x04, 0x35, 0x05, 0x1a, 0x65, 0x20, 0x54, 0x48, 0x45, 0x53,
    0x45, 0x20, 0x41, 0x52, 0x45, 0x20, 0x53, 0x4f, 0x4d, 0x45, 0x57, 0x48, 0x41, 0x54, 0x20, 0x53,
    0x50, 0x45, 0x43, 0x55, 0x4c, 0x41, 0x54, 0x45, 0x44, 0x2c, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c,
    0x64, 0x20, 0x62, 0x65, 0x20, 0x61, 0x20, 0x4e, 0x4f, 0x54, 0x20, 0x45, 0x4e, 0x4f, 0x55, 0x47,
    0x48, 0x20, 0x52, 0x4f, 0x4f, 0x4d, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x74, 0x6f,
    0x6f, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x62, 0x61, 0x67, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20,
    0x66, 0x75, 0x6c, 0x6c, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x77, 0x68, 0x65, 0x72, 0x65, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x31, 0x09, 0x0f, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x32, 0x08, 0x14, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x32, 0x08, 0x0f, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x32, 0x12, 0x13, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x33, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x33, 0x08, 0x0f, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x33, 0x12, 0x13, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x34, 0x08, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x03, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x34, 0x08, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x03, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x34, 0x1f, 0x20, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
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
