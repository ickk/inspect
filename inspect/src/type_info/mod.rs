mod discriminant_erased;
mod display;
mod impls;
#[doc(hidden)]
pub mod internal;
mod methods;

pub use discriminant_erased::DiscriminantErased;
use {
  self::internal::{AssociatedProvider, Provider, ProviderOfTypeInfo},
  ::core::any::TypeId,
};

/// Implemented for any type that can provide type info via [`TypeInfo::of`]
#[allow(private_bounds)]
pub trait TypeInfoProvider: AssociatedProvider {}
impl<T> TypeInfoProvider for T
where
  T: ?Sized,
  Provider<T>: ProviderOfTypeInfo<T>,
{
}

impl TypeInfo {
  /// Get the `TypeInfo` corresponding to some type `T`
  ///
  /// This method is available any time you have a type `T` that implements
  /// `TypeInfoProvider`:
  /// ```rust
  /// # use ::inspect::type_info::{TypeInfo, TypeInfoProvider};
  /// fn my_function<T: TypeInfoProvider>() {
  ///   TypeInfo::of::<T>();
  /// }
  /// ```
  pub fn of<T>() -> &'static TypeInfo
  where
    T: ?Sized + TypeInfoProvider,
  {
    <T as AssociatedProvider>::Provider::type_info()
  }

  /// Get the `TypeInfo` corresponding to some type `T`, when you have a
  /// reference to some extant `&T`.
  pub fn of_val<T>(_: &T) -> &'static TypeInfo
  where
    T: ?Sized + TypeInfoProvider,
  {
    <T as AssociatedProvider>::Provider::type_info()
  }
}

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
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
pub enum Tuple {
  Tuple {
    id: IdInfo,
    sized: SizedInfo,
    info: TupleInfo,
  },
}

#[derive(Debug)]
#[non_exhaustive]
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
#[non_exhaustive]
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
    /// The opaque [`DiscriminantErased`] for this variant
    variant_discriminant: DiscriminantErased,
    /// The numeric discriminant value for enums that opt into an explicit
    /// (non Rust) repr
    variant_discriminant_value: Option<usize>,
  },
  Tuple {
    variant_name: &'static str,
    /// The opaque [`DiscriminantErased`] for this variant
    variant_discriminant: DiscriminantErased,
    /// The numeric discriminant value for enums that opt into an explicit
    /// (non Rust) repr
    variant_discriminant_value: Option<usize>,
    field_infos: &'static [AnonymousFieldInfo],
  },
  Struct {
    variant_name: &'static str,
    /// The opaque [`DiscriminantErased`] for this variant
    variant_discriminant: DiscriminantErased,
    /// The numeric discriminant value for enums that opt into an explicit
    /// (non Rust) repr
    variant_discriminant_value: Option<usize>,
    field_infos: &'static [NamedFieldInfo],
  },
}

#[derive(Debug)]
pub struct ResultInfo {
  pub ok_type_info_fn: fn() -> &'static TypeInfo,
  pub err_type_info_fn: fn() -> &'static TypeInfo,
}
