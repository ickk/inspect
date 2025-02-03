mod display;
mod impls;
#[doc(hidden)]
pub mod internal;
mod methods;

use {
  self::internal::{Provider, ProviderOfTypeInfo},
  ::core::any::TypeId,
};

/// Implemented for any type that can provide type info via [`TypeInfo::of`]
pub trait TypeInfoProvider {}
impl<T> TypeInfoProvider for T where Provider<T>: ProviderOfTypeInfo<T> {}

#[derive(Debug)]
#[non_exhaustive]
pub enum TypeInfo {
  Pointer(Pointer),
  Primitive(Primitive),
  Sequence(Sequence),
  Std(Std),
  Tuple(Tuple),
  Struct(Struct),
  Enum(Enum),
}

#[derive(Debug)]
#[expect(non_camel_case_types)]
pub enum Primitive {
  u8,
  u16,
  u32,
  u64,
  u128,

  i8,
  i16,
  i32,
  i64,
  i128,

  f32,
  f64,

  usize,
  isize,
  Unit,
  bool,
  char,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Sequence {
  str,
  Slice {
    id: IdInfo,
    item: ItemInfo,
  },
  Array {
    id: IdInfo,
    sized: SizedInfo,
    item: ItemInfo,
    info: ArrayInfo,
  },
}

#[derive(Debug)]
pub enum Std {
  Vec {
    id: IdInfo,
    sized: SizedInfo,
    item: ItemInfo,
  },
  String,
  Option {
    id: IdInfo,
    sized: SizedInfo,
    item: ItemInfo,
  },
  Result {
    id: IdInfo,
    sized: SizedInfo,
    info: ResultInfo,
  },
  PhantomData {
    id: IdInfo,
    sized: SizedInfo,
    item: ItemInfo,
  },
}

#[derive(Debug)]
pub enum Pointer {
  Ref {
    id: IdInfo,
    sized: SizedInfo,
    item: ItemInfo,
  },
  RefMut {
    id: IdInfo,
    sized: SizedInfo,
    item: ItemInfo,
  },
  RawConst {
    id: IdInfo,
    sized: SizedInfo,
    item: ItemInfo,
  },
  RawMut {
    id: IdInfo,
    sized: SizedInfo,
    item: ItemInfo,
  },
  Box {
    id: IdInfo,
    sized: SizedInfo,
    item: ItemInfo,
  },
}

#[derive(Debug)]
pub enum Tuple {
  Tuple {
    id: IdInfo,
    sized: SizedInfo,
    info: TupleInfo,
  },
}

#[derive(Debug)]
pub enum Struct {
  UnitStruct {
    id: IdInfo,
    sized: SizedInfo,
  },
  TupleStruct {
    id: IdInfo,
    sized: SizedInfo,
    fields: TupleStructInfo,
  },
  Struct {
    id: IdInfo,
    sized: SizedInfo,
    fields: StructInfo,
  },
}

#[derive(Debug)]
pub enum Enum {
  Enum {
    id: IdInfo,
    sized: SizedInfo,
    variants: EnumInfo,
  },
}

#[derive(Debug)]
pub struct IdInfo {
  pub type_id: TypeId,
  pub type_name: &'static str,
}

#[derive(Debug)]
pub struct SizedInfo {
  pub size: usize,
  pub align: usize,
}

#[derive(Debug)]
pub struct ItemInfo {
  pub type_info_fn: fn() -> &'static TypeInfo,
}

#[derive(Debug)]
pub struct ArrayInfo {
  pub array_length: usize,
}

#[derive(Debug)]
pub struct TupleInfo {
  pub field_infos: &'static [AnonymousFieldInfo],
}

#[derive(Debug)]
pub struct TupleStructInfo {
  pub field_infos: &'static [AnonymousFieldInfo],
}

#[derive(Debug)]
pub struct StructInfo {
  pub field_infos: &'static [NamedFieldInfo],
}

#[derive(Debug)]
pub struct NamedFieldInfo {
  pub field_name: &'static str,
  pub field_offset: usize,
  pub type_info_fn: fn() -> &'static TypeInfo,
}

#[derive(Debug)]
pub struct AnonymousFieldInfo {
  pub field_index: usize,
  pub field_offset: usize,
  pub type_info_fn: fn() -> &'static TypeInfo,
}

#[derive(Debug)]
pub struct EnumInfo {
  pub variant_infos: &'static [EnumVariantInfo],
}

#[derive(Debug)]
pub enum EnumVariantInfo {
  Unit {
    variant_name: &'static str,
    variant_descriminant: Option<usize>,
  },
  Tuple {
    variant_name: &'static str,
    variant_descriminant: Option<usize>,
    field_infos: &'static [AnonymousFieldInfo],
  },
  Struct {
    variant_name: &'static str,
    variant_descriminant: Option<usize>,
    field_infos: &'static [NamedFieldInfo],
  },
}

#[derive(Debug)]
pub struct ResultInfo {
  pub ok_type_info_fn: fn() -> &'static TypeInfo,
  pub err_type_info_fn: fn() -> &'static TypeInfo,
}
