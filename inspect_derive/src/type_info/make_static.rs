use {
  ::proc_macro2::Span,
  ::syn::{
    AngleBracketedGenericArguments, AssocConst, GenericArgument, Lifetime,
    ParenthesizedGenericArguments, PathArguments, Type, TypeParamBound,
  },
};

pub fn make_static(ty: &mut Type) {
  fn make_static_angle_bracketed(args: &mut AngleBracketedGenericArguments) {
    for arg in &mut args.args {
      match arg {
        GenericArgument::Lifetime(l) => {
          *l = Lifetime::new("'static", Span::call_site())
        },
        GenericArgument::Type(t) => make_static(t),
        GenericArgument::Const(_) => (),
        GenericArgument::AssocType(a) => {
          if let Some(args) = &mut a.generics {
            make_static_angle_bracketed(args);
          }
          make_static(&mut a.ty);
        },
        GenericArgument::AssocConst(a) => {
          if let AssocConst {
            generics: Some(args),
            ..
          } = a
          {
            make_static_angle_bracketed(args)
          }
        },
        GenericArgument::Constraint(c) => {
          if let Some(args) = &mut c.generics {
            make_static_angle_bracketed(args);
          }
          for bound in &mut c.bounds {
            make_static_bound(bound);
          }
        },
        _ => unimplemented!(),
      }
    }
  }
  fn make_static_parenthesized(_args: &mut ParenthesizedGenericArguments) {
    todo!()
  }
  fn make_static_bound(bound: &mut TypeParamBound) {
    match bound {
      TypeParamBound::Lifetime(l) => {
        *l = Lifetime::new("'static", Span::call_site())
      },
      _ => todo!(),
    }
  }

  match ty {
    Type::Array(a) => make_static(&mut a.elem),
    Type::Paren(p) => make_static(&mut p.elem),
    Type::Path(p) => {
      for segment in &mut p.path.segments {
        match &mut segment.arguments {
          PathArguments::AngleBracketed(args) => {
            make_static_angle_bracketed(args)
          },
          PathArguments::Parenthesized(args) => {
            make_static_parenthesized(args)
          },
          PathArguments::None => (),
        }
      }
    },
    Type::Ptr(p) => make_static(&mut p.elem),
    Type::Reference(r) => {
      r.lifetime
        .replace(Lifetime::new("'static", Span::call_site()));
      make_static(&mut r.elem);
    },
    Type::Slice(s) => make_static(&mut s.elem),
    Type::TraitObject(o) => {
      for bound in &mut o.bounds {
        make_static_bound(bound);
      }
    },
    Type::Tuple(t) => {
      for ty in &mut t.elems {
        make_static(ty);
      }
    },
    _ => unimplemented!(),
  };
}
