use {
  super::TypeInfo,
  ::core::{hash::Hash, marker::PhantomData},
  ::parking_lot::RwLock,
  ::std::{collections::HashMap, sync::LazyLock},
};

/// A type, related to `T`, that we know does not implement `Deref`.
///
/// This is important, because we want to implement something like
/// `A::type_info`, but it is important that we don't unintentionally get the
/// implementation from a deref target if `impl Deref<B> for A`.
pub struct Provider<T: ?Sized>(PhantomData<T>);

/// The trait types implement (through derive macro) when they want to produce
/// a `TypeInfo` that describes it.
///
/// Intended to only be implemented on `Provider<T>`, with the generic to both
/// `Provider` and `TypeInfoProvider` being equal. This generic reflects the
/// type corresponding to the `TypeInfo` being provided.
///
/// The two associated types, `StaticTy` and `StaticTySized`, must store the
/// `'static` version of the type, and the `Sized + 'static` (if it exists)
/// respectively. This is so that `::core::any::TypeId::of::<T>()` can be
/// called. The `-Sized` variant is to account for implementation that require
/// a `Sized` or `?Sized` type in their implementation. i.e. `[T]` is unsized,
/// `Vec<T>` is sized.
///
/// See some discussion about `TypeId`'s `'static` requirement here:
/// <https://rust-lang.github.io/rfcs/1849-non-static-type-id.html>
///
/// ```
/// # use ::inspect::TypeInfo;
/// # use ::inspect::type_info::internal::{ProviderOfTypeInfo, Provider};
/// #
/// struct MyStruct<'s>(&'s str);
///
/// unsafe impl<'s> ProviderOfTypeInfo<MyStruct<'s>>
/// for Provider<MyStruct<'s>> {
///   type StaticTy = MyStruct<'static>;
///   type StaticTySized = MyStruct<'static>;
///
///   fn type_info() -> &'static TypeInfo { todo!() }
/// }
/// ```
///
/// The API is cleaned up for the user through `TypeInfo::of::<T>()`, which
/// provides the `TypeInfo` object corresponding to `T`.
///
/// # Safety
///
/// The implementation on `Provider<Ty>` must accurately reflect `Ty`
pub unsafe trait ProviderOfTypeInfo<Ty: ?Sized> {
  /// the `'static` version of `Ty`
  type StaticTy: ?Sized + 'static;
  /// the `'static` version of `Ty`. Only valid if `Ty` is `Sized`!
  type StaticTySized: Sized + 'static;

  fn type_info() -> &'static TypeInfo;
}

/// A Hashmap that can be put in a static
pub struct ConcurrentMap<K, V>(LazyLock<RwLock<HashMap<K, V>>>);

impl<K, V> ConcurrentMap<K, V>
where
  K: Hash + Eq,
  V: Clone,
{
  pub const fn new() -> Self {
    ConcurrentMap(LazyLock::new(|| RwLock::new(HashMap::new())))
  }

  /// Gets a clone of the value associated with `key`, or insert a value.
  pub fn get_or_insert_with(&self, key: K, default: impl FnOnce() -> V) -> V {
    if let Some(info) = self.0.read().get(&key) {
      return info.clone();
    }

    let mut dictionary = self.0.write();
    dictionary.entry(key).or_insert_with(default).clone()
  }
}
