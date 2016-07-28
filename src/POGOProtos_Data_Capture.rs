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
pub struct CaptureProbability {
    // message fields
    pokeball_type: ::std::vec::Vec<super::POGOProtos_Inventory_Item::ItemId>,
    capture_probability: ::std::vec::Vec<f32>,
    reticle_difficulty_scale: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CaptureProbability {}

impl CaptureProbability {
    pub fn new() -> CaptureProbability {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CaptureProbability {
        static mut instance: ::protobuf::lazy::Lazy<CaptureProbability> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CaptureProbability,
        };
        unsafe {
            instance.get(|| {
                CaptureProbability {
                    pokeball_type: ::std::vec::Vec::new(),
                    capture_probability: ::std::vec::Vec::new(),
                    reticle_difficulty_scale: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .POGOProtos.Inventory.Item.ItemId pokeball_type = 1;

    pub fn clear_pokeball_type(&mut self) {
        self.pokeball_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_pokeball_type(&mut self, v: ::std::vec::Vec<super::POGOProtos_Inventory_Item::ItemId>) {
        self.pokeball_type = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pokeball_type(&mut self) -> &mut ::std::vec::Vec<super::POGOProtos_Inventory_Item::ItemId> {
        &mut self.pokeball_type
    }

    // Take field
    pub fn take_pokeball_type(&mut self) -> ::std::vec::Vec<super::POGOProtos_Inventory_Item::ItemId> {
        ::std::mem::replace(&mut self.pokeball_type, ::std::vec::Vec::new())
    }

    pub fn get_pokeball_type(&self) -> &[super::POGOProtos_Inventory_Item::ItemId] {
        &self.pokeball_type
    }

    // repeated float capture_probability = 2;

    pub fn clear_capture_probability(&mut self) {
        self.capture_probability.clear();
    }

    // Param is passed by value, moved
    pub fn set_capture_probability(&mut self, v: ::std::vec::Vec<f32>) {
        self.capture_probability = v;
    }

    // Mutable pointer to the field.
    pub fn mut_capture_probability(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.capture_probability
    }

    // Take field
    pub fn take_capture_probability(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.capture_probability, ::std::vec::Vec::new())
    }

    pub fn get_capture_probability(&self) -> &[f32] {
        &self.capture_probability
    }

    // optional double reticle_difficulty_scale = 12;

    pub fn clear_reticle_difficulty_scale(&mut self) {
        self.reticle_difficulty_scale = ::std::option::Option::None;
    }

    pub fn has_reticle_difficulty_scale(&self) -> bool {
        self.reticle_difficulty_scale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reticle_difficulty_scale(&mut self, v: f64) {
        self.reticle_difficulty_scale = ::std::option::Option::Some(v);
    }

    pub fn get_reticle_difficulty_scale(&self) -> f64 {
        self.reticle_difficulty_scale.unwrap_or(0.)
    }
}

impl ::protobuf::Message for CaptureProbability {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.pokeball_type));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.capture_probability));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.reticle_difficulty_scale = ::std::option::Option::Some(tmp);
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
        if !self.pokeball_type.is_empty() {
            my_size += ::protobuf::rt::vec_packed_enum_size(1, &self.pokeball_type);
        };
        if !self.capture_probability.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.capture_probability.len() as u32) + (self.capture_probability.len() * 4) as u32;
        };
        if self.reticle_difficulty_scale.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.pokeball_type.is_empty() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_enum_data_size(&self.pokeball_type)));
            for v in &self.pokeball_type {
                try!(os.write_enum_no_tag(v.value()));
            };
        };
        if !self.capture_probability.is_empty() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.capture_probability.len() * 4) as u32));
            for v in &self.capture_probability {
                try!(os.write_float_no_tag(*v));
            };
        };
        if let Some(v) = self.reticle_difficulty_scale {
            try!(os.write_double(12, v));
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
        ::std::any::TypeId::of::<CaptureProbability>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CaptureProbability {
    fn new() -> CaptureProbability {
        CaptureProbability::new()
    }

    fn descriptor_static(_: ::std::option::Option<CaptureProbability>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "pokeball_type",
                    CaptureProbability::get_pokeball_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "capture_probability",
                    CaptureProbability::get_capture_probability,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "reticle_difficulty_scale",
                    CaptureProbability::has_reticle_difficulty_scale,
                    CaptureProbability::get_reticle_difficulty_scale,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CaptureProbability>(
                    "CaptureProbability",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CaptureProbability {
    fn clear(&mut self) {
        self.clear_pokeball_type();
        self.clear_capture_probability();
        self.clear_reticle_difficulty_scale();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CaptureProbability {
    fn eq(&self, other: &CaptureProbability) -> bool {
        self.pokeball_type == other.pokeball_type &&
        self.capture_probability == other.capture_probability &&
        self.reticle_difficulty_scale == other.reticle_difficulty_scale &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CaptureProbability {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CaptureAward {
    // message fields
    activity_type: ::std::vec::Vec<super::POGOProtos_Enums::ActivityType>,
    xp: ::std::vec::Vec<i32>,
    candy: ::std::vec::Vec<i32>,
    stardust: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CaptureAward {}

impl CaptureAward {
    pub fn new() -> CaptureAward {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CaptureAward {
        static mut instance: ::protobuf::lazy::Lazy<CaptureAward> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CaptureAward,
        };
        unsafe {
            instance.get(|| {
                CaptureAward {
                    activity_type: ::std::vec::Vec::new(),
                    xp: ::std::vec::Vec::new(),
                    candy: ::std::vec::Vec::new(),
                    stardust: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .POGOProtos.Enums.ActivityType activity_type = 1;

    pub fn clear_activity_type(&mut self) {
        self.activity_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_activity_type(&mut self, v: ::std::vec::Vec<super::POGOProtos_Enums::ActivityType>) {
        self.activity_type = v;
    }

    // Mutable pointer to the field.
    pub fn mut_activity_type(&mut self) -> &mut ::std::vec::Vec<super::POGOProtos_Enums::ActivityType> {
        &mut self.activity_type
    }

    // Take field
    pub fn take_activity_type(&mut self) -> ::std::vec::Vec<super::POGOProtos_Enums::ActivityType> {
        ::std::mem::replace(&mut self.activity_type, ::std::vec::Vec::new())
    }

    pub fn get_activity_type(&self) -> &[super::POGOProtos_Enums::ActivityType] {
        &self.activity_type
    }

    // repeated int32 xp = 2;

    pub fn clear_xp(&mut self) {
        self.xp.clear();
    }

    // Param is passed by value, moved
    pub fn set_xp(&mut self, v: ::std::vec::Vec<i32>) {
        self.xp = v;
    }

    // Mutable pointer to the field.
    pub fn mut_xp(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.xp
    }

    // Take field
    pub fn take_xp(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.xp, ::std::vec::Vec::new())
    }

    pub fn get_xp(&self) -> &[i32] {
        &self.xp
    }

    // repeated int32 candy = 3;

    pub fn clear_candy(&mut self) {
        self.candy.clear();
    }

    // Param is passed by value, moved
    pub fn set_candy(&mut self, v: ::std::vec::Vec<i32>) {
        self.candy = v;
    }

    // Mutable pointer to the field.
    pub fn mut_candy(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.candy
    }

    // Take field
    pub fn take_candy(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.candy, ::std::vec::Vec::new())
    }

    pub fn get_candy(&self) -> &[i32] {
        &self.candy
    }

    // repeated int32 stardust = 4;

    pub fn clear_stardust(&mut self) {
        self.stardust.clear();
    }

    // Param is passed by value, moved
    pub fn set_stardust(&mut self, v: ::std::vec::Vec<i32>) {
        self.stardust = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stardust(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.stardust
    }

    // Take field
    pub fn take_stardust(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.stardust, ::std::vec::Vec::new())
    }

    pub fn get_stardust(&self) -> &[i32] {
        &self.stardust
    }
}

impl ::protobuf::Message for CaptureAward {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.activity_type));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.xp));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.candy));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.stardust));
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
        for value in &self.activity_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.xp {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.candy {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.stardust {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.activity_type {
            try!(os.write_enum(1, v.value()));
        };
        for v in &self.xp {
            try!(os.write_int32(2, *v));
        };
        for v in &self.candy {
            try!(os.write_int32(3, *v));
        };
        for v in &self.stardust {
            try!(os.write_int32(4, *v));
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
        ::std::any::TypeId::of::<CaptureAward>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CaptureAward {
    fn new() -> CaptureAward {
        CaptureAward::new()
    }

    fn descriptor_static(_: ::std::option::Option<CaptureAward>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "activity_type",
                    CaptureAward::get_activity_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "xp",
                    CaptureAward::get_xp,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "candy",
                    CaptureAward::get_candy,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "stardust",
                    CaptureAward::get_stardust,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CaptureAward>(
                    "CaptureAward",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CaptureAward {
    fn clear(&mut self) {
        self.clear_activity_type();
        self.clear_xp();
        self.clear_candy();
        self.clear_stardust();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CaptureAward {
    fn eq(&self, other: &CaptureAward) -> bool {
        self.activity_type == other.activity_type &&
        self.xp == other.xp &&
        self.candy == other.candy &&
        self.stardust == other.stardust &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CaptureAward {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1d, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x2e, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x17, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61,
    0x2e, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x1a, 0x1f, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49,
    0x74, 0x65, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x16, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x50, 0x00, 0x50, 0x01, 0x22, 0xcf, 0x01, 0x0a, 0x12, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72,
    0x65, 0x50, 0x72, 0x6f, 0x62, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x12, 0x4a, 0x0a, 0x0d,
    0x70, 0x6f, 0x6b, 0x65, 0x62, 0x61, 0x6c, 0x6c, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0e, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e,
    0x49, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x42, 0x02, 0x10, 0x01, 0x52, 0x0c, 0x70, 0x6f, 0x6b, 0x65,
    0x62, 0x61, 0x6c, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x12, 0x33, 0x0a, 0x13, 0x63, 0x61, 0x70, 0x74,
    0x75, 0x72, 0x65, 0x5f, 0x70, 0x72, 0x6f, 0x62, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x02, 0x42, 0x02, 0x10, 0x01, 0x52, 0x12, 0x63, 0x61, 0x70, 0x74, 0x75,
    0x72, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x12, 0x38, 0x0a,
    0x18, 0x72, 0x65, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x5f, 0x64, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75,
    0x6c, 0x74, 0x79, 0x5f, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x01, 0x52,
    0x16, 0x72, 0x65, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x44, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c,
    0x74, 0x79, 0x53, 0x63, 0x61, 0x6c, 0x65, 0x22, 0x95, 0x01, 0x0a, 0x0c, 0x43, 0x61, 0x70, 0x74,
    0x75, 0x72, 0x65, 0x41, 0x77, 0x61, 0x72, 0x64, 0x12, 0x43, 0x0a, 0x0d, 0x61, 0x63, 0x74, 0x69,
    0x76, 0x69, 0x74, 0x79, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0e, 0x32,
    0x1e, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75,
    0x6d, 0x73, 0x2e, 0x41, 0x63, 0x74, 0x69, 0x76, 0x69, 0x74, 0x79, 0x54, 0x79, 0x70, 0x65, 0x52,
    0x0c, 0x61, 0x63, 0x74, 0x69, 0x76, 0x69, 0x74, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0e, 0x0a,
    0x02, 0x78, 0x70, 0x18, 0x02, 0x20, 0x03, 0x28, 0x05, 0x52, 0x02, 0x78, 0x70, 0x12, 0x14, 0x0a,
    0x05, 0x63, 0x61, 0x6e, 0x64, 0x79, 0x18, 0x03, 0x20, 0x03, 0x28, 0x05, 0x52, 0x05, 0x63, 0x61,
    0x6e, 0x64, 0x79, 0x12, 0x1a, 0x0a, 0x08, 0x73, 0x74, 0x61, 0x72, 0x64, 0x75, 0x73, 0x74, 0x18,
    0x04, 0x20, 0x03, 0x28, 0x05, 0x52, 0x08, 0x73, 0x74, 0x61, 0x72, 0x64, 0x75, 0x73, 0x74, 0x4a,
    0xb0, 0x06, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x10, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x1f, 0x0a, 0x09,
    0x0a, 0x02, 0x0a, 0x00, 0x12, 0x03, 0x03, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x03, 0x0e, 0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x04, 0x07, 0x0d, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x0e, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x06, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06,
    0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x08, 0x53, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x07, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x07, 0x11, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x33, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x07, 0x43, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08,
    0x12, 0x03, 0x07, 0x45, 0x52, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x07, 0x46, 0x51, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x07, 0x46, 0x4c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x46, 0x4c, 0x0a, 0x12, 0x0a, 0x0b, 0x04,
    0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x46, 0x4c, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x07, 0x4d,
    0x51, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x08, 0x08, 0x3d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x08, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x08, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x08, 0x17, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x08, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12,
    0x03, 0x08, 0x2f, 0x3c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x08, 0x30, 0x3b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x08, 0x30, 0x36, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x01, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x30, 0x36, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x30, 0x36, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x08, 0x37, 0x3b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x09, 0x08, 0x2d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x09, 0x08, 0x08, 0x3d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x09, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x09, 0x0f, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x09, 0x2a, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0b, 0x00,
    0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x14, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x08, 0x42, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x0c, 0x11, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0c, 0x30, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0c, 0x40, 0x41, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0d, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0d, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x17, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02,
    0x12, 0x03, 0x0e, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x0e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0e, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x17, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x1f, 0x20, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x0f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x0f, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x0f, 0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0f,
    0x22, 0x23, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
