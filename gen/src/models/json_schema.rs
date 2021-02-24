#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct JsonSchema {
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A reference to another schema. The value of this property is the \"id\" of another schema."]
    pub _ref: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "additionalProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this is a schema for an object, this property is the schema for any additional properties with dynamic keys on this object."]
    pub additional_properties: ::std::option::Option<::std::boxed::Box<JsonSchema>>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional information about this property."]
    pub annotations: ::std::option::Option<JsonSchemaAnnotations>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default value of this property (if one exists)."]
    pub _default: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of this object."]
    pub description: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "enum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Values this parameter may take (if it is an enum)."]
    pub _enum: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "enumDescriptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The descriptions for the enums. Each position maps to the corresponding value in the \"enum\" array."]
    pub enum_descriptions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An additional regular expression or key that helps constrain the value. For more details see: http://tools.ietf.org/html/draft-zyp-json-schema-03#section-5.23"]
    pub format: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for this schema."]
    pub id: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this is a schema for an array, this property is the schema for each element in the array."]
    pub items: ::std::option::Option<::std::boxed::Box<JsonSchema>>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this parameter goes in the query or the path for REST requests."]
    pub location: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "maximum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum value of this parameter."]
    pub maximum: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "minimum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum value of this parameter."]
    pub minimum: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "pattern")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The regular expression this parameter must conform to. Uses Java 6 regex format: http://docs.oracle.com/javase/6/docs/api/java/util/regex/Pattern.html"]
    pub pattern: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this is a schema for an object, list the schema for each property of this object."]
    pub properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<JsonSchema>>>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value is read-only, generated by the service. The value cannot be modified by the client. If the value is included in a POST, PUT, or PATCH request, it is ignored by the service."]
    pub read_only: ::std::option::Option<::std::primitive::bool>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "repeated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this parameter may appear multiple times."]
    pub repeated: ::std::option::Option<::std::primitive::bool>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "required")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the parameter is required."]
    pub required: ::std::option::Option<::std::primitive::bool>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value type for this schema. A list of values can be found here: http://tools.ietf.org/html/draft-zyp-json-schema-03#section-5.1"]
    pub _type: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "variant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "In a variant data type, the value of one property is used to determine how to interpret the entire entity. Its value must exist in a map of descriminant values to schema names."]
    pub variant: ::std::option::Option<JsonSchemaVariant>,
}
impl JsonSchema {
    pub fn builder() -> JsonSchemaBuilder {
        JsonSchemaBuilder::default()
    }
}
#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
#[doc = "Additional information about this property."]
pub struct JsonSchemaAnnotations {
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "required")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of methods for which this property is required on requests."]
    pub required: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl JsonSchemaAnnotations {
    pub fn builder() -> JsonSchemaAnnotationsBuilder {
        JsonSchemaAnnotationsBuilder::default()
    }
}
#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
#[doc = "In a variant data type, the value of one property is used to determine how to interpret the entire entity. Its value must exist in a map of descriminant values to schema names."]
pub struct JsonSchemaVariant {
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "discriminant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the type discriminant property."]
    pub discriminant: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "map")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The map of discriminant value to schema to use for parsing.."]
    pub map: ::std::option::Option<::std::vec::Vec<JsonSchemaVariantMap>>,
}
impl JsonSchemaVariant {
    pub fn builder() -> JsonSchemaVariantBuilder {
        JsonSchemaVariantBuilder::default()
    }
}
#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct JsonSchemaVariantMap {
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub _ref: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "type_value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub type_value: ::std::option::Option<::std::string::String>,
}
impl JsonSchemaVariantMap {
    pub fn builder() -> JsonSchemaVariantMapBuilder {
        JsonSchemaVariantMapBuilder::default()
    }
}
