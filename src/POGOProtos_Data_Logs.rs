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
pub struct FortSearchLogEntry {
    // message fields
    result: ::std::option::Option<FortSearchLogEntry_Result>,
    fort_id: ::protobuf::SingularField<::std::string::String>,
    items: ::protobuf::RepeatedField<super::POGOProtos_Inventory_Item::ItemData>,
    eggs: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FortSearchLogEntry {}

impl FortSearchLogEntry {
    pub fn new() -> FortSearchLogEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FortSearchLogEntry {
        static mut instance: ::protobuf::lazy::Lazy<FortSearchLogEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FortSearchLogEntry,
        };
        unsafe {
            instance.get(|| {
                FortSearchLogEntry {
                    result: ::std::option::Option::None,
                    fort_id: ::protobuf::SingularField::none(),
                    items: ::protobuf::RepeatedField::new(),
                    eggs: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Data.Logs.FortSearchLogEntry.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: FortSearchLogEntry_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> FortSearchLogEntry_Result {
        self.result.unwrap_or(FortSearchLogEntry_Result::UNSET)
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

    // repeated .POGOProtos.Inventory.Item.ItemData items = 3;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<super::POGOProtos_Inventory_Item::ItemData>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<super::POGOProtos_Inventory_Item::ItemData> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<super::POGOProtos_Inventory_Item::ItemData> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[super::POGOProtos_Inventory_Item::ItemData] {
        &self.items
    }

    // optional int32 eggs = 4;

    pub fn clear_eggs(&mut self) {
        self.eggs = ::std::option::Option::None;
    }

    pub fn has_eggs(&self) -> bool {
        self.eggs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eggs(&mut self, v: i32) {
        self.eggs = ::std::option::Option::Some(v);
    }

    pub fn get_eggs(&self) -> i32 {
        self.eggs.unwrap_or(0)
    }
}

impl ::protobuf::Message for FortSearchLogEntry {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fort_id));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.eggs = ::std::option::Option::Some(tmp);
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
        for value in &self.fort_id {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.eggs {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.fort_id.as_ref() {
            try!(os.write_string(2, &v));
        };
        for v in &self.items {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.eggs {
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
        ::std::any::TypeId::of::<FortSearchLogEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FortSearchLogEntry {
    fn new() -> FortSearchLogEntry {
        FortSearchLogEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<FortSearchLogEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    FortSearchLogEntry::has_result,
                    FortSearchLogEntry::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "fort_id",
                    FortSearchLogEntry::has_fort_id,
                    FortSearchLogEntry::get_fort_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "items",
                    FortSearchLogEntry::get_items,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "eggs",
                    FortSearchLogEntry::has_eggs,
                    FortSearchLogEntry::get_eggs,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FortSearchLogEntry>(
                    "FortSearchLogEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FortSearchLogEntry {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_fort_id();
        self.clear_items();
        self.clear_eggs();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FortSearchLogEntry {
    fn eq(&self, other: &FortSearchLogEntry) -> bool {
        self.result == other.result &&
        self.fort_id == other.fort_id &&
        self.items == other.items &&
        self.eggs == other.eggs &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FortSearchLogEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FortSearchLogEntry_Result {
    UNSET = 0,
    SUCCESS = 1,
}

impl ::protobuf::ProtobufEnum for FortSearchLogEntry_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FortSearchLogEntry_Result> {
        match value {
            0 => ::std::option::Option::Some(FortSearchLogEntry_Result::UNSET),
            1 => ::std::option::Option::Some(FortSearchLogEntry_Result::SUCCESS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FortSearchLogEntry_Result] = &[
            FortSearchLogEntry_Result::UNSET,
            FortSearchLogEntry_Result::SUCCESS,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<FortSearchLogEntry_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("FortSearchLogEntry_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for FortSearchLogEntry_Result {
}

#[derive(Clone,Default)]
pub struct ActionLogEntry {
    // message fields
    timestamp_ms: ::std::option::Option<i64>,
    sfida: ::std::option::Option<bool>,
    // message oneof groups
    Action: ::std::option::Option<ActionLogEntry_oneof_Action>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionLogEntry {}

#[derive(Clone,PartialEq)]
pub enum ActionLogEntry_oneof_Action {
    catch_pokemon(CatchPokemonLogEntry),
    fort_search(FortSearchLogEntry),
    buddy_pokemon(BuddyPokemonLogEntry),
}

impl ActionLogEntry {
    pub fn new() -> ActionLogEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionLogEntry {
        static mut instance: ::protobuf::lazy::Lazy<ActionLogEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionLogEntry,
        };
        unsafe {
            instance.get(|| {
                ActionLogEntry {
                    timestamp_ms: ::std::option::Option::None,
                    sfida: ::std::option::Option::None,
                    Action: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 timestamp_ms = 1;

    pub fn clear_timestamp_ms(&mut self) {
        self.timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_timestamp_ms(&self) -> bool {
        self.timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp_ms(&mut self, v: i64) {
        self.timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp_ms(&self) -> i64 {
        self.timestamp_ms.unwrap_or(0)
    }

    // optional bool sfida = 2;

    pub fn clear_sfida(&mut self) {
        self.sfida = ::std::option::Option::None;
    }

    pub fn has_sfida(&self) -> bool {
        self.sfida.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sfida(&mut self, v: bool) {
        self.sfida = ::std::option::Option::Some(v);
    }

    pub fn get_sfida(&self) -> bool {
        self.sfida.unwrap_or(false)
    }

    // optional .POGOProtos.Data.Logs.CatchPokemonLogEntry catch_pokemon = 3;

    pub fn clear_catch_pokemon(&mut self) {
        self.Action = ::std::option::Option::None;
    }

    pub fn has_catch_pokemon(&self) -> bool {
        match self.Action {
            ::std::option::Option::Some(ActionLogEntry_oneof_Action::catch_pokemon(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_catch_pokemon(&mut self, v: CatchPokemonLogEntry) {
        self.Action = ::std::option::Option::Some(ActionLogEntry_oneof_Action::catch_pokemon(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_catch_pokemon(&mut self) -> &mut CatchPokemonLogEntry {
        if let ::std::option::Option::Some(ActionLogEntry_oneof_Action::catch_pokemon(_)) = self.Action {
        } else {
            self.Action = ::std::option::Option::Some(ActionLogEntry_oneof_Action::catch_pokemon(CatchPokemonLogEntry::new()));
        }
        match self.Action {
            ::std::option::Option::Some(ActionLogEntry_oneof_Action::catch_pokemon(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_catch_pokemon(&mut self) -> CatchPokemonLogEntry {
        if self.has_catch_pokemon() {
            match self.Action.take() {
                ::std::option::Option::Some(ActionLogEntry_oneof_Action::catch_pokemon(v)) => v,
                _ => panic!(),
            }
        } else {
            CatchPokemonLogEntry::new()
        }
    }

    pub fn get_catch_pokemon(&self) -> &CatchPokemonLogEntry {
        match self.Action {
            ::std::option::Option::Some(ActionLogEntry_oneof_Action::catch_pokemon(ref v)) => v,
            _ => CatchPokemonLogEntry::default_instance(),
        }
    }

    // optional .POGOProtos.Data.Logs.FortSearchLogEntry fort_search = 4;

    pub fn clear_fort_search(&mut self) {
        self.Action = ::std::option::Option::None;
    }

    pub fn has_fort_search(&self) -> bool {
        match self.Action {
            ::std::option::Option::Some(ActionLogEntry_oneof_Action::fort_search(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_fort_search(&mut self, v: FortSearchLogEntry) {
        self.Action = ::std::option::Option::Some(ActionLogEntry_oneof_Action::fort_search(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fort_search(&mut self) -> &mut FortSearchLogEntry {
        if let ::std::option::Option::Some(ActionLogEntry_oneof_Action::fort_search(_)) = self.Action {
        } else {
            self.Action = ::std::option::Option::Some(ActionLogEntry_oneof_Action::fort_search(FortSearchLogEntry::new()));
        }
        match self.Action {
            ::std::option::Option::Some(ActionLogEntry_oneof_Action::fort_search(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_fort_search(&mut self) -> FortSearchLogEntry {
        if self.has_fort_search() {
            match self.Action.take() {
                ::std::option::Option::Some(ActionLogEntry_oneof_Action::fort_search(v)) => v,
                _ => panic!(),
            }
        } else {
            FortSearchLogEntry::new()
        }
    }

    pub fn get_fort_search(&self) -> &FortSearchLogEntry {
        match self.Action {
            ::std::option::Option::Some(ActionLogEntry_oneof_Action::fort_search(ref v)) => v,
            _ => FortSearchLogEntry::default_instance(),
        }
    }

    // optional .POGOProtos.Data.Logs.BuddyPokemonLogEntry buddy_pokemon = 5;

    pub fn clear_buddy_pokemon(&mut self) {
        self.Action = ::std::option::Option::None;
    }

    pub fn has_buddy_pokemon(&self) -> bool {
        match self.Action {
            ::std::option::Option::Some(ActionLogEntry_oneof_Action::buddy_pokemon(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_buddy_pokemon(&mut self, v: BuddyPokemonLogEntry) {
        self.Action = ::std::option::Option::Some(ActionLogEntry_oneof_Action::buddy_pokemon(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_buddy_pokemon(&mut self) -> &mut BuddyPokemonLogEntry {
        if let ::std::option::Option::Some(ActionLogEntry_oneof_Action::buddy_pokemon(_)) = self.Action {
        } else {
            self.Action = ::std::option::Option::Some(ActionLogEntry_oneof_Action::buddy_pokemon(BuddyPokemonLogEntry::new()));
        }
        match self.Action {
            ::std::option::Option::Some(ActionLogEntry_oneof_Action::buddy_pokemon(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_buddy_pokemon(&mut self) -> BuddyPokemonLogEntry {
        if self.has_buddy_pokemon() {
            match self.Action.take() {
                ::std::option::Option::Some(ActionLogEntry_oneof_Action::buddy_pokemon(v)) => v,
                _ => panic!(),
            }
        } else {
            BuddyPokemonLogEntry::new()
        }
    }

    pub fn get_buddy_pokemon(&self) -> &BuddyPokemonLogEntry {
        match self.Action {
            ::std::option::Option::Some(ActionLogEntry_oneof_Action::buddy_pokemon(ref v)) => v,
            _ => BuddyPokemonLogEntry::default_instance(),
        }
    }
}

impl ::protobuf::Message for ActionLogEntry {
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
                    self.timestamp_ms = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.sfida = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.Action = ::std::option::Option::Some(ActionLogEntry_oneof_Action::catch_pokemon(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.Action = ::std::option::Option::Some(ActionLogEntry_oneof_Action::fort_search(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.Action = ::std::option::Option::Some(ActionLogEntry_oneof_Action::buddy_pokemon(try!(is.read_message())));
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
        for value in &self.timestamp_ms {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.sfida.is_some() {
            my_size += 2;
        };
        if let ::std::option::Option::Some(ref v) = self.Action {
            match v {
                &ActionLogEntry_oneof_Action::catch_pokemon(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionLogEntry_oneof_Action::fort_search(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionLogEntry_oneof_Action::buddy_pokemon(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.timestamp_ms {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.sfida {
            try!(os.write_bool(2, v));
        };
        if let ::std::option::Option::Some(ref v) = self.Action {
            match v {
                &ActionLogEntry_oneof_Action::catch_pokemon(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ActionLogEntry_oneof_Action::fort_search(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ActionLogEntry_oneof_Action::buddy_pokemon(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
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
        ::std::any::TypeId::of::<ActionLogEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ActionLogEntry {
    fn new() -> ActionLogEntry {
        ActionLogEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionLogEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "timestamp_ms",
                    ActionLogEntry::has_timestamp_ms,
                    ActionLogEntry::get_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "sfida",
                    ActionLogEntry::has_sfida,
                    ActionLogEntry::get_sfida,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "catch_pokemon",
                    ActionLogEntry::has_catch_pokemon,
                    ActionLogEntry::get_catch_pokemon,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fort_search",
                    ActionLogEntry::has_fort_search,
                    ActionLogEntry::get_fort_search,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "buddy_pokemon",
                    ActionLogEntry::has_buddy_pokemon,
                    ActionLogEntry::get_buddy_pokemon,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionLogEntry>(
                    "ActionLogEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionLogEntry {
    fn clear(&mut self) {
        self.clear_timestamp_ms();
        self.clear_sfida();
        self.clear_catch_pokemon();
        self.clear_fort_search();
        self.clear_buddy_pokemon();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ActionLogEntry {
    fn eq(&self, other: &ActionLogEntry) -> bool {
        self.timestamp_ms == other.timestamp_ms &&
        self.sfida == other.sfida &&
        self.Action == other.Action &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ActionLogEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BuddyPokemonLogEntry {
    // message fields
    result: ::std::option::Option<BuddyPokemonLogEntry_Result>,
    pokemon_id: ::std::option::Option<super::POGOProtos_Enums::PokemonId>,
    amount: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BuddyPokemonLogEntry {}

impl BuddyPokemonLogEntry {
    pub fn new() -> BuddyPokemonLogEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BuddyPokemonLogEntry {
        static mut instance: ::protobuf::lazy::Lazy<BuddyPokemonLogEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BuddyPokemonLogEntry,
        };
        unsafe {
            instance.get(|| {
                BuddyPokemonLogEntry {
                    result: ::std::option::Option::None,
                    pokemon_id: ::std::option::Option::None,
                    amount: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Data.Logs.BuddyPokemonLogEntry.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: BuddyPokemonLogEntry_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> BuddyPokemonLogEntry_Result {
        self.result.unwrap_or(BuddyPokemonLogEntry_Result::UNSET)
    }

    // optional .POGOProtos.Enums.PokemonId pokemon_id = 2;

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

    // optional int32 amount = 3;

    pub fn clear_amount(&mut self) {
        self.amount = ::std::option::Option::None;
    }

    pub fn has_amount(&self) -> bool {
        self.amount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: i32) {
        self.amount = ::std::option::Option::Some(v);
    }

    pub fn get_amount(&self) -> i32 {
        self.amount.unwrap_or(0)
    }
}

impl ::protobuf::Message for BuddyPokemonLogEntry {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.pokemon_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.amount = ::std::option::Option::Some(tmp);
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
        for value in &self.pokemon_id {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.amount {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.pokemon_id {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.amount {
            try!(os.write_int32(3, v));
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
        ::std::any::TypeId::of::<BuddyPokemonLogEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BuddyPokemonLogEntry {
    fn new() -> BuddyPokemonLogEntry {
        BuddyPokemonLogEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<BuddyPokemonLogEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    BuddyPokemonLogEntry::has_result,
                    BuddyPokemonLogEntry::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "pokemon_id",
                    BuddyPokemonLogEntry::has_pokemon_id,
                    BuddyPokemonLogEntry::get_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "amount",
                    BuddyPokemonLogEntry::has_amount,
                    BuddyPokemonLogEntry::get_amount,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BuddyPokemonLogEntry>(
                    "BuddyPokemonLogEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BuddyPokemonLogEntry {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_pokemon_id();
        self.clear_amount();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BuddyPokemonLogEntry {
    fn eq(&self, other: &BuddyPokemonLogEntry) -> bool {
        self.result == other.result &&
        self.pokemon_id == other.pokemon_id &&
        self.amount == other.amount &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BuddyPokemonLogEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum BuddyPokemonLogEntry_Result {
    UNSET = 0,
    CANDY_FOUND = 1,
}

impl ::protobuf::ProtobufEnum for BuddyPokemonLogEntry_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<BuddyPokemonLogEntry_Result> {
        match value {
            0 => ::std::option::Option::Some(BuddyPokemonLogEntry_Result::UNSET),
            1 => ::std::option::Option::Some(BuddyPokemonLogEntry_Result::CANDY_FOUND),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [BuddyPokemonLogEntry_Result] = &[
            BuddyPokemonLogEntry_Result::UNSET,
            BuddyPokemonLogEntry_Result::CANDY_FOUND,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<BuddyPokemonLogEntry_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("BuddyPokemonLogEntry_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for BuddyPokemonLogEntry_Result {
}

#[derive(Clone,Default)]
pub struct CatchPokemonLogEntry {
    // message fields
    result: ::std::option::Option<CatchPokemonLogEntry_Result>,
    pokemon_id: ::std::option::Option<super::POGOProtos_Enums::PokemonId>,
    combat_points: ::std::option::Option<i32>,
    pokemon_data_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CatchPokemonLogEntry {}

impl CatchPokemonLogEntry {
    pub fn new() -> CatchPokemonLogEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CatchPokemonLogEntry {
        static mut instance: ::protobuf::lazy::Lazy<CatchPokemonLogEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CatchPokemonLogEntry,
        };
        unsafe {
            instance.get(|| {
                CatchPokemonLogEntry {
                    result: ::std::option::Option::None,
                    pokemon_id: ::std::option::Option::None,
                    combat_points: ::std::option::Option::None,
                    pokemon_data_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Data.Logs.CatchPokemonLogEntry.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CatchPokemonLogEntry_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CatchPokemonLogEntry_Result {
        self.result.unwrap_or(CatchPokemonLogEntry_Result::UNSET)
    }

    // optional .POGOProtos.Enums.PokemonId pokemon_id = 2;

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

    // optional int32 combat_points = 3;

    pub fn clear_combat_points(&mut self) {
        self.combat_points = ::std::option::Option::None;
    }

    pub fn has_combat_points(&self) -> bool {
        self.combat_points.is_some()
    }

    // Param is passed by value, moved
    pub fn set_combat_points(&mut self, v: i32) {
        self.combat_points = ::std::option::Option::Some(v);
    }

    pub fn get_combat_points(&self) -> i32 {
        self.combat_points.unwrap_or(0)
    }

    // optional fixed64 pokemon_data_id = 4;

    pub fn clear_pokemon_data_id(&mut self) {
        self.pokemon_data_id = ::std::option::Option::None;
    }

    pub fn has_pokemon_data_id(&self) -> bool {
        self.pokemon_data_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_data_id(&mut self, v: u64) {
        self.pokemon_data_id = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_data_id(&self) -> u64 {
        self.pokemon_data_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for CatchPokemonLogEntry {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.pokemon_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.combat_points = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.pokemon_data_id = ::std::option::Option::Some(tmp);
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
        for value in &self.pokemon_id {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.combat_points {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.pokemon_data_id.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.pokemon_id {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.combat_points {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.pokemon_data_id {
            try!(os.write_fixed64(4, v));
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
        ::std::any::TypeId::of::<CatchPokemonLogEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CatchPokemonLogEntry {
    fn new() -> CatchPokemonLogEntry {
        CatchPokemonLogEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<CatchPokemonLogEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "result",
                    CatchPokemonLogEntry::has_result,
                    CatchPokemonLogEntry::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "pokemon_id",
                    CatchPokemonLogEntry::has_pokemon_id,
                    CatchPokemonLogEntry::get_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "combat_points",
                    CatchPokemonLogEntry::has_combat_points,
                    CatchPokemonLogEntry::get_combat_points,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "pokemon_data_id",
                    CatchPokemonLogEntry::has_pokemon_data_id,
                    CatchPokemonLogEntry::get_pokemon_data_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CatchPokemonLogEntry>(
                    "CatchPokemonLogEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CatchPokemonLogEntry {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_pokemon_id();
        self.clear_combat_points();
        self.clear_pokemon_data_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CatchPokemonLogEntry {
    fn eq(&self, other: &CatchPokemonLogEntry) -> bool {
        self.result == other.result &&
        self.pokemon_id == other.pokemon_id &&
        self.combat_points == other.combat_points &&
        self.pokemon_data_id == other.pokemon_data_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CatchPokemonLogEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CatchPokemonLogEntry_Result {
    UNSET = 0,
    POKEMON_CAPTURED = 1,
    POKEMON_FLED = 2,
    POKEMON_HATCHED = 3,
}

impl ::protobuf::ProtobufEnum for CatchPokemonLogEntry_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CatchPokemonLogEntry_Result> {
        match value {
            0 => ::std::option::Option::Some(CatchPokemonLogEntry_Result::UNSET),
            1 => ::std::option::Option::Some(CatchPokemonLogEntry_Result::POKEMON_CAPTURED),
            2 => ::std::option::Option::Some(CatchPokemonLogEntry_Result::POKEMON_FLED),
            3 => ::std::option::Option::Some(CatchPokemonLogEntry_Result::POKEMON_HATCHED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CatchPokemonLogEntry_Result] = &[
            CatchPokemonLogEntry_Result::UNSET,
            CatchPokemonLogEntry_Result::POKEMON_CAPTURED,
            CatchPokemonLogEntry_Result::POKEMON_FLED,
            CatchPokemonLogEntry_Result::POKEMON_HATCHED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CatchPokemonLogEntry_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CatchPokemonLogEntry_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CatchPokemonLogEntry_Result {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1a, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x2e, 0x4c, 0x6f, 0x67, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x14, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x4c, 0x6f,
    0x67, 0x73, 0x1a, 0x1f, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49,
    0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x16, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x00, 0x50, 0x01, 0x22,
    0xe7, 0x01, 0x0a, 0x12, 0x46, 0x6f, 0x72, 0x74, 0x53, 0x65, 0x61, 0x72, 0x63, 0x68, 0x4c, 0x6f,
    0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x47, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2f, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x4c, 0x6f, 0x67, 0x73, 0x2e, 0x46, 0x6f,
    0x72, 0x74, 0x53, 0x65, 0x61, 0x72, 0x63, 0x68, 0x4c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79,
    0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12,
    0x17, 0x0a, 0x07, 0x66, 0x6f, 0x72, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x06, 0x66, 0x6f, 0x72, 0x74, 0x49, 0x64, 0x12, 0x39, 0x0a, 0x05, 0x69, 0x74, 0x65, 0x6d,
    0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49,
    0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x52, 0x05, 0x69, 0x74,
    0x65, 0x6d, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x65, 0x67, 0x67, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x04, 0x65, 0x67, 0x67, 0x73, 0x22, 0x20, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07,
    0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x01, 0x22, 0xc6, 0x02, 0x0a, 0x0e, 0x41, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x4c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x21, 0x0a, 0x0c,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x03, 0x52, 0x0b, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12,
    0x14, 0x0a, 0x05, 0x73, 0x66, 0x69, 0x64, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x05,
    0x73, 0x66, 0x69, 0x64, 0x61, 0x12, 0x51, 0x0a, 0x0d, 0x63, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x70,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x4c,
    0x6f, 0x67, 0x73, 0x2e, 0x43, 0x61, 0x74, 0x63, 0x68, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e,
    0x4c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x48, 0x00, 0x52, 0x0c, 0x63, 0x61, 0x74, 0x63,
    0x68, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x12, 0x4b, 0x0a, 0x0b, 0x66, 0x6f, 0x72, 0x74,
    0x5f, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e,
    0x4c, 0x6f, 0x67, 0x73, 0x2e, 0x46, 0x6f, 0x72, 0x74, 0x53, 0x65, 0x61, 0x72, 0x63, 0x68, 0x4c,
    0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x48, 0x00, 0x52, 0x0a, 0x66, 0x6f, 0x72, 0x74, 0x53,
    0x65, 0x61, 0x72, 0x63, 0x68, 0x12, 0x51, 0x0a, 0x0d, 0x62, 0x75, 0x64, 0x64, 0x79, 0x5f, 0x70,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x4c,
    0x6f, 0x67, 0x73, 0x2e, 0x42, 0x75, 0x64, 0x64, 0x79, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e,
    0x4c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x48, 0x00, 0x52, 0x0c, 0x62, 0x75, 0x64, 0x64,
    0x79, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x42, 0x08, 0x0a, 0x06, 0x41, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x22, 0xdb, 0x01, 0x0a, 0x14, 0x42, 0x75, 0x64, 0x64, 0x79, 0x50, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x4c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x49, 0x0a, 0x06, 0x72,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x31, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x4c, 0x6f,
    0x67, 0x73, 0x2e, 0x42, 0x75, 0x64, 0x64, 0x79, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x4c,
    0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06,
    0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x3a, 0x0a, 0x0a, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f,
    0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x52, 0x09, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e,
    0x49, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x22, 0x24, 0x0a, 0x06, 0x52, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54, 0x10, 0x00, 0x12,
    0x0f, 0x0a, 0x0b, 0x43, 0x41, 0x4e, 0x44, 0x59, 0x5f, 0x46, 0x4f, 0x55, 0x4e, 0x44, 0x10, 0x01,
    0x22, 0xbc, 0x02, 0x0a, 0x14, 0x43, 0x61, 0x74, 0x63, 0x68, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f,
    0x6e, 0x4c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x49, 0x0a, 0x06, 0x72, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x31, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x4c, 0x6f, 0x67, 0x73,
    0x2e, 0x43, 0x61, 0x74, 0x63, 0x68, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x4c, 0x6f, 0x67,
    0x45, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x12, 0x3a, 0x0a, 0x0a, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f,
    0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x52, 0x09, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64,
    0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6f, 0x6d, 0x62, 0x61, 0x74, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74,
    0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x63, 0x6f, 0x6d, 0x62, 0x61, 0x74, 0x50,
    0x6f, 0x69, 0x6e, 0x74, 0x73, 0x12, 0x26, 0x0a, 0x0f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e,
    0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0d,
    0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x49, 0x64, 0x22, 0x50, 0x0a,
    0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x55, 0x4e, 0x53, 0x45, 0x54,
    0x10, 0x00, 0x12, 0x14, 0x0a, 0x10, 0x50, 0x4f, 0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x43, 0x41,
    0x50, 0x54, 0x55, 0x52, 0x45, 0x44, 0x10, 0x01, 0x12, 0x10, 0x0a, 0x0c, 0x50, 0x4f, 0x4b, 0x45,
    0x4d, 0x4f, 0x4e, 0x5f, 0x46, 0x4c, 0x45, 0x44, 0x10, 0x02, 0x12, 0x13, 0x0a, 0x0f, 0x50, 0x4f,
    0x4b, 0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x48, 0x41, 0x54, 0x43, 0x48, 0x45, 0x44, 0x10, 0x03, 0x4a,
    0xc2, 0x0d, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x31, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x1c, 0x0a, 0x09,
    0x0a, 0x02, 0x0a, 0x00, 0x12, 0x03, 0x03, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x03, 0x0e, 0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x04, 0x07, 0x0d, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x0e, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x06, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06,
    0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x08, 0x43, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x07, 0x08, 0x06, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x07, 0x08, 0x37, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x38, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x41, 0x42, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x08, 0x08, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x08, 0x08, 0x07, 0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x08,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x08, 0x0f, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x08, 0x19, 0x1a, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x09, 0x08, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x09, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x06, 0x12, 0x03, 0x09, 0x11, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x09, 0x35, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x09, 0x3d, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x08, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x0a, 0x08, 0x09, 0x3f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0a, 0x08, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0a, 0x0e, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0a, 0x15, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04,
    0x00, 0x12, 0x04, 0x0c, 0x08, 0x0f, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x0c, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x0d, 0x10, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x0d, 0x10, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x0d, 0x18, 0x19, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x0e, 0x10, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x0e, 0x10, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x0e, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x11, 0x00, 0x1a, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x11, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x12, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x12, 0x08, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x12, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x12, 0x0e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x1d,
    0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x13, 0x08, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x13, 0x08, 0x12, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x13, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x13, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x13, 0x15, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x08, 0x00, 0x12,
    0x04, 0x15, 0x08, 0x19, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x08, 0x00, 0x01, 0x12, 0x03,
    0x15, 0x0e, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x16, 0x10, 0x4d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x16, 0x10, 0x3a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x16, 0x3b, 0x48, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x16, 0x4b, 0x4c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x03, 0x12, 0x03, 0x17, 0x10, 0x49, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x06,
    0x12, 0x03, 0x17, 0x10, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x17, 0x39, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x17, 0x47,
    0x48, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x18, 0x10, 0x4d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x06, 0x12, 0x03, 0x18, 0x10, 0x3a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x18, 0x3b, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x18, 0x4b, 0x4c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04,
    0x1b, 0x00, 0x24, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x1c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x08, 0x45, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x1c, 0x08, 0x1b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1c, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x3a, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x1c, 0x43, 0x44, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x1d, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x1d, 0x08,
    0x1c, 0x45, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x1d, 0x08, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1d, 0x24, 0x2e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1d, 0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1e, 0x08, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x04, 0x12, 0x04, 0x1e, 0x08, 0x1d, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x1e, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x1e, 0x0e, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1e,
    0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02, 0x04, 0x00, 0x12, 0x04, 0x20, 0x08, 0x23, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x04, 0x00, 0x01, 0x12, 0x03, 0x20, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x21, 0x10, 0x1a, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x10, 0x15, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x21, 0x18, 0x19, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x02, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x22, 0x10, 0x20, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x22, 0x10, 0x1b, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x22, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x25, 0x00, 0x31, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x25, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x26, 0x08,
    0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x26, 0x08, 0x25, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x26, 0x08, 0x39, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x26, 0x3a, 0x40, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x26, 0x43, 0x44, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x01, 0x12, 0x03, 0x27, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x27, 0x08, 0x26, 0x45, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x27, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x27,
    0x24, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x27, 0x31, 0x32,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x28, 0x08, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x04, 0x28, 0x08, 0x27, 0x33, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x28, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x28, 0x0e, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x28, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03,
    0x29, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x04, 0x29, 0x08,
    0x28, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x29, 0x08, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x29, 0x10, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x29, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x03, 0x04, 0x00, 0x12, 0x04, 0x2b, 0x08, 0x30, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x2c, 0x10, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x2c, 0x10, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x2c, 0x18, 0x19, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x2d, 0x10, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x2d, 0x10, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x2d, 0x23, 0x24, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x2e, 0x10, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x2e, 0x10, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x2e, 0x1f, 0x20, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x2f, 0x10, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x2f, 0x10, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12,
    0x03, 0x2f, 0x22, 0x23, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
