use {
  ::core::any::TypeId,
  ::inspect::{type_info::DiscriminantErased, TypeInfo},
};

#[allow(unused)]
#[derive(TypeInfo)]
enum MyEnum {
  One,
  Two(Vec<u8>, usize),
  Three { f0: usize, f1: Vec<u8> },
}

#[test]
fn type_id_matches_enum() {
  assert_eq!(TypeInfo::of::<MyEnum>().type_id(), TypeId::of::<MyEnum>());
}

#[test]
fn discriminants_eq() {
  use ::core::{iter::zip, mem};

  let discriminants: Vec<mem::Discriminant<MyEnum>> = vec![
    mem::discriminant(&MyEnum::One),
    mem::discriminant(&MyEnum::Two(Default::default(), Default::default())),
    mem::discriminant(&MyEnum::Three {
      f0: Default::default(),
      f1: Default::default(),
    }),
  ];
  let discriminants_erased: Vec<DiscriminantErased> =
    TypeInfo::of::<MyEnum>().discriminants().collect();

  for (discriminant_erased, discriminant) in
    zip(discriminants_erased, discriminants)
  {
    assert!(
      discriminant_erased.eq(&discriminant_erased),
      "DiscriminantErased and DiscriminantErased are equal"
    );
    assert!(
      discriminant_erased.eq(&discriminant),
      "DiscriminantErased and Discriminant are equal"
    );
    assert!(
      discriminant.eq(&discriminant_erased),
      "Discriminant and DiscriminantErased are equal"
    );
  }
}

#[test]
fn discriminants_hash() {
  use {
    ::core::{iter::zip, mem},
    ::std::hash::{DefaultHasher, Hash, Hasher},
  };

  fn hash<T: Hash>(v: T) -> u64 {
    let mut s = DefaultHasher::new();
    v.hash(&mut s);
    s.finish()
  }

  let discriminants: Vec<mem::Discriminant<MyEnum>> = vec![
    mem::discriminant(&MyEnum::One),
    mem::discriminant(&MyEnum::Two(Default::default(), Default::default())),
    mem::discriminant(&MyEnum::Three {
      f0: Default::default(),
      f1: Default::default(),
    }),
  ];
  let discriminants_erased: Vec<DiscriminantErased> =
    TypeInfo::of::<MyEnum>().discriminants().collect();

  for (discriminant_erased, discriminant) in
    zip(discriminants_erased, discriminants)
  {
    assert_eq!(
      hash(discriminant_erased),
      hash(discriminant),
      "DiscriminantErased and Discriminant hash the same"
    )
  }
}
