use {::core::any::TypeId, ::inspect::TypeInfo};

#[derive(TypeInfo)]
struct UnitStruct;

#[derive(TypeInfo)]
struct TupleStruct(&'static str, usize, f32);

#[derive(TypeInfo)]
struct TupleStructWithLifetime<'a, 'b, 'c>(&'a u8, &'b u8, &'c u8);

#[derive(TypeInfo)]
struct RegularStruct {
  optional_child: &'static Option<Child>,
}

#[derive(TypeInfo)]
struct Child {
  usize_field: usize,
  optional_boxed_str: Option<Box<str>>,
}

#[derive(TypeInfo)]
struct GenericTupleStruct<A, B>(A, B, B, A);

#[derive(TypeInfo)]
struct GenericTupleStructWithLifetime<'a, 'b, A, B>(&'a A, &'b B);

#[test]
fn type_id_matches_unit_struct() {
  assert_eq!(
    TypeInfo::of::<UnitStruct>().type_id(),
    TypeId::of::<UnitStruct>()
  );
}

#[test]
fn type_id_matches_tuple_struct() {
  assert_eq!(
    TypeInfo::of::<TupleStruct>().type_id(),
    TypeId::of::<TupleStruct>()
  );
}

#[test]
fn type_id_matches_tuple_struct_with_lifetime() {
  {
    let a: &u8 = &Box::new(1);
    let b: &u8 = &2;
    let c: &u8 = &Box::new(3);

    let s = TupleStructWithLifetime(a, b, c);

    assert_eq!(
      TypeInfo::of_val(&s).type_id(),
      TypeId::of::<TupleStructWithLifetime<'static, 'static, 'static>>(),
    )
  }
}

#[test]
fn type_id_matches_regular_struct() {
  assert_eq!(
    TypeInfo::of::<RegularStruct>().type_id(),
    TypeId::of::<RegularStruct>(),
  );
}

#[test]
fn type_id_matches_generic_tuple_struct() {
  {
    let a: &usize = &Box::new(1usize);
    let b: u8 = 5;

    let s: T = GenericTupleStruct(a, b, b, a);
    type T<'a> = GenericTupleStruct<&'a usize, u8>;

    assert_eq!(TypeInfo::of_val(&s).type_id(), TypeId::of::<T<'static>>());
  }

  {
    let inner_a: &usize = &Box::new(1usize);
    let inner_b: u8 = 5;

    let a: usize = 10;
    let b: &GenericTupleStruct<&usize, u8> =
      &Box::new(GenericTupleStruct(inner_a, inner_b, inner_b, inner_a));

    let s: T = GenericTupleStruct(a, b, b, a);
    type T<'b, 'inner_a> =
      GenericTupleStruct<usize, &'b GenericTupleStruct<&'inner_a usize, u8>>;

    assert_eq!(
      TypeInfo::of_val(&s).type_id(),
      TypeId::of::<T<'static, 'static>>()
    );

    eprintln!("\n{:#}", TypeInfo::of_val(&s));
  }
}

#[test]
fn type_id_matches_generic_tuple_struct_with_lifetime() {
  {
    let a: &u8 = &Box::new(1);
    let b: &u8 = &2;

    let s: T = GenericTupleStructWithLifetime(&a, &b);
    type T<'a, 'b> = GenericTupleStructWithLifetime<'a, 'b, &'a u8, &'b u8>;

    assert_eq!(
      TypeInfo::of_val(&s).type_id(),
      TypeId::of::<T<'static, 'static>>()
    );

  }
}
