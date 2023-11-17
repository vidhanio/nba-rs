use darling::{ast::Data, util::Ignored, Error, FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, ToTokens};
use syn::{DeriveInput, Field, Generics, Ident, Type};

#[derive(FromDeriveInput)]
#[darling(attributes(endpoint), supports(struct_named))]
struct EndpointOpts {
    ident: Ident,
    generics: Generics,
    data: Data<Ignored, Field>,

    path: String,

    #[darling(multiple)]
    result_set: Vec<ResultSet>,
}

#[derive(FromMeta)]
struct ResultSet {
    field: Ident,
    ty: Type,
    name: String,
}

impl ToTokens for EndpointOpts {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            ident,
            generics,
            data,
            path,
            result_set: row,
        } = self;

        let (impl_generics, type_generics, where_generics) = generics.split_for_impl();

        let fields = data
            .as_ref()
            .take_struct()
            .expect("should only be struct")
            .fields;

        let result_sets_name = format_ident!("{ident}ResultSets");

        let rows = row.iter().map(
            |ResultSet {
                 field,
                 ty,
                 name: result_set,
             }| {
                quote::quote! {
                    #[serde(rename = #result_set)]
                    pub #field: Vec<#ty>,
                }
            },
        );

        let builder_methods = fields.iter().map(|Field { ident, ty, .. }| {
            let ident = ident.as_ref().expect("should only be named fields");
            let method_name = format_ident!("with_{ident}");

            quote::quote! {
                pub fn #method_name(self, #ident: #ty) -> Self {
                    Self {
                        #ident,
                        ..self
                    }
                }
            }
        });

        let builder_impl = quote::quote! {
            impl #impl_generics #ident #type_generics #where_generics {
                #(#builder_methods)*
            }
        };

        let endpoint_impl = quote::quote! {
            impl #impl_generics crate::Endpoint for #ident #type_generics #where_generics {
                type Parameters = Self;
                type ResultSets = #result_sets_name #type_generics;

                fn path(&self) -> ::std::borrow::Cow<'static, str> {
                    #path.into()
                }

                fn parameters(&self) -> Self::Parameters {
                    self.clone()
                }
            }
        };

        let result_sets = quote::quote! {
            #[allow(missing_copy_implementations)]
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, Debug, Default, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
            pub struct #result_sets_name #generics #where_generics {
                #(#rows)*
            }
        };

        quote::quote! {
            #builder_impl
            #endpoint_impl
            #result_sets
        }
        .to_tokens(tokens);
    }
}

#[proc_macro_derive(Endpoint, attributes(endpoint))]
pub fn derive_endpoint(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    EndpointOpts::from_derive_input(&input)
        .map_or_else(Error::write_errors, ToTokens::into_token_stream)
        .into()
}
