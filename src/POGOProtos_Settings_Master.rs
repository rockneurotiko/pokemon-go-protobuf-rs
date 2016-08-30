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
pub struct TypeEffectiveSettings {
    // message fields
    attack_scalar: ::std::vec::Vec<f32>,
    attack_type: ::std::option::Option<super::POGOProtos_Enums::PokemonType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TypeEffectiveSettings {}

impl TypeEffectiveSettings {
    pub fn new() -> TypeEffectiveSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TypeEffectiveSettings {
        static mut instance: ::protobuf::lazy::Lazy<TypeEffectiveSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TypeEffectiveSettings,
        };
        unsafe {
            instance.get(|| {
                TypeEffectiveSettings {
                    attack_scalar: ::std::vec::Vec::new(),
                    attack_type: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated float attack_scalar = 1;

    pub fn clear_attack_scalar(&mut self) {
        self.attack_scalar.clear();
    }

    // Param is passed by value, moved
    pub fn set_attack_scalar(&mut self, v: ::std::vec::Vec<f32>) {
        self.attack_scalar = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attack_scalar(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.attack_scalar
    }

    // Take field
    pub fn take_attack_scalar(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.attack_scalar, ::std::vec::Vec::new())
    }

    pub fn get_attack_scalar(&self) -> &[f32] {
        &self.attack_scalar
    }

    // optional .POGOProtos.Enums.PokemonType attack_type = 2;

    pub fn clear_attack_type(&mut self) {
        self.attack_type = ::std::option::Option::None;
    }

    pub fn has_attack_type(&self) -> bool {
        self.attack_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attack_type(&mut self, v: super::POGOProtos_Enums::PokemonType) {
        self.attack_type = ::std::option::Option::Some(v);
    }

    pub fn get_attack_type(&self) -> super::POGOProtos_Enums::PokemonType {
        self.attack_type.unwrap_or(super::POGOProtos_Enums::PokemonType::POKEMON_TYPE_NONE)
    }
}

impl ::protobuf::Message for TypeEffectiveSettings {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.attack_scalar));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.attack_type = ::std::option::Option::Some(tmp);
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
        my_size += 5 * self.attack_scalar.len() as u32;
        for value in &self.attack_type {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.attack_scalar {
            try!(os.write_float(1, *v));
        };
        if let Some(v) = self.attack_type {
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
        ::std::any::TypeId::of::<TypeEffectiveSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TypeEffectiveSettings {
    fn new() -> TypeEffectiveSettings {
        TypeEffectiveSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<TypeEffectiveSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "attack_scalar",
                    TypeEffectiveSettings::get_attack_scalar,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "attack_type",
                    TypeEffectiveSettings::has_attack_type,
                    TypeEffectiveSettings::get_attack_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TypeEffectiveSettings>(
                    "TypeEffectiveSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TypeEffectiveSettings {
    fn clear(&mut self) {
        self.clear_attack_scalar();
        self.clear_attack_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TypeEffectiveSettings {
    fn eq(&self, other: &TypeEffectiveSettings) -> bool {
        self.attack_scalar == other.attack_scalar &&
        self.attack_type == other.attack_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TypeEffectiveSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PokemonUpgradeSettings {
    // message fields
    upgrades_per_level: ::std::option::Option<i32>,
    allowed_levels_above_player: ::std::option::Option<i32>,
    candy_cost: ::std::vec::Vec<i32>,
    stardust_cost: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PokemonUpgradeSettings {}

impl PokemonUpgradeSettings {
    pub fn new() -> PokemonUpgradeSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PokemonUpgradeSettings {
        static mut instance: ::protobuf::lazy::Lazy<PokemonUpgradeSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PokemonUpgradeSettings,
        };
        unsafe {
            instance.get(|| {
                PokemonUpgradeSettings {
                    upgrades_per_level: ::std::option::Option::None,
                    allowed_levels_above_player: ::std::option::Option::None,
                    candy_cost: ::std::vec::Vec::new(),
                    stardust_cost: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 upgrades_per_level = 1;

    pub fn clear_upgrades_per_level(&mut self) {
        self.upgrades_per_level = ::std::option::Option::None;
    }

    pub fn has_upgrades_per_level(&self) -> bool {
        self.upgrades_per_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upgrades_per_level(&mut self, v: i32) {
        self.upgrades_per_level = ::std::option::Option::Some(v);
    }

    pub fn get_upgrades_per_level(&self) -> i32 {
        self.upgrades_per_level.unwrap_or(0)
    }

    // optional int32 allowed_levels_above_player = 2;

    pub fn clear_allowed_levels_above_player(&mut self) {
        self.allowed_levels_above_player = ::std::option::Option::None;
    }

    pub fn has_allowed_levels_above_player(&self) -> bool {
        self.allowed_levels_above_player.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allowed_levels_above_player(&mut self, v: i32) {
        self.allowed_levels_above_player = ::std::option::Option::Some(v);
    }

    pub fn get_allowed_levels_above_player(&self) -> i32 {
        self.allowed_levels_above_player.unwrap_or(0)
    }

    // repeated int32 candy_cost = 3;

    pub fn clear_candy_cost(&mut self) {
        self.candy_cost.clear();
    }

    // Param is passed by value, moved
    pub fn set_candy_cost(&mut self, v: ::std::vec::Vec<i32>) {
        self.candy_cost = v;
    }

    // Mutable pointer to the field.
    pub fn mut_candy_cost(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.candy_cost
    }

    // Take field
    pub fn take_candy_cost(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.candy_cost, ::std::vec::Vec::new())
    }

    pub fn get_candy_cost(&self) -> &[i32] {
        &self.candy_cost
    }

    // repeated int32 stardust_cost = 4;

    pub fn clear_stardust_cost(&mut self) {
        self.stardust_cost.clear();
    }

    // Param is passed by value, moved
    pub fn set_stardust_cost(&mut self, v: ::std::vec::Vec<i32>) {
        self.stardust_cost = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stardust_cost(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.stardust_cost
    }

    // Take field
    pub fn take_stardust_cost(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.stardust_cost, ::std::vec::Vec::new())
    }

    pub fn get_stardust_cost(&self) -> &[i32] {
        &self.stardust_cost
    }
}

impl ::protobuf::Message for PokemonUpgradeSettings {
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
                    self.upgrades_per_level = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.allowed_levels_above_player = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.candy_cost));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.stardust_cost));
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
        for value in &self.upgrades_per_level {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.allowed_levels_above_player {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.candy_cost {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.stardust_cost {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.upgrades_per_level {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.allowed_levels_above_player {
            try!(os.write_int32(2, v));
        };
        for v in &self.candy_cost {
            try!(os.write_int32(3, *v));
        };
        for v in &self.stardust_cost {
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
        ::std::any::TypeId::of::<PokemonUpgradeSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PokemonUpgradeSettings {
    fn new() -> PokemonUpgradeSettings {
        PokemonUpgradeSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<PokemonUpgradeSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "upgrades_per_level",
                    PokemonUpgradeSettings::has_upgrades_per_level,
                    PokemonUpgradeSettings::get_upgrades_per_level,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "allowed_levels_above_player",
                    PokemonUpgradeSettings::has_allowed_levels_above_player,
                    PokemonUpgradeSettings::get_allowed_levels_above_player,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "candy_cost",
                    PokemonUpgradeSettings::get_candy_cost,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "stardust_cost",
                    PokemonUpgradeSettings::get_stardust_cost,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PokemonUpgradeSettings>(
                    "PokemonUpgradeSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PokemonUpgradeSettings {
    fn clear(&mut self) {
        self.clear_upgrades_per_level();
        self.clear_allowed_levels_above_player();
        self.clear_candy_cost();
        self.clear_stardust_cost();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PokemonUpgradeSettings {
    fn eq(&self, other: &PokemonUpgradeSettings) -> bool {
        self.upgrades_per_level == other.upgrades_per_level &&
        self.allowed_levels_above_player == other.allowed_levels_above_player &&
        self.candy_cost == other.candy_cost &&
        self.stardust_cost == other.stardust_cost &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PokemonUpgradeSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ItemSettings {
    // message fields
    item_id: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemId>,
    item_type: ::std::option::Option<super::POGOProtos_Inventory_Item::ItemType>,
    category: ::std::option::Option<super::POGOProtos_Enums::ItemCategory>,
    drop_freq: ::std::option::Option<f32>,
    drop_trainer_level: ::std::option::Option<i32>,
    pokeball: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master_Item::PokeballAttributes>,
    potion: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master_Item::PotionAttributes>,
    revive: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master_Item::ReviveAttributes>,
    battle: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master_Item::BattleAttributes>,
    food: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master_Item::FoodAttributes>,
    inventory_upgrade: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master_Item::InventoryUpgradeAttributes>,
    xp_boost: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master_Item::ExperienceBoostAttributes>,
    incense: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master_Item::IncenseAttributes>,
    egg_incubator: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master_Item::EggIncubatorAttributes>,
    fort_modifier: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master_Item::FortModifierAttributes>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ItemSettings {}

impl ItemSettings {
    pub fn new() -> ItemSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ItemSettings {
        static mut instance: ::protobuf::lazy::Lazy<ItemSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ItemSettings,
        };
        unsafe {
            instance.get(|| {
                ItemSettings {
                    item_id: ::std::option::Option::None,
                    item_type: ::std::option::Option::None,
                    category: ::std::option::Option::None,
                    drop_freq: ::std::option::Option::None,
                    drop_trainer_level: ::std::option::Option::None,
                    pokeball: ::protobuf::SingularPtrField::none(),
                    potion: ::protobuf::SingularPtrField::none(),
                    revive: ::protobuf::SingularPtrField::none(),
                    battle: ::protobuf::SingularPtrField::none(),
                    food: ::protobuf::SingularPtrField::none(),
                    inventory_upgrade: ::protobuf::SingularPtrField::none(),
                    xp_boost: ::protobuf::SingularPtrField::none(),
                    incense: ::protobuf::SingularPtrField::none(),
                    egg_incubator: ::protobuf::SingularPtrField::none(),
                    fort_modifier: ::protobuf::SingularPtrField::none(),
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

    // optional .POGOProtos.Enums.ItemCategory category = 3;

    pub fn clear_category(&mut self) {
        self.category = ::std::option::Option::None;
    }

    pub fn has_category(&self) -> bool {
        self.category.is_some()
    }

    // Param is passed by value, moved
    pub fn set_category(&mut self, v: super::POGOProtos_Enums::ItemCategory) {
        self.category = ::std::option::Option::Some(v);
    }

    pub fn get_category(&self) -> super::POGOProtos_Enums::ItemCategory {
        self.category.unwrap_or(super::POGOProtos_Enums::ItemCategory::ITEM_CATEGORY_NONE)
    }

    // optional float drop_freq = 4;

    pub fn clear_drop_freq(&mut self) {
        self.drop_freq = ::std::option::Option::None;
    }

    pub fn has_drop_freq(&self) -> bool {
        self.drop_freq.is_some()
    }

    // Param is passed by value, moved
    pub fn set_drop_freq(&mut self, v: f32) {
        self.drop_freq = ::std::option::Option::Some(v);
    }

    pub fn get_drop_freq(&self) -> f32 {
        self.drop_freq.unwrap_or(0.)
    }

    // optional int32 drop_trainer_level = 5;

    pub fn clear_drop_trainer_level(&mut self) {
        self.drop_trainer_level = ::std::option::Option::None;
    }

    pub fn has_drop_trainer_level(&self) -> bool {
        self.drop_trainer_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_drop_trainer_level(&mut self, v: i32) {
        self.drop_trainer_level = ::std::option::Option::Some(v);
    }

    pub fn get_drop_trainer_level(&self) -> i32 {
        self.drop_trainer_level.unwrap_or(0)
    }

    // optional .POGOProtos.Settings.Master.Item.PokeballAttributes pokeball = 6;

    pub fn clear_pokeball(&mut self) {
        self.pokeball.clear();
    }

    pub fn has_pokeball(&self) -> bool {
        self.pokeball.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokeball(&mut self, v: super::POGOProtos_Settings_Master_Item::PokeballAttributes) {
        self.pokeball = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pokeball(&mut self) -> &mut super::POGOProtos_Settings_Master_Item::PokeballAttributes {
        if self.pokeball.is_none() {
            self.pokeball.set_default();
        };
        self.pokeball.as_mut().unwrap()
    }

    // Take field
    pub fn take_pokeball(&mut self) -> super::POGOProtos_Settings_Master_Item::PokeballAttributes {
        self.pokeball.take().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::PokeballAttributes::new())
    }

    pub fn get_pokeball(&self) -> &super::POGOProtos_Settings_Master_Item::PokeballAttributes {
        self.pokeball.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::PokeballAttributes::default_instance())
    }

    // optional .POGOProtos.Settings.Master.Item.PotionAttributes potion = 7;

    pub fn clear_potion(&mut self) {
        self.potion.clear();
    }

    pub fn has_potion(&self) -> bool {
        self.potion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_potion(&mut self, v: super::POGOProtos_Settings_Master_Item::PotionAttributes) {
        self.potion = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_potion(&mut self) -> &mut super::POGOProtos_Settings_Master_Item::PotionAttributes {
        if self.potion.is_none() {
            self.potion.set_default();
        };
        self.potion.as_mut().unwrap()
    }

    // Take field
    pub fn take_potion(&mut self) -> super::POGOProtos_Settings_Master_Item::PotionAttributes {
        self.potion.take().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::PotionAttributes::new())
    }

    pub fn get_potion(&self) -> &super::POGOProtos_Settings_Master_Item::PotionAttributes {
        self.potion.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::PotionAttributes::default_instance())
    }

    // optional .POGOProtos.Settings.Master.Item.ReviveAttributes revive = 8;

    pub fn clear_revive(&mut self) {
        self.revive.clear();
    }

    pub fn has_revive(&self) -> bool {
        self.revive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revive(&mut self, v: super::POGOProtos_Settings_Master_Item::ReviveAttributes) {
        self.revive = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_revive(&mut self) -> &mut super::POGOProtos_Settings_Master_Item::ReviveAttributes {
        if self.revive.is_none() {
            self.revive.set_default();
        };
        self.revive.as_mut().unwrap()
    }

    // Take field
    pub fn take_revive(&mut self) -> super::POGOProtos_Settings_Master_Item::ReviveAttributes {
        self.revive.take().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::ReviveAttributes::new())
    }

    pub fn get_revive(&self) -> &super::POGOProtos_Settings_Master_Item::ReviveAttributes {
        self.revive.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::ReviveAttributes::default_instance())
    }

    // optional .POGOProtos.Settings.Master.Item.BattleAttributes battle = 9;

    pub fn clear_battle(&mut self) {
        self.battle.clear();
    }

    pub fn has_battle(&self) -> bool {
        self.battle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battle(&mut self, v: super::POGOProtos_Settings_Master_Item::BattleAttributes) {
        self.battle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_battle(&mut self) -> &mut super::POGOProtos_Settings_Master_Item::BattleAttributes {
        if self.battle.is_none() {
            self.battle.set_default();
        };
        self.battle.as_mut().unwrap()
    }

    // Take field
    pub fn take_battle(&mut self) -> super::POGOProtos_Settings_Master_Item::BattleAttributes {
        self.battle.take().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::BattleAttributes::new())
    }

    pub fn get_battle(&self) -> &super::POGOProtos_Settings_Master_Item::BattleAttributes {
        self.battle.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::BattleAttributes::default_instance())
    }

    // optional .POGOProtos.Settings.Master.Item.FoodAttributes food = 10;

    pub fn clear_food(&mut self) {
        self.food.clear();
    }

    pub fn has_food(&self) -> bool {
        self.food.is_some()
    }

    // Param is passed by value, moved
    pub fn set_food(&mut self, v: super::POGOProtos_Settings_Master_Item::FoodAttributes) {
        self.food = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_food(&mut self) -> &mut super::POGOProtos_Settings_Master_Item::FoodAttributes {
        if self.food.is_none() {
            self.food.set_default();
        };
        self.food.as_mut().unwrap()
    }

    // Take field
    pub fn take_food(&mut self) -> super::POGOProtos_Settings_Master_Item::FoodAttributes {
        self.food.take().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::FoodAttributes::new())
    }

    pub fn get_food(&self) -> &super::POGOProtos_Settings_Master_Item::FoodAttributes {
        self.food.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::FoodAttributes::default_instance())
    }

    // optional .POGOProtos.Settings.Master.Item.InventoryUpgradeAttributes inventory_upgrade = 11;

    pub fn clear_inventory_upgrade(&mut self) {
        self.inventory_upgrade.clear();
    }

    pub fn has_inventory_upgrade(&self) -> bool {
        self.inventory_upgrade.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inventory_upgrade(&mut self, v: super::POGOProtos_Settings_Master_Item::InventoryUpgradeAttributes) {
        self.inventory_upgrade = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inventory_upgrade(&mut self) -> &mut super::POGOProtos_Settings_Master_Item::InventoryUpgradeAttributes {
        if self.inventory_upgrade.is_none() {
            self.inventory_upgrade.set_default();
        };
        self.inventory_upgrade.as_mut().unwrap()
    }

    // Take field
    pub fn take_inventory_upgrade(&mut self) -> super::POGOProtos_Settings_Master_Item::InventoryUpgradeAttributes {
        self.inventory_upgrade.take().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::InventoryUpgradeAttributes::new())
    }

    pub fn get_inventory_upgrade(&self) -> &super::POGOProtos_Settings_Master_Item::InventoryUpgradeAttributes {
        self.inventory_upgrade.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::InventoryUpgradeAttributes::default_instance())
    }

    // optional .POGOProtos.Settings.Master.Item.ExperienceBoostAttributes xp_boost = 12;

    pub fn clear_xp_boost(&mut self) {
        self.xp_boost.clear();
    }

    pub fn has_xp_boost(&self) -> bool {
        self.xp_boost.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xp_boost(&mut self, v: super::POGOProtos_Settings_Master_Item::ExperienceBoostAttributes) {
        self.xp_boost = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_xp_boost(&mut self) -> &mut super::POGOProtos_Settings_Master_Item::ExperienceBoostAttributes {
        if self.xp_boost.is_none() {
            self.xp_boost.set_default();
        };
        self.xp_boost.as_mut().unwrap()
    }

    // Take field
    pub fn take_xp_boost(&mut self) -> super::POGOProtos_Settings_Master_Item::ExperienceBoostAttributes {
        self.xp_boost.take().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::ExperienceBoostAttributes::new())
    }

    pub fn get_xp_boost(&self) -> &super::POGOProtos_Settings_Master_Item::ExperienceBoostAttributes {
        self.xp_boost.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::ExperienceBoostAttributes::default_instance())
    }

    // optional .POGOProtos.Settings.Master.Item.IncenseAttributes incense = 13;

    pub fn clear_incense(&mut self) {
        self.incense.clear();
    }

    pub fn has_incense(&self) -> bool {
        self.incense.is_some()
    }

    // Param is passed by value, moved
    pub fn set_incense(&mut self, v: super::POGOProtos_Settings_Master_Item::IncenseAttributes) {
        self.incense = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_incense(&mut self) -> &mut super::POGOProtos_Settings_Master_Item::IncenseAttributes {
        if self.incense.is_none() {
            self.incense.set_default();
        };
        self.incense.as_mut().unwrap()
    }

    // Take field
    pub fn take_incense(&mut self) -> super::POGOProtos_Settings_Master_Item::IncenseAttributes {
        self.incense.take().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::IncenseAttributes::new())
    }

    pub fn get_incense(&self) -> &super::POGOProtos_Settings_Master_Item::IncenseAttributes {
        self.incense.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::IncenseAttributes::default_instance())
    }

    // optional .POGOProtos.Settings.Master.Item.EggIncubatorAttributes egg_incubator = 14;

    pub fn clear_egg_incubator(&mut self) {
        self.egg_incubator.clear();
    }

    pub fn has_egg_incubator(&self) -> bool {
        self.egg_incubator.is_some()
    }

    // Param is passed by value, moved
    pub fn set_egg_incubator(&mut self, v: super::POGOProtos_Settings_Master_Item::EggIncubatorAttributes) {
        self.egg_incubator = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_egg_incubator(&mut self) -> &mut super::POGOProtos_Settings_Master_Item::EggIncubatorAttributes {
        if self.egg_incubator.is_none() {
            self.egg_incubator.set_default();
        };
        self.egg_incubator.as_mut().unwrap()
    }

    // Take field
    pub fn take_egg_incubator(&mut self) -> super::POGOProtos_Settings_Master_Item::EggIncubatorAttributes {
        self.egg_incubator.take().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::EggIncubatorAttributes::new())
    }

    pub fn get_egg_incubator(&self) -> &super::POGOProtos_Settings_Master_Item::EggIncubatorAttributes {
        self.egg_incubator.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::EggIncubatorAttributes::default_instance())
    }

    // optional .POGOProtos.Settings.Master.Item.FortModifierAttributes fort_modifier = 15;

    pub fn clear_fort_modifier(&mut self) {
        self.fort_modifier.clear();
    }

    pub fn has_fort_modifier(&self) -> bool {
        self.fort_modifier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fort_modifier(&mut self, v: super::POGOProtos_Settings_Master_Item::FortModifierAttributes) {
        self.fort_modifier = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fort_modifier(&mut self) -> &mut super::POGOProtos_Settings_Master_Item::FortModifierAttributes {
        if self.fort_modifier.is_none() {
            self.fort_modifier.set_default();
        };
        self.fort_modifier.as_mut().unwrap()
    }

    // Take field
    pub fn take_fort_modifier(&mut self) -> super::POGOProtos_Settings_Master_Item::FortModifierAttributes {
        self.fort_modifier.take().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::FortModifierAttributes::new())
    }

    pub fn get_fort_modifier(&self) -> &super::POGOProtos_Settings_Master_Item::FortModifierAttributes {
        self.fort_modifier.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master_Item::FortModifierAttributes::default_instance())
    }
}

impl ::protobuf::Message for ItemSettings {
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
                    let tmp = try!(is.read_enum());
                    self.category = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.drop_freq = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.drop_trainer_level = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pokeball));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.potion));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.revive));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.battle));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.food));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.inventory_upgrade));
                },
                12 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.xp_boost));
                },
                13 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.incense));
                },
                14 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.egg_incubator));
                },
                15 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fort_modifier));
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
        for value in &self.category {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        if self.drop_freq.is_some() {
            my_size += 5;
        };
        for value in &self.drop_trainer_level {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pokeball {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.potion {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.revive {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.battle {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.food {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.inventory_upgrade {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.xp_boost {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.incense {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.egg_incubator {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.fort_modifier {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(v) = self.category {
            try!(os.write_enum(3, v.value()));
        };
        if let Some(v) = self.drop_freq {
            try!(os.write_float(4, v));
        };
        if let Some(v) = self.drop_trainer_level {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.pokeball.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.potion.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.revive.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.battle.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.food.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.inventory_upgrade.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.xp_boost.as_ref() {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.incense.as_ref() {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.egg_incubator.as_ref() {
            try!(os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.fort_modifier.as_ref() {
            try!(os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<ItemSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ItemSettings {
    fn new() -> ItemSettings {
        ItemSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<ItemSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "item_id",
                    ItemSettings::has_item_id,
                    ItemSettings::get_item_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "item_type",
                    ItemSettings::has_item_type,
                    ItemSettings::get_item_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "category",
                    ItemSettings::has_category,
                    ItemSettings::get_category,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "drop_freq",
                    ItemSettings::has_drop_freq,
                    ItemSettings::get_drop_freq,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "drop_trainer_level",
                    ItemSettings::has_drop_trainer_level,
                    ItemSettings::get_drop_trainer_level,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pokeball",
                    ItemSettings::has_pokeball,
                    ItemSettings::get_pokeball,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "potion",
                    ItemSettings::has_potion,
                    ItemSettings::get_potion,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "revive",
                    ItemSettings::has_revive,
                    ItemSettings::get_revive,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "battle",
                    ItemSettings::has_battle,
                    ItemSettings::get_battle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "food",
                    ItemSettings::has_food,
                    ItemSettings::get_food,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "inventory_upgrade",
                    ItemSettings::has_inventory_upgrade,
                    ItemSettings::get_inventory_upgrade,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "xp_boost",
                    ItemSettings::has_xp_boost,
                    ItemSettings::get_xp_boost,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "incense",
                    ItemSettings::has_incense,
                    ItemSettings::get_incense,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "egg_incubator",
                    ItemSettings::has_egg_incubator,
                    ItemSettings::get_egg_incubator,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fort_modifier",
                    ItemSettings::has_fort_modifier,
                    ItemSettings::get_fort_modifier,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ItemSettings>(
                    "ItemSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ItemSettings {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_item_type();
        self.clear_category();
        self.clear_drop_freq();
        self.clear_drop_trainer_level();
        self.clear_pokeball();
        self.clear_potion();
        self.clear_revive();
        self.clear_battle();
        self.clear_food();
        self.clear_inventory_upgrade();
        self.clear_xp_boost();
        self.clear_incense();
        self.clear_egg_incubator();
        self.clear_fort_modifier();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ItemSettings {
    fn eq(&self, other: &ItemSettings) -> bool {
        self.item_id == other.item_id &&
        self.item_type == other.item_type &&
        self.category == other.category &&
        self.drop_freq == other.drop_freq &&
        self.drop_trainer_level == other.drop_trainer_level &&
        self.pokeball == other.pokeball &&
        self.potion == other.potion &&
        self.revive == other.revive &&
        self.battle == other.battle &&
        self.food == other.food &&
        self.inventory_upgrade == other.inventory_upgrade &&
        self.xp_boost == other.xp_boost &&
        self.incense == other.incense &&
        self.egg_incubator == other.egg_incubator &&
        self.fort_modifier == other.fort_modifier &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ItemSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct IapSettings {
    // message fields
    daily_bonus_coins: ::std::option::Option<i32>,
    daily_defender_bonus_per_pokemon: ::std::vec::Vec<i32>,
    daily_defender_bonus_max_defenders: ::std::option::Option<i32>,
    daily_defender_bonus_currency: ::protobuf::RepeatedField<::std::string::String>,
    min_time_between_claims_ms: ::std::option::Option<i64>,
    daily_bonus_enabled: ::std::option::Option<bool>,
    daily_defender_bonus_enabled: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IapSettings {}

impl IapSettings {
    pub fn new() -> IapSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IapSettings {
        static mut instance: ::protobuf::lazy::Lazy<IapSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IapSettings,
        };
        unsafe {
            instance.get(|| {
                IapSettings {
                    daily_bonus_coins: ::std::option::Option::None,
                    daily_defender_bonus_per_pokemon: ::std::vec::Vec::new(),
                    daily_defender_bonus_max_defenders: ::std::option::Option::None,
                    daily_defender_bonus_currency: ::protobuf::RepeatedField::new(),
                    min_time_between_claims_ms: ::std::option::Option::None,
                    daily_bonus_enabled: ::std::option::Option::None,
                    daily_defender_bonus_enabled: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 daily_bonus_coins = 1;

    pub fn clear_daily_bonus_coins(&mut self) {
        self.daily_bonus_coins = ::std::option::Option::None;
    }

    pub fn has_daily_bonus_coins(&self) -> bool {
        self.daily_bonus_coins.is_some()
    }

    // Param is passed by value, moved
    pub fn set_daily_bonus_coins(&mut self, v: i32) {
        self.daily_bonus_coins = ::std::option::Option::Some(v);
    }

    pub fn get_daily_bonus_coins(&self) -> i32 {
        self.daily_bonus_coins.unwrap_or(0)
    }

    // repeated int32 daily_defender_bonus_per_pokemon = 2;

    pub fn clear_daily_defender_bonus_per_pokemon(&mut self) {
        self.daily_defender_bonus_per_pokemon.clear();
    }

    // Param is passed by value, moved
    pub fn set_daily_defender_bonus_per_pokemon(&mut self, v: ::std::vec::Vec<i32>) {
        self.daily_defender_bonus_per_pokemon = v;
    }

    // Mutable pointer to the field.
    pub fn mut_daily_defender_bonus_per_pokemon(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.daily_defender_bonus_per_pokemon
    }

    // Take field
    pub fn take_daily_defender_bonus_per_pokemon(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.daily_defender_bonus_per_pokemon, ::std::vec::Vec::new())
    }

    pub fn get_daily_defender_bonus_per_pokemon(&self) -> &[i32] {
        &self.daily_defender_bonus_per_pokemon
    }

    // optional int32 daily_defender_bonus_max_defenders = 3;

    pub fn clear_daily_defender_bonus_max_defenders(&mut self) {
        self.daily_defender_bonus_max_defenders = ::std::option::Option::None;
    }

    pub fn has_daily_defender_bonus_max_defenders(&self) -> bool {
        self.daily_defender_bonus_max_defenders.is_some()
    }

    // Param is passed by value, moved
    pub fn set_daily_defender_bonus_max_defenders(&mut self, v: i32) {
        self.daily_defender_bonus_max_defenders = ::std::option::Option::Some(v);
    }

    pub fn get_daily_defender_bonus_max_defenders(&self) -> i32 {
        self.daily_defender_bonus_max_defenders.unwrap_or(0)
    }

    // repeated string daily_defender_bonus_currency = 4;

    pub fn clear_daily_defender_bonus_currency(&mut self) {
        self.daily_defender_bonus_currency.clear();
    }

    // Param is passed by value, moved
    pub fn set_daily_defender_bonus_currency(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.daily_defender_bonus_currency = v;
    }

    // Mutable pointer to the field.
    pub fn mut_daily_defender_bonus_currency(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.daily_defender_bonus_currency
    }

    // Take field
    pub fn take_daily_defender_bonus_currency(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.daily_defender_bonus_currency, ::protobuf::RepeatedField::new())
    }

    pub fn get_daily_defender_bonus_currency(&self) -> &[::std::string::String] {
        &self.daily_defender_bonus_currency
    }

    // optional int64 min_time_between_claims_ms = 5;

    pub fn clear_min_time_between_claims_ms(&mut self) {
        self.min_time_between_claims_ms = ::std::option::Option::None;
    }

    pub fn has_min_time_between_claims_ms(&self) -> bool {
        self.min_time_between_claims_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min_time_between_claims_ms(&mut self, v: i64) {
        self.min_time_between_claims_ms = ::std::option::Option::Some(v);
    }

    pub fn get_min_time_between_claims_ms(&self) -> i64 {
        self.min_time_between_claims_ms.unwrap_or(0)
    }

    // optional bool daily_bonus_enabled = 6;

    pub fn clear_daily_bonus_enabled(&mut self) {
        self.daily_bonus_enabled = ::std::option::Option::None;
    }

    pub fn has_daily_bonus_enabled(&self) -> bool {
        self.daily_bonus_enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_daily_bonus_enabled(&mut self, v: bool) {
        self.daily_bonus_enabled = ::std::option::Option::Some(v);
    }

    pub fn get_daily_bonus_enabled(&self) -> bool {
        self.daily_bonus_enabled.unwrap_or(false)
    }

    // optional bool daily_defender_bonus_enabled = 7;

    pub fn clear_daily_defender_bonus_enabled(&mut self) {
        self.daily_defender_bonus_enabled = ::std::option::Option::None;
    }

    pub fn has_daily_defender_bonus_enabled(&self) -> bool {
        self.daily_defender_bonus_enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_daily_defender_bonus_enabled(&mut self, v: bool) {
        self.daily_defender_bonus_enabled = ::std::option::Option::Some(v);
    }

    pub fn get_daily_defender_bonus_enabled(&self) -> bool {
        self.daily_defender_bonus_enabled.unwrap_or(false)
    }
}

impl ::protobuf::Message for IapSettings {
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
                    self.daily_bonus_coins = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.daily_defender_bonus_per_pokemon));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.daily_defender_bonus_max_defenders = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.daily_defender_bonus_currency));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.min_time_between_claims_ms = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.daily_bonus_enabled = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.daily_defender_bonus_enabled = ::std::option::Option::Some(tmp);
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
        for value in &self.daily_bonus_coins {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.daily_defender_bonus_per_pokemon {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.daily_defender_bonus_max_defenders {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.daily_defender_bonus_currency {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.min_time_between_claims_ms {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.daily_bonus_enabled.is_some() {
            my_size += 2;
        };
        if self.daily_defender_bonus_enabled.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.daily_bonus_coins {
            try!(os.write_int32(1, v));
        };
        for v in &self.daily_defender_bonus_per_pokemon {
            try!(os.write_int32(2, *v));
        };
        if let Some(v) = self.daily_defender_bonus_max_defenders {
            try!(os.write_int32(3, v));
        };
        for v in &self.daily_defender_bonus_currency {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.min_time_between_claims_ms {
            try!(os.write_int64(5, v));
        };
        if let Some(v) = self.daily_bonus_enabled {
            try!(os.write_bool(6, v));
        };
        if let Some(v) = self.daily_defender_bonus_enabled {
            try!(os.write_bool(7, v));
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
        ::std::any::TypeId::of::<IapSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IapSettings {
    fn new() -> IapSettings {
        IapSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<IapSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "daily_bonus_coins",
                    IapSettings::has_daily_bonus_coins,
                    IapSettings::get_daily_bonus_coins,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "daily_defender_bonus_per_pokemon",
                    IapSettings::get_daily_defender_bonus_per_pokemon,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "daily_defender_bonus_max_defenders",
                    IapSettings::has_daily_defender_bonus_max_defenders,
                    IapSettings::get_daily_defender_bonus_max_defenders,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "daily_defender_bonus_currency",
                    IapSettings::get_daily_defender_bonus_currency,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "min_time_between_claims_ms",
                    IapSettings::has_min_time_between_claims_ms,
                    IapSettings::get_min_time_between_claims_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "daily_bonus_enabled",
                    IapSettings::has_daily_bonus_enabled,
                    IapSettings::get_daily_bonus_enabled,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "daily_defender_bonus_enabled",
                    IapSettings::has_daily_defender_bonus_enabled,
                    IapSettings::get_daily_defender_bonus_enabled,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IapSettings>(
                    "IapSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IapSettings {
    fn clear(&mut self) {
        self.clear_daily_bonus_coins();
        self.clear_daily_defender_bonus_per_pokemon();
        self.clear_daily_defender_bonus_max_defenders();
        self.clear_daily_defender_bonus_currency();
        self.clear_min_time_between_claims_ms();
        self.clear_daily_bonus_enabled();
        self.clear_daily_defender_bonus_enabled();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IapSettings {
    fn eq(&self, other: &IapSettings) -> bool {
        self.daily_bonus_coins == other.daily_bonus_coins &&
        self.daily_defender_bonus_per_pokemon == other.daily_defender_bonus_per_pokemon &&
        self.daily_defender_bonus_max_defenders == other.daily_defender_bonus_max_defenders &&
        self.daily_defender_bonus_currency == other.daily_defender_bonus_currency &&
        self.min_time_between_claims_ms == other.min_time_between_claims_ms &&
        self.daily_bonus_enabled == other.daily_bonus_enabled &&
        self.daily_defender_bonus_enabled == other.daily_defender_bonus_enabled &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IapSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CameraSettings {
    // message fields
    next_camera: ::protobuf::SingularField<::std::string::String>,
    interpolation: ::std::vec::Vec<super::POGOProtos_Enums::CameraInterpolation>,
    target_type: ::std::vec::Vec<super::POGOProtos_Enums::CameraTarget>,
    ease_in_speed: ::std::vec::Vec<f32>,
    east_out_speed: ::std::vec::Vec<f32>,
    duration_seconds: ::std::vec::Vec<f32>,
    wait_seconds: ::std::vec::Vec<f32>,
    transition_seconds: ::std::vec::Vec<f32>,
    angle_degree: ::std::vec::Vec<f32>,
    angle_offset_degree: ::std::vec::Vec<f32>,
    pitch_degree: ::std::vec::Vec<f32>,
    pitch_offset_degree: ::std::vec::Vec<f32>,
    roll_degree: ::std::vec::Vec<f32>,
    distance_meters: ::std::vec::Vec<f32>,
    height_percent: ::std::vec::Vec<f32>,
    vert_ctr_ratio: ::std::vec::Vec<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CameraSettings {}

impl CameraSettings {
    pub fn new() -> CameraSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CameraSettings {
        static mut instance: ::protobuf::lazy::Lazy<CameraSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CameraSettings,
        };
        unsafe {
            instance.get(|| {
                CameraSettings {
                    next_camera: ::protobuf::SingularField::none(),
                    interpolation: ::std::vec::Vec::new(),
                    target_type: ::std::vec::Vec::new(),
                    ease_in_speed: ::std::vec::Vec::new(),
                    east_out_speed: ::std::vec::Vec::new(),
                    duration_seconds: ::std::vec::Vec::new(),
                    wait_seconds: ::std::vec::Vec::new(),
                    transition_seconds: ::std::vec::Vec::new(),
                    angle_degree: ::std::vec::Vec::new(),
                    angle_offset_degree: ::std::vec::Vec::new(),
                    pitch_degree: ::std::vec::Vec::new(),
                    pitch_offset_degree: ::std::vec::Vec::new(),
                    roll_degree: ::std::vec::Vec::new(),
                    distance_meters: ::std::vec::Vec::new(),
                    height_percent: ::std::vec::Vec::new(),
                    vert_ctr_ratio: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string next_camera = 1;

    pub fn clear_next_camera(&mut self) {
        self.next_camera.clear();
    }

    pub fn has_next_camera(&self) -> bool {
        self.next_camera.is_some()
    }

    // Param is passed by value, moved
    pub fn set_next_camera(&mut self, v: ::std::string::String) {
        self.next_camera = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_next_camera(&mut self) -> &mut ::std::string::String {
        if self.next_camera.is_none() {
            self.next_camera.set_default();
        };
        self.next_camera.as_mut().unwrap()
    }

    // Take field
    pub fn take_next_camera(&mut self) -> ::std::string::String {
        self.next_camera.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_next_camera(&self) -> &str {
        match self.next_camera.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .POGOProtos.Enums.CameraInterpolation interpolation = 2;

    pub fn clear_interpolation(&mut self) {
        self.interpolation.clear();
    }

    // Param is passed by value, moved
    pub fn set_interpolation(&mut self, v: ::std::vec::Vec<super::POGOProtos_Enums::CameraInterpolation>) {
        self.interpolation = v;
    }

    // Mutable pointer to the field.
    pub fn mut_interpolation(&mut self) -> &mut ::std::vec::Vec<super::POGOProtos_Enums::CameraInterpolation> {
        &mut self.interpolation
    }

    // Take field
    pub fn take_interpolation(&mut self) -> ::std::vec::Vec<super::POGOProtos_Enums::CameraInterpolation> {
        ::std::mem::replace(&mut self.interpolation, ::std::vec::Vec::new())
    }

    pub fn get_interpolation(&self) -> &[super::POGOProtos_Enums::CameraInterpolation] {
        &self.interpolation
    }

    // repeated .POGOProtos.Enums.CameraTarget target_type = 3;

    pub fn clear_target_type(&mut self) {
        self.target_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_target_type(&mut self, v: ::std::vec::Vec<super::POGOProtos_Enums::CameraTarget>) {
        self.target_type = v;
    }

    // Mutable pointer to the field.
    pub fn mut_target_type(&mut self) -> &mut ::std::vec::Vec<super::POGOProtos_Enums::CameraTarget> {
        &mut self.target_type
    }

    // Take field
    pub fn take_target_type(&mut self) -> ::std::vec::Vec<super::POGOProtos_Enums::CameraTarget> {
        ::std::mem::replace(&mut self.target_type, ::std::vec::Vec::new())
    }

    pub fn get_target_type(&self) -> &[super::POGOProtos_Enums::CameraTarget] {
        &self.target_type
    }

    // repeated float ease_in_speed = 4;

    pub fn clear_ease_in_speed(&mut self) {
        self.ease_in_speed.clear();
    }

    // Param is passed by value, moved
    pub fn set_ease_in_speed(&mut self, v: ::std::vec::Vec<f32>) {
        self.ease_in_speed = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ease_in_speed(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.ease_in_speed
    }

    // Take field
    pub fn take_ease_in_speed(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.ease_in_speed, ::std::vec::Vec::new())
    }

    pub fn get_ease_in_speed(&self) -> &[f32] {
        &self.ease_in_speed
    }

    // repeated float east_out_speed = 5;

    pub fn clear_east_out_speed(&mut self) {
        self.east_out_speed.clear();
    }

    // Param is passed by value, moved
    pub fn set_east_out_speed(&mut self, v: ::std::vec::Vec<f32>) {
        self.east_out_speed = v;
    }

    // Mutable pointer to the field.
    pub fn mut_east_out_speed(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.east_out_speed
    }

    // Take field
    pub fn take_east_out_speed(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.east_out_speed, ::std::vec::Vec::new())
    }

    pub fn get_east_out_speed(&self) -> &[f32] {
        &self.east_out_speed
    }

    // repeated float duration_seconds = 6;

    pub fn clear_duration_seconds(&mut self) {
        self.duration_seconds.clear();
    }

    // Param is passed by value, moved
    pub fn set_duration_seconds(&mut self, v: ::std::vec::Vec<f32>) {
        self.duration_seconds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_duration_seconds(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.duration_seconds
    }

    // Take field
    pub fn take_duration_seconds(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.duration_seconds, ::std::vec::Vec::new())
    }

    pub fn get_duration_seconds(&self) -> &[f32] {
        &self.duration_seconds
    }

    // repeated float wait_seconds = 7;

    pub fn clear_wait_seconds(&mut self) {
        self.wait_seconds.clear();
    }

    // Param is passed by value, moved
    pub fn set_wait_seconds(&mut self, v: ::std::vec::Vec<f32>) {
        self.wait_seconds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_wait_seconds(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.wait_seconds
    }

    // Take field
    pub fn take_wait_seconds(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.wait_seconds, ::std::vec::Vec::new())
    }

    pub fn get_wait_seconds(&self) -> &[f32] {
        &self.wait_seconds
    }

    // repeated float transition_seconds = 8;

    pub fn clear_transition_seconds(&mut self) {
        self.transition_seconds.clear();
    }

    // Param is passed by value, moved
    pub fn set_transition_seconds(&mut self, v: ::std::vec::Vec<f32>) {
        self.transition_seconds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_transition_seconds(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.transition_seconds
    }

    // Take field
    pub fn take_transition_seconds(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.transition_seconds, ::std::vec::Vec::new())
    }

    pub fn get_transition_seconds(&self) -> &[f32] {
        &self.transition_seconds
    }

    // repeated float angle_degree = 9;

    pub fn clear_angle_degree(&mut self) {
        self.angle_degree.clear();
    }

    // Param is passed by value, moved
    pub fn set_angle_degree(&mut self, v: ::std::vec::Vec<f32>) {
        self.angle_degree = v;
    }

    // Mutable pointer to the field.
    pub fn mut_angle_degree(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.angle_degree
    }

    // Take field
    pub fn take_angle_degree(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.angle_degree, ::std::vec::Vec::new())
    }

    pub fn get_angle_degree(&self) -> &[f32] {
        &self.angle_degree
    }

    // repeated float angle_offset_degree = 10;

    pub fn clear_angle_offset_degree(&mut self) {
        self.angle_offset_degree.clear();
    }

    // Param is passed by value, moved
    pub fn set_angle_offset_degree(&mut self, v: ::std::vec::Vec<f32>) {
        self.angle_offset_degree = v;
    }

    // Mutable pointer to the field.
    pub fn mut_angle_offset_degree(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.angle_offset_degree
    }

    // Take field
    pub fn take_angle_offset_degree(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.angle_offset_degree, ::std::vec::Vec::new())
    }

    pub fn get_angle_offset_degree(&self) -> &[f32] {
        &self.angle_offset_degree
    }

    // repeated float pitch_degree = 11;

    pub fn clear_pitch_degree(&mut self) {
        self.pitch_degree.clear();
    }

    // Param is passed by value, moved
    pub fn set_pitch_degree(&mut self, v: ::std::vec::Vec<f32>) {
        self.pitch_degree = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pitch_degree(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.pitch_degree
    }

    // Take field
    pub fn take_pitch_degree(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.pitch_degree, ::std::vec::Vec::new())
    }

    pub fn get_pitch_degree(&self) -> &[f32] {
        &self.pitch_degree
    }

    // repeated float pitch_offset_degree = 12;

    pub fn clear_pitch_offset_degree(&mut self) {
        self.pitch_offset_degree.clear();
    }

    // Param is passed by value, moved
    pub fn set_pitch_offset_degree(&mut self, v: ::std::vec::Vec<f32>) {
        self.pitch_offset_degree = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pitch_offset_degree(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.pitch_offset_degree
    }

    // Take field
    pub fn take_pitch_offset_degree(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.pitch_offset_degree, ::std::vec::Vec::new())
    }

    pub fn get_pitch_offset_degree(&self) -> &[f32] {
        &self.pitch_offset_degree
    }

    // repeated float roll_degree = 13;

    pub fn clear_roll_degree(&mut self) {
        self.roll_degree.clear();
    }

    // Param is passed by value, moved
    pub fn set_roll_degree(&mut self, v: ::std::vec::Vec<f32>) {
        self.roll_degree = v;
    }

    // Mutable pointer to the field.
    pub fn mut_roll_degree(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.roll_degree
    }

    // Take field
    pub fn take_roll_degree(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.roll_degree, ::std::vec::Vec::new())
    }

    pub fn get_roll_degree(&self) -> &[f32] {
        &self.roll_degree
    }

    // repeated float distance_meters = 14;

    pub fn clear_distance_meters(&mut self) {
        self.distance_meters.clear();
    }

    // Param is passed by value, moved
    pub fn set_distance_meters(&mut self, v: ::std::vec::Vec<f32>) {
        self.distance_meters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_distance_meters(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.distance_meters
    }

    // Take field
    pub fn take_distance_meters(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.distance_meters, ::std::vec::Vec::new())
    }

    pub fn get_distance_meters(&self) -> &[f32] {
        &self.distance_meters
    }

    // repeated float height_percent = 15;

    pub fn clear_height_percent(&mut self) {
        self.height_percent.clear();
    }

    // Param is passed by value, moved
    pub fn set_height_percent(&mut self, v: ::std::vec::Vec<f32>) {
        self.height_percent = v;
    }

    // Mutable pointer to the field.
    pub fn mut_height_percent(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.height_percent
    }

    // Take field
    pub fn take_height_percent(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.height_percent, ::std::vec::Vec::new())
    }

    pub fn get_height_percent(&self) -> &[f32] {
        &self.height_percent
    }

    // repeated float vert_ctr_ratio = 16;

    pub fn clear_vert_ctr_ratio(&mut self) {
        self.vert_ctr_ratio.clear();
    }

    // Param is passed by value, moved
    pub fn set_vert_ctr_ratio(&mut self, v: ::std::vec::Vec<f32>) {
        self.vert_ctr_ratio = v;
    }

    // Mutable pointer to the field.
    pub fn mut_vert_ctr_ratio(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.vert_ctr_ratio
    }

    // Take field
    pub fn take_vert_ctr_ratio(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.vert_ctr_ratio, ::std::vec::Vec::new())
    }

    pub fn get_vert_ctr_ratio(&self) -> &[f32] {
        &self.vert_ctr_ratio
    }
}

impl ::protobuf::Message for CameraSettings {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.next_camera));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.interpolation));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.target_type));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.ease_in_speed));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.east_out_speed));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.duration_seconds));
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.wait_seconds));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.transition_seconds));
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.angle_degree));
                },
                10 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.angle_offset_degree));
                },
                11 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.pitch_degree));
                },
                12 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.pitch_offset_degree));
                },
                13 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.roll_degree));
                },
                14 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.distance_meters));
                },
                15 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.height_percent));
                },
                16 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.vert_ctr_ratio));
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
        for value in &self.next_camera {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.interpolation {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.target_type {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        my_size += 5 * self.ease_in_speed.len() as u32;
        my_size += 5 * self.east_out_speed.len() as u32;
        my_size += 5 * self.duration_seconds.len() as u32;
        my_size += 5 * self.wait_seconds.len() as u32;
        my_size += 5 * self.transition_seconds.len() as u32;
        my_size += 5 * self.angle_degree.len() as u32;
        my_size += 5 * self.angle_offset_degree.len() as u32;
        my_size += 5 * self.pitch_degree.len() as u32;
        my_size += 5 * self.pitch_offset_degree.len() as u32;
        my_size += 5 * self.roll_degree.len() as u32;
        my_size += 5 * self.distance_meters.len() as u32;
        my_size += 5 * self.height_percent.len() as u32;
        my_size += 6 * self.vert_ctr_ratio.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.next_camera.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in &self.interpolation {
            try!(os.write_enum(2, v.value()));
        };
        for v in &self.target_type {
            try!(os.write_enum(3, v.value()));
        };
        for v in &self.ease_in_speed {
            try!(os.write_float(4, *v));
        };
        for v in &self.east_out_speed {
            try!(os.write_float(5, *v));
        };
        for v in &self.duration_seconds {
            try!(os.write_float(6, *v));
        };
        for v in &self.wait_seconds {
            try!(os.write_float(7, *v));
        };
        for v in &self.transition_seconds {
            try!(os.write_float(8, *v));
        };
        for v in &self.angle_degree {
            try!(os.write_float(9, *v));
        };
        for v in &self.angle_offset_degree {
            try!(os.write_float(10, *v));
        };
        for v in &self.pitch_degree {
            try!(os.write_float(11, *v));
        };
        for v in &self.pitch_offset_degree {
            try!(os.write_float(12, *v));
        };
        for v in &self.roll_degree {
            try!(os.write_float(13, *v));
        };
        for v in &self.distance_meters {
            try!(os.write_float(14, *v));
        };
        for v in &self.height_percent {
            try!(os.write_float(15, *v));
        };
        for v in &self.vert_ctr_ratio {
            try!(os.write_float(16, *v));
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
        ::std::any::TypeId::of::<CameraSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CameraSettings {
    fn new() -> CameraSettings {
        CameraSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<CameraSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "next_camera",
                    CameraSettings::has_next_camera,
                    CameraSettings::get_next_camera,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "interpolation",
                    CameraSettings::get_interpolation,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "target_type",
                    CameraSettings::get_target_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "ease_in_speed",
                    CameraSettings::get_ease_in_speed,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "east_out_speed",
                    CameraSettings::get_east_out_speed,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "duration_seconds",
                    CameraSettings::get_duration_seconds,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "wait_seconds",
                    CameraSettings::get_wait_seconds,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "transition_seconds",
                    CameraSettings::get_transition_seconds,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "angle_degree",
                    CameraSettings::get_angle_degree,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "angle_offset_degree",
                    CameraSettings::get_angle_offset_degree,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "pitch_degree",
                    CameraSettings::get_pitch_degree,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "pitch_offset_degree",
                    CameraSettings::get_pitch_offset_degree,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "roll_degree",
                    CameraSettings::get_roll_degree,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "distance_meters",
                    CameraSettings::get_distance_meters,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "height_percent",
                    CameraSettings::get_height_percent,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "vert_ctr_ratio",
                    CameraSettings::get_vert_ctr_ratio,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CameraSettings>(
                    "CameraSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CameraSettings {
    fn clear(&mut self) {
        self.clear_next_camera();
        self.clear_interpolation();
        self.clear_target_type();
        self.clear_ease_in_speed();
        self.clear_east_out_speed();
        self.clear_duration_seconds();
        self.clear_wait_seconds();
        self.clear_transition_seconds();
        self.clear_angle_degree();
        self.clear_angle_offset_degree();
        self.clear_pitch_degree();
        self.clear_pitch_offset_degree();
        self.clear_roll_degree();
        self.clear_distance_meters();
        self.clear_height_percent();
        self.clear_vert_ctr_ratio();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CameraSettings {
    fn eq(&self, other: &CameraSettings) -> bool {
        self.next_camera == other.next_camera &&
        self.interpolation == other.interpolation &&
        self.target_type == other.target_type &&
        self.ease_in_speed == other.ease_in_speed &&
        self.east_out_speed == other.east_out_speed &&
        self.duration_seconds == other.duration_seconds &&
        self.wait_seconds == other.wait_seconds &&
        self.transition_seconds == other.transition_seconds &&
        self.angle_degree == other.angle_degree &&
        self.angle_offset_degree == other.angle_offset_degree &&
        self.pitch_degree == other.pitch_degree &&
        self.pitch_offset_degree == other.pitch_offset_degree &&
        self.roll_degree == other.roll_degree &&
        self.distance_meters == other.distance_meters &&
        self.height_percent == other.height_percent &&
        self.vert_ctr_ratio == other.vert_ctr_ratio &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CameraSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct MoveSettings {
    // message fields
    movement_id: ::std::option::Option<super::POGOProtos_Enums::PokemonMove>,
    animation_id: ::std::option::Option<i32>,
    pokemon_type: ::std::option::Option<super::POGOProtos_Enums::PokemonType>,
    power: ::std::option::Option<f32>,
    accuracy_chance: ::std::option::Option<f32>,
    critical_chance: ::std::option::Option<f32>,
    heal_scalar: ::std::option::Option<f32>,
    stamina_loss_scalar: ::std::option::Option<f32>,
    trainer_level_min: ::std::option::Option<i32>,
    trainer_level_max: ::std::option::Option<i32>,
    vfx_name: ::protobuf::SingularField<::std::string::String>,
    duration_ms: ::std::option::Option<i32>,
    damage_window_start_ms: ::std::option::Option<i32>,
    damage_window_end_ms: ::std::option::Option<i32>,
    energy_delta: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MoveSettings {}

impl MoveSettings {
    pub fn new() -> MoveSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MoveSettings {
        static mut instance: ::protobuf::lazy::Lazy<MoveSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MoveSettings,
        };
        unsafe {
            instance.get(|| {
                MoveSettings {
                    movement_id: ::std::option::Option::None,
                    animation_id: ::std::option::Option::None,
                    pokemon_type: ::std::option::Option::None,
                    power: ::std::option::Option::None,
                    accuracy_chance: ::std::option::Option::None,
                    critical_chance: ::std::option::Option::None,
                    heal_scalar: ::std::option::Option::None,
                    stamina_loss_scalar: ::std::option::Option::None,
                    trainer_level_min: ::std::option::Option::None,
                    trainer_level_max: ::std::option::Option::None,
                    vfx_name: ::protobuf::SingularField::none(),
                    duration_ms: ::std::option::Option::None,
                    damage_window_start_ms: ::std::option::Option::None,
                    damage_window_end_ms: ::std::option::Option::None,
                    energy_delta: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .POGOProtos.Enums.PokemonMove movement_id = 1;

    pub fn clear_movement_id(&mut self) {
        self.movement_id = ::std::option::Option::None;
    }

    pub fn has_movement_id(&self) -> bool {
        self.movement_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_movement_id(&mut self, v: super::POGOProtos_Enums::PokemonMove) {
        self.movement_id = ::std::option::Option::Some(v);
    }

    pub fn get_movement_id(&self) -> super::POGOProtos_Enums::PokemonMove {
        self.movement_id.unwrap_or(super::POGOProtos_Enums::PokemonMove::MOVE_UNSET)
    }

    // optional int32 animation_id = 2;

    pub fn clear_animation_id(&mut self) {
        self.animation_id = ::std::option::Option::None;
    }

    pub fn has_animation_id(&self) -> bool {
        self.animation_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_animation_id(&mut self, v: i32) {
        self.animation_id = ::std::option::Option::Some(v);
    }

    pub fn get_animation_id(&self) -> i32 {
        self.animation_id.unwrap_or(0)
    }

    // optional .POGOProtos.Enums.PokemonType pokemon_type = 3;

    pub fn clear_pokemon_type(&mut self) {
        self.pokemon_type = ::std::option::Option::None;
    }

    pub fn has_pokemon_type(&self) -> bool {
        self.pokemon_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokemon_type(&mut self, v: super::POGOProtos_Enums::PokemonType) {
        self.pokemon_type = ::std::option::Option::Some(v);
    }

    pub fn get_pokemon_type(&self) -> super::POGOProtos_Enums::PokemonType {
        self.pokemon_type.unwrap_or(super::POGOProtos_Enums::PokemonType::POKEMON_TYPE_NONE)
    }

    // optional float power = 4;

    pub fn clear_power(&mut self) {
        self.power = ::std::option::Option::None;
    }

    pub fn has_power(&self) -> bool {
        self.power.is_some()
    }

    // Param is passed by value, moved
    pub fn set_power(&mut self, v: f32) {
        self.power = ::std::option::Option::Some(v);
    }

    pub fn get_power(&self) -> f32 {
        self.power.unwrap_or(0.)
    }

    // optional float accuracy_chance = 5;

    pub fn clear_accuracy_chance(&mut self) {
        self.accuracy_chance = ::std::option::Option::None;
    }

    pub fn has_accuracy_chance(&self) -> bool {
        self.accuracy_chance.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accuracy_chance(&mut self, v: f32) {
        self.accuracy_chance = ::std::option::Option::Some(v);
    }

    pub fn get_accuracy_chance(&self) -> f32 {
        self.accuracy_chance.unwrap_or(0.)
    }

    // optional float critical_chance = 6;

    pub fn clear_critical_chance(&mut self) {
        self.critical_chance = ::std::option::Option::None;
    }

    pub fn has_critical_chance(&self) -> bool {
        self.critical_chance.is_some()
    }

    // Param is passed by value, moved
    pub fn set_critical_chance(&mut self, v: f32) {
        self.critical_chance = ::std::option::Option::Some(v);
    }

    pub fn get_critical_chance(&self) -> f32 {
        self.critical_chance.unwrap_or(0.)
    }

    // optional float heal_scalar = 7;

    pub fn clear_heal_scalar(&mut self) {
        self.heal_scalar = ::std::option::Option::None;
    }

    pub fn has_heal_scalar(&self) -> bool {
        self.heal_scalar.is_some()
    }

    // Param is passed by value, moved
    pub fn set_heal_scalar(&mut self, v: f32) {
        self.heal_scalar = ::std::option::Option::Some(v);
    }

    pub fn get_heal_scalar(&self) -> f32 {
        self.heal_scalar.unwrap_or(0.)
    }

    // optional float stamina_loss_scalar = 8;

    pub fn clear_stamina_loss_scalar(&mut self) {
        self.stamina_loss_scalar = ::std::option::Option::None;
    }

    pub fn has_stamina_loss_scalar(&self) -> bool {
        self.stamina_loss_scalar.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stamina_loss_scalar(&mut self, v: f32) {
        self.stamina_loss_scalar = ::std::option::Option::Some(v);
    }

    pub fn get_stamina_loss_scalar(&self) -> f32 {
        self.stamina_loss_scalar.unwrap_or(0.)
    }

    // optional int32 trainer_level_min = 9;

    pub fn clear_trainer_level_min(&mut self) {
        self.trainer_level_min = ::std::option::Option::None;
    }

    pub fn has_trainer_level_min(&self) -> bool {
        self.trainer_level_min.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trainer_level_min(&mut self, v: i32) {
        self.trainer_level_min = ::std::option::Option::Some(v);
    }

    pub fn get_trainer_level_min(&self) -> i32 {
        self.trainer_level_min.unwrap_or(0)
    }

    // optional int32 trainer_level_max = 10;

    pub fn clear_trainer_level_max(&mut self) {
        self.trainer_level_max = ::std::option::Option::None;
    }

    pub fn has_trainer_level_max(&self) -> bool {
        self.trainer_level_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trainer_level_max(&mut self, v: i32) {
        self.trainer_level_max = ::std::option::Option::Some(v);
    }

    pub fn get_trainer_level_max(&self) -> i32 {
        self.trainer_level_max.unwrap_or(0)
    }

    // optional string vfx_name = 11;

    pub fn clear_vfx_name(&mut self) {
        self.vfx_name.clear();
    }

    pub fn has_vfx_name(&self) -> bool {
        self.vfx_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vfx_name(&mut self, v: ::std::string::String) {
        self.vfx_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vfx_name(&mut self) -> &mut ::std::string::String {
        if self.vfx_name.is_none() {
            self.vfx_name.set_default();
        };
        self.vfx_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_vfx_name(&mut self) -> ::std::string::String {
        self.vfx_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_vfx_name(&self) -> &str {
        match self.vfx_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 duration_ms = 12;

    pub fn clear_duration_ms(&mut self) {
        self.duration_ms = ::std::option::Option::None;
    }

    pub fn has_duration_ms(&self) -> bool {
        self.duration_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration_ms(&mut self, v: i32) {
        self.duration_ms = ::std::option::Option::Some(v);
    }

    pub fn get_duration_ms(&self) -> i32 {
        self.duration_ms.unwrap_or(0)
    }

    // optional int32 damage_window_start_ms = 13;

    pub fn clear_damage_window_start_ms(&mut self) {
        self.damage_window_start_ms = ::std::option::Option::None;
    }

    pub fn has_damage_window_start_ms(&self) -> bool {
        self.damage_window_start_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_damage_window_start_ms(&mut self, v: i32) {
        self.damage_window_start_ms = ::std::option::Option::Some(v);
    }

    pub fn get_damage_window_start_ms(&self) -> i32 {
        self.damage_window_start_ms.unwrap_or(0)
    }

    // optional int32 damage_window_end_ms = 14;

    pub fn clear_damage_window_end_ms(&mut self) {
        self.damage_window_end_ms = ::std::option::Option::None;
    }

    pub fn has_damage_window_end_ms(&self) -> bool {
        self.damage_window_end_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_damage_window_end_ms(&mut self, v: i32) {
        self.damage_window_end_ms = ::std::option::Option::Some(v);
    }

    pub fn get_damage_window_end_ms(&self) -> i32 {
        self.damage_window_end_ms.unwrap_or(0)
    }

    // optional int32 energy_delta = 15;

    pub fn clear_energy_delta(&mut self) {
        self.energy_delta = ::std::option::Option::None;
    }

    pub fn has_energy_delta(&self) -> bool {
        self.energy_delta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_energy_delta(&mut self, v: i32) {
        self.energy_delta = ::std::option::Option::Some(v);
    }

    pub fn get_energy_delta(&self) -> i32 {
        self.energy_delta.unwrap_or(0)
    }
}

impl ::protobuf::Message for MoveSettings {
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
                    self.movement_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.animation_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.pokemon_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.power = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.accuracy_chance = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.critical_chance = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.heal_scalar = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.stamina_loss_scalar = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.trainer_level_min = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.trainer_level_max = ::std::option::Option::Some(tmp);
                },
                11 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.vfx_name));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.duration_ms = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.damage_window_start_ms = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.damage_window_end_ms = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.energy_delta = ::std::option::Option::Some(tmp);
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
        for value in &self.movement_id {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.animation_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pokemon_type {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        if self.power.is_some() {
            my_size += 5;
        };
        if self.accuracy_chance.is_some() {
            my_size += 5;
        };
        if self.critical_chance.is_some() {
            my_size += 5;
        };
        if self.heal_scalar.is_some() {
            my_size += 5;
        };
        if self.stamina_loss_scalar.is_some() {
            my_size += 5;
        };
        for value in &self.trainer_level_min {
            my_size += ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.trainer_level_max {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.vfx_name {
            my_size += ::protobuf::rt::string_size(11, &value);
        };
        for value in &self.duration_ms {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.damage_window_start_ms {
            my_size += ::protobuf::rt::value_size(13, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.damage_window_end_ms {
            my_size += ::protobuf::rt::value_size(14, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.energy_delta {
            my_size += ::protobuf::rt::value_size(15, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.movement_id {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.animation_id {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.pokemon_type {
            try!(os.write_enum(3, v.value()));
        };
        if let Some(v) = self.power {
            try!(os.write_float(4, v));
        };
        if let Some(v) = self.accuracy_chance {
            try!(os.write_float(5, v));
        };
        if let Some(v) = self.critical_chance {
            try!(os.write_float(6, v));
        };
        if let Some(v) = self.heal_scalar {
            try!(os.write_float(7, v));
        };
        if let Some(v) = self.stamina_loss_scalar {
            try!(os.write_float(8, v));
        };
        if let Some(v) = self.trainer_level_min {
            try!(os.write_int32(9, v));
        };
        if let Some(v) = self.trainer_level_max {
            try!(os.write_int32(10, v));
        };
        if let Some(v) = self.vfx_name.as_ref() {
            try!(os.write_string(11, &v));
        };
        if let Some(v) = self.duration_ms {
            try!(os.write_int32(12, v));
        };
        if let Some(v) = self.damage_window_start_ms {
            try!(os.write_int32(13, v));
        };
        if let Some(v) = self.damage_window_end_ms {
            try!(os.write_int32(14, v));
        };
        if let Some(v) = self.energy_delta {
            try!(os.write_int32(15, v));
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
        ::std::any::TypeId::of::<MoveSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MoveSettings {
    fn new() -> MoveSettings {
        MoveSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<MoveSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "movement_id",
                    MoveSettings::has_movement_id,
                    MoveSettings::get_movement_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "animation_id",
                    MoveSettings::has_animation_id,
                    MoveSettings::get_animation_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "pokemon_type",
                    MoveSettings::has_pokemon_type,
                    MoveSettings::get_pokemon_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "power",
                    MoveSettings::has_power,
                    MoveSettings::get_power,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "accuracy_chance",
                    MoveSettings::has_accuracy_chance,
                    MoveSettings::get_accuracy_chance,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "critical_chance",
                    MoveSettings::has_critical_chance,
                    MoveSettings::get_critical_chance,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "heal_scalar",
                    MoveSettings::has_heal_scalar,
                    MoveSettings::get_heal_scalar,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "stamina_loss_scalar",
                    MoveSettings::has_stamina_loss_scalar,
                    MoveSettings::get_stamina_loss_scalar,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "trainer_level_min",
                    MoveSettings::has_trainer_level_min,
                    MoveSettings::get_trainer_level_min,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "trainer_level_max",
                    MoveSettings::has_trainer_level_max,
                    MoveSettings::get_trainer_level_max,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "vfx_name",
                    MoveSettings::has_vfx_name,
                    MoveSettings::get_vfx_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "duration_ms",
                    MoveSettings::has_duration_ms,
                    MoveSettings::get_duration_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "damage_window_start_ms",
                    MoveSettings::has_damage_window_start_ms,
                    MoveSettings::get_damage_window_start_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "damage_window_end_ms",
                    MoveSettings::has_damage_window_end_ms,
                    MoveSettings::get_damage_window_end_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "energy_delta",
                    MoveSettings::has_energy_delta,
                    MoveSettings::get_energy_delta,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MoveSettings>(
                    "MoveSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MoveSettings {
    fn clear(&mut self) {
        self.clear_movement_id();
        self.clear_animation_id();
        self.clear_pokemon_type();
        self.clear_power();
        self.clear_accuracy_chance();
        self.clear_critical_chance();
        self.clear_heal_scalar();
        self.clear_stamina_loss_scalar();
        self.clear_trainer_level_min();
        self.clear_trainer_level_max();
        self.clear_vfx_name();
        self.clear_duration_ms();
        self.clear_damage_window_start_ms();
        self.clear_damage_window_end_ms();
        self.clear_energy_delta();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for MoveSettings {
    fn eq(&self, other: &MoveSettings) -> bool {
        self.movement_id == other.movement_id &&
        self.animation_id == other.animation_id &&
        self.pokemon_type == other.pokemon_type &&
        self.power == other.power &&
        self.accuracy_chance == other.accuracy_chance &&
        self.critical_chance == other.critical_chance &&
        self.heal_scalar == other.heal_scalar &&
        self.stamina_loss_scalar == other.stamina_loss_scalar &&
        self.trainer_level_min == other.trainer_level_min &&
        self.trainer_level_max == other.trainer_level_max &&
        self.vfx_name == other.vfx_name &&
        self.duration_ms == other.duration_ms &&
        self.damage_window_start_ms == other.damage_window_start_ms &&
        self.damage_window_end_ms == other.damage_window_end_ms &&
        self.energy_delta == other.energy_delta &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for MoveSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PlayerLevelSettings {
    // message fields
    rank_num: ::std::vec::Vec<i32>,
    required_experience: ::std::vec::Vec<i32>,
    cp_multiplier: ::std::vec::Vec<f32>,
    max_egg_player_level: ::std::option::Option<i32>,
    max_encounter_player_level: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayerLevelSettings {}

impl PlayerLevelSettings {
    pub fn new() -> PlayerLevelSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayerLevelSettings {
        static mut instance: ::protobuf::lazy::Lazy<PlayerLevelSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayerLevelSettings,
        };
        unsafe {
            instance.get(|| {
                PlayerLevelSettings {
                    rank_num: ::std::vec::Vec::new(),
                    required_experience: ::std::vec::Vec::new(),
                    cp_multiplier: ::std::vec::Vec::new(),
                    max_egg_player_level: ::std::option::Option::None,
                    max_encounter_player_level: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated int32 rank_num = 1;

    pub fn clear_rank_num(&mut self) {
        self.rank_num.clear();
    }

    // Param is passed by value, moved
    pub fn set_rank_num(&mut self, v: ::std::vec::Vec<i32>) {
        self.rank_num = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rank_num(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.rank_num
    }

    // Take field
    pub fn take_rank_num(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.rank_num, ::std::vec::Vec::new())
    }

    pub fn get_rank_num(&self) -> &[i32] {
        &self.rank_num
    }

    // repeated int32 required_experience = 2;

    pub fn clear_required_experience(&mut self) {
        self.required_experience.clear();
    }

    // Param is passed by value, moved
    pub fn set_required_experience(&mut self, v: ::std::vec::Vec<i32>) {
        self.required_experience = v;
    }

    // Mutable pointer to the field.
    pub fn mut_required_experience(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.required_experience
    }

    // Take field
    pub fn take_required_experience(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.required_experience, ::std::vec::Vec::new())
    }

    pub fn get_required_experience(&self) -> &[i32] {
        &self.required_experience
    }

    // repeated float cp_multiplier = 3;

    pub fn clear_cp_multiplier(&mut self) {
        self.cp_multiplier.clear();
    }

    // Param is passed by value, moved
    pub fn set_cp_multiplier(&mut self, v: ::std::vec::Vec<f32>) {
        self.cp_multiplier = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cp_multiplier(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.cp_multiplier
    }

    // Take field
    pub fn take_cp_multiplier(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.cp_multiplier, ::std::vec::Vec::new())
    }

    pub fn get_cp_multiplier(&self) -> &[f32] {
        &self.cp_multiplier
    }

    // optional int32 max_egg_player_level = 4;

    pub fn clear_max_egg_player_level(&mut self) {
        self.max_egg_player_level = ::std::option::Option::None;
    }

    pub fn has_max_egg_player_level(&self) -> bool {
        self.max_egg_player_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_egg_player_level(&mut self, v: i32) {
        self.max_egg_player_level = ::std::option::Option::Some(v);
    }

    pub fn get_max_egg_player_level(&self) -> i32 {
        self.max_egg_player_level.unwrap_or(0)
    }

    // optional int32 max_encounter_player_level = 5;

    pub fn clear_max_encounter_player_level(&mut self) {
        self.max_encounter_player_level = ::std::option::Option::None;
    }

    pub fn has_max_encounter_player_level(&self) -> bool {
        self.max_encounter_player_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_encounter_player_level(&mut self, v: i32) {
        self.max_encounter_player_level = ::std::option::Option::Some(v);
    }

    pub fn get_max_encounter_player_level(&self) -> i32 {
        self.max_encounter_player_level.unwrap_or(0)
    }
}

impl ::protobuf::Message for PlayerLevelSettings {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.rank_num));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.required_experience));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.cp_multiplier));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.max_egg_player_level = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.max_encounter_player_level = ::std::option::Option::Some(tmp);
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
        for value in &self.rank_num {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.required_experience {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += 5 * self.cp_multiplier.len() as u32;
        for value in &self.max_egg_player_level {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.max_encounter_player_level {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.rank_num {
            try!(os.write_int32(1, *v));
        };
        for v in &self.required_experience {
            try!(os.write_int32(2, *v));
        };
        for v in &self.cp_multiplier {
            try!(os.write_float(3, *v));
        };
        if let Some(v) = self.max_egg_player_level {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.max_encounter_player_level {
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
        ::std::any::TypeId::of::<PlayerLevelSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PlayerLevelSettings {
    fn new() -> PlayerLevelSettings {
        PlayerLevelSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayerLevelSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "rank_num",
                    PlayerLevelSettings::get_rank_num,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "required_experience",
                    PlayerLevelSettings::get_required_experience,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "cp_multiplier",
                    PlayerLevelSettings::get_cp_multiplier,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "max_egg_player_level",
                    PlayerLevelSettings::has_max_egg_player_level,
                    PlayerLevelSettings::get_max_egg_player_level,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "max_encounter_player_level",
                    PlayerLevelSettings::has_max_encounter_player_level,
                    PlayerLevelSettings::get_max_encounter_player_level,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlayerLevelSettings>(
                    "PlayerLevelSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayerLevelSettings {
    fn clear(&mut self) {
        self.clear_rank_num();
        self.clear_required_experience();
        self.clear_cp_multiplier();
        self.clear_max_egg_player_level();
        self.clear_max_encounter_player_level();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PlayerLevelSettings {
    fn eq(&self, other: &PlayerLevelSettings) -> bool {
        self.rank_num == other.rank_num &&
        self.required_experience == other.required_experience &&
        self.cp_multiplier == other.cp_multiplier &&
        self.max_egg_player_level == other.max_egg_player_level &&
        self.max_encounter_player_level == other.max_encounter_player_level &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PlayerLevelSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct IapItemDisplay {
    // message fields
    sku: ::protobuf::SingularField<::std::string::String>,
    category: ::std::option::Option<super::POGOProtos_Enums::HoloIapItemCategory>,
    sort_order: ::std::option::Option<i32>,
    item_ids: ::std::vec::Vec<super::POGOProtos_Inventory_Item::ItemId>,
    counts: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IapItemDisplay {}

impl IapItemDisplay {
    pub fn new() -> IapItemDisplay {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IapItemDisplay {
        static mut instance: ::protobuf::lazy::Lazy<IapItemDisplay> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IapItemDisplay,
        };
        unsafe {
            instance.get(|| {
                IapItemDisplay {
                    sku: ::protobuf::SingularField::none(),
                    category: ::std::option::Option::None,
                    sort_order: ::std::option::Option::None,
                    item_ids: ::std::vec::Vec::new(),
                    counts: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string sku = 1;

    pub fn clear_sku(&mut self) {
        self.sku.clear();
    }

    pub fn has_sku(&self) -> bool {
        self.sku.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sku(&mut self, v: ::std::string::String) {
        self.sku = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sku(&mut self) -> &mut ::std::string::String {
        if self.sku.is_none() {
            self.sku.set_default();
        };
        self.sku.as_mut().unwrap()
    }

    // Take field
    pub fn take_sku(&mut self) -> ::std::string::String {
        self.sku.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sku(&self) -> &str {
        match self.sku.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .POGOProtos.Enums.HoloIapItemCategory category = 2;

    pub fn clear_category(&mut self) {
        self.category = ::std::option::Option::None;
    }

    pub fn has_category(&self) -> bool {
        self.category.is_some()
    }

    // Param is passed by value, moved
    pub fn set_category(&mut self, v: super::POGOProtos_Enums::HoloIapItemCategory) {
        self.category = ::std::option::Option::Some(v);
    }

    pub fn get_category(&self) -> super::POGOProtos_Enums::HoloIapItemCategory {
        self.category.unwrap_or(super::POGOProtos_Enums::HoloIapItemCategory::IAP_CATEGORY_NONE)
    }

    // optional int32 sort_order = 3;

    pub fn clear_sort_order(&mut self) {
        self.sort_order = ::std::option::Option::None;
    }

    pub fn has_sort_order(&self) -> bool {
        self.sort_order.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sort_order(&mut self, v: i32) {
        self.sort_order = ::std::option::Option::Some(v);
    }

    pub fn get_sort_order(&self) -> i32 {
        self.sort_order.unwrap_or(0)
    }

    // repeated .POGOProtos.Inventory.Item.ItemId item_ids = 4;

    pub fn clear_item_ids(&mut self) {
        self.item_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_item_ids(&mut self, v: ::std::vec::Vec<super::POGOProtos_Inventory_Item::ItemId>) {
        self.item_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_item_ids(&mut self) -> &mut ::std::vec::Vec<super::POGOProtos_Inventory_Item::ItemId> {
        &mut self.item_ids
    }

    // Take field
    pub fn take_item_ids(&mut self) -> ::std::vec::Vec<super::POGOProtos_Inventory_Item::ItemId> {
        ::std::mem::replace(&mut self.item_ids, ::std::vec::Vec::new())
    }

    pub fn get_item_ids(&self) -> &[super::POGOProtos_Inventory_Item::ItemId] {
        &self.item_ids
    }

    // repeated int32 counts = 5;

    pub fn clear_counts(&mut self) {
        self.counts.clear();
    }

    // Param is passed by value, moved
    pub fn set_counts(&mut self, v: ::std::vec::Vec<i32>) {
        self.counts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_counts(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.counts
    }

    // Take field
    pub fn take_counts(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.counts, ::std::vec::Vec::new())
    }

    pub fn get_counts(&self) -> &[i32] {
        &self.counts
    }
}

impl ::protobuf::Message for IapItemDisplay {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sku));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.category = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.sort_order = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.item_ids));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.counts));
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
        for value in &self.sku {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.category {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.sort_order {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.item_ids {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        for value in &self.counts {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sku.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.category {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.sort_order {
            try!(os.write_int32(3, v));
        };
        for v in &self.item_ids {
            try!(os.write_enum(4, v.value()));
        };
        for v in &self.counts {
            try!(os.write_int32(5, *v));
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
        ::std::any::TypeId::of::<IapItemDisplay>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IapItemDisplay {
    fn new() -> IapItemDisplay {
        IapItemDisplay::new()
    }

    fn descriptor_static(_: ::std::option::Option<IapItemDisplay>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "sku",
                    IapItemDisplay::has_sku,
                    IapItemDisplay::get_sku,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "category",
                    IapItemDisplay::has_category,
                    IapItemDisplay::get_category,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sort_order",
                    IapItemDisplay::has_sort_order,
                    IapItemDisplay::get_sort_order,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "item_ids",
                    IapItemDisplay::get_item_ids,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "counts",
                    IapItemDisplay::get_counts,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IapItemDisplay>(
                    "IapItemDisplay",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IapItemDisplay {
    fn clear(&mut self) {
        self.clear_sku();
        self.clear_category();
        self.clear_sort_order();
        self.clear_item_ids();
        self.clear_counts();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IapItemDisplay {
    fn eq(&self, other: &IapItemDisplay) -> bool {
        self.sku == other.sku &&
        self.category == other.category &&
        self.sort_order == other.sort_order &&
        self.item_ids == other.item_ids &&
        self.counts == other.counts &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IapItemDisplay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EncounterSettings {
    // message fields
    spin_bonus_threshold: ::std::option::Option<f32>,
    excellent_throw_threshold: ::std::option::Option<f32>,
    great_throw_threshold: ::std::option::Option<f32>,
    nice_throw_threshold: ::std::option::Option<f32>,
    milestone_threshold: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EncounterSettings {}

impl EncounterSettings {
    pub fn new() -> EncounterSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EncounterSettings {
        static mut instance: ::protobuf::lazy::Lazy<EncounterSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EncounterSettings,
        };
        unsafe {
            instance.get(|| {
                EncounterSettings {
                    spin_bonus_threshold: ::std::option::Option::None,
                    excellent_throw_threshold: ::std::option::Option::None,
                    great_throw_threshold: ::std::option::Option::None,
                    nice_throw_threshold: ::std::option::Option::None,
                    milestone_threshold: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float spin_bonus_threshold = 1;

    pub fn clear_spin_bonus_threshold(&mut self) {
        self.spin_bonus_threshold = ::std::option::Option::None;
    }

    pub fn has_spin_bonus_threshold(&self) -> bool {
        self.spin_bonus_threshold.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spin_bonus_threshold(&mut self, v: f32) {
        self.spin_bonus_threshold = ::std::option::Option::Some(v);
    }

    pub fn get_spin_bonus_threshold(&self) -> f32 {
        self.spin_bonus_threshold.unwrap_or(0.)
    }

    // optional float excellent_throw_threshold = 2;

    pub fn clear_excellent_throw_threshold(&mut self) {
        self.excellent_throw_threshold = ::std::option::Option::None;
    }

    pub fn has_excellent_throw_threshold(&self) -> bool {
        self.excellent_throw_threshold.is_some()
    }

    // Param is passed by value, moved
    pub fn set_excellent_throw_threshold(&mut self, v: f32) {
        self.excellent_throw_threshold = ::std::option::Option::Some(v);
    }

    pub fn get_excellent_throw_threshold(&self) -> f32 {
        self.excellent_throw_threshold.unwrap_or(0.)
    }

    // optional float great_throw_threshold = 3;

    pub fn clear_great_throw_threshold(&mut self) {
        self.great_throw_threshold = ::std::option::Option::None;
    }

    pub fn has_great_throw_threshold(&self) -> bool {
        self.great_throw_threshold.is_some()
    }

    // Param is passed by value, moved
    pub fn set_great_throw_threshold(&mut self, v: f32) {
        self.great_throw_threshold = ::std::option::Option::Some(v);
    }

    pub fn get_great_throw_threshold(&self) -> f32 {
        self.great_throw_threshold.unwrap_or(0.)
    }

    // optional float nice_throw_threshold = 4;

    pub fn clear_nice_throw_threshold(&mut self) {
        self.nice_throw_threshold = ::std::option::Option::None;
    }

    pub fn has_nice_throw_threshold(&self) -> bool {
        self.nice_throw_threshold.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nice_throw_threshold(&mut self, v: f32) {
        self.nice_throw_threshold = ::std::option::Option::Some(v);
    }

    pub fn get_nice_throw_threshold(&self) -> f32 {
        self.nice_throw_threshold.unwrap_or(0.)
    }

    // optional int32 milestone_threshold = 5;

    pub fn clear_milestone_threshold(&mut self) {
        self.milestone_threshold = ::std::option::Option::None;
    }

    pub fn has_milestone_threshold(&self) -> bool {
        self.milestone_threshold.is_some()
    }

    // Param is passed by value, moved
    pub fn set_milestone_threshold(&mut self, v: i32) {
        self.milestone_threshold = ::std::option::Option::Some(v);
    }

    pub fn get_milestone_threshold(&self) -> i32 {
        self.milestone_threshold.unwrap_or(0)
    }
}

impl ::protobuf::Message for EncounterSettings {
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
                    self.spin_bonus_threshold = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.excellent_throw_threshold = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.great_throw_threshold = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.nice_throw_threshold = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.milestone_threshold = ::std::option::Option::Some(tmp);
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
        if self.spin_bonus_threshold.is_some() {
            my_size += 5;
        };
        if self.excellent_throw_threshold.is_some() {
            my_size += 5;
        };
        if self.great_throw_threshold.is_some() {
            my_size += 5;
        };
        if self.nice_throw_threshold.is_some() {
            my_size += 5;
        };
        for value in &self.milestone_threshold {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.spin_bonus_threshold {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.excellent_throw_threshold {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.great_throw_threshold {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.nice_throw_threshold {
            try!(os.write_float(4, v));
        };
        if let Some(v) = self.milestone_threshold {
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
        ::std::any::TypeId::of::<EncounterSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EncounterSettings {
    fn new() -> EncounterSettings {
        EncounterSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<EncounterSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "spin_bonus_threshold",
                    EncounterSettings::has_spin_bonus_threshold,
                    EncounterSettings::get_spin_bonus_threshold,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "excellent_throw_threshold",
                    EncounterSettings::has_excellent_throw_threshold,
                    EncounterSettings::get_excellent_throw_threshold,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "great_throw_threshold",
                    EncounterSettings::has_great_throw_threshold,
                    EncounterSettings::get_great_throw_threshold,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "nice_throw_threshold",
                    EncounterSettings::has_nice_throw_threshold,
                    EncounterSettings::get_nice_throw_threshold,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "milestone_threshold",
                    EncounterSettings::has_milestone_threshold,
                    EncounterSettings::get_milestone_threshold,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EncounterSettings>(
                    "EncounterSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EncounterSettings {
    fn clear(&mut self) {
        self.clear_spin_bonus_threshold();
        self.clear_excellent_throw_threshold();
        self.clear_great_throw_threshold();
        self.clear_nice_throw_threshold();
        self.clear_milestone_threshold();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EncounterSettings {
    fn eq(&self, other: &EncounterSettings) -> bool {
        self.spin_bonus_threshold == other.spin_bonus_threshold &&
        self.excellent_throw_threshold == other.excellent_throw_threshold &&
        self.great_throw_threshold == other.great_throw_threshold &&
        self.nice_throw_threshold == other.nice_throw_threshold &&
        self.milestone_threshold == other.milestone_threshold &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EncounterSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GymLevelSettings {
    // message fields
    required_experience: ::std::vec::Vec<i32>,
    leader_slots: ::std::vec::Vec<i32>,
    trainer_slots: ::std::vec::Vec<i32>,
    search_roll_bonus: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GymLevelSettings {}

impl GymLevelSettings {
    pub fn new() -> GymLevelSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GymLevelSettings {
        static mut instance: ::protobuf::lazy::Lazy<GymLevelSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GymLevelSettings,
        };
        unsafe {
            instance.get(|| {
                GymLevelSettings {
                    required_experience: ::std::vec::Vec::new(),
                    leader_slots: ::std::vec::Vec::new(),
                    trainer_slots: ::std::vec::Vec::new(),
                    search_roll_bonus: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated int32 required_experience = 1;

    pub fn clear_required_experience(&mut self) {
        self.required_experience.clear();
    }

    // Param is passed by value, moved
    pub fn set_required_experience(&mut self, v: ::std::vec::Vec<i32>) {
        self.required_experience = v;
    }

    // Mutable pointer to the field.
    pub fn mut_required_experience(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.required_experience
    }

    // Take field
    pub fn take_required_experience(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.required_experience, ::std::vec::Vec::new())
    }

    pub fn get_required_experience(&self) -> &[i32] {
        &self.required_experience
    }

    // repeated int32 leader_slots = 2;

    pub fn clear_leader_slots(&mut self) {
        self.leader_slots.clear();
    }

    // Param is passed by value, moved
    pub fn set_leader_slots(&mut self, v: ::std::vec::Vec<i32>) {
        self.leader_slots = v;
    }

    // Mutable pointer to the field.
    pub fn mut_leader_slots(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.leader_slots
    }

    // Take field
    pub fn take_leader_slots(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.leader_slots, ::std::vec::Vec::new())
    }

    pub fn get_leader_slots(&self) -> &[i32] {
        &self.leader_slots
    }

    // repeated int32 trainer_slots = 3;

    pub fn clear_trainer_slots(&mut self) {
        self.trainer_slots.clear();
    }

    // Param is passed by value, moved
    pub fn set_trainer_slots(&mut self, v: ::std::vec::Vec<i32>) {
        self.trainer_slots = v;
    }

    // Mutable pointer to the field.
    pub fn mut_trainer_slots(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.trainer_slots
    }

    // Take field
    pub fn take_trainer_slots(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.trainer_slots, ::std::vec::Vec::new())
    }

    pub fn get_trainer_slots(&self) -> &[i32] {
        &self.trainer_slots
    }

    // repeated int32 search_roll_bonus = 4;

    pub fn clear_search_roll_bonus(&mut self) {
        self.search_roll_bonus.clear();
    }

    // Param is passed by value, moved
    pub fn set_search_roll_bonus(&mut self, v: ::std::vec::Vec<i32>) {
        self.search_roll_bonus = v;
    }

    // Mutable pointer to the field.
    pub fn mut_search_roll_bonus(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.search_roll_bonus
    }

    // Take field
    pub fn take_search_roll_bonus(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.search_roll_bonus, ::std::vec::Vec::new())
    }

    pub fn get_search_roll_bonus(&self) -> &[i32] {
        &self.search_roll_bonus
    }
}

impl ::protobuf::Message for GymLevelSettings {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.required_experience));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.leader_slots));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.trainer_slots));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.search_roll_bonus));
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
        for value in &self.required_experience {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.leader_slots {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.trainer_slots {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.search_roll_bonus {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.required_experience {
            try!(os.write_int32(1, *v));
        };
        for v in &self.leader_slots {
            try!(os.write_int32(2, *v));
        };
        for v in &self.trainer_slots {
            try!(os.write_int32(3, *v));
        };
        for v in &self.search_roll_bonus {
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
        ::std::any::TypeId::of::<GymLevelSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GymLevelSettings {
    fn new() -> GymLevelSettings {
        GymLevelSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<GymLevelSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "required_experience",
                    GymLevelSettings::get_required_experience,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "leader_slots",
                    GymLevelSettings::get_leader_slots,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "trainer_slots",
                    GymLevelSettings::get_trainer_slots,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "search_roll_bonus",
                    GymLevelSettings::get_search_roll_bonus,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GymLevelSettings>(
                    "GymLevelSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GymLevelSettings {
    fn clear(&mut self) {
        self.clear_required_experience();
        self.clear_leader_slots();
        self.clear_trainer_slots();
        self.clear_search_roll_bonus();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GymLevelSettings {
    fn eq(&self, other: &GymLevelSettings) -> bool {
        self.required_experience == other.required_experience &&
        self.leader_slots == other.leader_slots &&
        self.trainer_slots == other.trainer_slots &&
        self.search_roll_bonus == other.search_roll_bonus &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GymLevelSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EquippedBadgeSettings {
    // message fields
    equip_badge_cooldown_ms: ::std::option::Option<i64>,
    catch_probability_bonus: ::std::vec::Vec<f32>,
    flee_probability_bonus: ::std::vec::Vec<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EquippedBadgeSettings {}

impl EquippedBadgeSettings {
    pub fn new() -> EquippedBadgeSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EquippedBadgeSettings {
        static mut instance: ::protobuf::lazy::Lazy<EquippedBadgeSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EquippedBadgeSettings,
        };
        unsafe {
            instance.get(|| {
                EquippedBadgeSettings {
                    equip_badge_cooldown_ms: ::std::option::Option::None,
                    catch_probability_bonus: ::std::vec::Vec::new(),
                    flee_probability_bonus: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 equip_badge_cooldown_ms = 1;

    pub fn clear_equip_badge_cooldown_ms(&mut self) {
        self.equip_badge_cooldown_ms = ::std::option::Option::None;
    }

    pub fn has_equip_badge_cooldown_ms(&self) -> bool {
        self.equip_badge_cooldown_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_equip_badge_cooldown_ms(&mut self, v: i64) {
        self.equip_badge_cooldown_ms = ::std::option::Option::Some(v);
    }

    pub fn get_equip_badge_cooldown_ms(&self) -> i64 {
        self.equip_badge_cooldown_ms.unwrap_or(0)
    }

    // repeated float catch_probability_bonus = 2;

    pub fn clear_catch_probability_bonus(&mut self) {
        self.catch_probability_bonus.clear();
    }

    // Param is passed by value, moved
    pub fn set_catch_probability_bonus(&mut self, v: ::std::vec::Vec<f32>) {
        self.catch_probability_bonus = v;
    }

    // Mutable pointer to the field.
    pub fn mut_catch_probability_bonus(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.catch_probability_bonus
    }

    // Take field
    pub fn take_catch_probability_bonus(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.catch_probability_bonus, ::std::vec::Vec::new())
    }

    pub fn get_catch_probability_bonus(&self) -> &[f32] {
        &self.catch_probability_bonus
    }

    // repeated float flee_probability_bonus = 3;

    pub fn clear_flee_probability_bonus(&mut self) {
        self.flee_probability_bonus.clear();
    }

    // Param is passed by value, moved
    pub fn set_flee_probability_bonus(&mut self, v: ::std::vec::Vec<f32>) {
        self.flee_probability_bonus = v;
    }

    // Mutable pointer to the field.
    pub fn mut_flee_probability_bonus(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.flee_probability_bonus
    }

    // Take field
    pub fn take_flee_probability_bonus(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.flee_probability_bonus, ::std::vec::Vec::new())
    }

    pub fn get_flee_probability_bonus(&self) -> &[f32] {
        &self.flee_probability_bonus
    }
}

impl ::protobuf::Message for EquippedBadgeSettings {
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
                    self.equip_badge_cooldown_ms = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.catch_probability_bonus));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.flee_probability_bonus));
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
        for value in &self.equip_badge_cooldown_ms {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += 5 * self.catch_probability_bonus.len() as u32;
        my_size += 5 * self.flee_probability_bonus.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.equip_badge_cooldown_ms {
            try!(os.write_int64(1, v));
        };
        for v in &self.catch_probability_bonus {
            try!(os.write_float(2, *v));
        };
        for v in &self.flee_probability_bonus {
            try!(os.write_float(3, *v));
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
        ::std::any::TypeId::of::<EquippedBadgeSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EquippedBadgeSettings {
    fn new() -> EquippedBadgeSettings {
        EquippedBadgeSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<EquippedBadgeSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "equip_badge_cooldown_ms",
                    EquippedBadgeSettings::has_equip_badge_cooldown_ms,
                    EquippedBadgeSettings::get_equip_badge_cooldown_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "catch_probability_bonus",
                    EquippedBadgeSettings::get_catch_probability_bonus,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "flee_probability_bonus",
                    EquippedBadgeSettings::get_flee_probability_bonus,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EquippedBadgeSettings>(
                    "EquippedBadgeSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EquippedBadgeSettings {
    fn clear(&mut self) {
        self.clear_equip_badge_cooldown_ms();
        self.clear_catch_probability_bonus();
        self.clear_flee_probability_bonus();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EquippedBadgeSettings {
    fn eq(&self, other: &EquippedBadgeSettings) -> bool {
        self.equip_badge_cooldown_ms == other.equip_badge_cooldown_ms &&
        self.catch_probability_bonus == other.catch_probability_bonus &&
        self.flee_probability_bonus == other.flee_probability_bonus &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EquippedBadgeSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct MoveSequenceSettings {
    // message fields
    sequence: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MoveSequenceSettings {}

impl MoveSequenceSettings {
    pub fn new() -> MoveSequenceSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MoveSequenceSettings {
        static mut instance: ::protobuf::lazy::Lazy<MoveSequenceSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MoveSequenceSettings,
        };
        unsafe {
            instance.get(|| {
                MoveSequenceSettings {
                    sequence: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string sequence = 1;

    pub fn clear_sequence(&mut self) {
        self.sequence.clear();
    }

    // Param is passed by value, moved
    pub fn set_sequence(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.sequence = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sequence(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.sequence
    }

    // Take field
    pub fn take_sequence(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.sequence, ::protobuf::RepeatedField::new())
    }

    pub fn get_sequence(&self) -> &[::std::string::String] {
        &self.sequence
    }
}

impl ::protobuf::Message for MoveSequenceSettings {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.sequence));
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
        for value in &self.sequence {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.sequence {
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
        ::std::any::TypeId::of::<MoveSequenceSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MoveSequenceSettings {
    fn new() -> MoveSequenceSettings {
        MoveSequenceSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<MoveSequenceSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "sequence",
                    MoveSequenceSettings::get_sequence,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MoveSequenceSettings>(
                    "MoveSequenceSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MoveSequenceSettings {
    fn clear(&mut self) {
        self.clear_sequence();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for MoveSequenceSettings {
    fn eq(&self, other: &MoveSequenceSettings) -> bool {
        self.sequence == other.sequence &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for MoveSequenceSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BadgeSettings {
    // message fields
    badge_type: ::std::option::Option<super::POGOProtos_Enums::BadgeType>,
    badge_rank: ::std::option::Option<i32>,
    targets: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BadgeSettings {}

impl BadgeSettings {
    pub fn new() -> BadgeSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BadgeSettings {
        static mut instance: ::protobuf::lazy::Lazy<BadgeSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BadgeSettings,
        };
        unsafe {
            instance.get(|| {
                BadgeSettings {
                    badge_type: ::std::option::Option::None,
                    badge_rank: ::std::option::Option::None,
                    targets: ::std::vec::Vec::new(),
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

    // optional int32 badge_rank = 2;

    pub fn clear_badge_rank(&mut self) {
        self.badge_rank = ::std::option::Option::None;
    }

    pub fn has_badge_rank(&self) -> bool {
        self.badge_rank.is_some()
    }

    // Param is passed by value, moved
    pub fn set_badge_rank(&mut self, v: i32) {
        self.badge_rank = ::std::option::Option::Some(v);
    }

    pub fn get_badge_rank(&self) -> i32 {
        self.badge_rank.unwrap_or(0)
    }

    // repeated int32 targets = 3;

    pub fn clear_targets(&mut self) {
        self.targets.clear();
    }

    // Param is passed by value, moved
    pub fn set_targets(&mut self, v: ::std::vec::Vec<i32>) {
        self.targets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targets(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.targets
    }

    // Take field
    pub fn take_targets(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.targets, ::std::vec::Vec::new())
    }

    pub fn get_targets(&self) -> &[i32] {
        &self.targets
    }
}

impl ::protobuf::Message for BadgeSettings {
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
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.badge_rank = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.targets));
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
        for value in &self.badge_rank {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.targets {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.badge_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.badge_rank {
            try!(os.write_int32(2, v));
        };
        for v in &self.targets {
            try!(os.write_int32(3, *v));
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
        ::std::any::TypeId::of::<BadgeSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BadgeSettings {
    fn new() -> BadgeSettings {
        BadgeSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<BadgeSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "badge_type",
                    BadgeSettings::has_badge_type,
                    BadgeSettings::get_badge_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "badge_rank",
                    BadgeSettings::has_badge_rank,
                    BadgeSettings::get_badge_rank,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "targets",
                    BadgeSettings::get_targets,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BadgeSettings>(
                    "BadgeSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BadgeSettings {
    fn clear(&mut self) {
        self.clear_badge_type();
        self.clear_badge_rank();
        self.clear_targets();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BadgeSettings {
    fn eq(&self, other: &BadgeSettings) -> bool {
        self.badge_type == other.badge_type &&
        self.badge_rank == other.badge_rank &&
        self.targets == other.targets &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BadgeSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GymBattleSettings {
    // message fields
    energy_per_sec: ::std::option::Option<f32>,
    dodge_energy_cost: ::std::option::Option<f32>,
    retarget_seconds: ::std::option::Option<f32>,
    enemy_attack_interval: ::std::option::Option<f32>,
    attack_server_interval: ::std::option::Option<f32>,
    round_duration_seconds: ::std::option::Option<f32>,
    bonus_time_per_ally_seconds: ::std::option::Option<f32>,
    maximum_attackers_per_battle: ::std::option::Option<i32>,
    same_type_attack_bonus_multiplier: ::std::option::Option<f32>,
    maximum_energy: ::std::option::Option<i32>,
    energy_delta_per_health_lost: ::std::option::Option<f32>,
    dodge_duration_ms: ::std::option::Option<i32>,
    minimum_player_level: ::std::option::Option<i32>,
    swap_duration_ms: ::std::option::Option<i32>,
    dodge_damage_reduction_percent: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GymBattleSettings {}

impl GymBattleSettings {
    pub fn new() -> GymBattleSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GymBattleSettings {
        static mut instance: ::protobuf::lazy::Lazy<GymBattleSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GymBattleSettings,
        };
        unsafe {
            instance.get(|| {
                GymBattleSettings {
                    energy_per_sec: ::std::option::Option::None,
                    dodge_energy_cost: ::std::option::Option::None,
                    retarget_seconds: ::std::option::Option::None,
                    enemy_attack_interval: ::std::option::Option::None,
                    attack_server_interval: ::std::option::Option::None,
                    round_duration_seconds: ::std::option::Option::None,
                    bonus_time_per_ally_seconds: ::std::option::Option::None,
                    maximum_attackers_per_battle: ::std::option::Option::None,
                    same_type_attack_bonus_multiplier: ::std::option::Option::None,
                    maximum_energy: ::std::option::Option::None,
                    energy_delta_per_health_lost: ::std::option::Option::None,
                    dodge_duration_ms: ::std::option::Option::None,
                    minimum_player_level: ::std::option::Option::None,
                    swap_duration_ms: ::std::option::Option::None,
                    dodge_damage_reduction_percent: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float energy_per_sec = 1;

    pub fn clear_energy_per_sec(&mut self) {
        self.energy_per_sec = ::std::option::Option::None;
    }

    pub fn has_energy_per_sec(&self) -> bool {
        self.energy_per_sec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_energy_per_sec(&mut self, v: f32) {
        self.energy_per_sec = ::std::option::Option::Some(v);
    }

    pub fn get_energy_per_sec(&self) -> f32 {
        self.energy_per_sec.unwrap_or(0.)
    }

    // optional float dodge_energy_cost = 2;

    pub fn clear_dodge_energy_cost(&mut self) {
        self.dodge_energy_cost = ::std::option::Option::None;
    }

    pub fn has_dodge_energy_cost(&self) -> bool {
        self.dodge_energy_cost.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dodge_energy_cost(&mut self, v: f32) {
        self.dodge_energy_cost = ::std::option::Option::Some(v);
    }

    pub fn get_dodge_energy_cost(&self) -> f32 {
        self.dodge_energy_cost.unwrap_or(0.)
    }

    // optional float retarget_seconds = 3;

    pub fn clear_retarget_seconds(&mut self) {
        self.retarget_seconds = ::std::option::Option::None;
    }

    pub fn has_retarget_seconds(&self) -> bool {
        self.retarget_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_retarget_seconds(&mut self, v: f32) {
        self.retarget_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_retarget_seconds(&self) -> f32 {
        self.retarget_seconds.unwrap_or(0.)
    }

    // optional float enemy_attack_interval = 4;

    pub fn clear_enemy_attack_interval(&mut self) {
        self.enemy_attack_interval = ::std::option::Option::None;
    }

    pub fn has_enemy_attack_interval(&self) -> bool {
        self.enemy_attack_interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enemy_attack_interval(&mut self, v: f32) {
        self.enemy_attack_interval = ::std::option::Option::Some(v);
    }

    pub fn get_enemy_attack_interval(&self) -> f32 {
        self.enemy_attack_interval.unwrap_or(0.)
    }

    // optional float attack_server_interval = 5;

    pub fn clear_attack_server_interval(&mut self) {
        self.attack_server_interval = ::std::option::Option::None;
    }

    pub fn has_attack_server_interval(&self) -> bool {
        self.attack_server_interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attack_server_interval(&mut self, v: f32) {
        self.attack_server_interval = ::std::option::Option::Some(v);
    }

    pub fn get_attack_server_interval(&self) -> f32 {
        self.attack_server_interval.unwrap_or(0.)
    }

    // optional float round_duration_seconds = 6;

    pub fn clear_round_duration_seconds(&mut self) {
        self.round_duration_seconds = ::std::option::Option::None;
    }

    pub fn has_round_duration_seconds(&self) -> bool {
        self.round_duration_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_round_duration_seconds(&mut self, v: f32) {
        self.round_duration_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_round_duration_seconds(&self) -> f32 {
        self.round_duration_seconds.unwrap_or(0.)
    }

    // optional float bonus_time_per_ally_seconds = 7;

    pub fn clear_bonus_time_per_ally_seconds(&mut self) {
        self.bonus_time_per_ally_seconds = ::std::option::Option::None;
    }

    pub fn has_bonus_time_per_ally_seconds(&self) -> bool {
        self.bonus_time_per_ally_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bonus_time_per_ally_seconds(&mut self, v: f32) {
        self.bonus_time_per_ally_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_bonus_time_per_ally_seconds(&self) -> f32 {
        self.bonus_time_per_ally_seconds.unwrap_or(0.)
    }

    // optional int32 maximum_attackers_per_battle = 8;

    pub fn clear_maximum_attackers_per_battle(&mut self) {
        self.maximum_attackers_per_battle = ::std::option::Option::None;
    }

    pub fn has_maximum_attackers_per_battle(&self) -> bool {
        self.maximum_attackers_per_battle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maximum_attackers_per_battle(&mut self, v: i32) {
        self.maximum_attackers_per_battle = ::std::option::Option::Some(v);
    }

    pub fn get_maximum_attackers_per_battle(&self) -> i32 {
        self.maximum_attackers_per_battle.unwrap_or(0)
    }

    // optional float same_type_attack_bonus_multiplier = 9;

    pub fn clear_same_type_attack_bonus_multiplier(&mut self) {
        self.same_type_attack_bonus_multiplier = ::std::option::Option::None;
    }

    pub fn has_same_type_attack_bonus_multiplier(&self) -> bool {
        self.same_type_attack_bonus_multiplier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_same_type_attack_bonus_multiplier(&mut self, v: f32) {
        self.same_type_attack_bonus_multiplier = ::std::option::Option::Some(v);
    }

    pub fn get_same_type_attack_bonus_multiplier(&self) -> f32 {
        self.same_type_attack_bonus_multiplier.unwrap_or(0.)
    }

    // optional int32 maximum_energy = 10;

    pub fn clear_maximum_energy(&mut self) {
        self.maximum_energy = ::std::option::Option::None;
    }

    pub fn has_maximum_energy(&self) -> bool {
        self.maximum_energy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maximum_energy(&mut self, v: i32) {
        self.maximum_energy = ::std::option::Option::Some(v);
    }

    pub fn get_maximum_energy(&self) -> i32 {
        self.maximum_energy.unwrap_or(0)
    }

    // optional float energy_delta_per_health_lost = 11;

    pub fn clear_energy_delta_per_health_lost(&mut self) {
        self.energy_delta_per_health_lost = ::std::option::Option::None;
    }

    pub fn has_energy_delta_per_health_lost(&self) -> bool {
        self.energy_delta_per_health_lost.is_some()
    }

    // Param is passed by value, moved
    pub fn set_energy_delta_per_health_lost(&mut self, v: f32) {
        self.energy_delta_per_health_lost = ::std::option::Option::Some(v);
    }

    pub fn get_energy_delta_per_health_lost(&self) -> f32 {
        self.energy_delta_per_health_lost.unwrap_or(0.)
    }

    // optional int32 dodge_duration_ms = 12;

    pub fn clear_dodge_duration_ms(&mut self) {
        self.dodge_duration_ms = ::std::option::Option::None;
    }

    pub fn has_dodge_duration_ms(&self) -> bool {
        self.dodge_duration_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dodge_duration_ms(&mut self, v: i32) {
        self.dodge_duration_ms = ::std::option::Option::Some(v);
    }

    pub fn get_dodge_duration_ms(&self) -> i32 {
        self.dodge_duration_ms.unwrap_or(0)
    }

    // optional int32 minimum_player_level = 13;

    pub fn clear_minimum_player_level(&mut self) {
        self.minimum_player_level = ::std::option::Option::None;
    }

    pub fn has_minimum_player_level(&self) -> bool {
        self.minimum_player_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minimum_player_level(&mut self, v: i32) {
        self.minimum_player_level = ::std::option::Option::Some(v);
    }

    pub fn get_minimum_player_level(&self) -> i32 {
        self.minimum_player_level.unwrap_or(0)
    }

    // optional int32 swap_duration_ms = 14;

    pub fn clear_swap_duration_ms(&mut self) {
        self.swap_duration_ms = ::std::option::Option::None;
    }

    pub fn has_swap_duration_ms(&self) -> bool {
        self.swap_duration_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_swap_duration_ms(&mut self, v: i32) {
        self.swap_duration_ms = ::std::option::Option::Some(v);
    }

    pub fn get_swap_duration_ms(&self) -> i32 {
        self.swap_duration_ms.unwrap_or(0)
    }

    // optional float dodge_damage_reduction_percent = 15;

    pub fn clear_dodge_damage_reduction_percent(&mut self) {
        self.dodge_damage_reduction_percent = ::std::option::Option::None;
    }

    pub fn has_dodge_damage_reduction_percent(&self) -> bool {
        self.dodge_damage_reduction_percent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dodge_damage_reduction_percent(&mut self, v: f32) {
        self.dodge_damage_reduction_percent = ::std::option::Option::Some(v);
    }

    pub fn get_dodge_damage_reduction_percent(&self) -> f32 {
        self.dodge_damage_reduction_percent.unwrap_or(0.)
    }
}

impl ::protobuf::Message for GymBattleSettings {
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
                    self.energy_per_sec = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.dodge_energy_cost = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.retarget_seconds = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.enemy_attack_interval = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.attack_server_interval = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.round_duration_seconds = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.bonus_time_per_ally_seconds = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.maximum_attackers_per_battle = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.same_type_attack_bonus_multiplier = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.maximum_energy = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.energy_delta_per_health_lost = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.dodge_duration_ms = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.minimum_player_level = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.swap_duration_ms = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.dodge_damage_reduction_percent = ::std::option::Option::Some(tmp);
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
        if self.energy_per_sec.is_some() {
            my_size += 5;
        };
        if self.dodge_energy_cost.is_some() {
            my_size += 5;
        };
        if self.retarget_seconds.is_some() {
            my_size += 5;
        };
        if self.enemy_attack_interval.is_some() {
            my_size += 5;
        };
        if self.attack_server_interval.is_some() {
            my_size += 5;
        };
        if self.round_duration_seconds.is_some() {
            my_size += 5;
        };
        if self.bonus_time_per_ally_seconds.is_some() {
            my_size += 5;
        };
        for value in &self.maximum_attackers_per_battle {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.same_type_attack_bonus_multiplier.is_some() {
            my_size += 5;
        };
        for value in &self.maximum_energy {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.energy_delta_per_health_lost.is_some() {
            my_size += 5;
        };
        for value in &self.dodge_duration_ms {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.minimum_player_level {
            my_size += ::protobuf::rt::value_size(13, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.swap_duration_ms {
            my_size += ::protobuf::rt::value_size(14, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.dodge_damage_reduction_percent.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.energy_per_sec {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.dodge_energy_cost {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.retarget_seconds {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.enemy_attack_interval {
            try!(os.write_float(4, v));
        };
        if let Some(v) = self.attack_server_interval {
            try!(os.write_float(5, v));
        };
        if let Some(v) = self.round_duration_seconds {
            try!(os.write_float(6, v));
        };
        if let Some(v) = self.bonus_time_per_ally_seconds {
            try!(os.write_float(7, v));
        };
        if let Some(v) = self.maximum_attackers_per_battle {
            try!(os.write_int32(8, v));
        };
        if let Some(v) = self.same_type_attack_bonus_multiplier {
            try!(os.write_float(9, v));
        };
        if let Some(v) = self.maximum_energy {
            try!(os.write_int32(10, v));
        };
        if let Some(v) = self.energy_delta_per_health_lost {
            try!(os.write_float(11, v));
        };
        if let Some(v) = self.dodge_duration_ms {
            try!(os.write_int32(12, v));
        };
        if let Some(v) = self.minimum_player_level {
            try!(os.write_int32(13, v));
        };
        if let Some(v) = self.swap_duration_ms {
            try!(os.write_int32(14, v));
        };
        if let Some(v) = self.dodge_damage_reduction_percent {
            try!(os.write_float(15, v));
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
        ::std::any::TypeId::of::<GymBattleSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GymBattleSettings {
    fn new() -> GymBattleSettings {
        GymBattleSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<GymBattleSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "energy_per_sec",
                    GymBattleSettings::has_energy_per_sec,
                    GymBattleSettings::get_energy_per_sec,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "dodge_energy_cost",
                    GymBattleSettings::has_dodge_energy_cost,
                    GymBattleSettings::get_dodge_energy_cost,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "retarget_seconds",
                    GymBattleSettings::has_retarget_seconds,
                    GymBattleSettings::get_retarget_seconds,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "enemy_attack_interval",
                    GymBattleSettings::has_enemy_attack_interval,
                    GymBattleSettings::get_enemy_attack_interval,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "attack_server_interval",
                    GymBattleSettings::has_attack_server_interval,
                    GymBattleSettings::get_attack_server_interval,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "round_duration_seconds",
                    GymBattleSettings::has_round_duration_seconds,
                    GymBattleSettings::get_round_duration_seconds,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "bonus_time_per_ally_seconds",
                    GymBattleSettings::has_bonus_time_per_ally_seconds,
                    GymBattleSettings::get_bonus_time_per_ally_seconds,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "maximum_attackers_per_battle",
                    GymBattleSettings::has_maximum_attackers_per_battle,
                    GymBattleSettings::get_maximum_attackers_per_battle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "same_type_attack_bonus_multiplier",
                    GymBattleSettings::has_same_type_attack_bonus_multiplier,
                    GymBattleSettings::get_same_type_attack_bonus_multiplier,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "maximum_energy",
                    GymBattleSettings::has_maximum_energy,
                    GymBattleSettings::get_maximum_energy,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "energy_delta_per_health_lost",
                    GymBattleSettings::has_energy_delta_per_health_lost,
                    GymBattleSettings::get_energy_delta_per_health_lost,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "dodge_duration_ms",
                    GymBattleSettings::has_dodge_duration_ms,
                    GymBattleSettings::get_dodge_duration_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "minimum_player_level",
                    GymBattleSettings::has_minimum_player_level,
                    GymBattleSettings::get_minimum_player_level,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "swap_duration_ms",
                    GymBattleSettings::has_swap_duration_ms,
                    GymBattleSettings::get_swap_duration_ms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "dodge_damage_reduction_percent",
                    GymBattleSettings::has_dodge_damage_reduction_percent,
                    GymBattleSettings::get_dodge_damage_reduction_percent,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GymBattleSettings>(
                    "GymBattleSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GymBattleSettings {
    fn clear(&mut self) {
        self.clear_energy_per_sec();
        self.clear_dodge_energy_cost();
        self.clear_retarget_seconds();
        self.clear_enemy_attack_interval();
        self.clear_attack_server_interval();
        self.clear_round_duration_seconds();
        self.clear_bonus_time_per_ally_seconds();
        self.clear_maximum_attackers_per_battle();
        self.clear_same_type_attack_bonus_multiplier();
        self.clear_maximum_energy();
        self.clear_energy_delta_per_health_lost();
        self.clear_dodge_duration_ms();
        self.clear_minimum_player_level();
        self.clear_swap_duration_ms();
        self.clear_dodge_damage_reduction_percent();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GymBattleSettings {
    fn eq(&self, other: &GymBattleSettings) -> bool {
        self.energy_per_sec == other.energy_per_sec &&
        self.dodge_energy_cost == other.dodge_energy_cost &&
        self.retarget_seconds == other.retarget_seconds &&
        self.enemy_attack_interval == other.enemy_attack_interval &&
        self.attack_server_interval == other.attack_server_interval &&
        self.round_duration_seconds == other.round_duration_seconds &&
        self.bonus_time_per_ally_seconds == other.bonus_time_per_ally_seconds &&
        self.maximum_attackers_per_battle == other.maximum_attackers_per_battle &&
        self.same_type_attack_bonus_multiplier == other.same_type_attack_bonus_multiplier &&
        self.maximum_energy == other.maximum_energy &&
        self.energy_delta_per_health_lost == other.energy_delta_per_health_lost &&
        self.dodge_duration_ms == other.dodge_duration_ms &&
        self.minimum_player_level == other.minimum_player_level &&
        self.swap_duration_ms == other.swap_duration_ms &&
        self.dodge_damage_reduction_percent == other.dodge_damage_reduction_percent &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GymBattleSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PokemonSettings {
    // message fields
    pokemon_id: ::std::option::Option<super::POGOProtos_Enums::PokemonId>,
    model_scale: ::std::option::Option<f32>,
    field_type: ::std::option::Option<super::POGOProtos_Enums::PokemonType>,
    type_2: ::std::option::Option<super::POGOProtos_Enums::PokemonType>,
    camera: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master_Pokemon::CameraAttributes>,
    encounter: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master_Pokemon::EncounterAttributes>,
    stats: ::protobuf::SingularPtrField<super::POGOProtos_Settings_Master_Pokemon::StatsAttributes>,
    quick_moves: ::std::vec::Vec<super::POGOProtos_Enums::PokemonMove>,
    cinematic_moves: ::std::vec::Vec<super::POGOProtos_Enums::PokemonMove>,
    animation_time: ::std::vec::Vec<f32>,
    evolution_ids: ::std::vec::Vec<super::POGOProtos_Enums::PokemonId>,
    evolution_pips: ::std::option::Option<i32>,
    rarity: ::std::option::Option<super::POGOProtos_Enums::PokemonRarity>,
    pokedex_height_m: ::std::option::Option<f32>,
    pokedex_weight_kg: ::std::option::Option<f32>,
    parent_pokemon_id: ::std::option::Option<super::POGOProtos_Enums::PokemonId>,
    height_std_dev: ::std::option::Option<f32>,
    weight_std_dev: ::std::option::Option<f32>,
    km_distance_to_hatch: ::std::option::Option<f32>,
    family_id: ::std::option::Option<super::POGOProtos_Enums::PokemonFamilyId>,
    candy_to_evolve: ::std::option::Option<i32>,
    km_buddy_distance: ::std::option::Option<f32>,
    buddy_size: ::std::option::Option<PokemonSettings_BuddySize>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PokemonSettings {}

impl PokemonSettings {
    pub fn new() -> PokemonSettings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PokemonSettings {
        static mut instance: ::protobuf::lazy::Lazy<PokemonSettings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PokemonSettings,
        };
        unsafe {
            instance.get(|| {
                PokemonSettings {
                    pokemon_id: ::std::option::Option::None,
                    model_scale: ::std::option::Option::None,
                    field_type: ::std::option::Option::None,
                    type_2: ::std::option::Option::None,
                    camera: ::protobuf::SingularPtrField::none(),
                    encounter: ::protobuf::SingularPtrField::none(),
                    stats: ::protobuf::SingularPtrField::none(),
                    quick_moves: ::std::vec::Vec::new(),
                    cinematic_moves: ::std::vec::Vec::new(),
                    animation_time: ::std::vec::Vec::new(),
                    evolution_ids: ::std::vec::Vec::new(),
                    evolution_pips: ::std::option::Option::None,
                    rarity: ::std::option::Option::None,
                    pokedex_height_m: ::std::option::Option::None,
                    pokedex_weight_kg: ::std::option::Option::None,
                    parent_pokemon_id: ::std::option::Option::None,
                    height_std_dev: ::std::option::Option::None,
                    weight_std_dev: ::std::option::Option::None,
                    km_distance_to_hatch: ::std::option::Option::None,
                    family_id: ::std::option::Option::None,
                    candy_to_evolve: ::std::option::Option::None,
                    km_buddy_distance: ::std::option::Option::None,
                    buddy_size: ::std::option::Option::None,
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

    // optional float model_scale = 3;

    pub fn clear_model_scale(&mut self) {
        self.model_scale = ::std::option::Option::None;
    }

    pub fn has_model_scale(&self) -> bool {
        self.model_scale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_model_scale(&mut self, v: f32) {
        self.model_scale = ::std::option::Option::Some(v);
    }

    pub fn get_model_scale(&self) -> f32 {
        self.model_scale.unwrap_or(0.)
    }

    // optional .POGOProtos.Enums.PokemonType type = 4;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: super::POGOProtos_Enums::PokemonType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> super::POGOProtos_Enums::PokemonType {
        self.field_type.unwrap_or(super::POGOProtos_Enums::PokemonType::POKEMON_TYPE_NONE)
    }

    // optional .POGOProtos.Enums.PokemonType type_2 = 5;

    pub fn clear_type_2(&mut self) {
        self.type_2 = ::std::option::Option::None;
    }

    pub fn has_type_2(&self) -> bool {
        self.type_2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_type_2(&mut self, v: super::POGOProtos_Enums::PokemonType) {
        self.type_2 = ::std::option::Option::Some(v);
    }

    pub fn get_type_2(&self) -> super::POGOProtos_Enums::PokemonType {
        self.type_2.unwrap_or(super::POGOProtos_Enums::PokemonType::POKEMON_TYPE_NONE)
    }

    // optional .POGOProtos.Settings.Master.Pokemon.CameraAttributes camera = 6;

    pub fn clear_camera(&mut self) {
        self.camera.clear();
    }

    pub fn has_camera(&self) -> bool {
        self.camera.is_some()
    }

    // Param is passed by value, moved
    pub fn set_camera(&mut self, v: super::POGOProtos_Settings_Master_Pokemon::CameraAttributes) {
        self.camera = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_camera(&mut self) -> &mut super::POGOProtos_Settings_Master_Pokemon::CameraAttributes {
        if self.camera.is_none() {
            self.camera.set_default();
        };
        self.camera.as_mut().unwrap()
    }

    // Take field
    pub fn take_camera(&mut self) -> super::POGOProtos_Settings_Master_Pokemon::CameraAttributes {
        self.camera.take().unwrap_or_else(|| super::POGOProtos_Settings_Master_Pokemon::CameraAttributes::new())
    }

    pub fn get_camera(&self) -> &super::POGOProtos_Settings_Master_Pokemon::CameraAttributes {
        self.camera.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master_Pokemon::CameraAttributes::default_instance())
    }

    // optional .POGOProtos.Settings.Master.Pokemon.EncounterAttributes encounter = 7;

    pub fn clear_encounter(&mut self) {
        self.encounter.clear();
    }

    pub fn has_encounter(&self) -> bool {
        self.encounter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encounter(&mut self, v: super::POGOProtos_Settings_Master_Pokemon::EncounterAttributes) {
        self.encounter = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encounter(&mut self) -> &mut super::POGOProtos_Settings_Master_Pokemon::EncounterAttributes {
        if self.encounter.is_none() {
            self.encounter.set_default();
        };
        self.encounter.as_mut().unwrap()
    }

    // Take field
    pub fn take_encounter(&mut self) -> super::POGOProtos_Settings_Master_Pokemon::EncounterAttributes {
        self.encounter.take().unwrap_or_else(|| super::POGOProtos_Settings_Master_Pokemon::EncounterAttributes::new())
    }

    pub fn get_encounter(&self) -> &super::POGOProtos_Settings_Master_Pokemon::EncounterAttributes {
        self.encounter.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master_Pokemon::EncounterAttributes::default_instance())
    }

    // optional .POGOProtos.Settings.Master.Pokemon.StatsAttributes stats = 8;

    pub fn clear_stats(&mut self) {
        self.stats.clear();
    }

    pub fn has_stats(&self) -> bool {
        self.stats.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stats(&mut self, v: super::POGOProtos_Settings_Master_Pokemon::StatsAttributes) {
        self.stats = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stats(&mut self) -> &mut super::POGOProtos_Settings_Master_Pokemon::StatsAttributes {
        if self.stats.is_none() {
            self.stats.set_default();
        };
        self.stats.as_mut().unwrap()
    }

    // Take field
    pub fn take_stats(&mut self) -> super::POGOProtos_Settings_Master_Pokemon::StatsAttributes {
        self.stats.take().unwrap_or_else(|| super::POGOProtos_Settings_Master_Pokemon::StatsAttributes::new())
    }

    pub fn get_stats(&self) -> &super::POGOProtos_Settings_Master_Pokemon::StatsAttributes {
        self.stats.as_ref().unwrap_or_else(|| super::POGOProtos_Settings_Master_Pokemon::StatsAttributes::default_instance())
    }

    // repeated .POGOProtos.Enums.PokemonMove quick_moves = 9;

    pub fn clear_quick_moves(&mut self) {
        self.quick_moves.clear();
    }

    // Param is passed by value, moved
    pub fn set_quick_moves(&mut self, v: ::std::vec::Vec<super::POGOProtos_Enums::PokemonMove>) {
        self.quick_moves = v;
    }

    // Mutable pointer to the field.
    pub fn mut_quick_moves(&mut self) -> &mut ::std::vec::Vec<super::POGOProtos_Enums::PokemonMove> {
        &mut self.quick_moves
    }

    // Take field
    pub fn take_quick_moves(&mut self) -> ::std::vec::Vec<super::POGOProtos_Enums::PokemonMove> {
        ::std::mem::replace(&mut self.quick_moves, ::std::vec::Vec::new())
    }

    pub fn get_quick_moves(&self) -> &[super::POGOProtos_Enums::PokemonMove] {
        &self.quick_moves
    }

    // repeated .POGOProtos.Enums.PokemonMove cinematic_moves = 10;

    pub fn clear_cinematic_moves(&mut self) {
        self.cinematic_moves.clear();
    }

    // Param is passed by value, moved
    pub fn set_cinematic_moves(&mut self, v: ::std::vec::Vec<super::POGOProtos_Enums::PokemonMove>) {
        self.cinematic_moves = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cinematic_moves(&mut self) -> &mut ::std::vec::Vec<super::POGOProtos_Enums::PokemonMove> {
        &mut self.cinematic_moves
    }

    // Take field
    pub fn take_cinematic_moves(&mut self) -> ::std::vec::Vec<super::POGOProtos_Enums::PokemonMove> {
        ::std::mem::replace(&mut self.cinematic_moves, ::std::vec::Vec::new())
    }

    pub fn get_cinematic_moves(&self) -> &[super::POGOProtos_Enums::PokemonMove] {
        &self.cinematic_moves
    }

    // repeated float animation_time = 11;

    pub fn clear_animation_time(&mut self) {
        self.animation_time.clear();
    }

    // Param is passed by value, moved
    pub fn set_animation_time(&mut self, v: ::std::vec::Vec<f32>) {
        self.animation_time = v;
    }

    // Mutable pointer to the field.
    pub fn mut_animation_time(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.animation_time
    }

    // Take field
    pub fn take_animation_time(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.animation_time, ::std::vec::Vec::new())
    }

    pub fn get_animation_time(&self) -> &[f32] {
        &self.animation_time
    }

    // repeated .POGOProtos.Enums.PokemonId evolution_ids = 12;

    pub fn clear_evolution_ids(&mut self) {
        self.evolution_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_evolution_ids(&mut self, v: ::std::vec::Vec<super::POGOProtos_Enums::PokemonId>) {
        self.evolution_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_evolution_ids(&mut self) -> &mut ::std::vec::Vec<super::POGOProtos_Enums::PokemonId> {
        &mut self.evolution_ids
    }

    // Take field
    pub fn take_evolution_ids(&mut self) -> ::std::vec::Vec<super::POGOProtos_Enums::PokemonId> {
        ::std::mem::replace(&mut self.evolution_ids, ::std::vec::Vec::new())
    }

    pub fn get_evolution_ids(&self) -> &[super::POGOProtos_Enums::PokemonId] {
        &self.evolution_ids
    }

    // optional int32 evolution_pips = 13;

    pub fn clear_evolution_pips(&mut self) {
        self.evolution_pips = ::std::option::Option::None;
    }

    pub fn has_evolution_pips(&self) -> bool {
        self.evolution_pips.is_some()
    }

    // Param is passed by value, moved
    pub fn set_evolution_pips(&mut self, v: i32) {
        self.evolution_pips = ::std::option::Option::Some(v);
    }

    pub fn get_evolution_pips(&self) -> i32 {
        self.evolution_pips.unwrap_or(0)
    }

    // optional .POGOProtos.Enums.PokemonRarity rarity = 14;

    pub fn clear_rarity(&mut self) {
        self.rarity = ::std::option::Option::None;
    }

    pub fn has_rarity(&self) -> bool {
        self.rarity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rarity(&mut self, v: super::POGOProtos_Enums::PokemonRarity) {
        self.rarity = ::std::option::Option::Some(v);
    }

    pub fn get_rarity(&self) -> super::POGOProtos_Enums::PokemonRarity {
        self.rarity.unwrap_or(super::POGOProtos_Enums::PokemonRarity::POKEMON_RARITY_NORMAL)
    }

    // optional float pokedex_height_m = 15;

    pub fn clear_pokedex_height_m(&mut self) {
        self.pokedex_height_m = ::std::option::Option::None;
    }

    pub fn has_pokedex_height_m(&self) -> bool {
        self.pokedex_height_m.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokedex_height_m(&mut self, v: f32) {
        self.pokedex_height_m = ::std::option::Option::Some(v);
    }

    pub fn get_pokedex_height_m(&self) -> f32 {
        self.pokedex_height_m.unwrap_or(0.)
    }

    // optional float pokedex_weight_kg = 16;

    pub fn clear_pokedex_weight_kg(&mut self) {
        self.pokedex_weight_kg = ::std::option::Option::None;
    }

    pub fn has_pokedex_weight_kg(&self) -> bool {
        self.pokedex_weight_kg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pokedex_weight_kg(&mut self, v: f32) {
        self.pokedex_weight_kg = ::std::option::Option::Some(v);
    }

    pub fn get_pokedex_weight_kg(&self) -> f32 {
        self.pokedex_weight_kg.unwrap_or(0.)
    }

    // optional .POGOProtos.Enums.PokemonId parent_pokemon_id = 17;

    pub fn clear_parent_pokemon_id(&mut self) {
        self.parent_pokemon_id = ::std::option::Option::None;
    }

    pub fn has_parent_pokemon_id(&self) -> bool {
        self.parent_pokemon_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent_pokemon_id(&mut self, v: super::POGOProtos_Enums::PokemonId) {
        self.parent_pokemon_id = ::std::option::Option::Some(v);
    }

    pub fn get_parent_pokemon_id(&self) -> super::POGOProtos_Enums::PokemonId {
        self.parent_pokemon_id.unwrap_or(super::POGOProtos_Enums::PokemonId::MISSINGNO)
    }

    // optional float height_std_dev = 18;

    pub fn clear_height_std_dev(&mut self) {
        self.height_std_dev = ::std::option::Option::None;
    }

    pub fn has_height_std_dev(&self) -> bool {
        self.height_std_dev.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height_std_dev(&mut self, v: f32) {
        self.height_std_dev = ::std::option::Option::Some(v);
    }

    pub fn get_height_std_dev(&self) -> f32 {
        self.height_std_dev.unwrap_or(0.)
    }

    // optional float weight_std_dev = 19;

    pub fn clear_weight_std_dev(&mut self) {
        self.weight_std_dev = ::std::option::Option::None;
    }

    pub fn has_weight_std_dev(&self) -> bool {
        self.weight_std_dev.is_some()
    }

    // Param is passed by value, moved
    pub fn set_weight_std_dev(&mut self, v: f32) {
        self.weight_std_dev = ::std::option::Option::Some(v);
    }

    pub fn get_weight_std_dev(&self) -> f32 {
        self.weight_std_dev.unwrap_or(0.)
    }

    // optional float km_distance_to_hatch = 20;

    pub fn clear_km_distance_to_hatch(&mut self) {
        self.km_distance_to_hatch = ::std::option::Option::None;
    }

    pub fn has_km_distance_to_hatch(&self) -> bool {
        self.km_distance_to_hatch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_km_distance_to_hatch(&mut self, v: f32) {
        self.km_distance_to_hatch = ::std::option::Option::Some(v);
    }

    pub fn get_km_distance_to_hatch(&self) -> f32 {
        self.km_distance_to_hatch.unwrap_or(0.)
    }

    // optional .POGOProtos.Enums.PokemonFamilyId family_id = 21;

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

    // optional int32 candy_to_evolve = 22;

    pub fn clear_candy_to_evolve(&mut self) {
        self.candy_to_evolve = ::std::option::Option::None;
    }

    pub fn has_candy_to_evolve(&self) -> bool {
        self.candy_to_evolve.is_some()
    }

    // Param is passed by value, moved
    pub fn set_candy_to_evolve(&mut self, v: i32) {
        self.candy_to_evolve = ::std::option::Option::Some(v);
    }

    pub fn get_candy_to_evolve(&self) -> i32 {
        self.candy_to_evolve.unwrap_or(0)
    }

    // optional float km_buddy_distance = 23;

    pub fn clear_km_buddy_distance(&mut self) {
        self.km_buddy_distance = ::std::option::Option::None;
    }

    pub fn has_km_buddy_distance(&self) -> bool {
        self.km_buddy_distance.is_some()
    }

    // Param is passed by value, moved
    pub fn set_km_buddy_distance(&mut self, v: f32) {
        self.km_buddy_distance = ::std::option::Option::Some(v);
    }

    pub fn get_km_buddy_distance(&self) -> f32 {
        self.km_buddy_distance.unwrap_or(0.)
    }

    // optional .POGOProtos.Settings.Master.PokemonSettings.BuddySize buddy_size = 24;

    pub fn clear_buddy_size(&mut self) {
        self.buddy_size = ::std::option::Option::None;
    }

    pub fn has_buddy_size(&self) -> bool {
        self.buddy_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buddy_size(&mut self, v: PokemonSettings_BuddySize) {
        self.buddy_size = ::std::option::Option::Some(v);
    }

    pub fn get_buddy_size(&self) -> PokemonSettings_BuddySize {
        self.buddy_size.unwrap_or(PokemonSettings_BuddySize::BUDDY_MEDIUM)
    }
}

impl ::protobuf::Message for PokemonSettings {
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
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.model_scale = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.type_2 = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.camera));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.encounter));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stats));
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.quick_moves));
                },
                10 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.cinematic_moves));
                },
                11 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.animation_time));
                },
                12 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.evolution_ids));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.evolution_pips = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.rarity = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.pokedex_height_m = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.pokedex_weight_kg = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.parent_pokemon_id = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.height_std_dev = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.weight_std_dev = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.km_distance_to_hatch = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.family_id = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.candy_to_evolve = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.km_buddy_distance = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.buddy_size = ::std::option::Option::Some(tmp);
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
        if self.model_scale.is_some() {
            my_size += 5;
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        for value in &self.type_2 {
            my_size += ::protobuf::rt::enum_size(5, *value);
        };
        for value in &self.camera {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.encounter {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.stats {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.quick_moves {
            my_size += ::protobuf::rt::enum_size(9, *value);
        };
        for value in &self.cinematic_moves {
            my_size += ::protobuf::rt::enum_size(10, *value);
        };
        my_size += 5 * self.animation_time.len() as u32;
        for value in &self.evolution_ids {
            my_size += ::protobuf::rt::enum_size(12, *value);
        };
        for value in &self.evolution_pips {
            my_size += ::protobuf::rt::value_size(13, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.rarity {
            my_size += ::protobuf::rt::enum_size(14, *value);
        };
        if self.pokedex_height_m.is_some() {
            my_size += 5;
        };
        if self.pokedex_weight_kg.is_some() {
            my_size += 6;
        };
        for value in &self.parent_pokemon_id {
            my_size += ::protobuf::rt::enum_size(17, *value);
        };
        if self.height_std_dev.is_some() {
            my_size += 6;
        };
        if self.weight_std_dev.is_some() {
            my_size += 6;
        };
        if self.km_distance_to_hatch.is_some() {
            my_size += 6;
        };
        for value in &self.family_id {
            my_size += ::protobuf::rt::enum_size(21, *value);
        };
        for value in &self.candy_to_evolve {
            my_size += ::protobuf::rt::value_size(22, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.km_buddy_distance.is_some() {
            my_size += 6;
        };
        for value in &self.buddy_size {
            my_size += ::protobuf::rt::enum_size(24, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pokemon_id {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.model_scale {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.field_type {
            try!(os.write_enum(4, v.value()));
        };
        if let Some(v) = self.type_2 {
            try!(os.write_enum(5, v.value()));
        };
        if let Some(v) = self.camera.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.encounter.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.stats.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.quick_moves {
            try!(os.write_enum(9, v.value()));
        };
        for v in &self.cinematic_moves {
            try!(os.write_enum(10, v.value()));
        };
        for v in &self.animation_time {
            try!(os.write_float(11, *v));
        };
        for v in &self.evolution_ids {
            try!(os.write_enum(12, v.value()));
        };
        if let Some(v) = self.evolution_pips {
            try!(os.write_int32(13, v));
        };
        if let Some(v) = self.rarity {
            try!(os.write_enum(14, v.value()));
        };
        if let Some(v) = self.pokedex_height_m {
            try!(os.write_float(15, v));
        };
        if let Some(v) = self.pokedex_weight_kg {
            try!(os.write_float(16, v));
        };
        if let Some(v) = self.parent_pokemon_id {
            try!(os.write_enum(17, v.value()));
        };
        if let Some(v) = self.height_std_dev {
            try!(os.write_float(18, v));
        };
        if let Some(v) = self.weight_std_dev {
            try!(os.write_float(19, v));
        };
        if let Some(v) = self.km_distance_to_hatch {
            try!(os.write_float(20, v));
        };
        if let Some(v) = self.family_id {
            try!(os.write_enum(21, v.value()));
        };
        if let Some(v) = self.candy_to_evolve {
            try!(os.write_int32(22, v));
        };
        if let Some(v) = self.km_buddy_distance {
            try!(os.write_float(23, v));
        };
        if let Some(v) = self.buddy_size {
            try!(os.write_enum(24, v.value()));
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
        ::std::any::TypeId::of::<PokemonSettings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PokemonSettings {
    fn new() -> PokemonSettings {
        PokemonSettings::new()
    }

    fn descriptor_static(_: ::std::option::Option<PokemonSettings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "pokemon_id",
                    PokemonSettings::has_pokemon_id,
                    PokemonSettings::get_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "model_scale",
                    PokemonSettings::has_model_scale,
                    PokemonSettings::get_model_scale,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    PokemonSettings::has_field_type,
                    PokemonSettings::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type_2",
                    PokemonSettings::has_type_2,
                    PokemonSettings::get_type_2,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "camera",
                    PokemonSettings::has_camera,
                    PokemonSettings::get_camera,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "encounter",
                    PokemonSettings::has_encounter,
                    PokemonSettings::get_encounter,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stats",
                    PokemonSettings::has_stats,
                    PokemonSettings::get_stats,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "quick_moves",
                    PokemonSettings::get_quick_moves,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "cinematic_moves",
                    PokemonSettings::get_cinematic_moves,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "animation_time",
                    PokemonSettings::get_animation_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "evolution_ids",
                    PokemonSettings::get_evolution_ids,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "evolution_pips",
                    PokemonSettings::has_evolution_pips,
                    PokemonSettings::get_evolution_pips,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "rarity",
                    PokemonSettings::has_rarity,
                    PokemonSettings::get_rarity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "pokedex_height_m",
                    PokemonSettings::has_pokedex_height_m,
                    PokemonSettings::get_pokedex_height_m,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "pokedex_weight_kg",
                    PokemonSettings::has_pokedex_weight_kg,
                    PokemonSettings::get_pokedex_weight_kg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "parent_pokemon_id",
                    PokemonSettings::has_parent_pokemon_id,
                    PokemonSettings::get_parent_pokemon_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "height_std_dev",
                    PokemonSettings::has_height_std_dev,
                    PokemonSettings::get_height_std_dev,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "weight_std_dev",
                    PokemonSettings::has_weight_std_dev,
                    PokemonSettings::get_weight_std_dev,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "km_distance_to_hatch",
                    PokemonSettings::has_km_distance_to_hatch,
                    PokemonSettings::get_km_distance_to_hatch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "family_id",
                    PokemonSettings::has_family_id,
                    PokemonSettings::get_family_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "candy_to_evolve",
                    PokemonSettings::has_candy_to_evolve,
                    PokemonSettings::get_candy_to_evolve,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "km_buddy_distance",
                    PokemonSettings::has_km_buddy_distance,
                    PokemonSettings::get_km_buddy_distance,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "buddy_size",
                    PokemonSettings::has_buddy_size,
                    PokemonSettings::get_buddy_size,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PokemonSettings>(
                    "PokemonSettings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PokemonSettings {
    fn clear(&mut self) {
        self.clear_pokemon_id();
        self.clear_model_scale();
        self.clear_field_type();
        self.clear_type_2();
        self.clear_camera();
        self.clear_encounter();
        self.clear_stats();
        self.clear_quick_moves();
        self.clear_cinematic_moves();
        self.clear_animation_time();
        self.clear_evolution_ids();
        self.clear_evolution_pips();
        self.clear_rarity();
        self.clear_pokedex_height_m();
        self.clear_pokedex_weight_kg();
        self.clear_parent_pokemon_id();
        self.clear_height_std_dev();
        self.clear_weight_std_dev();
        self.clear_km_distance_to_hatch();
        self.clear_family_id();
        self.clear_candy_to_evolve();
        self.clear_km_buddy_distance();
        self.clear_buddy_size();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PokemonSettings {
    fn eq(&self, other: &PokemonSettings) -> bool {
        self.pokemon_id == other.pokemon_id &&
        self.model_scale == other.model_scale &&
        self.field_type == other.field_type &&
        self.type_2 == other.type_2 &&
        self.camera == other.camera &&
        self.encounter == other.encounter &&
        self.stats == other.stats &&
        self.quick_moves == other.quick_moves &&
        self.cinematic_moves == other.cinematic_moves &&
        self.animation_time == other.animation_time &&
        self.evolution_ids == other.evolution_ids &&
        self.evolution_pips == other.evolution_pips &&
        self.rarity == other.rarity &&
        self.pokedex_height_m == other.pokedex_height_m &&
        self.pokedex_weight_kg == other.pokedex_weight_kg &&
        self.parent_pokemon_id == other.parent_pokemon_id &&
        self.height_std_dev == other.height_std_dev &&
        self.weight_std_dev == other.weight_std_dev &&
        self.km_distance_to_hatch == other.km_distance_to_hatch &&
        self.family_id == other.family_id &&
        self.candy_to_evolve == other.candy_to_evolve &&
        self.km_buddy_distance == other.km_buddy_distance &&
        self.buddy_size == other.buddy_size &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PokemonSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum PokemonSettings_BuddySize {
    BUDDY_MEDIUM = 0,
    BUDDY_SHOULDER = 1,
    BUDDY_BIG = 2,
    BUDDY_FLYING = 3,
}

impl ::protobuf::ProtobufEnum for PokemonSettings_BuddySize {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PokemonSettings_BuddySize> {
        match value {
            0 => ::std::option::Option::Some(PokemonSettings_BuddySize::BUDDY_MEDIUM),
            1 => ::std::option::Option::Some(PokemonSettings_BuddySize::BUDDY_SHOULDER),
            2 => ::std::option::Option::Some(PokemonSettings_BuddySize::BUDDY_BIG),
            3 => ::std::option::Option::Some(PokemonSettings_BuddySize::BUDDY_FLYING),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [PokemonSettings_BuddySize] = &[
            PokemonSettings_BuddySize::BUDDY_MEDIUM,
            PokemonSettings_BuddySize::BUDDY_SHOULDER,
            PokemonSettings_BuddySize::BUDDY_BIG,
            PokemonSettings_BuddySize::BUDDY_FLYING,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<PokemonSettings_BuddySize>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("PokemonSettings_BuddySize", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for PokemonSettings_BuddySize {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x20, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74,
    0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x1a, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53,
    0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x1a, 0x16,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65,
    0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x25, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x28,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69,
    0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x00, 0x50, 0x01, 0x50, 0x02, 0x50, 0x03,
    0x22, 0x7c, 0x0a, 0x15, 0x54, 0x79, 0x70, 0x65, 0x45, 0x66, 0x66, 0x65, 0x63, 0x74, 0x69, 0x76,
    0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x23, 0x0a, 0x0d, 0x61, 0x74, 0x74,
    0x61, 0x63, 0x6b, 0x5f, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x18, 0x01, 0x20, 0x03, 0x28, 0x02,
    0x52, 0x0c, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x53, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x12, 0x3e,
    0x0a, 0x0b, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x54, 0x79,
    0x70, 0x65, 0x52, 0x0a, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x54, 0x79, 0x70, 0x65, 0x22, 0xc9,
    0x01, 0x0a, 0x16, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64,
    0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x2c, 0x0a, 0x12, 0x75, 0x70, 0x67,
    0x72, 0x61, 0x64, 0x65, 0x73, 0x5f, 0x70, 0x65, 0x72, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x10, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x73, 0x50,
    0x65, 0x72, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x3d, 0x0a, 0x1b, 0x61, 0x6c, 0x6c, 0x6f, 0x77,
    0x65, 0x64, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x73, 0x5f, 0x61, 0x62, 0x6f, 0x76, 0x65, 0x5f,
    0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x18, 0x61, 0x6c,
    0x6c, 0x6f, 0x77, 0x65, 0x64, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x73, 0x41, 0x62, 0x6f, 0x76, 0x65,
    0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x61, 0x6e, 0x64, 0x79, 0x5f,
    0x63, 0x6f, 0x73, 0x74, 0x18, 0x03, 0x20, 0x03, 0x28, 0x05, 0x52, 0x09, 0x63, 0x61, 0x6e, 0x64,
    0x79, 0x43, 0x6f, 0x73, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x74, 0x61, 0x72, 0x64, 0x75, 0x73,
    0x74, 0x5f, 0x63, 0x6f, 0x73, 0x74, 0x18, 0x04, 0x20, 0x03, 0x28, 0x05, 0x52, 0x0c, 0x73, 0x74,
    0x61, 0x72, 0x64, 0x75, 0x73, 0x74, 0x43, 0x6f, 0x73, 0x74, 0x22, 0xd5, 0x08, 0x0a, 0x0c, 0x49,
    0x74, 0x65, 0x6d, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x3a, 0x0a, 0x07, 0x69,
    0x74, 0x65, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x21, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74,
    0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x52,
    0x06, 0x69, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x12, 0x40, 0x0a, 0x09, 0x69, 0x74, 0x65, 0x6d, 0x5f,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x23, 0x2e, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72,
    0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x54, 0x79, 0x70, 0x65, 0x52,
    0x08, 0x69, 0x74, 0x65, 0x6d, 0x54, 0x79, 0x70, 0x65, 0x12, 0x3a, 0x0a, 0x08, 0x63, 0x61, 0x74,
    0x65, 0x67, 0x6f, 0x72, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1e, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x49,
    0x74, 0x65, 0x6d, 0x43, 0x61, 0x74, 0x65, 0x67, 0x6f, 0x72, 0x79, 0x52, 0x08, 0x63, 0x61, 0x74,
    0x65, 0x67, 0x6f, 0x72, 0x79, 0x12, 0x1b, 0x0a, 0x09, 0x64, 0x72, 0x6f, 0x70, 0x5f, 0x66, 0x72,
    0x65, 0x71, 0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x52, 0x08, 0x64, 0x72, 0x6f, 0x70, 0x46, 0x72,
    0x65, 0x71, 0x12, 0x2c, 0x0a, 0x12, 0x64, 0x72, 0x6f, 0x70, 0x5f, 0x74, 0x72, 0x61, 0x69, 0x6e,
    0x65, 0x72, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x52, 0x10,
    0x64, 0x72, 0x6f, 0x70, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x4c, 0x65, 0x76, 0x65, 0x6c,
    0x12, 0x4f, 0x0a, 0x08, 0x70, 0x6f, 0x6b, 0x65, 0x62, 0x61, 0x6c, 0x6c, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x33, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e,
    0x49, 0x74, 0x65, 0x6d, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x62, 0x61, 0x6c, 0x6c, 0x41, 0x74, 0x74,
    0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x52, 0x08, 0x70, 0x6f, 0x6b, 0x65, 0x62, 0x61, 0x6c,
    0x6c, 0x12, 0x49, 0x0a, 0x06, 0x70, 0x6f, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x31, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53,
    0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x49,
    0x74, 0x65, 0x6d, 0x2e, 0x50, 0x6f, 0x74, 0x69, 0x6f, 0x6e, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62,
    0x75, 0x74, 0x65, 0x73, 0x52, 0x06, 0x70, 0x6f, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x49, 0x0a, 0x06,
    0x72, 0x65, 0x76, 0x69, 0x76, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x31, 0x2e, 0x50,
    0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e,
    0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x52,
    0x65, 0x76, 0x69, 0x76, 0x65, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x52,
    0x06, 0x72, 0x65, 0x76, 0x69, 0x76, 0x65, 0x12, 0x49, 0x0a, 0x06, 0x62, 0x61, 0x74, 0x74, 0x6c,
    0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x31, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61,
    0x73, 0x74, 0x65, 0x72, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65,
    0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x52, 0x06, 0x62, 0x61, 0x74, 0x74,
    0x6c, 0x65, 0x12, 0x43, 0x0a, 0x04, 0x66, 0x6f, 0x6f, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x2f, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65,
    0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x49, 0x74,
    0x65, 0x6d, 0x2e, 0x46, 0x6f, 0x6f, 0x64, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65,
    0x73, 0x52, 0x04, 0x66, 0x6f, 0x6f, 0x64, 0x12, 0x68, 0x0a, 0x11, 0x69, 0x6e, 0x76, 0x65, 0x6e,
    0x74, 0x6f, 0x72, 0x79, 0x5f, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x18, 0x0b, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x3b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e,
    0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x55, 0x70,
    0x67, 0x72, 0x61, 0x64, 0x65, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x52,
    0x10, 0x69, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64,
    0x65, 0x12, 0x55, 0x0a, 0x08, 0x78, 0x70, 0x5f, 0x62, 0x6f, 0x6f, 0x73, 0x74, 0x18, 0x0c, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x3a, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x45, 0x78, 0x70, 0x65, 0x72, 0x69, 0x65, 0x6e, 0x63, 0x65,
    0x42, 0x6f, 0x6f, 0x73, 0x74, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x52,
    0x07, 0x78, 0x70, 0x42, 0x6f, 0x6f, 0x73, 0x74, 0x12, 0x4c, 0x0a, 0x07, 0x69, 0x6e, 0x63, 0x65,
    0x6e, 0x73, 0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e,
    0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x49, 0x6e, 0x63, 0x65,
    0x6e, 0x73, 0x65, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x52, 0x07, 0x69,
    0x6e, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x12, 0x5c, 0x0a, 0x0d, 0x65, 0x67, 0x67, 0x5f, 0x69, 0x6e,
    0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x37, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69,
    0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e,
    0x45, 0x67, 0x67, 0x49, 0x6e, 0x63, 0x75, 0x62, 0x61, 0x74, 0x6f, 0x72, 0x41, 0x74, 0x74, 0x72,
    0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x52, 0x0c, 0x65, 0x67, 0x67, 0x49, 0x6e, 0x63, 0x75, 0x62,
    0x61, 0x74, 0x6f, 0x72, 0x12, 0x5c, 0x0a, 0x0d, 0x66, 0x6f, 0x72, 0x74, 0x5f, 0x6d, 0x6f, 0x64,
    0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x37, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67,
    0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x49, 0x74, 0x65, 0x6d, 0x2e, 0x46, 0x6f,
    0x72, 0x74, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62,
    0x75, 0x74, 0x65, 0x73, 0x52, 0x0c, 0x66, 0x6f, 0x72, 0x74, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69,
    0x65, 0x72, 0x22, 0xbd, 0x03, 0x0a, 0x0b, 0x49, 0x61, 0x70, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e,
    0x67, 0x73, 0x12, 0x2a, 0x0a, 0x11, 0x64, 0x61, 0x69, 0x6c, 0x79, 0x5f, 0x62, 0x6f, 0x6e, 0x75,
    0x73, 0x5f, 0x63, 0x6f, 0x69, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0f, 0x64,
    0x61, 0x69, 0x6c, 0x79, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x43, 0x6f, 0x69, 0x6e, 0x73, 0x12, 0x46,
    0x0a, 0x20, 0x64, 0x61, 0x69, 0x6c, 0x79, 0x5f, 0x64, 0x65, 0x66, 0x65, 0x6e, 0x64, 0x65, 0x72,
    0x5f, 0x62, 0x6f, 0x6e, 0x75, 0x73, 0x5f, 0x70, 0x65, 0x72, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x18, 0x02, 0x20, 0x03, 0x28, 0x05, 0x52, 0x1c, 0x64, 0x61, 0x69, 0x6c, 0x79, 0x44,
    0x65, 0x66, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x50, 0x65, 0x72, 0x50,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x12, 0x4a, 0x0a, 0x22, 0x64, 0x61, 0x69, 0x6c, 0x79, 0x5f,
    0x64, 0x65, 0x66, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x5f, 0x62, 0x6f, 0x6e, 0x75, 0x73, 0x5f, 0x6d,
    0x61, 0x78, 0x5f, 0x64, 0x65, 0x66, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x73, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x1e, 0x64, 0x61, 0x69, 0x6c, 0x79, 0x44, 0x65, 0x66, 0x65, 0x6e, 0x64, 0x65,
    0x72, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x4d, 0x61, 0x78, 0x44, 0x65, 0x66, 0x65, 0x6e, 0x64, 0x65,
    0x72, 0x73, 0x12, 0x41, 0x0a, 0x1d, 0x64, 0x61, 0x69, 0x6c, 0x79, 0x5f, 0x64, 0x65, 0x66, 0x65,
    0x6e, 0x64, 0x65, 0x72, 0x5f, 0x62, 0x6f, 0x6e, 0x75, 0x73, 0x5f, 0x63, 0x75, 0x72, 0x72, 0x65,
    0x6e, 0x63, 0x79, 0x18, 0x04, 0x20, 0x03, 0x28, 0x09, 0x52, 0x1a, 0x64, 0x61, 0x69, 0x6c, 0x79,
    0x44, 0x65, 0x66, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x43, 0x75, 0x72,
    0x72, 0x65, 0x6e, 0x63, 0x79, 0x12, 0x3a, 0x0a, 0x1a, 0x6d, 0x69, 0x6e, 0x5f, 0x74, 0x69, 0x6d,
    0x65, 0x5f, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x5f, 0x63, 0x6c, 0x61, 0x69, 0x6d, 0x73,
    0x5f, 0x6d, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x03, 0x52, 0x16, 0x6d, 0x69, 0x6e, 0x54, 0x69,
    0x6d, 0x65, 0x42, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x43, 0x6c, 0x61, 0x69, 0x6d, 0x73, 0x4d,
    0x73, 0x12, 0x2e, 0x0a, 0x13, 0x64, 0x61, 0x69, 0x6c, 0x79, 0x5f, 0x62, 0x6f, 0x6e, 0x75, 0x73,
    0x5f, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x52, 0x11,
    0x64, 0x61, 0x69, 0x6c, 0x79, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65,
    0x64, 0x12, 0x3f, 0x0a, 0x1c, 0x64, 0x61, 0x69, 0x6c, 0x79, 0x5f, 0x64, 0x65, 0x66, 0x65, 0x6e,
    0x64, 0x65, 0x72, 0x5f, 0x62, 0x6f, 0x6e, 0x75, 0x73, 0x5f, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65,
    0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x19, 0x64, 0x61, 0x69, 0x6c, 0x79, 0x44, 0x65,
    0x66, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x45, 0x6e, 0x61, 0x62, 0x6c,
    0x65, 0x64, 0x22, 0xc3, 0x05, 0x0a, 0x0e, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x53, 0x65, 0x74,
    0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x6e, 0x65, 0x78, 0x74, 0x5f, 0x63, 0x61,
    0x6d, 0x65, 0x72, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x6e, 0x65, 0x78, 0x74,
    0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x12, 0x4b, 0x0a, 0x0d, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x70,
    0x6f, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x25, 0x2e,
    0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73,
    0x2e, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x70, 0x6f, 0x6c, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0d, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x70, 0x6f, 0x6c, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x12, 0x3f, 0x0a, 0x0b, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x1e, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x43, 0x61, 0x6d, 0x65,
    0x72, 0x61, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74, 0x52, 0x0a, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x22, 0x0a, 0x0d, 0x65, 0x61, 0x73, 0x65, 0x5f, 0x69, 0x6e, 0x5f,
    0x73, 0x70, 0x65, 0x65, 0x64, 0x18, 0x04, 0x20, 0x03, 0x28, 0x02, 0x52, 0x0b, 0x65, 0x61, 0x73,
    0x65, 0x49, 0x6e, 0x53, 0x70, 0x65, 0x65, 0x64, 0x12, 0x24, 0x0a, 0x0e, 0x65, 0x61, 0x73, 0x74,
    0x5f, 0x6f, 0x75, 0x74, 0x5f, 0x73, 0x70, 0x65, 0x65, 0x64, 0x18, 0x05, 0x20, 0x03, 0x28, 0x02,
    0x52, 0x0c, 0x65, 0x61, 0x73, 0x74, 0x4f, 0x75, 0x74, 0x53, 0x70, 0x65, 0x65, 0x64, 0x12, 0x29,
    0x0a, 0x10, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e,
    0x64, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x02, 0x52, 0x0f, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x12, 0x21, 0x0a, 0x0c, 0x77, 0x61, 0x69,
    0x74, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x07, 0x20, 0x03, 0x28, 0x02, 0x52,
    0x0b, 0x77, 0x61, 0x69, 0x74, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x12, 0x2d, 0x0a, 0x12,
    0x74, 0x72, 0x61, 0x6e, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e,
    0x64, 0x73, 0x18, 0x08, 0x20, 0x03, 0x28, 0x02, 0x52, 0x11, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x12, 0x21, 0x0a, 0x0c, 0x61,
    0x6e, 0x67, 0x6c, 0x65, 0x5f, 0x64, 0x65, 0x67, 0x72, 0x65, 0x65, 0x18, 0x09, 0x20, 0x03, 0x28,
    0x02, 0x52, 0x0b, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x44, 0x65, 0x67, 0x72, 0x65, 0x65, 0x12, 0x2e,
    0x0a, 0x13, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x5f, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x5f, 0x64,
    0x65, 0x67, 0x72, 0x65, 0x65, 0x18, 0x0a, 0x20, 0x03, 0x28, 0x02, 0x52, 0x11, 0x61, 0x6e, 0x67,
    0x6c, 0x65, 0x4f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x44, 0x65, 0x67, 0x72, 0x65, 0x65, 0x12, 0x21,
    0x0a, 0x0c, 0x70, 0x69, 0x74, 0x63, 0x68, 0x5f, 0x64, 0x65, 0x67, 0x72, 0x65, 0x65, 0x18, 0x0b,
    0x20, 0x03, 0x28, 0x02, 0x52, 0x0b, 0x70, 0x69, 0x74, 0x63, 0x68, 0x44, 0x65, 0x67, 0x72, 0x65,
    0x65, 0x12, 0x2e, 0x0a, 0x13, 0x70, 0x69, 0x74, 0x63, 0x68, 0x5f, 0x6f, 0x66, 0x66, 0x73, 0x65,
    0x74, 0x5f, 0x64, 0x65, 0x67, 0x72, 0x65, 0x65, 0x18, 0x0c, 0x20, 0x03, 0x28, 0x02, 0x52, 0x11,
    0x70, 0x69, 0x74, 0x63, 0x68, 0x4f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x44, 0x65, 0x67, 0x72, 0x65,
    0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x72, 0x6f, 0x6c, 0x6c, 0x5f, 0x64, 0x65, 0x67, 0x72, 0x65, 0x65,
    0x18, 0x0d, 0x20, 0x03, 0x28, 0x02, 0x52, 0x0a, 0x72, 0x6f, 0x6c, 0x6c, 0x44, 0x65, 0x67, 0x72,
    0x65, 0x65, 0x12, 0x27, 0x0a, 0x0f, 0x64, 0x69, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x5f, 0x6d,
    0x65, 0x74, 0x65, 0x72, 0x73, 0x18, 0x0e, 0x20, 0x03, 0x28, 0x02, 0x52, 0x0e, 0x64, 0x69, 0x73,
    0x74, 0x61, 0x6e, 0x63, 0x65, 0x4d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x12, 0x25, 0x0a, 0x0e, 0x68,
    0x65, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x18, 0x0f, 0x20,
    0x03, 0x28, 0x02, 0x52, 0x0d, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x50, 0x65, 0x72, 0x63, 0x65,
    0x6e, 0x74, 0x12, 0x24, 0x0a, 0x0e, 0x76, 0x65, 0x72, 0x74, 0x5f, 0x63, 0x74, 0x72, 0x5f, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x18, 0x10, 0x20, 0x03, 0x28, 0x02, 0x52, 0x0c, 0x76, 0x65, 0x72, 0x74,
    0x43, 0x74, 0x72, 0x52, 0x61, 0x74, 0x69, 0x6f, 0x22, 0x89, 0x05, 0x0a, 0x0c, 0x4d, 0x6f, 0x76,
    0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x3e, 0x0a, 0x0b, 0x6d, 0x6f, 0x76,
    0x65, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1d,
    0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d,
    0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x4d, 0x6f, 0x76, 0x65, 0x52, 0x0a, 0x6d,
    0x6f, 0x76, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x61, 0x6e, 0x69,
    0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x0b, 0x61, 0x6e, 0x69, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x40, 0x0a, 0x0c,
    0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x54, 0x79, 0x70,
    0x65, 0x52, 0x0b, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x14,
    0x0a, 0x05, 0x70, 0x6f, 0x77, 0x65, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x70,
    0x6f, 0x77, 0x65, 0x72, 0x12, 0x27, 0x0a, 0x0f, 0x61, 0x63, 0x63, 0x75, 0x72, 0x61, 0x63, 0x79,
    0x5f, 0x63, 0x68, 0x61, 0x6e, 0x63, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0e, 0x61,
    0x63, 0x63, 0x75, 0x72, 0x61, 0x63, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x63, 0x65, 0x12, 0x27, 0x0a,
    0x0f, 0x63, 0x72, 0x69, 0x74, 0x69, 0x63, 0x61, 0x6c, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x63, 0x65,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0e, 0x63, 0x72, 0x69, 0x74, 0x69, 0x63, 0x61, 0x6c,
    0x43, 0x68, 0x61, 0x6e, 0x63, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x68, 0x65, 0x61, 0x6c, 0x5f, 0x73,
    0x63, 0x61, 0x6c, 0x61, 0x72, 0x18, 0x07, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0a, 0x68, 0x65, 0x61,
    0x6c, 0x53, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x12, 0x2e, 0x0a, 0x13, 0x73, 0x74, 0x61, 0x6d, 0x69,
    0x6e, 0x61, 0x5f, 0x6c, 0x6f, 0x73, 0x73, 0x5f, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x18, 0x08,
    0x20, 0x01, 0x28, 0x02, 0x52, 0x11, 0x73, 0x74, 0x61, 0x6d, 0x69, 0x6e, 0x61, 0x4c, 0x6f, 0x73,
    0x73, 0x53, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x12, 0x2a, 0x0a, 0x11, 0x74, 0x72, 0x61, 0x69, 0x6e,
    0x65, 0x72, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x5f, 0x6d, 0x69, 0x6e, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x0f, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x4c, 0x65, 0x76, 0x65, 0x6c,
    0x4d, 0x69, 0x6e, 0x12, 0x2a, 0x0a, 0x11, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x5f, 0x6c,
    0x65, 0x76, 0x65, 0x6c, 0x5f, 0x6d, 0x61, 0x78, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0f,
    0x74, 0x72, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x4d, 0x61, 0x78, 0x12,
    0x19, 0x0a, 0x08, 0x76, 0x66, 0x78, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x0b, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x07, 0x76, 0x66, 0x78, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x64, 0x75,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x73, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x0a, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x73, 0x12, 0x33, 0x0a, 0x16, 0x64,
    0x61, 0x6d, 0x61, 0x67, 0x65, 0x5f, 0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x5f, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x5f, 0x6d, 0x73, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x05, 0x52, 0x13, 0x64, 0x61, 0x6d,
    0x61, 0x67, 0x65, 0x57, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x53, 0x74, 0x61, 0x72, 0x74, 0x4d, 0x73,
    0x12, 0x2f, 0x0a, 0x14, 0x64, 0x61, 0x6d, 0x61, 0x67, 0x65, 0x5f, 0x77, 0x69, 0x6e, 0x64, 0x6f,
    0x77, 0x5f, 0x65, 0x6e, 0x64, 0x5f, 0x6d, 0x73, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x05, 0x52, 0x11,
    0x64, 0x61, 0x6d, 0x61, 0x67, 0x65, 0x57, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x45, 0x6e, 0x64, 0x4d,
    0x73, 0x12, 0x21, 0x0a, 0x0c, 0x65, 0x6e, 0x65, 0x72, 0x67, 0x79, 0x5f, 0x64, 0x65, 0x6c, 0x74,
    0x61, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x65, 0x6e, 0x65, 0x72, 0x67, 0x79, 0x44,
    0x65, 0x6c, 0x74, 0x61, 0x22, 0xf4, 0x01, 0x0a, 0x13, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c,
    0x65, 0x76, 0x65, 0x6c, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x19, 0x0a, 0x08,
    0x72, 0x61, 0x6e, 0x6b, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x01, 0x20, 0x03, 0x28, 0x05, 0x52, 0x07,
    0x72, 0x61, 0x6e, 0x6b, 0x4e, 0x75, 0x6d, 0x12, 0x2f, 0x0a, 0x13, 0x72, 0x65, 0x71, 0x75, 0x69,
    0x72, 0x65, 0x64, 0x5f, 0x65, 0x78, 0x70, 0x65, 0x72, 0x69, 0x65, 0x6e, 0x63, 0x65, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x05, 0x52, 0x12, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x45, 0x78,
    0x70, 0x65, 0x72, 0x69, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x70, 0x5f, 0x6d,
    0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x69, 0x65, 0x72, 0x18, 0x03, 0x20, 0x03, 0x28, 0x02, 0x52,
    0x0c, 0x63, 0x70, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x69, 0x65, 0x72, 0x12, 0x2f, 0x0a,
    0x14, 0x6d, 0x61, 0x78, 0x5f, 0x65, 0x67, 0x67, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f,
    0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x52, 0x11, 0x6d, 0x61, 0x78,
    0x45, 0x67, 0x67, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x3b,
    0x0a, 0x1a, 0x6d, 0x61, 0x78, 0x5f, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x5f,
    0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x17, 0x6d, 0x61, 0x78, 0x45, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72,
    0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x22, 0xda, 0x01, 0x0a, 0x0e,
    0x49, 0x61, 0x70, 0x49, 0x74, 0x65, 0x6d, 0x44, 0x69, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x12, 0x10,
    0x0a, 0x03, 0x73, 0x6b, 0x75, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x73, 0x6b, 0x75,
    0x12, 0x41, 0x0a, 0x08, 0x63, 0x61, 0x74, 0x65, 0x67, 0x6f, 0x72, 0x79, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x25, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x48, 0x6f, 0x6c, 0x6f, 0x49, 0x61, 0x70, 0x49, 0x74, 0x65,
    0x6d, 0x43, 0x61, 0x74, 0x65, 0x67, 0x6f, 0x72, 0x79, 0x52, 0x08, 0x63, 0x61, 0x74, 0x65, 0x67,
    0x6f, 0x72, 0x79, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x6f, 0x72, 0x74, 0x5f, 0x6f, 0x72, 0x64, 0x65,
    0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x09, 0x73, 0x6f, 0x72, 0x74, 0x4f, 0x72, 0x64,
    0x65, 0x72, 0x12, 0x3c, 0x0a, 0x08, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x04,
    0x20, 0x03, 0x28, 0x0e, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x73, 0x2e, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x49, 0x74, 0x65, 0x6d,
    0x2e, 0x49, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x52, 0x07, 0x69, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x73,
    0x12, 0x16, 0x0a, 0x06, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x05,
    0x52, 0x06, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x73, 0x22, 0x98, 0x02, 0x0a, 0x11, 0x45, 0x6e, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x30,
    0x0a, 0x14, 0x73, 0x70, 0x69, 0x6e, 0x5f, 0x62, 0x6f, 0x6e, 0x75, 0x73, 0x5f, 0x74, 0x68, 0x72,
    0x65, 0x73, 0x68, 0x6f, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x52, 0x12, 0x73, 0x70,
    0x69, 0x6e, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x54, 0x68, 0x72, 0x65, 0x73, 0x68, 0x6f, 0x6c, 0x64,
    0x12, 0x3a, 0x0a, 0x19, 0x65, 0x78, 0x63, 0x65, 0x6c, 0x6c, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x68,
    0x72, 0x6f, 0x77, 0x5f, 0x74, 0x68, 0x72, 0x65, 0x73, 0x68, 0x6f, 0x6c, 0x64, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x02, 0x52, 0x17, 0x65, 0x78, 0x63, 0x65, 0x6c, 0x6c, 0x65, 0x6e, 0x74, 0x54, 0x68,
    0x72, 0x6f, 0x77, 0x54, 0x68, 0x72, 0x65, 0x73, 0x68, 0x6f, 0x6c, 0x64, 0x12, 0x32, 0x0a, 0x15,
    0x67, 0x72, 0x65, 0x61, 0x74, 0x5f, 0x74, 0x68, 0x72, 0x6f, 0x77, 0x5f, 0x74, 0x68, 0x72, 0x65,
    0x73, 0x68, 0x6f, 0x6c, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x13, 0x67, 0x72, 0x65,
    0x61, 0x74, 0x54, 0x68, 0x72, 0x6f, 0x77, 0x54, 0x68, 0x72, 0x65, 0x73, 0x68, 0x6f, 0x6c, 0x64,
    0x12, 0x30, 0x0a, 0x14, 0x6e, 0x69, 0x63, 0x65, 0x5f, 0x74, 0x68, 0x72, 0x6f, 0x77, 0x5f, 0x74,
    0x68, 0x72, 0x65, 0x73, 0x68, 0x6f, 0x6c, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x52, 0x12,
    0x6e, 0x69, 0x63, 0x65, 0x54, 0x68, 0x72, 0x6f, 0x77, 0x54, 0x68, 0x72, 0x65, 0x73, 0x68, 0x6f,
    0x6c, 0x64, 0x12, 0x2f, 0x0a, 0x13, 0x6d, 0x69, 0x6c, 0x65, 0x73, 0x74, 0x6f, 0x6e, 0x65, 0x5f,
    0x74, 0x68, 0x72, 0x65, 0x73, 0x68, 0x6f, 0x6c, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x12, 0x6d, 0x69, 0x6c, 0x65, 0x73, 0x74, 0x6f, 0x6e, 0x65, 0x54, 0x68, 0x72, 0x65, 0x73, 0x68,
    0x6f, 0x6c, 0x64, 0x22, 0xb7, 0x01, 0x0a, 0x10, 0x47, 0x79, 0x6d, 0x4c, 0x65, 0x76, 0x65, 0x6c,
    0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x2f, 0x0a, 0x13, 0x72, 0x65, 0x71, 0x75,
    0x69, 0x72, 0x65, 0x64, 0x5f, 0x65, 0x78, 0x70, 0x65, 0x72, 0x69, 0x65, 0x6e, 0x63, 0x65, 0x18,
    0x01, 0x20, 0x03, 0x28, 0x05, 0x52, 0x12, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x45,
    0x78, 0x70, 0x65, 0x72, 0x69, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x6c, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x5f, 0x73, 0x6c, 0x6f, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x05, 0x52,
    0x0b, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x53, 0x6c, 0x6f, 0x74, 0x73, 0x12, 0x23, 0x0a, 0x0d,
    0x74, 0x72, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x5f, 0x73, 0x6c, 0x6f, 0x74, 0x73, 0x18, 0x03, 0x20,
    0x03, 0x28, 0x05, 0x52, 0x0c, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x53, 0x6c, 0x6f, 0x74,
    0x73, 0x12, 0x2a, 0x0a, 0x11, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x5f, 0x72, 0x6f, 0x6c, 0x6c,
    0x5f, 0x62, 0x6f, 0x6e, 0x75, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x05, 0x52, 0x0f, 0x73, 0x65,
    0x61, 0x72, 0x63, 0x68, 0x52, 0x6f, 0x6c, 0x6c, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x22, 0xbc, 0x01,
    0x0a, 0x15, 0x45, 0x71, 0x75, 0x69, 0x70, 0x70, 0x65, 0x64, 0x42, 0x61, 0x64, 0x67, 0x65, 0x53,
    0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x35, 0x0a, 0x17, 0x65, 0x71, 0x75, 0x69, 0x70,
    0x5f, 0x62, 0x61, 0x64, 0x67, 0x65, 0x5f, 0x63, 0x6f, 0x6f, 0x6c, 0x64, 0x6f, 0x77, 0x6e, 0x5f,
    0x6d, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x14, 0x65, 0x71, 0x75, 0x69, 0x70, 0x42,
    0x61, 0x64, 0x67, 0x65, 0x43, 0x6f, 0x6f, 0x6c, 0x64, 0x6f, 0x77, 0x6e, 0x4d, 0x73, 0x12, 0x36,
    0x0a, 0x17, 0x63, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x70, 0x72, 0x6f, 0x62, 0x61, 0x62, 0x69, 0x6c,
    0x69, 0x74, 0x79, 0x5f, 0x62, 0x6f, 0x6e, 0x75, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x02, 0x52,
    0x15, 0x63, 0x61, 0x74, 0x63, 0x68, 0x50, 0x72, 0x6f, 0x62, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74,
    0x79, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x12, 0x34, 0x0a, 0x16, 0x66, 0x6c, 0x65, 0x65, 0x5f, 0x70,
    0x72, 0x6f, 0x62, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x5f, 0x62, 0x6f, 0x6e, 0x75, 0x73,
    0x18, 0x03, 0x20, 0x03, 0x28, 0x02, 0x52, 0x14, 0x66, 0x6c, 0x65, 0x65, 0x50, 0x72, 0x6f, 0x62,
    0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x22, 0x32, 0x0a, 0x14,
    0x4d, 0x6f, 0x76, 0x65, 0x53, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x53, 0x65, 0x74, 0x74,
    0x69, 0x6e, 0x67, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x52, 0x08, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65,
    0x22, 0x84, 0x01, 0x0a, 0x0d, 0x42, 0x61, 0x64, 0x67, 0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e,
    0x67, 0x73, 0x12, 0x3a, 0x0a, 0x0a, 0x62, 0x61, 0x64, 0x67, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x42, 0x61, 0x64, 0x67, 0x65, 0x54,
    0x79, 0x70, 0x65, 0x52, 0x09, 0x62, 0x61, 0x64, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1d,
    0x0a, 0x0a, 0x62, 0x61, 0x64, 0x67, 0x65, 0x5f, 0x72, 0x61, 0x6e, 0x6b, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x09, 0x62, 0x61, 0x64, 0x67, 0x65, 0x52, 0x61, 0x6e, 0x6b, 0x12, 0x18, 0x0a,
    0x07, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x05, 0x52, 0x07,
    0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x73, 0x22, 0xad, 0x06, 0x0a, 0x11, 0x47, 0x79, 0x6d, 0x42,
    0x61, 0x74, 0x74, 0x6c, 0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x24, 0x0a,
    0x0e, 0x65, 0x6e, 0x65, 0x72, 0x67, 0x79, 0x5f, 0x70, 0x65, 0x72, 0x5f, 0x73, 0x65, 0x63, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0c, 0x65, 0x6e, 0x65, 0x72, 0x67, 0x79, 0x50, 0x65, 0x72,
    0x53, 0x65, 0x63, 0x12, 0x2a, 0x0a, 0x11, 0x64, 0x6f, 0x64, 0x67, 0x65, 0x5f, 0x65, 0x6e, 0x65,
    0x72, 0x67, 0x79, 0x5f, 0x63, 0x6f, 0x73, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0f,
    0x64, 0x6f, 0x64, 0x67, 0x65, 0x45, 0x6e, 0x65, 0x72, 0x67, 0x79, 0x43, 0x6f, 0x73, 0x74, 0x12,
    0x29, 0x0a, 0x10, 0x72, 0x65, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x73, 0x65, 0x63, 0x6f,
    0x6e, 0x64, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0f, 0x72, 0x65, 0x74, 0x61, 0x72,
    0x67, 0x65, 0x74, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x12, 0x32, 0x0a, 0x15, 0x65, 0x6e,
    0x65, 0x6d, 0x79, 0x5f, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72,
    0x76, 0x61, 0x6c, 0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x52, 0x13, 0x65, 0x6e, 0x65, 0x6d, 0x79,
    0x41, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x12, 0x34,
    0x0a, 0x16, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f,
    0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x18, 0x05, 0x20, 0x01, 0x28, 0x02, 0x52, 0x14,
    0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x49, 0x6e, 0x74, 0x65,
    0x72, 0x76, 0x61, 0x6c, 0x12, 0x34, 0x0a, 0x16, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x64, 0x75,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x02, 0x52, 0x14, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x44, 0x75, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x12, 0x3c, 0x0a, 0x1b, 0x62, 0x6f,
    0x6e, 0x75, 0x73, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x70, 0x65, 0x72, 0x5f, 0x61, 0x6c, 0x6c,
    0x79, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x02, 0x52,
    0x17, 0x62, 0x6f, 0x6e, 0x75, 0x73, 0x54, 0x69, 0x6d, 0x65, 0x50, 0x65, 0x72, 0x41, 0x6c, 0x6c,
    0x79, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x12, 0x3f, 0x0a, 0x1c, 0x6d, 0x61, 0x78, 0x69,
    0x6d, 0x75, 0x6d, 0x5f, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x73, 0x5f, 0x70, 0x65,
    0x72, 0x5f, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x05, 0x52, 0x19,
    0x6d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x41, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x73,
    0x50, 0x65, 0x72, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x12, 0x48, 0x0a, 0x21, 0x73, 0x61, 0x6d,
    0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x5f, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x5f, 0x62, 0x6f,
    0x6e, 0x75, 0x73, 0x5f, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x69, 0x65, 0x72, 0x18, 0x09,
    0x20, 0x01, 0x28, 0x02, 0x52, 0x1d, 0x73, 0x61, 0x6d, 0x65, 0x54, 0x79, 0x70, 0x65, 0x41, 0x74,
    0x74, 0x61, 0x63, 0x6b, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c,
    0x69, 0x65, 0x72, 0x12, 0x25, 0x0a, 0x0e, 0x6d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x5f, 0x65,
    0x6e, 0x65, 0x72, 0x67, 0x79, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x6d, 0x61, 0x78,
    0x69, 0x6d, 0x75, 0x6d, 0x45, 0x6e, 0x65, 0x72, 0x67, 0x79, 0x12, 0x3e, 0x0a, 0x1c, 0x65, 0x6e,
    0x65, 0x72, 0x67, 0x79, 0x5f, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x5f, 0x70, 0x65, 0x72, 0x5f, 0x68,
    0x65, 0x61, 0x6c, 0x74, 0x68, 0x5f, 0x6c, 0x6f, 0x73, 0x74, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x02,
    0x52, 0x18, 0x65, 0x6e, 0x65, 0x72, 0x67, 0x79, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x50, 0x65, 0x72,
    0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x4c, 0x6f, 0x73, 0x74, 0x12, 0x2a, 0x0a, 0x11, 0x64, 0x6f,
    0x64, 0x67, 0x65, 0x5f, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x73, 0x18,
    0x0c, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0f, 0x64, 0x6f, 0x64, 0x67, 0x65, 0x44, 0x75, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x73, 0x12, 0x30, 0x0a, 0x14, 0x6d, 0x69, 0x6e, 0x69, 0x6d, 0x75,
    0x6d, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x0d,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x12, 0x6d, 0x69, 0x6e, 0x69, 0x6d, 0x75, 0x6d, 0x50, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x28, 0x0a, 0x10, 0x73, 0x77, 0x61, 0x70,
    0x5f, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x73, 0x18, 0x0e, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x0e, 0x73, 0x77, 0x61, 0x70, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x4d, 0x73, 0x12, 0x43, 0x0a, 0x1e, 0x64, 0x6f, 0x64, 0x67, 0x65, 0x5f, 0x64, 0x61, 0x6d, 0x61,
    0x67, 0x65, 0x5f, 0x72, 0x65, 0x64, 0x75, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x70, 0x65, 0x72,
    0x63, 0x65, 0x6e, 0x74, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x02, 0x52, 0x1b, 0x64, 0x6f, 0x64, 0x67,
    0x65, 0x44, 0x61, 0x6d, 0x61, 0x67, 0x65, 0x52, 0x65, 0x64, 0x75, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x50, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x22, 0xf2, 0x0a, 0x0a, 0x0f, 0x50, 0x6f, 0x6b, 0x65,
    0x6d, 0x6f, 0x6e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x3a, 0x0a, 0x0a, 0x70,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75,
    0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x52, 0x09, 0x70, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x6d, 0x6f, 0x64, 0x65, 0x6c,
    0x5f, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0a, 0x6d, 0x6f,
    0x64, 0x65, 0x6c, 0x53, 0x63, 0x61, 0x6c, 0x65, 0x12, 0x31, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f,
    0x6e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x34, 0x0a, 0x06, 0x74,
    0x79, 0x70, 0x65, 0x5f, 0x32, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x50, 0x4f,
    0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x05, 0x74, 0x79, 0x70, 0x65,
    0x32, 0x12, 0x4c, 0x0a, 0x06, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x34, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x53,
    0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x50,
    0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x2e, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x41, 0x74, 0x74,
    0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x52, 0x06, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x12,
    0x55, 0x0a, 0x09, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x37, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e,
    0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e,
    0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x2e, 0x45, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65,
    0x72, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x52, 0x09, 0x65, 0x6e, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x12, 0x49, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x73, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x33, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x73,
    0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x52, 0x05, 0x73, 0x74, 0x61, 0x74,
    0x73, 0x12, 0x3e, 0x0a, 0x0b, 0x71, 0x75, 0x69, 0x63, 0x6b, 0x5f, 0x6d, 0x6f, 0x76, 0x65, 0x73,
    0x18, 0x09, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f,
    0x6e, 0x4d, 0x6f, 0x76, 0x65, 0x52, 0x0a, 0x71, 0x75, 0x69, 0x63, 0x6b, 0x4d, 0x6f, 0x76, 0x65,
    0x73, 0x12, 0x46, 0x0a, 0x0f, 0x63, 0x69, 0x6e, 0x65, 0x6d, 0x61, 0x74, 0x69, 0x63, 0x5f, 0x6d,
    0x6f, 0x76, 0x65, 0x73, 0x18, 0x0a, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x50, 0x4f, 0x47,
    0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f,
    0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x4d, 0x6f, 0x76, 0x65, 0x52, 0x0e, 0x63, 0x69, 0x6e, 0x65, 0x6d,
    0x61, 0x74, 0x69, 0x63, 0x4d, 0x6f, 0x76, 0x65, 0x73, 0x12, 0x25, 0x0a, 0x0e, 0x61, 0x6e, 0x69,
    0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x0b, 0x20, 0x03, 0x28,
    0x02, 0x52, 0x0d, 0x61, 0x6e, 0x69, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x69, 0x6d, 0x65,
    0x12, 0x40, 0x0a, 0x0d, 0x65, 0x76, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64,
    0x73, 0x18, 0x0c, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d,
    0x6f, 0x6e, 0x49, 0x64, 0x52, 0x0c, 0x65, 0x76, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x49,
    0x64, 0x73, 0x12, 0x25, 0x0a, 0x0e, 0x65, 0x76, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x5f,
    0x70, 0x69, 0x70, 0x73, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x65, 0x76, 0x6f, 0x6c,
    0x75, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x69, 0x70, 0x73, 0x12, 0x37, 0x0a, 0x06, 0x72, 0x61, 0x72,
    0x69, 0x74, 0x79, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1f, 0x2e, 0x50, 0x4f, 0x47, 0x4f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b,
    0x65, 0x6d, 0x6f, 0x6e, 0x52, 0x61, 0x72, 0x69, 0x74, 0x79, 0x52, 0x06, 0x72, 0x61, 0x72, 0x69,
    0x74, 0x79, 0x12, 0x28, 0x0a, 0x10, 0x70, 0x6f, 0x6b, 0x65, 0x64, 0x65, 0x78, 0x5f, 0x68, 0x65,
    0x69, 0x67, 0x68, 0x74, 0x5f, 0x6d, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0e, 0x70, 0x6f,
    0x6b, 0x65, 0x64, 0x65, 0x78, 0x48, 0x65, 0x69, 0x67, 0x68, 0x74, 0x4d, 0x12, 0x2a, 0x0a, 0x11,
    0x70, 0x6f, 0x6b, 0x65, 0x64, 0x65, 0x78, 0x5f, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x6b,
    0x67, 0x18, 0x10, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0f, 0x70, 0x6f, 0x6b, 0x65, 0x64, 0x65, 0x78,
    0x57, 0x65, 0x69, 0x67, 0x68, 0x74, 0x4b, 0x67, 0x12, 0x47, 0x0a, 0x11, 0x70, 0x61, 0x72, 0x65,
    0x6e, 0x74, 0x5f, 0x70, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x11, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73,
    0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49, 0x64,
    0x52, 0x0f, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x49,
    0x64, 0x12, 0x24, 0x0a, 0x0e, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x73, 0x74, 0x64, 0x5f,
    0x64, 0x65, 0x76, 0x18, 0x12, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0c, 0x68, 0x65, 0x69, 0x67, 0x68,
    0x74, 0x53, 0x74, 0x64, 0x44, 0x65, 0x76, 0x12, 0x24, 0x0a, 0x0e, 0x77, 0x65, 0x69, 0x67, 0x68,
    0x74, 0x5f, 0x73, 0x74, 0x64, 0x5f, 0x64, 0x65, 0x76, 0x18, 0x13, 0x20, 0x01, 0x28, 0x02, 0x52,
    0x0c, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74, 0x53, 0x74, 0x64, 0x44, 0x65, 0x76, 0x12, 0x2f, 0x0a,
    0x14, 0x6b, 0x6d, 0x5f, 0x64, 0x69, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x5f, 0x74, 0x6f, 0x5f,
    0x68, 0x61, 0x74, 0x63, 0x68, 0x18, 0x14, 0x20, 0x01, 0x28, 0x02, 0x52, 0x11, 0x6b, 0x6d, 0x44,
    0x69, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x54, 0x6f, 0x48, 0x61, 0x74, 0x63, 0x68, 0x12, 0x3e,
    0x0a, 0x09, 0x66, 0x61, 0x6d, 0x69, 0x6c, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x15, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x21, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2e, 0x45,
    0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x46, 0x61, 0x6d, 0x69,
    0x6c, 0x79, 0x49, 0x64, 0x52, 0x08, 0x66, 0x61, 0x6d, 0x69, 0x6c, 0x79, 0x49, 0x64, 0x12, 0x26,
    0x0a, 0x0f, 0x63, 0x61, 0x6e, 0x64, 0x79, 0x5f, 0x74, 0x6f, 0x5f, 0x65, 0x76, 0x6f, 0x6c, 0x76,
    0x65, 0x18, 0x16, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x63, 0x61, 0x6e, 0x64, 0x79, 0x54, 0x6f,
    0x45, 0x76, 0x6f, 0x6c, 0x76, 0x65, 0x12, 0x2a, 0x0a, 0x11, 0x6b, 0x6d, 0x5f, 0x62, 0x75, 0x64,
    0x64, 0x79, 0x5f, 0x64, 0x69, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x18, 0x17, 0x20, 0x01, 0x28,
    0x02, 0x52, 0x0f, 0x6b, 0x6d, 0x42, 0x75, 0x64, 0x64, 0x79, 0x44, 0x69, 0x73, 0x74, 0x61, 0x6e,
    0x63, 0x65, 0x12, 0x54, 0x0a, 0x0a, 0x62, 0x75, 0x64, 0x64, 0x79, 0x5f, 0x73, 0x69, 0x7a, 0x65,
    0x18, 0x18, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x35, 0x2e, 0x50, 0x4f, 0x47, 0x4f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x73, 0x2e, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x2e, 0x4d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x2e, 0x50, 0x6f, 0x6b, 0x65, 0x6d, 0x6f, 0x6e, 0x53, 0x65, 0x74, 0x74, 0x69,
    0x6e, 0x67, 0x73, 0x2e, 0x42, 0x75, 0x64, 0x64, 0x79, 0x53, 0x69, 0x7a, 0x65, 0x52, 0x09, 0x62,
    0x75, 0x64, 0x64, 0x79, 0x53, 0x69, 0x7a, 0x65, 0x22, 0x52, 0x0a, 0x09, 0x42, 0x75, 0x64, 0x64,
    0x79, 0x53, 0x69, 0x7a, 0x65, 0x12, 0x10, 0x0a, 0x0c, 0x42, 0x55, 0x44, 0x44, 0x59, 0x5f, 0x4d,
    0x45, 0x44, 0x49, 0x55, 0x4d, 0x10, 0x00, 0x12, 0x12, 0x0a, 0x0e, 0x42, 0x55, 0x44, 0x44, 0x59,
    0x5f, 0x53, 0x48, 0x4f, 0x55, 0x4c, 0x44, 0x45, 0x52, 0x10, 0x01, 0x12, 0x0d, 0x0a, 0x09, 0x42,
    0x55, 0x44, 0x44, 0x59, 0x5f, 0x42, 0x49, 0x47, 0x10, 0x02, 0x12, 0x10, 0x0a, 0x0c, 0x42, 0x55,
    0x44, 0x44, 0x59, 0x5f, 0x46, 0x4c, 0x59, 0x49, 0x4e, 0x47, 0x10, 0x03, 0x4a, 0xdb, 0x4a, 0x0a,
    0x07, 0x12, 0x05, 0x00, 0x00, 0xab, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x22, 0x0a, 0x09, 0x0a, 0x02,
    0x0a, 0x00, 0x12, 0x03, 0x03, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03,
    0x0e, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x04, 0x07, 0x0d, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x0e, 0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x0a, 0x02, 0x12, 0x03,
    0x05, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x05, 0x0e, 0x35, 0x0a, 0x09,
    0x0a, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x06, 0x07, 0x0d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x06, 0x0e, 0x38, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x0b, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x09, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x09, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x09, 0x17, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x27,
    0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x36, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0a, 0x08, 0x09, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0a, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x26, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x0a, 0x34, 0x35, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0c,
    0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x1e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0d, 0x08, 0x0c, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x0d, 0x0e, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x0d, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0e,
    0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0e, 0x08, 0x0d,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0e, 0x08, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x0e, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x0f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x0f, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f,
    0x17, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x24, 0x25,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x10, 0x08, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x10, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x10, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x10, 0x17, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x10, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x12, 0x00, 0x25,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x12, 0x08, 0x14, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x13, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x13, 0x08, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x13, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x13, 0x2a, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x13, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x14, 0x08, 0x3a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x14, 0x08, 0x13, 0x36, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x14, 0x08, 0x2b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x2c, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x38, 0x39, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x15, 0x08, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x15, 0x08, 0x14, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x15, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x15, 0x27,
    0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x15, 0x32, 0x33, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x16, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x04, 0x16, 0x08, 0x15, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x16, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x16, 0x0e, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x16, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x17,
    0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x04, 0x17, 0x08, 0x16,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x17, 0x08, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x17, 0x0e, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x17, 0x23, 0x24, 0x0a, 0x60, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x1b, 0x08, 0x49, 0x32, 0x53, 0x20, 0x4f, 0x6e, 0x65, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x65, 0x6c, 0x6f, 0x77, 0x20, 0x61, 0x74, 0x74, 0x72,
    0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x73,
    0x65, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x73, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x6e, 0x75, 0x6c, 0x6c, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x05, 0x04, 0x12, 0x04, 0x1b, 0x08, 0x17, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x05, 0x06, 0x12, 0x03, 0x1b, 0x08, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x1b, 0x3c, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x1b, 0x47, 0x48, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x1c, 0x08, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x04, 0x12, 0x04, 0x1c, 0x08,
    0x1b, 0x49, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x06, 0x12, 0x03, 0x1c, 0x08, 0x39,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1c, 0x3a, 0x40, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x03, 0x12, 0x03, 0x1c, 0x43, 0x44, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x07, 0x12, 0x03, 0x1d, 0x08, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x07, 0x04, 0x12, 0x04, 0x1d, 0x08, 0x1c, 0x45, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07,
    0x06, 0x12, 0x03, 0x1d, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x1d, 0x3a, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x1d,
    0x43, 0x44, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x08, 0x12, 0x03, 0x1e, 0x08, 0x45, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x04, 0x12, 0x04, 0x1e, 0x08, 0x1d, 0x45, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x06, 0x12, 0x03, 0x1e, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x08, 0x01, 0x12, 0x03, 0x1e, 0x3a, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x08, 0x03, 0x12, 0x03, 0x1e, 0x43, 0x44, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x09,
    0x12, 0x03, 0x1f, 0x08, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x04, 0x12, 0x04,
    0x1f, 0x08, 0x1e, 0x45, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x06, 0x12, 0x03, 0x1f,
    0x08, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x01, 0x12, 0x03, 0x1f, 0x38, 0x3c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x03, 0x12, 0x03, 0x1f, 0x3f, 0x41, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x0a, 0x12, 0x03, 0x20, 0x08, 0x5b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x20, 0x08, 0x1f, 0x42, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x0a, 0x06, 0x12, 0x03, 0x20, 0x08, 0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a,
    0x01, 0x12, 0x03, 0x20, 0x44, 0x55, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x03, 0x12,
    0x03, 0x20, 0x58, 0x5a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x0b, 0x12, 0x03, 0x21, 0x08,
    0x51, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x04, 0x12, 0x04, 0x21, 0x08, 0x20, 0x5b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x06, 0x12, 0x03, 0x21, 0x08, 0x42, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x21, 0x43, 0x4b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x21, 0x4e, 0x50, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x0c, 0x12, 0x03, 0x22, 0x08, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0c, 0x04,
    0x12, 0x04, 0x22, 0x08, 0x21, 0x51, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0c, 0x06, 0x12,
    0x03, 0x22, 0x08, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x22,
    0x3b, 0x42, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x22, 0x45, 0x47,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x0d, 0x12, 0x03, 0x23, 0x08, 0x53, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x0d, 0x04, 0x12, 0x04, 0x23, 0x08, 0x22, 0x48, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x0d, 0x06, 0x12, 0x03, 0x23, 0x08, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x0d, 0x01, 0x12, 0x03, 0x23, 0x40, 0x4d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0d,
    0x03, 0x12, 0x03, 0x23, 0x50, 0x52, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x0e, 0x12, 0x03,
    0x24, 0x08, 0x53, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0e, 0x04, 0x12, 0x04, 0x24, 0x08,
    0x23, 0x53, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0e, 0x06, 0x12, 0x03, 0x24, 0x08, 0x3f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x24, 0x40, 0x4d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x24, 0x50, 0x52, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x26, 0x00, 0x2e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x26, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x27, 0x08,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x27, 0x08, 0x26, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x27, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x27, 0x0e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x27, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x01, 0x12, 0x03, 0x28, 0x08, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x28, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x28, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x28, 0x17,
    0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x28, 0x3a, 0x3b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x29, 0x08, 0x35, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x04, 0x29, 0x08, 0x28, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x29, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x29, 0x0e, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x29, 0x33, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x2a,
    0x08, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2a, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x2a, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2a, 0x18, 0x35, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2a, 0x38, 0x39, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x04, 0x12, 0x03, 0x2b, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x2b, 0x08, 0x2a, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x2b, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2b,
    0x0e, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2b, 0x2b, 0x2c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x05, 0x12, 0x03, 0x2c, 0x08, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x05, 0x04, 0x12, 0x04, 0x2c, 0x08, 0x2b, 0x2d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x05, 0x05, 0x12, 0x03, 0x2c, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x2c, 0x0d, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x2c, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x06, 0x12, 0x03,
    0x2d, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x04, 0x12, 0x04, 0x2d, 0x08,
    0x2c, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x05, 0x12, 0x03, 0x2d, 0x08, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x01, 0x12, 0x03, 0x2d, 0x0d, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x03, 0x12, 0x03, 0x2d, 0x2c, 0x2d, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x2f, 0x00, 0x40, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12,
    0x03, 0x2f, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x30, 0x08,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x30, 0x08, 0x2f, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x30, 0x08, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x30, 0x0f, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x30, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x31, 0x08, 0x49, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x31, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x31, 0x11, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x31, 0x37,
    0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x31, 0x47, 0x48, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x32, 0x08, 0x40, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x32, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x02, 0x06, 0x12, 0x03, 0x32, 0x11, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x32, 0x30, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x32, 0x3e, 0x3f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x33, 0x08,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x33, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05, 0x12, 0x03, 0x33, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x33, 0x17, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x33, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x04, 0x12, 0x03, 0x34, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x34, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x05, 0x12, 0x03, 0x34,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01, 0x12, 0x03, 0x34, 0x17, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12, 0x03, 0x34, 0x28, 0x29, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x03, 0x35, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x05, 0x04, 0x12, 0x03, 0x35, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x05, 0x05, 0x12, 0x03, 0x35, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x35, 0x17, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x03, 0x12, 0x03,
    0x35, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x06, 0x12, 0x03, 0x36, 0x08, 0x28,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x04, 0x12, 0x03, 0x36, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x05, 0x12, 0x03, 0x36, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x06, 0x01, 0x12, 0x03, 0x36, 0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x06, 0x03, 0x12, 0x03, 0x36, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x07,
    0x12, 0x03, 0x37, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x04, 0x12, 0x03,
    0x37, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x05, 0x12, 0x03, 0x37, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x01, 0x12, 0x03, 0x37, 0x17, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x03, 0x12, 0x03, 0x37, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x08, 0x12, 0x03, 0x38, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x08, 0x04, 0x12, 0x03, 0x38, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08,
    0x05, 0x12, 0x03, 0x38, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x38, 0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x03, 0x12, 0x03, 0x38,
    0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x09, 0x12, 0x03, 0x39, 0x08, 0x30, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x04, 0x12, 0x03, 0x39, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x09, 0x05, 0x12, 0x03, 0x39, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x09, 0x01, 0x12, 0x03, 0x39, 0x17, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x09, 0x03, 0x12, 0x03, 0x39, 0x2d, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0a, 0x12,
    0x03, 0x3a, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x3a,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x3a, 0x11, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x3a, 0x17, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x3a, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x0b, 0x12, 0x03, 0x3b, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x0b, 0x04, 0x12, 0x03, 0x3b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x05,
    0x12, 0x03, 0x3b, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x01, 0x12, 0x03,
    0x3b, 0x17, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x3b, 0x2d,
    0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0c, 0x12, 0x03, 0x3c, 0x08, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x3c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x3c, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x0c, 0x01, 0x12, 0x03, 0x3c, 0x17, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c,
    0x03, 0x12, 0x03, 0x3c, 0x25, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0d, 0x12, 0x03,
    0x3d, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x3d, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x3d, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x3d, 0x17, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x3d, 0x29, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x0e, 0x12, 0x03, 0x3e, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e,
    0x04, 0x12, 0x03, 0x3e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x05, 0x12,
    0x03, 0x3e, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x3e,
    0x17, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x3e, 0x28, 0x2a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0f, 0x12, 0x03, 0x3f, 0x08, 0x2b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x0f, 0x04, 0x12, 0x03, 0x3f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x3f, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x0f, 0x01, 0x12, 0x03, 0x3f, 0x17, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0f, 0x03,
    0x12, 0x03, 0x3f, 0x28, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x41, 0x00, 0x51,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x41, 0x08, 0x14, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x42, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x42, 0x08, 0x41, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x42, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x42, 0x26, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x42, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x43, 0x08, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x04, 0x43, 0x08, 0x42, 0x36, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x43, 0x08, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x43, 0x0e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x43, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x02, 0x12, 0x03, 0x44, 0x08, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x44, 0x08, 0x43, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x44, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x44, 0x26,
    0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x44, 0x35, 0x36, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x45, 0x08, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x04, 0x45, 0x08, 0x44, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x03, 0x05, 0x12, 0x03, 0x45, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x45, 0x0e, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x45, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x04, 0x12, 0x03, 0x46,
    0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x04, 0x12, 0x04, 0x46, 0x08, 0x45,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x05, 0x12, 0x03, 0x46, 0x08, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01, 0x12, 0x03, 0x46, 0x0e, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x03, 0x46, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x05, 0x12, 0x03, 0x47, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05,
    0x04, 0x12, 0x04, 0x47, 0x08, 0x46, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x47, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x47, 0x0e, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x03, 0x12, 0x03, 0x47, 0x20,
    0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x06, 0x12, 0x03, 0x48, 0x08, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x04, 0x12, 0x04, 0x48, 0x08, 0x47, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x06, 0x05, 0x12, 0x03, 0x48, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x06, 0x01, 0x12, 0x03, 0x48, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x48, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x07, 0x12,
    0x03, 0x49, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x04, 0x12, 0x04, 0x49,
    0x08, 0x48, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x05, 0x12, 0x03, 0x49, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x01, 0x12, 0x03, 0x49, 0x0e, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x03, 0x12, 0x03, 0x49, 0x24, 0x25, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x08, 0x12, 0x03, 0x4a, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x08, 0x04, 0x12, 0x04, 0x4a, 0x08, 0x49, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x08, 0x05, 0x12, 0x03, 0x4a, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x4a, 0x0e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x4a, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x09, 0x12, 0x03, 0x4b, 0x08, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x04, 0x12, 0x04, 0x4b, 0x08, 0x4a, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x05, 0x12, 0x03, 0x4b, 0x08, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x09, 0x01, 0x12, 0x03, 0x4b, 0x0e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x09, 0x03, 0x12, 0x03, 0x4b, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x0a, 0x12, 0x03, 0x4c, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0a, 0x04, 0x12,
    0x04, 0x4c, 0x08, 0x4b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0a, 0x05, 0x12, 0x03,
    0x4c, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x4c, 0x0f,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x4c, 0x1a, 0x1c, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0b, 0x12, 0x03, 0x4d, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x0b, 0x04, 0x12, 0x04, 0x4d, 0x08, 0x4c, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x4d, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x0b, 0x01, 0x12, 0x03, 0x4d, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0b, 0x03,
    0x12, 0x03, 0x4d, 0x1c, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0c, 0x12, 0x03, 0x4e,
    0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0c, 0x04, 0x12, 0x04, 0x4e, 0x08, 0x4d,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x4e, 0x08, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x4e, 0x0e, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x4e, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x0d, 0x12, 0x03, 0x4f, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0d,
    0x04, 0x12, 0x04, 0x4f, 0x08, 0x4e, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0d, 0x05,
    0x12, 0x03, 0x4f, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0d, 0x01, 0x12, 0x03,
    0x4f, 0x0e, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x4f, 0x25,
    0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0e, 0x12, 0x03, 0x50, 0x08, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e, 0x04, 0x12, 0x04, 0x50, 0x08, 0x4f, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x50, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x50, 0x0e, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x0e, 0x03, 0x12, 0x03, 0x50, 0x1d, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x52,
    0x00, 0x58, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x52, 0x08, 0x1b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x53, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x53, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x53, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x53, 0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x53, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x54, 0x08,
    0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x54, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x54, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x54, 0x17, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x54, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x02, 0x12, 0x03, 0x55, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x55, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x03, 0x55,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x55, 0x17, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x55, 0x27, 0x28, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x56, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x03, 0x04, 0x12, 0x04, 0x56, 0x08, 0x55, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x56, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x56, 0x0e, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x56, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03, 0x57, 0x08,
    0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x04, 0x57, 0x08, 0x56, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x03, 0x57, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x57, 0x0e, 0x28, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03, 0x57, 0x2b, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07,
    0x12, 0x04, 0x59, 0x00, 0x5f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x59,
    0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x5a, 0x08, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x5a, 0x08, 0x59, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5a, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5a, 0x0f, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x5a, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01,
    0x12, 0x03, 0x5b, 0x08, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x5b, 0x08, 0x5a, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x5b,
    0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5b, 0x2e, 0x36,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5b, 0x39, 0x3a, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x5c, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x02, 0x04, 0x12, 0x04, 0x5c, 0x08, 0x5b, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x5c, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x5c, 0x0e, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x5c, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x03, 0x12, 0x03, 0x5d, 0x08,
    0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x04, 0x12, 0x03, 0x5d, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x06, 0x12, 0x03, 0x5d, 0x11, 0x32, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03, 0x5d, 0x33, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x03, 0x03, 0x12, 0x03, 0x5d, 0x3e, 0x3f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x04, 0x12, 0x03, 0x5e, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x5e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x05, 0x12, 0x03, 0x5e,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x01, 0x12, 0x03, 0x5e, 0x17, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x03, 0x12, 0x03, 0x5e, 0x20, 0x21, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x60, 0x00, 0x66, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08,
    0x01, 0x12, 0x03, 0x60, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03,
    0x61, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0x61, 0x08,
    0x60, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x61, 0x08, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x61, 0x0e, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x61, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x62, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x62, 0x08, 0x61, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x62, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x62, 0x0e, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x62,
    0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x63, 0x08, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x04, 0x63, 0x08, 0x62, 0x2c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x63, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x63, 0x0e, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x63, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03,
    0x12, 0x03, 0x64, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x04,
    0x64, 0x08, 0x63, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x05, 0x12, 0x03, 0x64,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x01, 0x12, 0x03, 0x64, 0x0e, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x03, 0x12, 0x03, 0x64, 0x25, 0x26, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12, 0x03, 0x65, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x04, 0x04, 0x12, 0x04, 0x65, 0x08, 0x64, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x65, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x65, 0x0e, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x65, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x67, 0x00, 0x6c, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x67, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x68, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x68, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x68, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x68, 0x17, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x68, 0x2d,
    0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x69, 0x08, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x69, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x69, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x69, 0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x69, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03,
    0x6a, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x03, 0x6a, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x05, 0x12, 0x03, 0x6a, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6a, 0x17, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6a, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x03, 0x12, 0x03, 0x6b, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x6b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x6b, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x01, 0x12, 0x03, 0x6b,
    0x17, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x03, 0x12, 0x03, 0x6b, 0x2b, 0x2c,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x6d, 0x00, 0x71, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x0a, 0x01, 0x12, 0x03, 0x6d, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00,
    0x12, 0x03, 0x6e, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x6e, 0x08, 0x6d, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6e,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6e, 0x0e, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6e, 0x28, 0x29, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03, 0x6f, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x6f, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x6f, 0x17, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x6f, 0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x02, 0x12, 0x03, 0x70, 0x08, 0x32,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x04, 0x12, 0x03, 0x70, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x05, 0x12, 0x03, 0x70, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x02, 0x01, 0x12, 0x03, 0x70, 0x17, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x70, 0x30, 0x31, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04,
    0x72, 0x00, 0x74, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x72, 0x08, 0x1c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x73, 0x08, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x73, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x73, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x73, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x73, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x75, 0x00, 0x79,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x75, 0x08, 0x15, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x76, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x76, 0x08, 0x75, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x76, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x76, 0x24, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x76, 0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x77, 0x08, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x04, 0x77, 0x08, 0x76, 0x33, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x03, 0x77, 0x08, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03, 0x77, 0x0e, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x01, 0x03, 0x12, 0x03, 0x77, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02,
    0x02, 0x12, 0x03, 0x78, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x78, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x03, 0x78,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x03, 0x78, 0x17, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x03, 0x78, 0x21, 0x22, 0x0a, 0x0b,
    0x0a, 0x02, 0x04, 0x0d, 0x12, 0x05, 0x7a, 0x00, 0x8a, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x0d, 0x01, 0x12, 0x03, 0x7a, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12,
    0x03, 0x7b, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04, 0x7b,
    0x08, 0x7a, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7b, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7b, 0x0e, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7b, 0x1f, 0x20, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x03, 0x7c, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x7c, 0x08, 0x7b, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x7c, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x7c, 0x0e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x7c, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x02, 0x12, 0x03, 0x7d, 0x08, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x04, 0x12, 0x04, 0x7d, 0x08, 0x7c, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x05, 0x12, 0x03, 0x7d, 0x08, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x02, 0x01, 0x12, 0x03, 0x7d, 0x0e, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x02, 0x03, 0x12, 0x03, 0x7d, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02,
    0x03, 0x12, 0x03, 0x7e, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x04, 0x12,
    0x04, 0x7e, 0x08, 0x7d, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x7e, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x01, 0x12, 0x03, 0x7e, 0x0e,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x03, 0x12, 0x03, 0x7e, 0x26, 0x27, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x04, 0x12, 0x03, 0x7f, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x04, 0x04, 0x12, 0x04, 0x7f, 0x08, 0x7e, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x04, 0x05, 0x12, 0x03, 0x7f, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x7f, 0x0e, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x7f, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x05, 0x12, 0x04, 0x80,
    0x01, 0x08, 0x29, 0x0a, 0x0e, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x04, 0x12, 0x05, 0x80, 0x01,
    0x08, 0x7f, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x05, 0x12, 0x04, 0x80, 0x01,
    0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x01, 0x12, 0x04, 0x80, 0x01, 0x0e,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x03, 0x12, 0x04, 0x80, 0x01, 0x27, 0x28,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x06, 0x12, 0x04, 0x81, 0x01, 0x08, 0x2e, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x04, 0x12, 0x06, 0x81, 0x01, 0x08, 0x80, 0x01, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x05, 0x12, 0x04, 0x81, 0x01, 0x08, 0x0d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x01, 0x12, 0x04, 0x81, 0x01, 0x0e, 0x29, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x06, 0x03, 0x12, 0x04, 0x81, 0x01, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0d, 0x02, 0x07, 0x12, 0x04, 0x82, 0x01, 0x08, 0x2f, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x07, 0x04, 0x12, 0x06, 0x82, 0x01, 0x08, 0x81, 0x01, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x07, 0x05, 0x12, 0x04, 0x82, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x07, 0x01, 0x12, 0x04, 0x82, 0x01, 0x0e, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x07, 0x03, 0x12, 0x04, 0x82, 0x01, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x08,
    0x12, 0x04, 0x83, 0x01, 0x08, 0x34, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x04, 0x12,
    0x06, 0x83, 0x01, 0x08, 0x82, 0x01, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x05,
    0x12, 0x04, 0x83, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x01, 0x12,
    0x04, 0x83, 0x01, 0x0e, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x03, 0x12, 0x04,
    0x83, 0x01, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x09, 0x12, 0x04, 0x84, 0x01,
    0x08, 0x22, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x04, 0x12, 0x06, 0x84, 0x01, 0x08,
    0x83, 0x01, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x05, 0x12, 0x04, 0x84, 0x01,
    0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x01, 0x12, 0x04, 0x84, 0x01, 0x0e,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x03, 0x12, 0x04, 0x84, 0x01, 0x1f, 0x21,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0a, 0x12, 0x04, 0x85, 0x01, 0x08, 0x30, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x04, 0x12, 0x06, 0x85, 0x01, 0x08, 0x84, 0x01, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x05, 0x12, 0x04, 0x85, 0x01, 0x08, 0x0d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x85, 0x01, 0x0e, 0x2a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x0a, 0x03, 0x12, 0x04, 0x85, 0x01, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0d, 0x02, 0x0b, 0x12, 0x04, 0x86, 0x01, 0x08, 0x25, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x0b, 0x04, 0x12, 0x06, 0x86, 0x01, 0x08, 0x85, 0x01, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x0b, 0x05, 0x12, 0x04, 0x86, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x0b, 0x01, 0x12, 0x04, 0x86, 0x01, 0x0e, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x0b, 0x03, 0x12, 0x04, 0x86, 0x01, 0x22, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0c,
    0x12, 0x04, 0x87, 0x01, 0x08, 0x28, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x04, 0x12,
    0x06, 0x87, 0x01, 0x08, 0x86, 0x01, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x05,
    0x12, 0x04, 0x87, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x01, 0x12,
    0x04, 0x87, 0x01, 0x0e, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x03, 0x12, 0x04,
    0x87, 0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0d, 0x12, 0x04, 0x88, 0x01,
    0x08, 0x24, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d, 0x04, 0x12, 0x06, 0x88, 0x01, 0x08,
    0x87, 0x01, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d, 0x05, 0x12, 0x04, 0x88, 0x01,
    0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d, 0x01, 0x12, 0x04, 0x88, 0x01, 0x0e,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d, 0x03, 0x12, 0x04, 0x88, 0x01, 0x21, 0x23,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0e, 0x12, 0x04, 0x89, 0x01, 0x08, 0x32, 0x0a, 0x0f,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x04, 0x12, 0x06, 0x89, 0x01, 0x08, 0x88, 0x01, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x05, 0x12, 0x04, 0x89, 0x01, 0x08, 0x0d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x01, 0x12, 0x04, 0x89, 0x01, 0x0e, 0x2c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x0e, 0x03, 0x12, 0x04, 0x89, 0x01, 0x2f, 0x31, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x0e, 0x12, 0x06, 0x8b, 0x01, 0x00, 0xab, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e,
    0x01, 0x12, 0x04, 0x8b, 0x01, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12,
    0x04, 0x8c, 0x01, 0x08, 0x33, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x06,
    0x8c, 0x01, 0x08, 0x8b, 0x01, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x06, 0x12,
    0x04, 0x8c, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04,
    0x8c, 0x01, 0x24, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8c,
    0x01, 0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x08,
    0x1e, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x06, 0x8d, 0x01, 0x08, 0x8c,
    0x01, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8d, 0x01, 0x08,
    0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x0e, 0x19,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8d, 0x01, 0x1c, 0x1d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x02, 0x12, 0x04, 0x8e, 0x01, 0x08, 0x2f, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x02, 0x04, 0x12, 0x06, 0x8e, 0x01, 0x08, 0x8d, 0x01, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x06, 0x12, 0x04, 0x8e, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x26, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8e, 0x01, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0e, 0x02, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x31, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x03, 0x04, 0x12, 0x06, 0x8f, 0x01, 0x08, 0x8e, 0x01, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x03, 0x06, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x03, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x26, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03,
    0x03, 0x12, 0x04, 0x8f, 0x01, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x04, 0x12,
    0x04, 0x90, 0x01, 0x08, 0x48, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x04, 0x12, 0x06,
    0x90, 0x01, 0x08, 0x8f, 0x01, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x06, 0x12,
    0x04, 0x90, 0x01, 0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x90, 0x01, 0x3d, 0x43, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x03, 0x12, 0x04, 0x90,
    0x01, 0x46, 0x47, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x05, 0x12, 0x04, 0x91, 0x01, 0x08,
    0x4e, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x04, 0x12, 0x06, 0x91, 0x01, 0x08, 0x90,
    0x01, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x06, 0x12, 0x04, 0x91, 0x01, 0x08,
    0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x01, 0x12, 0x04, 0x91, 0x01, 0x40, 0x49,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x03, 0x12, 0x04, 0x91, 0x01, 0x4c, 0x4d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x06, 0x12, 0x04, 0x92, 0x01, 0x08, 0x46, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x06, 0x04, 0x12, 0x06, 0x92, 0x01, 0x08, 0x91, 0x01, 0x4e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x06, 0x06, 0x12, 0x04, 0x92, 0x01, 0x08, 0x3b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x06, 0x01, 0x12, 0x04, 0x92, 0x01, 0x3c, 0x41, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x06, 0x03, 0x12, 0x04, 0x92, 0x01, 0x44, 0x45, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0e, 0x02, 0x07, 0x12, 0x04, 0x93, 0x01, 0x08, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x07, 0x04, 0x12, 0x04, 0x93, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x07,
    0x06, 0x12, 0x04, 0x93, 0x01, 0x11, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x07, 0x01,
    0x12, 0x04, 0x93, 0x01, 0x2f, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x07, 0x03, 0x12,
    0x04, 0x93, 0x01, 0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x08, 0x12, 0x04, 0x94,
    0x01, 0x08, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x08, 0x04, 0x12, 0x04, 0x94, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x08, 0x06, 0x12, 0x04, 0x94, 0x01, 0x11,
    0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x08, 0x01, 0x12, 0x04, 0x94, 0x01, 0x2f, 0x3e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x08, 0x03, 0x12, 0x04, 0x94, 0x01, 0x41, 0x43, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x09, 0x12, 0x04, 0x95, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x09, 0x04, 0x12, 0x04, 0x95, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x09, 0x05, 0x12, 0x04, 0x95, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x09, 0x01, 0x12, 0x04, 0x95, 0x01, 0x17, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x09, 0x03, 0x12, 0x04, 0x95, 0x01, 0x28, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02,
    0x0a, 0x12, 0x04, 0x96, 0x01, 0x08, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0a, 0x04,
    0x12, 0x04, 0x96, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0a, 0x06, 0x12,
    0x04, 0x96, 0x01, 0x11, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0a, 0x01, 0x12, 0x04,
    0x96, 0x01, 0x2d, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0a, 0x03, 0x12, 0x04, 0x96,
    0x01, 0x3d, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x0b, 0x12, 0x04, 0x97, 0x01, 0x08,
    0x22, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0b, 0x04, 0x12, 0x06, 0x97, 0x01, 0x08, 0x96,
    0x01, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0b, 0x05, 0x12, 0x04, 0x97, 0x01, 0x08,
    0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0b, 0x01, 0x12, 0x04, 0x97, 0x01, 0x0e, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0b, 0x03, 0x12, 0x04, 0x97, 0x01, 0x1f, 0x21, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x0c, 0x12, 0x04, 0x98, 0x01, 0x08, 0x34, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x0c, 0x04, 0x12, 0x06, 0x98, 0x01, 0x08, 0x97, 0x01, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0c, 0x06, 0x12, 0x04, 0x98, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x0c, 0x01, 0x12, 0x04, 0x98, 0x01, 0x28, 0x2e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x0c, 0x03, 0x12, 0x04, 0x98, 0x01, 0x31, 0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0e, 0x02, 0x0d, 0x12, 0x04, 0x99, 0x01, 0x08, 0x24, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x0d, 0x04, 0x12, 0x06, 0x99, 0x01, 0x08, 0x98, 0x01, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x0d, 0x05, 0x12, 0x04, 0x99, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x0d, 0x01, 0x12, 0x04, 0x99, 0x01, 0x0e, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0d,
    0x03, 0x12, 0x04, 0x99, 0x01, 0x21, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x0e, 0x12,
    0x04, 0x9a, 0x01, 0x08, 0x25, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0e, 0x04, 0x12, 0x06,
    0x9a, 0x01, 0x08, 0x99, 0x01, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0e, 0x05, 0x12,
    0x04, 0x9a, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0e, 0x01, 0x12, 0x04,
    0x9a, 0x01, 0x0e, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0e, 0x03, 0x12, 0x04, 0x9a,
    0x01, 0x22, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x0f, 0x12, 0x04, 0x9b, 0x01, 0x08,
    0x3b, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0f, 0x04, 0x12, 0x06, 0x9b, 0x01, 0x08, 0x9a,
    0x01, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0f, 0x06, 0x12, 0x04, 0x9b, 0x01, 0x08,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0f, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x24, 0x35,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0f, 0x03, 0x12, 0x04, 0x9b, 0x01, 0x38, 0x3a, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x10, 0x12, 0x04, 0x9c, 0x01, 0x08, 0x22, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x10, 0x04, 0x12, 0x06, 0x9c, 0x01, 0x08, 0x9b, 0x01, 0x3b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x10, 0x05, 0x12, 0x04, 0x9c, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x10, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x0e, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x10, 0x03, 0x12, 0x04, 0x9c, 0x01, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0e, 0x02, 0x11, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x22, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x11, 0x04, 0x12, 0x06, 0x9d, 0x01, 0x08, 0x9c, 0x01, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x11, 0x05, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x11, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x0e, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x11,
    0x03, 0x12, 0x04, 0x9d, 0x01, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x12, 0x12,
    0x04, 0x9e, 0x01, 0x08, 0x28, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x12, 0x04, 0x12, 0x06,
    0x9e, 0x01, 0x08, 0x9d, 0x01, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x12, 0x05, 0x12,
    0x04, 0x9e, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x12, 0x01, 0x12, 0x04,
    0x9e, 0x01, 0x0e, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x12, 0x03, 0x12, 0x04, 0x9e,
    0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x13, 0x12, 0x04, 0x9f, 0x01, 0x08,
    0x39, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x13, 0x04, 0x12, 0x06, 0x9f, 0x01, 0x08, 0x9e,
    0x01, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x13, 0x06, 0x12, 0x04, 0x9f, 0x01, 0x08,
    0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x13, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x2a, 0x33,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x13, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x36, 0x38, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x14, 0x12, 0x04, 0xa0, 0x01, 0x08, 0x23, 0x0a, 0x0f, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x14, 0x04, 0x12, 0x06, 0xa0, 0x01, 0x08, 0x9f, 0x01, 0x39, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x14, 0x05, 0x12, 0x04, 0xa0, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x14, 0x01, 0x12, 0x04, 0xa0, 0x01, 0x0e, 0x1d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x14, 0x03, 0x12, 0x04, 0xa0, 0x01, 0x20, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0e, 0x02, 0x15, 0x12, 0x04, 0xa1, 0x01, 0x08, 0x25, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x15, 0x04, 0x12, 0x06, 0xa1, 0x01, 0x08, 0xa0, 0x01, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x15, 0x05, 0x12, 0x04, 0xa1, 0x01, 0x08, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x15, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x0e, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x15,
    0x03, 0x12, 0x04, 0xa1, 0x01, 0x22, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x16, 0x12,
    0x04, 0xa2, 0x01, 0x08, 0x4e, 0x0a, 0x0f, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x16, 0x04, 0x12, 0x06,
    0xa2, 0x01, 0x08, 0xa1, 0x01, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x16, 0x06, 0x12,
    0x04, 0xa2, 0x01, 0x08, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x16, 0x01, 0x12, 0x04,
    0xa2, 0x01, 0x3e, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x16, 0x03, 0x12, 0x04, 0xa2,
    0x01, 0x4b, 0x4d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0e, 0x04, 0x00, 0x12, 0x06, 0xa4, 0x01, 0x08,
    0xaa, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x04, 0x00, 0x01, 0x12, 0x04, 0xa4, 0x01,
    0x0d, 0x16, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0e, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xa6, 0x01,
    0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa6,
    0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04,
    0xa6, 0x01, 0x1f, 0x20, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0e, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04,
    0xa7, 0x01, 0x10, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xa7, 0x01, 0x10, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x04, 0x00, 0x02, 0x01, 0x02,
    0x12, 0x04, 0xa7, 0x01, 0x21, 0x22, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0e, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x04, 0xa8, 0x01, 0x10, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x04, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xa8, 0x01, 0x10, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x04, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x04, 0xa8, 0x01, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0e, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0e, 0x04,
    0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xa9, 0x01, 0x1f, 0x20, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x33,
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
