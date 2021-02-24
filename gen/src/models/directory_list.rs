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
        String::from("v1")
    }
    pub fn kind() -> ::std::string::String {
        String::from("discovery#directoryList")
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
        String::from("discovery#directoryItem")
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
