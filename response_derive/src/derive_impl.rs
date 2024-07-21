use darling::FromMeta;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, PathArguments};

use crate::attribute_impl::{SchemaAttribute, VariantAttribute};

pub(crate) fn response_variants_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut output = quote!();

    // Bring traits into scope.
    output.extend(quote! {
        use ::orch::response::OrchResponseVariant;
        use ::serde::de::Error;
    });

    let original_enum = parse_macro_input!(input as DeriveInput);
    let DeriveInput { data, ident, .. } = original_enum.clone();
    let syn::Data::Enum(data) = data else {
        panic!("#[derive(OrchResponseVariants)] can only be used with enums");
    };
    let original_enum_ident = ident;

    // // 2.
    // // Create a new derived struct, such that we later add an impl trait for the original
    // // enum which returns the derived struct.

    // let derived_enum_ident = syn::Ident::new(&format!("{}Derived", original_enum_ident), original_enum_ident.span());
    // let derived_enum_variant_fields = data.variants.iter().map(|v| {
    //     let ident = &v.ident;
    //     let variant_struct_name = syn::Ident::new(&format!("{}{}", original_enum_ident, v.ident), v.ident.span());
    //     quote! { #ident(#variant_struct_name), }
    // });
    // output.extend(quote! {
    //     #[derive(Debug, ::serde::Deserialize)]
    //     #[serde(tag = "response_type")]
    //     #vis enum #derived_enum_ident {
    //         #(#derived_enum_variant_fields)*
    //     }
    // });

    // // 3. Add the new derived enum variant structs.
    // for variant in data.variants.iter() {
    //     let ident = &variant.ident;
    //     let variant_struct_name = syn::Ident::new(&format!("{}{}", original_enum_ident, ident), ident.span());
    //     let fields = variant.fields.iter();
    //     output.extend(quote! {
    //         #[derive(Debug, ::serde::Deserialize)]
    //         pub struct #variant_struct_name {
    //             #(#fields),*
    //         }
    //     });
    // }

    // // 4. Declare a new struct that will be used to parse the response.
    // let parser_struct_ident = syn::Ident::new(&format!("{}Parser", original_enum_ident), original_enum_ident.span());
    // output.extend(quote! {
    //     #[derive(Debug)]
    //     pub struct #parser_struct_ident;
    // });

    // // 4. Implement the `ResponseOptions` trait for a new struct that will be used to parse the response.
    // let mut options_vec_pushes = quote!();
    // for syn::Variant { ident, attrs, fields, .. } in data.variants.iter() {
    //     let schema_attrs = attrs
    //         .iter()
    //         .filter_map(|attr| SchemaAttribute::from_meta(&attr.meta).ok())
    //         .collect::<Vec<_>>();
    //     if schema_attrs.len() != fields.len() {
    //         panic!("Expected a single #[schema(...)] attribute for each field of the enum variant");
    //     }
    //     let mut schema_fields = Vec::new();
    //     for variant_field in fields.iter() {
    //         let schema_attr_for_field = schema_attrs
    //             .iter()
    //             .find(|attr| *variant_field.ident.as_ref().unwrap() == attr.field)
    //             .unwrap_or_else(|| {
    //                 panic!(
    //                     "Field {} not found in #[schema(...)] attributes",
    //                     variant_field.ident.as_ref().unwrap()
    //                 )
    //             });
    //         let SchemaAttribute {
    //             field,
    //             description,
    //             example,
    //         } = schema_attr_for_field;
    //         let typ = ast_type_to_str(&variant_field.ty).unwrap_or_else(|_| {
    //             panic!(
    //                 "Failed to convert type to string for field `{}` of variant `{}`",
    //                 variant_field.ident.as_ref().unwrap(),
    //                 ident
    //             )
    //         });
    //         let typ = syn::LitStr::new(&typ, variant_field.span());
    //         schema_fields.push(quote! {
    //             ::orch::response::ResponseSchemaField {
    //                 name: #field.to_string(),
    //                 description: #description.to_string(),
    //                 typ: #typ.to_string(),
    //                 example: #example.to_string(),
    //             }
    //         })
    //     }

    //     // Each enum variant should implement the `OrchResponseOption` trait.
    //     options_vec_pushes.extend(quote! {
    //         options.push(#original_enum_ident::#ident::option())
    //     });
    // }

    let vec_capacity = data.variants.len();
    // let mut parse_match_arms = quote!();
    // Transform the derived enum variant into the original enum variant.
    // for original_variant in data.variants.iter() {
    //     let ident = &original_variant.ident;
    //     let fields = original_variant.fields.iter().map(|field| {
    //         let ident = &field.ident;
    //         quote! { #ident: parsed_response.#ident }
    //     });
    //     parse_match_arms.extend(quote! {
    //         #derived_enum_ident::#ident(parsed_response) => Ok(#original_enum_ident::#ident {
    //             #(#fields),*
    //         }),
    //     });
    // }

    let mut options_vec_pushes = quote!();
    for variant in data.variants.iter() {
        let ident = syn::Ident::new(
            &get_enum_variant_struct_ident(variant).expect("Failed to parse enum variant"),
            variant.ident.span(),
        );

        options_vec_pushes.extend(quote! {
            options.push(#ident::variant());
        });
    }

    // We construct a new struct that will be used to parse the response.
    // NOTE: This is hacky, but a workaround for the fact that the enum cannot be constructed.
    let derived_enum_struct_ident = syn::Ident::new(&format!("{}Derived", original_enum_ident), original_enum_ident.span());

    output.extend(quote! {
        #[derive(Debug)]
        pub struct #derived_enum_struct_ident;
    });

    // Note: We parse with a dynamic evaluation and looking at the `response_type` field, but this could be done
    // by deriving #[serde(tag = "response_type")] on the enum.
    let mut response_type_arms = quote!();
    for variant in data.variants.iter() {
        let variant_ident = variant.ident.clone();
        let variant_ident_str = syn::LitStr::new(&variant.ident.to_string(), variant.ident.span());
        let struct_ident = syn::Ident::new(
            &get_enum_variant_struct_ident(variant).expect("Failed to parse enum variant"),
            variant.ident.span(),
        );
        response_type_arms.extend(quote! {
            #variant_ident_str => Ok(#original_enum_ident::#variant_ident(serde_json::from_str::<#struct_ident>(response)?)),
        });
    }

    output.extend(quote! {
        impl ::orch::response::OrchResponseVariants<#original_enum_ident> for #derived_enum_struct_ident {
            fn variants(&self) -> Vec<::orch::response::ResponseOption> {
                let mut options = Vec::with_capacity(#vec_capacity);
                #options_vec_pushes
                options
            }

            fn parse(&self, response: &str) -> Result<#original_enum_ident, ::serde_json::Error> {
                let dynamic_parsed = serde_json::from_str::<serde_json::Value>(response)?;
                let response_type = dynamic_parsed.get("response_type").unwrap().as_str().unwrap();
                match response_type {
                    #response_type_arms
                    _ => Err(::serde_json::Error::custom("Invalid response type")),
                }
            }
        }
    });

    output.into()
}

pub fn response_variant_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let original_struct = parse_macro_input!(input as DeriveInput);
    let DeriveInput { data, ident, attrs, .. } = original_struct.clone();
    let syn::Data::Struct(data) = data else {
        panic!("#[derive(OrchResponseOption)] can only be used with structs");
    };
    let original_struct_ident = ident.clone();

    let fields = data.fields;

    // Parse the #[variant(...)] attribute.
    let variant_attr = attrs
        .iter()
        .filter_map(|attr| VariantAttribute::from_meta(&attr.meta).ok())
        .next()
        .expect("#[variant(...)] attribute not found on variant field");
    let VariantAttribute {
        variant,
        scenario,
        description,
    } = variant_attr;

    // Parse the fields used in [`orch::response::OrchResponseVariant`].
    let mut schema_fields = Vec::new();
    for variant_field in fields.iter() {
        // Parse the #[schema(...)] attribute.
        let schema_attr = variant_field
            .attrs
            .iter()
            .filter_map(|attr| SchemaAttribute::from_meta(&attr.meta).ok())
            .collect::<Vec<_>>();
        if schema_attr.len() != 1 {
            panic!("Expected a single #[schema(...)] attribute for each field of the enum variant with the correct format and parameters");
        }
        let SchemaAttribute { description, example } = schema_attr.first().expect("Failed to parse schema attribute");

        let typ = ast_type_to_str(&variant_field.ty).unwrap_or_else(|_| {
            panic!(
                "Failed to convert type to string for field `{}` of variant `{}`",
                variant_field.ident.as_ref().unwrap(),
                ident
            )
        });
        let typ = syn::LitStr::new(&typ, variant_field.span());
        let field_ident = syn::LitStr::new(&variant_field.ident.as_ref().unwrap().to_string(), variant_field.span());
        schema_fields.push(quote! {
            ::orch::response::ResponseSchemaField {
                name: #field_ident.to_string(),
                description: #description.to_string(),
                typ: #typ.to_string(),
                example: #example.to_string(),
            }
        })
    }

    quote! {
        impl ::orch::response::OrchResponseVariant for #original_struct_ident {
            fn variant() -> ::orch::response::ResponseOption {
                ::orch::response::ResponseOption {
                    type_name: #variant.to_string(),
                    scenario: #scenario.to_string(),
                    description: #description.to_string(),
                    schema: vec![
                        #(#schema_fields),*
                    ]
                }
            }
        }
    }
    .into()
}

// Parse `Answer(AnswerResponseOption)` into `AnswerResponseOption`.
fn get_enum_variant_struct_ident(variant: &syn::Variant) -> Result<String, String> {
    // We expect the enum variant to look like this: `Answer(AnswerResponseOption)`,
    // so we parse the `AnswerResponseOption` struct.
    let syn::Fields::Unnamed(fields) = &variant.fields else {
        panic!("Expected an unnamed struct for each enum variant");
    };
    let Some(syn::Field { ty, .. }) = fields.unnamed.first() else {
        panic!("Expected an unnamed struct for each enum variant");
    };
    let syn::Type::Path(p) = ty else {
        panic!("Expected an unnamed struct for each enum variant");
    };
    let ident = &p.path.segments.first().unwrap().ident;
    Ok(ident.to_string())
}

fn ast_type_to_str(ty: &syn::Type) -> Result<String, String> {
    match ty {
        syn::Type::Path(tp) => {
            let ps = tp.path.segments.first();
            let Some(first_path_segment) = ps else {
                return Err(format!("Unsupported/unexpected type: {:?}", ty).to_owned());
            };
            let t = first_path_segment.ident.to_string();
            match t.as_ref() {
                "String" => {
                    // SUPPORTED: String
                    Ok("string".to_owned())
                }
                "bool" => {
                    // SUPPORTED: bool
                    Ok("boolean".to_owned())
                }
                "Vec" => {
                    let PathArguments::AngleBracketed(ab) = &tp.path.segments.first().unwrap().arguments else {
                        return Err(format!("Unsupported/unexpected type: {:?}", ty).to_owned());
                    };
                    let syn::GenericArgument::Type(t) = ab.args.first().unwrap() else {
                        return Err(format!("Unsupported/unexpected type: {:?}", ty).to_owned());
                    };
                    let syn::Type::Path(p) = t else {
                        return Err(format!("Unsupported/unexpected type: {:?}", ty).to_owned());
                    };
                    let t = p.path.segments.first().unwrap().ident.to_string();
                    match t.as_ref() {
                        // SUPPORTED: Vec<String>
                        "String" => Ok("string[]".to_owned()),
                        _ => Err(format!("Unsupported/unexpected type: {}", t).to_owned()),
                    }
                }
                _ => Err(format!("Unsupported/unexpected type: {}", t).to_owned()),
            }
        }
        _ => Err(format!("Unsupported/unexpected type: {:?}", ty).to_owned()),
    }
}
