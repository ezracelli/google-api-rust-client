#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata describing a family of fonts."]
pub struct Webfont {
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The category of the font."]
    pub category: ::std::option::Option<::std::string::String>,
    #[serde(rename = "family")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the font."]
    pub family: ::std::option::Option<::std::string::String>,
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The font files (with all supported scripts) for each one of the available variants, as a key : value map."]
    pub files: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This kind represents a webfont object in the webfonts service."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastModified")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date (format \"yyyy-MM-dd\") the font was modified for the last time."]
    pub last_modified: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subsets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The scripts supported by the font."]
    pub subsets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "variants")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The available variants for the font."]
    pub variants: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The font version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response containing the list of fonts currently served by the Google Fonts API."]
pub struct WebfontList {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of fonts currently served by the Google Fonts API."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Webfont>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This kind represents a list of webfont objects in the webfonts service."]
    pub kind: ::std::option::Option<::std::string::String>,
}
