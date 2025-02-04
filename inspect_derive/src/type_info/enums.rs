use {
  ::proc_macro2::TokenStream as TokenStream2,
  ::quote::{format_ident, quote},
  ::syn::{DataEnum, Fields, Generics, Ident, Index},
};

pub fn derive_enum(
  name: Ident,
  generics: Generics,
  data_enum: &DataEnum,
) -> TokenStream2 {
  if generics.lifetimes().next().is_some()
    | generics.const_params().next().is_some()
    | generics.type_params().next().is_some()
  {
    panic!("Generics are not supported for enums (yet)");
  }

  let variant_info_tokenstreams = data_enum
    .variants
    .iter()
    .map(|variant| {
      let variant_ident = variant.ident.clone();
      let variant_name = variant_ident.to_string();
      match &variant.fields {
        Fields::Unit => quote! {
          EnumVariantInfo::Unit {
            variant_name: #variant_name,
            variant_descriminant: None,
          }
        },
        Fields::Unnamed(fields) => {
          let defaults = ::std::iter::repeat_n(
            quote!(Default::default()),
            fields.unnamed.len(),
          );
          let full_variant_name = quote!(#name::#variant_ident);
          let field_idents = fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _)| format_ident!("field_{i}")).collect::<Vec<_>>();
          let field_indices = fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _)| Index::from(i));
          let field_types = fields
            .unnamed
            .iter()
            .map(|field| field.ty.clone())
            .collect::<Vec<_>>();

          quote! {
            EnumVariantInfo::Tuple {
              variant_name: #variant_name,
              variant_descriminant: None,
              field_infos: {
                let variant = &#full_variant_name(#(#defaults),*);

                match variant {
                  #full_variant_name(#(ref #field_idents),*) => {
                    let base = ptr::from_ref(variant) as usize;
                    #(let #field_idents = ptr::from_ref(#field_idents) as usize;)*
                    Box::leak(
                      vec![
                        #(AnonymousFieldInfo {
                          field_index: #field_indices,
                          field_offset: #field_idents - base,
                          type_info_fn: Provider::<#field_types>::type_info,
                        },)*
                      ]
                      .into_boxed_slice()
                    )
                  },
                  _ => unreachable!()
                }
              }
            }
          }
        },
        Fields::Named(fields) => {
          let defaults = ::std::iter::repeat_n(
            quote!(Default::default()),
            fields.named.len(),
          );
          let full_variant_name = quote!(#name::#variant_ident);
          let field_idents = fields
            .named
            .iter()
            .map(|field| field.ident.clone()).collect::<Vec<_>>();
          let field_ident_idents = fields
            .named
            .iter()
            .map(|field|
              format_ident!("field_{}", field.ident.as_ref().unwrap())
            ).collect::<Vec<_>>();
          let field_names = fields
            .named
            .iter()
            .map(|field| field.ident.as_ref().unwrap().to_string());
          let field_types = fields
            .named
            .iter()
            .map(|field| field.ty.clone())
            .collect::<Vec<_>>();

          quote! {
            EnumVariantInfo::Struct {
              variant_name: #variant_name,
              variant_descriminant: None,
              field_infos: {
                let variant = &#full_variant_name{#(#field_idents: #defaults),*};

                match variant {
                  #full_variant_name{
                    #(#field_idents: ref #field_ident_idents),*
                  } => {
                    let base = ptr::from_ref(variant) as usize;
                    #(let #field_ident_idents =
                      ptr::from_ref(#field_ident_idents) as usize;)*

                    Box::leak(
                      vec![
                        #(NamedFieldInfo {
                          field_name: #field_names,
                          field_offset: #field_ident_idents - base,
                          type_info_fn: Provider::<#field_types>::type_info,
                        },)*
                      ]
                      .into_boxed_slice(),
                    )
                  },
                  _ => unreachable!(),
                }
              }
            }
          }
        }
      }
    })
    .collect::<Vec<_>>();

  quote! {
    unsafe impl ::inspect::type_info::internal::ProviderOfTypeInfo<#name>
    for ::inspect::type_info::internal::Provider<#name> {
      type StaticTy = #name;
      type StaticTySized = #name;

      fn type_info() -> &'static TypeInfo {
        use {
          ::core::{
            any::{type_name, TypeId},
            mem::{align_of, size_of},
            ptr,
          },
          ::inspect::type_info::{
            internal::{Provider, ProviderOfTypeInfo},
            TypeInfo, Enum, IdInfo, SizedInfo, EnumInfo, EnumVariantInfo,
            AnonymousFieldInfo, NamedFieldInfo,
          },
        };

        let variant_infos = Box::leak(
          vec![
            #(#variant_info_tokenstreams,)*
          ]
          .into_boxed_slice(),
        );

        let type_id = TypeId::of::<#name>();
        let info = TypeInfo::Enum(Enum::Enum {
          id: IdInfo {
            type_id,
            type_name: type_name::<#name>(),
          },
          sized: SizedInfo {
            size: size_of::<#name>(),
            align: align_of::<#name>(),
          },
          variants: EnumInfo { variant_infos },
        });

        Box::leak(Box::new(info))
      }
    }
  }
}
