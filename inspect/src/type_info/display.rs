use {
  super::{
    Enum, EnumInfo, EnumVariantInfo, Pointer, Sequence, Std, Struct, Tuple,
    TypeInfo,
  },
  ::core::fmt,
};

// The precision controls how many levels of pointers are followed; avoiding
// infinite recursion on circular types.
impl fmt::Display for TypeInfo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let alternate = f.alternate();
    let precision = f.precision().unwrap_or(10);
    let short_name = {
      let mut n = ::disqualified::ShortName::of::<()>();
      n.0 = self.type_name();
      n
    };
    match self {
      TypeInfo::Pointer(pointer) => match pointer {
        Pointer::Ref { item, .. } => {
          if precision == 0 {
            return f.write_str("&..");
          }
          if alternate {
            f.write_fmt(format_args!(
              "&{:#.*}",
              precision - 1,
              (item.type_info_fn)(),
            ))
          } else {
            f.write_fmt(format_args!(
              "&{:.*}",
              precision - 1,
              (item.type_info_fn)()
            ))
          }
        },
        Pointer::RefMut { item, .. } => {
          if precision == 0 {
            return f.write_str("&mut ..");
          }
          if alternate {
            f.write_fmt(format_args!(
              "&mut {:#.*}",
              precision - 1,
              (item.type_info_fn)(),
            ))
          } else {
            f.write_fmt(format_args!(
              "&mut {:.*}",
              precision - 1,
              (item.type_info_fn)()
            ))
          }
        },
        Pointer::RawConst { item, .. } => {
          if precision == 0 {
            return f.write_str("*const ..");
          }
          if alternate {
            f.write_fmt(format_args!(
              "*const {:#.*}",
              precision - 1,
              (item.type_info_fn)(),
            ))
          } else {
            f.write_fmt(format_args!(
              "*const {:.*}",
              precision - 1,
              (item.type_info_fn)()
            ))
          }
        },
        Pointer::RawMut { item, .. } => {
          if precision == 0 {
            return f.write_str("*mut ..");
          }
          if alternate {
            f.write_fmt(format_args!(
              "*mut {:#.*}",
              precision - 1,
              (item.type_info_fn)(),
            ))
          } else {
            f.write_fmt(format_args!(
              "*mut {:.*}",
              precision - 1,
              (item.type_info_fn)()
            ))
          }
        },
        Pointer::Box { item, .. } => {
          if precision == 0 {
            return f.write_str("Box<..>");
          }
          if alternate {
            f.write_fmt(format_args!(
              "Box<{:#.*}>",
              precision - 1,
              (item.type_info_fn)(),
            ))
          } else {
            f.write_fmt(format_args!(
              "Box<{:.*}>",
              precision - 1,
              (item.type_info_fn)()
            ))
          }
        },
      },
      TypeInfo::Primitive(..) => f.write_fmt(format_args!("{short_name}")),
      TypeInfo::Sequence(sequence) => match sequence {
        Sequence::str => f.write_fmt(format_args!("{short_name}")),
        Sequence::Slice { item, .. } => {
          if alternate {
            f.write_fmt(format_args!("[{:#}]", (item.type_info_fn)()))
          } else {
            f.write_fmt(format_args!("[{}]", (item.type_info_fn)()))
          }
        },
        Sequence::Array { item, info, .. } => {
          if alternate {
            f.write_fmt(format_args!(
              "[{:#}; {}]",
              (item.type_info_fn)(),
              info.array_length
            ))
          } else {
            f.write_fmt(format_args!(
              "[{}; {}]",
              (item.type_info_fn)(),
              info.array_length
            ))
          }
        },
      },
      TypeInfo::Std(std) => match std {
        Std::Vec { item, .. } => {
          if alternate {
            f.write_fmt(format_args!(
              "Vec<{:#.*}>",
              precision - 1,
              (item.type_info_fn)(),
            ))
          } else {
            f.write_fmt(format_args!(
              "Vec<{:.*}>",
              precision - 1,
              (item.type_info_fn)()
            ))
          }
        },
        Std::String => f.write_fmt(format_args!("{short_name}")),
        Std::Option { item, .. } => {
          if alternate {
            f.write_fmt(format_args!(
              "Option<{:#.*}>",
              precision,
              (item.type_info_fn)()
            ))
          } else {
            f.write_fmt(format_args!(
              "Option<{:.*}>",
              precision,
              (item.type_info_fn)()
            ))
          }
        },
        Std::PhantomData { .. } => f.write_fmt(format_args!("{short_name}")),
        Std::Result { info, .. } => {
          if alternate {
            f.write_fmt(format_args!(
              "Result<{:#.*}, {:#.*}>",
              precision,
              (info.ok_type_info_fn)(),
              precision,
              (info.err_type_info_fn)(),
            ))
          } else {
            f.write_fmt(format_args!(
              "Result<{:.*}, {:.*}>",
              precision,
              (info.ok_type_info_fn)(),
              precision,
              (info.err_type_info_fn)(),
            ))
          }
        },
      },
      TypeInfo::Tuple(tuple) => match tuple {
        Tuple::Tuple { info, .. } => {
          let mut tuple = f.debug_tuple("");
          for field in info.field_infos {
            if alternate {
              tuple.field(&format_args!(
                "{:#.*}",
                precision,
                (field.type_info_fn)()
              ));
            } else {
              tuple.field(&format_args!(
                "{:.*}",
                precision,
                (field.type_info_fn)()
              ));
            }
          }
          tuple.finish()
        },
      },
      TypeInfo::Struct(structure) => match structure {
        Struct::UnitStruct { .. } => f.write_fmt(format_args!("{short_name}")),
        Struct::TupleStruct { fields, .. } => {
          f.write_fmt(format_args!("{short_name}"))?;
          let mut tuple = f.debug_tuple("");
          for field in fields.field_infos {
            if alternate {
              tuple.field(&format_args!(
                "{:#.*}",
                precision,
                (field.type_info_fn)()
              ));
            } else {
              tuple.field(&format_args!(
                "{:.*}",
                precision,
                (field.type_info_fn)()
              ));
            }
          }
          tuple.finish()
        },
        Struct::Struct { fields, .. } => {
          f.write_fmt(format_args!("{short_name}"))?;
          let mut set = f.debug_struct("");
          for field in fields.field_infos {
            if alternate {
              set.field(
                field.field_name,
                &format_args!("{:#.*}", precision, (field.type_info_fn)()),
              );
            } else {
              set.field(
                field.field_name,
                &format_args!("{:.*}", precision, (field.type_info_fn)()),
              );
            }
          }
          set.finish()
        },
      },
      TypeInfo::Enum(Enum::Enum {
        variants: EnumInfo { variant_infos },
        ..
      }) => {
        struct DisplayVariant<'v>(&'v EnumVariantInfo);
        impl fmt::Display for DisplayVariant<'_> {
          fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let precision = f.precision().unwrap();
            let alternate = f.alternate();
            match self.0 {
              EnumVariantInfo::Unit { variant_name, .. } => {
                f.write_str(variant_name)
              },
              EnumVariantInfo::Tuple {
                variant_name,
                field_infos,
                ..
              } => {
                let mut tuple = f.debug_tuple(variant_name);
                for field in field_infos.iter() {
                  if alternate {
                    tuple.field(&format_args!(
                      "{:#.*}",
                      precision,
                      (field.type_info_fn)(),
                    ));
                  } else {
                    tuple.field(&format_args!(
                      "{:.*}",
                      precision,
                      (field.type_info_fn)(),
                    ));
                  }
                }
                tuple.finish()
              },
              EnumVariantInfo::Struct {
                variant_name,
                field_infos,
                ..
              } => {
                let mut structure = f.debug_struct(variant_name);
                for field in field_infos.iter() {
                  if alternate {
                    structure.field(
                      field.field_name,
                      &format_args!(
                        "{:#.*}",
                        precision,
                        (field.type_info_fn)(),
                      ),
                    );
                  } else {
                    structure.field(
                      field.field_name,
                      &format_args!(
                        "{:.*}",
                        precision,
                        (field.type_info_fn)(),
                      ),
                    );
                  }
                }
                structure.finish()
              },
            }
          }
        }

        f.write_fmt(format_args!("{short_name} "))?;
        let mut set = f.debug_set();
        for variant in variant_infos.iter() {
          if alternate {
            set.entry(&format_args!(
              "{:#.*}",
              precision,
              DisplayVariant(variant)
            ));
          } else {
            set.entry(&format_args!(
              "{:.*}",
              precision,
              DisplayVariant(variant)
            ));
          }
        }
        set.finish()
      },
    }
  }
}
