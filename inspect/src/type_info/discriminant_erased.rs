use ::core::{
  any::Any,
  fmt::Debug,
  hash::{Hash, Hasher},
  mem::Discriminant,
};

/// Shenanigans to erase the type parameter from [`::core::mem::Discriminant`]
#[derive(Debug, Clone, Copy)]
pub struct DiscriminantErased(&'static dyn DiscriminantDyn);
impl DiscriminantErased {
  /// This creates a `DiscriminantErased` from a `Discriminant<T>`. It's
  /// exported for macro use by `internal::leak_erase_discriminant`
  pub(crate) fn leak_erase_discriminant<E>(concrete: Discriminant<E>) -> Self
  where
    E: 'static,
  {
    let discriminant_dyn: &'static dyn DiscriminantDyn =
      Box::leak(Box::new(concrete));
    DiscriminantErased(discriminant_dyn)
  }

  /// Attempt to downcast back to [`Discriminant<E>`], if `DiscriminantErased`
  /// belongs to enum `E`
  pub fn downcast<E>(&self) -> Option<Discriminant<E>>
  where
    E: 'static,
  {
    self.0.as_any().downcast_ref().copied()
  }
}

impl Hash for DiscriminantErased {
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    self.0.dyn_hash(state)
  }
}
impl PartialEq for DiscriminantErased {
  fn eq(&self, other: &Self) -> bool {
    self.0.dyn_eq(other.0)
  }
}
impl Eq for DiscriminantErased {}
impl<E> PartialEq<Discriminant<E>> for DiscriminantErased
where
  E: 'static,
{
  fn eq(&self, other: &Discriminant<E>) -> bool {
    self
      .0
      .as_any()
      .downcast_ref::<Discriminant<E>>()
      .is_some_and(|this| this == other)
  }
}
impl<E> PartialEq<DiscriminantErased> for Discriminant<E>
where
  E: 'static,
{
  fn eq(&self, other: &DiscriminantErased) -> bool {
    other
      .0
      .as_any()
      .downcast_ref::<Discriminant<E>>()
      .is_some_and(|other| other == self)
  }
}

trait DiscriminantDyn: Send + Sync + Debug {
  fn as_any(&self) -> &dyn Any;
  fn dyn_eq(&self, other: &dyn DiscriminantDyn) -> bool;
  fn dyn_hash(&self, state: &mut dyn Hasher);
}

impl<E> DiscriminantDyn for Discriminant<E>
where
  E: 'static,
{
  fn as_any(&self) -> &dyn Any {
    self
  }
  fn dyn_eq(&self, other: &dyn DiscriminantDyn) -> bool {
    other
      .as_any()
      .downcast_ref::<Discriminant<E>>()
      .is_some_and(|other| other == self)
  }
  fn dyn_hash(&self, mut state: &mut dyn Hasher) {
    self.hash(&mut state)
  }
}
