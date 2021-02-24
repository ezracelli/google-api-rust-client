#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message includes the context and a list of matching results which contain the detail of associated entities."]
pub struct SearchResponse {
    #[serde(rename = "@context")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The local context applicable for the response. See more details at http://www.w3.org/TR/json-ld/#context-definitions."]
    pub context: ::std::option::Option<::serde_json::Value>,
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The schema type of top-level JSON-LD object, e.g. ItemList."]
    pub _type: ::std::option::Option<::serde_json::Value>,
    #[serde(rename = "itemListElement")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The item list of search results."]
    pub item_list_element: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
}
