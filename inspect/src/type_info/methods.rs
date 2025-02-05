use super::{
  DiscriminantErased, Enum, EnumVariantInfo, Pointer, Primitive, Sequence,
  Std, Struct, Tuple, TypeInfo,
};

impl TypeInfo {
  /// The [`TypeId`] of the `'static` version of the type
  ///
  /// [`TypeId`]: ::core::any::TypeId
  pub fn type_id(&self) -> ::core::any::TypeId {
    use ::core::any::TypeId;

    match self {
      TypeInfo::Pointer(pointer) => match pointer {
        Pointer::Ref { id, .. }
        | Pointer::RefMut { id, .. }
        | Pointer::RawConst { id, .. }
        | Pointer::RawMut { id, .. }
        | Pointer::Box { id, .. } => id.type_id,
      },
      TypeInfo::Primitive(primitive) => match primitive {
        Primitive::u8 => TypeId::of::<u8>(),
        Primitive::u16 => TypeId::of::<u16>(),
        Primitive::u32 => TypeId::of::<u32>(),
        Primitive::u64 => TypeId::of::<u64>(),
        Primitive::u128 => TypeId::of::<u128>(),

        Primitive::i8 => TypeId::of::<i8>(),
        Primitive::i16 => TypeId::of::<i16>(),
        Primitive::i32 => TypeId::of::<i32>(),
        Primitive::i64 => TypeId::of::<i64>(),
        Primitive::i128 => TypeId::of::<i128>(),

        Primitive::f32 => TypeId::of::<f32>(),
        Primitive::f64 => TypeId::of::<f64>(),

        Primitive::usize => TypeId::of::<usize>(),
        Primitive::isize => TypeId::of::<isize>(),
        Primitive::Unit => TypeId::of::<()>(),
        Primitive::bool => TypeId::of::<bool>(),
        Primitive::char => TypeId::of::<char>(),
      },
      TypeInfo::Sequence(sequence) => match sequence {
        Sequence::str => TypeId::of::<str>(),
        Sequence::Slice { id, .. } | Sequence::Array { id, .. } => id.type_id,
      },
      TypeInfo::Std(std) => match std {
        Std::String => TypeId::of::<String>(),
        Std::Vec { id, .. }
        | Std::Option { id, .. }
        | Std::Result { id, .. }
        | Std::PhantomData { id, .. } => id.type_id,
      },
      TypeInfo::Tuple(Tuple::Tuple { id, .. }) => id.type_id,
      TypeInfo::Struct(structure) => match structure {
        Struct::UnitStruct { id, .. }
        | Struct::TupleStruct { id, .. }
        | Struct::Struct { id, .. } => id.type_id,
      },
      TypeInfo::Enum(enumeration) => match enumeration {
        Enum::Enum { id, .. } => id.type_id,
      },
    }
  }

  /// Get the [type name] of the type
  ///
  /// [type name]: ::core::any::type_name
  pub fn type_name(&self) -> &'static str {
    use ::core::any::type_name;

    match self {
      TypeInfo::Pointer(pointer) => match pointer {
        Pointer::Ref { id, .. }
        | Pointer::RefMut { id, .. }
        | Pointer::RawConst { id, .. }
        | Pointer::RawMut { id, .. }
        | Pointer::Box { id, .. } => id.type_name,
      },
      TypeInfo::Primitive(primitive) => match primitive {
        Primitive::u8 => type_name::<u8>(),
        Primitive::u16 => type_name::<u16>(),
        Primitive::u32 => type_name::<u32>(),
        Primitive::u64 => type_name::<u64>(),
        Primitive::u128 => type_name::<u128>(),

        Primitive::i8 => type_name::<i8>(),
        Primitive::i16 => type_name::<i16>(),
        Primitive::i32 => type_name::<i32>(),
        Primitive::i64 => type_name::<i64>(),
        Primitive::i128 => type_name::<i128>(),

        Primitive::f32 => type_name::<f32>(),
        Primitive::f64 => type_name::<f64>(),

        Primitive::usize => type_name::<usize>(),
        Primitive::isize => type_name::<isize>(),
        Primitive::Unit => type_name::<()>(),
        Primitive::bool => type_name::<bool>(),
        Primitive::char => type_name::<char>(),
      },
      TypeInfo::Sequence(sequence) => match sequence {
        Sequence::str => type_name::<str>(),
        Sequence::Slice { id, .. } | Sequence::Array { id, .. } => {
          id.type_name
        },
      },
      TypeInfo::Tuple(Tuple::Tuple { id, .. }) => id.type_name,
      TypeInfo::Std(std) => match std {
        Std::String => type_name::<String>(),
        Std::Vec { id, .. }
        | Std::Option { id, .. }
        | Std::Result { id, .. }
        | Std::PhantomData { id, .. } => id.type_name,
      },
      TypeInfo::Struct(structure) => match structure {
        Struct::UnitStruct { id, .. }
        | Struct::TupleStruct { id, .. }
        | Struct::Struct { id, .. } => id.type_name,
      },
      TypeInfo::Enum(enumeration) => match enumeration {
        Enum::Enum { id, .. } => id.type_name,
      },
    }
  }

  /// Get the [size] of the type, if it's a statically sized type
  ///
  /// [size]: ::core::mem::size_of
  pub fn size(&self) -> Option<usize> {
    use ::core::mem::size_of;

    match self {
      TypeInfo::Pointer(pointer) => match pointer {
        Pointer::Ref { sized, .. }
        | Pointer::RefMut { sized, .. }
        | Pointer::RawConst { sized, .. }
        | Pointer::RawMut { sized, .. }
        | Pointer::Box { sized, .. } => Some(sized.size),
      },
      TypeInfo::Primitive(primitive) => Some(match primitive {
        Primitive::u8 => size_of::<u8>(),
        Primitive::u16 => size_of::<u16>(),
        Primitive::u32 => size_of::<u32>(),
        Primitive::u64 => size_of::<u64>(),
        Primitive::u128 => size_of::<u128>(),

        Primitive::i8 => size_of::<i8>(),
        Primitive::i16 => size_of::<i16>(),
        Primitive::i32 => size_of::<i32>(),
        Primitive::i64 => size_of::<i64>(),
        Primitive::i128 => size_of::<i128>(),

        Primitive::f32 => size_of::<f32>(),
        Primitive::f64 => size_of::<f64>(),

        Primitive::usize => size_of::<usize>(),
        Primitive::isize => size_of::<isize>(),
        Primitive::Unit => size_of::<()>(),
        Primitive::bool => size_of::<bool>(),
        Primitive::char => size_of::<char>(),
      }),
      TypeInfo::Sequence(sequence) => match sequence {
        Sequence::str | Sequence::Slice { .. } => None,
        Sequence::Array { sized, .. } => Some(sized.size),
      },
      TypeInfo::Tuple(Tuple::Tuple { sized, .. }) => Some(sized.size),
      TypeInfo::Std(std) => match std {
        Std::String => Some(size_of::<String>()),
        Std::Vec { sized, .. }
        | Std::Option { sized, .. }
        | Std::Result { sized, .. }
        | Std::PhantomData { sized, .. } => Some(sized.size),
      },
      TypeInfo::Struct(structure) => match structure {
        Struct::UnitStruct { sized, .. }
        | Struct::TupleStruct { sized, .. }
        | Struct::Struct { sized, .. } => Some(sized.size),
      },
      TypeInfo::Enum(enumeration) => match enumeration {
        Enum::Enum { sized, .. } => Some(sized.size),
      },
    }
  }

  /// Get the [align] of the type, if it's a statically sized type
  ///
  /// [align]: ::core::mem::align_of
  pub fn align(&self) -> Option<usize> {
    use ::core::mem::align_of;

    match self {
      TypeInfo::Pointer(pointer) => match pointer {
        Pointer::Ref { sized, .. }
        | Pointer::RefMut { sized, .. }
        | Pointer::RawConst { sized, .. }
        | Pointer::RawMut { sized, .. }
        | Pointer::Box { sized, .. } => Some(sized.align),
      },
      TypeInfo::Primitive(primitive) => Some(match primitive {
        Primitive::u8 => align_of::<u8>(),
        Primitive::u16 => align_of::<u16>(),
        Primitive::u32 => align_of::<u32>(),
        Primitive::u64 => align_of::<u64>(),
        Primitive::u128 => align_of::<u128>(),

        Primitive::i8 => align_of::<i8>(),
        Primitive::i16 => align_of::<i16>(),
        Primitive::i32 => align_of::<i32>(),
        Primitive::i64 => align_of::<i64>(),
        Primitive::i128 => align_of::<i128>(),

        Primitive::f32 => align_of::<f32>(),
        Primitive::f64 => align_of::<f64>(),

        Primitive::usize => align_of::<usize>(),
        Primitive::isize => align_of::<isize>(),
        Primitive::Unit => align_of::<()>(),
        Primitive::bool => align_of::<bool>(),
        Primitive::char => align_of::<char>(),
      }),
      TypeInfo::Sequence(sequence) => match sequence {
        Sequence::str | Sequence::Slice { .. } => None,
        Sequence::Array { sized, .. } => Some(sized.align),
      },
      TypeInfo::Tuple(Tuple::Tuple { sized, .. }) => Some(sized.align),
      TypeInfo::Std(std) => match std {
        Std::String => Some(align_of::<String>()),
        Std::Vec { sized, .. }
        | Std::Option { sized, .. }
        | Std::Result { sized, .. }
        | Std::PhantomData { sized, .. } => Some(sized.align),
      },
      TypeInfo::Struct(structure) => match structure {
        Struct::UnitStruct { sized, .. }
        | Struct::TupleStruct { sized, .. }
        | Struct::Struct { sized, .. } => Some(sized.align),
      },
      TypeInfo::Enum(enumeration) => match enumeration {
        Enum::Enum { sized, .. } => Some(sized.align),
      },
    }
  }

  /// Get an iterator of [`DiscriminantErased`], the type-erased
  /// [discriminants], of the type when it is an enum
  ///
  /// [discriminants]: ::core::mem::discriminant
  pub fn discriminants(&self) -> impl Iterator<Item = DiscriminantErased> {
    match self {
      TypeInfo::Enum(Enum::Enum { variants, .. }) => variants.variant_infos,
      _ => &[],
    }
    .iter()
    .map(|info| match info {
      EnumVariantInfo::Unit {
        variant_discriminant,
        ..
      } => *variant_discriminant,
      EnumVariantInfo::Tuple {
        variant_discriminant,
        ..
      } => *variant_discriminant,
      EnumVariantInfo::Struct {
        variant_discriminant,
        ..
      } => *variant_discriminant,
    })
  }
}
