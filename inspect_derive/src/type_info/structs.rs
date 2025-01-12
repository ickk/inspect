use {
  super::make_static::make_static,
  ::proc_macro2::{Span, TokenStream as TokenStream2},
  ::quote::quote,
  ::syn::{FieldsNamed, FieldsUnnamed, Generics, Ident, Index, Lifetime},
};

/// derive implementation for unit structs: i.e.
///
/// ```ignore
/// struct MyStruct;
/// ```
pub fn derive_unit_struct(name: Ident) -> TokenStream2 {
  quote! {
    unsafe impl ::inspect::type_info::internal::ProviderOfTypeInfo<#name>
    for ::inspect::type_info::internal::Provider<#name>
    {
      type StaticTy = #name;
      type StaticTySized = #name;

      fn type_info() -> &'static ::inspect::TypeInfo {
        use {
          ::inspect::{TypeInfo, type_info::{IdInfo, SizedInfo, Struct}},
          ::core::{
            any::{TypeId, type_name},
            mem::{size_of, align_of}
          },
          ::std::sync::LazyLock,
        };

        static INFO: LazyLock<TypeInfo> = LazyLock::new(|| {
          TypeInfo::Struct(Struct::UnitStruct {
            id: IdInfo {
              type_id: TypeId::of::<#name>(),
              type_name: type_name::<#name>(),
            },
            sized: SizedInfo {
              size: size_of::<#name>(),
              align: align_of::<#name>(),
            },
          })
        });

        &INFO
      }
    }
  }
}

/// derive implementation for tuple structs: i.e.
///
/// ```ignore
/// struct MyStruct(A, B);
/// ```
pub fn derive_tuple_struct(
  name: Ident,
  generics: Generics,
  fields: &FieldsUnnamed,
) -> TokenStream2 {
  let field_types = fields
    .unnamed
    .iter()
    .map(|field| field.ty.clone())
    .collect::<Vec<_>>();
  let field_types_static = {
    let mut f = field_types.clone();
    f.iter_mut().for_each(make_static);
    f
  };
  let field_indices = fields
    .unnamed
    .iter()
    .enumerate()
    .map(|(i, _field)| Index::from(i));
  let generic_lifetimes = generics
    .lifetimes()
    .map(|param| param.lifetime.clone())
    .collect::<Vec<_>>();
  let static_lifetimes = generic_lifetimes
    .iter()
    .map(|_| Lifetime::new("'static", Span::call_site()));
  let full_name = quote!(#name<#(#generic_lifetimes),*>);
  let full_name_static = quote!(#name<#(#static_lifetimes),*>);

  quote! {
    unsafe impl<#(#generic_lifetimes),*>
    ::inspect::type_info::internal::ProviderOfTypeInfo<#full_name>
    for ::inspect::type_info::internal::Provider<#full_name>
    {
      type StaticTy = #full_name_static;
      type StaticTySized = #full_name_static;

      fn type_info() -> &'static ::inspect::TypeInfo {
        use {
          ::inspect::type_info::{
            internal::{Provider, ProviderOfTypeInfo},
            TypeInfo, Struct, IdInfo, SizedInfo, TupleStructInfo, AnonymousFieldInfo,
          },
          ::core::{
            any::{TypeId, type_name},
            mem::{offset_of, size_of, align_of},
          },
          ::std::sync::LazyLock,
        };

        static INFO: LazyLock<TypeInfo> = LazyLock::new(|| {
          let field_infos: &'static [AnonymousFieldInfo] = Box::leak(
            vec![
              #(AnonymousFieldInfo {
                field_index: #field_indices,
                field_offset: offset_of!(#full_name_static, #field_indices),
                // type_info_fn: (&&&Specialise::<#field_types>::new()).type_info_fn(),
                type_info_fn: Provider::<#field_types_static>::type_info,
              }),*
            ].into_boxed_slice()
          );

          TypeInfo::Struct(Struct::TupleStruct {
            id: IdInfo {
              type_id: TypeId::of::<#full_name_static>(),
              type_name: type_name::<#full_name_static>(),
            },
            sized: SizedInfo {
              size: size_of::<#full_name_static>(),
              align: align_of::<#full_name_static>(),
            },
            fields: TupleStructInfo {
              field_infos,
            },
          })
        });

        &INFO
      }
    }
  }
}

/// derive implemenation for regular structs: i.e.
///
/// ```ignore
/// struct MyStruct {
///   a: A,
///   b: B,
/// }
/// ```
pub fn derive_struct(
  name: Ident,
  generics: Generics,
  fields: &FieldsNamed,
) -> TokenStream2 {
  let field_types = fields
    .named
    .iter()
    .map(|field| field.ty.clone())
    .collect::<Vec<_>>();
  let field_types_static = {
    let mut f = field_types.clone();
    f.iter_mut().for_each(make_static);
    f
  };
  let field_idents = fields
    .named
    .iter()
    .map(|field| field.ident.as_ref().unwrap());
  let field_names = fields
    .named
    .iter()
    .map(|field| field.ident.as_ref().unwrap().to_string());
  let generic_lifetimes = generics
    .lifetimes()
    .map(|param| param.lifetime.clone())
    .collect::<Vec<_>>();
  let static_lifetimes = generic_lifetimes
    .iter()
    .map(|_| Lifetime::new("'static", Span::call_site()));
  let full_name = quote!(#name<#(#generic_lifetimes),*>);
  let full_name_static = quote!(#name<#(#static_lifetimes),*>);

  quote! {
    unsafe impl<#(#generic_lifetimes),*>
    ::inspect::type_info::internal::ProviderOfTypeInfo<#full_name>
    for ::inspect::type_info::internal::Provider<#full_name>
    {
      type StaticTy = #full_name_static;
      type StaticTySized = #full_name_static;

      fn type_info() -> &'static ::inspect::TypeInfo {
        use {
          ::inspect::type_info::{
            internal::{Provider, ProviderOfTypeInfo},
            TypeInfo, Struct, IdInfo, SizedInfo, StructInfo, NamedFieldInfo,
          },
          ::core::{
            any::{TypeId, type_name},
            mem::{offset_of, size_of, align_of},
          },
          ::std::sync::LazyLock,
        };

        static INFO: LazyLock<TypeInfo> = LazyLock::new(|| {
          let field_infos: &'static [NamedFieldInfo] = Box::leak(
            vec![
              #(NamedFieldInfo {
                field_name: #field_names,
                field_offset: offset_of!(#full_name_static, #field_idents),
                type_info_fn: Provider::<#field_types_static>::type_info,
              }),*
            ].into_boxed_slice()
          );

          TypeInfo::Struct(Struct::Struct {
            id: IdInfo {
              type_id: TypeId::of::<#full_name_static>(),
              type_name: type_name::<#full_name_static>(),
            },
            sized: SizedInfo {
              size: size_of::<#full_name_static>(),
              align: align_of::<#full_name_static>(),
            },
            fields: StructInfo {
              field_infos,
            },
          })
        });

        &INFO
      }
    }
  }
}

/// derive implementation for generic tuple structs: i.e.
///
/// ```ignore
/// struct MyStruct<A, B>(A, B);
/// ```
pub fn derive_generic_tuple_struct(
  name: Ident,
  generics: Generics,
  fields: &FieldsUnnamed,
) -> TokenStream2 {
  let field_types = fields.unnamed.iter().map(|field| &field.ty);
  let field_indices = fields
    .unnamed
    .iter()
    .enumerate()
    .map(|(i, _field)| Index::from(i));
  let generic_types = generics
    .type_params()
    .map(|param| param.ident.clone())
    .collect::<Vec<_>>();
  let generic_lifetimes = generics
    .lifetimes()
    .map(|param| param.lifetime.clone())
    .collect::<Vec<_>>();
  let static_lifetimes = generic_lifetimes
    .iter()
    .map(|_| Lifetime::new("'static", Span::call_site()));
  let full_name = quote!(#name<#(#generic_lifetimes,)* #(#generic_types),*>);

  let static_ty_sized = quote!(
    #name<
      #(#static_lifetimes,)*
      #(
      <::inspect::type_info::internal::Provider<#generic_types> as
      ::inspect::type_info::internal::ProviderOfTypeInfo<#generic_types>>::StaticTySized
      ),*
    >
  );

  quote! {
    unsafe impl<#(#generic_lifetimes,)* #(#generic_types),*> ::inspect::type_info::internal::ProviderOfTypeInfo<#full_name>
    for ::inspect::type_info::internal::Provider<#full_name>
    where #(
      #generic_types: Sized,
      ::inspect::type_info::internal::Provider<#generic_types>: ::inspect::type_info::internal::ProviderOfTypeInfo<#generic_types>
    ),*
    {
      type StaticTy = #static_ty_sized;
      type StaticTySized = #static_ty_sized;

      fn type_info() -> &'static ::inspect::TypeInfo {
        use {
          ::inspect::type_info::{
            internal::{ConcurrentMap, Provider, ProviderOfTypeInfo},
            TypeInfo, Struct, IdInfo, SizedInfo, TupleStructInfo, AnonymousFieldInfo,
          },
          ::core::{
            any::{TypeId, type_name},
            mem::{offset_of, size_of, align_of},
          },
        };

        static DICTIONARY: ConcurrentMap<TypeId, &'static TypeInfo>
          = ConcurrentMap::new();

        let type_id = TypeId::of::<Self::StaticTy>();
        DICTIONARY.get_or_insert_with(type_id, || {
          let field_infos: &'static [AnonymousFieldInfo] = Box::leak(
            vec![
              #(AnonymousFieldInfo {
                field_index: #field_indices,
                field_offset: offset_of!(#full_name, #field_indices),
                // autoderef specialisation doesn't work in generic context :(
                type_info_fn: Provider::<#field_types>::type_info,
              }),*
            ].into_boxed_slice(),
          );
          let info = TypeInfo::Struct(Struct::TupleStruct {
            id: IdInfo {
              type_id,
              type_name: type_name::<#full_name>(),
            },
            sized: SizedInfo {
              size: size_of::<#full_name>(),
              align: align_of::<#full_name>(),
            },
            fields: TupleStructInfo {
              field_infos,
            },
          });

          Box::leak(Box::new(info))
        })
      }
    }
  }
}

/// derive implemenation for generic structs: i.e.
///
/// ```ignore
/// struct MyStruct<A, B> {
///   a: A,
///   b: B,
/// }
/// ```
pub fn derive_generic_struct(
  name: Ident,
  generics: Generics,
  fields: &FieldsNamed,
) -> TokenStream2 {
  let field_types = fields.named.iter().map(|field| &field.ty);
  let field_idents = fields
    .named
    .iter()
    .map(|field| field.ident.as_ref().unwrap());
  let field_names = fields
    .named
    .iter()
    .map(|field| field.ident.as_ref().unwrap().to_string());
  let generic_types = generics
    .type_params()
    .map(|param| param.ident.clone())
    .collect::<Vec<_>>();
  let full_name = quote!(#name<#(#generic_types),*>);

  quote! {
    unsafe impl<#(#generic_types),*> ::inspect::type_info::internal::ProviderOfTypeInfo<#full_name>
    for ::inspect::type_info::internal::Provider<#full_name>
    where #(
      ::inspect::type_info::internal::Provider<#generic_types>:
      ::inspect::type_info::internal::ProviderOfTypeInfo<#generic_types> + 'static
    ),*
    {
      type StaticTy = #full_name;
      type StaticTySized = #full_name;

      fn type_info() -> &'static ::inspect::TypeInfo {
        use {
          ::inspect::type_info::{
            internal::{ConcurrentMap, Provider, ProviderOfTypeInfo},
            TypeInfo, Struct, IdInfo, SizedInfo, StructInfo, NamedFieldInfo,
          },
          ::core::{
            any::{TypeId, type_name},
            mem::{offset_of, size_of, align_of},
          },
        };

        static DICTIONARY: ConcurrentMap<TypeId, &'static TypeInfo> =
          ConcurrentMap::new();

        let type_id = TypeId::of::<#full_name>();
        DICTIONARY.get_or_insert_with(type_id, || {
          let field_infos: &'static [NamedFieldInfo] = Box::leak(
            vec![
              #(NamedFieldInfo {
                field_name: #field_names,
                field_offset: offset_of!(#full_name, #field_idents),
                type_info_fn: Provider::<#field_types>::type_info,
              }),*
            ].into_boxed_slice(),
          );
          let info = TypeInfo::Struct(Struct::Struct {
            id: IdInfo {
              type_id,
              type_name: type_name::<#full_name>(),
            },
            sized: SizedInfo {
              size: size_of::<#full_name>(),
              align: align_of::<#full_name>(),
            },
            fields: StructInfo {
              field_infos,
            },
          });

          Box::leak(Box::new(info))
        })
      }
    }
  }
}
