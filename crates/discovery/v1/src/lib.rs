#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DirectoryList {
    #[serde(rename = "discoveryVersion")]
    #[serde(default = "directory_list_defaults :: discovery_version")]
    #[doc = "Indicate the version of the Discovery API used to generate this doc."]
    pub discovery_version: ::std::string::String,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The individual directory entries. One entry per api/version pair."]
    pub items: ::std::option::Option<::std::vec::Vec<DirectoryListItems>>,
    #[serde(rename = "kind")]
    #[serde(default = "directory_list_defaults :: kind")]
    #[doc = "The kind for this response."]
    pub kind: ::std::string::String,
}
mod directory_list_defaults {
    pub fn discovery_version() -> ::std::string::String {
        String::from("v1")
    }
    pub fn kind() -> ::std::string::String {
        String::from("discovery#directoryList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DirectoryListItems {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of this API."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "discoveryLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the discovery document."]
    pub discovery_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "discoveryRestUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL for the discovery REST document."]
    pub discovery_rest_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "documentationLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to human readable documentation for the API."]
    pub documentation_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "icons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Links to 16x16 and 32x32 icons representing the API."]
    pub icons: ::std::option::Option<DirectoryListItemsIcons>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of this API."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "directory_list_items_defaults :: kind")]
    #[doc = "The kind for this response."]
    pub kind: ::std::string::String,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Labels for the status of this API, such as labs or deprecated."]
    pub labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the API."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "preferred")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if this version is the preferred version to use."]
    pub preferred: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of this API."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of the API."]
    pub version: ::std::option::Option<::std::string::String>,
}
mod directory_list_items_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("discovery#directoryItem")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Links to 16x16 and 32x32 icons representing the API."]
pub struct DirectoryListItemsIcons {
    #[serde(rename = "x16")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the 16x16 icon."]
    pub x16: ::std::option::Option<::std::string::String>,
    #[serde(rename = "x32")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the 32x32 icon."]
    pub x32: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonSchema {
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A reference to another schema. The value of this property is the \"id\" of another schema."]
    pub _ref: ::std::option::Option<::std::string::String>,
    #[serde(rename = "additionalProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this is a schema for an object, this property is the schema for any additional properties with dynamic keys on this object."]
    pub additional_properties: ::std::option::Option<::std::boxed::Box<JsonSchema>>,
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional information about this property."]
    pub annotations: ::std::option::Option<JsonSchemaAnnotations>,
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default value of this property (if one exists)."]
    pub _default: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of this object."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Values this parameter may take (if it is an enum)."]
    pub _enum: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "enumDescriptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The descriptions for the enums. Each position maps to the corresponding value in the \"enum\" array."]
    pub enum_descriptions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An additional regular expression or key that helps constrain the value. For more details see: http://tools.ietf.org/html/draft-zyp-json-schema-03#section-5.23"]
    pub format: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for this schema."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this is a schema for an array, this property is the schema for each element in the array."]
    pub items: ::std::option::Option<::std::boxed::Box<JsonSchema>>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this parameter goes in the query or the path for REST requests."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maximum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum value of this parameter."]
    pub maximum: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minimum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum value of this parameter."]
    pub minimum: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pattern")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The regular expression this parameter must conform to. Uses Java 6 regex format: http://docs.oracle.com/javase/6/docs/api/java/util/regex/Pattern.html"]
    pub pattern: ::std::option::Option<::std::string::String>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this is a schema for an object, list the schema for each property of this object."]
    pub properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<JsonSchema>>>,
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value is read-only, generated by the service. The value cannot be modified by the client. If the value is included in a POST, PUT, or PATCH request, it is ignored by the service."]
    pub read_only: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "repeated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this parameter may appear multiple times."]
    pub repeated: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "required")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the parameter is required."]
    pub required: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value type for this schema. A list of values can be found here: http://tools.ietf.org/html/draft-zyp-json-schema-03#section-5.1"]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "variant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "In a variant data type, the value of one property is used to determine how to interpret the entire entity. Its value must exist in a map of descriminant values to schema names."]
    pub variant: ::std::option::Option<JsonSchemaVariant>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional information about this property."]
pub struct JsonSchemaAnnotations {
    #[serde(rename = "required")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of methods for which this property is required on requests."]
    pub required: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "In a variant data type, the value of one property is used to determine how to interpret the entire entity. Its value must exist in a map of descriminant values to schema names."]
pub struct JsonSchemaVariant {
    #[serde(rename = "discriminant")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the type discriminant property."]
    pub discriminant: ::std::option::Option<::std::string::String>,
    #[serde(rename = "map")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The map of discriminant value to schema to use for parsing.."]
    pub map: ::std::option::Option<::std::vec::Vec<JsonSchemaVariantMap>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonSchemaVariantMap {
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub _ref: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type_value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub type_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RestDescription {
    #[serde(rename = "auth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Authentication information."]
    pub auth: ::std::option::Option<RestDescriptionAuth>,
    #[serde(rename = "basePath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[DEPRECATED] The base path for REST requests."]
    #[deprecated]
    pub base_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "baseUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[DEPRECATED] The base URL for REST requests."]
    #[deprecated]
    pub base_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "batchPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The path for REST batch requests."]
    pub batch_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "canonicalName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates how the API name should be capitalized and split into various parts. Useful for generating pretty class names."]
    pub canonical_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of this API."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "discoveryVersion")]
    #[serde(default = "rest_description_defaults :: discovery_version")]
    #[doc = "Indicate the version of the Discovery API used to generate this doc."]
    pub discovery_version: ::std::string::String,
    #[serde(rename = "documentationLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to human readable documentation for the API."]
    pub documentation_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ETag for this response."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "exponentialBackoffDefault")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enable exponential backoff for suitable methods in the generated clients."]
    pub exponential_backoff_default: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of supported features for this API."]
    pub features: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "icons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Links to 16x16 and 32x32 icons representing the API."]
    pub icons: ::std::option::Option<RestDescriptionIcons>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of this API."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "rest_description_defaults :: kind")]
    #[doc = "The kind for this response."]
    pub kind: ::std::string::String,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Labels for the status of this API, such as labs or deprecated."]
    pub labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "methods")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API-level methods for this API."]
    pub methods:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<RestMethod>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this API."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ownerDomain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The domain of the owner of this API. Together with the ownerName and a packagePath values, this can be used to generate a library for this API which would have a unique fully qualified name."]
    pub owner_domain: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ownerName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the owner of this API. See ownerDomain."]
    pub owner_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "packagePath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The package of the owner of this API. See ownerDomain."]
    pub package_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common parameters that apply across all apis."]
    pub parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<JsonSchema>>>,
    #[serde(rename = "protocol")]
    #[serde(default = "rest_description_defaults :: protocol")]
    #[doc = "The protocol described by this document."]
    pub protocol: ::std::string::String,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resources in this API."]
    pub resources: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<RestResource>>,
    >,
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of this API."]
    pub revision: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rootUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The root URL under which all API services live."]
    pub root_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The schemas for this API."]
    pub schemas:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<JsonSchema>>>,
    #[serde(rename = "servicePath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The base path for all REST requests."]
    pub service_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of this API."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of this API."]
    pub version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version_module")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub version_module: ::std::option::Option<::std::primitive::bool>,
}
mod rest_description_defaults {
    pub fn discovery_version() -> ::std::string::String {
        String::from("v1")
    }
    pub fn kind() -> ::std::string::String {
        String::from("discovery#restDescription")
    }
    pub fn protocol() -> ::std::string::String {
        String::from("rest")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Authentication information."]
pub struct RestDescriptionAuth {
    #[serde(rename = "oauth2")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth 2.0 authentication information."]
    pub oauth2: ::std::option::Option<RestDescriptionAuthOauth2>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "OAuth 2.0 authentication information."]
pub struct RestDescriptionAuthOauth2 {
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Available OAuth 2.0 scopes."]
    pub scopes: ::std::option::Option<
        ::std::collections::BTreeMap<String, RestDescriptionAuthOauth2Scopes>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The scope value."]
pub struct RestDescriptionAuthOauth2Scopes {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of scope."]
    pub description: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Links to 16x16 and 32x32 icons representing the API."]
pub struct RestDescriptionIcons {
    #[serde(rename = "x16")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the 16x16 icon."]
    pub x16: ::std::option::Option<::std::string::String>,
    #[serde(rename = "x32")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the 32x32 icon."]
    pub x32: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RestMethod {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of this method."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etagRequired")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this method requires an ETag to be specified. The ETag is sent as an HTTP If-Match or If-None-Match header."]
    pub etag_required: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP method used by this method."]
    pub http_method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique ID for this method. This property can be used to match methods between different versions of Discovery."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mediaUpload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Media upload parameters."]
    pub media_upload: ::std::option::Option<RestMethodMediaUpload>,
    #[serde(rename = "parameterOrder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ordered list of required parameters, serves as a hint to clients on how to structure their method signatures. The array is ordered such that the \"most-significant\" parameter appears first."]
    pub parameter_order: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details for all parameters in this method."]
    pub parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<JsonSchema>>>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI path of this REST method. Should be used in conjunction with the basePath property at the api-level."]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "request")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The schema for the request."]
    pub request: ::std::option::Option<RestMethodRequest>,
    #[serde(rename = "response")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The schema for the response."]
    pub response: ::std::option::Option<RestMethodResponse>,
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth 2.0 scopes applicable to this method."]
    pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "supportsMediaDownload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this method supports media downloads."]
    pub supports_media_download: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "supportsMediaUpload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this method supports media uploads."]
    pub supports_media_upload: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "supportsSubscription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this method supports subscriptions."]
    pub supports_subscription: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "useMediaDownloadService")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that downloads from this method should use the download service URL (i.e. \"/download\"). Only applies if the method supports media download."]
    pub use_media_download_service: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Media upload parameters."]
pub struct RestMethodMediaUpload {
    #[serde(rename = "accept")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "MIME Media Ranges for acceptable media uploads to this method."]
    pub accept: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "maxSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum size of a media upload, such as \"1MB\", \"2GB\" or \"3TB\"."]
    pub max_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "protocols")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supported upload protocols."]
    pub protocols: ::std::option::Option<RestMethodMediaUploadProtocols>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Supported upload protocols."]
pub struct RestMethodMediaUploadProtocols {
    #[serde(rename = "resumable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supports the Resumable Media Upload protocol."]
    pub resumable: ::std::option::Option<RestMethodMediaUploadProtocolsResumable>,
    #[serde(rename = "simple")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supports uploading as a single HTTP request."]
    pub simple: ::std::option::Option<RestMethodMediaUploadProtocolsSimple>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Supports the Resumable Media Upload protocol."]
pub struct RestMethodMediaUploadProtocolsResumable {
    #[serde(rename = "multipart")]
    #[serde(default = "rest_method_media_upload_protocols_resumable_defaults :: multipart")]
    #[doc = "True if this endpoint supports uploading multipart media."]
    pub multipart: ::std::primitive::bool,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI path to be used for upload. Should be used in conjunction with the basePath property at the api-level."]
    pub path: ::std::option::Option<::std::string::String>,
}
mod rest_method_media_upload_protocols_resumable_defaults {
    pub fn multipart() -> ::std::primitive::bool {
        serde_json::from_str(&"true").unwrap()
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Supports uploading as a single HTTP request."]
pub struct RestMethodMediaUploadProtocolsSimple {
    #[serde(rename = "multipart")]
    #[serde(default = "rest_method_media_upload_protocols_simple_defaults :: multipart")]
    #[doc = "True if this endpoint supports upload multipart media."]
    pub multipart: ::std::primitive::bool,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI path to be used for upload. Should be used in conjunction with the basePath property at the api-level."]
    pub path: ::std::option::Option<::std::string::String>,
}
mod rest_method_media_upload_protocols_simple_defaults {
    pub fn multipart() -> ::std::primitive::bool {
        serde_json::from_str(&"true").unwrap()
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The schema for the request."]
pub struct RestMethodRequest {
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Schema ID for the request schema."]
    pub _ref: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameterName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "parameter name."]
    pub parameter_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The schema for the response."]
pub struct RestMethodResponse {
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Schema ID for the response schema."]
    pub _ref: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RestResource {
    #[serde(rename = "methods")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Methods on this resource."]
    pub methods:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<RestMethod>>>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sub-resources on this resource."]
    pub resources: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<RestResource>>,
    >,
}
