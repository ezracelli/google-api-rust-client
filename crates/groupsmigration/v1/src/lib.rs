#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "JSON response template for groups migration API."]
pub struct Groups {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of insert resource this is."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "responseCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the insert request."]
    pub response_code: ::std::option::Option<::std::string::String>,
}
