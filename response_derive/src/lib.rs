use quote::quote;

mod attribute_impl;
mod derive_impl;

/// Used to derive the `ResponseOptions` trait for a given enum.
#[proc_macro_derive(OrchResponseOptions, attributes(response, schema))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_impl::derive_impl(input)
}

/// Used to create a new `ResponseOptions` instance.
#[proc_macro]
pub fn options(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Expects the identifier of the derived enum.
    let enum_ident = syn::parse_macro_input!(input as syn::Ident);
    let derived_enum_ident = syn::Ident::new(&format!("{}Parser", enum_ident), enum_ident.span());
    quote! {
        #derived_enum_ident {}
    }
    .into()
}
