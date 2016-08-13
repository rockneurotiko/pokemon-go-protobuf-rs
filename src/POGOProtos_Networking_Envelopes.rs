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
pub struct Signature {
    // message fields
    timestamp_since_start: ::std::option::Option<u64>,
    location_fix: ::protobuf::RepeatedField<Signature_LocationFix>,
    gps_info: ::protobuf::SingularPtrField<Signature_AndroidGpsInfo>,
    sensor_info: ::protobuf::SingularPtrField<Signature_SensorInfo>,
    device_info: ::protobuf::SingularPtrField<Signature_DeviceInfo>,
    activity_status: ::protobuf::SingularPtrField<Signature_ActivityStatus>,
    location_hash1: ::std::option::Option<u64>,
    location_hash2: ::std::option::Option<u64>,
    session_hash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timestamp: ::std::option::Option<u64>,
    request_hash: ::std::vec::Vec<i64>,
    unknown25: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Signature {}

impl Signature {
    pub fn new() -> Signature {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Signature {
        static mut instance: ::protobuf::lazy::Lazy<Signature> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Signature,
        };
        unsafe {
            instance.get(|| {
                Signature {
                    timestamp_since_start: ::std::option::Option::None,
                    location_fix: ::protobuf::RepeatedField::new(),
                    gps_info: ::protobuf::SingularPtrField::none(),
                    sensor_info: ::protobuf::SingularPtrField::none(),
                    device_info: ::protobuf::SingularPtrField::none(),
                    activity_status: ::protobuf::SingularPtrField::none(),
                    location_hash1: ::std::option::Option::None,
                    location_hash2: ::std::option::Option::None,
                    session_hash: ::protobuf::SingularField::none(),
                    timestamp: ::std::option::Option::None,
                    request_hash: ::std::vec::Vec::new(),
                    unknown25: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 timestamp_since_start = 2;

    pub fn clear_timestamp_since_start(&mut self) {
        self.timestamp_since_start = ::std::option::Option::None;
    }

    pub fn has_timestamp_since_start(&self) -> bool {
        self.timestamp_since_start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp_since_start(&mut self, v: u64) {
        self.timestamp_since_start = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp_since_start(&self) -> u64 {
        self.timestamp_since_start.unwrap_or(0)
    }

    // repeated .POGOProtos.Networking.Envelopes.Signature.LocationFix location_fix = 4;

    pub fn clear_location_fix(&mut self) {
        self.location_fix.clear();
    }

    // Param is passed by value, moved
    pub fn set_location_fix(&mut self, v: ::protobuf::RepeatedField<Signature_LocationFix>) {
        self.location_fix = v;
    }

    // Mutable pointer to the field.
    pub fn mut_location_fix(&mut self) -> &mut ::protobuf::RepeatedField<Signature_LocationFix> {
        &mut self.location_fix
    }

    // Take field
    pub fn take_location_fix(&mut self) -> ::protobuf::RepeatedField<Signature_LocationFix> {
        ::std::mem::replace(&mut self.location_fix, ::protobuf::RepeatedField::new())
    }

    pub fn get_location_fix(&self) -> &[Signature_LocationFix] {
        &self.location_fix
    }

    // optional .POGOProtos.Networking.Envelopes.Signature.AndroidGpsInfo gps_info = 5;

    pub fn clear_gps_info(&mut self) {
        self.gps_info.clear();
    }

    pub fn has_gps_info(&self) -> bool {
        self.gps_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gps_info(&mut self, v: Signature_AndroidGpsInfo) {
        self.gps_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gps_info(&mut self) -> &mut Signature_AndroidGpsInfo {
        if self.gps_info.is_none() {
            self.gps_info.set_default();
        };
        self.gps_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_gps_info(&mut self) -> Signature_AndroidGpsInfo {
        self.gps_info.take().unwrap_or_else(|| Signature_AndroidGpsInfo::new())
    }

    pub fn get_gps_info(&self) -> &Signature_AndroidGpsInfo {
        self.gps_info.as_ref().unwrap_or_else(|| Signature_AndroidGpsInfo::default_instance())
    }

    // optional .POGOProtos.Networking.Envelopes.Signature.SensorInfo sensor_info = 7;

    pub fn clear_sensor_info(&mut self) {
        self.sensor_info.clear();
    }

    pub fn has_sensor_info(&self) -> bool {
        self.sensor_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sensor_info(&mut self, v: Signature_SensorInfo) {
        self.sensor_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sensor_info(&mut self) -> &mut Signature_SensorInfo {
        if self.sensor_info.is_none() {
            self.sensor_info.set_default();
        };
        self.sensor_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_sensor_info(&mut self) -> Signature_SensorInfo {
        self.sensor_info.take().unwrap_or_else(|| Signature_SensorInfo::new())
    }

    pub fn get_sensor_info(&self) -> &Signature_SensorInfo {
        self.sensor_info.as_ref().unwrap_or_else(|| Signature_SensorInfo::default_instance())
    }

    // optional .POGOProtos.Networking.Envelopes.Signature.DeviceInfo device_info = 8;

    pub fn clear_device_info(&mut self) {
        self.device_info.clear();
    }

    pub fn has_device_info(&self) -> bool {
        self.device_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_info(&mut self, v: Signature_DeviceInfo) {
        self.device_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_info(&mut self) -> &mut Signature_DeviceInfo {
        if self.device_info.is_none() {
            self.device_info.set_default();
        };
        self.device_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_info(&mut self) -> Signature_DeviceInfo {
        self.device_info.take().unwrap_or_else(|| Signature_DeviceInfo::new())
    }

    pub fn get_device_info(&self) -> &Signature_DeviceInfo {
        self.device_info.as_ref().unwrap_or_else(|| Signature_DeviceInfo::default_instance())
    }

    // optional .POGOProtos.Networking.Envelopes.Signature.ActivityStatus activity_status = 9;

    pub fn clear_activity_status(&mut self) {
        self.activity_status.clear();
    }

    pub fn has_activity_status(&self) -> bool {
        self.activity_status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_activity_status(&mut self, v: Signature_ActivityStatus) {
        self.activity_status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_activity_status(&mut self) -> &mut Signature_ActivityStatus {
        if self.activity_status.is_none() {
            self.activity_status.set_default();
        };
        self.activity_status.as_mut().unwrap()
    }

    // Take field
    pub fn take_activity_status(&mut self) -> Signature_ActivityStatus {
        self.activity_status.take().unwrap_or_else(|| Signature_ActivityStatus::new())
    }

    pub fn get_activity_status(&self) -> &Signature_ActivityStatus {
        self.activity_status.as_ref().unwrap_or_else(|| Signature_ActivityStatus::default_instance())
    }

    // optional uint64 location_hash1 = 10;

    pub fn clear_location_hash1(&mut self) {
        self.location_hash1 = ::std::option::Option::None;
    }

    pub fn has_location_hash1(&self) -> bool {
        self.location_hash1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location_hash1(&mut self, v: u64) {
        self.location_hash1 = ::std::option::Option::Some(v);
    }

    pub fn get_location_hash1(&self) -> u64 {
        self.location_hash1.unwrap_or(0)
    }

    // optional uint64 location_hash2 = 20;

    pub fn clear_location_hash2(&mut self) {
        self.location_hash2 = ::std::option::Option::None;
    }

    pub fn has_location_hash2(&self) -> bool {
        self.location_hash2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location_hash2(&mut self, v: u64) {
        self.location_hash2 = ::std::option::Option::Some(v);
    }

    pub fn get_location_hash2(&self) -> u64 {
        self.location_hash2.unwrap_or(0)
    }

    // optional bytes session_hash = 22;

    pub fn clear_session_hash(&mut self) {
        self.session_hash.clear();
    }

    pub fn has_session_hash(&self) -> bool {
        self.session_hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_session_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.session_hash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_session_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.session_hash.is_none() {
            self.session_hash.set_default();
        };
        self.session_hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_session_hash(&mut self) -> ::std::vec::Vec<u8> {
        self.session_hash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_session_hash(&self) -> &[u8] {
        match self.session_hash.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint64 timestamp = 23;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp.unwrap_or(0)
    }

    // repeated int64 request_hash = 24;

    pub fn clear_request_hash(&mut self) {
        self.request_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_request_hash(&mut self, v: ::std::vec::Vec<i64>) {
        self.request_hash = v;
    }

    // Mutable pointer to the field.
    pub fn mut_request_hash(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.request_hash
    }

    // Take field
    pub fn take_request_hash(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.request_hash, ::std::vec::Vec::new())
    }

    pub fn get_request_hash(&self) -> &[i64] {
        &self.request_hash
    }

    // optional int64 unknown25 = 25;

    pub fn clear_unknown25(&mut self) {
        self.unknown25 = ::std::option::Option::None;
    }

    pub fn has_unknown25(&self) -> bool {
        self.unknown25.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unknown25(&mut self, v: i64) {
        self.unknown25 = ::std::option::Option::Some(v);
    }

    pub fn get_unknown25(&self) -> i64 {
        self.unknown25.unwrap_or(0)
    }
}

impl ::protobuf::Message for Signature {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.timestamp_since_start = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.location_fix));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.gps_info));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sensor_info));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.device_info));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.activity_status));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.location_hash1 = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.location_hash2 = ::std::option::Option::Some(tmp);
                },
                22 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.session_hash));
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                24 => {
                    try!(::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.request_hash));
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.unknown25 = ::std::option::Option::Some(tmp);
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
        for value in &self.timestamp_since_start {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.location_fix {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.gps_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.sensor_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.device_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.activity_status {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.location_hash1 {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.location_hash2 {
            my_size += ::protobuf::rt::value_size(20, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.session_hash {
            my_size += ::protobuf::rt::bytes_size(22, &value);
        };
        for value in &self.timestamp {
            my_size += ::protobuf::rt::value_size(23, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.request_hash {
            my_size += ::protobuf::rt::value_size(24, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.unknown25 {
            my_size += ::protobuf::rt::value_size(25, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.timestamp_since_start {
            try!(os.write_uint64(2, v));
        };
        for v in &self.location_fix {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.gps_info.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.sensor_info.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.device_info.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.activity_status.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.location_hash1 {
            try!(os.write_uint64(10, v));
        };
        if let Some(v) = self.location_hash2 {
            try!(os.write_uint64(20, v));
        };
        if let Some(v) = self.session_hash.as_ref() {
            try!(os.write_bytes(22, &v));
        };
        if let Some(v) = self.timestamp {
            try!(os.write_uint64(23, v));
        };
        for v in &self.request_hash {
            try!(os.write_int64(24, *v));
        };
        if let Some(v) = self.unknown25 {
            try!(os.write_int64(25, v));
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
        ::std::any::TypeId::of::<Signature>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Signature {
    fn new() -> Signature {
        Signature::new()
    }

    fn descriptor_static(_: ::std::option::Option<Signature>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "timestamp_since_start",
                    Signature::has_timestamp_since_start,
                    Signature::get_timestamp_since_start,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "location_fix",
                    Signature::get_location_fix,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "gps_info",
                    Signature::has_gps_info,
                    Signature::get_gps_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "sensor_info",
                    Signature::has_sensor_info,
                    Signature::get_sensor_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "device_info",
                    Signature::has_device_info,
                    Signature::get_device_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "activity_status",
                    Signature::has_activity_status,
                    Signature::get_activity_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "location_hash1",
                    Signature::has_location_hash1,
                    Signature::get_location_hash1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "location_hash2",
                    Signature::has_location_hash2,
                    Signature::get_location_hash2,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "session_hash",
                    Signature::has_session_hash,
                    Signature::get_session_hash,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "timestamp",
                    Signature::has_timestamp,
                    Signature::get_timestamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "request_hash",
                    Signature::get_request_hash,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "unknown25",
                    Signature::has_unknown25,
                    Signature::get_unknown25,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Signature>(
                    "Signature",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Signature {
    fn clear(&mut self) {
        self.clear_timestamp_since_start();
        self.clear_location_fix();
        self.clear_gps_info();
        self.clear_sensor_info();
        self.clear_device_info();
        self.clear_activity_status();
        self.clear_location_hash1();
        self.clear_location_hash2();
        self.clear_session_hash();
        self.clear_timestamp();
        self.clear_request_hash();
        self.clear_unknown25();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Signature {
    fn eq(&self, other: &Signature) -> bool {
        self.timestamp_since_start == other.timestamp_since_start &&
        self.location_fix == other.location_fix &&
        self.gps_info == other.gps_info &&
        self.sensor_info == other.sensor_info &&
        self.device_info == other.device_info &&
        self.activity_status == other.activity_status &&
        self.location_hash1 == other.location_hash1 &&
        self.location_hash2 == other.location_hash2 &&
        self.session_hash == other.session_hash &&
        self.timestamp == other.timestamp &&
        self.request_hash == other.request_hash &&
        self.unknown25 == other.unknown25 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Signature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Signature_LocationFix {
    // message fields
    provider: ::protobuf::SingularField<::std::string::String>,
    timestamp_snapshot: ::std::option::Option<u64>,
    latitude: ::std::option::Option<f32>,
    longitude: ::std::option::Option<f32>,
    horizontal_accuracy: ::std::option::Option<f32>,
    altitude: ::std::option::Option<f32>,
    vertical_accuracy: ::std::option::Option<f32>,
    provider_status: ::std::option::Option<u64>,
    floor: ::std::option::Option<u32>,
    location_type: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Signature_LocationFix {}

impl Signature_LocationFix {
    pub fn new() -> Signature_LocationFix {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Signature_LocationFix {
        static mut instance: ::protobuf::lazy::Lazy<Signature_LocationFix> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Signature_LocationFix,
        };
        unsafe {
            instance.get(|| {
                Signature_LocationFix {
                    provider: ::protobuf::SingularField::none(),
                    timestamp_snapshot: ::std::option::Option::None,
                    latitude: ::std::option::Option::None,
                    longitude: ::std::option::Option::None,
                    horizontal_accuracy: ::std::option::Option::None,
                    altitude: ::std::option::Option::None,
                    vertical_accuracy: ::std::option::Option::None,
                    provider_status: ::std::option::Option::None,
                    floor: ::std::option::Option::None,
                    location_type: ::std::option::Option::None,
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

    // optional uint64 timestamp_snapshot = 2;

    pub fn clear_timestamp_snapshot(&mut self) {
        self.timestamp_snapshot = ::std::option::Option::None;
    }

    pub fn has_timestamp_snapshot(&self) -> bool {
        self.timestamp_snapshot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp_snapshot(&mut self, v: u64) {
        self.timestamp_snapshot = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp_snapshot(&self) -> u64 {
        self.timestamp_snapshot.unwrap_or(0)
    }

    // optional float latitude = 13;

    pub fn clear_latitude(&mut self) {
        self.latitude = ::std::option::Option::None;
    }

    pub fn has_latitude(&self) -> bool {
        self.latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latitude(&mut self, v: f32) {
        self.latitude = ::std::option::Option::Some(v);
    }

    pub fn get_latitude(&self) -> f32 {
        self.latitude.unwrap_or(0.)
    }

    // optional float longitude = 14;

    pub fn clear_longitude(&mut self) {
        self.longitude = ::std::option::Option::None;
    }

    pub fn has_longitude(&self) -> bool {
        self.longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_longitude(&mut self, v: f32) {
        self.longitude = ::std::option::Option::Some(v);
    }

    pub fn get_longitude(&self) -> f32 {
        self.longitude.unwrap_or(0.)
    }

    // optional float horizontal_accuracy = 20;

    pub fn clear_horizontal_accuracy(&mut self) {
        self.horizontal_accuracy = ::std::option::Option::None;
    }

    pub fn has_horizontal_accuracy(&self) -> bool {
        self.horizontal_accuracy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_horizontal_accuracy(&mut self, v: f32) {
        self.horizontal_accuracy = ::std::option::Option::Some(v);
    }

    pub fn get_horizontal_accuracy(&self) -> f32 {
        self.horizontal_accuracy.unwrap_or(0.)
    }

    // optional float altitude = 21;

    pub fn clear_altitude(&mut self) {
        self.altitude = ::std::option::Option::None;
    }

    pub fn has_altitude(&self) -> bool {
        self.altitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_altitude(&mut self, v: f32) {
        self.altitude = ::std::option::Option::Some(v);
    }

    pub fn get_altitude(&self) -> f32 {
        self.altitude.unwrap_or(0.)
    }

    // optional float vertical_accuracy = 22;

    pub fn clear_vertical_accuracy(&mut self) {
        self.vertical_accuracy = ::std::option::Option::None;
    }

    pub fn has_vertical_accuracy(&self) -> bool {
        self.vertical_accuracy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vertical_accuracy(&mut self, v: f32) {
        self.vertical_accuracy = ::std::option::Option::Some(v);
    }

    pub fn get_vertical_accuracy(&self) -> f32 {
        self.vertical_accuracy.unwrap_or(0.)
    }

    // optional uint64 provider_status = 26;

    pub fn clear_provider_status(&mut self) {
        self.provider_status = ::std::option::Option::None;
    }

    pub fn has_provider_status(&self) -> bool {
        self.provider_status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_provider_status(&mut self, v: u64) {
        self.provider_status = ::std::option::Option::Some(v);
    }

    pub fn get_provider_status(&self) -> u64 {
        self.provider_status.unwrap_or(0)
    }

    // optional uint32 floor = 27;

    pub fn clear_floor(&mut self) {
        self.floor = ::std::option::Option::None;
    }

    pub fn has_floor(&self) -> bool {
        self.floor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_floor(&mut self, v: u32) {
        self.floor = ::std::option::Option::Some(v);
    }

    pub fn get_floor(&self) -> u32 {
        self.floor.unwrap_or(0)
    }

    // optional uint64 location_type = 28;

    pub fn clear_location_type(&mut self) {
        self.location_type = ::std::option::Option::None;
    }

    pub fn has_location_type(&self) -> bool {
        self.location_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location_type(&mut self, v: u64) {
        self.location_type = ::std::option::Option::Some(v);
    }

    pub fn get_location_type(&self) -> u64 {
        self.location_type.unwrap_or(0)
    }
}

impl ::protobuf::Message for Signature_LocationFix {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.timestamp_snapshot = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.latitude = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.longitude = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.horizontal_accuracy = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.altitude = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.vertical_accuracy = ::std::option::Option::Some(tmp);
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.provider_status = ::std::option::Option::Some(tmp);
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.floor = ::std::option::Option::Some(tmp);
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.location_type = ::std::option::Option::Some(tmp);
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
        for value in &self.timestamp_snapshot {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.latitude.is_some() {
            my_size += 5;
        };
        if self.longitude.is_some() {
            my_size += 5;
        };
        if self.horizontal_accuracy.is_some() {
            my_size += 6;
        };
        if self.altitude.is_some() {
            my_size += 6;
        };
        if self.vertical_accuracy.is_some() {
            my_size += 6;
        };
        for value in &self.provider_status {
            my_size += ::protobuf::rt::value_size(26, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.floor {
            my_size += ::protobuf::rt::value_size(27, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.location_type {
            my_size += ::protobuf::rt::value_size(28, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.provider.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.timestamp_snapshot {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.latitude {
            try!(os.write_float(13, v));
        };
        if let Some(v) = self.longitude {
            try!(os.write_float(14, v));
        };
        if let Some(v) = self.horizontal_accuracy {
            try!(os.write_float(20, v));
        };
        if let Some(v) = self.altitude {
            try!(os.write_float(21, v));
        };
        if let Some(v) = self.vertical_accuracy {
            try!(os.write_float(22, v));
        };
        if let Some(v) = self.provider_status {
            try!(os.write_uint64(26, v));
        };
        if let Some(v) = self.floor {
            try!(os.write_uint32(27, v));
        };
        if let Some(v) = self.location_type {
            try!(os.write_uint64(28, v));
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
        ::std::any::TypeId::of::<Signature_LocationFix>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Signature_LocationFix {
    fn new() -> Signature_LocationFix {
        Signature_LocationFix::new()
    }

    fn descriptor_static(_: ::std::option::Option<Signature_LocationFix>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "provider",
                    Signature_LocationFix::has_provider,
                    Signature_LocationFix::get_provider,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "timestamp_snapshot",
                    Signature_LocationFix::has_timestamp_snapshot,
                    Signature_LocationFix::get_timestamp_snapshot,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "latitude",
                    Signature_LocationFix::has_latitude,
                    Signature_LocationFix::get_latitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "longitude",
                    Signature_LocationFix::has_longitude,
                    Signature_LocationFix::get_longitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "horizontal_accuracy",
                    Signature_LocationFix::has_horizontal_accuracy,
                    Signature_LocationFix::get_horizontal_accuracy,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "altitude",
                    Signature_LocationFix::has_altitude,
                    Signature_LocationFix::get_altitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "vertical_accuracy",
                    Signature_LocationFix::has_vertical_accuracy,
                    Signature_LocationFix::get_vertical_accuracy,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "provider_status",
                    Signature_LocationFix::has_provider_status,
                    Signature_LocationFix::get_provider_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "floor",
                    Signature_LocationFix::has_floor,
                    Signature_LocationFix::get_floor,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "location_type",
                    Signature_LocationFix::has_location_type,
                    Signature_LocationFix::get_location_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Signature_LocationFix>(
                    "Signature_LocationFix",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Signature_LocationFix {
    fn clear(&mut self) {
        self.clear_provider();
        self.clear_timestamp_snapshot();
        self.clear_latitude();
        self.clear_longitude();
        self.clear_horizontal_accuracy();
        self.clear_altitude();
        self.clear_vertical_accuracy();
        self.clear_provider_status();
        self.clear_floor();
        self.clear_location_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Signature_LocationFix {
    fn eq(&self, other: &Signature_LocationFix) -> bool {
        self.provider == other.provider &&
        self.timestamp_snapshot == other.timestamp_snapshot &&
        self.latitude == other.latitude &&
        self.longitude == other.longitude &&
        self.horizontal_accuracy == other.horizontal_accuracy &&
        self.altitude == other.altitude &&
        self.vertical_accuracy == other.vertical_accuracy &&
        self.provider_status == other.provider_status &&
        self.floor == other.floor &&
        self.location_type == other.location_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Signature_LocationFix {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Signature_AndroidGpsInfo {
    // message fields
    time_to_fix: ::std::option::Option<u64>,
    satellites_prn: ::std::vec::Vec<i32>,
    snr: ::std::vec::Vec<f32>,
    azimuth: ::std::vec::Vec<f32>,
    elevation: ::std::vec::Vec<f32>,
    has_almanac: ::std::vec::Vec<bool>,
    has_ephemeris: ::std::vec::Vec<bool>,
    used_in_fix: ::std::vec::Vec<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Signature_AndroidGpsInfo {}

impl Signature_AndroidGpsInfo {
    pub fn new() -> Signature_AndroidGpsInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Signature_AndroidGpsInfo {
        static mut instance: ::protobuf::lazy::Lazy<Signature_AndroidGpsInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Signature_AndroidGpsInfo,
        };
        unsafe {
            instance.get(|| {
                Signature_AndroidGpsInfo {
                    time_to_fix: ::std::option::Option::None,
                    satellites_prn: ::std::vec::Vec::new(),
                    snr: ::std::vec::Vec::new(),
                    azimuth: ::std::vec::Vec::new(),
                    elevation: ::std::vec::Vec::new(),
                    has_almanac: ::std::vec::Vec::new(),
                    has_ephemeris: ::std::vec::Vec::new(),
                    used_in_fix: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 time_to_fix = 1;

    pub fn clear_time_to_fix(&mut self) {
        self.time_to_fix = ::std::option::Option::None;
    }

    pub fn has_time_to_fix(&self) -> bool {
        self.time_to_fix.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_to_fix(&mut self, v: u64) {
        self.time_to_fix = ::std::option::Option::Some(v);
    }

    pub fn get_time_to_fix(&self) -> u64 {
        self.time_to_fix.unwrap_or(0)
    }

    // repeated int32 satellites_prn = 2;

    pub fn clear_satellites_prn(&mut self) {
        self.satellites_prn.clear();
    }

    // Param is passed by value, moved
    pub fn set_satellites_prn(&mut self, v: ::std::vec::Vec<i32>) {
        self.satellites_prn = v;
    }

    // Mutable pointer to the field.
    pub fn mut_satellites_prn(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.satellites_prn
    }

    // Take field
    pub fn take_satellites_prn(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.satellites_prn, ::std::vec::Vec::new())
    }

    pub fn get_satellites_prn(&self) -> &[i32] {
        &self.satellites_prn
    }

    // repeated float snr = 3;

    pub fn clear_snr(&mut self) {
        self.snr.clear();
    }

    // Param is passed by value, moved
    pub fn set_snr(&mut self, v: ::std::vec::Vec<f32>) {
        self.snr = v;
    }

    // Mutable pointer to the field.
    pub fn mut_snr(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.snr
    }

    // Take field
    pub fn take_snr(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.snr, ::std::vec::Vec::new())
    }

    pub fn get_snr(&self) -> &[f32] {
        &self.snr
    }

    // repeated float azimuth = 4;

    pub fn clear_azimuth(&mut self) {
        self.azimuth.clear();
    }

    // Param is passed by value, moved
    pub fn set_azimuth(&mut self, v: ::std::vec::Vec<f32>) {
        self.azimuth = v;
    }

    // Mutable pointer to the field.
    pub fn mut_azimuth(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.azimuth
    }

    // Take field
    pub fn take_azimuth(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.azimuth, ::std::vec::Vec::new())
    }

    pub fn get_azimuth(&self) -> &[f32] {
        &self.azimuth
    }

    // repeated float elevation = 5;

    pub fn clear_elevation(&mut self) {
        self.elevation.clear();
    }

    // Param is passed by value, moved
    pub fn set_elevation(&mut self, v: ::std::vec::Vec<f32>) {
        self.elevation = v;
    }

    // Mutable pointer to the field.
    pub fn mut_elevation(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.elevation
    }

    // Take field
    pub fn take_elevation(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.elevation, ::std::vec::Vec::new())
    }

    pub fn get_elevation(&self) -> &[f32] {
        &self.elevation
    }

    // repeated bool has_almanac = 6;

    pub fn clear_has_almanac(&mut self) {
        self.has_almanac.clear();
    }

    // Param is passed by value, moved
    pub fn set_has_almanac(&mut self, v: ::std::vec::Vec<bool>) {
        self.has_almanac = v;
    }

    // Mutable pointer to the field.
    pub fn mut_has_almanac(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.has_almanac
    }

    // Take field
    pub fn take_has_almanac(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.has_almanac, ::std::vec::Vec::new())
    }

    pub fn get_has_almanac(&self) -> &[bool] {
        &self.has_almanac
    }

    // repeated bool has_ephemeris = 7;

    pub fn clear_has_ephemeris(&mut self) {
        self.has_ephemeris.clear();
    }

    // Param is passed by value, moved
    pub fn set_has_ephemeris(&mut self, v: ::std::vec::Vec<bool>) {
        self.has_ephemeris = v;
    }

    // Mutable pointer to the field.
    pub fn mut_has_ephemeris(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.has_ephemeris
    }

    // Take field
    pub fn take_has_ephemeris(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.has_ephemeris, ::std::vec::Vec::new())
    }

    pub fn get_has_ephemeris(&self) -> &[bool] {
        &self.has_ephemeris
    }

    // repeated bool used_in_fix = 8;

    pub fn clear_used_in_fix(&mut self) {
        self.used_in_fix.clear();
    }

    // Param is passed by value, moved
    pub fn set_used_in_fix(&mut self, v: ::std::vec::Vec<bool>) {
        self.used_in_fix = v;
    }

    // Mutable pointer to the field.
    pub fn mut_used_in_fix(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.used_in_fix
    }

    // Take field
    pub fn take_used_in_fix(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.used_in_fix, ::std::vec::Vec::new())
    }

    pub fn get_used_in_fix(&self) -> &[bool] {
        &self.used_in_fix
    }
}

impl ::protobuf::Message for Signature_AndroidGpsInfo {
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
                    self.time_to_fix = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.satellites_prn));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.snr));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.azimuth));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.elevation));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.has_almanac));
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.has_ephemeris));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.used_in_fix));
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
        for value in &self.time_to_fix {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.satellites_prn {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += 5 * self.snr.len() as u32;
        my_size += 5 * self.azimuth.len() as u32;
        my_size += 5 * self.elevation.len() as u32;
        my_size += 2 * self.has_almanac.len() as u32;
        my_size += 2 * self.has_ephemeris.len() as u32;
        my_size += 2 * self.used_in_fix.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.time_to_fix {
            try!(os.write_uint64(1, v));
        };
        for v in &self.satellites_prn {
            try!(os.write_int32(2, *v));
        };
        for v in &self.snr {
            try!(os.write_float(3, *v));
        };
        for v in &self.azimuth {
            try!(os.write_float(4, *v));
        };
        for v in &self.elevation {
            try!(os.write_float(5, *v));
        };
        for v in &self.has_almanac {
            try!(os.write_bool(6, *v));
        };
        for v in &self.has_ephemeris {
            try!(os.write_bool(7, *v));
        };
        for v in &self.used_in_fix {
            try!(os.write_bool(8, *v));
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
        ::std::any::TypeId::of::<Signature_AndroidGpsInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Signature_AndroidGpsInfo {
    fn new() -> Signature_AndroidGpsInfo {
        Signature_AndroidGpsInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<Signature_AndroidGpsInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "time_to_fix",
                    Signature_AndroidGpsInfo::has_time_to_fix,
                    Signature_AndroidGpsInfo::get_time_to_fix,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "satellites_prn",
                    Signature_AndroidGpsInfo::get_satellites_prn,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "snr",
                    Signature_AndroidGpsInfo::get_snr,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "azimuth",
                    Signature_AndroidGpsInfo::get_azimuth,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "elevation",
                    Signature_AndroidGpsInfo::get_elevation,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bool_accessor(
                    "has_almanac",
                    Signature_AndroidGpsInfo::get_has_almanac,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bool_accessor(
                    "has_ephemeris",
                    Signature_AndroidGpsInfo::get_has_ephemeris,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bool_accessor(
                    "used_in_fix",
                    Signature_AndroidGpsInfo::get_used_in_fix,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Signature_AndroidGpsInfo>(
                    "Signature_AndroidGpsInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Signature_AndroidGpsInfo {
    fn clear(&mut self) {
        self.clear_time_to_fix();
        self.clear_satellites_prn();
        self.clear_snr();
        self.clear_azimuth();
        self.clear_elevation();
        self.clear_has_almanac();
        self.clear_has_ephemeris();
        self.clear_used_in_fix();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Signature_AndroidGpsInfo {
    fn eq(&self, other: &Signature_AndroidGpsInfo) -> bool {
        self.time_to_fix == other.time_to_fix &&
        self.satellites_prn == other.satellites_prn &&
        self.snr == other.snr &&
        self.azimuth == other.azimuth &&
        self.elevation == other.elevation &&
        self.has_almanac == other.has_almanac &&
        self.has_ephemeris == other.has_ephemeris &&
        self.used_in_fix == other.used_in_fix &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Signature_AndroidGpsInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Signature_SensorInfo {
    // message fields
    timestamp_snapshot: ::std::option::Option<u64>,
    magnetometer_x: ::std::option::Option<f64>,
    magnetometer_y: ::std::option::Option<f64>,
    magnetometer_z: ::std::option::Option<f64>,
    angle_normalized_x: ::std::option::Option<f64>,
    angle_normalized_y: ::std::option::Option<f64>,
    angle_normalized_z: ::std::option::Option<f64>,
    accel_raw_x: ::std::option::Option<f64>,
    accel_raw_y: ::std::option::Option<f64>,
    accel_raw_z: ::std::option::Option<f64>,
    gyroscope_raw_x: ::std::option::Option<f64>,
    gyroscope_raw_y: ::std::option::Option<f64>,
    gyroscope_raw_z: ::std::option::Option<f64>,
    accel_normalized_x: ::std::option::Option<f64>,
    accel_normalized_y: ::std::option::Option<f64>,
    accel_normalized_z: ::std::option::Option<f64>,
    accelerometer_axes: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Signature_SensorInfo {}

impl Signature_SensorInfo {
    pub fn new() -> Signature_SensorInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Signature_SensorInfo {
        static mut instance: ::protobuf::lazy::Lazy<Signature_SensorInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Signature_SensorInfo,
        };
        unsafe {
            instance.get(|| {
                Signature_SensorInfo {
                    timestamp_snapshot: ::std::option::Option::None,
                    magnetometer_x: ::std::option::Option::None,
                    magnetometer_y: ::std::option::Option::None,
                    magnetometer_z: ::std::option::Option::None,
                    angle_normalized_x: ::std::option::Option::None,
                    angle_normalized_y: ::std::option::Option::None,
                    angle_normalized_z: ::std::option::Option::None,
                    accel_raw_x: ::std::option::Option::None,
                    accel_raw_y: ::std::option::Option::None,
                    accel_raw_z: ::std::option::Option::None,
                    gyroscope_raw_x: ::std::option::Option::None,
                    gyroscope_raw_y: ::std::option::Option::None,
                    gyroscope_raw_z: ::std::option::Option::None,
                    accel_normalized_x: ::std::option::Option::None,
                    accel_normalized_y: ::std::option::Option::None,
                    accel_normalized_z: ::std::option::Option::None,
                    accelerometer_axes: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 timestamp_snapshot = 1;

    pub fn clear_timestamp_snapshot(&mut self) {
        self.timestamp_snapshot = ::std::option::Option::None;
    }

    pub fn has_timestamp_snapshot(&self) -> bool {
        self.timestamp_snapshot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp_snapshot(&mut self, v: u64) {
        self.timestamp_snapshot = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp_snapshot(&self) -> u64 {
        self.timestamp_snapshot.unwrap_or(0)
    }

    // optional double magnetometer_x = 3;

    pub fn clear_magnetometer_x(&mut self) {
        self.magnetometer_x = ::std::option::Option::None;
    }

    pub fn has_magnetometer_x(&self) -> bool {
        self.magnetometer_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_magnetometer_x(&mut self, v: f64) {
        self.magnetometer_x = ::std::option::Option::Some(v);
    }

    pub fn get_magnetometer_x(&self) -> f64 {
        self.magnetometer_x.unwrap_or(0.)
    }

    // optional double magnetometer_y = 4;

    pub fn clear_magnetometer_y(&mut self) {
        self.magnetometer_y = ::std::option::Option::None;
    }

    pub fn has_magnetometer_y(&self) -> bool {
        self.magnetometer_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_magnetometer_y(&mut self, v: f64) {
        self.magnetometer_y = ::std::option::Option::Some(v);
    }

    pub fn get_magnetometer_y(&self) -> f64 {
        self.magnetometer_y.unwrap_or(0.)
    }

    // optional double magnetometer_z = 5;

    pub fn clear_magnetometer_z(&mut self) {
        self.magnetometer_z = ::std::option::Option::None;
    }

    pub fn has_magnetometer_z(&self) -> bool {
        self.magnetometer_z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_magnetometer_z(&mut self, v: f64) {
        self.magnetometer_z = ::std::option::Option::Some(v);
    }

    pub fn get_magnetometer_z(&self) -> f64 {
        self.magnetometer_z.unwrap_or(0.)
    }

    // optional double angle_normalized_x = 6;

    pub fn clear_angle_normalized_x(&mut self) {
        self.angle_normalized_x = ::std::option::Option::None;
    }

    pub fn has_angle_normalized_x(&self) -> bool {
        self.angle_normalized_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle_normalized_x(&mut self, v: f64) {
        self.angle_normalized_x = ::std::option::Option::Some(v);
    }

    pub fn get_angle_normalized_x(&self) -> f64 {
        self.angle_normalized_x.unwrap_or(0.)
    }

    // optional double angle_normalized_y = 7;

    pub fn clear_angle_normalized_y(&mut self) {
        self.angle_normalized_y = ::std::option::Option::None;
    }

    pub fn has_angle_normalized_y(&self) -> bool {
        self.angle_normalized_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle_normalized_y(&mut self, v: f64) {
        self.angle_normalized_y = ::std::option::Option::Some(v);
    }

    pub fn get_angle_normalized_y(&self) -> f64 {
        self.angle_normalized_y.unwrap_or(0.)
    }

    // optional double angle_normalized_z = 8;

    pub fn clear_angle_normalized_z(&mut self) {
        self.angle_normalized_z = ::std::option::Option::None;
    }

    pub fn has_angle_normalized_z(&self) -> bool {
        self.angle_normalized_z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle_normalized_z(&mut self, v: f64) {
        self.angle_normalized_z = ::std::option::Option::Some(v);
    }

    pub fn get_angle_normalized_z(&self) -> f64 {
        self.angle_normalized_z.unwrap_or(0.)
    }

    // optional double accel_raw_x = 10;

    pub fn clear_accel_raw_x(&mut self) {
        self.accel_raw_x = ::std::option::Option::None;
    }

    pub fn has_accel_raw_x(&self) -> bool {
        self.accel_raw_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accel_raw_x(&mut self, v: f64) {
        self.accel_raw_x = ::std::option::Option::Some(v);
    }

    pub fn get_accel_raw_x(&self) -> f64 {
        self.accel_raw_x.unwrap_or(0.)
    }

    // optional double accel_raw_y = 11;

    pub fn clear_accel_raw_y(&mut self) {
        self.accel_raw_y = ::std::option::Option::None;
    }

    pub fn has_accel_raw_y(&self) -> bool {
        self.accel_raw_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accel_raw_y(&mut self, v: f64) {
        self.accel_raw_y = ::std::option::Option::Some(v);
    }

    pub fn get_accel_raw_y(&self) -> f64 {
        self.accel_raw_y.unwrap_or(0.)
    }

    // optional double accel_raw_z = 12;

    pub fn clear_accel_raw_z(&mut self) {
        self.accel_raw_z = ::std::option::Option::None;
    }

    pub fn has_accel_raw_z(&self) -> bool {
        self.accel_raw_z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accel_raw_z(&mut self, v: f64) {
        self.accel_raw_z = ::std::option::Option::Some(v);
    }

    pub fn get_accel_raw_z(&self) -> f64 {
        self.accel_raw_z.unwrap_or(0.)
    }

    // optional double gyroscope_raw_x = 13;

    pub fn clear_gyroscope_raw_x(&mut self) {
        self.gyroscope_raw_x = ::std::option::Option::None;
    }

    pub fn has_gyroscope_raw_x(&self) -> bool {
        self.gyroscope_raw_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gyroscope_raw_x(&mut self, v: f64) {
        self.gyroscope_raw_x = ::std::option::Option::Some(v);
    }

    pub fn get_gyroscope_raw_x(&self) -> f64 {
        self.gyroscope_raw_x.unwrap_or(0.)
    }

    // optional double gyroscope_raw_y = 14;

    pub fn clear_gyroscope_raw_y(&mut self) {
        self.gyroscope_raw_y = ::std::option::Option::None;
    }

    pub fn has_gyroscope_raw_y(&self) -> bool {
        self.gyroscope_raw_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gyroscope_raw_y(&mut self, v: f64) {
        self.gyroscope_raw_y = ::std::option::Option::Some(v);
    }

    pub fn get_gyroscope_raw_y(&self) -> f64 {
        self.gyroscope_raw_y.unwrap_or(0.)
    }

    // optional double gyroscope_raw_z = 15;

    pub fn clear_gyroscope_raw_z(&mut self) {
        self.gyroscope_raw_z = ::std::option::Option::None;
    }

    pub fn has_gyroscope_raw_z(&self) -> bool {
        self.gyroscope_raw_z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gyroscope_raw_z(&mut self, v: f64) {
        self.gyroscope_raw_z = ::std::option::Option::Some(v);
    }

    pub fn get_gyroscope_raw_z(&self) -> f64 {
        self.gyroscope_raw_z.unwrap_or(0.)
    }

    // optional double accel_normalized_x = 16;

    pub fn clear_accel_normalized_x(&mut self) {
        self.accel_normalized_x = ::std::option::Option::None;
    }

    pub fn has_accel_normalized_x(&self) -> bool {
        self.accel_normalized_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accel_normalized_x(&mut self, v: f64) {
        self.accel_normalized_x = ::std::option::Option::Some(v);
    }

    pub fn get_accel_normalized_x(&self) -> f64 {
        self.accel_normalized_x.unwrap_or(0.)
    }

    // optional double accel_normalized_y = 17;

    pub fn clear_accel_normalized_y(&mut self) {
        self.accel_normalized_y = ::std::option::Option::None;
    }

    pub fn has_accel_normalized_y(&self) -> bool {
        self.accel_normalized_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accel_normalized_y(&mut self, v: f64) {
        self.accel_normalized_y = ::std::option::Option::Some(v);
    }

    pub fn get_accel_normalized_y(&self) -> f64 {
        self.accel_normalized_y.unwrap_or(0.)
    }

    // optional double accel_normalized_z = 18;

    pub fn clear_accel_normalized_z(&mut self) {
        self.accel_normalized_z = ::std::option::Option::None;
    }

    pub fn has_accel_normalized_z(&self) -> bool {
        self.accel_normalized_z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accel_normalized_z(&mut self, v: f64) {
        self.accel_normalized_z = ::std::option::Option::Some(v);
    }

    pub fn get_accel_normalized_z(&self) -> f64 {
        self.accel_normalized_z.unwrap_or(0.)
    }

    // optional uint64 accelerometer_axes = 19;

    pub fn clear_accelerometer_axes(&mut self) {
        self.accelerometer_axes = ::std::option::Option::None;
    }

    pub fn has_accelerometer_axes(&self) -> bool {
        self.accelerometer_axes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accelerometer_axes(&mut self, v: u64) {
        self.accelerometer_axes = ::std::option::Option::Some(v);
    }

    pub fn get_accelerometer_axes(&self) -> u64 {
        self.accelerometer_axes.unwrap_or(0)
    }
}

impl ::protobuf::Message for Signature_SensorInfo {
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
                    self.timestamp_snapshot = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.magnetometer_x = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.magnetometer_y = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.magnetometer_z = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.angle_normalized_x = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.angle_normalized_y = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.angle_normalized_z = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.accel_raw_x = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.accel_raw_y = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.accel_raw_z = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.gyroscope_raw_x = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.gyroscope_raw_y = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.gyroscope_raw_z = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.accel_normalized_x = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.accel_normalized_y = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.accel_normalized_z = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.accelerometer_axes = ::std::option::Option::Some(tmp);
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
        for value in &self.timestamp_snapshot {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.magnetometer_x.is_some() {
            my_size += 9;
        };
        if self.magnetometer_y.is_some() {
            my_size += 9;
        };
        if self.magnetometer_z.is_some() {
            my_size += 9;
        };
        if self.angle_normalized_x.is_some() {
            my_size += 9;
        };
        if self.angle_normalized_y.is_some() {
            my_size += 9;
        };
        if self.angle_normalized_z.is_some() {
            my_size += 9;
        };
        if self.accel_raw_x.is_some() {
            my_size += 9;
        };
        if self.accel_raw_y.is_some() {
            my_size += 9;
        };
        if self.accel_raw_z.is_some() {
            my_size += 9;
        };
        if self.gyroscope_raw_x.is_some() {
            my_size += 9;
        };
        if self.gyroscope_raw_y.is_some() {
            my_size += 9;
        };
        if self.gyroscope_raw_z.is_some() {
            my_size += 9;
        };
        if self.accel_normalized_x.is_some() {
            my_size += 10;
        };
        if self.accel_normalized_y.is_some() {
            my_size += 10;
        };
        if self.accel_normalized_z.is_some() {
            my_size += 10;
        };
        for value in &self.accelerometer_axes {
            my_size += ::protobuf::rt::value_size(19, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.timestamp_snapshot {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.magnetometer_x {
            try!(os.write_double(3, v));
        };
        if let Some(v) = self.magnetometer_y {
            try!(os.write_double(4, v));
        };
        if let Some(v) = self.magnetometer_z {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.angle_normalized_x {
            try!(os.write_double(6, v));
        };
        if let Some(v) = self.angle_normalized_y {
            try!(os.write_double(7, v));
        };
        if let Some(v) = self.angle_normalized_z {
            try!(os.write_double(8, v));
        };
        if let Some(v) = self.accel_raw_x {
            try!(os.write_double(10, v));
        };
        if let Some(v) = self.accel_raw_y {
            try!(os.write_double(11, v));
        };
        if let Some(v) = self.accel_raw_z {
            try!(os.write_double(12, v));
        };
        if let Some(v) = self.gyroscope_raw_x {
            try!(os.write_double(13, v));
        };
        if let Some(v) = self.gyroscope_raw_y {
            try!(os.write_double(14, v));
        };
        if let Some(v) = self.gyroscope_raw_z {
            try!(os.write_double(15, v));
        };
        if let Some(v) = self.accel_normalized_x {
            try!(os.write_double(16, v));
        };
        if let Some(v) = self.accel_normalized_y {
            try!(os.write_double(17, v));
        };
        if let Some(v) = self.accel_normalized_z {
            try!(os.write_double(18, v));
        };
        if let Some(v) = self.accelerometer_axes {
            try!(os.write_uint64(19, v));
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
        ::std::any::TypeId::of::<Signature_SensorInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Signature_SensorInfo {
    fn new() -> Signature_SensorInfo {
        Signature_SensorInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<Signature_SensorInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "timestamp_snapshot",
                    Signature_SensorInfo::has_timestamp_snapshot,
                    Signature_SensorInfo::get_timestamp_snapshot,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "magnetometer_x",
                    Signature_SensorInfo::has_magnetometer_x,
                    Signature_SensorInfo::get_magnetometer_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "magnetometer_y",
                    Signature_SensorInfo::has_magnetometer_y,
                    Signature_SensorInfo::get_magnetometer_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "magnetometer_z",
                    Signature_SensorInfo::has_magnetometer_z,
                    Signature_SensorInfo::get_magnetometer_z,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "angle_normalized_x",
                    Signature_SensorInfo::has_angle_normalized_x,
                    Signature_SensorInfo::get_angle_normalized_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "angle_normalized_y",
                    Signature_SensorInfo::has_angle_normalized_y,
                    Signature_SensorInfo::get_angle_normalized_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "angle_normalized_z",
                    Signature_SensorInfo::has_angle_normalized_z,
                    Signature_SensorInfo::get_angle_normalized_z,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "accel_raw_x",
                    Signature_SensorInfo::has_accel_raw_x,
                    Signature_SensorInfo::get_accel_raw_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "accel_raw_y",
                    Signature_SensorInfo::has_accel_raw_y,
                    Signature_SensorInfo::get_accel_raw_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "accel_raw_z",
                    Signature_SensorInfo::has_accel_raw_z,
                    Signature_SensorInfo::get_accel_raw_z,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "gyroscope_raw_x",
                    Signature_SensorInfo::has_gyroscope_raw_x,
                    Signature_SensorInfo::get_gyroscope_raw_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "gyroscope_raw_y",
                    Signature_SensorInfo::has_gyroscope_raw_y,
                    Signature_SensorInfo::get_gyroscope_raw_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "gyroscope_raw_z",
                    Signature_SensorInfo::has_gyroscope_raw_z,
                    Signature_SensorInfo::get_gyroscope_raw_z,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "accel_normalized_x",
                    Signature_SensorInfo::has_accel_normalized_x,
                    Signature_SensorInfo::get_accel_normalized_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "accel_normalized_y",
                    Signature_SensorInfo::has_accel_normalized_y,
                    Signature_SensorInfo::get_accel_normalized_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "accel_normalized_z",
                    Signature_SensorInfo::has_accel_normalized_z,
                    Signature_SensorInfo::get_accel_normalized_z,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "accelerometer_axes",
                    Signature_SensorInfo::has_accelerometer_axes,
                    Signature_SensorInfo::get_accelerometer_axes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Signature_SensorInfo>(
                    "Signature_SensorInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Signature_SensorInfo {
    fn clear(&mut self) {
        self.clear_timestamp_snapshot();
        self.clear_magnetometer_x();
        self.clear_magnetometer_y();
        self.clear_magnetometer_z();
        self.clear_angle_normalized_x();
        self.clear_angle_normalized_y();
        self.clear_angle_normalized_z();
        self.clear_accel_raw_x();
        self.clear_accel_raw_y();
        self.clear_accel_raw_z();
        self.clear_gyroscope_raw_x();
        self.clear_gyroscope_raw_y();
        self.clear_gyroscope_raw_z();
        self.clear_accel_normalized_x();
        self.clear_accel_normalized_y();
        self.clear_accel_normalized_z();
        self.clear_accelerometer_axes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Signature_SensorInfo {
    fn eq(&self, other: &Signature_SensorInfo) -> bool {
        self.timestamp_snapshot == other.timestamp_snapshot &&
        self.magnetometer_x == other.magnetometer_x &&
        self.magnetometer_y == other.magnetometer_y &&
        self.magnetometer_z == other.magnetometer_z &&
        self.angle_normalized_x == other.angle_normalized_x &&
        self.angle_normalized_y == other.angle_normalized_y &&
        self.angle_normalized_z == other.angle_normalized_z &&
        self.accel_raw_x == other.accel_raw_x &&
        self.accel_raw_y == other.accel_raw_y &&
        self.accel_raw_z == other.accel_raw_z &&
        self.gyroscope_raw_x == other.gyroscope_raw_x &&
        self.gyroscope_raw_y == other.gyroscope_raw_y &&
        self.gyroscope_raw_z == other.gyroscope_raw_z &&
        self.accel_normalized_x == other.accel_normalized_x &&
        self.accel_normalized_y == other.accel_normalized_y &&
        self.accel_normalized_z == other.accel_normalized_z &&
        self.accelerometer_axes == other.accelerometer_axes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Signature_SensorInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Signature_DeviceInfo {
    // message fields
    device_id: ::protobuf::SingularField<::std::string::String>,
    android_board_name: ::protobuf::SingularField<::std::string::String>,
    android_bootloader: ::protobuf::SingularField<::std::string::String>,
    device_brand: ::protobuf::SingularField<::std::string::String>,
    device_model: ::protobuf::SingularField<::std::string::String>,
    device_model_identifier: ::protobuf::SingularField<::std::string::String>,
    device_model_boot: ::protobuf::SingularField<::std::string::String>,
    hardware_manufacturer: ::protobuf::SingularField<::std::string::String>,
    hardware_model: ::protobuf::SingularField<::std::string::String>,
    firmware_brand: ::protobuf::SingularField<::std::string::String>,
    firmware_tags: ::protobuf::SingularField<::std::string::String>,
    firmware_type: ::protobuf::SingularField<::std::string::String>,
    firmware_fingerprint: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Signature_DeviceInfo {}

impl Signature_DeviceInfo {
    pub fn new() -> Signature_DeviceInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Signature_DeviceInfo {
        static mut instance: ::protobuf::lazy::Lazy<Signature_DeviceInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Signature_DeviceInfo,
        };
        unsafe {
            instance.get(|| {
                Signature_DeviceInfo {
                    device_id: ::protobuf::SingularField::none(),
                    android_board_name: ::protobuf::SingularField::none(),
                    android_bootloader: ::protobuf::SingularField::none(),
                    device_brand: ::protobuf::SingularField::none(),
                    device_model: ::protobuf::SingularField::none(),
                    device_model_identifier: ::protobuf::SingularField::none(),
                    device_model_boot: ::protobuf::SingularField::none(),
                    hardware_manufacturer: ::protobuf::SingularField::none(),
                    hardware_model: ::protobuf::SingularField::none(),
                    firmware_brand: ::protobuf::SingularField::none(),
                    firmware_tags: ::protobuf::SingularField::none(),
                    firmware_type: ::protobuf::SingularField::none(),
                    firmware_fingerprint: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string device_id = 1;

    pub fn clear_device_id(&mut self) {
        self.device_id.clear();
    }

    pub fn has_device_id(&self) -> bool {
        self.device_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_id(&mut self, v: ::std::string::String) {
        self.device_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_id(&mut self) -> &mut ::std::string::String {
        if self.device_id.is_none() {
            self.device_id.set_default();
        };
        self.device_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_id(&mut self) -> ::std::string::String {
        self.device_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_device_id(&self) -> &str {
        match self.device_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string android_board_name = 2;

    pub fn clear_android_board_name(&mut self) {
        self.android_board_name.clear();
    }

    pub fn has_android_board_name(&self) -> bool {
        self.android_board_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_android_board_name(&mut self, v: ::std::string::String) {
        self.android_board_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_android_board_name(&mut self) -> &mut ::std::string::String {
        if self.android_board_name.is_none() {
            self.android_board_name.set_default();
        };
        self.android_board_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_android_board_name(&mut self) -> ::std::string::String {
        self.android_board_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_android_board_name(&self) -> &str {
        match self.android_board_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string android_bootloader = 3;

    pub fn clear_android_bootloader(&mut self) {
        self.android_bootloader.clear();
    }

    pub fn has_android_bootloader(&self) -> bool {
        self.android_bootloader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_android_bootloader(&mut self, v: ::std::string::String) {
        self.android_bootloader = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_android_bootloader(&mut self) -> &mut ::std::string::String {
        if self.android_bootloader.is_none() {
            self.android_bootloader.set_default();
        };
        self.android_bootloader.as_mut().unwrap()
    }

    // Take field
    pub fn take_android_bootloader(&mut self) -> ::std::string::String {
        self.android_bootloader.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_android_bootloader(&self) -> &str {
        match self.android_bootloader.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string device_brand = 4;

    pub fn clear_device_brand(&mut self) {
        self.device_brand.clear();
    }

    pub fn has_device_brand(&self) -> bool {
        self.device_brand.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_brand(&mut self, v: ::std::string::String) {
        self.device_brand = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_brand(&mut self) -> &mut ::std::string::String {
        if self.device_brand.is_none() {
            self.device_brand.set_default();
        };
        self.device_brand.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_brand(&mut self) -> ::std::string::String {
        self.device_brand.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_device_brand(&self) -> &str {
        match self.device_brand.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string device_model = 5;

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

    // optional string device_model_identifier = 6;

    pub fn clear_device_model_identifier(&mut self) {
        self.device_model_identifier.clear();
    }

    pub fn has_device_model_identifier(&self) -> bool {
        self.device_model_identifier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_model_identifier(&mut self, v: ::std::string::String) {
        self.device_model_identifier = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_model_identifier(&mut self) -> &mut ::std::string::String {
        if self.device_model_identifier.is_none() {
            self.device_model_identifier.set_default();
        };
        self.device_model_identifier.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_model_identifier(&mut self) -> ::std::string::String {
        self.device_model_identifier.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_device_model_identifier(&self) -> &str {
        match self.device_model_identifier.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string device_model_boot = 7;

    pub fn clear_device_model_boot(&mut self) {
        self.device_model_boot.clear();
    }

    pub fn has_device_model_boot(&self) -> bool {
        self.device_model_boot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_model_boot(&mut self, v: ::std::string::String) {
        self.device_model_boot = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_model_boot(&mut self) -> &mut ::std::string::String {
        if self.device_model_boot.is_none() {
            self.device_model_boot.set_default();
        };
        self.device_model_boot.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_model_boot(&mut self) -> ::std::string::String {
        self.device_model_boot.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_device_model_boot(&self) -> &str {
        match self.device_model_boot.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string hardware_manufacturer = 8;

    pub fn clear_hardware_manufacturer(&mut self) {
        self.hardware_manufacturer.clear();
    }

    pub fn has_hardware_manufacturer(&self) -> bool {
        self.hardware_manufacturer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hardware_manufacturer(&mut self, v: ::std::string::String) {
        self.hardware_manufacturer = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hardware_manufacturer(&mut self) -> &mut ::std::string::String {
        if self.hardware_manufacturer.is_none() {
            self.hardware_manufacturer.set_default();
        };
        self.hardware_manufacturer.as_mut().unwrap()
    }

    // Take field
    pub fn take_hardware_manufacturer(&mut self) -> ::std::string::String {
        self.hardware_manufacturer.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hardware_manufacturer(&self) -> &str {
        match self.hardware_manufacturer.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string hardware_model = 9;

    pub fn clear_hardware_model(&mut self) {
        self.hardware_model.clear();
    }

    pub fn has_hardware_model(&self) -> bool {
        self.hardware_model.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hardware_model(&mut self, v: ::std::string::String) {
        self.hardware_model = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hardware_model(&mut self) -> &mut ::std::string::String {
        if self.hardware_model.is_none() {
            self.hardware_model.set_default();
        };
        self.hardware_model.as_mut().unwrap()
    }

    // Take field
    pub fn take_hardware_model(&mut self) -> ::std::string::String {
        self.hardware_model.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hardware_model(&self) -> &str {
        match self.hardware_model.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string firmware_brand = 10;

    pub fn clear_firmware_brand(&mut self) {
        self.firmware_brand.clear();
    }

    pub fn has_firmware_brand(&self) -> bool {
        self.firmware_brand.is_some()
    }

    // Param is passed by value, moved
    pub fn set_firmware_brand(&mut self, v: ::std::string::String) {
        self.firmware_brand = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_firmware_brand(&mut self) -> &mut ::std::string::String {
        if self.firmware_brand.is_none() {
            self.firmware_brand.set_default();
        };
        self.firmware_brand.as_mut().unwrap()
    }

    // Take field
    pub fn take_firmware_brand(&mut self) -> ::std::string::String {
        self.firmware_brand.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_firmware_brand(&self) -> &str {
        match self.firmware_brand.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string firmware_tags = 12;

    pub fn clear_firmware_tags(&mut self) {
        self.firmware_tags.clear();
    }

    pub fn has_firmware_tags(&self) -> bool {
        self.firmware_tags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_firmware_tags(&mut self, v: ::std::string::String) {
        self.firmware_tags = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_firmware_tags(&mut self) -> &mut ::std::string::String {
        if self.firmware_tags.is_none() {
            self.firmware_tags.set_default();
        };
        self.firmware_tags.as_mut().unwrap()
    }

    // Take field
    pub fn take_firmware_tags(&mut self) -> ::std::string::String {
        self.firmware_tags.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_firmware_tags(&self) -> &str {
        match self.firmware_tags.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string firmware_type = 13;

    pub fn clear_firmware_type(&mut self) {
        self.firmware_type.clear();
    }

    pub fn has_firmware_type(&self) -> bool {
        self.firmware_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_firmware_type(&mut self, v: ::std::string::String) {
        self.firmware_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_firmware_type(&mut self) -> &mut ::std::string::String {
        if self.firmware_type.is_none() {
            self.firmware_type.set_default();
        };
        self.firmware_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_firmware_type(&mut self) -> ::std::string::String {
        self.firmware_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_firmware_type(&self) -> &str {
        match self.firmware_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string firmware_fingerprint = 14;

    pub fn clear_firmware_fingerprint(&mut self) {
        self.firmware_fingerprint.clear();
    }

    pub fn has_firmware_fingerprint(&self) -> bool {
        self.firmware_fingerprint.is_some()
    }

    // Param is passed by value, moved
    pub fn set_firmware_fingerprint(&mut self, v: ::std::string::String) {
        self.firmware_fingerprint = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_firmware_fingerprint(&mut self) -> &mut ::std::string::String {
        if self.firmware_fingerprint.is_none() {
            self.firmware_fingerprint.set_default();
        };
        self.firmware_fingerprint.as_mut().unwrap()
    }

    // Take field
    pub fn take_firmware_fingerprint(&mut self) -> ::std::string::String {
        self.firmware_fingerprint.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_firmware_fingerprint(&self) -> &str {
        match self.firmware_fingerprint.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Signature_DeviceInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.device_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.android_board_name));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.android_bootloader));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.device_brand));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.device_model));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.device_model_identifier));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.device_model_boot));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hardware_manufacturer));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hardware_model));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.firmware_brand));
                },
                12 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.firmware_tags));
                },
                13 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.firmware_type));
                },
                14 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.firmware_fingerprint));
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
        for value in &self.device_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.android_board_name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.android_bootloader {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.device_brand {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.device_model {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in &self.device_model_identifier {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        for value in &self.device_model_boot {
            my_size += ::protobuf::rt::string_size(7, &value);
        };
        for value in &self.hardware_manufacturer {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        for value in &self.hardware_model {
            my_size += ::protobuf::rt::string_size(9, &value);
        };
        for value in &self.firmware_brand {
            my_size += ::protobuf::rt::string_size(10, &value);
        };
        for value in &self.firmware_tags {
            my_size += ::protobuf::rt::string_size(12, &value);
        };
        for value in &self.firmware_type {
            my_size += ::protobuf::rt::string_size(13, &value);
        };
        for value in &self.firmware_fingerprint {
            my_size += ::protobuf::rt::string_size(14, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.device_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.android_board_name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.android_bootloader.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.device_brand.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.device_model.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.device_model_identifier.as_ref() {
            try!(os.write_string(6, &v));
        };
        if let Some(v) = self.device_model_boot.as_ref() {
            try!(os.write_string(7, &v));
        };
        if let Some(v) = self.hardware_manufacturer.as_ref() {
            try!(os.write_string(8, &v));
        };
        if let Some(v) = self.hardware_model.as_ref() {
            try!(os.write_string(9, &v));
        };
        if let Some(v) = self.firmware_brand.as_ref() {
            try!(os.write_string(10, &v));
        };
        if let Some(v) = self.firmware_tags.as_ref() {
            try!(os.write_string(12, &v));
        };
        if let Some(v) = self.firmware_type.as_ref() {
            try!(os.write_string(13, &v));
        };
        if let Some(v) = self.firmware_fingerprint.as_ref() {
            try!(os.write_string(14, &v));
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
        ::std::any::TypeId::of::<Signature_DeviceInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Signature_DeviceInfo {
    fn new() -> Signature_DeviceInfo {
        Signature_DeviceInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<Signature_DeviceInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "device_id",
                    Signature_DeviceInfo::has_device_id,
                    Signature_DeviceInfo::get_device_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "android_board_name",
                    Signature_DeviceInfo::has_android_board_name,
                    Signature_DeviceInfo::get_android_board_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "android_bootloader",
                    Signature_DeviceInfo::has_android_bootloader,
                    Signature_DeviceInfo::get_android_bootloader,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "device_brand",
                    Signature_DeviceInfo::has_device_brand,
                    Signature_DeviceInfo::get_device_brand,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "device_model",
                    Signature_DeviceInfo::has_device_model,
                    Signature_DeviceInfo::get_device_model,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "device_model_identifier",
                    Signature_DeviceInfo::has_device_model_identifier,
                    Signature_DeviceInfo::get_device_model_identifier,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "device_model_boot",
                    Signature_DeviceInfo::has_device_model_boot,
                    Signature_DeviceInfo::get_device_model_boot,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "hardware_manufacturer",
                    Signature_DeviceInfo::has_hardware_manufacturer,
                    Signature_DeviceInfo::get_hardware_manufacturer,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "hardware_model",
                    Signature_DeviceInfo::has_hardware_model,
                    Signature_DeviceInfo::get_hardware_model,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "firmware_brand",
                    Signature_DeviceInfo::has_firmware_brand,
                    Signature_DeviceInfo::get_firmware_brand,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "firmware_tags",
                    Signature_DeviceInfo::has_firmware_tags,
                    Signature_DeviceInfo::get_firmware_tags,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "firmware_type",
                    Signature_DeviceInfo::has_firmware_type,
                    Signature_DeviceInfo::get_firmware_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "firmware_fingerprint",
                    Signature_DeviceInfo::has_firmware_fingerprint,
                    Signature_DeviceInfo::get_firmware_fingerprint,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Signature_DeviceInfo>(
                    "Signature_DeviceInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Signature_DeviceInfo {
    fn clear(&mut self) {
        self.clear_device_id();
        self.clear_android_board_name();
        self.clear_android_bootloader();
        self.clear_device_brand();
        self.clear_device_model();
        self.clear_device_model_identifier();
        self.clear_device_model_boot();
        self.clear_hardware_manufacturer();
        self.clear_hardware_model();
        self.clear_firmware_brand();
        self.clear_firmware_tags();
        self.clear_firmware_type();
        self.clear_firmware_fingerprint();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Signature_DeviceInfo {
    fn eq(&self, other: &Signature_DeviceInfo) -> bool {
        self.device_id == other.device_id &&
        self.android_board_name == other.android_board_name &&
        self.android_bootloader == other.android_bootloader &&
        self.device_brand == other.device_brand &&
        self.device_model == other.device_model &&
        self.device_model_identifier == other.device_model_identifier &&
        self.device_model_boot == other.device_model_boot &&
        self.hardware_manufacturer == other.hardware_manufacturer &&
        self.hardware_model == other.hardware_model &&
        self.firmware_brand == other.firmware_brand &&
        self.firmware_tags == other.firmware_tags &&
        self.firmware_type == other.firmware_type &&
        self.firmware_fingerprint == other.firmware_fingerprint &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Signature_DeviceInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Signature_ActivityStatus {
    // message fields
    start_time_ms: ::std::option::Option<u64>,
    unknown_status: ::std::option::Option<bool>,
    walking: ::std::option::Option<bool>,
    running: ::std::option::Option<bool>,
    stationary: ::std::option::Option<bool>,
    automotive: ::std::option::Option<bool>,
    tilting: ::std::option::Option<bool>,
    cycling: ::std::option::Option<bool>,
    status: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Signature_ActivityStatus {}

impl Signature_ActivityStatus {
    pub fn new() -> Signature_ActivityStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Signature_ActivityStatus {
        static mut instance: ::protobuf::lazy::Lazy<Signature_ActivityStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Signature_ActivityStatus,
        };
        unsafe {
            instance.get(|| {
                Signature_ActivityStatus {
                    start_time_ms: ::std::option::Option::None,
                    unknown_status: ::std::option::Option::None,
                    walking: ::std::option::Option::None,
                    running: ::std::option::Option::None,
                    stationary: ::std::option::Option::None,
                    automotive: ::std::option::Option::None,
                    tilting: ::std::option::Option::None,
                    cycling: ::std::option::Option::None,
                    status: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 start_time_ms = 1;

    pub fn clear_start_time_ms(&mut self) {
        self.start_time_ms = ::std::option::Option::None;
    }

    pub fn has_start_time_ms(&self) -> bool {
        self.start_time_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_time_ms(&mut self, v: u64) {
        self.start_time_ms = ::std::option::Option::Some(v);
    }

    pub fn get_start_time_ms(&self) -> u64 {
        self.start_time_ms.unwrap_or(0)
    }

    // optional bool unknown_status = 2;

    pub fn clear_unknown_status(&mut self) {
        self.unknown_status = ::std::option::Option::None;
    }

    pub fn has_unknown_status(&self) -> bool {
        self.unknown_status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unknown_status(&mut self, v: bool) {
        self.unknown_status = ::std::option::Option::Some(v);
    }

    pub fn get_unknown_status(&self) -> bool {
        self.unknown_status.unwrap_or(false)
    }

    // optional bool walking = 3;

    pub fn clear_walking(&mut self) {
        self.walking = ::std::option::Option::None;
    }

    pub fn has_walking(&self) -> bool {
        self.walking.is_some()
    }

    // Param is passed by value, moved
    pub fn set_walking(&mut self, v: bool) {
        self.walking = ::std::option::Option::Some(v);
    }

    pub fn get_walking(&self) -> bool {
        self.walking.unwrap_or(false)
    }

    // optional bool running = 4;

    pub fn clear_running(&mut self) {
        self.running = ::std::option::Option::None;
    }

    pub fn has_running(&self) -> bool {
        self.running.is_some()
    }

    // Param is passed by value, moved
    pub fn set_running(&mut self, v: bool) {
        self.running = ::std::option::Option::Some(v);
    }

    pub fn get_running(&self) -> bool {
        self.running.unwrap_or(false)
    }

    // optional bool stationary = 5;

    pub fn clear_stationary(&mut self) {
        self.stationary = ::std::option::Option::None;
    }

    pub fn has_stationary(&self) -> bool {
        self.stationary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stationary(&mut self, v: bool) {
        self.stationary = ::std::option::Option::Some(v);
    }

    pub fn get_stationary(&self) -> bool {
        self.stationary.unwrap_or(false)
    }

    // optional bool automotive = 6;

    pub fn clear_automotive(&mut self) {
        self.automotive = ::std::option::Option::None;
    }

    pub fn has_automotive(&self) -> bool {
        self.automotive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_automotive(&mut self, v: bool) {
        self.automotive = ::std::option::Option::Some(v);
    }

    pub fn get_automotive(&self) -> bool {
        self.automotive.unwrap_or(false)
    }

    // optional bool tilting = 7;

    pub fn clear_tilting(&mut self) {
        self.tilting = ::std::option::Option::None;
    }

    pub fn has_tilting(&self) -> bool {
        self.tilting.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tilting(&mut self, v: bool) {
        self.tilting = ::std::option::Option::Some(v);
    }

    pub fn get_tilting(&self) -> bool {
        self.tilting.unwrap_or(false)
    }

    // optional bool cycling = 8;

    pub fn clear_cycling(&mut self) {
        self.cycling = ::std::option::Option::None;
    }

    pub fn has_cycling(&self) -> bool {
        self.cycling.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cycling(&mut self, v: bool) {
        self.cycling = ::std::option::Option::Some(v);
    }

    pub fn get_cycling(&self) -> bool {
        self.cycling.unwrap_or(false)
    }

    // optional bytes status = 9;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: ::std::vec::Vec<u8>) {
        self.status = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.status.is_none() {
            self.status.set_default();
        };
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> ::std::vec::Vec<u8> {
        self.status.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_status(&self) -> &[u8] {
        match self.status.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for Signature_ActivityStatus {
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
                    self.start_time_ms = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.unknown_status = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.walking = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.running = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.stationary = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.automotive = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.tilting = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.cycling = ::std::option::Option::Some(tmp);
                },
                9 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.status));
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
        for value in &self.start_time_ms {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.unknown_status.is_some() {
            my_size += 2;
        };
        if self.walking.is_some() {
            my_size += 2;
        };
        if self.running.is_some() {
            my_size += 2;
        };
        if self.stationary.is_some() {
            my_size += 2;
        };
        if self.automotive.is_some() {
            my_size += 2;
        };
        if self.tilting.is_some() {
            my_size += 2;
        };
        if self.cycling.is_some() {
            my_size += 2;
        };
        for value in &self.status {
            my_size += ::protobuf::rt::bytes_size(9, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_time_ms {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.unknown_status {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.walking {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.running {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.stationary {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.automotive {
            try!(os.write_bool(6, v));
        };
        if let Some(v) = self.tilting {
            try!(os.write_bool(7, v));
        };
        if let Some(v) = self.cycling {
            try!(os.write_bool(8, v));
        };
        if let Some(v) = self.status.as_ref() {
            try!(os.write_bytes(9, &v));
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
        ::std::any::TypeId::of::<Signature_ActivityStatus>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Signature_ActivityStatus {
    fn new() -> Signature_ActivityStatus {
        Signature_ActivityStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<Signature_ActivityStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "start_time_ms",
                    Signature_ActivityStatus::has_start_time_ms,
                    Signature_ActivityStatus::get_start_time_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "unknown_status",
                    Signature_ActivityStatus::has_unknown_status,
                    Signature_ActivityStatus::get_unknown_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "walking",
                    Signature_ActivityStatus::has_walking,
                    Signature_ActivityStatus::get_walking,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "running",
                    Signature_ActivityStatus::has_running,
                    Signature_ActivityStatus::get_running,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "stationary",
                    Signature_ActivityStatus::has_stationary,
                    Signature_ActivityStatus::get_stationary,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "automotive",
                    Signature_ActivityStatus::has_automotive,
                    Signature_ActivityStatus::get_automotive,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "tilting",
                    Signature_ActivityStatus::has_tilting,
                    Signature_ActivityStatus::get_tilting,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "cycling",
                    Signature_ActivityStatus::has_cycling,
                    Signature_ActivityStatus::get_cycling,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "status",
                    Signature_ActivityStatus::has_status,
                    Signature_ActivityStatus::get_status,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Signature_ActivityStatus>(
                    "Signature_ActivityStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Signature_ActivityStatus {
    fn clear(&mut self) {
        self.clear_start_time_ms();
        self.clear_unknown_status();
        self.clear_walking();
        self.clear_running();
        self.clear_stationary();
        self.clear_automotive();
        self.clear_tilting();
        self.clear_cycling();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Signature_ActivityStatus {
    fn eq(&self, other: &Signature_ActivityStatus) -> bool {
        self.start_time_ms == other.start_time_ms &&
        self.unknown_status == other.unknown_status &&
        self.walking == other.walking &&
        self.running == other.running &&
        self.stationary == other.stationary &&
        self.automotive == other.automotive &&
        self.tilting == other.tilting &&
        self.cycling == other.cycling &&
        self.status == other.status &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Signature_ActivityStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

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
    unknown6: ::protobuf::SingularPtrField<Unknown6>,
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
                    unknown6: ::protobuf::SingularPtrField::none(),
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

    // optional .POGOProtos.Networking.Envelopes.Unknown6 unknown6 = 6;

    pub fn clear_unknown6(&mut self) {
        self.unknown6.clear();
    }

    pub fn has_unknown6(&self) -> bool {
        self.unknown6.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unknown6(&mut self, v: Unknown6) {
        self.unknown6 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unknown6(&mut self) -> &mut Unknown6 {
        if self.unknown6.is_none() {
            self.unknown6.set_default();
        };
        self.unknown6.as_mut().unwrap()
    }

    // Take field
    pub fn take_unknown6(&mut self) -> Unknown6 {
        self.unknown6.take().unwrap_or_else(|| Unknown6::new())
    }

    pub fn get_unknown6(&self) -> &Unknown6 {
        self.unknown6.as_ref().unwrap_or_else(|| Unknown6::default_instance())
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unknown6));
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
        if let Some(v) = self.unknown6.as_ref() {
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "unknown6",
                    RequestEnvelope::has_unknown6,
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
    encrypted_signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
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
                    encrypted_signature: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes encrypted_signature = 1;

    pub fn clear_encrypted_signature(&mut self) {
        self.encrypted_signature.clear();
    }

    pub fn has_encrypted_signature(&self) -> bool {
        self.encrypted_signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encrypted_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.encrypted_signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encrypted_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.encrypted_signature.is_none() {
            self.encrypted_signature.set_default();
        };
        self.encrypted_signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_encrypted_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.encrypted_signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_encrypted_signature(&self) -> &[u8] {
        match self.encrypted_signature.as_ref() {
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.encrypted_signature));
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
        for value in &self.encrypted_signature {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.encrypted_signature.as_ref() {
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
                    "encrypted_signature",
                    Unknown6_Unknown2::has_encrypted_signature,
                    Unknown6_Unknown2::get_encrypted_signature,
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
        self.clear_encrypted_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Unknown6_Unknown2 {
    fn eq(&self, other: &Unknown6_Unknown2) -> bool {
        self.encrypted_signature == other.encrypted_signature &&
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
    0x01, 0x50, 0x02, 0x22, 0xf0, 0x16, 0x0a, 0x09, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72,
    0x65, 0x12, 0x32, 0x0a, 0x15, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x73,
    0x69, 0x6e, 0x63, 0x65, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x13, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x53, 0x69, 0x6e, 0x63, 0x65,
    0x53, 0x74, 0x61, 0x72, 0x74, 0x12, 0x59, 0x0a, 0x0c, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x5f, 0x66, 0x69, 0x78, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x36, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x53, 0x69,
    0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x46, 0x69, 0x78, 0x52, 0x0b, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x46, 0x69, 0x78,
    0x12, 0x54, 0x0a, 0x08, 0x67, 0x70, 0x73, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x39, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c,
    0x6f, 0x70, 0x65, 0x73, 0x2e, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x41,
    0x6e, 0x64, 0x72, 0x6f, 0x69, 0x64, 0x47, 0x70, 0x73, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x07, 0x67,
    0x70, 0x73, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x56, 0x0a, 0x0b, 0x73, 0x65, 0x6e, 0x73, 0x6f, 0x72,
    0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x35, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x53, 0x69,
    0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x53, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x49, 0x6e,
    0x66, 0x6f, 0x52, 0x0a, 0x73, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x56,
    0x0a, 0x0b, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x35, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65,
    0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x2e,
    0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x0a, 0x64, 0x65, 0x76, 0x69,
    0x63, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x62, 0x0a, 0x0f, 0x61, 0x63, 0x74, 0x69, 0x76, 0x69,
    0x74, 0x79, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x39, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65,
    0x73, 0x2e, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x41, 0x63, 0x74, 0x69,
    0x76, 0x69, 0x74, 0x79, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x0e, 0x61, 0x63, 0x74, 0x69,
    0x76, 0x69, 0x74, 0x79, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x25, 0x0a, 0x0e, 0x6c, 0x6f,
    0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x31, 0x18, 0x0a, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x0d, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x61, 0x73, 0x68,
    0x31, 0x12, 0x25, 0x0a, 0x0e, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x68, 0x61,
    0x73, 0x68, 0x32, 0x18, 0x14, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0d, 0x6c, 0x6f, 0x63, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x48, 0x61, 0x73, 0x68, 0x32, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x65, 0x73, 0x73,
    0x69, 0x6f, 0x6e, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x16, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0b,
    0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x48, 0x61, 0x73, 0x68, 0x12, 0x1c, 0x0a, 0x09, 0x74,
    0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x17, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x21, 0x0a, 0x0c, 0x72, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x18, 0x20, 0x03, 0x28, 0x03, 0x52,
    0x0b, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x61, 0x73, 0x68, 0x12, 0x1c, 0x0a, 0x09,
    0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x32, 0x35, 0x18, 0x19, 0x20, 0x01, 0x28, 0x03, 0x52,
    0x09, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x32, 0x35, 0x1a, 0xf0, 0x02, 0x0a, 0x0b, 0x4c,
    0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x46, 0x69, 0x78, 0x12, 0x1a, 0x0a, 0x08, 0x70, 0x72,
    0x6f, 0x76, 0x69, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x70, 0x72,
    0x6f, 0x76, 0x69, 0x64, 0x65, 0x72, 0x12, 0x2d, 0x0a, 0x12, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x5f, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x11, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x53, 0x6e, 0x61,
    0x70, 0x73, 0x68, 0x6f, 0x74, 0x12, 0x1a, 0x0a, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64,
    0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x02, 0x52, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64,
    0x65, 0x12, 0x1c, 0x0a, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x0e,
    0x20, 0x01, 0x28, 0x02, 0x52, 0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12,
    0x2f, 0x0a, 0x13, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x6f, 0x6e, 0x74, 0x61, 0x6c, 0x5f, 0x61, 0x63,
    0x63, 0x75, 0x72, 0x61, 0x63, 0x79, 0x18, 0x14, 0x20, 0x01, 0x28, 0x02, 0x52, 0x12, 0x68, 0x6f,
    0x72, 0x69, 0x7a, 0x6f, 0x6e, 0x74, 0x61, 0x6c, 0x41, 0x63, 0x63, 0x75, 0x72, 0x61, 0x63, 0x79,
    0x12, 0x1a, 0x0a, 0x08, 0x61, 0x6c, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x15, 0x20, 0x01,
    0x28, 0x02, 0x52, 0x08, 0x61, 0x6c, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x2b, 0x0a, 0x11,
    0x76, 0x65, 0x72, 0x74, 0x69, 0x63, 0x61, 0x6c, 0x5f, 0x61, 0x63, 0x63, 0x75, 0x72, 0x61, 0x63,
    0x79, 0x18, 0x16, 0x20, 0x01, 0x28, 0x02, 0x52, 0x10, 0x76, 0x65, 0x72, 0x74, 0x69, 0x63, 0x61,
    0x6c, 0x41, 0x63, 0x63, 0x75, 0x72, 0x61, 0x63, 0x79, 0x12, 0x27, 0x0a, 0x0f, 0x70, 0x72, 0x6f,
    0x76, 0x69, 0x64, 0x65, 0x72, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x1a, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x0e, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x12, 0x14, 0x0a, 0x05, 0x66, 0x6c, 0x6f, 0x6f, 0x72, 0x18, 0x1b, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x05, 0x66, 0x6c, 0x6f, 0x6f, 0x72, 0x12, 0x23, 0x0a, 0x0d, 0x6c, 0x6f, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x1c, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x0c, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x1a, 0x87, 0x02,
    0x0a, 0x0e, 0x41, 0x6e, 0x64, 0x72, 0x6f, 0x69, 0x64, 0x47, 0x70, 0x73, 0x49, 0x6e, 0x66, 0x6f,
    0x12, 0x1e, 0x0a, 0x0b, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x74, 0x6f, 0x5f, 0x66, 0x69, 0x78, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x54, 0x6f, 0x46, 0x69, 0x78,
    0x12, 0x25, 0x0a, 0x0e, 0x73, 0x61, 0x74, 0x65, 0x6c, 0x6c, 0x69, 0x74, 0x65, 0x73, 0x5f, 0x70,
    0x72, 0x6e, 0x18, 0x02, 0x20, 0x03, 0x28, 0x05, 0x52, 0x0d, 0x73, 0x61, 0x74, 0x65, 0x6c, 0x6c,
    0x69, 0x74, 0x65, 0x73, 0x50, 0x72, 0x6e, 0x12, 0x10, 0x0a, 0x03, 0x73, 0x6e, 0x72, 0x18, 0x03,
    0x20, 0x03, 0x28, 0x02, 0x52, 0x03, 0x73, 0x6e, 0x72, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x7a, 0x69,
    0x6d, 0x75, 0x74, 0x68, 0x18, 0x04, 0x20, 0x03, 0x28, 0x02, 0x52, 0x07, 0x61, 0x7a, 0x69, 0x6d,
    0x75, 0x74, 0x68, 0x12, 0x1c, 0x0a, 0x09, 0x65, 0x6c, 0x65, 0x76, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x05, 0x20, 0x03, 0x28, 0x02, 0x52, 0x09, 0x65, 0x6c, 0x65, 0x76, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x1f, 0x0a, 0x0b, 0x68, 0x61, 0x73, 0x5f, 0x61, 0x6c, 0x6d, 0x61, 0x6e, 0x61, 0x63,
    0x18, 0x06, 0x20, 0x03, 0x28, 0x08, 0x52, 0x0a, 0x68, 0x61, 0x73, 0x41, 0x6c, 0x6d, 0x61, 0x6e,
    0x61, 0x63, 0x12, 0x23, 0x0a, 0x0d, 0x68, 0x61, 0x73, 0x5f, 0x65, 0x70, 0x68, 0x65, 0x6d, 0x65,
    0x72, 0x69, 0x73, 0x18, 0x07, 0x20, 0x03, 0x28, 0x08, 0x52, 0x0c, 0x68, 0x61, 0x73, 0x45, 0x70,
    0x68, 0x65, 0x6d, 0x65, 0x72, 0x69, 0x73, 0x12, 0x1e, 0x0a, 0x0b, 0x75, 0x73, 0x65, 0x64, 0x5f,
    0x69, 0x6e, 0x5f, 0x66, 0x69, 0x78, 0x18, 0x08, 0x20, 0x03, 0x28, 0x08, 0x52, 0x09, 0x75, 0x73,
    0x65, 0x64, 0x49, 0x6e, 0x46, 0x69, 0x78, 0x1a, 0xcb, 0x05, 0x0a, 0x0a, 0x53, 0x65, 0x6e, 0x73,
    0x6f, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x2d, 0x0a, 0x12, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x5f, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x11, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x53, 0x6e, 0x61,
    0x70, 0x73, 0x68, 0x6f, 0x74, 0x12, 0x25, 0x0a, 0x0e, 0x6d, 0x61, 0x67, 0x6e, 0x65, 0x74, 0x6f,
    0x6d, 0x65, 0x74, 0x65, 0x72, 0x5f, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0d, 0x6d,
    0x61, 0x67, 0x6e, 0x65, 0x74, 0x6f, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x58, 0x12, 0x25, 0x0a, 0x0e,
    0x6d, 0x61, 0x67, 0x6e, 0x65, 0x74, 0x6f, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x5f, 0x79, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x01, 0x52, 0x0d, 0x6d, 0x61, 0x67, 0x6e, 0x65, 0x74, 0x6f, 0x6d, 0x65, 0x74,
    0x65, 0x72, 0x59, 0x12, 0x25, 0x0a, 0x0e, 0x6d, 0x61, 0x67, 0x6e, 0x65, 0x74, 0x6f, 0x6d, 0x65,
    0x74, 0x65, 0x72, 0x5f, 0x7a, 0x18, 0x05, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0d, 0x6d, 0x61, 0x67,
    0x6e, 0x65, 0x74, 0x6f, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x5a, 0x12, 0x2c, 0x0a, 0x12, 0x61, 0x6e,
    0x67, 0x6c, 0x65, 0x5f, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x5f, 0x78,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x01, 0x52, 0x10, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x4e, 0x6f, 0x72,
    0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x58, 0x12, 0x2c, 0x0a, 0x12, 0x61, 0x6e, 0x67, 0x6c,
    0x65, 0x5f, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x5f, 0x79, 0x18, 0x07,
    0x20, 0x01, 0x28, 0x01, 0x52, 0x10, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x4e, 0x6f, 0x72, 0x6d, 0x61,
    0x6c, 0x69, 0x7a, 0x65, 0x64, 0x59, 0x12, 0x2c, 0x0a, 0x12, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x5f,
    0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x5f, 0x7a, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x01, 0x52, 0x10, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x4e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x69,
    0x7a, 0x65, 0x64, 0x5a, 0x12, 0x1e, 0x0a, 0x0b, 0x61, 0x63, 0x63, 0x65, 0x6c, 0x5f, 0x72, 0x61,
    0x77, 0x5f, 0x78, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x01, 0x52, 0x09, 0x61, 0x63, 0x63, 0x65, 0x6c,
    0x52, 0x61, 0x77, 0x58, 0x12, 0x1e, 0x0a, 0x0b, 0x61, 0x63, 0x63, 0x65, 0x6c, 0x5f, 0x72, 0x61,
    0x77, 0x5f, 0x79, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x01, 0x52, 0x09, 0x61, 0x63, 0x63, 0x65, 0x6c,
    0x52, 0x61, 0x77, 0x59, 0x12, 0x1e, 0x0a, 0x0b, 0x61, 0x63, 0x63, 0x65, 0x6c, 0x5f, 0x72, 0x61,
    0x77, 0x5f, 0x7a, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x01, 0x52, 0x09, 0x61, 0x63, 0x63, 0x65, 0x6c,
    0x52, 0x61, 0x77, 0x5a, 0x12, 0x26, 0x0a, 0x0f, 0x67, 0x79, 0x72, 0x6f, 0x73, 0x63, 0x6f, 0x70,
    0x65, 0x5f, 0x72, 0x61, 0x77, 0x5f, 0x78, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0d, 0x67,
    0x79, 0x72, 0x6f, 0x73, 0x63, 0x6f, 0x70, 0x65, 0x52, 0x61, 0x77, 0x58, 0x12, 0x26, 0x0a, 0x0f,
    0x67, 0x79, 0x72, 0x6f, 0x73, 0x63, 0x6f, 0x70, 0x65, 0x5f, 0x72, 0x61, 0x77, 0x5f, 0x79, 0x18,
    0x0e, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0d, 0x67, 0x79, 0x72, 0x6f, 0x73, 0x63, 0x6f, 0x70, 0x65,
    0x52, 0x61, 0x77, 0x59, 0x12, 0x26, 0x0a, 0x0f, 0x67, 0x79, 0x72, 0x6f, 0x73, 0x63, 0x6f, 0x70,
    0x65, 0x5f, 0x72, 0x61, 0x77, 0x5f, 0x7a, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0d, 0x67,
    0x79, 0x72, 0x6f, 0x73, 0x63, 0x6f, 0x70, 0x65, 0x52, 0x61, 0x77, 0x5a, 0x12, 0x2c, 0x0a, 0x12,
    0x61, 0x63, 0x63, 0x65, 0x6c, 0x5f, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64,
    0x5f, 0x78, 0x18, 0x10, 0x20, 0x01, 0x28, 0x01, 0x52, 0x10, 0x61, 0x63, 0x63, 0x65, 0x6c, 0x4e,
    0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x58, 0x12, 0x2c, 0x0a, 0x12, 0x61, 0x63,
    0x63, 0x65, 0x6c, 0x5f, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x5f, 0x79,
    0x18, 0x11, 0x20, 0x01, 0x28, 0x01, 0x52, 0x10, 0x61, 0x63, 0x63, 0x65, 0x6c, 0x4e, 0x6f, 0x72,
    0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x59, 0x12, 0x2c, 0x0a, 0x12, 0x61, 0x63, 0x63, 0x65,
    0x6c, 0x5f, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x5f, 0x7a, 0x18, 0x12,
    0x20, 0x01, 0x28, 0x01, 0x52, 0x10, 0x61, 0x63, 0x63, 0x65, 0x6c, 0x4e, 0x6f, 0x72, 0x6d, 0x61,
    0x6c, 0x69, 0x7a, 0x65, 0x64, 0x5a, 0x12, 0x2d, 0x0a, 0x12, 0x61, 0x63, 0x63, 0x65, 0x6c, 0x65,
    0x72, 0x6f, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x5f, 0x61, 0x78, 0x65, 0x73, 0x18, 0x13, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x11, 0x61, 0x63, 0x63, 0x65, 0x6c, 0x65, 0x72, 0x6f, 0x6d, 0x65, 0x74, 0x65,
    0x72, 0x41, 0x78, 0x65, 0x73, 0x1a, 0xb0, 0x04, 0x0a, 0x0a, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65,
    0x49, 0x6e, 0x66, 0x6f, 0x12, 0x1b, 0x0a, 0x09, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x49,
    0x64, 0x12, 0x2c, 0x0a, 0x12, 0x61, 0x6e, 0x64, 0x72, 0x6f, 0x69, 0x64, 0x5f, 0x62, 0x6f, 0x61,
    0x72, 0x64, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x61,
    0x6e, 0x64, 0x72, 0x6f, 0x69, 0x64, 0x42, 0x6f, 0x61, 0x72, 0x64, 0x4e, 0x61, 0x6d, 0x65, 0x12,
    0x2d, 0x0a, 0x12, 0x61, 0x6e, 0x64, 0x72, 0x6f, 0x69, 0x64, 0x5f, 0x62, 0x6f, 0x6f, 0x74, 0x6c,
    0x6f, 0x61, 0x64, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x11, 0x61, 0x6e, 0x64,
    0x72, 0x6f, 0x69, 0x64, 0x42, 0x6f, 0x6f, 0x74, 0x6c, 0x6f, 0x61, 0x64, 0x65, 0x72, 0x12, 0x21,
    0x0a, 0x0c, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x62, 0x72, 0x61, 0x6e, 0x64, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x42, 0x72, 0x61, 0x6e,
    0x64, 0x12, 0x21, 0x0a, 0x0c, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65,
    0x6c, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x4d,
    0x6f, 0x64, 0x65, 0x6c, 0x12, 0x36, 0x0a, 0x17, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x6d,
    0x6f, 0x64, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x15, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x4d, 0x6f, 0x64,
    0x65, 0x6c, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x2a, 0x0a, 0x11,
    0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x5f, 0x62, 0x6f, 0x6f,
    0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x4d,
    0x6f, 0x64, 0x65, 0x6c, 0x42, 0x6f, 0x6f, 0x74, 0x12, 0x33, 0x0a, 0x15, 0x68, 0x61, 0x72, 0x64,
    0x77, 0x61, 0x72, 0x65, 0x5f, 0x6d, 0x61, 0x6e, 0x75, 0x66, 0x61, 0x63, 0x74, 0x75, 0x72, 0x65,
    0x72, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09, 0x52, 0x14, 0x68, 0x61, 0x72, 0x64, 0x77, 0x61, 0x72,
    0x65, 0x4d, 0x61, 0x6e, 0x75, 0x66, 0x61, 0x63, 0x74, 0x75, 0x72, 0x65, 0x72, 0x12, 0x25, 0x0a,
    0x0e, 0x68, 0x61, 0x72, 0x64, 0x77, 0x61, 0x72, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x18,
    0x09, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x68, 0x61, 0x72, 0x64, 0x77, 0x61, 0x72, 0x65, 0x4d,
    0x6f, 0x64, 0x65, 0x6c, 0x12, 0x25, 0x0a, 0x0e, 0x66, 0x69, 0x72, 0x6d, 0x77, 0x61, 0x72, 0x65,
    0x5f, 0x62, 0x72, 0x61, 0x6e, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x66, 0x69,
    0x72, 0x6d, 0x77, 0x61, 0x72, 0x65, 0x42, 0x72, 0x61, 0x6e, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x66,
    0x69, 0x72, 0x6d, 0x77, 0x61, 0x72, 0x65, 0x5f, 0x74, 0x61, 0x67, 0x73, 0x18, 0x0c, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0c, 0x66, 0x69, 0x72, 0x6d, 0x77, 0x61, 0x72, 0x65, 0x54, 0x61, 0x67, 0x73,
    0x12, 0x23, 0x0a, 0x0d, 0x66, 0x69, 0x72, 0x6d, 0x77, 0x61, 0x72, 0x65, 0x5f, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x66, 0x69, 0x72, 0x6d, 0x77, 0x61, 0x72,
    0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x31, 0x0a, 0x14, 0x66, 0x69, 0x72, 0x6d, 0x77, 0x61, 0x72,
    0x65, 0x5f, 0x66, 0x69, 0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x18, 0x0e, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x13, 0x66, 0x69, 0x72, 0x6d, 0x77, 0x61, 0x72, 0x65, 0x46, 0x69, 0x6e,
    0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x1a, 0x9b, 0x02, 0x0a, 0x0e, 0x41, 0x63, 0x74,
    0x69, 0x76, 0x69, 0x74, 0x79, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x22, 0x0a, 0x0d, 0x73,
    0x74, 0x61, 0x72, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x0b, 0x73, 0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x4d, 0x73, 0x12,
    0x25, 0x0a, 0x0e, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0d, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x77, 0x61, 0x6c, 0x6b, 0x69, 0x6e,
    0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x77, 0x61, 0x6c, 0x6b, 0x69, 0x6e, 0x67,
    0x12, 0x18, 0x0a, 0x07, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x07, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x12, 0x1e, 0x0a, 0x0a, 0x73, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x72, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a,
    0x73, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x72, 0x79, 0x12, 0x1e, 0x0a, 0x0a, 0x61, 0x75,
    0x74, 0x6f, 0x6d, 0x6f, 0x74, 0x69, 0x76, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a,
    0x61, 0x75, 0x74, 0x6f, 0x6d, 0x6f, 0x74, 0x69, 0x76, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x74, 0x69,
    0x6c, 0x74, 0x69, 0x6e, 0x67, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x74, 0x69, 0x6c,
    0x74, 0x69, 0x6e, 0x67, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x79, 0x63, 0x6c, 0x69, 0x6e, 0x67, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x63, 0x79, 0x63, 0x6c, 0x69, 0x6e, 0x67, 0x12, 0x16,
    0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x22, 0x64, 0x0a, 0x0a, 0x41, 0x75, 0x74, 0x68, 0x54, 0x69,
    0x63, 0x6b, 0x65, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x72, 0x74, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x05, 0x73, 0x74, 0x61, 0x72, 0x74, 0x12, 0x2e, 0x0a, 0x13, 0x65, 0x78,
    0x70, 0x69, 0x72, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d,
    0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x11, 0x65, 0x78, 0x70, 0x69, 0x72, 0x65, 0x54,
    0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12, 0x10, 0x0a, 0x03, 0x65, 0x6e,
    0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x65, 0x6e, 0x64, 0x22, 0xc1, 0x06, 0x0a,
    0x10, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x36, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x23, 0x0a, 0x0d, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x56, 0x0a, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77,
    0x6e, 0x32, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x3a, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67,
    0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f,
    0x77, 0x6e, 0x36, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x55, 0x6e, 0x6b, 0x6e,
    0x6f, 0x77, 0x6e, 0x32, 0x52, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x32, 0x1a, 0xaf,
    0x05, 0x0a, 0x08, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x32, 0x12, 0x1a, 0x0a, 0x08, 0x75,
    0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x31, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x75,
    0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x31, 0x12, 0x5a, 0x0a, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x44, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45,
    0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e,
    0x36, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77,
    0x6e, 0x32, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x52, 0x05, 0x69, 0x74,
    0x65, 0x6d, 0x73, 0x12, 0x4d, 0x0a, 0x11, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x63, 0x69, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x20,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61,
    0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79,
    0x52, 0x10, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x69,
    0x65, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x34, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x34, 0x1a, 0xbf,
    0x03, 0x0a, 0x09, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x12, 0x17, 0x0a, 0x07,
    0x69, 0x74, 0x65, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x69,
    0x74, 0x65, 0x6d, 0x49, 0x64, 0x12, 0x15, 0x0a, 0x06, 0x69, 0x73, 0x5f, 0x69, 0x61, 0x70, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x05, 0x69, 0x73, 0x49, 0x61, 0x70, 0x12, 0x48, 0x0a, 0x0f,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x5f, 0x74, 0x6f, 0x5f, 0x62, 0x75, 0x79, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x43,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x52, 0x0d, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63,
    0x79, 0x54, 0x6f, 0x42, 0x75, 0x79, 0x12, 0x49, 0x0a, 0x0f, 0x79, 0x69, 0x65, 0x6c, 0x64, 0x73,
    0x5f, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x20, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63,
    0x79, 0x52, 0x0e, 0x79, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63,
    0x79, 0x12, 0x44, 0x0a, 0x0b, 0x79, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x5f, 0x69, 0x74, 0x65, 0x6d,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74,
    0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x52, 0x0a, 0x79, 0x69, 0x65,
    0x6c, 0x64, 0x73, 0x49, 0x74, 0x65, 0x6d, 0x12, 0x5c, 0x0a, 0x04, 0x74, 0x61, 0x67, 0x73, 0x18,
    0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x48, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e,
    0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x36,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e,
    0x32, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x54, 0x61, 0x67, 0x52,
    0x04, 0x74, 0x61, 0x67, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e,
    0x37, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e,
    0x37, 0x1a, 0x2d, 0x0a, 0x03, 0x54, 0x61, 0x67, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x22, 0xb4, 0x05, 0x0a, 0x0f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x45, 0x6e, 0x76, 0x65,
    0x6c, 0x6f, 0x70, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x63,
    0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x49, 0x64, 0x12, 0x43, 0x0a, 0x08, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73,
    0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52,
    0x08, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x12, 0x45, 0x0a, 0x08, 0x75, 0x6e, 0x6b,
    0x6e, 0x6f, 0x77, 0x6e, 0x36, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x29, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x55, 0x6e,
    0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x36, 0x52, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x36,
    0x12, 0x1a, 0x0a, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x01, 0x52, 0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x1c, 0x0a, 0x09,
    0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x01, 0x52,
    0x09, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x61, 0x6c,
    0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x01, 0x52, 0x08, 0x61, 0x6c,
    0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x56, 0x0a, 0x09, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x69,
    0x6e, 0x66, 0x6f, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x39, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e,
    0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x2e, 0x41, 0x75, 0x74, 0x68,
    0x49, 0x6e, 0x66, 0x6f, 0x52, 0x08, 0x61, 0x75, 0x74, 0x68, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x4c,
    0x0a, 0x0b, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x74, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x0b, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65,
    0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74,
    0x52, 0x0a, 0x61, 0x75, 0x74, 0x68, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x1c, 0x0a, 0x09,
    0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x31, 0x32, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x03, 0x52,
    0x09, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x31, 0x32, 0x1a, 0xba, 0x01, 0x0a, 0x08, 0x41,
    0x75, 0x74, 0x68, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x1a, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x76, 0x69,
    0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x70, 0x72, 0x6f, 0x76, 0x69,
    0x64, 0x65, 0x72, 0x12, 0x53, 0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x3d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c,
    0x6f, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x45, 0x6e, 0x76, 0x65,
    0x6c, 0x6f, 0x70, 0x65, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x4a, 0x57,
    0x54, 0x52, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x1a, 0x3d, 0x0a, 0x03, 0x4a, 0x57, 0x54, 0x12,
    0x1a, 0x0a, 0x08, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x08, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x75,
    0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x32, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x75,
    0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x32, 0x22, 0xba, 0x01, 0x0a, 0x08, 0x55, 0x6e, 0x6b, 0x6e,
    0x6f, 0x77, 0x6e, 0x36, 0x12, 0x21, 0x0a, 0x0c, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x4e, 0x0a, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f,
    0x77, 0x6e, 0x32, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e,
    0x67, 0x2e, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x55, 0x6e, 0x6b, 0x6e,
    0x6f, 0x77, 0x6e, 0x36, 0x2e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x32, 0x52, 0x08, 0x75,
    0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x32, 0x1a, 0x3b, 0x0a, 0x08, 0x55, 0x6e, 0x6b, 0x6e, 0x6f,
    0x77, 0x6e, 0x32, 0x12, 0x2f, 0x0a, 0x13, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64,
    0x5f, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x12, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x53, 0x69, 0x67, 0x6e, 0x61,
    0x74, 0x75, 0x72, 0x65, 0x22, 0xb8, 0x02, 0x0a, 0x10, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x49, 0x64, 0x12, 0x17, 0x0a, 0x07, 0x61, 0x70, 0x69,
    0x5f, 0x75, 0x72, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61, 0x70, 0x69, 0x55,
    0x72, 0x6c, 0x12, 0x4d, 0x0a, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x36, 0x18, 0x06,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x31, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45, 0x6e, 0x76,
    0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x36, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x08, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e,
    0x36, 0x12, 0x4c, 0x0a, 0x0b, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x74, 0x69, 0x63, 0x6b, 0x65, 0x74,
    0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x45,
    0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x54, 0x69, 0x63,
    0x6b, 0x65, 0x74, 0x52, 0x0a, 0x61, 0x75, 0x74, 0x68, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x12,
    0x18, 0x0a, 0x07, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x18, 0x64, 0x20, 0x03, 0x28, 0x0c,
    0x52, 0x07, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x18, 0x65, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x4a,
    0xe0, 0x5d, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xb3, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c,
    0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x27, 0x0a,
    0x09, 0x0a, 0x02, 0x0a, 0x00, 0x12, 0x03, 0x03, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00,
    0x12, 0x03, 0x03, 0x0e, 0x2c, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x04, 0x07, 0x0d,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x0e, 0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x0a,
    0x02, 0x12, 0x03, 0x05, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x05, 0x0e,
    0x34, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x66, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03,
    0x00, 0x12, 0x04, 0x09, 0x04, 0x19, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01,
    0x12, 0x03, 0x09, 0x0c, 0x17, 0x0a, 0x3b, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x0a, 0x08, 0x1c, 0x22, 0x2c, 0x20, 0x22, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x22,
    0x2c, 0x20, 0x22, 0x67, 0x70, 0x73, 0x22, 0x2c, 0x20, 0x22, 0x66, 0x75, 0x73, 0x65, 0x64, 0x22,
    0x2c, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x79, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72,
    0x73, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0a,
    0x08, 0x09, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x0a, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x0a, 0x0f, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0a, 0x1a, 0x1b, 0x0a, 0x22, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b,
    0x08, 0x26, 0x22, 0x13, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x73, 0x20, 0x73, 0x69, 0x6e, 0x63, 0x65,
    0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x0b, 0x08, 0x0a, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x0b, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x0f, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x0b, 0x24, 0x25, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x0c, 0x08, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x04, 0x0c, 0x08, 0x0b, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x0c, 0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x0e, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x19, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x0d, 0x08, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x03, 0x04, 0x12, 0x04, 0x0d, 0x08, 0x0c, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x0d, 0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x0e, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x0d, 0x1a, 0x1c, 0x0a, 0x92, 0x01, 0x0a, 0x06, 0x04, 0x00, 0x03,
    0x00, 0x02, 0x04, 0x12, 0x03, 0x12, 0x08, 0x27, 0x1a, 0x59, 0x20, 0x3f, 0x3f, 0x3f, 0x20, 0x73,
    0x68, 0x6f, 0x77, 0x73, 0x20, 0x75, 0x70, 0x20, 0x69, 0x6e, 0x20, 0x73, 0x74, 0x72, 0x75, 0x63,
    0x74, 0x2c, 0x20, 0x64, 0x75, 0x6e, 0x6e, 0x6f, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20, 0x74,
    0x68, 0x65, 0x73, 0x65, 0x20, 0x67, 0x6f, 0x0a, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x20, 0x64,
    0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73, 0x70, 0x65, 0x65, 0x64, 0x3b, 0x0a, 0x20, 0x66, 0x6c,
    0x6f, 0x61, 0x74, 0x20, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x63, 0x6f, 0x75, 0x72, 0x73,
    0x65, 0x3b, 0x0a, 0x22, 0x28, 0x20, 0x69, 0x4f, 0x53, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x3f, 0x20,
    0x28, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x73, 0x65, 0x65, 0x6d, 0x73, 0x20, 0x74, 0x6f, 0x20,
    0x62, 0x65, 0x20, 0x2d, 0x31, 0x20, 0x74, 0x6f, 0x20, 0x2b, 0x31, 0x29, 0x0a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x04, 0x04, 0x12, 0x04, 0x12, 0x08, 0x0d, 0x1d, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x12, 0x08, 0x0d, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x12, 0x0e, 0x21, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x12, 0x24, 0x26, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x05, 0x12, 0x03, 0x13, 0x08, 0x1c, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x05, 0x04, 0x12, 0x04, 0x13, 0x08, 0x12, 0x27, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x13, 0x08, 0x0d, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x13, 0x0e, 0x16, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x13, 0x19, 0x1b, 0x0a, 0x35,
    0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x06, 0x12, 0x03, 0x14, 0x08, 0x25, 0x22, 0x26, 0x20,
    0x69, 0x4f, 0x53, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x3f, 0x20, 0x28, 0x72, 0x61, 0x6e, 0x67, 0x65,
    0x20, 0x73, 0x65, 0x65, 0x6d, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x7e, 0x31, 0x30,
    0x2d, 0x31, 0x32, 0x29, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x06, 0x04,
    0x12, 0x04, 0x14, 0x08, 0x13, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x14, 0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x14, 0x0e, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x06,
    0x03, 0x12, 0x03, 0x14, 0x22, 0x24, 0x0a, 0x68, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x07,
    0x12, 0x03, 0x15, 0x08, 0x24, 0x22, 0x59, 0x20, 0x55, 0x73, 0x75, 0x61, 0x6c, 0x6c, 0x79, 0x20,
    0x33, 0x20, 0x28, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x79, 0x20, 0x47, 0x50, 0x53, 0x20,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x3a, 0x20, 0x31, 0x20, 0x3d, 0x20, 0x6e, 0x6f, 0x20, 0x66,
    0x69, 0x78, 0x2c, 0x20, 0x32, 0x20, 0x3d, 0x20, 0x61, 0x63, 0x71, 0x75, 0x69, 0x72, 0x69, 0x6e,
    0x67, 0x2f, 0x69, 0x6e, 0x61, 0x63, 0x63, 0x75, 0x72, 0x61, 0x74, 0x65, 0x2c, 0x20, 0x33, 0x20,
    0x3d, 0x20, 0x66, 0x69, 0x78, 0x20, 0x61, 0x63, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x29, 0x0a,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x07, 0x04, 0x12, 0x04, 0x15, 0x08, 0x14,
    0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x15, 0x08,
    0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x15, 0x0f,
    0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x15, 0x21,
    0x23, 0x0a, 0x89, 0x01, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x08, 0x12, 0x03, 0x17, 0x08,
    0x1a, 0x1a, 0x4c, 0x20, 0x4f, 0x6e, 0x20, 0x69, 0x4f, 0x53, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65,
    0x20, 0x61, 0x72, 0x65, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x20, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x46, 0x69, 0x78, 0x65, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x75, 0x6e, 0x6b,
    0x32, 0x36, 0x3d, 0x31, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x74, 0x68,
    0x69, 0x6e, 0x67, 0x20, 0x65, 0x6c, 0x73, 0x65, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x0a, 0x22,
    0x2c, 0x20, 0x4e, 0x6f, 0x20, 0x69, 0x64, 0x65, 0x61, 0x20, 0x77, 0x68, 0x61, 0x74, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x2c, 0x20, 0x73, 0x65, 0x65, 0x6d, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x62, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x0a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x08, 0x04, 0x12, 0x04, 0x17, 0x08, 0x15, 0x24, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x17, 0x08, 0x0e, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x17, 0x0f, 0x14, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x17, 0x17, 0x19, 0x0a, 0x33,
    0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x09, 0x12, 0x03, 0x18, 0x08, 0x22, 0x22, 0x24, 0x20,
    0x41, 0x6c, 0x77, 0x61, 0x79, 0x73, 0x20, 0x31, 0x20, 0x28, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x72, 0x65, 0x20, 0x69, 0x73, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x61, 0x74, 0x20, 0x61, 0x6c,
    0x6c, 0x29, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x09, 0x04, 0x12, 0x04,
    0x18, 0x08, 0x17, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x09, 0x05, 0x12,
    0x03, 0x18, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x09, 0x01, 0x12,
    0x03, 0x18, 0x0f, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x09, 0x03, 0x12,
    0x03, 0x18, 0x1f, 0x21, 0x0a, 0x45, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x01, 0x12, 0x04, 0x1c, 0x04,
    0x25, 0x05, 0x1a, 0x37, 0x20, 0x64, 0x6f, 0x6e, 0x27, 0x74, 0x20, 0x72, 0x65, 0x61, 0x6c, 0x6c,
    0x79, 0x20, 0x63, 0x61, 0x72, 0x65, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x73, 0x69, 0x6e, 0x63, 0x65, 0x20, 0x77, 0x65, 0x27, 0x72, 0x65, 0x20, 0x6e, 0x6f,
    0x74, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x03, 0x01, 0x01, 0x12, 0x03, 0x1c, 0x0c, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x08, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x1d, 0x08, 0x1c, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1d, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x0f, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x1d, 0x1e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03,
    0x01, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x1e, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x1e, 0x11, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x17, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x1e, 0x28, 0x29, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x1f, 0x08, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x1f, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x1f, 0x11, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x1f, 0x17, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x1f, 0x1d, 0x1e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02,
    0x03, 0x12, 0x03, 0x20, 0x08, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x20, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x20, 0x11, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x20, 0x17, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x20, 0x21, 0x22, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x04,
    0x12, 0x03, 0x21, 0x08, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x21, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x21, 0x11, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x21, 0x17, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x21, 0x23, 0x24, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x05, 0x12,
    0x03, 0x22, 0x08, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x05, 0x04, 0x12,
    0x03, 0x22, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x05, 0x05, 0x12,
    0x03, 0x22, 0x11, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x22, 0x16, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x22, 0x24, 0x25, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x06, 0x12, 0x03,
    0x23, 0x08, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x06, 0x04, 0x12, 0x03,
    0x23, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03,
    0x23, 0x11, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x23, 0x16, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03,
    0x23, 0x26, 0x27, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x07, 0x12, 0x03, 0x24,
    0x08, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x07, 0x04, 0x12, 0x03, 0x24,
    0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x07, 0x05, 0x12, 0x03, 0x24,
    0x11, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x24,
    0x16, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x07, 0x03, 0x12, 0x03, 0x24,
    0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x02, 0x12, 0x04, 0x27, 0x04, 0x39, 0x05,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x02, 0x01, 0x12, 0x03, 0x27, 0x0c, 0x16, 0x0a, 0x22,
    0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x00, 0x12, 0x03, 0x28, 0x08, 0x26, 0x22, 0x13, 0x20,
    0x69, 0x6e, 0x20, 0x6d, 0x73, 0x20, 0x73, 0x69, 0x6e, 0x63, 0x65, 0x20, 0x73, 0x74, 0x61, 0x72,
    0x74, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x28,
    0x08, 0x27, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x28, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x28, 0x0f, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x28, 0x24, 0x25, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x29,
    0x08, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x29,
    0x08, 0x28, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x29, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x29, 0x0f, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x29, 0x20, 0x21, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x02, 0x12, 0x03, 0x2a,
    0x08, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x02, 0x04, 0x12, 0x04, 0x2a,
    0x08, 0x29, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x2a, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x2a, 0x0f, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x2a, 0x20, 0x21, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2b,
    0x08, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x03, 0x04, 0x12, 0x04, 0x2b,
    0x08, 0x2a, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x2b, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x2b, 0x0f, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x2b, 0x20, 0x21, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2c,
    0x08, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2c,
    0x08, 0x2b, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x2c, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x2c, 0x0f, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x2c, 0x24, 0x25, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2d,
    0x08, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x05, 0x04, 0x12, 0x04, 0x2d,
    0x08, 0x2c, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x05, 0x05, 0x12, 0x03,
    0x2d, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x2d, 0x0f, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03,
    0x2d, 0x24, 0x25, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2e,
    0x08, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x06, 0x04, 0x12, 0x04, 0x2e,
    0x08, 0x2d, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x06, 0x05, 0x12, 0x03,
    0x2e, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x2e, 0x0f, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x06, 0x03, 0x12, 0x03,
    0x2e, 0x24, 0x25, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x07, 0x12, 0x03, 0x2f,
    0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x07, 0x04, 0x12, 0x04, 0x2f,
    0x08, 0x2e, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x07, 0x05, 0x12, 0x03,
    0x2f, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x2f, 0x0f, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x07, 0x03, 0x12, 0x03,
    0x2f, 0x1d, 0x1f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x08, 0x12, 0x03, 0x30,
    0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x08, 0x04, 0x12, 0x04, 0x30,
    0x08, 0x2f, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x08, 0x05, 0x12, 0x03,
    0x30, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x08, 0x01, 0x12, 0x03,
    0x30, 0x0f, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x30, 0x1d, 0x1f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x09, 0x12, 0x03, 0x31,
    0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x09, 0x04, 0x12, 0x04, 0x31,
    0x08, 0x30, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x09, 0x05, 0x12, 0x03,
    0x31, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x09, 0x01, 0x12, 0x03,
    0x31, 0x0f, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x09, 0x03, 0x12, 0x03,
    0x31, 0x1d, 0x1f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0a, 0x12, 0x03, 0x32,
    0x08, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x32,
    0x08, 0x31, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0a, 0x05, 0x12, 0x03,
    0x32, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0a, 0x01, 0x12, 0x03,
    0x32, 0x0f, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0a, 0x03, 0x12, 0x03,
    0x32, 0x21, 0x23, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0b, 0x12, 0x03, 0x33,
    0x08, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0b, 0x04, 0x12, 0x04, 0x33,
    0x08, 0x32, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0b, 0x05, 0x12, 0x03,
    0x33, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0b, 0x01, 0x12, 0x03,
    0x33, 0x0f, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0b, 0x03, 0x12, 0x03,
    0x33, 0x21, 0x23, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0c, 0x12, 0x03, 0x34,
    0x08, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0c, 0x04, 0x12, 0x04, 0x34,
    0x08, 0x33, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0c, 0x05, 0x12, 0x03,
    0x34, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0c, 0x01, 0x12, 0x03,
    0x34, 0x0f, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0c, 0x03, 0x12, 0x03,
    0x34, 0x21, 0x23, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0d, 0x12, 0x03, 0x35,
    0x08, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0d, 0x04, 0x12, 0x04, 0x35,
    0x08, 0x34, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0d, 0x05, 0x12, 0x03,
    0x35, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0d, 0x01, 0x12, 0x03,
    0x35, 0x0f, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0d, 0x03, 0x12, 0x03,
    0x35, 0x24, 0x26, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0e, 0x12, 0x03, 0x36,
    0x08, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0e, 0x04, 0x12, 0x04, 0x36,
    0x08, 0x35, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0e, 0x05, 0x12, 0x03,
    0x36, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0e, 0x01, 0x12, 0x03,
    0x36, 0x0f, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0e, 0x03, 0x12, 0x03,
    0x36, 0x24, 0x26, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0f, 0x12, 0x03, 0x37,
    0x08, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0f, 0x04, 0x12, 0x04, 0x37,
    0x08, 0x36, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0f, 0x05, 0x12, 0x03,
    0x37, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0f, 0x01, 0x12, 0x03,
    0x37, 0x0f, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x0f, 0x03, 0x12, 0x03,
    0x37, 0x24, 0x26, 0x0a, 0x19, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02, 0x02, 0x10, 0x12, 0x03, 0x38,
    0x08, 0x27, 0x22, 0x0a, 0x20, 0x41, 0x6c, 0x77, 0x61, 0x79, 0x73, 0x20, 0x33, 0x0a, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x10, 0x04, 0x12, 0x04, 0x38, 0x08, 0x37, 0x27, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x10, 0x05, 0x12, 0x03, 0x38, 0x08, 0x0e, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x10, 0x01, 0x12, 0x03, 0x38, 0x0f, 0x21, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02, 0x10, 0x03, 0x12, 0x03, 0x38, 0x24, 0x26, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x03, 0x12, 0x04, 0x3b, 0x04, 0x49, 0x05, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x03, 0x03, 0x01, 0x12, 0x03, 0x3b, 0x0c, 0x16, 0x0a, 0x1b, 0x0a, 0x06, 0x04,
    0x00, 0x03, 0x03, 0x02, 0x00, 0x12, 0x03, 0x3c, 0x08, 0x1d, 0x22, 0x0c, 0x20, 0x48, 0x65, 0x78,
    0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x3c, 0x08, 0x3b, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3c, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3c, 0x0f, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3c, 0x1b, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03,
    0x03, 0x02, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x3d, 0x08, 0x3c, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3d, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3d, 0x0f, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3d, 0x24, 0x25, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03,
    0x03, 0x02, 0x02, 0x12, 0x03, 0x3e, 0x08, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x3e, 0x08, 0x3d, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3e, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x0f, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3e, 0x24, 0x25, 0x0a, 0x2a, 0x0a, 0x06, 0x04, 0x00, 0x03,
    0x03, 0x02, 0x03, 0x12, 0x03, 0x3f, 0x08, 0x20, 0x22, 0x1b, 0x20, 0x4f, 0x6e, 0x20, 0x41, 0x6e,
    0x64, 0x72, 0x6f, 0x69, 0x64, 0x3a, 0x20, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x2e, 0x62,
    0x72, 0x61, 0x6e, 0x64, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x03, 0x04,
    0x12, 0x04, 0x3f, 0x08, 0x3e, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x3f, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x3f, 0x0f, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x3f, 0x1e, 0x1f, 0x0a, 0x2b, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x03, 0x02, 0x04,
    0x12, 0x03, 0x40, 0x08, 0x20, 0x22, 0x1c, 0x20, 0x4f, 0x6e, 0x20, 0x41, 0x6e, 0x64, 0x72, 0x6f,
    0x69, 0x64, 0x3a, 0x20, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x2e, 0x64, 0x65, 0x76, 0x69,
    0x63, 0x65, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x04, 0x04, 0x12, 0x04,
    0x40, 0x08, 0x3f, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x40, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x40, 0x0f, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x40, 0x1e, 0x1f, 0x0a, 0x2f, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x03, 0x02, 0x05, 0x12, 0x03,
    0x41, 0x08, 0x2b, 0x22, 0x20, 0x20, 0x41, 0x6e, 0x64, 0x72, 0x6f, 0x69, 0x64, 0x20, 0x6f, 0x6e,
    0x6c, 0x79, 0x2c, 0x20, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x64, 0x69, 0x73, 0x70, 0x6c, 0x61,
    0x79, 0x2e, 0x69, 0x64, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x05, 0x04,
    0x12, 0x04, 0x41, 0x08, 0x40, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x41, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x41, 0x0f, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x41, 0x29, 0x2a, 0x0a, 0x2a, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x03, 0x02, 0x06,
    0x12, 0x03, 0x42, 0x08, 0x25, 0x22, 0x1b, 0x20, 0x4f, 0x6e, 0x20, 0x41, 0x6e, 0x64, 0x72, 0x6f,
    0x69, 0x64, 0x3a, 0x20, 0x62, 0x6f, 0x6f, 0x74, 0x2e, 0x68, 0x61, 0x72, 0x64, 0x77, 0x61, 0x72,
    0x65, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x06, 0x04, 0x12, 0x04, 0x42,
    0x08, 0x41, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x06, 0x05, 0x12, 0x03,
    0x42, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x42, 0x0f, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x06, 0x03, 0x12, 0x03,
    0x42, 0x23, 0x24, 0x0a, 0x31, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x03, 0x02, 0x07, 0x12, 0x03, 0x43,
    0x08, 0x29, 0x22, 0x22, 0x20, 0x4f, 0x6e, 0x20, 0x41, 0x6e, 0x64, 0x72, 0x6f, 0x69, 0x64, 0x3a,
    0x20, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x2e, 0x6d, 0x61, 0x6e, 0x75, 0x66, 0x61, 0x63,
    0x74, 0x75, 0x72, 0x65, 0x72, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x07,
    0x04, 0x12, 0x04, 0x43, 0x08, 0x42, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02,
    0x07, 0x05, 0x12, 0x03, 0x43, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02,
    0x07, 0x01, 0x12, 0x03, 0x43, 0x0f, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02,
    0x07, 0x03, 0x12, 0x03, 0x43, 0x27, 0x28, 0x0a, 0x2a, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x03, 0x02,
    0x08, 0x12, 0x03, 0x44, 0x08, 0x22, 0x22, 0x1b, 0x20, 0x4f, 0x6e, 0x20, 0x41, 0x6e, 0x64, 0x72,
    0x6f, 0x69, 0x64, 0x3a, 0x20, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x2e, 0x6d, 0x6f, 0x64,
    0x65, 0x6c, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x08, 0x04, 0x12, 0x04,
    0x44, 0x08, 0x43, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x08, 0x05, 0x12,
    0x03, 0x44, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x44, 0x0f, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x08, 0x03, 0x12,
    0x03, 0x44, 0x20, 0x21, 0x0a, 0x3e, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x03, 0x02, 0x09, 0x12, 0x03,
    0x45, 0x08, 0x23, 0x22, 0x2f, 0x20, 0x4f, 0x6e, 0x20, 0x41, 0x6e, 0x64, 0x72, 0x6f, 0x69, 0x64,
    0x3a, 0x20, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x2e, 0x6e, 0x61, 0x6d, 0x65, 0x2c, 0x20,
    0x6f, 0x6e, 0x20, 0x69, 0x4f, 0x53, 0x3a, 0x20, 0x22, 0x69, 0x50, 0x68, 0x6f, 0x6e, 0x65, 0x20,
    0x4f, 0x53, 0x22, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x09, 0x04, 0x12,
    0x04, 0x45, 0x08, 0x44, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x09, 0x05,
    0x12, 0x03, 0x45, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x09, 0x01,
    0x12, 0x03, 0x45, 0x0f, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x09, 0x03,
    0x12, 0x03, 0x45, 0x20, 0x22, 0x0a, 0x29, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x03, 0x02, 0x0a, 0x12,
    0x03, 0x46, 0x08, 0x22, 0x22, 0x1a, 0x20, 0x41, 0x6e, 0x64, 0x72, 0x6f, 0x69, 0x64, 0x20, 0x6f,
    0x6e, 0x6c, 0x79, 0x2c, 0x20, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x74, 0x61, 0x67, 0x73, 0x0a,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x46, 0x08, 0x45,
    0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x46, 0x08,
    0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x46, 0x0f,
    0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x46, 0x1f,
    0x21, 0x0a, 0x44, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x03, 0x02, 0x0b, 0x12, 0x03, 0x47, 0x08, 0x22,
    0x22, 0x35, 0x20, 0x4f, 0x6e, 0x20, 0x41, 0x6e, 0x64, 0x72, 0x6f, 0x69, 0x64, 0x3a, 0x20, 0x62,
    0x75, 0x69, 0x6c, 0x64, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20, 0x6f, 0x6e, 0x20, 0x69, 0x4f,
    0x53, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x65, 0x61, 0x64, 0x3a, 0x20, 0x69, 0x4f, 0x53, 0x20, 0x76,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02,
    0x0b, 0x04, 0x12, 0x04, 0x47, 0x08, 0x46, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03,
    0x02, 0x0b, 0x05, 0x12, 0x03, 0x47, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03,
    0x02, 0x0b, 0x01, 0x12, 0x03, 0x47, 0x0f, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03,
    0x02, 0x0b, 0x03, 0x12, 0x03, 0x47, 0x1f, 0x21, 0x0a, 0x30, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x03,
    0x02, 0x0c, 0x12, 0x03, 0x48, 0x08, 0x29, 0x22, 0x21, 0x20, 0x41, 0x6e, 0x64, 0x72, 0x6f, 0x69,
    0x64, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x2c, 0x20, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x66, 0x69,
    0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00,
    0x03, 0x03, 0x02, 0x0c, 0x04, 0x12, 0x04, 0x48, 0x08, 0x47, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x03, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x48, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x03, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x48, 0x0f, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x03, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x48, 0x26, 0x28, 0x0a, 0x46, 0x0a, 0x04, 0x04,
    0x00, 0x03, 0x04, 0x12, 0x04, 0x4c, 0x04, 0x57, 0x05, 0x1a, 0x38, 0x20, 0x4f, 0x6e, 0x6c, 0x79,
    0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x69, 0x4f, 0x53, 0x20, 0x2d, 0x20, 0x41,
    0x6e, 0x64, 0x72, 0x6f, 0x69, 0x64, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x20, 0x73, 0x65, 0x6e, 0x64,
    0x73, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x04, 0x01, 0x12, 0x03, 0x4c, 0x0c,
    0x1a, 0x0a, 0x32, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x04, 0x02, 0x00, 0x12, 0x03, 0x4e, 0x08, 0x21,
    0x1a, 0x23, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x73, 0x65, 0x20,
    0x68, 0x61, 0x64, 0x20, 0x31, 0x20, 0x61, 0x73, 0x20, 0x74, 0x68, 0x65, 0x69, 0x72, 0x20, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x4e, 0x08, 0x4c, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x4e, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x4e, 0x0f, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x4e, 0x1f, 0x20, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x4f, 0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x4f, 0x08, 0x4e, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x4f, 0x08, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x4f, 0x0d, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x4f, 0x1e, 0x1f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x04, 0x02, 0x02,
    0x12, 0x03, 0x50, 0x08, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x50, 0x08, 0x4f, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x50, 0x08, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x50, 0x0d, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x50, 0x17, 0x18, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x04, 0x02, 0x03,
    0x12, 0x03, 0x51, 0x08, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x03, 0x04,
    0x12, 0x04, 0x51, 0x08, 0x50, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x51, 0x08, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x51, 0x0d, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x51, 0x17, 0x18, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x04, 0x02, 0x04,
    0x12, 0x03, 0x52, 0x08, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x52, 0x08, 0x51, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x52, 0x08, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x52, 0x0d, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x52, 0x1a, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x04, 0x02, 0x05,
    0x12, 0x03, 0x53, 0x08, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x05, 0x04,
    0x12, 0x04, 0x53, 0x08, 0x52, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x53, 0x08, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x53, 0x0d, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x53, 0x1a, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x04, 0x02, 0x06,
    0x12, 0x03, 0x54, 0x08, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x06, 0x04,
    0x12, 0x04, 0x54, 0x08, 0x53, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x54, 0x08, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x54, 0x0d, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x06,
    0x03, 0x12, 0x03, 0x54, 0x17, 0x18, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x04, 0x02, 0x07,
    0x12, 0x03, 0x55, 0x08, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x07, 0x04,
    0x12, 0x04, 0x55, 0x08, 0x54, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x07,
    0x05, 0x12, 0x03, 0x55, 0x08, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x07,
    0x01, 0x12, 0x03, 0x55, 0x0d, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x07,
    0x03, 0x12, 0x03, 0x55, 0x17, 0x18, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x04, 0x02, 0x08,
    0x12, 0x03, 0x56, 0x08, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x08, 0x04,
    0x12, 0x04, 0x56, 0x08, 0x55, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x08,
    0x05, 0x12, 0x03, 0x56, 0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x08,
    0x01, 0x12, 0x03, 0x56, 0x0e, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x04, 0x02, 0x08,
    0x03, 0x12, 0x03, 0x56, 0x17, 0x18, 0x0a, 0x14, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x59, 0x04, 0x25, 0x22, 0x07, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x59, 0x04, 0x57, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x59, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x59, 0x0b, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x59, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x5a,
    0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5a, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x5a, 0x0d, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5a, 0x19, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5a, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x5b, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x5b, 0x04, 0x5a, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x5b, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5b,
    0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5b, 0x1e, 0x1f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x5c, 0x04, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x5c, 0x04, 0x5b, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x5c, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x5c, 0x0f, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x5c, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03,
    0x5d, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x04, 0x5d, 0x04,
    0x5c, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x5d, 0x04, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x5d, 0x0f, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x5d, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x5e, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x04, 0x12, 0x04, 0x5e, 0x04, 0x5d, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x06, 0x12, 0x03, 0x5e, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x5e, 0x13, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x5e,
    0x25, 0x26, 0x0a, 0x56, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x5f, 0x04, 0x1f, 0x22,
    0x49, 0x20, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x31, 0x20, 0x68, 0x61, 0x73, 0x68,
    0x65, 0x64, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20, 0x62, 0x61, 0x73, 0x65, 0x64, 0x20,
    0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x74, 0x6f, 0x6b, 0x65,
    0x6e, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x20, 0x2d,
    0x20, 0x78, 0x78, 0x48, 0x61, 0x73, 0x68, 0x33, 0x32, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x06, 0x04, 0x12, 0x04, 0x5f, 0x04, 0x5e, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x06, 0x05, 0x12, 0x03, 0x5f, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01,
    0x12, 0x03, 0x5f, 0x0b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03,
    0x5f, 0x1c, 0x1e, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x60, 0x04, 0x1f,
    0x22, 0x28, 0x20, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x32, 0x20, 0x68, 0x61, 0x73,
    0x68, 0x65, 0x64, 0x20, 0x28, 0x75, 0x6e, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x29, 0x20, 0x2d,
    0x20, 0x78, 0x78, 0x48, 0x61, 0x73, 0x68, 0x33, 0x32, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x04, 0x12, 0x04, 0x60, 0x04, 0x5f, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x05, 0x12, 0x03, 0x60, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x60, 0x0b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03,
    0x60, 0x1c, 0x1e, 0x0a, 0x2b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x61, 0x04, 0x1c,
    0x22, 0x1e, 0x20, 0x31, 0x36, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x2c, 0x20, 0x75, 0x6e, 0x69,
    0x71, 0x75, 0x65, 0x20, 0x70, 0x65, 0x72, 0x20, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x04, 0x61, 0x04, 0x60, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x61, 0x04, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x61, 0x0a, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x61, 0x19, 0x1b, 0x0a, 0x24, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x09, 0x12, 0x03, 0x62, 0x04, 0x1a, 0x22, 0x17, 0x20, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x20, 0x74,
    0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x73, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x04, 0x62, 0x04, 0x61, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x62, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x62, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x09, 0x03, 0x12, 0x03, 0x62, 0x17, 0x19, 0x0a, 0x73, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a,
    0x12, 0x03, 0x63, 0x04, 0x25, 0x22, 0x66, 0x20, 0x68, 0x61, 0x73, 0x68, 0x65, 0x73, 0x20, 0x6f,
    0x66, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x68, 0x61, 0x73, 0x68,
    0x41, 0x72, 0x72, 0x61, 0x79, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20, 0x62, 0x61, 0x73,
    0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x74,
    0x6f, 0x6b, 0x65, 0x6e, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x69, 0x6e, 0x66,
    0x6f, 0x20, 0x2d, 0x20, 0x78, 0x78, 0x68, 0x61, 0x73, 0x68, 0x36, 0x34, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x63, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x63, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0a, 0x01, 0x12, 0x03, 0x63, 0x13, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03,
    0x12, 0x03, 0x63, 0x22, 0x24, 0x0a, 0xb0, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03,
    0x64, 0x04, 0x19, 0x22, 0xa2, 0x01, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x30, 0x2e, 0x33, 0x33, 0x20,
    0x69, 0x74, 0x73, 0x20, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x2d, 0x38, 0x35, 0x33, 0x37,
    0x30, 0x34, 0x32, 0x37, 0x33, 0x34, 0x38, 0x30, 0x39, 0x38, 0x39, 0x37, 0x38, 0x35, 0x35, 0x20,
    0x6f, 0x72, 0x20, 0x30, 0x78, 0x38, 0x39, 0x38, 0x36, 0x35, 0x34, 0x64, 0x64, 0x32, 0x37, 0x35,
    0x33, 0x61, 0x34, 0x38, 0x31, 0x2c, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64,
    0x20, 0x76, 0x69, 0x61, 0x20, 0x78, 0x78, 0x48, 0x61, 0x73, 0x68, 0x36, 0x34, 0x28, 0x22, 0x5c,
    0x22, 0x62, 0x38, 0x66, 0x61, 0x39, 0x37, 0x35, 0x37, 0x31, 0x39, 0x35, 0x38, 0x39, 0x37, 0x61,
    0x61, 0x65, 0x39, 0x32, 0x63, 0x35, 0x33, 0x64, 0x62, 0x63, 0x66, 0x38, 0x61, 0x36, 0x30, 0x66,
    0x62, 0x33, 0x64, 0x38, 0x36, 0x64, 0x37, 0x34, 0x35, 0x5c, 0x22, 0x22, 0x2e, 0x54, 0x6f, 0x42,
    0x79, 0x74, 0x65, 0x41, 0x72, 0x72, 0x61, 0x79, 0x28, 0x29, 0x2c, 0x20, 0x30, 0x78, 0x38, 0x38,
    0x35, 0x33, 0x33, 0x37, 0x38, 0x37, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b,
    0x04, 0x12, 0x04, 0x64, 0x04, 0x63, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x05,
    0x12, 0x03, 0x64, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03,
    0x64, 0x0a, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x64, 0x16,
    0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x67, 0x00, 0x6b, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x67, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x68, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x68, 0x08, 0x67, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x68, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x68, 0x0e,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x68, 0x16, 0x17, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x69, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x69, 0x08, 0x68, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x69, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x69, 0x0f, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x69, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x6a,
    0x08, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0x6a, 0x08, 0x69,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x6a, 0x08, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6a, 0x0e, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6a, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x05, 0x6c, 0x00, 0x85, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x6c, 0x08, 0x18, 0x0a, 0x51, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x6d, 0x08,
    0x20, 0x22, 0x44, 0x20, 0x53, 0x74, 0x69, 0x6c, 0x6c, 0x20, 0x64, 0x6f, 0x6e, 0x27, 0x74, 0x20,
    0x6b, 0x6e, 0x6f, 0x77, 0x20, 0x77, 0x68, 0x61, 0x74, 0x20, 0x36, 0x20, 0x69, 0x73, 0x2c, 0x20,
    0x62, 0x75, 0x74, 0x20, 0x35, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x73, 0x20, 0x69, 0x74, 0x65, 0x6d,
    0x73, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x76, 0x69, 0x61, 0x20,
    0x49, 0x41, 0x50, 0x73, 0x2e, 0x20, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x6d, 0x08, 0x6c, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x6d, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6d,
    0x0e, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6d, 0x1e, 0x1f,
    0x0a, 0x1c, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6e, 0x08, 0x1e, 0x22, 0x0f, 0x20,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x64, 0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x6e, 0x08, 0x6d, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x6e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6e, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x6e, 0x1c, 0x1d, 0x0a, 0x0d, 0x0a, 0x04, 0x04, 0x02, 0x03, 0x00, 0x12,
    0x05, 0x70, 0x08, 0x84, 0x01, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x03, 0x00, 0x01, 0x12,
    0x03, 0x70, 0x10, 0x18, 0x0a, 0x4e, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x71, 0x10, 0x24, 0x22, 0x3f, 0x20, 0x4d, 0x61, 0x79, 0x62, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x3f, 0x20, 0x49, 0x74, 0x27, 0x73, 0x20, 0x61, 0x6c, 0x77, 0x61, 0x79, 0x73, 0x20,
    0x31, 0x20, 0x28, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x29, 0x2c, 0x20, 0x73, 0x6f, 0x20,
    0x69, 0x74, 0x27, 0x73, 0x20, 0x70, 0x72, 0x6f, 0x62, 0x61, 0x62, 0x6c, 0x79, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x71, 0x10, 0x70, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x71, 0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x71, 0x17, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x71, 0x22, 0x23, 0x0a, 0x2a, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x72, 0x10, 0x2d, 0x22, 0x1b, 0x20, 0x49, 0x74, 0x65, 0x6d, 0x73, 0x20, 0x74, 0x6f, 0x20,
    0x73, 0x68, 0x6f, 0x77, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x68, 0x6f, 0x70,
    0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x72, 0x10,
    0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x72, 0x19,
    0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x72, 0x23,
    0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x72, 0x2b,
    0x2c, 0x0a, 0x39, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x73, 0x10, 0x50,
    0x22, 0x2a, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x20, 0x68, 0x61, 0x73, 0x20, 0x61, 0x74,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x6f, 0x6d, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x73, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x73, 0x19, 0x39, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x73, 0x3a, 0x4b, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x73, 0x4e, 0x4f, 0x0a, 0x2d, 0x0a, 0x06,
    0x04, 0x02, 0x03, 0x00, 0x02, 0x03, 0x12, 0x03, 0x74, 0x10, 0x24, 0x22, 0x1e, 0x20, 0x53, 0x6f,
    0x6d, 0x65, 0x20, 0x62, 0x61, 0x73, 0x65, 0x36, 0x34, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65,
    0x64, 0x20, 0x73, 0x74, 0x75, 0x66, 0x66, 0x2e, 0x2e, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x74, 0x10, 0x73, 0x50, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x74, 0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x74, 0x17, 0x1f, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x74, 0x22, 0x23, 0x0a, 0x0f, 0x0a, 0x06,
    0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x12, 0x05, 0x76, 0x10, 0x83, 0x01, 0x11, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x76, 0x18, 0x21, 0x0a, 0x6b, 0x0a,
    0x08, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x77, 0x18, 0x2b, 0x22, 0x5a,
    0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20, 0x49, 0x44, 0x20, 0x28, 0x70, 0x72,
    0x6f, 0x62, 0x61, 0x62, 0x6c, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x47, 0x6f, 0x6f, 0x67, 0x6c,
    0x65, 0x20, 0x50, 0x6c, 0x61, 0x79, 0x2f, 0x41, 0x70, 0x70, 0x20, 0x53, 0x74, 0x6f, 0x72, 0x65,
    0x29, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x3a, 0x20, 0x22, 0x70, 0x67, 0x6f, 0x72,
    0x65, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x2e, 0x69, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x6f, 0x72,
    0x64, 0x69, 0x6e, 0x61, 0x72, 0x79, 0x2e, 0x31, 0x22, 0x0a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x02,
    0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x77, 0x18, 0x76, 0x23, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x77, 0x18, 0x1e, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x77, 0x1f,
    0x26, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x77, 0x29, 0x2a, 0x0a, 0x7e, 0x0a, 0x08, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x78, 0x18, 0x28, 0x22, 0x6d, 0x20, 0x49, 0x66, 0x20, 0x74, 0x72, 0x75, 0x65, 0x2c, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x74, 0x65, 0x6d, 0x20, 0x69, 0x73, 0x20, 0x62, 0x6f, 0x75,
    0x67, 0x68, 0x74, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x72, 0x65, 0x61, 0x6c, 0x20, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x20, 0x28, 0x55, 0x53, 0x44, 0x2c, 0x20, 0x65, 0x74, 0x63,
    0x2e, 0x29, 0x20, 0x74, 0x68, 0x72, 0x6f, 0x75, 0x67, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x50,
    0x6c, 0x61, 0x79, 0x2f, 0x41, 0x70, 0x70, 0x20, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x6e,
    0x73, 0x74, 0x65, 0x61, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x50, 0x6f, 0x6b, 0x65, 0x63, 0x6f, 0x69,
    0x6e, 0x73, 0x0a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x78, 0x18, 0x77, 0x2b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x78, 0x18, 0x1c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00,
    0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x78, 0x1d, 0x23, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02,
    0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x78, 0x26, 0x27, 0x0a, 0xa2, 0x01, 0x0a,
    0x08, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x79, 0x18, 0x4d, 0x22, 0x90,
    0x01, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x68,
    0x6f, 0x77, 0x20, 0x6d, 0x75, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x74, 0x65, 0x6d,
    0x20, 0x63, 0x6f, 0x73, 0x74, 0x73, 0x20, 0x28, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x65, 0x78, 0x63, 0x65, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x69, 0x74,
    0x65, 0x6d, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x6f, 0x73, 0x74, 0x20, 0x72, 0x65,
    0x61, 0x6c, 0x20, 0x6d, 0x6f, 0x6e, 0x65, 0x79, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x50, 0x6f,
    0x6b, 0x65, 0x63, 0x6f, 0x69, 0x6e, 0x73, 0x2c, 0x20, 0x74, 0x68, 0x61, 0x74, 0x27, 0x73, 0x20,
    0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72,
    0x65, 0x73, 0x70, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x29,
    0x0a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x79, 0x18, 0x78, 0x28, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x79, 0x18, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x79, 0x39, 0x48, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00,
    0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x79, 0x4b, 0x4c, 0x0a, 0x46, 0x0a, 0x08, 0x04, 0x02,
    0x03, 0x00, 0x03, 0x00, 0x02, 0x03, 0x12, 0x03, 0x7a, 0x18, 0x4d, 0x22, 0x35, 0x20, 0x57, 0x68,
    0x65, 0x6e, 0x20, 0x62, 0x6f, 0x75, 0x67, 0x68, 0x74, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
    0x49, 0x41, 0x50, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x79, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x6d, 0x75, 0x63, 0x68, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63,
    0x79, 0x0a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12,
    0x04, 0x7a, 0x18, 0x79, 0x4d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x7a, 0x18, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x7a, 0x39, 0x48, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03,
    0x00, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x7a, 0x4b, 0x4c, 0x0a, 0x4b, 0x0a, 0x08, 0x04,
    0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x04, 0x12, 0x03, 0x7b, 0x18, 0x4c, 0x22, 0x3a, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x69, 0x74, 0x65, 0x6d, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x75, 0x63, 0x68, 0x20, 0x69, 0x74, 0x65, 0x6d, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x49, 0x41, 0x50, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x79, 0x69, 0x65, 0x6c, 0x64, 0x0a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00,
    0x03, 0x00, 0x02, 0x04, 0x04, 0x12, 0x04, 0x7b, 0x18, 0x7a, 0x4d, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x7b, 0x18, 0x3b, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x7b, 0x3c, 0x47, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x7b, 0x4a,
    0x4b, 0x0a, 0x35, 0x0a, 0x08, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x05, 0x12, 0x03, 0x7c,
    0x18, 0x2e, 0x22, 0x24, 0x20, 0x53, 0x74, 0x75, 0x66, 0x66, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20,
    0x53, 0x4f, 0x52, 0x54, 0x3a, 0x31, 0x32, 0x2c, 0x20, 0x43, 0x41, 0x54, 0x45, 0x47, 0x4f, 0x52,
    0x59, 0x3a, 0x49, 0x54, 0x45, 0x4d, 0x53, 0x0a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00,
    0x03, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x7c, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02,
    0x03, 0x00, 0x03, 0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x7c, 0x21, 0x24, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x7c, 0x25, 0x29, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x7c, 0x2c, 0x2d,
    0x0a, 0x57, 0x0a, 0x08, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x06, 0x12, 0x03, 0x7d, 0x18,
    0x2b, 0x22, 0x46, 0x20, 0x50, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x79, 0x20, 0x73, 0x6f, 0x6d,
    0x65, 0x74, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x6f, 0x67, 0x67, 0x6c, 0x65,
    0x20, 0x76, 0x69, 0x73, 0x69, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x20, 0x69, 0x6e, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2f, 0x70, 0x75, 0x72, 0x63, 0x68, 0x61, 0x73,
    0x69, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x3f, 0x0a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x02, 0x03,
    0x00, 0x03, 0x00, 0x02, 0x06, 0x04, 0x12, 0x04, 0x7d, 0x18, 0x7c, 0x2e, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x7d, 0x18, 0x1d, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x7d, 0x1e, 0x26,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x7d,
    0x29, 0x2a, 0x0a, 0x11, 0x0a, 0x08, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x12, 0x05,
    0x7f, 0x18, 0x82, 0x01, 0x19, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x03,
    0x00, 0x01, 0x12, 0x03, 0x7f, 0x20, 0x23, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x02, 0x03, 0x00, 0x03,
    0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0x80, 0x01, 0x20, 0x2f, 0x0a, 0x14, 0x0a, 0x0b, 0x04,
    0x02, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x05, 0x80, 0x01, 0x20, 0x7f,
    0x25, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x04, 0x80, 0x01, 0x20, 0x26, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00,
    0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x80, 0x01, 0x27, 0x2a, 0x0a, 0x13, 0x0a, 0x0b, 0x04,
    0x02, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0x80, 0x01, 0x2d, 0x2e,
    0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04,
    0x81, 0x01, 0x20, 0x31, 0x0a, 0x15, 0x0a, 0x0b, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x06, 0x81, 0x01, 0x20, 0x80, 0x01, 0x2f, 0x0a, 0x13, 0x0a, 0x0b, 0x04,
    0x02, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0x81, 0x01, 0x20, 0x26,
    0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x81, 0x01, 0x27, 0x2c, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x03,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0x81, 0x01, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x06, 0x86, 0x01, 0x00, 0x9f, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x04, 0x86, 0x01, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x04, 0x87,
    0x01, 0x08, 0x1e, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x06, 0x87, 0x01,
    0x08, 0x86, 0x01, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x04, 0x87,
    0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x04, 0x87, 0x01,
    0x0e, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x04, 0x87, 0x01, 0x1c,
    0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x04, 0x89, 0x01, 0x08, 0x1e, 0x0a,
    0x0f, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x06, 0x89, 0x01, 0x08, 0x87, 0x01, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x04, 0x89, 0x01, 0x08, 0x0e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x04, 0x89, 0x01, 0x0f, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x04, 0x89, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x06, 0x12, 0x04, 0x8a, 0x01, 0x11, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x39, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x03, 0x12, 0x04, 0x8a, 0x01, 0x44, 0x45, 0x0a, 0xbb, 0x01, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03,
    0x12, 0x04, 0x8e, 0x01, 0x08, 0x3f, 0x1a, 0xac, 0x01, 0x20, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77,
    0x6e, 0x36, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x67, 0x65, 0x74, 0x20, 0x61, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x2e, 0x0a, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c,
    0x65, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f,
    0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6b, 0x65, 0x79, 0x70, 0x68,
    0x61, 0x63, 0x74, 0x2f, 0x70, 0x67, 0x6f, 0x61, 0x70, 0x69, 0x2f, 0x62, 0x6c, 0x6f, 0x62, 0x2f,
    0x37, 0x35, 0x65, 0x62, 0x61, 0x36, 0x62, 0x35, 0x62, 0x36, 0x33, 0x30, 0x38, 0x34, 0x31, 0x65,
    0x65, 0x34, 0x66, 0x37, 0x63, 0x32, 0x65, 0x61, 0x39, 0x38, 0x33, 0x66, 0x31, 0x35, 0x38, 0x37,
    0x34, 0x66, 0x62, 0x30, 0x38, 0x36, 0x32, 0x64, 0x2f, 0x70, 0x67, 0x6f, 0x61, 0x70, 0x69, 0x2f,
    0x72, 0x70, 0x63, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x70, 0x79, 0x23, 0x4c, 0x31, 0x39, 0x32, 0x2d,
    0x4c, 0x32, 0x31, 0x32, 0x0a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x06,
    0x8e, 0x01, 0x08, 0x8a, 0x01, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x06, 0x12,
    0x04, 0x8e, 0x01, 0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x04,
    0x8e, 0x01, 0x32, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8e,
    0x01, 0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x04, 0x8f, 0x01, 0x08,
    0x1c, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x06, 0x8f, 0x01, 0x08, 0x8e,
    0x01, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x04, 0x8f, 0x01, 0x08,
    0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x0f, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x1a, 0x1b, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x05, 0x12, 0x04, 0x90, 0x01, 0x08, 0x1d, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x05, 0x04, 0x12, 0x06, 0x90, 0x01, 0x08, 0x8f, 0x01, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x05, 0x12, 0x04, 0x90, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12, 0x04, 0x90, 0x01, 0x0f, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x05, 0x03, 0x12, 0x04, 0x90, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x06, 0x12, 0x04, 0x91, 0x01, 0x08, 0x1c, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x06, 0x04, 0x12, 0x06, 0x91, 0x01, 0x08, 0x90, 0x01, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x06, 0x05, 0x12, 0x04, 0x91, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x06, 0x01, 0x12, 0x04, 0x91, 0x01, 0x0f, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06,
    0x03, 0x12, 0x04, 0x91, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x07, 0x12,
    0x04, 0x92, 0x01, 0x08, 0x20, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x04, 0x12, 0x06,
    0x92, 0x01, 0x08, 0x91, 0x01, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x06, 0x12,
    0x04, 0x92, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x01, 0x12, 0x04,
    0x92, 0x01, 0x11, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x03, 0x12, 0x04, 0x92,
    0x01, 0x1d, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x08, 0x12, 0x04, 0x93, 0x01, 0x08,
    0x45, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x04, 0x12, 0x06, 0x93, 0x01, 0x08, 0x92,
    0x01, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x06, 0x12, 0x04, 0x93, 0x01, 0x08,
    0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x01, 0x12, 0x04, 0x93, 0x01, 0x34, 0x3f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x03, 0x12, 0x04, 0x93, 0x01, 0x42, 0x44, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x09, 0x12, 0x04, 0x94, 0x01, 0x08, 0x1d, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x09, 0x04, 0x12, 0x06, 0x94, 0x01, 0x08, 0x93, 0x01, 0x45, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x05, 0x12, 0x04, 0x94, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x09, 0x01, 0x12, 0x04, 0x94, 0x01, 0x0e, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x09, 0x03, 0x12, 0x04, 0x94, 0x01, 0x1a, 0x1c, 0x0a, 0x0e, 0x0a, 0x04, 0x04,
    0x03, 0x03, 0x00, 0x12, 0x06, 0x96, 0x01, 0x08, 0x9e, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x03, 0x03, 0x00, 0x01, 0x12, 0x04, 0x96, 0x01, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x03,
    0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0x97, 0x01, 0x10, 0x24, 0x0a, 0x11, 0x0a, 0x07, 0x04, 0x03,
    0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x06, 0x97, 0x01, 0x10, 0x96, 0x01, 0x1a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0x97, 0x01, 0x10, 0x16, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x97, 0x01, 0x17, 0x1f, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0x97, 0x01, 0x22, 0x23,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0x98, 0x01, 0x10, 0x1e,
    0x0a, 0x11, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x06, 0x98, 0x01, 0x10,
    0x97, 0x01, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x04,
    0x98, 0x01, 0x10, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x98, 0x01, 0x14, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x98, 0x01, 0x1c, 0x1d, 0x0a, 0x10, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x03, 0x00,
    0x12, 0x06, 0x9a, 0x01, 0x10, 0x9d, 0x01, 0x11, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00,
    0x03, 0x00, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x18, 0x1b, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x03, 0x03,
    0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0x9b, 0x01, 0x18, 0x2c, 0x0a, 0x13, 0x0a, 0x09, 0x04,
    0x03, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x06, 0x9b, 0x01, 0x18, 0x9a, 0x01, 0x1d,
    0x0a, 0x11, 0x0a, 0x09, 0x04, 0x03, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9b,
    0x01, 0x18, 0x1e, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x03, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x9b, 0x01, 0x1f, 0x27, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x03, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x9b, 0x01, 0x2a, 0x2b, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x03, 0x03,
    0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x18, 0x2c, 0x0a, 0x13, 0x0a, 0x09, 0x04,
    0x03, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x06, 0x9c, 0x01, 0x18, 0x9b, 0x01, 0x2c,
    0x0a, 0x11, 0x0a, 0x09, 0x04, 0x03, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9c,
    0x01, 0x18, 0x1d, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x03, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x04, 0x9c, 0x01, 0x1e, 0x26, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x03, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x9c, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x04, 0x12,
    0x06, 0xa0, 0x01, 0x00, 0xa8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x04,
    0xa0, 0x01, 0x08, 0x10, 0x0a, 0x2e, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x04, 0xa1, 0x01,
    0x08, 0x1f, 0x22, 0x20, 0x20, 0x35, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x49, 0x41, 0x50, 0x73, 0x2c,
    0x20, 0x36, 0x20, 0x69, 0x73, 0x20, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x20, 0x73, 0x74,
    0x69, 0x6c, 0x6c, 0x0a, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x06, 0xa1,
    0x01, 0x08, 0xa0, 0x01, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xa1, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa1,
    0x01, 0x0e, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa1, 0x01,
    0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x1e,
    0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x06, 0xa2, 0x01, 0x08, 0xa1, 0x01,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x11, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa2, 0x01, 0x1c, 0x1d, 0x0a, 0x0e,
    0x0a, 0x04, 0x04, 0x04, 0x03, 0x00, 0x12, 0x06, 0xa4, 0x01, 0x08, 0xa7, 0x01, 0x09, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x04, 0x03, 0x00, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x10, 0x18, 0x0a, 0x62, 0x0a,
    0x06, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xa5, 0x01, 0x10, 0x2e, 0x22, 0x52, 0x20,
    0x54, 0x68, 0x69, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x79, 0x74,
    0x65, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2f, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x2f, 0x45, 0x6e, 0x76, 0x65,
    0x6c, 0x6f, 0x70, 0x65, 0x73, 0x2f, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x20, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x2e,
    0x0a, 0x0a, 0x11, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x06, 0xa5, 0x01,
    0x10, 0xa4, 0x01, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xa5, 0x01, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xa5, 0x01, 0x16, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xa5, 0x01, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x06, 0xa9,
    0x01, 0x00, 0xb3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x04, 0xa9, 0x01,
    0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x1e,
    0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x06, 0xaa, 0x01, 0x08, 0xa9, 0x01,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x0d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x0e, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x04, 0xaa, 0x01, 0x1c, 0x1d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x04, 0xab, 0x01, 0x08, 0x1e, 0x0a, 0x0f, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x06, 0xab, 0x01, 0x08, 0xaa, 0x01, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x04, 0xab, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x04, 0xab, 0x01, 0x0f, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x03, 0x12, 0x04, 0xab, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x02, 0x12, 0x04, 0xac, 0x01, 0x08, 0x1b, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02,
    0x04, 0x12, 0x06, 0xac, 0x01, 0x08, 0xab, 0x01, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x05, 0x12, 0x04, 0xac, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xac, 0x01, 0x0f, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xac, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x04,
    0xae, 0x01, 0x08, 0x50, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x04, 0xae,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x06, 0x12, 0x04, 0xae, 0x01,
    0x11, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x04, 0xae, 0x01, 0x43,
    0x4b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x04, 0xae, 0x01, 0x4e, 0x4f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x04, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x44, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x04, 0x12, 0x06, 0xaf, 0x01, 0x08, 0xae, 0x01, 0x50, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x06, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x33, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x34, 0x3f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x04, 0xaf, 0x01, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x05, 0x12, 0x04, 0xb1, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x05, 0x04, 0x12, 0x04, 0xb1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x05, 0x05, 0x12, 0x04, 0xb1, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05,
    0x01, 0x12, 0x04, 0xb1, 0x01, 0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x03,
    0x12, 0x04, 0xb1, 0x01, 0x21, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x06, 0x12, 0x04,
    0xb2, 0x01, 0x08, 0x1b, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x04, 0x12, 0x06, 0xb2,
    0x01, 0x08, 0xb1, 0x01, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x05, 0x12, 0x04,
    0xb2, 0x01, 0x08, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x01, 0x12, 0x04, 0xb2,
    0x01, 0x0f, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x03, 0x12, 0x04, 0xb2, 0x01,
    0x17, 0x1a, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
