use crate::models::json_schema::{JsonSchema, JsonSchemaType};

lazy_static::lazy_static! {
    static ref RE_UNDERSCORE: regex::Regex = regex::Regex::new(r"[_.]").unwrap();
    static ref RE_INVALID: regex::Regex =
        regex::Regex::new(r"(?x)^(
                (?:\d.*)|
                as|
                async|
                await|
                break|
                const|
                continue|
                crate|
                default|
                dyn|
                else|
                enum|
                extern|
                false|
                final|
                fn|
                for|
                if|
                impl|
                in|
                let|
                loop|
                macro|
                match|
                mod|
                move|
                mut|
                pub|
                ref|
                return|
                self|
                static|
                struct|
                super|
                trait|
                true|
                type|
                union|
                unsafe|
                use|
                where|
                while
            )$").unwrap();
}

/// returns (ident, type, additional_tokens)
pub fn generate_tokens_for_schema<S: AsRef<str>>(
    parent_ident: Option<&proc_macro2::Ident>,
    name: S,
    schema: &JsonSchema,
) -> anyhow::Result<(
    proc_macro2::Ident,
    proc_macro2::TokenStream,
    Option<proc_macro2::TokenStream>,
)> {
    let name = change_case::snake_case(name.as_ref());
    let name = RE_INVALID.replace_all(&*name, "_${1}");

    if let Some(ref r#ref) = schema.r#ref {
        let r#ref = change_case::pascal_case(&*r#ref);
        let r#ref = RE_UNDERSCORE.replace_all(&*r#ref, "");
        let r#ref = quote::format_ident!("{}", r#ref);

        Ok((
            quote::format_ident!("{}", &*name),
            quote::quote!(::std::boxed::Box<#r#ref>),
            None,
        ))
    } else if let Some(ref ty) = schema.r#type {
        match ty {
            JsonSchemaType::Any | JsonSchemaType::Null => Ok((
                quote::format_ident!("{}", name),
                quote::quote!(::serde_json::Value),
                None,
            )),
            JsonSchemaType::Array => {
                if let Some(ref items) = schema.items {
                    let (ident, ty, tokens) =
                        generate_tokens_for_schema(parent_ident, name, &**items)?;

                    Ok((ident, quote::quote!(::std::vec::Vec<#ty>), tokens))
                } else {
                    unimplemented!()
                }
            }
            JsonSchemaType::Boolean => {
                Ok((quote::format_ident!("{}", name), quote::quote!(::std::primitive::bool), None))
            }
            JsonSchemaType::Integer => {
                Ok((quote::format_ident!("{}", name), quote::quote!(::std::primitive::i64), None))
            }
            JsonSchemaType::Number => {
                Ok((quote::format_ident!("{}", name), quote::quote!(::std::primitive::f64), None))
            }
            JsonSchemaType::Object => {
                if let Some(ref properties) = schema.properties {
                    let doc = schema
                        .description
                        .as_ref()
                        .map(|doc| quote::quote!(#[doc = #doc]));

                    let ident = change_case::pascal_case(&*name);
                    let ident = RE_UNDERSCORE.replace_all(&*ident, "");
                    let ident = match parent_ident {
                        Some(parent_ident) => quote::format_ident!("{}{}", parent_ident, ident),
                        None => quote::format_ident!("{}", ident),
                    };

                    let defaults_mod_ident =
                        quote::format_ident!("{}_defaults", change_case::snake_case(&*ident.to_string()));

                    let generated: Vec<_> = properties
                        .iter()
                        .map(|(name, schema)| {
                            let doc = schema
                                .description
                                .as_ref()
                                .map(|doc| quote::quote!(#[doc = #doc]));

                            let deprecated = schema.description.as_ref().and_then(|doc| {
                                if doc.starts_with("[DEPRECATED]") {
                                    Some(quote::quote!(#[deprecated]))
                                } else {
                                    None
                                }
                            });

                            let (ident, ty, tokens) =
                                generate_tokens_for_schema(Some(&ident), name, schema)?;

                            let ty = if schema.required == Some(true) || schema.default.is_some() {
                                ty
                            } else {
                                quote::quote!(::std::option::Option<#ty>)
                            };

                            let serde_default = schema.default.as_ref().map(|_| {
                                let default =
                                    quote::quote!(#defaults_mod_ident::#ident).to_string();

                                quote::quote! {
                                    #[serde(default = #default)]
                                }
                            });

                            let serde_skip = if schema.required == Some(true)
                                || schema.default.is_some()
                            {
                                None
                            } else {
                                Some(quote::quote! {
                                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                                })
                            };

                            let default_fn = schema.default.as_ref().map(|default| {
                                let default = if Some(JsonSchemaType::String) == schema.r#type
                                    && schema.r#enum.is_none()
                                {
                                    quote::quote!(String::from(#default))
                                } else {
                                    quote::quote!(serde_json::from_str(&#default).unwrap())
                                };

                                let default_ty = if let Some(proc_macro2::TokenTree::Ident(_)) =
                                    ty.clone().into_iter().next()
                                {
                                    quote::quote!(super::#ty)
                                } else {
                                    ty.clone()
                                };

                                quote::quote! {
                                    pub fn #ident() -> #default_ty {
                                        #default
                                    }
                                }
                            });

                            Ok::<_, anyhow::Error>((
                                quote::quote! {
                                    #[serde(rename = #name)]
                                    #serde_default
                                    #serde_skip
                                    #doc
                                    #deprecated
                                    pub #ident: #ty
                                },
                                tokens,
                                default_fn,
                            ))
                        })
                        .collect::<Result<_, _>>()?;

                    let fields: Vec<_> = generated.iter().map(|gen| &gen.0).collect();
                    let tokens: Vec<_> = generated.iter().map(|gen| &gen.1).collect();

                    let default_fns: Vec<_> = generated.iter().map(|gen| &gen.2).collect();
                    let defaults_mod = if default_fns.iter().any(|d| d.is_some()) {
                        Some(quote::quote! {
                            mod #defaults_mod_ident {
                                #(#default_fns)*
                            }
                        })
                    } else {
                        None
                    };

                    Ok((
                        quote::format_ident!("{}", name),
                        quote::quote!(#ident),
                        Some(quote::quote! {
                            #[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
                            #[serde(deny_unknown_fields)]
                            #doc
                            pub struct #ident {
                                #(#fields),*
                            }

                            #defaults_mod

                            #(#tokens)*
                        }),
                    ))
                } else if let Some(ref additional_properties) = schema.additional_properties {
                    let (ident, ty, tokens) =
                        generate_tokens_for_schema(parent_ident, &*name, &**additional_properties)?;

                    Ok((
                        ident,
                        quote::quote!(::std::collections::BTreeMap<String, #ty>),
                        tokens,
                    ))
                } else {
                    unimplemented!()
                }
            }
            JsonSchemaType::String => {
                if let Some(ref r#enum) = schema.r#enum {
                    let doc = schema
                        .description
                        .as_ref()
                        .map(|doc| quote::quote!(#[doc = #doc]));

                    let ident = change_case::pascal_case(&*name);
                    let ident = RE_UNDERSCORE.replace_all(&*ident, "");
                    let ident = match parent_ident {
                        Some(parent_ident) => quote::format_ident!("{}{}Enum", parent_ident, ident),
                        None => quote::format_ident!("{}Enum", ident),
                    };

                    let variants: Vec<_> = r#enum
                        .iter()
                        .enumerate()
                        .map(|(i, variant_name)| {
                            let doc = schema
                                .enum_descriptions
                                .as_ref()
                                .and_then(|docs| docs.get(i))
                                .map(|doc| quote::quote!(#[doc = #doc]));

                            let variant = change_case::pascal_case(&*variant_name);
                            let variant = RE_UNDERSCORE.replace_all(&*variant, "");
                            let variant = quote::format_ident!(
                                "{}",
                                RE_INVALID.replace_all(&*variant, "_${1}")
                            );

                            quote::quote! {
                                #[serde(rename = #variant_name)]
                                #doc
                                #variant
                            }
                        })
                        .collect();

                    let default_impl = schema.default.as_ref().map(|default| {
                        let default = change_case::pascal_case(&*default);
                        let default = RE_UNDERSCORE.replace_all(&*default, "");
                        let default =
                            quote::format_ident!("{}", RE_INVALID.replace_all(&*default, "_${1}"));

                        quote::quote! {
                            impl ::std::default::Default for #ident {
                                fn default() -> Self {
                                    Self::#default
                                }
                            }
                        }
                    });

                    Ok((
                        quote::format_ident!("{}", name),
                        quote::quote!(#ident),
                        Some(quote::quote! {
                            #[derive(Debug, PartialEq, Copy, Clone, serde::Serialize, serde::Deserialize)]
                            #doc
                            pub enum #ident {
                                #(#variants),*
                            }

                            #default_impl
                        }),
                    ))
                } else {
                    Ok((
                        quote::format_ident!("{}", name),
                        quote::quote!(::std::string::String),
                        None,
                    ))
                }
            }
        }
    } else {
        unimplemented!()
    }
}
