use super::json_schema::JsonSchema;
use super::rest_method::RestMethod;
use super::rest_resource::RestResource;

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
    pub methods:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<RestMethod>>>,
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
    pub parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<JsonSchema>>>,
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
    pub schemas:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<JsonSchema>>>,
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
        String::from("v1")
    }
    pub fn kind() -> ::std::string::String {
        String::from("discovery#restDescription")
    }
    pub fn protocol() -> ::std::string::String {
        String::from("rest")
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
