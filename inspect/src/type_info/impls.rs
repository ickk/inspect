use {
  crate::type_info::{
    internal::{ConcurrentMap, Provider, ProviderOfTypeInfo},
    AnonymousFieldInfo, IdInfo, ItemInfo, Pointer, Primitive, ResultInfo,
    Sequence, SizedInfo, Std, Tuple, TupleInfo, TypeInfo,
  },
  ::core::{
    any::{type_name, TypeId},
    marker::PhantomData,
    mem::{align_of, offset_of, size_of},
    option::Option,
  },
  ::std::vec::Vec,
};

macro_rules! impl_type_info_trivial {
  ($ty:ty as $variant:expr) => {
    unsafe impl ProviderOfTypeInfo<$ty> for Provider<$ty> {
      type StaticTy = $ty;
      type StaticTySized = $ty;

      fn type_info() -> &'static TypeInfo {
        &$variant
      }
    }
  };
  (unsized $ty:ty as $variant:expr) => {
    unsafe impl ProviderOfTypeInfo<$ty> for Provider<$ty> {
      type StaticTy = $ty;
      type StaticTySized = ();

      fn type_info() -> &'static TypeInfo {
        &$variant
      }
    }
  };
}

impl_type_info_trivial! { u8 as TypeInfo::Primitive(Primitive::u8) }
impl_type_info_trivial! { u16 as TypeInfo::Primitive(Primitive::u16) }
impl_type_info_trivial! { u32 as TypeInfo::Primitive(Primitive::u32) }
impl_type_info_trivial! { u64 as TypeInfo::Primitive(Primitive::u64) }
impl_type_info_trivial! { u128 as TypeInfo::Primitive(Primitive::u128) }

impl_type_info_trivial! { i8 as TypeInfo::Primitive(Primitive::i8) }
impl_type_info_trivial! { i16 as TypeInfo::Primitive(Primitive::i16) }
impl_type_info_trivial! { i32 as TypeInfo::Primitive(Primitive::i32) }
impl_type_info_trivial! { i64 as TypeInfo::Primitive(Primitive::i64) }
impl_type_info_trivial! { i128 as TypeInfo::Primitive(Primitive::i128) }

impl_type_info_trivial! { f32 as TypeInfo::Primitive(Primitive::f32) }
impl_type_info_trivial! { f64 as TypeInfo::Primitive(Primitive::f64) }

impl_type_info_trivial! { usize as TypeInfo::Primitive(Primitive::usize) }
impl_type_info_trivial! { isize as TypeInfo::Primitive(Primitive::isize) }
impl_type_info_trivial! { () as TypeInfo::Primitive(Primitive::Unit) }
impl_type_info_trivial! { bool as TypeInfo::Primitive(Primitive::bool) }
impl_type_info_trivial! { char as TypeInfo::Primitive(Primitive::char) }

impl_type_info_trivial! { unsized str as TypeInfo::Sequence(Sequence::str) }
impl_type_info_trivial! { String as TypeInfo::Std(Std::String) }

macro_rules! impl_type_info_sized_with_item {
  (
    <$($lifetime:lifetime,)? $item:ident> $ty:ty as
    $outer:ident::$outer_variant:ident($inner:ident::$inner_variant:ident);
    type StaticTy = $static_ty:ty;
  ) => {
    unsafe impl<$($lifetime,)? $item> ProviderOfTypeInfo<$ty> for Provider<$ty>
    where
      $item: Sized,
      Provider<$item>: ProviderOfTypeInfo<$item>,
    {
      type StaticTy = $static_ty;
      type StaticTySized = $static_ty;

      fn type_info() -> &'static TypeInfo {
        static DICTIONARY: ConcurrentMap<TypeId, &'static TypeInfo> =
          ConcurrentMap::new();

        let type_id = TypeId::of::<Self::StaticTy>();
        DICTIONARY.get_or_insert_with(type_id, || {
          let info = $outer::$outer_variant($inner::$inner_variant {
            id: IdInfo {
              type_id,
              type_name: type_name::<$ty>(),
            },
            sized: SizedInfo {
              size: size_of::<$ty>(),
              align: align_of::<$ty>(),
            },
            item: ItemInfo {
              type_info_fn: Provider::<$item>::type_info,
            },
          });
          Box::leak(Box::new(info))
        })
      }
    }
  };
}

impl_type_info_sized_with_item! {
  <T> Vec<T> as TypeInfo::Std(Std::Vec);
  type StaticTy = Vec<<Provider<T> as ProviderOfTypeInfo<T>>::StaticTySized>;
}
impl_type_info_sized_with_item! {
  <T> Option<T> as TypeInfo::Std(Std::Option);
  type StaticTy = Option<<Provider<T> as ProviderOfTypeInfo<T>>::StaticTySized>;
}

macro_rules! impl_type_info_sized_with_unsized_item {
  (
    <$($lifetime:lifetime,)? $item:ident> $ty:ty as
    $outer:ident::$outer_variant:ident($inner:ident::$inner_variant:ident);
    type StaticTy = $static_ty:ty;
  ) => {
    unsafe impl<$($lifetime,)? $item> ProviderOfTypeInfo<$ty> for Provider<$ty>
    where
      $item: ?Sized,
      Provider<$item>: ProviderOfTypeInfo<$item>,
    {
      type StaticTy = $static_ty;
      type StaticTySized = $static_ty;

      fn type_info() -> &'static TypeInfo {
        static DICTIONARY: ConcurrentMap<TypeId, &'static TypeInfo> =
          ConcurrentMap::new();

        let type_id = TypeId::of::<Self::StaticTy>();
        DICTIONARY.get_or_insert_with(type_id, || {
          let info = $outer::$outer_variant($inner::$inner_variant {
            id: IdInfo {
              type_id,
              type_name: type_name::<$ty>(),
            },
            sized: SizedInfo {
              size: size_of::<$ty>(),
              align: align_of::<$ty>(),
            },
            item: ItemInfo {
              type_info_fn: Provider::<$item>::type_info,
            },
          });
          Box::leak(Box::new(info))
        })
      }
    }
  };
}

impl_type_info_sized_with_unsized_item! {
  <T> *const T as TypeInfo::Pointer(Pointer::RawConst);
  type StaticTy = *const <Provider<T> as ProviderOfTypeInfo<T>>::StaticTy;
}
impl_type_info_sized_with_unsized_item! {
  <T> *mut T as TypeInfo::Pointer(Pointer::RawMut);
  type StaticTy = *mut <Provider<T> as ProviderOfTypeInfo<T>>::StaticTy;
}
impl_type_info_sized_with_unsized_item! {
  <T> Box<T> as TypeInfo::Pointer(Pointer::Box);
  type StaticTy = Box<<Provider<T> as ProviderOfTypeInfo<T>>::StaticTy>;
}
impl_type_info_sized_with_unsized_item! {
  <'s, T> &'s T as TypeInfo::Pointer(Pointer::Ref);
  type StaticTy = &'static <Provider<T> as ProviderOfTypeInfo<T>>::StaticTy;
}
impl_type_info_sized_with_unsized_item! {
  <'s, T> &'s mut T as TypeInfo::Pointer(Pointer::RefMut);
  type StaticTy = &'static mut <Provider<T> as ProviderOfTypeInfo<T>>::StaticTy;
}

impl_type_info_sized_with_unsized_item! {
  <T> PhantomData<T> as TypeInfo::Std(Std::PhantomData);
  type StaticTy = PhantomData<<Provider<T> as ProviderOfTypeInfo<T>>::StaticTy>;
}

macro_rules! impl_type_info_unsized_with_item {
  (
    <$($lifetime:lifetime,)? $item:ident> $ty:ty as
    $outer:ident::$outer_variant:ident($inner:ident::$inner_variant:ident);
    type StaticTy = $static_ty:ty;
  ) => {
    unsafe impl<$($lifetime,)? $item> ProviderOfTypeInfo<$ty> for Provider<$ty>
    where
      Provider<$item>: ProviderOfTypeInfo<$item>,
    {
      type StaticTy = $static_ty;
      type StaticTySized = ();

      fn type_info() -> &'static TypeInfo {
        static DICTIONARY: ConcurrentMap<TypeId, &'static TypeInfo> =
          ConcurrentMap::new();

        let type_id = TypeId::of::<Self::StaticTy>();
        DICTIONARY.get_or_insert_with(type_id, || {
          let info = $outer::$outer_variant($inner::$inner_variant {
            id: IdInfo {
              type_id,
              type_name: type_name::<$ty>(),
            },
            item: ItemInfo {
              type_info_fn: Provider::<$item>::type_info,
            },
          });
          Box::leak(Box::new(info))
        })
      }
    }
  };
}

impl_type_info_unsized_with_item! {
  <T> [T] as TypeInfo::Sequence(Sequence::Slice);
  type StaticTy = [<Provider<T> as ProviderOfTypeInfo<T>>::StaticTySized];
}

unsafe impl<O, E> ProviderOfTypeInfo<Result<O, E>> for Provider<Result<O, E>>
where
  O: Sized,
  E: Sized,
  Provider<O>: ProviderOfTypeInfo<O>,
  Provider<E>: ProviderOfTypeInfo<E>,
{
  type StaticTy = Result<
    <Provider<O> as ProviderOfTypeInfo<O>>::StaticTySized,
    <Provider<E> as ProviderOfTypeInfo<E>>::StaticTySized,
  >;
  type StaticTySized = Result<
    <Provider<O> as ProviderOfTypeInfo<O>>::StaticTySized,
    <Provider<E> as ProviderOfTypeInfo<E>>::StaticTySized,
  >;

  fn type_info() -> &'static TypeInfo {
    static DICTIONARY: ConcurrentMap<TypeId, &'static TypeInfo> =
      ConcurrentMap::new();

    let type_id = TypeId::of::<Self::StaticTy>();
    DICTIONARY.get_or_insert_with(type_id, || {
      let info = TypeInfo::Std(Std::Result {
        id: IdInfo {
          type_id,
          type_name: type_name::<Result<O, E>>(),
        },
        sized: SizedInfo {
          size: size_of::<Result<O, E>>(),
          align: align_of::<Result<O, E>>(),
        },
        info: ResultInfo {
          ok_type_info_fn: Provider::<O>::type_info,
          err_type_info_fn: Provider::<E>::type_info,
        },
      });
      Box::leak(Box::new(info))
    })
  }
}

// does not support tuples with unsized last element - this is probably not
// possible without specialisation, or giving up `SizedInfo`. Maybe it's
// possible to abuse existing specialisation such as array-specialisation to
// optionally provide `SizedInfo`?
macro_rules! impl_type_info_tuple {
  ($($index:literal:$generic:ident),+) => {
    unsafe impl<$($generic,)+> ProviderOfTypeInfo<($($generic,)+)>
    for Provider<($($generic,)+)>
    where
      $($generic: Sized,)+
      $(Provider<$generic>: ProviderOfTypeInfo<$generic>,)+
    {
      type StaticTy = ($(
        <Provider<$generic> as ProviderOfTypeInfo<$generic>>::StaticTySized,
      )+);
      type StaticTySized = ($(
        <Provider<$generic> as ProviderOfTypeInfo<$generic>>::StaticTySized,
      )+);

      fn type_info() -> &'static TypeInfo {
        // ensure the indices are monotonically increasing from 0
        // I'd guess the compiler probably will remove this
        [$($index ),+]
          .iter()
          .copied()
          .enumerate()
          .for_each(|(e, i)| debug_assert_eq!(e, i));

        static DICTIONARY: ConcurrentMap<TypeId, &'static TypeInfo> =
          ConcurrentMap::new();

        let type_id = TypeId::of::<Self::StaticTy>();
        DICTIONARY.get_or_insert_with(type_id, || {
          let field_infos = Box::leak(
            vec![$(
              AnonymousFieldInfo {
                field_index: $index,
                field_offset: offset_of!(Self::StaticTy, $index),
                type_info_fn: Provider::<$generic>::type_info,
              },
            )+]
            .into_boxed_slice(),
          );

          let info = TypeInfo::Tuple(Tuple::Tuple {
            id: IdInfo {
              type_id,
              type_name: type_name::<($($generic,)+)>(),
            },
            sized: SizedInfo {
              size: size_of::<($($generic,)+)>(),
              align: align_of::<($($generic,)+)>(),
            },
            info: TupleInfo { field_infos },
          });
          Box::leak(Box::new(info))
        })
      }
    }
  };
}

impl_type_info_tuple! { 0:A }
impl_type_info_tuple! { 0:A, 1:B }
impl_type_info_tuple! { 0:A, 1:B, 2:C }
impl_type_info_tuple! { 0:A, 1:B, 2:C, 3:D }
impl_type_info_tuple! { 0:A, 1:B, 2:C, 3:D, 4:E }
impl_type_info_tuple! { 0:A, 1:B, 2:C, 3:D, 4:E, 5:F }
impl_type_info_tuple! { 0:A, 1:B, 2:C, 3:D, 4:E, 5:F, 6:G }
impl_type_info_tuple! { 0:A, 1:B, 2:C, 3:D, 4:E, 5:F, 6:G, 7:H }
