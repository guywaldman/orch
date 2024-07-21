mod attribute_impl;
mod derive_impl;

use quote::quote;

/// Used to derive the `OrchResponseVariants` trait for a given enum
#[proc_macro_derive(Variants)]
pub fn derive_orch_response_variants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_impl::response_variants_derive(input)
}

/// Used to derive the `OrchResponseVariant` trait for a given enum.
#[proc_macro_derive(Variant, attributes(variant, schema))]
pub fn derive_orch_response_variant_variant(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_impl::response_variant_derive(input)
}

/// Used to construct the identifier of the derived enum.
#[proc_macro]
pub fn variants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Expects the identifier of the derived enum.
    let enum_ident = syn::parse_macro_input!(input as syn::Ident);
    let derived_enum_ident = syn::Ident::new(&format!("{}Derived", enum_ident), enum_ident.span());
    quote! {
        #derived_enum_ident {}
    }
    .into()
}
