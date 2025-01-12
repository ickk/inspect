> *Do you think God stays in heaven because he too lives in fear of what he's
created?* ~ Spy Kids 2

`inspect`
---------

An experiment collecting information about types to be queried at run-time.

This can work for types containing references, and even those with non-static
lifetime parameters; In the latter case the `TypeId` collected corresponds to
the `'static` version of the given type.

For now it is implemented for common primitive types, including numerics,
pointers, references, slices, and some types from `std` like `Option`,
`Result`, `Vec`, `PhantomData`, &c.

`TypeInfo` contains the type_name, size, align, and `TypeId` of a types. For
structs the names, offsets, and types of fields are available. For types like
`Vec<T>`, `Option<T>`, `[T]` the type info of the generic item is available.

```rust
use ::inspect::TypeInfo;

#[derive(TypeInfo)]
struct MyStruct<T>(Option<Child<T>>);
#[derive(TypeInfo)]
struct Child<T>(usize, Option<Box<str>>, Box<[T]>);

let type_info = TypeInfo::of::<MyStruct<u32>>();
assert_eq!(
  format!(
    "size: {size:?}, align: {align:?}",
    size = type_info.size(),
    align = type_info.align()
  ),
  "size: Some(40), align: Some(8)"
);
assert_eq!(
  format!("{type_info:#}"),
"MyStruct<u32>(
    Option<Child<u32>(
        usize,
        Option<Box<str>>,
        Box<[u32]>,
    )>,
)",
  );

use ::core::marker::PhantomData;
let type_info = TypeInfo::of::<[&'static Vec<u8>]>();
assert_eq!(
  format!(
    "size: {size:?}, align: {align:?}",
    size = type_info.size(),
    align = type_info.align()
  ),
  "size: None, align: None",
);
assert_eq!(
  format!("{type_info:#}"),
  "[&Vec<u8>]",
);

let type_info = TypeInfo::of::<PhantomData<Vec<usize>>>();
assert_eq!(
  format!(
    "size: {size:?}, align: {align:?}",
    size = type_info.size(),
    align = type_info.align()
  ),
  "size: Some(0), align: Some(1)",
);
assert_eq!(
  format!("{type_info:#}"),
  "PhantomData<Vec<usize>>",
);
```
