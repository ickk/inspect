use {::core::any::TypeId, ::inspect::TypeInfo, ::paste::paste};

macro_rules! assert_type_id_matches {
  ($ty:ty) => {
    paste! {
      #[allow(non_snake_case)]
      #[test]
      fn [<type_id_matches_$ty>]() {
        assert_eq!(TypeInfo::of::<$ty>().type_id(), TypeId::of::<$ty>());
      }
    }
  };
  // rename the type so that it can be used in the function name
  (type $name:ident = $ty:ty) => {
    paste! {
      #[allow(non_snake_case)]
      #[test]
      fn [<type_id_matches_$name>]() {
        #[allow(non_camel_case_types)]
        type $name = $ty;
        assert_eq!(TypeInfo::of::<$name>().type_id(), TypeId::of::<$name>());
      }
    }
  };
}

assert_type_id_matches!(u8);
assert_type_id_matches!(u16);
assert_type_id_matches!(u32);
assert_type_id_matches!(u64);
assert_type_id_matches!(u128);

assert_type_id_matches!(i8);
assert_type_id_matches!(i16);
assert_type_id_matches!(i32);
assert_type_id_matches!(i64);
assert_type_id_matches!(i128);

assert_type_id_matches!(f32);
assert_type_id_matches!(f64);

assert_type_id_matches!(usize);
assert_type_id_matches!(isize);
assert_type_id_matches!(type Unit = ());
assert_type_id_matches!(bool);
assert_type_id_matches!(char);

assert_type_id_matches!(str);
assert_type_id_matches!(String);

assert_type_id_matches!(type Vec_u8 = Vec<u8>);
assert_type_id_matches!(type Option_u8 = Option<u8>);

assert_type_id_matches!(type raw_const_u8 = *const u8);
assert_type_id_matches!(type raw_mut_u8 = *mut u8);
assert_type_id_matches!(type Box_u8 = Box<u8>);
assert_type_id_matches!(type ref_u8 = &'static u8);
assert_type_id_matches!(type ref_mut_u8 = &'static mut u8);

assert_type_id_matches!(type PhantomData_u8 = ::core::marker::PhantomData<u8>);

assert_type_id_matches!(type slice_u8 = [u8]);

#[test]
fn type_id_matches_ref_u8_non_static() {
  let v: Box<u8> = Box::new(1);
  let s: &u8 = &v;
  assert_eq!(TypeInfo::of_val(&s).type_id(), TypeId::of::<&'static u8>());
}

#[test]
fn type_id_matches_ref_mut_u8_non_static() {
  let mut v: Box<u8> = Box::new(2);
  let s: &mut u8 = &mut v;
  assert_eq!(
    TypeInfo::of_val(&s).type_id(),
    TypeId::of::<&'static mut u8>()
  );
}

#[test]
fn type_id_matches_slice_ref_u8_non_static() {
  let a = Box::new(1);
  let b = Box::new(2);
  let c = Box::new(3);
  let v: Vec<&u8> = vec![&a, &b, &c];

  let s: &[&u8] = &v;
  assert_eq!(
    TypeInfo::of_val(&s).type_id(),
    TypeId::of::<&'static [&'static u8]>()
  );
}

#[test]
fn type_id_matches_slice_ref_mut_u8_non_static() {
  let mut a = Box::new(1);
  let mut b = Box::new(2);
  let mut c = Box::new(3);
  let mut v: Vec<&mut u8> = vec![&mut a, &mut b, &mut c];

  let s: &mut [&mut u8] = &mut v;
  assert_eq!(
    TypeInfo::of_val(&s).type_id(),
    TypeId::of::<&'static mut [&'static mut u8]>()
  );
}

#[test]
fn type_id_matches_result_u8_usize() {
  assert_eq!(
    TypeInfo::of::<Result<u8, usize>>().type_id(),
    TypeId::of::<Result<u8, usize>>(),
  )
}

#[test]
fn type_id_matches_result_ref_u8_non_static() {
  fn fallible<'a, 'b>(a: &'a u8, b: &'b u8) -> Result<&'a u8, &'b u8> {
    if (a & b) == 0 {
      Ok(a)
    } else {
      Err(b)
    }
  }

  let a = Box::new(1);
  let b = Box::new(2);
  let o = fallible(&a, &b);
  assert_eq!(
    TypeInfo::of_val(&o).type_id(),
    TypeId::of::<Result<&'static u8, &'static u8>>(),
  );

  let a = Box::new(1);
  let b = Box::new(2);
  let e = fallible(&a, &b);
  assert_eq!(
    TypeInfo::of_val(&e).type_id(),
    TypeId::of::<Result<&'static u8, &'static u8>>(),
  )
}
