#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct JsonSchemaAnnotations {
    pub required: Option<Vec<String>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(untagged)]
pub enum JsonSchemaType {
    Any,
    Array,
    Boolean,
    Integer,
    Null,
    Number,
    Object,
    String,
    // Multi(Vec<JsonSchemaType>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct JsonSchemaVariantMap {
    #[serde(rename = "$ref")]
    pub r#ref: Option<String>,
    pub type_value: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct JsonSchemaVariant {
    pub discriminant: Option<String>,
    pub map: Option<Vec<JsonSchemaVariantMap>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct JsonSchema {
    pub additional_properties: Option<Box<JsonSchema>>,
    pub annotations: Option<JsonSchemaAnnotations>,
    pub default: Option<String>,
    pub description: Option<String>,
    pub enum_descriptions: Option<Vec<String>>,
    pub format: Option<String>,
    pub id: Option<String>,
    pub items: Option<Box<JsonSchema>>,
    pub location: Option<String>,
    pub maximum: Option<String>,
    pub minimum: Option<String>,
    pub pattern: Option<String>,
    pub properties: Option<std::collections::HashMap<String, JsonSchema>>,
    pub r#enum: Option<Vec<String>>,
    #[serde(rename = "$ref")]
    pub r#ref: Option<String>,
    pub r#type: Option<JsonSchemaType>,
    pub read_only: Option<bool>,
    pub repeated: Option<bool>,
    pub required: Option<bool>,
    pub variant: Option<JsonSchemaVariant>,
}
