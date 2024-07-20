use std::option;

use darling::FromMeta;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, token::Final, DeriveInput};

use crate::attribute_impl::{ResponseAttribute, SchemaAttribute};

/*
// ORIGINAL:

pub enum MyResponseOption {
    #[response(type_name = "foo")]
    Answer { capital: String },
}

#[derive(Debug, ::serde::Deserialize)]
struct MyResponseOptionAnswer {
    capital: String,
}

#[derive(Debug, ::serde::Deserialize)]
struct MyResponseOptionFail {
    reason: String,
}

impl ResponseOptions for MyResponseOption {
    fn options() -> Vec<ResponseOption> {
        vec![
            ResponseOption::Answer {
                response_type: "Answer".to_string(),
                schema: vec![
                    ResponseSchemaField {
                        name: "capital".to_string(),
                        description: "Capital city of the country".to_string(),
                        example: "London".to_string(),
                        typ: "string".to_string(),
                    },
                ],
            },
            ResponseOption::Fail {
                response_type: "Fail".to_string(),
                schema: vec![
                    ResponseSchemaField {
                        name: "reason".to_string(),
                        description: "Reason why the capital city is not known".to_string(),
                        example: "Country 'foobar' does not exist".to_string(),
                        typ: "string".to_string(),
                    },
                ],
            },
        ]
    }

    fn parse(&self, response: String) -> Result<MyResponseOption, Box<dyn std::error::Error>> {
        let parsed_dynamic = serde_json::from_str::<serde_json::Value>(&response);
        let response_type = parsed_dynamic["response_type"].as_str().unwrap();
        match response_type {
            "Answer" => {
                let parsed_answer = serde_json::from_str::<MyResponseOptionAnswer>(&response).unwrap();
                Ok(MyResponseOption::Answer(parsed_answer))
            }
            "Fail" => {
                let parsed_fail = serde_json::from_str::<MyResponseOptionFail>(&response).unwrap();
                Ok(MyResponseOption::Fail(parsed_fail))
            }
            _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Unknown type name: {}", response_type)))),
        }
    }
}
*/

pub(crate) fn derive_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut output = quote!();

    // 1.
    // Parse the input and validate its type (only an enum is supported).
    let original_enum = parse_macro_input!(input as DeriveInput);
    let DeriveInput { data, ident, vis, .. } = original_enum.clone();
    let syn::Data::Enum(data) = data else {
        panic!("#[derive(OrchResponseOptions)] can only be used with enums");
    };

    // 2.
    // Create a new derived struct, such that we later add an impl trait for the original
    // enum which returns the derived struct.

    let original_enum_ident = ident;
    let derived_enum_ident = syn::Ident::new(&format!("{}Derived", original_enum_ident), original_enum_ident.span());
    let derived_enum_variant_fields = data.variants.iter().map(|v| {
        let ident = &v.ident;
        let variant_struct_name = syn::Ident::new(&format!("{}{}", original_enum_ident, v.ident), v.ident.span());
        quote! { #ident(#variant_struct_name), }
    });
    output.extend(quote! {
        #[derive(Debug, ::serde::Deserialize)]
        #[serde(tag = "response_type")]
        #vis enum #derived_enum_ident {
            #(#derived_enum_variant_fields)*
        }
    });

    // 3. Add the new derived enum variant structs.
    for variant in data.variants.iter() {
        let ident = &variant.ident;
        let variant_struct_name = syn::Ident::new(&format!("{}{}", original_enum_ident, ident), ident.span());
        let fields = variant.fields.iter();
        output.extend(quote! {
            #[derive(Debug, ::serde::Deserialize)]
            pub struct #variant_struct_name {
                #(#fields),*
            }
        });
    }

    // 4. Declare a new struct that will be used to parse the response.
    let parser_struct_ident = syn::Ident::new(&format!("{}Parser", original_enum_ident), original_enum_ident.span());
    output.extend(quote! {
        #[derive(Debug)]
        pub struct #parser_struct_ident;
    });

    // 4. Implement the `ResponseOptions` trait for a new struct that will be used to parse the response.
    let mut options_vec_pushes = quote!();
    for syn::Variant { ident, attrs, fields, .. } in data.variants.iter() {
        let response_attr = attrs
            .iter()
            .filter_map(|attr| ResponseAttribute::from_meta(&attr.meta).ok())
            .next()
            .expect("#[response] attribute not found on variant field");
        let ResponseAttribute { scenario, description } = response_attr;

        let schema_attrs = attrs
            .iter()
            .filter_map(|attr| SchemaAttribute::from_meta(&attr.meta).ok())
            .collect::<Vec<_>>();
        if schema_attrs.len() != fields.len() {
            panic!("Expected a single #[schema(...)] attribute for each field of the enum variant");
        }
        let mut schema_fields = Vec::new();
        for variant_field in fields.iter() {
            let schema_attr_for_field = schema_attrs
                .iter()
                .find(|attr| *variant_field.ident.as_ref().unwrap() == attr.field)
                .unwrap_or_else(|| {
                    panic!(
                        "Field {} not found in #[schema(...)] attributes",
                        variant_field.ident.as_ref().unwrap()
                    )
                });
            let SchemaAttribute {
                field,
                description,
                example,
            } = schema_attr_for_field;
            let typ = ast_type_to_str(&variant_field.ty).unwrap_or_else(|_| {
                panic!(
                    "Failed to convert type to string for field {} of variant {}",
                    variant_field.ident.as_ref().unwrap(),
                    ident
                )
            });
            let typ = syn::LitStr::new(&typ, variant_field.span());
            schema_fields.push(quote! {
                ::orch_response::ResponseSchemaField {
                    name: #field.to_string(),
                    description: #description.to_string(),
                    typ: #typ.to_string(),
                    example: #example.to_string(),
                },
            })
        }

        let shhema_fields = schema_fields.iter();
        let ident_str = syn::LitStr::new(&ident.to_string(), ident.span());
        options_vec_pushes.extend(quote! {
            options.push(::orch_response::ResponseOption {
                type_name: #ident_str.to_string(),
                scenario: #scenario.to_string(),
                description: #description.to_string(),
                schema: vec![
                    #(#shhema_fields),*
                ]
            });
        });
    }

    let vec_capacity = data.variants.len();
    output.extend(quote! {
        impl ::orch_response::ResponseOptions<#derived_enum_ident> for #parser_struct_ident {
            fn options(&self) -> Vec<::orch_response::ResponseOption> {
                let mut options = Vec::with_capacity(#vec_capacity);
                #options_vec_pushes
                options
            }
        }
    });

    output.into()
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
                "String" => Ok("string".to_owned()),
                _ => Err(format!("Unsupported/unexpected type: {}", t).to_owned()),
            }
        }
        _ => Err(format!("Unsupported/unexpected type: {:?}", ty).to_owned()),
    }
}
