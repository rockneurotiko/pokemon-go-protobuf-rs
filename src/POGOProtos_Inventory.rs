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
pub struct InventoryUpgrade {
    // message fields
    item_id: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemId>,
    upgrade_type: ::std::option::Option<InventoryUpgradeType>,
    additional_storage: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InventoryUpgrade {}

impl InventoryUpgrade {
    pub fn new() -> InventoryUpgrade {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InventoryUpgrade {
        static mut instance: ::protobuf::lazy::Lazy<InventoryUpgrade> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InventoryUpgrade,
        };
        unsafe {
            instance.get(|| {
                InventoryUpgrade {
                    item_id: ::std::option::Option::None,
                    upgrade_type: ::std::option::Option::None,
                    additional_storage: ::std::option::Option::None,
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

    // optional .POGOProtos.Inventory.InventoryUpgradeType upgrade_type = 2;

    pub fn clear_upgrade_type(&mut self) {
        self.upgrade_type = ::std::option::Option::None;
    }

    pub fn has_upgrade_type(&self) -> bool {
        self.upgrade_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upgrade_type(&mut self, v: InventoryUpgradeType) {
        self.upgrade_type = ::std::option::Option::Some(v);
    }

    pub fn get_upgrade_type(&self) -> InventoryUpgradeType {
        self.upgrade_type.unwrap_or(InventoryUpgradeType::UPGRADE_UNSET)
    }

    // optional int32 additional_storage = 3;

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
}

impl ::protobuf::Message for InventoryUpgrade {
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
                    let tmp = try!(is.read_enum());
                    self.upgrade_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.additional_storage = ::std::option::Option::Some(tmp);
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
        for value in &self.upgrade_type {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.additional_storage {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_id {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.upgrade_type {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.additional_storage {
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
        ::std::any::TypeId::of::<InventoryUpgrade>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InventoryUpgrade {
    fn new() -> InventoryUpgrade {
        InventoryUpgrade::new()
    }

    fn descriptor_static(_: ::std::option::Option<InventoryUpgrade>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "item_id",
                    InventoryUpgrade::has_item_id,
                    InventoryUpgrade::get_item_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "upgrade_type",
                    InventoryUpgrade::has_upgrade_type,
                    InventoryUpgrade::get_upgrade_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "additional_storage",
                    InventoryUpgrade::has_additional_storage,
                    InventoryUpgrade::get_additional_storage,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InventoryUpgrade>(
                    "InventoryUpgrade",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InventoryUpgrade {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_upgrade_type();
        self.clear_additional_storage();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InventoryUpgrade {
    fn eq(&self, other: &InventoryUpgrade) -> bool {
        self.item_id == other.item_id &&
        self.upgrade_type == other.upgrade_type &&
        self.additional_storage == other.additional_storage &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InventoryUpgrade {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InventoryItem {
    // message fields
    modified_timestamp_ms: ::std::option::Option<i64>,
    deleted_item: ::protobuf::SingularPtrField<InventoryItem_DeletedItem>,
    inventory_item_data: ::protobuf::SingularPtrField<InventoryItemData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InventoryItem {}

impl InventoryItem {
    pub fn new() -> InventoryItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InventoryItem {
        static mut instance: ::protobuf::lazy::Lazy<InventoryItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InventoryItem,
        };
        unsafe {
            instance.get(|| {
                InventoryItem {
                    modified_timestamp_ms: ::std::option::Option::None,
                    deleted_item: ::protobuf::SingularPtrField::none(),
                    inventory_item_data: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 modified_timestamp_ms = 1;

    pub fn clear_modified_timestamp_ms(&mut self) {
        self.modified_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_modified_timestamp_ms(&self) -> bool {
        self.modified_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modified_timestamp_ms(&mut self, v: i64) {
        self.modified_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_modified_timestamp_ms(&self) -> i64 {
        self.modified_timestamp_ms.unwrap_or(0)
    }

    // optional .POGOProtos.Inventory.InventoryItem.DeletedItem deleted_item = 2;

    pub fn clear_deleted_item(&mut self) {
        self.deleted_item.clear();
    }

    pub fn has_deleted_item(&self) -> bool {
        self.deleted_item.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deleted_item(&mut self, v: InventoryItem_DeletedItem) {
        self.deleted_item = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deleted_item(&mut self) -> &mut InventoryItem_DeletedItem {
        if self.deleted_item.is_none() {
            self.deleted_item.set_default();
        };
        self.deleted_item.as_mut().unwrap()
    }

    // Take field
    pub fn take_deleted_item(&mut self) -> InventoryItem_DeletedItem {
        self.deleted_item.take().unwrap_or_else(|| InventoryItem_DeletedItem::new())
    }

    pub fn get_deleted_item(&self) -> &InventoryItem_DeletedItem {
        self.deleted_item.as_ref().unwrap_or_else(|| InventoryItem_DeletedItem::default_instance())
    }

    // optional .POGOProtos.Inventory.InventoryItemData inventory_item_data = 3;

    pub fn clear_inventory_item_data(&mut self) {
        self.inventory_item_data.clear();
    }

    pub fn has_inventory_item_data(&self) -> bool {
        self.inventory_item_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inventory_item_data(&mut self, v: InventoryItemData) {
        self.inventory_item_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inventory_item_data(&mut self) -> &mut InventoryItemData {
        if self.inventory_item_data.is_none() {
            self.inventory_item_data.set_default();
        };
        self.inventory_item_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_inventory_item_data(&mut self) -> InventoryItemData {
        self.inventory_item_data.take().unwrap_or_else(|| InventoryItemData::new())
    }

    pub fn get_inventory_item_data(&self) -> &InventoryItemData {
        self.inventory_item_data.as_ref().unwrap_or_else(|| InventoryItemData::default_instance())
    }
}

impl ::protobuf::Message for InventoryItem {
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
                    self.modified_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.deleted_item));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.inventory_item_data));
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
        for value in &self.modified_timestamp_ms {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.deleted_item {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.inventory_item_data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.modified_timestamp_ms {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.deleted_item.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.inventory_item_data.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<InventoryItem>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InventoryItem {
    fn new() -> InventoryItem {
        InventoryItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<InventoryItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "modified_timestamp_ms",
                    InventoryItem::has_modified_timestamp_ms,
                    InventoryItem::get_modified_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "deleted_item",
                    InventoryItem::has_deleted_item,
                    InventoryItem::get_deleted_item,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "inventory_item_data",
                    InventoryItem::has_inventory_item_data,
                    InventoryItem::get_inventory_item_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InventoryItem>(
                    "InventoryItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InventoryItem {
    fn clear(&mut self) {
        self.clear_modified_timestamp_ms();
        self.clear_deleted_item();
        self.clear_inventory_item_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InventoryItem {
    fn eq(&self, other: &InventoryItem) -> bool {
        self.modified_timestamp_ms == other.modified_timestamp_ms &&
        self.deleted_item == other.deleted_item &&
        self.inventory_item_data == other.inventory_item_data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InventoryItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InventoryItem_DeletedItem {
    // message fields
    pokemon_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InventoryItem_DeletedItem {}

impl InventoryItem_DeletedItem {
    pub fn new() -> InventoryItem_DeletedItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InventoryItem_DeletedItem {
        static mut instance: ::protobuf::lazy::Lazy<InventoryItem_DeletedItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InventoryItem_DeletedItem,
        };
        unsafe {
            instance.get(|| {
                InventoryItem_DeletedItem {
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

impl ::protobuf::Message for InventoryItem_DeletedItem {
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
        ::std::any::TypeId::of::<InventoryItem_DeletedItem>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InventoryItem_DeletedItem {
    fn new() -> InventoryItem_DeletedItem {
        InventoryItem_DeletedItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<InventoryItem_DeletedItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "pokemon_id",
                    InventoryItem_DeletedItem::has_pokemon_id,
                    InventoryItem_DeletedItem::get_pokemon_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InventoryItem_DeletedItem>(
                    "InventoryItem_DeletedItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InventoryItem_DeletedItem {
    fn clear(&mut self) {
        self.clear_pokemon_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InventoryItem_DeletedItem {
    fn eq(&self, other: &InventoryItem_DeletedItem) -> bool {
        self.pokemon_id == other.pokemon_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InventoryItem_DeletedItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EggIncubator {
    // message fields
    id: ::protobuf::SingularField<::std::string::String>,
    item_id: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemId>,
    incubator_type: ::std::option::Option<EggIncubatorType>,
    uses_remaining: ::std::option::Option<i32>,
    pokemon_id: ::std::option::Option<u64>,
    start_km_walked: ::std::option::Option<f64>,
    target_km_walked: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EggIncubator {}

impl EggIncubator {
    pub fn new() -> EggIncubator {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EggIncubator {
        static mut instance: ::protobuf::lazy::Lazy<EggIncubator> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EggIncubator,
        };
        unsafe {
            instance.get(|| {
                EggIncubator {
                    id: ::protobuf::SingularField::none(),
                    item_id: ::std::option::Option::None,
                    incubator_type: ::std::option::Option::None,
                    uses_remaining: ::std::option::Option::None,
                    pokemon_id: ::std::option::Option::None,
                    start_km_walked: ::std::option::Option::None,
                    target_km_walked: ::std::option::Option::None,
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

    // optional .POGOProtos.Inventory.Item.ItemId item_id = 2;

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

    // optional .POGOProtos.Inventory.EggIncubatorType incubator_type = 3;

    pub fn clear_incubator_type(&mut self) {
        self.incubator_type = ::std::option::Option::None;
    }

    pub fn has_incubator_type(&self) -> bool {
        self.incubator_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_incubator_type(&mut self, v: EggIncubatorType) {
        self.incubator_type = ::std::option::Option::Some(v);
    }

    pub fn get_incubator_type(&self) -> EggIncubatorType {
        self.incubator_type.unwrap_or(EggIncubatorType::INCUBATOR_UNSET)
    }

    // optional int32 uses_remaining = 4;

    pub fn clear_uses_remaining(&mut self) {
        self.uses_remaining = ::std::option::Option::None;
    }

    pub fn has_uses_remaining(&self) -> bool {
        self.uses_remaining.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uses_remaining(&mut self, v: i32) {
        self.uses_remaining = ::std::option::Option::Some(v);
    }

    pub fn get_uses_remaining(&self) -> i32 {
        self.uses_remaining.unwrap_or(0)
    }

    // optional uint64 pokemon_id = 5;

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

    // optional double start_km_walked = 6;

    pub fn clear_start_km_walked(&mut self) {
        self.start_km_walked = ::std::option::Option::None;
    }

    pub fn has_start_km_walked(&self) -> bool {
        self.start_km_walked.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_km_walked(&mut self, v: f64) {
        self.start_km_walked = ::std::option::Option::Some(v);
    }

    pub fn get_start_km_walked(&self) -> f64 {
        self.start_km_walked.unwrap_or(0.)
    }

    // optional double target_km_walked = 7;

    pub fn clear_target_km_walked(&mut self) {
        self.target_km_walked = ::std::option::Option::None;
    }

    pub fn has_target_km_walked(&self) -> bool {
        self.target_km_walked.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_km_walked(&mut self, v: f64) {
        self.target_km_walked = ::std::option::Option::Some(v);
    }

    pub fn get_target_km_walked(&self) -> f64 {
        self.target_km_walked.unwrap_or(0.)
    }
}

impl ::protobuf::Message for EggIncubator {
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
                    let tmp = try!(is.read_enum());
                    self.item_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.incubator_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.uses_remaining = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.pokemon_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.start_km_walked = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.target_km_walked = ::std::option::Option::Some(tmp);
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
        for value in &self.item_id {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.incubator_type {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in &self.uses_remaining {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pokemon_id {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.start_km_walked.is_some() {
            my_size += 9;
        };
        if self.target_km_walked.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.item_id {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.incubator_type {
            try!(os.write_enum(3, v.value()));
        };
        if let Some(v) = self.uses_remaining {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.pokemon_id {
            try!(os.write_uint64(5, v));
        };
        if let Some(v) = self.start_km_walked {
            try!(os.write_double(6, v));
        };
        if let Some(v) = self.target_km_walked {
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
        ::std::any::TypeId::of::<EggIncubator>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EggIncubator {
    fn new() -> EggIncubator {
        EggIncubator::new()
    }

    fn descriptor_static(_: ::std::option::Option<EggIncubator>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "id",
                    EggIncubator::has_id,
                    EggIncubator::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "item_id",
                    EggIncubator::has_item_id,
                    EggIncubator::get_item_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "incubator_type",
                    EggIncubator::has_incubator_type,
                    EggIncubator::get_incubator_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "uses_remaining",
                    EggIncubator::has_uses_remaining,
                    EggIncubator::get_uses_remaining,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "pokemon_id",
                    EggIncubator::has_pokemon_id,
                    EggIncubator::get_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "start_km_walked",
                    EggIncubator::has_start_km_walked,
                    EggIncubator::get_start_km_walked,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "target_km_walked",
                    EggIncubator::has_target_km_walked,
                    EggIncubator::get_target_km_walked,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EggIncubator>(
                    "EggIncubator",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EggIncubator {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_item_id();
        self.clear_incubator_type();
        self.clear_uses_remaining();
        self.clear_pokemon_id();
        self.clear_start_km_walked();
        self.clear_target_km_walked();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EggIncubator {
    fn eq(&self, other: &EggIncubator) -> bool {
        self.id == other.id &&
        self.item_id == other.item_id &&
        self.incubator_type == other.incubator_type &&
        self.uses_remaining == other.uses_remaining &&
        self.pokemon_id == other.pokemon_id &&
        self.start_km_walked == other.start_km_walked &&
        self.target_km_walked == other.target_km_walked &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EggIncubator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AppliedItems {
    // message fields
    item: ::protobuf::RepeatedField<AppliedItem>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AppliedItems {}

impl AppliedItems {
    pub fn new() -> AppliedItems {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AppliedItems {
        static mut instance: ::protobuf::lazy::Lazy<AppliedItems> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AppliedItems,
        };
        unsafe {
            instance.get(|| {
                AppliedItems {
                    item: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .POGOProtos.Inventory.AppliedItem item = 4;

    pub fn clear_item(&mut self) {
        self.item.clear();
    }

    // Param is passed by value, moved
    pub fn set_item(&mut self, v: ::protobuf::RepeatedField<AppliedItem>) {
        self.item = v;
    }

    // Mutable pointer to the field.
    pub fn mut_item(&mut self) -> &mut ::protobuf::RepeatedField<AppliedItem> {
        &mut self.item
    }

    // Take field
    pub fn take_item(&mut self) -> ::protobuf::RepeatedField<AppliedItem> {
        ::std::mem::replace(&mut self.item, ::protobuf::RepeatedField::new())
    }

    pub fn get_item(&self) -> &[AppliedItem] {
        &self.item
    }
}

impl ::protobuf::Message for AppliedItems {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.item));
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
        for value in &self.item {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.item {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<AppliedItems>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AppliedItems {
    fn new() -> AppliedItems {
        AppliedItems::new()
    }

    fn descriptor_static(_: ::std::option::Option<AppliedItems>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "item",
                    AppliedItems::get_item,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AppliedItems>(
                    "AppliedItems",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AppliedItems {
    fn clear(&mut self) {
        self.clear_item();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AppliedItems {
    fn eq(&self, other: &AppliedItems) -> bool {
        self.item == other.item &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AppliedItems {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EggIncubators {
    // message fields
    egg_incubator: ::protobuf::RepeatedField<EggIncubator>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EggIncubators {}

impl EggIncubators {
    pub fn new() -> EggIncubators {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EggIncubators {
        static mut instance: ::protobuf::lazy::Lazy<EggIncubators> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EggIncubators,
        };
        unsafe {
            instance.get(|| {
                EggIncubators {
                    egg_incubator: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .POGOProtos.Inventory.EggIncubator egg_incubator = 1;

    pub fn clear_egg_incubator(&mut self) {
        self.egg_incubator.clear();
    }

    // Param is passed by value, moved
    pub fn set_egg_incubator(&mut self, v: ::protobuf::RepeatedField<EggIncubator>) {
        self.egg_incubator = v;
    }

    // Mutable pointer to the field.
    pub fn mut_egg_incubator(&mut self) -> &mut ::protobuf::RepeatedField<EggIncubator> {
        &mut self.egg_incubator
    }

    // Take field
    pub fn take_egg_incubator(&mut self) -> ::protobuf::RepeatedField<EggIncubator> {
        ::std::mem::replace(&mut self.egg_incubator, ::protobuf::RepeatedField::new())
    }

    pub fn get_egg_incubator(&self) -> &[EggIncubator] {
        &self.egg_incubator
    }
}

impl ::protobuf::Message for EggIncubators {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.egg_incubator));
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
        for value in &self.egg_incubator {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.egg_incubator {
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
        ::std::any::TypeId::of::<EggIncubators>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EggIncubators {
    fn new() -> EggIncubators {
        EggIncubators::new()
    }

    fn descriptor_static(_: ::std::option::Option<EggIncubators>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "egg_incubator",
                    EggIncubators::get_egg_incubator,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EggIncubators>(
                    "EggIncubators",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EggIncubators {
    fn clear(&mut self) {
        self.clear_egg_incubator();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EggIncubators {
    fn eq(&self, other: &EggIncubators) -> bool {
        self.egg_incubator == other.egg_incubator &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EggIncubators {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AppliedItem {
    // message fields
    item_id: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemId>,
    item_type: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemType>,
    expire_ms: ::std::option::Option<i64>,
    applied_ms: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AppliedItem {}

impl AppliedItem {
    pub fn new() -> AppliedItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AppliedItem {
        static mut instance: ::protobuf::lazy::Lazy<AppliedItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AppliedItem,
        };
        unsafe {
            instance.get(|| {
                AppliedItem {
                    item_id: ::std::option::Option::None,
                    item_type: ::std::option::Option::None,
                    expire_ms: ::std::option::Option::None,
                    applied_ms: ::std::option::Option::None,
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

    // optional .POGOProtos.Inventory.Item.ItemType item_type = 2;

    pub fn clear_item_type(&mut self) {
        self.item_type = ::std::option::Option::None;
    }

    pub fn has_item_type(&self) -> bool {
        self.item_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_type(&mut self, v: super::POGOProtos_Inventory_Item::ItemType) {
        self.item_type = ::std::option::Option::Some(v);
    }

    pub fn get_item_type(&self) -> super::POGOProtos_Inventory_Item::ItemType {
        self.item_type.unwrap_or(super::POGOProtos_Inventory_Item::ItemType::ITEM_TYPE_NONE)
    }

    // optional int64 expire_ms = 3;

    pub fn clear_expire_ms(&mut self) {
        self.expire_ms = ::std::option::Option::None;
    }

    pub fn has_expire_ms(&self) -> bool {
        self.expire_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expire_ms(&mut self, v: i64) {
        self.expire_ms = ::std::option::Option::Some(v);
    }

    pub fn get_expire_ms(&self) -> i64 {
        self.expire_ms.unwrap_or(0)
    }

    // optional int64 applied_ms = 4;

    pub fn clear_applied_ms(&mut self) {
        self.applied_ms = ::std::option::Option::None;
    }

    pub fn has_applied_ms(&self) -> bool {
        self.applied_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_applied_ms(&mut self, v: i64) {
        self.applied_ms = ::std::option::Option::Some(v);
    }

    pub fn get_applied_ms(&self) -> i64 {
        self.applied_ms.unwrap_or(0)
    }
}

impl ::protobuf::Message for AppliedItem {
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
                    let tmp = try!(is.read_enum());
                    self.item_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.expire_ms = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.applied_ms = ::std::option::Option::Some(tmp);
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
        for value in &self.item_type {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.expire_ms {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.applied_ms {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_id {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.item_type {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.expire_ms {
            try!(os.write_int64(3, v));
        };
        if let Some(v) = self.applied_ms {
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
        ::std::any::TypeId::of::<AppliedItem>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AppliedItem {
    fn new() -> AppliedItem {
        AppliedItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<AppliedItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "item_id",
                    AppliedItem::has_item_id,
                    AppliedItem::get_item_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "item_type",
                    AppliedItem::has_item_type,
                    AppliedItem::get_item_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "expire_ms",
                    AppliedItem::has_expire_ms,
                    AppliedItem::get_expire_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "applied_ms",
                    AppliedItem::has_applied_ms,
                    AppliedItem::get_applied_ms,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AppliedItem>(
                    "AppliedItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AppliedItem {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_item_type();
        self.clear_expire_ms();
        self.clear_applied_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AppliedItem {
    fn eq(&self, other: &AppliedItem) -> bool {
        self.item_id == other.item_id &&
        self.item_type == other.item_type &&
        self.expire_ms == other.expire_ms &&
        self.applied_ms == other.applied_ms &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AppliedItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InventoryUpgrades {
    // message fields
    inventory_upgrades: ::protobuf::RepeatedField<InventoryUpgrade>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InventoryUpgrades {}

impl InventoryUpgrades {
    pub fn new() -> InventoryUpgrades {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InventoryUpgrades {
        static mut instance: ::protobuf::lazy::Lazy<InventoryUpgrades> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InventoryUpgrades,
        };
        unsafe {
            instance.get(|| {
                InventoryUpgrades {
                    inventory_upgrades: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .POGOProtos.Inventory.InventoryUpgrade inventory_upgrades = 1;

    pub fn clear_inventory_upgrades(&mut self) {
        self.inventory_upgrades.clear();
    }

    // Param is passed by value, moved
    pub fn set_inventory_upgrades(&mut self, v: ::protobuf::RepeatedField<InventoryUpgrade>) {
        self.inventory_upgrades = v;
    }

    // Mutable pointer to the field.
    pub fn mut_inventory_upgrades(&mut self) -> &mut ::protobuf::RepeatedField<InventoryUpgrade> {
        &mut self.inventory_upgrades
    }

    // Take field
    pub fn take_inventory_upgrades(&mut self) -> ::protobuf::RepeatedField<InventoryUpgrade> {
        ::std::mem::replace(&mut self.inventory_upgrades, ::protobuf::RepeatedField::new())
    }

    pub fn get_inventory_upgrades(&self) -> &[InventoryUpgrade] {
        &self.inventory_upgrades
    }
}

impl ::protobuf::Message for InventoryUpgrades {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.inventory_upgrades));
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
        for value in &self.inventory_upgrades {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.inventory_upgrades {
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
        ::std::any::TypeId::of::<InventoryUpgrades>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InventoryUpgrades {
    fn new() -> InventoryUpgrades {
        InventoryUpgrades::new()
    }

    fn descriptor_static(_: ::std::option::Option<InventoryUpgrades>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "inventory_upgrades",
                    InventoryUpgrades::get_inventory_upgrades,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InventoryUpgrades>(
                    "InventoryUpgrades",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InventoryUpgrades {
    fn clear(&mut self) {
        self.clear_inventory_upgrades();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InventoryUpgrades {
    fn eq(&self, other: &InventoryUpgrades) -> bool {
        self.inventory_upgrades == other.inventory_upgrades &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InventoryUpgrades {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Candy {
    // message fields
    family_id: ::std::option::Option<super::POGOProtos_Enums::PokemonFamilyId>,
    candy: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Candy {}

impl Candy {
    pub fn new() -> Candy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Candy {
        static mut instance: ::protobuf::lazy::Lazy<Candy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Candy,
        };
        unsafe {
            instance.get(|| {
                Candy {
                    family_id: ::std::option::Option::None,
                    candy: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Enums.PokemonFamilyId family_id = 1;

    pub fn clear_family_id(&mut self) {
        self.family_id = ::std::option::Option::None;
    }

    pub fn has_family_id(&self) -> bool {
        self.family_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_family_id(&mut self, v: super::POGOProtos_Enums::PokemonFamilyId) {
        self.family_id = ::std::option::Option::Some(v);
    }

    pub fn get_family_id(&self) -> super::POGOProtos_Enums::PokemonFamilyId {
        self.family_id.unwrap_or(super::POGOProtos_Enums::PokemonFamilyId::FAMILY_UNSET)
    }

    // optional int32 candy = 2;

    pub fn clear_candy(&mut self) {
        self.candy = ::std::option::Option::None;
    }

    pub fn has_candy(&self) -> bool {
        self.candy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_candy(&mut self, v: i32) {
        self.candy = ::std::option::Option::Some(v);
    }

    pub fn get_candy(&self) -> i32 {
        self.candy.unwrap_or(0)
    }
}

impl ::protobuf::Message for Candy {
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
                    self.family_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.candy = ::std::option::Option::Some(tmp);
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
        for value in &self.family_id {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.candy {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.family_id {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.candy {
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
        ::std::any::TypeId::of::<Candy>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Candy {
    fn new() -> Candy {
        Candy::new()
    }

    fn descriptor_static(_: ::std::option::Option<Candy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "family_id",
                    Candy::has_family_id,
                    Candy::get_family_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "candy",
                    Candy::has_candy,
                    Candy::get_candy,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Candy>(
                    "Candy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Candy {
    fn clear(&mut self) {
        self.clear_family_id();
        self.clear_candy();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Candy {
    fn eq(&self, other: &Candy) -> bool {
        self.family_id == other.family_id &&
        self.candy == other.candy &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Candy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InventoryItemData {
    // message fields
    pokemon_data: ::protobuf::SingularPtrField<super::POGOProtos_Data::PokemonData>,
    item: ::protobuf::SingularPtrField<super::POGOProtos_Inventory_Item::ItemData>,
    pokedex_entry: ::protobuf::SingularPtrField<super::POGOProtos_Data::PokedexEntry>,
    player_stats: ::protobuf::SingularPtrField<super::POGOProtos_Data_Player::PlayerStats>,
    player_currency: ::protobuf::SingularPtrField<super::POGOProtos_Data_Player::PlayerCurrency>,
    player_camera: ::protobuf::SingularPtrField<super::POGOProtos_Data_Player::PlayerCamera>,
    inventory_upgrades: ::protobuf::SingularPtrField<InventoryUpgrades>,
    applied_items: ::protobuf::SingularPtrField<AppliedItems>,
    egg_incubators: ::protobuf::SingularPtrField<EggIncubators>,
    candy: ::protobuf::SingularPtrField<Candy>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InventoryItemData {}

impl InventoryItemData {
    pub fn new() -> InventoryItemData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InventoryItemData {
        static mut instance: ::protobuf::lazy::Lazy<InventoryItemData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InventoryItemData,
        };
        unsafe {
            instance.get(|| {
                InventoryItemData {
                    pokemon_data: ::protobuf::SingularPtrField::none(),
                    item: ::protobuf::SingularPtrField::none(),
                    pokedex_entry: ::protobuf::SingularPtrField::none(),
                    player_stats: ::protobuf::SingularPtrField::none(),
                    player_currency: ::protobuf::SingularPtrField::none(),
                    player_camera: ::protobuf::SingularPtrField::none(),
                    inventory_upgrades: ::protobuf::SingularPtrField::none(),
                    applied_items: ::protobuf::SingularPtrField::none(),
                    egg_incubators: ::protobuf::SingularPtrField::none(),
                    candy: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Data.PokemonData pokemon_data = 1;

    pub fn clear_pokemon_data(&mut self) {
        self.pokemon_data.clear();
    }

    pub fn has_pokemon_data(&self) -> bool {
        self.pokemon_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_data(&mut self, v: super::POGOProtos_Data::PokemonData) {
        self.pokemon_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pokemon_data(&mut self) -> &mut super::POGOProtos_Data::PokemonData {
        if self.pokemon_data.is_none() {
            self.pokemon_data.set_default();
        };
        self.pokemon_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_pokemon_data(&mut self) -> super::POGOProtos_Data::PokemonData {
        self.pokemon_data.take().unwrap_or_else(|| super::POGOProtos_Data::PokemonData::new())
    }

    pub fn get_pokemon_data(&self) -> &super::POGOProtos_Data::PokemonData {
        self.pokemon_data.as_ref().unwrap_or_else(|| super::POGOProtos_Data::PokemonData::default_instance())
    }

    // optional .POGOProtos.Inventory.Item.ItemData item = 2;

    pub fn clear_item(&mut self) {
        self.item.clear();
    }

    pub fn has_item(&self) -> bool {
        self.item.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item(&mut self, v: super::POGOProtos_Inventory_Item::ItemData) {
        self.item = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_item(&mut self) -> &mut super::POGOProtos_Inventory_Item::ItemData {
        if self.item.is_none() {
            self.item.set_default();
        };
        self.item.as_mut().unwrap()
    }

    // Take field
    pub fn take_item(&mut self) -> super::POGOProtos_Inventory_Item::ItemData {
        self.item.take().unwrap_or_else(|| super::POGOProtos_Inventory_Item::ItemData::new())
    }

    pub fn get_item(&self) -> &super::POGOProtos_Inventory_Item::ItemData {
        self.item.as_ref().unwrap_or_else(|| super::POGOProtos_Inventory_Item::ItemData::default_instance())
    }

    // optional .POGOProtos.Data.PokedexEntry pokedex_entry = 3;

    pub fn clear_pokedex_entry(&mut self) {
        self.pokedex_entry.clear();
    }

    pub fn has_pokedex_entry(&self) -> bool {
        self.pokedex_entry.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokedex_entry(&mut self, v: super::POGOProtos_Data::PokedexEntry) {
        self.pokedex_entry = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pokedex_entry(&mut self) -> &mut super::POGOProtos_Data::PokedexEntry {
        if self.pokedex_entry.is_none() {
            self.pokedex_entry.set_default();
        };
        self.pokedex_entry.as_mut().unwrap()
    }

    // Take field
    pub fn take_pokedex_entry(&mut self) -> super::POGOProtos_Data::PokedexEntry {
        self.pokedex_entry.take().unwrap_or_else(|| super::POGOProtos_Data::PokedexEntry::new())
    }

    pub fn get_pokedex_entry(&self) -> &super::POGOProtos_Data::PokedexEntry {
        self.pokedex_entry.as_ref().unwrap_or_else(|| super::POGOProtos_Data::PokedexEntry::default_instance())
    }

    // optional .POGOProtos.Data.Player.PlayerStats player_stats = 4;

    pub fn clear_player_stats(&mut self) {
        self.player_stats.clear();
    }

    pub fn has_player_stats(&self) -> bool {
        self.player_stats.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_stats(&mut self, v: super::POGOProtos_Data_Player::PlayerStats) {
        self.player_stats = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_stats(&mut self) -> &mut super::POGOProtos_Data_Player::PlayerStats {
        if self.player_stats.is_none() {
            self.player_stats.set_default();
        };
        self.player_stats.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_stats(&mut self) -> super::POGOProtos_Data_Player::PlayerStats {
        self.player_stats.take().unwrap_or_else(|| super::POGOProtos_Data_Player::PlayerStats::new())
    }

    pub fn get_player_stats(&self) -> &super::POGOProtos_Data_Player::PlayerStats {
        self.player_stats.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Player::PlayerStats::default_instance())
    }

    // optional .POGOProtos.Data.Player.PlayerCurrency player_currency = 5;

    pub fn clear_player_currency(&mut self) {
        self.player_currency.clear();
    }

    pub fn has_player_currency(&self) -> bool {
        self.player_currency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_currency(&mut self, v: super::POGOProtos_Data_Player::PlayerCurrency) {
        self.player_currency = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_currency(&mut self) -> &mut super::POGOProtos_Data_Player::PlayerCurrency {
        if self.player_currency.is_none() {
            self.player_currency.set_default();
        };
        self.player_currency.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_currency(&mut self) -> super::POGOProtos_Data_Player::PlayerCurrency {
        self.player_currency.take().unwrap_or_else(|| super::POGOProtos_Data_Player::PlayerCurrency::new())
    }

    pub fn get_player_currency(&self) -> &super::POGOProtos_Data_Player::PlayerCurrency {
        self.player_currency.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Player::PlayerCurrency::default_instance())
    }

    // optional .POGOProtos.Data.Player.PlayerCamera player_camera = 6;

    pub fn clear_player_camera(&mut self) {
        self.player_camera.clear();
    }

    pub fn has_player_camera(&self) -> bool {
        self.player_camera.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_camera(&mut self, v: super::POGOProtos_Data_Player::PlayerCamera) {
        self.player_camera = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_camera(&mut self) -> &mut super::POGOProtos_Data_Player::PlayerCamera {
        if self.player_camera.is_none() {
            self.player_camera.set_default();
        };
        self.player_camera.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_camera(&mut self) -> super::POGOProtos_Data_Player::PlayerCamera {
        self.player_camera.take().unwrap_or_else(|| super::POGOProtos_Data_Player::PlayerCamera::new())
    }

    pub fn get_player_camera(&self) -> &super::POGOProtos_Data_Player::PlayerCamera {
        self.player_camera.as_ref().unwrap_or_else(|| super::POGOProtos_Data_Player::PlayerCamera::default_instance())
    }

    // optional .POGOProtos.Inventory.InventoryUpgrades inventory_upgrades = 7;

    pub fn clear_inventory_upgrades(&mut self) {
        self.inventory_upgrades.clear();
    }

    pub fn has_inventory_upgrades(&self) -> bool {
        self.inventory_upgrades.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inventory_upgrades(&mut self, v: InventoryUpgrades) {
        self.inventory_upgrades = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inventory_upgrades(&mut self) -> &mut InventoryUpgrades {
        if self.inventory_upgrades.is_none() {
            self.inventory_upgrades.set_default();
        };
        self.inventory_upgrades.as_mut().unwrap()
    }

    // Take field
    pub fn take_inventory_upgrades(&mut self) -> InventoryUpgrades {
        self.inventory_upgrades.take().unwrap_or_else(|| InventoryUpgrades::new())
    }

    pub fn get_inventory_upgrades(&self) -> &InventoryUpgrades {
        self.inventory_upgrades.as_ref().unwrap_or_else(|| InventoryUpgrades::default_instance())
    }

    // optional .POGOProtos.Inventory.AppliedItems applied_items = 8;

    pub fn clear_applied_items(&mut self) {
        self.applied_items.clear();
    }

    pub fn has_applied_items(&self) -> bool {
        self.applied_items.is_some()
    }

    // Param is passed by value, moved
    pub fn set_applied_items(&mut self, v: AppliedItems) {
        self.applied_items = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_applied_items(&mut self) -> &mut AppliedItems {
        if self.applied_items.is_none() {
            self.applied_items.set_default();
        };
        self.applied_items.as_mut().unwrap()
    }

    // Take field
    pub fn take_applied_items(&mut self) -> AppliedItems {
        self.applied_items.take().unwrap_or_else(|| AppliedItems::new())
    }

    pub fn get_applied_items(&self) -> &AppliedItems {
        self.applied_items.as_ref().unwrap_or_else(|| AppliedItems::default_instance())
    }

    // optional .POGOProtos.Inventory.EggIncubators egg_incubators = 9;

    pub fn clear_egg_incubators(&mut self) {
        self.egg_incubators.clear();
    }

    pub fn has_egg_incubators(&self) -> bool {
        self.egg_incubators.is_some()
    }

    // Param is passed by value, moved
    pub fn set_egg_incubators(&mut self, v: EggIncubators) {
        self.egg_incubators = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_egg_incubators(&mut self) -> &mut EggIncubators {
        if self.egg_incubators.is_none() {
            self.egg_incubators.set_default();
        };
        self.egg_incubators.as_mut().unwrap()
    }

    // Take field
    pub fn take_egg_incubators(&mut self) -> EggIncubators {
        self.egg_incubators.take().unwrap_or_else(|| EggIncubators::new())
    }

    pub fn get_egg_incubators(&self) -> &EggIncubators {
        self.egg_incubators.as_ref().unwrap_or_else(|| EggIncubators::default_instance())
    }

    // optional .POGOProtos.Inventory.Candy candy = 10;

    pub fn clear_candy(&mut self) {
        self.candy.clear();
    }

    pub fn has_candy(&self) -> bool {
        self.candy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_candy(&mut self, v: Candy) {
        self.candy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_candy(&mut self) -> &mut Candy {
        if self.candy.is_none() {
            self.candy.set_default();
        };
        self.candy.as_mut().unwrap()
    }

    // Take field
    pub fn take_candy(&mut self) -> Candy {
        self.candy.take().unwrap_or_else(|| Candy::new())
    }

    pub fn get_candy(&self) -> &Candy {
        self.candy.as_ref().unwrap_or_else(|| Candy::default_instance())
    }
}

impl ::protobuf::Message for InventoryItemData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pokemon_data));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.item));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pokedex_entry));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_stats));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_currency));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_camera));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.inventory_upgrades));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.applied_items));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.egg_incubators));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.candy));
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
        for value in &self.pokemon_data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.item {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.pokedex_entry {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.player_stats {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.player_currency {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.player_camera {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.inventory_upgrades {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.applied_items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.egg_incubators {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.candy {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pokemon_data.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.item.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pokedex_entry.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.player_stats.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.player_currency.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.player_camera.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.inventory_upgrades.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.applied_items.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.egg_incubators.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.candy.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<InventoryItemData>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InventoryItemData {
    fn new() -> InventoryItemData {
        InventoryItemData::new()
    }

    fn descriptor_static(_: ::std::option::Option<InventoryItemData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pokemon_data",
                    InventoryItemData::has_pokemon_data,
                    InventoryItemData::get_pokemon_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "item",
                    InventoryItemData::has_item,
                    InventoryItemData::get_item,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pokedex_entry",
                    InventoryItemData::has_pokedex_entry,
                    InventoryItemData::get_pokedex_entry,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "player_stats",
                    InventoryItemData::has_player_stats,
                    InventoryItemData::get_player_stats,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "player_currency",
                    InventoryItemData::has_player_currency,
                    InventoryItemData::get_player_currency,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "player_camera",
                    InventoryItemData::has_player_camera,
                    InventoryItemData::get_player_camera,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "inventory_upgrades",
                    InventoryItemData::has_inventory_upgrades,
                    InventoryItemData::get_inventory_upgrades,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "applied_items",
                    InventoryItemData::has_applied_items,
                    InventoryItemData::get_applied_items,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "egg_incubators",
                    InventoryItemData::has_egg_incubators,
                    InventoryItemData::get_egg_incubators,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "candy",
                    InventoryItemData::has_candy,
                    InventoryItemData::get_candy,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InventoryItemData>(
                    "InventoryItemData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InventoryItemData {
    fn clear(&mut self) {
        self.clear_pokemon_data();
        self.clear_item();
        self.clear_pokedex_entry();
        self.clear_player_stats();
        self.clear_player_currency();
        self.clear_player_camera();
        self.clear_inventory_upgrades();
        self.clear_applied_items();
        self.clear_egg_incubators();
        self.clear_candy();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InventoryItemData {
    fn eq(&self, other: &InventoryItemData) -> bool {
        self.pokemon_data == other.pokemon_data &&
        self.item == other.item &&
        self.pokedex_entry == other.pokedex_entry &&
        self.player_stats == other.player_stats &&
        self.player_currency == other.player_currency &&
        self.player_camera == other.player_camera &&
        self.inventory_upgrades == other.inventory_upgrades &&
        self.applied_items == other.applied_items &&
        self.egg_incubators == other.egg_incubators &&
        self.candy == other.candy &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InventoryItemData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InventoryDelta {
    // message fields
    original_timestamp_ms: ::std::option::Option<i64>,
    new_timestamp_ms: ::std::option::Option<i64>,
    inventory_items: ::protobuf::RepeatedField<InventoryItem>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InventoryDelta {}

impl InventoryDelta {
    pub fn new() -> InventoryDelta {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InventoryDelta {
        static mut instance: ::protobuf::lazy::Lazy<InventoryDelta> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InventoryDelta,
        };
        unsafe {
            instance.get(|| {
                InventoryDelta {
                    original_timestamp_ms: ::std::option::Option::None,
                    new_timestamp_ms: ::std::option::Option::None,
                    inventory_items: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 original_timestamp_ms = 1;

    pub fn clear_original_timestamp_ms(&mut self) {
        self.original_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_original_timestamp_ms(&self) -> bool {
        self.original_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_original_timestamp_ms(&mut self, v: i64) {
        self.original_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_original_timestamp_ms(&self) -> i64 {
        self.original_timestamp_ms.unwrap_or(0)
    }

    // optional int64 new_timestamp_ms = 2;

    pub fn clear_new_timestamp_ms(&mut self) {
        self.new_timestamp_ms = ::std::option::Option::None;
    }

    pub fn has_new_timestamp_ms(&self) -> bool {
        self.new_timestamp_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_timestamp_ms(&mut self, v: i64) {
        self.new_timestamp_ms = ::std::option::Option::Some(v);
    }

    pub fn get_new_timestamp_ms(&self) -> i64 {
        self.new_timestamp_ms.unwrap_or(0)
    }

    // repeated .POGOProtos.Inventory.InventoryItem inventory_items = 3;

    pub fn clear_inventory_items(&mut self) {
        self.inventory_items.clear();
    }

    // Param is passed by value, moved
    pub fn set_inventory_items(&mut self, v: ::protobuf::RepeatedField<InventoryItem>) {
        self.inventory_items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_inventory_items(&mut self) -> &mut ::protobuf::RepeatedField<InventoryItem> {
        &mut self.inventory_items
    }

    // Take field
    pub fn take_inventory_items(&mut self) -> ::protobuf::RepeatedField<InventoryItem> {
        ::std::mem::replace(&mut self.inventory_items, ::protobuf::RepeatedField::new())
    }

    pub fn get_inventory_items(&self) -> &[InventoryItem] {
        &self.inventory_items
    }
}

impl ::protobuf::Message for InventoryDelta {
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
                    self.original_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.new_timestamp_ms = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.inventory_items));
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
        for value in &self.original_timestamp_ms {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.new_timestamp_ms {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.inventory_items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.original_timestamp_ms {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.new_timestamp_ms {
            try!(os.write_int64(2, v));
        };
        for v in &self.inventory_items {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<InventoryDelta>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InventoryDelta {
    fn new() -> InventoryDelta {
        InventoryDelta::new()
    }

    fn descriptor_static(_: ::std::option::Option<InventoryDelta>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "original_timestamp_ms",
                    InventoryDelta::has_original_timestamp_ms,
                    InventoryDelta::get_original_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "new_timestamp_ms",
                    InventoryDelta::has_new_timestamp_ms,
                    InventoryDelta::get_new_timestamp_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "inventory_items",
                    InventoryDelta::get_inventory_items,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InventoryDelta>(
                    "InventoryDelta",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InventoryDelta {
    fn clear(&mut self) {
        self.clear_original_timestamp_ms();
        self.clear_new_timestamp_ms();
        self.clear_inventory_items();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InventoryDelta {
    fn eq(&self, other: &InventoryDelta) -> bool {
        self.original_timestamp_ms == other.original_timestamp_ms &&
        self.new_timestamp_ms == other.new_timestamp_ms &&
        self.inventory_items == other.inventory_items &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InventoryDelta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EggIncubatorType {
    INCUBATOR_UNSET = 0,
    INCUBATOR_DISTANCE = 1,
}

impl ::protobuf::ProtobufEnum for EggIncubatorType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EggIncubatorType> {
        match value {
            0 => ::std::option::Option::Some(EggIncubatorType::INCUBATOR_UNSET),
            1 => ::std::option::Option::Some(EggIncubatorType::INCUBATOR_DISTANCE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EggIncubatorType] = &[
            EggIncubatorType::INCUBATOR_UNSET,
            EggIncubatorType::INCUBATOR_DISTANCE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EggIncubatorType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EggIncubatorType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EggIncubatorType {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum InventoryUpgradeType {
    UPGRADE_UNSET = 0,
    INCREASE_ITEM_STORAGE = 1,
    INCREASE_POKEMON_STORAGE = 2,
}

impl ::protobuf::ProtobufEnum for InventoryUpgradeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<InventoryUpgradeType> {
        match value {
            0 => ::std::option::Option::Some(InventoryUpgradeType::UPGRADE_UNSET),
            1 => ::std::option::Option::Some(InventoryUpgradeType::INCREASE_ITEM_STORAGE),
            2 => ::std::option::Option::Some(InventoryUpgradeType::INCREASE_POKEMON_STORAGE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [InventoryUpgradeType] = &[
            InventoryUpgradeType::UPGRADE_UNSET,
            InventoryUpgradeType::INCREASE_ITEM_STORAGE,
            InventoryUpgradeType::INCREASE_POKEMON_STORAGE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<InventoryUpgradeType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("InventoryUpgradeType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for InventoryUpgradeType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1a, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76,
    0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x14, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f,
    0x72, 0x79, 0x1a, 0x1f, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49,
    0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x16, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x15, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x1a, 0x1c, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44,
    0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x50, 0x00, 0x50, 0x01, 0x50, 0x02, 0x50, 0x03, 0x22, 0xcc, 0x01, 0x0a, 0x10, 0x49, 0x6e, 0x76,
    0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x12, 0x3a, 0x0a,
    0x07, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x21,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65,
    0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x49,
    0x64, 0x52, 0x06, 0x69, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x12, 0x4d, 0x0a, 0x0c, 0x75, 0x70, 0x67,
    0x72, 0x61, 0x64, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x2a, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76,
    0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79,
    0x55, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0b, 0x75, 0x70, 0x67,
    0x72, 0x61, 0x64, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x2d, 0x0a, 0x12, 0x61, 0x64, 0x64, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x11, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c,
    0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x22, 0x9e, 0x02, 0x0a, 0x0d, 0x49, 0x6e, 0x76, 0x65,
    0x6e, 0x74, 0x6f, 0x72, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x12, 0x32, 0x0a, 0x15, 0x6d, 0x6f, 0x64,
    0x69, 0x66, 0x69, 0x65, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f,
    0x6d, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x13, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69,
    0x65, 0x64, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12, 0x52, 0x0a,
    0x0c, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x5f, 0x69, 0x74, 0x65, 0x6d, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x2f, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e,
    0x74, 0x6f, 0x72, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64,
    0x49, 0x74, 0x65, 0x6d, 0x52, 0x0b, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x49, 0x74, 0x65,
    0x6d, 0x12, 0x57, 0x0a, 0x13, 0x69, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x5f, 0x69,
    0x74, 0x65, 0x6d, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x27,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65,
    0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x49,
    0x74, 0x65, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x52, 0x11, 0x69, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f,
    0x72, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x1a, 0x2c, 0x0a, 0x0b, 0x44, 0x65,
    0x6c, 0x65, 0x74, 0x65, 0x64, 0x49, 0x74, 0x65, 0x6d, 0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09, 0x70,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x22, 0xc1, 0x02, 0x0a, 0x0c, 0x45, 0x67, 0x67,
    0x49, 0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x3a, 0x0a, 0x07, 0x69, 0x74, 0x65,
    0x6d, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72,
    0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x52, 0x06, 0x69,
    0x74, 0x65, 0x6d, 0x49, 0x64, 0x12, 0x4d, 0x0a, 0x0e, 0x69, 0x6e, 0x63, 0x75, 0x62, 0x61, 0x74,
    0x6f, 0x72, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x26, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e,
    0x74, 0x6f, 0x72, 0x79, 0x2e, 0x45, 0x67, 0x67, 0x49, 0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f,
    0x72, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0d, 0x69, 0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x25, 0x0a, 0x0e, 0x75, 0x73, 0x65, 0x73, 0x5f, 0x72, 0x65, 0x6d,
    0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x75, 0x73,
    0x65, 0x73, 0x52, 0x65, 0x6d, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x12, 0x1d, 0x0a, 0x0a, 0x70,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x09, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x26, 0x0a, 0x0f, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x5f, 0x6b, 0x6d, 0x5f, 0x77, 0x61, 0x6c, 0x6b, 0x65, 0x64, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x01, 0x52, 0x0d, 0x73, 0x74, 0x61, 0x72, 0x74, 0x4b, 0x6d, 0x57, 0x61, 0x6c, 0x6b,
    0x65, 0x64, 0x12, 0x28, 0x0a, 0x10, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x6b, 0x6d, 0x5f,
    0x77, 0x61, 0x6c, 0x6b, 0x65, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0e, 0x74, 0x61,
    0x72, 0x67, 0x65, 0x74, 0x4b, 0x6d, 0x57, 0x61, 0x6c, 0x6b, 0x65, 0x64, 0x22, 0x45, 0x0a, 0x0c,
    0x41, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x49, 0x74, 0x65, 0x6d, 0x73, 0x12, 0x35, 0x0a, 0x04,
    0x69, 0x74, 0x65, 0x6d, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72,
    0x79, 0x2e, 0x41, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x49, 0x74, 0x65, 0x6d, 0x52, 0x04, 0x69,
    0x74, 0x65, 0x6d, 0x22, 0x58, 0x0a, 0x0d, 0x45, 0x67, 0x67, 0x49, 0x6e, 0x63, 0x75, 0x62, 0x61,
    0x74, 0x6f, 0x72, 0x73, 0x12, 0x47, 0x0a, 0x0d, 0x65, 0x67, 0x67, 0x5f, 0x69, 0x6e, 0x63, 0x75,
    0x62, 0x61, 0x74, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f,
    0x72, 0x79, 0x2e, 0x45, 0x67, 0x67, 0x49, 0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x52,
    0x0c, 0x65, 0x67, 0x67, 0x49, 0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x22, 0xc7, 0x01,
    0x0a, 0x0b, 0x41, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x49, 0x74, 0x65, 0x6d, 0x12, 0x3a, 0x0a,
    0x07, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x21,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65,
    0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x49,
    0x64, 0x52, 0x06, 0x69, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x12, 0x40, 0x0a, 0x09, 0x69, 0x74, 0x65,
    0x6d, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x23, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74,
    0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x54, 0x79, 0x70,
    0x65, 0x52, 0x08, 0x69, 0x74, 0x65, 0x6d, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x65,
    0x78, 0x70, 0x69, 0x72, 0x65, 0x5f, 0x6d, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x52, 0x08,
    0x65, 0x78, 0x70, 0x69, 0x72, 0x65, 0x4d, 0x73, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x70, 0x70, 0x6c,
    0x69, 0x65, 0x64, 0x5f, 0x6d, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x52, 0x09, 0x61, 0x70,
    0x70, 0x6c, 0x69, 0x65, 0x64, 0x4d, 0x73, 0x22, 0x6a, 0x0a, 0x11, 0x49, 0x6e, 0x76, 0x65, 0x6e,
    0x74, 0x6f, 0x72, 0x79, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x73, 0x12, 0x55, 0x0a, 0x12,
    0x69, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x5f, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64,
    0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e,
    0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65,
    0x52, 0x11, 0x69, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x55, 0x70, 0x67, 0x72, 0x61,
    0x64, 0x65, 0x73, 0x22, 0x5d, 0x0a, 0x05, 0x43, 0x61, 0x6e, 0x64, 0x79, 0x12, 0x3e, 0x0a, 0x09,
    0x66, 0x61, 0x6d, 0x69, 0x6c, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75,
    0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x46, 0x61, 0x6d, 0x69, 0x6c, 0x79,
    0x49, 0x64, 0x52, 0x08, 0x66, 0x61, 0x6d, 0x69, 0x6c, 0x79, 0x49, 0x64, 0x12, 0x14, 0x0a, 0x05,
    0x63, 0x61, 0x6e, 0x64, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x05, 0x63, 0x61, 0x6e,
    0x64, 0x79, 0x22, 0xd5, 0x05, 0x0a, 0x11, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79,
    0x49, 0x74, 0x65, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x12, 0x3f, 0x0a, 0x0c, 0x70, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61,
    0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x52, 0x0b, 0x70, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x12, 0x37, 0x0a, 0x04, 0x69, 0x74, 0x65,
    0x6d, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49,
    0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x52, 0x04, 0x69, 0x74,
    0x65, 0x6d, 0x12, 0x42, 0x0a, 0x0d, 0x70, 0x6f, 0x6b, 0x65, 0x64, 0x65, 0x78, 0x5f, 0x65, 0x6e,
    0x74, 0x72, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6f, 0x6b, 0x65,
    0x64, 0x65, 0x78, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x0c, 0x70, 0x6f, 0x6b, 0x65, 0x64, 0x65,
    0x78, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x46, 0x0a, 0x0c, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72,
    0x5f, 0x73, 0x74, 0x61, 0x74, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74,
    0x73, 0x52, 0x0b, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x73, 0x12, 0x4f,
    0x0a, 0x0f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63,
    0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72,
    0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x52,
    0x0e, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x12,
    0x49, 0x0a, 0x0d, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x2e,
    0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x52, 0x0c, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x12, 0x56, 0x0a, 0x12, 0x69, 0x6e,
    0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x5f, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x73,
    0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x6e,
    0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x73, 0x52,
    0x11, 0x69, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64,
    0x65, 0x73, 0x12, 0x47, 0x0a, 0x0d, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x5f, 0x69, 0x74,
    0x65, 0x6d, 0x73, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79,
    0x2e, 0x41, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x49, 0x74, 0x65, 0x6d, 0x73, 0x52, 0x0c, 0x61,
    0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x49, 0x74, 0x65, 0x6d, 0x73, 0x12, 0x4a, 0x0a, 0x0e, 0x65,
    0x67, 0x67, 0x5f, 0x69, 0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x18, 0x09, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x45, 0x67, 0x67, 0x49, 0x6e,
    0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x52, 0x0d, 0x65, 0x67, 0x67, 0x49, 0x6e, 0x63,
    0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x12, 0x31, 0x0a, 0x05, 0x63, 0x61, 0x6e, 0x64, 0x79,
    0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x43, 0x61,
    0x6e, 0x64, 0x79, 0x52, 0x05, 0x63, 0x61, 0x6e, 0x64, 0x79, 0x22, 0xbc, 0x01, 0x0a, 0x0e, 0x49,
    0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x12, 0x32, 0x0a,
    0x15, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x13, 0x6f, 0x72,
    0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d,
    0x73, 0x12, 0x28, 0x0a, 0x10, 0x6e, 0x65, 0x77, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
    0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0e, 0x6e, 0x65, 0x77,
    0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x73, 0x12, 0x4c, 0x0a, 0x0f, 0x69,
    0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x5f, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x03,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x6e, 0x76, 0x65,
    0x6e, 0x74, 0x6f, 0x72, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x52, 0x0e, 0x69, 0x6e, 0x76, 0x65, 0x6e,
    0x74, 0x6f, 0x72, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x73, 0x2a, 0x3f, 0x0a, 0x10, 0x45, 0x67, 0x67,
    0x49, 0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x54, 0x79, 0x70, 0x65, 0x12, 0x13, 0x0a,
    0x0f, 0x49, 0x4e, 0x43, 0x55, 0x42, 0x41, 0x54, 0x4f, 0x52, 0x5f, 0x55, 0x4e, 0x53, 0x45, 0x54,
    0x10, 0x00, 0x12, 0x16, 0x0a, 0x12, 0x49, 0x4e, 0x43, 0x55, 0x42, 0x41, 0x54, 0x4f, 0x52, 0x5f,
    0x44, 0x49, 0x53, 0x54, 0x41, 0x4e, 0x43, 0x45, 0x10, 0x01, 0x2a, 0x62, 0x0a, 0x14, 0x49, 0x6e,
    0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x11, 0x0a, 0x0d, 0x55, 0x50, 0x47, 0x52, 0x41, 0x44, 0x45, 0x5f, 0x55, 0x4e,
    0x53, 0x45, 0x54, 0x10, 0x00, 0x12, 0x19, 0x0a, 0x15, 0x49, 0x4e, 0x43, 0x52, 0x45, 0x41, 0x53,
    0x45, 0x5f, 0x49, 0x54, 0x45, 0x4d, 0x5f, 0x53, 0x54, 0x4f, 0x52, 0x41, 0x47, 0x45, 0x10, 0x01,
    0x12, 0x1c, 0x0a, 0x18, 0x49, 0x4e, 0x43, 0x52, 0x45, 0x41, 0x53, 0x45, 0x5f, 0x50, 0x4f, 0x4b,
    0x45, 0x4d, 0x4f, 0x4e, 0x5f, 0x53, 0x54, 0x4f, 0x52, 0x41, 0x47, 0x45, 0x10, 0x02, 0x4a, 0xfc,
    0x18, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x4b, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x1c, 0x0a, 0x09, 0x0a,
    0x02, 0x0a, 0x00, 0x12, 0x03, 0x03, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x03, 0x0e, 0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x04, 0x07, 0x0d, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x0e, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x02, 0x12,
    0x03, 0x05, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x05, 0x0e, 0x25, 0x0a,
    0x09, 0x0a, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x06, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x06, 0x0e, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x0c,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x18, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x09, 0x08, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x09, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x09, 0x2a, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x09, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x44,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0a, 0x08, 0x09, 0x36, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0a, 0x08, 0x32, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x33, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x42, 0x43, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x0b, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x0b, 0x08, 0x0a, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x0b, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x0e,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x23, 0x24, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0d, 0x00, 0x15, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x0e, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0e,
    0x08, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0e, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x0e, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x26, 0x27, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x0f, 0x08, 0x0e, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x0f, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0f, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0f, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x10, 0x08, 0x48,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0x10, 0x08, 0x0f, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x10, 0x08, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x30, 0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x10, 0x46, 0x47, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x03,
    0x00, 0x12, 0x04, 0x12, 0x08, 0x14, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x01,
    0x12, 0x03, 0x12, 0x10, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x13, 0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x13, 0x10, 0x12, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x13, 0x10, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x13, 0x18, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x13, 0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x16, 0x00, 0x1e,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x16, 0x08, 0x14, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x17, 0x08, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x17, 0x08, 0x16, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x17, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x17, 0x0f, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x17, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x18, 0x08, 0x36,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x18, 0x08, 0x17, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x18, 0x08, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x18, 0x2a, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x18, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x19, 0x08, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x19, 0x08, 0x18, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x19, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x19, 0x2f,
    0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x19, 0x40, 0x41, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1a, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x04, 0x1a, 0x08, 0x19, 0x42, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x1a, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x1a, 0x0e, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x1a, 0x1f, 0x20, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1b,
    0x08, 0x1e, 0x22, 0x1f, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x3a, 0x20, 0x43, 0x68, 0x65, 0x63, 0x6b,
    0x20, 0x69, 0x66, 0x20, 0x69, 0x73, 0x20, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x54, 0x79,
    0x70, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1b, 0x08,
    0x1a, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x1b, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1b, 0x0f, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1b, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1c, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x05, 0x04, 0x12, 0x04, 0x1c, 0x08, 0x1b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x1c, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x1c, 0x0f, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1c,
    0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1d, 0x08, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x04, 0x12, 0x04, 0x1d, 0x08, 0x1c, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x05, 0x12, 0x03, 0x1d, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1d, 0x0f, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x06, 0x03, 0x12, 0x03, 0x1d, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x1f, 0x00, 0x21, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x14,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x20, 0x08, 0x3c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x20, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x20, 0x11, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x20, 0x33, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x20, 0x3a, 0x3b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x22, 0x00, 0x24,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x22, 0x08, 0x15, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x23, 0x08, 0x46, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x23, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x23, 0x11, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x23, 0x34, 0x41, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x23,
    0x44, 0x45, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x25, 0x00, 0x2a, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x25, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x00, 0x12, 0x03, 0x26, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x26, 0x08, 0x25, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x26, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x26,
    0x2a, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x26, 0x34, 0x35,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x27, 0x08, 0x3a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x04, 0x27, 0x08, 0x26, 0x36, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x27, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x27, 0x2c, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x27, 0x38, 0x39, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03,
    0x28, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x04, 0x28, 0x08,
    0x27, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x28, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x28, 0x0e, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x28, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x29, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x03, 0x04, 0x12, 0x04, 0x29, 0x08, 0x28, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x29, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x29, 0x0e, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x29,
    0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x2b, 0x00, 0x2e, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x05, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x2c, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x2c, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x2c, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x08, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2d, 0x08, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x2d, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x06, 0x12, 0x04, 0x2f, 0x00, 0x31, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12,
    0x03, 0x2f, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x30, 0x08,
    0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x30, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x30, 0x11, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x30, 0x22, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x30, 0x37, 0x38, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12,
    0x04, 0x32, 0x00, 0x35, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x32, 0x08,
    0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x33, 0x08, 0x38, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x33, 0x08, 0x32, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x33, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x33, 0x2a, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x33, 0x36, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12,
    0x03, 0x34, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04, 0x34,
    0x08, 0x33, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x34, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x34, 0x0e, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x34, 0x16, 0x17, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x08, 0x12, 0x04, 0x36, 0x00, 0x41, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01,
    0x12, 0x03, 0x36, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x37,
    0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0x37, 0x08, 0x36,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x37, 0x08, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x37, 0x25, 0x31, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x37, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x01, 0x12, 0x03, 0x38, 0x08, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x38, 0x08, 0x37, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x38, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x38, 0x2c, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x38, 0x33,
    0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x39, 0x08, 0x38, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x04, 0x39, 0x08, 0x38, 0x35, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x02, 0x06, 0x12, 0x03, 0x39, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x39, 0x26, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x39, 0x36, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12,
    0x03, 0x3a, 0x08, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x04, 0x3a,
    0x08, 0x39, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x06, 0x12, 0x03, 0x3a, 0x08,
    0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3a, 0x2c, 0x38, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x03, 0x12, 0x03, 0x3a, 0x3b, 0x3c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x04, 0x12, 0x03, 0x3b, 0x08, 0x43, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x3b, 0x08, 0x3a, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x04, 0x06, 0x12, 0x03, 0x3b, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x3b, 0x2f, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x3b, 0x41, 0x42, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x05, 0x12, 0x03, 0x3c, 0x08, 0x3f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x04, 0x12, 0x04, 0x3c, 0x08, 0x3b, 0x43, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x06, 0x12, 0x03, 0x3c, 0x08, 0x2c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x05, 0x01, 0x12, 0x03, 0x3c, 0x2d, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x05, 0x03, 0x12, 0x03, 0x3c, 0x3d, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02,
    0x06, 0x12, 0x03, 0x3d, 0x08, 0x47, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x04, 0x12,
    0x04, 0x3d, 0x08, 0x3c, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x06, 0x12, 0x03,
    0x3d, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x01, 0x12, 0x03, 0x3d, 0x30,
    0x42, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x03, 0x12, 0x03, 0x3d, 0x45, 0x46, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x07, 0x12, 0x03, 0x3e, 0x08, 0x3d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x07, 0x04, 0x12, 0x04, 0x3e, 0x08, 0x3d, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x07, 0x06, 0x12, 0x03, 0x3e, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x07, 0x01, 0x12, 0x03, 0x3e, 0x2b, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x07, 0x03,
    0x12, 0x03, 0x3e, 0x3b, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x08, 0x12, 0x03, 0x3f,
    0x08, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x08, 0x04, 0x12, 0x04, 0x3f, 0x08, 0x3e,
    0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x08, 0x06, 0x12, 0x03, 0x3f, 0x08, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x08, 0x01, 0x12, 0x03, 0x3f, 0x2c, 0x3a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x08, 0x03, 0x12, 0x03, 0x3f, 0x3d, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x09, 0x12, 0x03, 0x40, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x09,
    0x04, 0x12, 0x04, 0x40, 0x08, 0x3f, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x09, 0x06,
    0x12, 0x03, 0x40, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x09, 0x01, 0x12, 0x03,
    0x40, 0x24, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x09, 0x03, 0x12, 0x03, 0x40, 0x2c,
    0x2e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x42, 0x00, 0x46, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x42, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x00, 0x12, 0x03, 0x43, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x43, 0x08, 0x42, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x43, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x0e,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x43, 0x26, 0x27, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x44, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x04, 0x44, 0x08, 0x43, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x44, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x44, 0x0e, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x44, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x45,
    0x08, 0x49, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x03, 0x45, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x06, 0x12, 0x03, 0x45, 0x11, 0x34, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x03, 0x45, 0x35, 0x44, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x03, 0x45, 0x47, 0x48, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01,
    0x12, 0x04, 0x47, 0x00, 0x4b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x47,
    0x05, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x48, 0x08, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x48, 0x08, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x48, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x01, 0x02, 0x01, 0x12, 0x03, 0x49, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x49, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x49, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x4a, 0x08,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4a, 0x08, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x4a, 0x23, 0x24, 0x62, 0x06, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x33,
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
