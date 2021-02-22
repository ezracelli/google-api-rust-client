use crate::models::json_schema::{JsonSchema, JsonSchemaType};

pub fn property_ty(property: &JsonSchema) -> anyhow::Result<proc_macro2::TokenStream> {
    Ok(if let Some(ref r#ref) = property.r#ref {
        let r#ref = quote::format_ident!("{}", r#ref);
        quote::quote!(#r#ref)
    } else if let Some(ref ty) = property.r#type {
        match ty {
            JsonSchemaType::Any => quote::quote!(serde_json::Value),
            JsonSchemaType::Array => {
                if let Some(ref items) = property.items {
                    let ty = if let Some(ref r#ref) = (**items).r#ref {
                        let r#ref = quote::format_ident!("{}", r#ref);
                        quote::quote!(#r#ref)
                    } else {
                        property_ty(&**items)?
                    };

                    quote::quote!(Vec<#ty>)
                } else {
                    anyhow::bail!("no .items for array {:?}", property.id);
                }
            }
            JsonSchemaType::Boolean => quote::quote!(bool),
            JsonSchemaType::Integer => quote::quote!(i64),
            JsonSchemaType::Null => unimplemented!("null"),
            JsonSchemaType::Number => quote::quote!(f64),
            // JsonSchemaType::Object => unimplemented!("object"),
            JsonSchemaType::Object => quote::quote!(serde_json::Value),
            JsonSchemaType::String => quote::quote!(String),
        }
    } else {
        anyhow::bail!("no .$ref or .type for property {:?}", property.id);
    })
}
