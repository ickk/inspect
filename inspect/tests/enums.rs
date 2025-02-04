use {::core::any::TypeId, ::inspect::TypeInfo};

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
  // eprintln!("\n{:#}", TypeInfo::of::<MyEnum>());
}
