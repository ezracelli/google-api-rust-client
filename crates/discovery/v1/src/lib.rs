#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct QueryParameters {
    #[builder(default = "{ query_parameters_defaults :: alt () }", setter(into))]
    #[serde(rename = "alt")]
    #[serde(default = "query_parameters_defaults :: alt")]
    #[doc = "Data format for the response."]
    pub alt: QueryParametersAltEnum,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selector specifying which fields to include in a partial response."]
    pub fields: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
    pub key: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "oauth_token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth 2.0 token for the current user."]
    pub oauth_token: ::std::option::Option<::std::string::String>,
    #[builder(
        default = "{ query_parameters_defaults :: pretty_print () }",
        setter(into)
    )]
    #[serde(rename = "prettyPrint")]
    #[serde(default = "query_parameters_defaults :: pretty_print")]
    #[doc = "Returns response with indentations and line breaks."]
    pub pretty_print: ::std::primitive::bool,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "quotaUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
    pub quota_user: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "userIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use quotaUser instead."]
    pub user_ip: ::std::option::Option<::std::string::String>,
}
impl QueryParameters {
    pub fn builder() -> QueryParametersBuilder {
        QueryParametersBuilder::default()
    }
}
mod query_parameters_defaults {
    pub fn alt() -> super::QueryParametersAltEnum {
        serde_json::from_str(&"\"json\"").unwrap()
    }
    pub fn pretty_print() -> ::std::primitive::bool {
        serde_json::from_str(&"true").unwrap()
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Data format for the response."]
pub enum QueryParametersAltEnum {
    #[serde(rename = "json")]
    #[doc = "Responses with Content-Type of application/json"]
    Json,
}
impl ::std::default::Default for QueryParametersAltEnum {
    fn default() -> Self {
        Self::Json
    }
}
pub mod resources {
    pub mod apis {
        pub mod methods {
            pub mod list {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "name")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Only include APIs with the given name."]
                    pub name: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: preferred () }",
                        setter(into)
                    )]
                    #[serde(rename = "preferred")]
                    #[serde(default = "query_parameters_defaults :: preferred")]
                    #[doc = "Return only the preferred version of an API."]
                    pub preferred: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn preferred() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
            }
        }
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DirectoryList {
        #[builder(
            default = "{ directory_list_defaults :: discovery_version () }",
            setter(into)
        )]
        #[serde(rename = "discoveryVersion")]
        #[serde(default = "directory_list_defaults :: discovery_version")]
        #[doc = "Indicate the version of the Discovery API used to generate this doc."]
        pub discovery_version: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The individual directory entries. One entry per api/version pair."]
        pub items: ::std::option::Option<::std::vec::Vec<DirectoryListItems>>,
        #[builder(default = "{ directory_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "directory_list_defaults :: kind")]
        #[doc = "The kind for this response."]
        pub kind: ::std::string::String,
    }
    impl DirectoryList {
        pub fn builder() -> DirectoryListBuilder {
            DirectoryListBuilder::default()
        }
    }
    mod directory_list_defaults {
        pub fn discovery_version() -> ::std::string::String {
            serde_json::from_str(&"\"v1\"").unwrap()
        }
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"discovery#directoryList\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DirectoryListItems {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of this API."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "discoveryLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the discovery document."]
        pub discovery_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "discoveryRestUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL for the discovery REST document."]
        pub discovery_rest_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "documentationLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to human readable documentation for the API."]
        pub documentation_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "icons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Links to 16x16 and 32x32 icons representing the API."]
        pub icons: ::std::option::Option<DirectoryListItemsIcons>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of this API."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ directory_list_items_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "directory_list_items_defaults :: kind")]
        #[doc = "The kind for this response."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels for the status of this API, such as labs or deprecated."]
        pub labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the API."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "preferred")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if this version is the preferred version to use."]
        pub preferred: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of this API."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the API."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl DirectoryListItems {
        pub fn builder() -> DirectoryListItemsBuilder {
            DirectoryListItemsBuilder::default()
        }
    }
    mod directory_list_items_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"discovery#directoryItem\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Links to 16x16 and 32x32 icons representing the API."]
    pub struct DirectoryListItemsIcons {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "x16")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the 16x16 icon."]
        pub x16: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "x32")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the 32x32 icon."]
        pub x32: ::std::option::Option<::std::string::String>,
    }
    impl DirectoryListItemsIcons {
        pub fn builder() -> DirectoryListItemsIconsBuilder {
            DirectoryListItemsIconsBuilder::default()
        }
    }
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
        pub properties: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<JsonSchema>>,
        >,
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
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct RestDescription {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Authentication information."]
        pub auth: ::std::option::Option<RestDescriptionAuth>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[DEPRECATED] The base path for REST requests."]
        #[deprecated]
        pub base_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "baseUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[DEPRECATED] The base URL for REST requests."]
        #[deprecated]
        pub base_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The path for REST batch requests."]
        pub batch_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canonicalName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates how the API name should be capitalized and split into various parts. Useful for generating pretty class names."]
        pub canonical_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of this API."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ rest_description_defaults :: discovery_version () }",
            setter(into)
        )]
        #[serde(rename = "discoveryVersion")]
        #[serde(default = "rest_description_defaults :: discovery_version")]
        #[doc = "Indicate the version of the Discovery API used to generate this doc."]
        pub discovery_version: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "documentationLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to human readable documentation for the API."]
        pub documentation_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ETag for this response."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exponentialBackoffDefault")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enable exponential backoff for suitable methods in the generated clients."]
        pub exponential_backoff_default: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "features")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of supported features for this API."]
        pub features: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "icons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Links to 16x16 and 32x32 icons representing the API."]
        pub icons: ::std::option::Option<RestDescriptionIcons>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of this API."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ rest_description_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "rest_description_defaults :: kind")]
        #[doc = "The kind for this response."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels for the status of this API, such as labs or deprecated."]
        pub labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "methods")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "API-level methods for this API."]
        pub methods: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<RestMethod>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this API."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownerDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain of the owner of this API. Together with the ownerName and a packagePath values, this can be used to generate a library for this API which would have a unique fully qualified name."]
        pub owner_domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownerName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the owner of this API. See ownerDomain."]
        pub owner_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packagePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The package of the owner of this API. See ownerDomain."]
        pub package_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Common parameters that apply across all apis."]
        pub parameters: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<JsonSchema>>,
        >,
        #[builder(default = "{ rest_description_defaults :: protocol () }", setter(into))]
        #[serde(rename = "protocol")]
        #[serde(default = "rest_description_defaults :: protocol")]
        #[doc = "The protocol described by this document."]
        pub protocol: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resources in this API."]
        pub resources: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<RestResource>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of this API."]
        pub revision: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rootUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The root URL under which all API services live."]
        pub root_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schemas")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The schemas for this API."]
        pub schemas: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<JsonSchema>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "servicePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The base path for all REST requests."]
        pub service_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of this API."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of this API."]
        pub version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version_module")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub version_module: ::std::option::Option<::std::primitive::bool>,
    }
    impl RestDescription {
        pub fn builder() -> RestDescriptionBuilder {
            RestDescriptionBuilder::default()
        }
    }
    mod rest_description_defaults {
        pub fn discovery_version() -> ::std::string::String {
            serde_json::from_str(&"\"v1\"").unwrap()
        }
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"discovery#restDescription\"").unwrap()
        }
        pub fn protocol() -> ::std::string::String {
            serde_json::from_str(&"\"rest\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Authentication information."]
    pub struct RestDescriptionAuth {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauth2")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OAuth 2.0 authentication information."]
        pub oauth2: ::std::option::Option<RestDescriptionAuthOauth2>,
    }
    impl RestDescriptionAuth {
        pub fn builder() -> RestDescriptionAuthBuilder {
            RestDescriptionAuthBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "OAuth 2.0 authentication information."]
    pub struct RestDescriptionAuthOauth2 {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scopes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Available OAuth 2.0 scopes."]
        pub scopes: ::std::option::Option<
            ::std::collections::BTreeMap<String, RestDescriptionAuthOauth2Scopes>,
        >,
    }
    impl RestDescriptionAuthOauth2 {
        pub fn builder() -> RestDescriptionAuthOauth2Builder {
            RestDescriptionAuthOauth2Builder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The scope value."]
    pub struct RestDescriptionAuthOauth2Scopes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of scope."]
        pub description: ::std::option::Option<::std::string::String>,
    }
    impl RestDescriptionAuthOauth2Scopes {
        pub fn builder() -> RestDescriptionAuthOauth2ScopesBuilder {
            RestDescriptionAuthOauth2ScopesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Links to 16x16 and 32x32 icons representing the API."]
    pub struct RestDescriptionIcons {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "x16")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the 16x16 icon."]
        pub x16: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "x32")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the 32x32 icon."]
        pub x32: ::std::option::Option<::std::string::String>,
    }
    impl RestDescriptionIcons {
        pub fn builder() -> RestDescriptionIconsBuilder {
            RestDescriptionIconsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct RestMethod {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of this method."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etagRequired")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this method requires an ETag to be specified. The ETag is sent as an HTTP If-Match or If-None-Match header."]
        pub etag_required: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP method used by this method."]
        pub http_method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique ID for this method. This property can be used to match methods between different versions of Discovery."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mediaUpload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Media upload parameters."]
        pub media_upload: ::std::option::Option<RestMethodMediaUpload>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameterOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ordered list of required parameters, serves as a hint to clients on how to structure their method signatures. The array is ordered such that the \"most-significant\" parameter appears first."]
        pub parameter_order: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details for all parameters in this method."]
        pub parameters: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<JsonSchema>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI path of this REST method. Should be used in conjunction with the basePath property at the api-level."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "request")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The schema for the request."]
        pub request: ::std::option::Option<RestMethodRequest>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "response")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The schema for the response."]
        pub response: ::std::option::Option<RestMethodResponse>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scopes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OAuth 2.0 scopes applicable to this method."]
        pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportsMediaDownload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this method supports media downloads."]
        pub supports_media_download: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportsMediaUpload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this method supports media uploads."]
        pub supports_media_upload: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportsSubscription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this method supports subscriptions."]
        pub supports_subscription: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useMediaDownloadService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates that downloads from this method should use the download service URL (i.e. \"/download\"). Only applies if the method supports media download."]
        pub use_media_download_service: ::std::option::Option<::std::primitive::bool>,
    }
    impl RestMethod {
        pub fn builder() -> RestMethodBuilder {
            RestMethodBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Media upload parameters."]
    pub struct RestMethodMediaUpload {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accept")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "MIME Media Ranges for acceptable media uploads to this method."]
        pub accept: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum size of a media upload, such as \"1MB\", \"2GB\" or \"3TB\"."]
        pub max_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protocols")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Supported upload protocols."]
        pub protocols: ::std::option::Option<RestMethodMediaUploadProtocols>,
    }
    impl RestMethodMediaUpload {
        pub fn builder() -> RestMethodMediaUploadBuilder {
            RestMethodMediaUploadBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Supported upload protocols."]
    pub struct RestMethodMediaUploadProtocols {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resumable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Supports the Resumable Media Upload protocol."]
        pub resumable: ::std::option::Option<RestMethodMediaUploadProtocolsResumable>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "simple")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Supports uploading as a single HTTP request."]
        pub simple: ::std::option::Option<RestMethodMediaUploadProtocolsSimple>,
    }
    impl RestMethodMediaUploadProtocols {
        pub fn builder() -> RestMethodMediaUploadProtocolsBuilder {
            RestMethodMediaUploadProtocolsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Supports the Resumable Media Upload protocol."]
    pub struct RestMethodMediaUploadProtocolsResumable {
        #[builder(
            default = "{ rest_method_media_upload_protocols_resumable_defaults :: multipart () }",
            setter(into)
        )]
        #[serde(rename = "multipart")]
        #[serde(default = "rest_method_media_upload_protocols_resumable_defaults :: multipart")]
        #[doc = "True if this endpoint supports uploading multipart media."]
        pub multipart: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI path to be used for upload. Should be used in conjunction with the basePath property at the api-level."]
        pub path: ::std::option::Option<::std::string::String>,
    }
    impl RestMethodMediaUploadProtocolsResumable {
        pub fn builder() -> RestMethodMediaUploadProtocolsResumableBuilder {
            RestMethodMediaUploadProtocolsResumableBuilder::default()
        }
    }
    mod rest_method_media_upload_protocols_resumable_defaults {
        pub fn multipart() -> ::std::primitive::bool {
            serde_json::from_str(&"true").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Supports uploading as a single HTTP request."]
    pub struct RestMethodMediaUploadProtocolsSimple {
        #[builder(
            default = "{ rest_method_media_upload_protocols_simple_defaults :: multipart () }",
            setter(into)
        )]
        #[serde(rename = "multipart")]
        #[serde(default = "rest_method_media_upload_protocols_simple_defaults :: multipart")]
        #[doc = "True if this endpoint supports upload multipart media."]
        pub multipart: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI path to be used for upload. Should be used in conjunction with the basePath property at the api-level."]
        pub path: ::std::option::Option<::std::string::String>,
    }
    impl RestMethodMediaUploadProtocolsSimple {
        pub fn builder() -> RestMethodMediaUploadProtocolsSimpleBuilder {
            RestMethodMediaUploadProtocolsSimpleBuilder::default()
        }
    }
    mod rest_method_media_upload_protocols_simple_defaults {
        pub fn multipart() -> ::std::primitive::bool {
            serde_json::from_str(&"true").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The schema for the request."]
    pub struct RestMethodRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "$ref")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Schema ID for the request schema."]
        pub _ref: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameterName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "parameter name."]
        pub parameter_name: ::std::option::Option<::std::string::String>,
    }
    impl RestMethodRequest {
        pub fn builder() -> RestMethodRequestBuilder {
            RestMethodRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The schema for the response."]
    pub struct RestMethodResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "$ref")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Schema ID for the response schema."]
        pub _ref: ::std::option::Option<::std::string::String>,
    }
    impl RestMethodResponse {
        pub fn builder() -> RestMethodResponseBuilder {
            RestMethodResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct RestResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "methods")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Methods on this resource."]
        pub methods: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<RestMethod>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sub-resources on this resource."]
        pub resources: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<RestResource>>,
        >,
    }
    impl RestResource {
        pub fn builder() -> RestResourceBuilder {
            RestResourceBuilder::default()
        }
    }
}
