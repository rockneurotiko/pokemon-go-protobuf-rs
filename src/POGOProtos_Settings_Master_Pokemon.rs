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
pub struct CameraAttributes {
    // message fields
    disk_radius_m: ::std::option::Option<f32>,
    cylinder_radius_m: ::std::option::Option<f32>,
    cylinder_height_m: ::std::option::Option<f32>,
    cylinder_ground_m: ::std::option::Option<f32>,
    shoulder_mode_scale: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CameraAttributes {}

impl CameraAttributes {
    pub fn new() -> CameraAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CameraAttributes {
        static mut instance: ::protobuf::lazy::Lazy<CameraAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CameraAttributes,
        };
        unsafe {
            instance.get(|| {
                CameraAttributes {
                    disk_radius_m: ::std::option::Option::None,
                    cylinder_radius_m: ::std::option::Option::None,
                    cylinder_height_m: ::std::option::Option::None,
                    cylinder_ground_m: ::std::option::Option::None,
                    shoulder_mode_scale: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float disk_radius_m = 1;

    pub fn clear_disk_radius_m(&mut self) {
        self.disk_radius_m = ::std::option::Option::None;
    }

    pub fn has_disk_radius_m(&self) -> bool {
        self.disk_radius_m.is_some()
    }

    // Param is passed by value, moved
    pub fn set_disk_radius_m(&mut self, v: f32) {
        self.disk_radius_m = ::std::option::Option::Some(v);
    }

    pub fn get_disk_radius_m(&self) -> f32 {
        self.disk_radius_m.unwrap_or(0.)
    }

    // optional float cylinder_radius_m = 2;

    pub fn clear_cylinder_radius_m(&mut self) {
        self.cylinder_radius_m = ::std::option::Option::None;
    }

    pub fn has_cylinder_radius_m(&self) -> bool {
        self.cylinder_radius_m.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cylinder_radius_m(&mut self, v: f32) {
        self.cylinder_radius_m = ::std::option::Option::Some(v);
    }

    pub fn get_cylinder_radius_m(&self) -> f32 {
        self.cylinder_radius_m.unwrap_or(0.)
    }

    // optional float cylinder_height_m = 3;

    pub fn clear_cylinder_height_m(&mut self) {
        self.cylinder_height_m = ::std::option::Option::None;
    }

    pub fn has_cylinder_height_m(&self) -> bool {
        self.cylinder_height_m.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cylinder_height_m(&mut self, v: f32) {
        self.cylinder_height_m = ::std::option::Option::Some(v);
    }

    pub fn get_cylinder_height_m(&self) -> f32 {
        self.cylinder_height_m.unwrap_or(0.)
    }

    // optional float cylinder_ground_m = 4;

    pub fn clear_cylinder_ground_m(&mut self) {
        self.cylinder_ground_m = ::std::option::Option::None;
    }

    pub fn has_cylinder_ground_m(&self) -> bool {
        self.cylinder_ground_m.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cylinder_ground_m(&mut self, v: f32) {
        self.cylinder_ground_m = ::std::option::Option::Some(v);
    }

    pub fn get_cylinder_ground_m(&self) -> f32 {
        self.cylinder_ground_m.unwrap_or(0.)
    }

    // optional float shoulder_mode_scale = 5;

    pub fn clear_shoulder_mode_scale(&mut self) {
        self.shoulder_mode_scale = ::std::option::Option::None;
    }

    pub fn has_shoulder_mode_scale(&self) -> bool {
        self.shoulder_mode_scale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shoulder_mode_scale(&mut self, v: f32) {
        self.shoulder_mode_scale = ::std::option::Option::Some(v);
    }

    pub fn get_shoulder_mode_scale(&self) -> f32 {
        self.shoulder_mode_scale.unwrap_or(0.)
    }
}

impl ::protobuf::Message for CameraAttributes {
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
                    self.disk_radius_m = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.cylinder_radius_m = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.cylinder_height_m = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.cylinder_ground_m = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.shoulder_mode_scale = ::std::option::Option::Some(tmp);
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
        if self.disk_radius_m.is_some() {
            my_size += 5;
        };
        if self.cylinder_radius_m.is_some() {
            my_size += 5;
        };
        if self.cylinder_height_m.is_some() {
            my_size += 5;
        };
        if self.cylinder_ground_m.is_some() {
            my_size += 5;
        };
        if self.shoulder_mode_scale.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.disk_radius_m {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.cylinder_radius_m {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.cylinder_height_m {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.cylinder_ground_m {
            try!(os.write_float(4, v));
        };
        if let Some(v) = self.shoulder_mode_scale {
            try!(os.write_float(5, v));
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
        ::std::any::TypeId::of::<CameraAttributes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CameraAttributes {
    fn new() -> CameraAttributes {
        CameraAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<CameraAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "disk_radius_m",
                    CameraAttributes::has_disk_radius_m,
                    CameraAttributes::get_disk_radius_m,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "cylinder_radius_m",
                    CameraAttributes::has_cylinder_radius_m,
                    CameraAttributes::get_cylinder_radius_m,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "cylinder_height_m",
                    CameraAttributes::has_cylinder_height_m,
                    CameraAttributes::get_cylinder_height_m,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "cylinder_ground_m",
                    CameraAttributes::has_cylinder_ground_m,
                    CameraAttributes::get_cylinder_ground_m,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "shoulder_mode_scale",
                    CameraAttributes::has_shoulder_mode_scale,
                    CameraAttributes::get_shoulder_mode_scale,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CameraAttributes>(
                    "CameraAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CameraAttributes {
    fn clear(&mut self) {
        self.clear_disk_radius_m();
        self.clear_cylinder_radius_m();
        self.clear_cylinder_height_m();
        self.clear_cylinder_ground_m();
        self.clear_shoulder_mode_scale();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CameraAttributes {
    fn eq(&self, other: &CameraAttributes) -> bool {
        self.disk_radius_m == other.disk_radius_m &&
        self.cylinder_radius_m == other.cylinder_radius_m &&
        self.cylinder_height_m == other.cylinder_height_m &&
        self.cylinder_ground_m == other.cylinder_ground_m &&
        self.shoulder_mode_scale == other.shoulder_mode_scale &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CameraAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StatsAttributes {
    // message fields
    base_stamina: ::std::option::Option<i32>,
    base_attack: ::std::option::Option<i32>,
    base_defense: ::std::option::Option<i32>,
    dodge_energy_delta: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatsAttributes {}

impl StatsAttributes {
    pub fn new() -> StatsAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatsAttributes {
        static mut instance: ::protobuf::lazy::Lazy<StatsAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatsAttributes,
        };
        unsafe {
            instance.get(|| {
                StatsAttributes {
                    base_stamina: ::std::option::Option::None,
                    base_attack: ::std::option::Option::None,
                    base_defense: ::std::option::Option::None,
                    dodge_energy_delta: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 base_stamina = 1;

    pub fn clear_base_stamina(&mut self) {
        self.base_stamina = ::std::option::Option::None;
    }

    pub fn has_base_stamina(&self) -> bool {
        self.base_stamina.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_stamina(&mut self, v: i32) {
        self.base_stamina = ::std::option::Option::Some(v);
    }

    pub fn get_base_stamina(&self) -> i32 {
        self.base_stamina.unwrap_or(0)
    }

    // optional int32 base_attack = 2;

    pub fn clear_base_attack(&mut self) {
        self.base_attack = ::std::option::Option::None;
    }

    pub fn has_base_attack(&self) -> bool {
        self.base_attack.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_attack(&mut self, v: i32) {
        self.base_attack = ::std::option::Option::Some(v);
    }

    pub fn get_base_attack(&self) -> i32 {
        self.base_attack.unwrap_or(0)
    }

    // optional int32 base_defense = 3;

    pub fn clear_base_defense(&mut self) {
        self.base_defense = ::std::option::Option::None;
    }

    pub fn has_base_defense(&self) -> bool {
        self.base_defense.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_defense(&mut self, v: i32) {
        self.base_defense = ::std::option::Option::Some(v);
    }

    pub fn get_base_defense(&self) -> i32 {
        self.base_defense.unwrap_or(0)
    }

    // optional int32 dodge_energy_delta = 8;

    pub fn clear_dodge_energy_delta(&mut self) {
        self.dodge_energy_delta = ::std::option::Option::None;
    }

    pub fn has_dodge_energy_delta(&self) -> bool {
        self.dodge_energy_delta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dodge_energy_delta(&mut self, v: i32) {
        self.dodge_energy_delta = ::std::option::Option::Some(v);
    }

    pub fn get_dodge_energy_delta(&self) -> i32 {
        self.dodge_energy_delta.unwrap_or(0)
    }
}

impl ::protobuf::Message for StatsAttributes {
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
                    self.base_stamina = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.base_attack = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.base_defense = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.dodge_energy_delta = ::std::option::Option::Some(tmp);
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
        for value in &self.base_stamina {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.base_attack {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.base_defense {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.dodge_energy_delta {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.base_stamina {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.base_attack {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.base_defense {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.dodge_energy_delta {
            try!(os.write_int32(8, v));
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
        ::std::any::TypeId::of::<StatsAttributes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StatsAttributes {
    fn new() -> StatsAttributes {
        StatsAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatsAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "base_stamina",
                    StatsAttributes::has_base_stamina,
                    StatsAttributes::get_base_stamina,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "base_attack",
                    StatsAttributes::has_base_attack,
                    StatsAttributes::get_base_attack,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "base_defense",
                    StatsAttributes::has_base_defense,
                    StatsAttributes::get_base_defense,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "dodge_energy_delta",
                    StatsAttributes::has_dodge_energy_delta,
                    StatsAttributes::get_dodge_energy_delta,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatsAttributes>(
                    "StatsAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatsAttributes {
    fn clear(&mut self) {
        self.clear_base_stamina();
        self.clear_base_attack();
        self.clear_base_defense();
        self.clear_dodge_energy_delta();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StatsAttributes {
    fn eq(&self, other: &StatsAttributes) -> bool {
        self.base_stamina == other.base_stamina &&
        self.base_attack == other.base_attack &&
        self.base_defense == other.base_defense &&
        self.dodge_energy_delta == other.dodge_energy_delta &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StatsAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EncounterAttributes {
    // message fields
    base_capture_rate: ::std::option::Option<f32>,
    base_flee_rate: ::std::option::Option<f32>,
    collision_radius_m: ::std::option::Option<f32>,
    collision_height_m: ::std::option::Option<f32>,
    collision_head_radius_m: ::std::option::Option<f32>,
    movement_type: ::std::option::Option<super::POGOProtos_Enums::PokemonMovementType>,
    movement_timer_s: ::std::option::Option<f32>,
    jump_time_s: ::std::option::Option<f32>,
    attack_timer_s: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EncounterAttributes {}

impl EncounterAttributes {
    pub fn new() -> EncounterAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EncounterAttributes {
        static mut instance: ::protobuf::lazy::Lazy<EncounterAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EncounterAttributes,
        };
        unsafe {
            instance.get(|| {
                EncounterAttributes {
                    base_capture_rate: ::std::option::Option::None,
                    base_flee_rate: ::std::option::Option::None,
                    collision_radius_m: ::std::option::Option::None,
                    collision_height_m: ::std::option::Option::None,
                    collision_head_radius_m: ::std::option::Option::None,
                    movement_type: ::std::option::Option::None,
                    movement_timer_s: ::std::option::Option::None,
                    jump_time_s: ::std::option::Option::None,
                    attack_timer_s: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float base_capture_rate = 1;

    pub fn clear_base_capture_rate(&mut self) {
        self.base_capture_rate = ::std::option::Option::None;
    }

    pub fn has_base_capture_rate(&self) -> bool {
        self.base_capture_rate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_capture_rate(&mut self, v: f32) {
        self.base_capture_rate = ::std::option::Option::Some(v);
    }

    pub fn get_base_capture_rate(&self) -> f32 {
        self.base_capture_rate.unwrap_or(0.)
    }

    // optional float base_flee_rate = 2;

    pub fn clear_base_flee_rate(&mut self) {
        self.base_flee_rate = ::std::option::Option::None;
    }

    pub fn has_base_flee_rate(&self) -> bool {
        self.base_flee_rate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_flee_rate(&mut self, v: f32) {
        self.base_flee_rate = ::std::option::Option::Some(v);
    }

    pub fn get_base_flee_rate(&self) -> f32 {
        self.base_flee_rate.unwrap_or(0.)
    }

    // optional float collision_radius_m = 3;

    pub fn clear_collision_radius_m(&mut self) {
        self.collision_radius_m = ::std::option::Option::None;
    }

    pub fn has_collision_radius_m(&self) -> bool {
        self.collision_radius_m.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collision_radius_m(&mut self, v: f32) {
        self.collision_radius_m = ::std::option::Option::Some(v);
    }

    pub fn get_collision_radius_m(&self) -> f32 {
        self.collision_radius_m.unwrap_or(0.)
    }

    // optional float collision_height_m = 4;

    pub fn clear_collision_height_m(&mut self) {
        self.collision_height_m = ::std::option::Option::None;
    }

    pub fn has_collision_height_m(&self) -> bool {
        self.collision_height_m.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collision_height_m(&mut self, v: f32) {
        self.collision_height_m = ::std::option::Option::Some(v);
    }

    pub fn get_collision_height_m(&self) -> f32 {
        self.collision_height_m.unwrap_or(0.)
    }

    // optional float collision_head_radius_m = 5;

    pub fn clear_collision_head_radius_m(&mut self) {
        self.collision_head_radius_m = ::std::option::Option::None;
    }

    pub fn has_collision_head_radius_m(&self) -> bool {
        self.collision_head_radius_m.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collision_head_radius_m(&mut self, v: f32) {
        self.collision_head_radius_m = ::std::option::Option::Some(v);
    }

    pub fn get_collision_head_radius_m(&self) -> f32 {
        self.collision_head_radius_m.unwrap_or(0.)
    }

    // optional .POGOProtos.Enums.PokemonMovementType movement_type = 6;

    pub fn clear_movement_type(&mut self) {
        self.movement_type = ::std::option::Option::None;
    }

    pub fn has_movement_type(&self) -> bool {
        self.movement_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_movement_type(&mut self, v: super::POGOProtos_Enums::PokemonMovementType) {
        self.movement_type = ::std::option::Option::Some(v);
    }

    pub fn get_movement_type(&self) -> super::POGOProtos_Enums::PokemonMovementType {
        self.movement_type.unwrap_or(super::POGOProtos_Enums::PokemonMovementType::MOVEMENT_STATIC)
    }

    // optional float movement_timer_s = 7;

    pub fn clear_movement_timer_s(&mut self) {
        self.movement_timer_s = ::std::option::Option::None;
    }

    pub fn has_movement_timer_s(&self) -> bool {
        self.movement_timer_s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_movement_timer_s(&mut self, v: f32) {
        self.movement_timer_s = ::std::option::Option::Some(v);
    }

    pub fn get_movement_timer_s(&self) -> f32 {
        self.movement_timer_s.unwrap_or(0.)
    }

    // optional float jump_time_s = 8;

    pub fn clear_jump_time_s(&mut self) {
        self.jump_time_s = ::std::option::Option::None;
    }

    pub fn has_jump_time_s(&self) -> bool {
        self.jump_time_s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jump_time_s(&mut self, v: f32) {
        self.jump_time_s = ::std::option::Option::Some(v);
    }

    pub fn get_jump_time_s(&self) -> f32 {
        self.jump_time_s.unwrap_or(0.)
    }

    // optional float attack_timer_s = 9;

    pub fn clear_attack_timer_s(&mut self) {
        self.attack_timer_s = ::std::option::Option::None;
    }

    pub fn has_attack_timer_s(&self) -> bool {
        self.attack_timer_s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attack_timer_s(&mut self, v: f32) {
        self.attack_timer_s = ::std::option::Option::Some(v);
    }

    pub fn get_attack_timer_s(&self) -> f32 {
        self.attack_timer_s.unwrap_or(0.)
    }
}

impl ::protobuf::Message for EncounterAttributes {
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
                    self.base_capture_rate = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.base_flee_rate = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.collision_radius_m = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.collision_height_m = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.collision_head_radius_m = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.movement_type = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.movement_timer_s = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.jump_time_s = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.attack_timer_s = ::std::option::Option::Some(tmp);
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
        if self.base_capture_rate.is_some() {
            my_size += 5;
        };
        if self.base_flee_rate.is_some() {
            my_size += 5;
        };
        if self.collision_radius_m.is_some() {
            my_size += 5;
        };
        if self.collision_height_m.is_some() {
            my_size += 5;
        };
        if self.collision_head_radius_m.is_some() {
            my_size += 5;
        };
        for value in &self.movement_type {
            my_size += ::protobuf::rt::enum_size(6, *value);
        };
        if self.movement_timer_s.is_some() {
            my_size += 5;
        };
        if self.jump_time_s.is_some() {
            my_size += 5;
        };
        if self.attack_timer_s.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.base_capture_rate {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.base_flee_rate {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.collision_radius_m {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.collision_height_m {
            try!(os.write_float(4, v));
        };
        if let Some(v) = self.collision_head_radius_m {
            try!(os.write_float(5, v));
        };
        if let Some(v) = self.movement_type {
            try!(os.write_enum(6, v.value()));
        };
        if let Some(v) = self.movement_timer_s {
            try!(os.write_float(7, v));
        };
        if let Some(v) = self.jump_time_s {
            try!(os.write_float(8, v));
        };
        if let Some(v) = self.attack_timer_s {
            try!(os.write_float(9, v));
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
        ::std::any::TypeId::of::<EncounterAttributes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EncounterAttributes {
    fn new() -> EncounterAttributes {
        EncounterAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<EncounterAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "base_capture_rate",
                    EncounterAttributes::has_base_capture_rate,
                    EncounterAttributes::get_base_capture_rate,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "base_flee_rate",
                    EncounterAttributes::has_base_flee_rate,
                    EncounterAttributes::get_base_flee_rate,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "collision_radius_m",
                    EncounterAttributes::has_collision_radius_m,
                    EncounterAttributes::get_collision_radius_m,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "collision_height_m",
                    EncounterAttributes::has_collision_height_m,
                    EncounterAttributes::get_collision_height_m,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "collision_head_radius_m",
                    EncounterAttributes::has_collision_head_radius_m,
                    EncounterAttributes::get_collision_head_radius_m,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "movement_type",
                    EncounterAttributes::has_movement_type,
                    EncounterAttributes::get_movement_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "movement_timer_s",
                    EncounterAttributes::has_movement_timer_s,
                    EncounterAttributes::get_movement_timer_s,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "jump_time_s",
                    EncounterAttributes::has_jump_time_s,
                    EncounterAttributes::get_jump_time_s,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "attack_timer_s",
                    EncounterAttributes::has_attack_timer_s,
                    EncounterAttributes::get_attack_timer_s,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EncounterAttributes>(
                    "EncounterAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EncounterAttributes {
    fn clear(&mut self) {
        self.clear_base_capture_rate();
        self.clear_base_flee_rate();
        self.clear_collision_radius_m();
        self.clear_collision_height_m();
        self.clear_collision_head_radius_m();
        self.clear_movement_type();
        self.clear_movement_timer_s();
        self.clear_jump_time_s();
        self.clear_attack_timer_s();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EncounterAttributes {
    fn eq(&self, other: &EncounterAttributes) -> bool {
        self.base_capture_rate == other.base_capture_rate &&
        self.base_flee_rate == other.base_flee_rate &&
        self.collision_radius_m == other.collision_radius_m &&
        self.collision_height_m == other.collision_height_m &&
        self.collision_head_radius_m == other.collision_head_radius_m &&
        self.movement_type == other.movement_type &&
        self.movement_timer_s == other.movement_timer_s &&
        self.jump_time_s == other.jump_time_s &&
        self.attack_timer_s == other.attack_timer_s &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EncounterAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x28, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74,
    0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x50, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x22, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e,
    0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x1a, 0x16,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x00, 0x22, 0xea, 0x01, 0x0a, 0x10, 0x43, 0x61, 0x6d,
    0x65, 0x72, 0x61, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x22, 0x0a,
    0x0d, 0x64, 0x69, 0x73, 0x6b, 0x5f, 0x72, 0x61, 0x64, 0x69, 0x75, 0x73, 0x5f, 0x6d, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x02, 0x52, 0x0b, 0x64, 0x69, 0x73, 0x6b, 0x52, 0x61, 0x64, 0x69, 0x75, 0x73,
    0x4d, 0x12, 0x2a, 0x0a, 0x11, 0x63, 0x79, 0x6c, 0x69, 0x6e, 0x64, 0x65, 0x72, 0x5f, 0x72, 0x61,
    0x64, 0x69, 0x75, 0x73, 0x5f, 0x6d, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0f, 0x63, 0x79,
    0x6c, 0x69, 0x6e, 0x64, 0x65, 0x72, 0x52, 0x61, 0x64, 0x69, 0x75, 0x73, 0x4d, 0x12, 0x2a, 0x0a,
    0x11, 0x63, 0x79, 0x6c, 0x69, 0x6e, 0x64, 0x65, 0x72, 0x5f, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74,
    0x5f, 0x6d, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0f, 0x63, 0x79, 0x6c, 0x69, 0x6e, 0x64,
    0x65, 0x72, 0x48, 0x65, 0x69, 0x67, 0x68, 0x74, 0x4d, 0x12, 0x2a, 0x0a, 0x11, 0x63, 0x79, 0x6c,
    0x69, 0x6e, 0x64, 0x65, 0x72, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x6d, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x02, 0x52, 0x0f, 0x63, 0x79, 0x6c, 0x69, 0x6e, 0x64, 0x65, 0x72, 0x47, 0x72,
    0x6f, 0x75, 0x6e, 0x64, 0x4d, 0x12, 0x2e, 0x0a, 0x13, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x65,
    0x72, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x5f, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x02, 0x52, 0x11, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x65, 0x72, 0x4d, 0x6f, 0x64, 0x65,
    0x53, 0x63, 0x61, 0x6c, 0x65, 0x22, 0xa6, 0x01, 0x0a, 0x0f, 0x53, 0x74, 0x61, 0x74, 0x73, 0x41,
    0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x21, 0x0a, 0x0c, 0x62, 0x61, 0x73,
    0x65, 0x5f, 0x73, 0x74, 0x61, 0x6d, 0x69, 0x6e, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x0b, 0x62, 0x61, 0x73, 0x65, 0x53, 0x74, 0x61, 0x6d, 0x69, 0x6e, 0x61, 0x12, 0x1f, 0x0a, 0x0b,
    0x62, 0x61, 0x73, 0x65, 0x5f, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x0a, 0x62, 0x61, 0x73, 0x65, 0x41, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x12, 0x21, 0x0a,
    0x0c, 0x62, 0x61, 0x73, 0x65, 0x5f, 0x64, 0x65, 0x66, 0x65, 0x6e, 0x73, 0x65, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x0b, 0x62, 0x61, 0x73, 0x65, 0x44, 0x65, 0x66, 0x65, 0x6e, 0x73, 0x65,
    0x12, 0x2c, 0x0a, 0x12, 0x64, 0x6f, 0x64, 0x67, 0x65, 0x5f, 0x65, 0x6e, 0x65, 0x72, 0x67, 0x79,
    0x5f, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x18, 0x08, 0x20, 0x01, 0x28, 0x05, 0x52, 0x10, 0x64, 0x6f,
    0x64, 0x67, 0x65, 0x45, 0x6e, 0x65, 0x72, 0x67, 0x79, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x22, 0xb6,
    0x03, 0x0a, 0x13, 0x45, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x41, 0x74, 0x74, 0x72,
    0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x2a, 0x0a, 0x11, 0x62, 0x61, 0x73, 0x65, 0x5f, 0x63,
    0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x5f, 0x72, 0x61, 0x74, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x02, 0x52, 0x0f, 0x62, 0x61, 0x73, 0x65, 0x43, 0x61, 0x70, 0x74, 0x75, 0x72, 0x65, 0x52, 0x61,
    0x74, 0x65, 0x12, 0x24, 0x0a, 0x0e, 0x62, 0x61, 0x73, 0x65, 0x5f, 0x66, 0x6c, 0x65, 0x65, 0x5f,
    0x72, 0x61, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0c, 0x62, 0x61, 0x73, 0x65,
    0x46, 0x6c, 0x65, 0x65, 0x52, 0x61, 0x74, 0x65, 0x12, 0x2c, 0x0a, 0x12, 0x63, 0x6f, 0x6c, 0x6c,
    0x69, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x72, 0x61, 0x64, 0x69, 0x75, 0x73, 0x5f, 0x6d, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x02, 0x52, 0x10, 0x63, 0x6f, 0x6c, 0x6c, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x52,
    0x61, 0x64, 0x69, 0x75, 0x73, 0x4d, 0x12, 0x2c, 0x0a, 0x12, 0x63, 0x6f, 0x6c, 0x6c, 0x69, 0x73,
    0x69, 0x6f, 0x6e, 0x5f, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x6d, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x02, 0x52, 0x10, 0x63, 0x6f, 0x6c, 0x6c, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x48, 0x65, 0x69,
    0x67, 0x68, 0x74, 0x4d, 0x12, 0x35, 0x0a, 0x17, 0x63, 0x6f, 0x6c, 0x6c, 0x69, 0x73, 0x69, 0x6f,
    0x6e, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x5f, 0x72, 0x61, 0x64, 0x69, 0x75, 0x73, 0x5f, 0x6d, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x02, 0x52, 0x14, 0x63, 0x6f, 0x6c, 0x6c, 0x69, 0x73, 0x69, 0x6f, 0x6e,
    0x48, 0x65, 0x61, 0x64, 0x52, 0x61, 0x64, 0x69, 0x75, 0x73, 0x4d, 0x12, 0x4a, 0x0a, 0x0d, 0x6d,
    0x6f, 0x76, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x25, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x4d, 0x6f, 0x76,
    0x65, 0x6d, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0c, 0x6d, 0x6f, 0x76, 0x65, 0x6d,
    0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x28, 0x0a, 0x10, 0x6d, 0x6f, 0x76, 0x65, 0x6d,
    0x65, 0x6e, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x72, 0x5f, 0x73, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x02, 0x52, 0x0e, 0x6d, 0x6f, 0x76, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x72,
    0x53, 0x12, 0x1e, 0x0a, 0x0b, 0x6a, 0x75, 0x6d, 0x70, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x73,
    0x18, 0x08, 0x20, 0x01, 0x28, 0x02, 0x52, 0x09, 0x6a, 0x75, 0x6d, 0x70, 0x54, 0x69, 0x6d, 0x65,
    0x53, 0x12, 0x24, 0x0a, 0x0e, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x5f, 0x74, 0x69, 0x6d, 0x65,
    0x72, 0x5f, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0c, 0x61, 0x74, 0x74, 0x61, 0x63,
    0x6b, 0x54, 0x69, 0x6d, 0x65, 0x72, 0x53, 0x4a, 0xe6, 0x0a, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x1c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x01, 0x08, 0x2a, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x00, 0x12, 0x03, 0x03, 0x07,
    0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x0e, 0x26, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x05, 0x00, 0x0b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x05, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x08,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x06, 0x08, 0x05, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x06, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x0e, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x06, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x07, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x07, 0x08, 0x06, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x07, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07,
    0x0e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x07, 0x22, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x08, 0x08, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x08, 0x08, 0x07, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x08, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x08, 0x0e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x08, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x09, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x09, 0x08,
    0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x09, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x09, 0x0e, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x09, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0a, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x0a, 0x08, 0x09, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x0a, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x0a, 0x0e, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0a,
    0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0c, 0x00, 0x11, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x0d, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x0d, 0x08, 0x0c, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x0d, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d,
    0x0e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x1d, 0x1e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0e, 0x08, 0x0d, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0e, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x0e, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03,
    0x0f, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0x0f, 0x08,
    0x0e, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x0e, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x10, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x04, 0x12, 0x04, 0x10, 0x08, 0x0f, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x10, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x10, 0x0e, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x10,
    0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x12, 0x00, 0x1c, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x12, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x13, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x13, 0x08, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x13, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13,
    0x0e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x13, 0x22, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x14, 0x08, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x14, 0x08, 0x13, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x14, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x0e, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x14, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x15, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x04, 0x15, 0x08,
    0x14, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x15, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x15, 0x0e, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x15, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x16, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x04, 0x12, 0x04, 0x16, 0x08, 0x15, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x16, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x16, 0x0e, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x16,
    0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x17, 0x08, 0x2a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x04, 0x17, 0x08, 0x16, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x17, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x17, 0x0e, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x17, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x18, 0x08, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04, 0x12, 0x04,
    0x18, 0x08, 0x17, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x06, 0x12, 0x03, 0x18,
    0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x18, 0x2e, 0x3b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x18, 0x3e, 0x3f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x19, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x06, 0x04, 0x12, 0x04, 0x19, 0x08, 0x18, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x06, 0x05, 0x12, 0x03, 0x19, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x19, 0x0e, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x03, 0x12,
    0x03, 0x19, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x07, 0x12, 0x03, 0x1a, 0x08,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x04, 0x12, 0x04, 0x1a, 0x08, 0x19, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x05, 0x12, 0x03, 0x1a, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x1a, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x1a, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x08, 0x12, 0x03, 0x1b, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x04,
    0x12, 0x04, 0x1b, 0x08, 0x1a, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x05, 0x12,
    0x03, 0x1b, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x01, 0x12, 0x03, 0x1b,
    0x0e, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x03, 0x12, 0x03, 0x1b, 0x1f, 0x20,
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
