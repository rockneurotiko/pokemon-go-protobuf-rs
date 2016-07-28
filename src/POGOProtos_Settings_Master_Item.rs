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
pub struct FoodAttributes {
    // message fields
    item_effect: ::std::vec::Vec<super::POGOProtos_Enums::ItemEffect>,
    item_effect_percent: ::std::vec::Vec<f32>,
    growth_percent: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FoodAttributes {}

impl FoodAttributes {
    pub fn new() -> FoodAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FoodAttributes {
        static mut instance: ::protobuf::lazy::Lazy<FoodAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FoodAttributes,
        };
        unsafe {
            instance.get(|| {
                FoodAttributes {
                    item_effect: ::std::vec::Vec::new(),
                    item_effect_percent: ::std::vec::Vec::new(),
                    growth_percent: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .POGOProtos.Enums.ItemEffect item_effect = 1;

    pub fn clear_item_effect(&mut self) {
        self.item_effect.clear();
    }

    // Param is passed by value, moved
    pub fn set_item_effect(&mut self, v: ::std::vec::Vec<super::POGOProtos_Enums::ItemEffect>) {
        self.item_effect = v;
    }

    // Mutable pointer to the field.
    pub fn mut_item_effect(&mut self) -> &mut ::std::vec::Vec<super::POGOProtos_Enums::ItemEffect> {
        &mut self.item_effect
    }

    // Take field
    pub fn take_item_effect(&mut self) -> ::std::vec::Vec<super::POGOProtos_Enums::ItemEffect> {
        ::std::mem::replace(&mut self.item_effect, ::std::vec::Vec::new())
    }

    pub fn get_item_effect(&self) -> &[super::POGOProtos_Enums::ItemEffect] {
        &self.item_effect
    }

    // repeated float item_effect_percent = 2;

    pub fn clear_item_effect_percent(&mut self) {
        self.item_effect_percent.clear();
    }

    // Param is passed by value, moved
    pub fn set_item_effect_percent(&mut self, v: ::std::vec::Vec<f32>) {
        self.item_effect_percent = v;
    }

    // Mutable pointer to the field.
    pub fn mut_item_effect_percent(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.item_effect_percent
    }

    // Take field
    pub fn take_item_effect_percent(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.item_effect_percent, ::std::vec::Vec::new())
    }

    pub fn get_item_effect_percent(&self) -> &[f32] {
        &self.item_effect_percent
    }

    // optional float growth_percent = 3;

    pub fn clear_growth_percent(&mut self) {
        self.growth_percent = ::std::option::Option::None;
    }

    pub fn has_growth_percent(&self) -> bool {
        self.growth_percent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_growth_percent(&mut self, v: f32) {
        self.growth_percent = ::std::option::Option::Some(v);
    }

    pub fn get_growth_percent(&self) -> f32 {
        self.growth_percent.unwrap_or(0.)
    }
}

impl ::protobuf::Message for FoodAttributes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.item_effect));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.item_effect_percent));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.growth_percent = ::std::option::Option::Some(tmp);
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
        for value in &self.item_effect {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += 5 * self.item_effect_percent.len() as u32;
        if self.growth_percent.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.item_effect {
            try!(os.write_enum(1, v.value()));
        };
        for v in &self.item_effect_percent {
            try!(os.write_float(2, *v));
        };
        if let Some(v) = self.growth_percent {
            try!(os.write_float(3, v));
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
        ::std::any::TypeId::of::<FoodAttributes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FoodAttributes {
    fn new() -> FoodAttributes {
        FoodAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<FoodAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "item_effect",
                    FoodAttributes::get_item_effect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "item_effect_percent",
                    FoodAttributes::get_item_effect_percent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "growth_percent",
                    FoodAttributes::has_growth_percent,
                    FoodAttributes::get_growth_percent,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FoodAttributes>(
                    "FoodAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FoodAttributes {
    fn clear(&mut self) {
        self.clear_item_effect();
        self.clear_item_effect_percent();
        self.clear_growth_percent();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FoodAttributes {
    fn eq(&self, other: &FoodAttributes) -> bool {
        self.item_effect == other.item_effect &&
        self.item_effect_percent == other.item_effect_percent &&
        self.growth_percent == other.growth_percent &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FoodAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReviveAttributes {
    // message fields
    sta_percent: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReviveAttributes {}

impl ReviveAttributes {
    pub fn new() -> ReviveAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReviveAttributes {
        static mut instance: ::protobuf::lazy::Lazy<ReviveAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReviveAttributes,
        };
        unsafe {
            instance.get(|| {
                ReviveAttributes {
                    sta_percent: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float sta_percent = 1;

    pub fn clear_sta_percent(&mut self) {
        self.sta_percent = ::std::option::Option::None;
    }

    pub fn has_sta_percent(&self) -> bool {
        self.sta_percent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sta_percent(&mut self, v: f32) {
        self.sta_percent = ::std::option::Option::Some(v);
    }

    pub fn get_sta_percent(&self) -> f32 {
        self.sta_percent.unwrap_or(0.)
    }
}

impl ::protobuf::Message for ReviveAttributes {
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
                    self.sta_percent = ::std::option::Option::Some(tmp);
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
        if self.sta_percent.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sta_percent {
            try!(os.write_float(1, v));
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
        ::std::any::TypeId::of::<ReviveAttributes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReviveAttributes {
    fn new() -> ReviveAttributes {
        ReviveAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReviveAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "sta_percent",
                    ReviveAttributes::has_sta_percent,
                    ReviveAttributes::get_sta_percent,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReviveAttributes>(
                    "ReviveAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReviveAttributes {
    fn clear(&mut self) {
        self.clear_sta_percent();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReviveAttributes {
    fn eq(&self, other: &ReviveAttributes) -> bool {
        self.sta_percent == other.sta_percent &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReviveAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BattleAttributes {
    // message fields
    sta_percent: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BattleAttributes {}

impl BattleAttributes {
    pub fn new() -> BattleAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BattleAttributes {
        static mut instance: ::protobuf::lazy::Lazy<BattleAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BattleAttributes,
        };
        unsafe {
            instance.get(|| {
                BattleAttributes {
                    sta_percent: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float sta_percent = 1;

    pub fn clear_sta_percent(&mut self) {
        self.sta_percent = ::std::option::Option::None;
    }

    pub fn has_sta_percent(&self) -> bool {
        self.sta_percent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sta_percent(&mut self, v: f32) {
        self.sta_percent = ::std::option::Option::Some(v);
    }

    pub fn get_sta_percent(&self) -> f32 {
        self.sta_percent.unwrap_or(0.)
    }
}

impl ::protobuf::Message for BattleAttributes {
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
                    self.sta_percent = ::std::option::Option::Some(tmp);
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
        if self.sta_percent.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sta_percent {
            try!(os.write_float(1, v));
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
        ::std::any::TypeId::of::<BattleAttributes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BattleAttributes {
    fn new() -> BattleAttributes {
        BattleAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<BattleAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "sta_percent",
                    BattleAttributes::has_sta_percent,
                    BattleAttributes::get_sta_percent,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BattleAttributes>(
                    "BattleAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BattleAttributes {
    fn clear(&mut self) {
        self.clear_sta_percent();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BattleAttributes {
    fn eq(&self, other: &BattleAttributes) -> bool {
        self.sta_percent == other.sta_percent &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BattleAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PotionAttributes {
    // message fields
    sta_percent: ::std::option::Option<f32>,
    sta_amount: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PotionAttributes {}

impl PotionAttributes {
    pub fn new() -> PotionAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PotionAttributes {
        static mut instance: ::protobuf::lazy::Lazy<PotionAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PotionAttributes,
        };
        unsafe {
            instance.get(|| {
                PotionAttributes {
                    sta_percent: ::std::option::Option::None,
                    sta_amount: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float sta_percent = 1;

    pub fn clear_sta_percent(&mut self) {
        self.sta_percent = ::std::option::Option::None;
    }

    pub fn has_sta_percent(&self) -> bool {
        self.sta_percent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sta_percent(&mut self, v: f32) {
        self.sta_percent = ::std::option::Option::Some(v);
    }

    pub fn get_sta_percent(&self) -> f32 {
        self.sta_percent.unwrap_or(0.)
    }

    // optional int32 sta_amount = 2;

    pub fn clear_sta_amount(&mut self) {
        self.sta_amount = ::std::option::Option::None;
    }

    pub fn has_sta_amount(&self) -> bool {
        self.sta_amount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sta_amount(&mut self, v: i32) {
        self.sta_amount = ::std::option::Option::Some(v);
    }

    pub fn get_sta_amount(&self) -> i32 {
        self.sta_amount.unwrap_or(0)
    }
}

impl ::protobuf::Message for PotionAttributes {
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
                    self.sta_percent = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.sta_amount = ::std::option::Option::Some(tmp);
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
        if self.sta_percent.is_some() {
            my_size += 5;
        };
        for value in &self.sta_amount {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sta_percent {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.sta_amount {
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
        ::std::any::TypeId::of::<PotionAttributes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PotionAttributes {
    fn new() -> PotionAttributes {
        PotionAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<PotionAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "sta_percent",
                    PotionAttributes::has_sta_percent,
                    PotionAttributes::get_sta_percent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sta_amount",
                    PotionAttributes::has_sta_amount,
                    PotionAttributes::get_sta_amount,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PotionAttributes>(
                    "PotionAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PotionAttributes {
    fn clear(&mut self) {
        self.clear_sta_percent();
        self.clear_sta_amount();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PotionAttributes {
    fn eq(&self, other: &PotionAttributes) -> bool {
        self.sta_percent == other.sta_percent &&
        self.sta_amount == other.sta_amount &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PotionAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FortModifierAttributes {
    // message fields
    modifier_lifetime_seconds: ::std::option::Option<i32>,
    troy_disk_num_pokemon_spawned: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FortModifierAttributes {}

impl FortModifierAttributes {
    pub fn new() -> FortModifierAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FortModifierAttributes {
        static mut instance: ::protobuf::lazy::Lazy<FortModifierAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FortModifierAttributes,
        };
        unsafe {
            instance.get(|| {
                FortModifierAttributes {
                    modifier_lifetime_seconds: ::std::option::Option::None,
                    troy_disk_num_pokemon_spawned: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 modifier_lifetime_seconds = 1;

    pub fn clear_modifier_lifetime_seconds(&mut self) {
        self.modifier_lifetime_seconds = ::std::option::Option::None;
    }

    pub fn has_modifier_lifetime_seconds(&self) -> bool {
        self.modifier_lifetime_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modifier_lifetime_seconds(&mut self, v: i32) {
        self.modifier_lifetime_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_modifier_lifetime_seconds(&self) -> i32 {
        self.modifier_lifetime_seconds.unwrap_or(0)
    }

    // optional int32 troy_disk_num_pokemon_spawned = 2;

    pub fn clear_troy_disk_num_pokemon_spawned(&mut self) {
        self.troy_disk_num_pokemon_spawned = ::std::option::Option::None;
    }

    pub fn has_troy_disk_num_pokemon_spawned(&self) -> bool {
        self.troy_disk_num_pokemon_spawned.is_some()
    }

    // Param is passed by value, moved
    pub fn set_troy_disk_num_pokemon_spawned(&mut self, v: i32) {
        self.troy_disk_num_pokemon_spawned = ::std::option::Option::Some(v);
    }

    pub fn get_troy_disk_num_pokemon_spawned(&self) -> i32 {
        self.troy_disk_num_pokemon_spawned.unwrap_or(0)
    }
}

impl ::protobuf::Message for FortModifierAttributes {
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
                    self.modifier_lifetime_seconds = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.troy_disk_num_pokemon_spawned = ::std::option::Option::Some(tmp);
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
        for value in &self.modifier_lifetime_seconds {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.troy_disk_num_pokemon_spawned {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.modifier_lifetime_seconds {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.troy_disk_num_pokemon_spawned {
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
        ::std::any::TypeId::of::<FortModifierAttributes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FortModifierAttributes {
    fn new() -> FortModifierAttributes {
        FortModifierAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<FortModifierAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "modifier_lifetime_seconds",
                    FortModifierAttributes::has_modifier_lifetime_seconds,
                    FortModifierAttributes::get_modifier_lifetime_seconds,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "troy_disk_num_pokemon_spawned",
                    FortModifierAttributes::has_troy_disk_num_pokemon_spawned,
                    FortModifierAttributes::get_troy_disk_num_pokemon_spawned,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FortModifierAttributes>(
                    "FortModifierAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FortModifierAttributes {
    fn clear(&mut self) {
        self.clear_modifier_lifetime_seconds();
        self.clear_troy_disk_num_pokemon_spawned();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FortModifierAttributes {
    fn eq(&self, other: &FortModifierAttributes) -> bool {
        self.modifier_lifetime_seconds == other.modifier_lifetime_seconds &&
        self.troy_disk_num_pokemon_spawned == other.troy_disk_num_pokemon_spawned &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FortModifierAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EggIncubatorAttributes {
    // message fields
    incubator_type: ::std::option::Option<super::POGOProtos_Inventory::EggIncubatorType>,
    uses: ::std::option::Option<i32>,
    distance_multiplier: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EggIncubatorAttributes {}

impl EggIncubatorAttributes {
    pub fn new() -> EggIncubatorAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EggIncubatorAttributes {
        static mut instance: ::protobuf::lazy::Lazy<EggIncubatorAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EggIncubatorAttributes,
        };
        unsafe {
            instance.get(|| {
                EggIncubatorAttributes {
                    incubator_type: ::std::option::Option::None,
                    uses: ::std::option::Option::None,
                    distance_multiplier: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Inventory.EggIncubatorType incubator_type = 1;

    pub fn clear_incubator_type(&mut self) {
        self.incubator_type = ::std::option::Option::None;
    }

    pub fn has_incubator_type(&self) -> bool {
        self.incubator_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_incubator_type(&mut self, v: super::POGOProtos_Inventory::EggIncubatorType) {
        self.incubator_type = ::std::option::Option::Some(v);
    }

    pub fn get_incubator_type(&self) -> super::POGOProtos_Inventory::EggIncubatorType {
        self.incubator_type.unwrap_or(super::POGOProtos_Inventory::EggIncubatorType::INCUBATOR_UNSET)
    }

    // optional int32 uses = 2;

    pub fn clear_uses(&mut self) {
        self.uses = ::std::option::Option::None;
    }

    pub fn has_uses(&self) -> bool {
        self.uses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uses(&mut self, v: i32) {
        self.uses = ::std::option::Option::Some(v);
    }

    pub fn get_uses(&self) -> i32 {
        self.uses.unwrap_or(0)
    }

    // optional float distance_multiplier = 3;

    pub fn clear_distance_multiplier(&mut self) {
        self.distance_multiplier = ::std::option::Option::None;
    }

    pub fn has_distance_multiplier(&self) -> bool {
        self.distance_multiplier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_distance_multiplier(&mut self, v: f32) {
        self.distance_multiplier = ::std::option::Option::Some(v);
    }

    pub fn get_distance_multiplier(&self) -> f32 {
        self.distance_multiplier.unwrap_or(0.)
    }
}

impl ::protobuf::Message for EggIncubatorAttributes {
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
                    self.incubator_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.uses = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.distance_multiplier = ::std::option::Option::Some(tmp);
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
        for value in &self.incubator_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.uses {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.distance_multiplier.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.incubator_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.uses {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.distance_multiplier {
            try!(os.write_float(3, v));
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
        ::std::any::TypeId::of::<EggIncubatorAttributes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EggIncubatorAttributes {
    fn new() -> EggIncubatorAttributes {
        EggIncubatorAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<EggIncubatorAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "incubator_type",
                    EggIncubatorAttributes::has_incubator_type,
                    EggIncubatorAttributes::get_incubator_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "uses",
                    EggIncubatorAttributes::has_uses,
                    EggIncubatorAttributes::get_uses,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "distance_multiplier",
                    EggIncubatorAttributes::has_distance_multiplier,
                    EggIncubatorAttributes::get_distance_multiplier,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EggIncubatorAttributes>(
                    "EggIncubatorAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EggIncubatorAttributes {
    fn clear(&mut self) {
        self.clear_incubator_type();
        self.clear_uses();
        self.clear_distance_multiplier();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EggIncubatorAttributes {
    fn eq(&self, other: &EggIncubatorAttributes) -> bool {
        self.incubator_type == other.incubator_type &&
        self.uses == other.uses &&
        self.distance_multiplier == other.distance_multiplier &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EggIncubatorAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct IncenseAttributes {
    // message fields
    incense_lifetime_seconds: ::std::option::Option<i32>,
    pokemon_type: ::std::vec::Vec<super::POGOProtos_Enums::PokemonType>,
    pokemon_incense_type_probability: ::std::option::Option<f32>,
    standing_time_between_encounters_seconds: ::std::option::Option<i32>,
    moving_time_between_encounter_seconds: ::std::option::Option<i32>,
    distance_required_for_shorter_interval_meters: ::std::option::Option<i32>,
    pokemon_attracted_length_sec: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IncenseAttributes {}

impl IncenseAttributes {
    pub fn new() -> IncenseAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IncenseAttributes {
        static mut instance: ::protobuf::lazy::Lazy<IncenseAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IncenseAttributes,
        };
        unsafe {
            instance.get(|| {
                IncenseAttributes {
                    incense_lifetime_seconds: ::std::option::Option::None,
                    pokemon_type: ::std::vec::Vec::new(),
                    pokemon_incense_type_probability: ::std::option::Option::None,
                    standing_time_between_encounters_seconds: ::std::option::Option::None,
                    moving_time_between_encounter_seconds: ::std::option::Option::None,
                    distance_required_for_shorter_interval_meters: ::std::option::Option::None,
                    pokemon_attracted_length_sec: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 incense_lifetime_seconds = 1;

    pub fn clear_incense_lifetime_seconds(&mut self) {
        self.incense_lifetime_seconds = ::std::option::Option::None;
    }

    pub fn has_incense_lifetime_seconds(&self) -> bool {
        self.incense_lifetime_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_incense_lifetime_seconds(&mut self, v: i32) {
        self.incense_lifetime_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_incense_lifetime_seconds(&self) -> i32 {
        self.incense_lifetime_seconds.unwrap_or(0)
    }

    // repeated .POGOProtos.Enums.PokemonType pokemon_type = 2;

    pub fn clear_pokemon_type(&mut self) {
        self.pokemon_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_pokemon_type(&mut self, v: ::std::vec::Vec<super::POGOProtos_Enums::PokemonType>) {
        self.pokemon_type = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pokemon_type(&mut self) -> &mut ::std::vec::Vec<super::POGOProtos_Enums::PokemonType> {
        &mut self.pokemon_type
    }

    // Take field
    pub fn take_pokemon_type(&mut self) -> ::std::vec::Vec<super::POGOProtos_Enums::PokemonType> {
        ::std::mem::replace(&mut self.pokemon_type, ::std::vec::Vec::new())
    }

    pub fn get_pokemon_type(&self) -> &[super::POGOProtos_Enums::PokemonType] {
        &self.pokemon_type
    }

    // optional float pokemon_incense_type_probability = 3;

    pub fn clear_pokemon_incense_type_probability(&mut self) {
        self.pokemon_incense_type_probability = ::std::option::Option::None;
    }

    pub fn has_pokemon_incense_type_probability(&self) -> bool {
        self.pokemon_incense_type_probability.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_incense_type_probability(&mut self, v: f32) {
        self.pokemon_incense_type_probability = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_incense_type_probability(&self) -> f32 {
        self.pokemon_incense_type_probability.unwrap_or(0.)
    }

    // optional int32 standing_time_between_encounters_seconds = 4;

    pub fn clear_standing_time_between_encounters_seconds(&mut self) {
        self.standing_time_between_encounters_seconds = ::std::option::Option::None;
    }

    pub fn has_standing_time_between_encounters_seconds(&self) -> bool {
        self.standing_time_between_encounters_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_standing_time_between_encounters_seconds(&mut self, v: i32) {
        self.standing_time_between_encounters_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_standing_time_between_encounters_seconds(&self) -> i32 {
        self.standing_time_between_encounters_seconds.unwrap_or(0)
    }

    // optional int32 moving_time_between_encounter_seconds = 5;

    pub fn clear_moving_time_between_encounter_seconds(&mut self) {
        self.moving_time_between_encounter_seconds = ::std::option::Option::None;
    }

    pub fn has_moving_time_between_encounter_seconds(&self) -> bool {
        self.moving_time_between_encounter_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_moving_time_between_encounter_seconds(&mut self, v: i32) {
        self.moving_time_between_encounter_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_moving_time_between_encounter_seconds(&self) -> i32 {
        self.moving_time_between_encounter_seconds.unwrap_or(0)
    }

    // optional int32 distance_required_for_shorter_interval_meters = 6;

    pub fn clear_distance_required_for_shorter_interval_meters(&mut self) {
        self.distance_required_for_shorter_interval_meters = ::std::option::Option::None;
    }

    pub fn has_distance_required_for_shorter_interval_meters(&self) -> bool {
        self.distance_required_for_shorter_interval_meters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_distance_required_for_shorter_interval_meters(&mut self, v: i32) {
        self.distance_required_for_shorter_interval_meters = ::std::option::Option::Some(v);
    }

    pub fn get_distance_required_for_shorter_interval_meters(&self) -> i32 {
        self.distance_required_for_shorter_interval_meters.unwrap_or(0)
    }

    // optional int32 pokemon_attracted_length_sec = 7;

    pub fn clear_pokemon_attracted_length_sec(&mut self) {
        self.pokemon_attracted_length_sec = ::std::option::Option::None;
    }

    pub fn has_pokemon_attracted_length_sec(&self) -> bool {
        self.pokemon_attracted_length_sec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_attracted_length_sec(&mut self, v: i32) {
        self.pokemon_attracted_length_sec = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_attracted_length_sec(&self) -> i32 {
        self.pokemon_attracted_length_sec.unwrap_or(0)
    }
}

impl ::protobuf::Message for IncenseAttributes {
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
                    self.incense_lifetime_seconds = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.pokemon_type));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.pokemon_incense_type_probability = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.standing_time_between_encounters_seconds = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.moving_time_between_encounter_seconds = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.distance_required_for_shorter_interval_meters = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.pokemon_attracted_length_sec = ::std::option::Option::Some(tmp);
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
        for value in &self.incense_lifetime_seconds {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pokemon_type {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        if self.pokemon_incense_type_probability.is_some() {
            my_size += 5;
        };
        for value in &self.standing_time_between_encounters_seconds {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.moving_time_between_encounter_seconds {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.distance_required_for_shorter_interval_meters {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pokemon_attracted_length_sec {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.incense_lifetime_seconds {
            try!(os.write_int32(1, v));
        };
        for v in &self.pokemon_type {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.pokemon_incense_type_probability {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.standing_time_between_encounters_seconds {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.moving_time_between_encounter_seconds {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.distance_required_for_shorter_interval_meters {
            try!(os.write_int32(6, v));
        };
        if let Some(v) = self.pokemon_attracted_length_sec {
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
        ::std::any::TypeId::of::<IncenseAttributes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IncenseAttributes {
    fn new() -> IncenseAttributes {
        IncenseAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<IncenseAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "incense_lifetime_seconds",
                    IncenseAttributes::has_incense_lifetime_seconds,
                    IncenseAttributes::get_incense_lifetime_seconds,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "pokemon_type",
                    IncenseAttributes::get_pokemon_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "pokemon_incense_type_probability",
                    IncenseAttributes::has_pokemon_incense_type_probability,
                    IncenseAttributes::get_pokemon_incense_type_probability,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "standing_time_between_encounters_seconds",
                    IncenseAttributes::has_standing_time_between_encounters_seconds,
                    IncenseAttributes::get_standing_time_between_encounters_seconds,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "moving_time_between_encounter_seconds",
                    IncenseAttributes::has_moving_time_between_encounter_seconds,
                    IncenseAttributes::get_moving_time_between_encounter_seconds,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "distance_required_for_shorter_interval_meters",
                    IncenseAttributes::has_distance_required_for_shorter_interval_meters,
                    IncenseAttributes::get_distance_required_for_shorter_interval_meters,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "pokemon_attracted_length_sec",
                    IncenseAttributes::has_pokemon_attracted_length_sec,
                    IncenseAttributes::get_pokemon_attracted_length_sec,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IncenseAttributes>(
                    "IncenseAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IncenseAttributes {
    fn clear(&mut self) {
        self.clear_incense_lifetime_seconds();
        self.clear_pokemon_type();
        self.clear_pokemon_incense_type_probability();
        self.clear_standing_time_between_encounters_seconds();
        self.clear_moving_time_between_encounter_seconds();
        self.clear_distance_required_for_shorter_interval_meters();
        self.clear_pokemon_attracted_length_sec();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IncenseAttributes {
    fn eq(&self, other: &IncenseAttributes) -> bool {
        self.incense_lifetime_seconds == other.incense_lifetime_seconds &&
        self.pokemon_type == other.pokemon_type &&
        self.pokemon_incense_type_probability == other.pokemon_incense_type_probability &&
        self.standing_time_between_encounters_seconds == other.standing_time_between_encounters_seconds &&
        self.moving_time_between_encounter_seconds == other.moving_time_between_encounter_seconds &&
        self.distance_required_for_shorter_interval_meters == other.distance_required_for_shorter_interval_meters &&
        self.pokemon_attracted_length_sec == other.pokemon_attracted_length_sec &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IncenseAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InventoryUpgradeAttributes {
    // message fields
    additional_storage: ::std::option::Option<i32>,
    upgrade_type: ::std::option::Option<super::POGOProtos_Inventory::InventoryUpgradeType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InventoryUpgradeAttributes {}

impl InventoryUpgradeAttributes {
    pub fn new() -> InventoryUpgradeAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InventoryUpgradeAttributes {
        static mut instance: ::protobuf::lazy::Lazy<InventoryUpgradeAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InventoryUpgradeAttributes,
        };
        unsafe {
            instance.get(|| {
                InventoryUpgradeAttributes {
                    additional_storage: ::std::option::Option::None,
                    upgrade_type: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 additional_storage = 1;

    pub fn clear_additional_storage(&mut self) {
        self.additional_storage = ::std::option::Option::None;
    }

    pub fn has_additional_storage(&self) -> bool {
        self.additional_storage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_additional_storage(&mut self, v: i32) {
        self.additional_storage = ::std::option::Option::Some(v);
    }

    pub fn get_additional_storage(&self) -> i32 {
        self.additional_storage.unwrap_or(0)
    }

    // optional .POGOProtos.Inventory.InventoryUpgradeType upgrade_type = 2;

    pub fn clear_upgrade_type(&mut self) {
        self.upgrade_type = ::std::option::Option::None;
    }

    pub fn has_upgrade_type(&self) -> bool {
        self.upgrade_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upgrade_type(&mut self, v: super::POGOProtos_Inventory::InventoryUpgradeType) {
        self.upgrade_type = ::std::option::Option::Some(v);
    }

    pub fn get_upgrade_type(&self) -> super::POGOProtos_Inventory::InventoryUpgradeType {
        self.upgrade_type.unwrap_or(super::POGOProtos_Inventory::InventoryUpgradeType::UPGRADE_UNSET)
    }
}

impl ::protobuf::Message for InventoryUpgradeAttributes {
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
                    self.additional_storage = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.upgrade_type = ::std::option::Option::Some(tmp);
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
        for value in &self.additional_storage {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.upgrade_type {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.additional_storage {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.upgrade_type {
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
        ::std::any::TypeId::of::<InventoryUpgradeAttributes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InventoryUpgradeAttributes {
    fn new() -> InventoryUpgradeAttributes {
        InventoryUpgradeAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<InventoryUpgradeAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "additional_storage",
                    InventoryUpgradeAttributes::has_additional_storage,
                    InventoryUpgradeAttributes::get_additional_storage,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "upgrade_type",
                    InventoryUpgradeAttributes::has_upgrade_type,
                    InventoryUpgradeAttributes::get_upgrade_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InventoryUpgradeAttributes>(
                    "InventoryUpgradeAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InventoryUpgradeAttributes {
    fn clear(&mut self) {
        self.clear_additional_storage();
        self.clear_upgrade_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InventoryUpgradeAttributes {
    fn eq(&self, other: &InventoryUpgradeAttributes) -> bool {
        self.additional_storage == other.additional_storage &&
        self.upgrade_type == other.upgrade_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InventoryUpgradeAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PokeballAttributes {
    // message fields
    item_effect: ::std::option::Option<super::POGOProtos_Enums::ItemEffect>,
    capture_multi: ::std::option::Option<f32>,
    capture_multi_effect: ::std::option::Option<f32>,
    item_effect_mod: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PokeballAttributes {}

impl PokeballAttributes {
    pub fn new() -> PokeballAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PokeballAttributes {
        static mut instance: ::protobuf::lazy::Lazy<PokeballAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PokeballAttributes,
        };
        unsafe {
            instance.get(|| {
                PokeballAttributes {
                    item_effect: ::std::option::Option::None,
                    capture_multi: ::std::option::Option::None,
                    capture_multi_effect: ::std::option::Option::None,
                    item_effect_mod: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Enums.ItemEffect item_effect = 1;

    pub fn clear_item_effect(&mut self) {
        self.item_effect = ::std::option::Option::None;
    }

    pub fn has_item_effect(&self) -> bool {
        self.item_effect.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_effect(&mut self, v: super::POGOProtos_Enums::ItemEffect) {
        self.item_effect = ::std::option::Option::Some(v);
    }

    pub fn get_item_effect(&self) -> super::POGOProtos_Enums::ItemEffect {
        self.item_effect.unwrap_or(super::POGOProtos_Enums::ItemEffect::ITEM_EFFECT_NONE)
    }

    // optional float capture_multi = 2;

    pub fn clear_capture_multi(&mut self) {
        self.capture_multi = ::std::option::Option::None;
    }

    pub fn has_capture_multi(&self) -> bool {
        self.capture_multi.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capture_multi(&mut self, v: f32) {
        self.capture_multi = ::std::option::Option::Some(v);
    }

    pub fn get_capture_multi(&self) -> f32 {
        self.capture_multi.unwrap_or(0.)
    }

    // optional float capture_multi_effect = 3;

    pub fn clear_capture_multi_effect(&mut self) {
        self.capture_multi_effect = ::std::option::Option::None;
    }

    pub fn has_capture_multi_effect(&self) -> bool {
        self.capture_multi_effect.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capture_multi_effect(&mut self, v: f32) {
        self.capture_multi_effect = ::std::option::Option::Some(v);
    }

    pub fn get_capture_multi_effect(&self) -> f32 {
        self.capture_multi_effect.unwrap_or(0.)
    }

    // optional float item_effect_mod = 4;

    pub fn clear_item_effect_mod(&mut self) {
        self.item_effect_mod = ::std::option::Option::None;
    }

    pub fn has_item_effect_mod(&self) -> bool {
        self.item_effect_mod.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_effect_mod(&mut self, v: f32) {
        self.item_effect_mod = ::std::option::Option::Some(v);
    }

    pub fn get_item_effect_mod(&self) -> f32 {
        self.item_effect_mod.unwrap_or(0.)
    }
}

impl ::protobuf::Message for PokeballAttributes {
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
                    self.item_effect = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.capture_multi = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.capture_multi_effect = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.item_effect_mod = ::std::option::Option::Some(tmp);
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
        for value in &self.item_effect {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        if self.capture_multi.is_some() {
            my_size += 5;
        };
        if self.capture_multi_effect.is_some() {
            my_size += 5;
        };
        if self.item_effect_mod.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_effect {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.capture_multi {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.capture_multi_effect {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.item_effect_mod {
            try!(os.write_float(4, v));
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
        ::std::any::TypeId::of::<PokeballAttributes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PokeballAttributes {
    fn new() -> PokeballAttributes {
        PokeballAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<PokeballAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "item_effect",
                    PokeballAttributes::has_item_effect,
                    PokeballAttributes::get_item_effect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "capture_multi",
                    PokeballAttributes::has_capture_multi,
                    PokeballAttributes::get_capture_multi,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "capture_multi_effect",
                    PokeballAttributes::has_capture_multi_effect,
                    PokeballAttributes::get_capture_multi_effect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "item_effect_mod",
                    PokeballAttributes::has_item_effect_mod,
                    PokeballAttributes::get_item_effect_mod,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PokeballAttributes>(
                    "PokeballAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PokeballAttributes {
    fn clear(&mut self) {
        self.clear_item_effect();
        self.clear_capture_multi();
        self.clear_capture_multi_effect();
        self.clear_item_effect_mod();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PokeballAttributes {
    fn eq(&self, other: &PokeballAttributes) -> bool {
        self.item_effect == other.item_effect &&
        self.capture_multi == other.capture_multi &&
        self.capture_multi_effect == other.capture_multi_effect &&
        self.item_effect_mod == other.item_effect_mod &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PokeballAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExperienceBoostAttributes {
    // message fields
    xp_multiplier: ::std::option::Option<f32>,
    boost_duration_ms: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExperienceBoostAttributes {}

impl ExperienceBoostAttributes {
    pub fn new() -> ExperienceBoostAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExperienceBoostAttributes {
        static mut instance: ::protobuf::lazy::Lazy<ExperienceBoostAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExperienceBoostAttributes,
        };
        unsafe {
            instance.get(|| {
                ExperienceBoostAttributes {
                    xp_multiplier: ::std::option::Option::None,
                    boost_duration_ms: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float xp_multiplier = 1;

    pub fn clear_xp_multiplier(&mut self) {
        self.xp_multiplier = ::std::option::Option::None;
    }

    pub fn has_xp_multiplier(&self) -> bool {
        self.xp_multiplier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xp_multiplier(&mut self, v: f32) {
        self.xp_multiplier = ::std::option::Option::Some(v);
    }

    pub fn get_xp_multiplier(&self) -> f32 {
        self.xp_multiplier.unwrap_or(0.)
    }

    // optional int32 boost_duration_ms = 2;

    pub fn clear_boost_duration_ms(&mut self) {
        self.boost_duration_ms = ::std::option::Option::None;
    }

    pub fn has_boost_duration_ms(&self) -> bool {
        self.boost_duration_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_boost_duration_ms(&mut self, v: i32) {
        self.boost_duration_ms = ::std::option::Option::Some(v);
    }

    pub fn get_boost_duration_ms(&self) -> i32 {
        self.boost_duration_ms.unwrap_or(0)
    }
}

impl ::protobuf::Message for ExperienceBoostAttributes {
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
                    self.xp_multiplier = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.boost_duration_ms = ::std::option::Option::Some(tmp);
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
        if self.xp_multiplier.is_some() {
            my_size += 5;
        };
        for value in &self.boost_duration_ms {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.xp_multiplier {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.boost_duration_ms {
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
        ::std::any::TypeId::of::<ExperienceBoostAttributes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExperienceBoostAttributes {
    fn new() -> ExperienceBoostAttributes {
        ExperienceBoostAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExperienceBoostAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "xp_multiplier",
                    ExperienceBoostAttributes::has_xp_multiplier,
                    ExperienceBoostAttributes::get_xp_multiplier,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "boost_duration_ms",
                    ExperienceBoostAttributes::has_boost_duration_ms,
                    ExperienceBoostAttributes::get_boost_duration_ms,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExperienceBoostAttributes>(
                    "ExperienceBoostAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExperienceBoostAttributes {
    fn clear(&mut self) {
        self.clear_xp_multiplier();
        self.clear_boost_duration_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExperienceBoostAttributes {
    fn eq(&self, other: &ExperienceBoostAttributes) -> bool {
        self.xp_multiplier == other.xp_multiplier &&
        self.boost_duration_ms == other.boost_duration_ms &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExperienceBoostAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x25, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74,
    0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x49, 0x74, 0x65,
    0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1f, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x1a, 0x16, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x1a, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76,
    0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x00, 0x50, 0x01,
    0x22, 0xa6, 0x01, 0x0a, 0x0e, 0x46, 0x6f, 0x6f, 0x64, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75,
    0x74, 0x65, 0x73, 0x12, 0x3d, 0x0a, 0x0b, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x65, 0x66, 0x66, 0x65,
    0x63, 0x74, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x1c, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x49, 0x74, 0x65, 0x6d,
    0x45, 0x66, 0x66, 0x65, 0x63, 0x74, 0x52, 0x0a, 0x69, 0x74, 0x65, 0x6d, 0x45, 0x66, 0x66, 0x65,
    0x63, 0x74, 0x12, 0x2e, 0x0a, 0x13, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x65, 0x66, 0x66, 0x65, 0x63,
    0x74, 0x5f, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x03, 0x28, 0x02, 0x52,
    0x11, 0x69, 0x74, 0x65, 0x6d, 0x45, 0x66, 0x66, 0x65, 0x63, 0x74, 0x50, 0x65, 0x72, 0x63, 0x65,
    0x6e, 0x74, 0x12, 0x25, 0x0a, 0x0e, 0x67, 0x72, 0x6f, 0x77, 0x74, 0x68, 0x5f, 0x70, 0x65, 0x72,
    0x63, 0x65, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0d, 0x67, 0x72, 0x6f, 0x77,
    0x74, 0x68, 0x50, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x22, 0x33, 0x0a, 0x10, 0x52, 0x65, 0x76,
    0x69, 0x76, 0x65, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x1f, 0x0a,
    0x0b, 0x73, 0x74, 0x61, 0x5f, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x02, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x50, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x22, 0x33,
    0x0a, 0x10, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74,
    0x65, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x5f, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e,
    0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x50, 0x65, 0x72, 0x63,
    0x65, 0x6e, 0x74, 0x22, 0x52, 0x0a, 0x10, 0x50, 0x6f, 0x74, 0x69, 0x6f, 0x6e, 0x41, 0x74, 0x74,
    0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x5f, 0x70,
    0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0a, 0x73, 0x74,
    0x61, 0x50, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x74, 0x61, 0x5f,
    0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x09, 0x73, 0x74,
    0x61, 0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x22, 0x96, 0x01, 0x0a, 0x16, 0x46, 0x6f, 0x72, 0x74,
    0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74,
    0x65, 0x73, 0x12, 0x3a, 0x0a, 0x19, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x5f, 0x6c,
    0x69, 0x66, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x17, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x4c,
    0x69, 0x66, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x12, 0x40,
    0x0a, 0x1d, 0x74, 0x72, 0x6f, 0x79, 0x5f, 0x64, 0x69, 0x73, 0x6b, 0x5f, 0x6e, 0x75, 0x6d, 0x5f,
    0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x65, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x19, 0x74, 0x72, 0x6f, 0x79, 0x44, 0x69, 0x73, 0x6b, 0x4e,
    0x75, 0x6d, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x53, 0x70, 0x61, 0x77, 0x6e, 0x65, 0x64,
    0x22, 0xac, 0x01, 0x0a, 0x16, 0x45, 0x67, 0x67, 0x49, 0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f,
    0x72, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x4d, 0x0a, 0x0e, 0x69,
    0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x26, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x45, 0x67, 0x67, 0x49, 0x6e,
    0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0d, 0x69, 0x6e, 0x63,
    0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x54, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x75, 0x73,
    0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x75, 0x73, 0x65, 0x73, 0x12, 0x2f,
    0x0a, 0x13, 0x64, 0x69, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x5f, 0x6d, 0x75, 0x6c, 0x74, 0x69,
    0x70, 0x6c, 0x69, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x12, 0x64, 0x69, 0x73,
    0x74, 0x61, 0x6e, 0x63, 0x65, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x69, 0x65, 0x72, 0x22,
    0xa4, 0x04, 0x0a, 0x11, 0x49, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x41, 0x74, 0x74, 0x72, 0x69,
    0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x38, 0x0a, 0x18, 0x69, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65,
    0x5f, 0x6c, 0x69, 0x66, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64,
    0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x16, 0x69, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65,
    0x4c, 0x69, 0x66, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x12,
    0x40, 0x0a, 0x0c, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e,
    0x54, 0x79, 0x70, 0x65, 0x52, 0x0b, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x54, 0x79, 0x70,
    0x65, 0x12, 0x47, 0x0a, 0x20, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x6e, 0x63,
    0x65, 0x6e, 0x73, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x5f, 0x70, 0x72, 0x6f, 0x62, 0x61, 0x62,
    0x69, 0x6c, 0x69, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x1d, 0x70, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x54, 0x79, 0x70, 0x65, 0x50,
    0x72, 0x6f, 0x62, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x12, 0x56, 0x0a, 0x28, 0x73, 0x74,
    0x61, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x62, 0x65, 0x74, 0x77,
    0x65, 0x65, 0x6e, 0x5f, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x73,
    0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x52, 0x24, 0x73, 0x74,
    0x61, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x54, 0x69, 0x6d, 0x65, 0x42, 0x65, 0x74, 0x77, 0x65, 0x65,
    0x6e, 0x45, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x73, 0x53, 0x65, 0x63, 0x6f, 0x6e,
    0x64, 0x73, 0x12, 0x50, 0x0a, 0x25, 0x6d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x69, 0x6d,
    0x65, 0x5f, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x5f, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x65, 0x72, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x21, 0x6d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x54, 0x69, 0x6d, 0x65, 0x42, 0x65, 0x74,
    0x77, 0x65, 0x65, 0x6e, 0x45, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x53, 0x65, 0x63,
    0x6f, 0x6e, 0x64, 0x73, 0x12, 0x5f, 0x0a, 0x2d, 0x64, 0x69, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65,
    0x5f, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x5f, 0x66, 0x6f, 0x72, 0x5f, 0x73, 0x68,
    0x6f, 0x72, 0x74, 0x65, 0x72, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x5f, 0x6d,
    0x65, 0x74, 0x65, 0x72, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52, 0x28, 0x64, 0x69, 0x73,
    0x74, 0x61, 0x6e, 0x63, 0x65, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x46, 0x6f, 0x72,
    0x53, 0x68, 0x6f, 0x72, 0x74, 0x65, 0x72, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x4d,
    0x65, 0x74, 0x65, 0x72, 0x73, 0x12, 0x3f, 0x0a, 0x1c, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e,
    0x5f, 0x61, 0x74, 0x74, 0x72, 0x61, 0x63, 0x74, 0x65, 0x64, 0x5f, 0x6c, 0x65, 0x6e, 0x67, 0x74,
    0x68, 0x5f, 0x73, 0x65, 0x63, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x52, 0x19, 0x70, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x41, 0x74, 0x74, 0x72, 0x61, 0x63, 0x74, 0x65, 0x64, 0x4c, 0x65, 0x6e,
    0x67, 0x74, 0x68, 0x53, 0x65, 0x63, 0x22, 0x9a, 0x01, 0x0a, 0x1a, 0x49, 0x6e, 0x76, 0x65, 0x6e,
    0x74, 0x6f, 0x72, 0x79, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x41, 0x74, 0x74, 0x72, 0x69,
    0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x2d, 0x0a, 0x12, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x11, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x53, 0x74, 0x6f,
    0x72, 0x61, 0x67, 0x65, 0x12, 0x4d, 0x0a, 0x0c, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x5f,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2a, 0x2e, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72,
    0x79, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x55, 0x70, 0x67, 0x72, 0x61,
    0x64, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0b, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x54,
    0x79, 0x70, 0x65, 0x22, 0xd2, 0x01, 0x0a, 0x12, 0x50, 0x6f, 0x6b, 0x65, 0x62, 0x61, 0x6c, 0x6c,
    0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x3d, 0x0a, 0x0b, 0x69, 0x74,
    0x65, 0x6d, 0x5f, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x1c, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75,
    0x6d, 0x73, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x45, 0x66, 0x66, 0x65, 0x63, 0x74, 0x52, 0x0a, 0x69,
    0x74, 0x65, 0x6d, 0x45, 0x66, 0x66, 0x65, 0x63, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x61, 0x70,
    0x74, 0x75, 0x72, 0x65, 0x5f, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02,
    0x52, 0x0c, 0x63, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x12, 0x30,
    0x0a, 0x14, 0x63, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x5f, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x5f,
    0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x12, 0x63, 0x61,
    0x70, 0x74, 0x75, 0x72, 0x65, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x45, 0x66, 0x66, 0x65, 0x63, 0x74,
    0x12, 0x26, 0x0a, 0x0f, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x5f,
    0x6d, 0x6f, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0d, 0x69, 0x74, 0x65, 0x6d, 0x45,
    0x66, 0x66, 0x65, 0x63, 0x74, 0x4d, 0x6f, 0x64, 0x22, 0x6c, 0x0a, 0x19, 0x45, 0x78, 0x70, 0x65,
    0x72, 0x69, 0x65, 0x6e, 0x63, 0x65, 0x42, 0x6f, 0x6f, 0x73, 0x74, 0x41, 0x74, 0x74, 0x72, 0x69,
    0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x23, 0x0a, 0x0d, 0x78, 0x70, 0x5f, 0x6d, 0x75, 0x6c, 0x74,
    0x69, 0x70, 0x6c, 0x69, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0c, 0x78, 0x70,
    0x4d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x69, 0x65, 0x72, 0x12, 0x2a, 0x0a, 0x11, 0x62, 0x6f,
    0x6f, 0x73, 0x74, 0x5f, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x73, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0f, 0x62, 0x6f, 0x6f, 0x73, 0x74, 0x44, 0x75, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x73, 0x4a, 0x97, 0x11, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x34,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x01, 0x08, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x00, 0x12, 0x03, 0x03, 0x07, 0x0d,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x0e, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x0a,
    0x01, 0x12, 0x03, 0x04, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x0e,
    0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x06, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x07, 0x08, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x07, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x07,
    0x11, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x2d, 0x38,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x3b, 0x3c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x08, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x08, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x08, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x08, 0x17, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x08, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x09, 0x08, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x09, 0x08, 0x08, 0x2f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x09, 0x08, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x09, 0x0e, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x09, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x0b, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08,
    0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x08, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0c, 0x08, 0x0b, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0c, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x0c, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0e,
    0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x18, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0f, 0x08, 0x0e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0f, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x0f, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x0f, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x11, 0x00, 0x14,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x11, 0x08, 0x18, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x12, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x12, 0x08, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x12, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x12, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x12, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x13, 0x08, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x04, 0x13, 0x08, 0x12, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x13, 0x08, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x13, 0x0e, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x13, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x15, 0x00, 0x18, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x15, 0x08,
    0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x16, 0x08, 0x2c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x16, 0x08, 0x15, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x0e, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x16, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x17, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x04, 0x17,
    0x08, 0x16, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x17, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x0e, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x17, 0x2e, 0x2f, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x05, 0x12, 0x04, 0x19, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01,
    0x12, 0x03, 0x19, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x1a,
    0x02, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x1a, 0x02, 0x19,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1a, 0x02, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x29, 0x37, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1a, 0x3a, 0x3b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x02, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x1b, 0x02, 0x1a, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x1b, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x1b, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1b, 0x0f,
    0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x1c, 0x02, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x04, 0x1c, 0x02, 0x1b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1c, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x1c, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x1e,
    0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x19, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x1f, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04, 0x1f, 0x08, 0x1e, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1f, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x1f, 0x0e, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x1f, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x20,
    0x08, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x20, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x20, 0x11, 0x2e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x20, 0x2f, 0x3b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x20, 0x3e, 0x3f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x02, 0x12, 0x03, 0x21, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x21, 0x08, 0x20, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x21, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x21,
    0x0e, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x21, 0x31, 0x32,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x22, 0x08, 0x3b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x04, 0x22, 0x08, 0x21, 0x33, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x03, 0x22, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x22, 0x0e, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x22, 0x39, 0x3a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03,
    0x23, 0x08, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x04, 0x23, 0x08,
    0x22, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x03, 0x23, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x23, 0x0e, 0x33, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03, 0x23, 0x36, 0x37, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x05, 0x12, 0x03, 0x24, 0x08, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x05, 0x04, 0x12, 0x04, 0x24, 0x08, 0x23, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x24, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x24, 0x0e, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03, 0x24,
    0x3e, 0x3f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x06, 0x12, 0x03, 0x25, 0x08, 0x2f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x04, 0x12, 0x04, 0x25, 0x08, 0x24, 0x40, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x05, 0x12, 0x03, 0x25, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x06, 0x01, 0x12, 0x03, 0x25, 0x0e, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x06, 0x03, 0x12, 0x03, 0x25, 0x2d, 0x2e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04,
    0x27, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x27, 0x08, 0x22,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x28, 0x08, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x28, 0x08, 0x27, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x28, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x28, 0x0e, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x28, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03,
    0x29, 0x08, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04, 0x29, 0x08,
    0x28, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x29, 0x08, 0x32,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x33, 0x3f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x29, 0x42, 0x43, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x08, 0x12, 0x04, 0x2b, 0x00, 0x30, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12,
    0x03, 0x2b, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x2c, 0x08,
    0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0x2c, 0x08, 0x2b, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2c, 0x08, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x25, 0x30, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x33, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x01, 0x12, 0x03, 0x2d, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x2d, 0x08, 0x2c, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x2d, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2d,
    0x0e, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2d, 0x1e, 0x1f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x2e, 0x08, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x04, 0x2e, 0x08, 0x2d, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2e, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x0e, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x2e, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x03,
    0x2f, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x04, 0x2f, 0x08,
    0x2e, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x05, 0x12, 0x03, 0x2f, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2f, 0x0e, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2f, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x09, 0x12, 0x04, 0x31, 0x00, 0x34, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12,
    0x03, 0x31, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x32, 0x08,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0x32, 0x08, 0x31, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x32, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x32, 0x0e, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x32, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09,
    0x02, 0x01, 0x12, 0x03, 0x33, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x33, 0x08, 0x32, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x33, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x33,
    0x0e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x33, 0x22, 0x23,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
