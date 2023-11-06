use darling::{
    ast::Data,
    util::{Flag, Ignored},
    Error, FromDeriveInput, FromField, FromMeta,
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, ToTokens};
use syn::{DeriveInput, Generics, Ident, Type};

#[derive(FromDeriveInput)]
#[darling(attributes(endpoint), supports(struct_named))]
struct EndpointOpts {
    ident: Ident,
    generics: Generics,
    data: Data<Ignored, EndpointField>,

    path: String,

    #[darling(multiple)]
    row: Vec<EndpointRow>,
}

#[derive(FromField)]
#[darling(attributes(endpoint))]
struct EndpointField {
    ident: Option<Ident>,
    ty: Type,

    params: Flag,
}

#[derive(FromMeta)]
struct EndpointRow {
    field: Ident,
    ty: Type,
    row: String,
}

fn try_derive_endpoint(input: DeriveInput) -> darling::Result<TokenStream2> {
    let EndpointOpts {
        ident,
        generics,
        data,
        path,
        row,
    } = EndpointOpts::from_derive_input(&input)?;

    let (imp, ty, wher) = generics.split_for_impl();

    let fields = data.take_struct().expect("should only be struct").fields;

    let (params_field, params_ty) = fields
        .into_iter()
        .find_map(|f| {
            f.params
                .is_present()
                .then(|| (Some(f.ident), f.ty.into_token_stream()))
        })
        .unwrap_or((None, quote::quote! { Self }));

    let result_set_name = format_ident!("{ident}ResultSets");

    let rows = row.into_iter().map(|EndpointRow { field, ty, row }| {
        quote::quote! {
            #[serde(rename = #row)]
            pub #field: Vec<#ty>,
        }
    });

    let parameters = if let Some(params_field) = params_field.as_ref() {
        quote::quote! { self.#params_field }
    } else {
        quote::quote! { self }
    };

    Ok(quote::quote! {
        impl #imp crate::Endpoint for #ident #ty #wher {
            type Parameters = #params_ty;
            type ResultSets = #result_set_name #ty;

            fn endpoint(&self) -> ::std::borrow::Cow<'static, str> {
                #path.into()
            }

            fn parameters(&self) -> Self::Parameters {
                #parameters.clone()
            }
        }

        #[allow(missing_copy_implementations)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Debug, Default, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
        pub struct #result_set_name #generics #wher {
            #(#rows)*
        }
    })
}

#[proc_macro_derive(Endpoint, attributes(endpoint))]
pub fn derive_endpoint(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    try_derive_endpoint(input)
        .unwrap_or_else(Error::write_errors)
        .into()
}
