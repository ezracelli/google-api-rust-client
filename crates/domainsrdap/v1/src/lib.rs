#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message that represents an arbitrary HTTP body. It should only be used for payload formats that can't be represented as JSON, such as raw binary or an HTML page. This message can be used both in streaming and non-streaming API methods in the request as well as the response. It can be used as a top-level request field, which is convenient if one wants to extract parameters from either the URL or HTTP template into the request fields and also want access to the raw HTTP body. Example: message GetResourceRequest { // A unique request id. string request_id = 1; // The raw HTTP body is bound to this field. google.api.HttpBody http_body = 2; } service ResourceService { rpc GetResource(GetResourceRequest) returns (google.api.HttpBody); rpc UpdateResource(google.api.HttpBody) returns (google.protobuf.Empty); } Example with streaming methods: service CaldavService { rpc GetCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); rpc UpdateCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); } Use of this type only changes how the request and response bodies are handled, all other features will continue to work unchanged."]
pub struct HttpBody {
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HTTP Content-Type header value specifying the content type of the body."]
    pub content_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HTTP request/response body as raw binary."]
    pub data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Application specific response metadata. Must be set in the first response for streaming APIs."]
    pub extensions: ::std::option::Option<
        ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Links object defined in [section 4.2 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-4.2)."]
pub struct Link {
    #[serde(rename = "href")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target URL of a link. Example: \"http://example.com/previous\"."]
    pub href: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hreflang")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language code of a link. Example: \"en\"."]
    pub hreflang: ::std::option::Option<::std::string::String>,
    #[serde(rename = "media")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Media type of the link destination. Example: \"screen\"."]
    pub media: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Relation type of a link. Example: \"previous\"."]
    pub rel: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of this link. Example: \"title\"."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Content type of the link. Example: \"application/json\"."]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL giving context for the link. Example: \"http://example.com/current\"."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Notices object defined in [section 4.3 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-4.3)."]
pub struct Notice {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the notice."]
    pub description: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "links")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Link to a document containing more information."]
    pub links: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Link>>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of a notice. Example: \"Terms of Service\"."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type values defined in [section 10.2.1 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-10.2.1) specific to a whole response: \"result set truncated due to authorization\", \"result set truncated due to excessive load\", \"result set truncated due to unexplainable reasons\"."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response to a general RDAP query."]
pub struct RdapResponse {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error description."]
    pub description: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error HTTP code. Example: \"501\"."]
    pub error_code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "jsonResponse")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "HTTP response with content type set to \"application/json+rdap\"."]
    pub json_response: ::std::option::Option<::std::boxed::Box<HttpBody>>,
    #[serde(rename = "lang")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error language code. Error response info fields are defined in [section 6 of RFC 7483](https://tools.ietf.org/html/rfc7483#section-6)."]
    pub lang: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notices applying to this response."]
    pub notices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Notice>>>,
    #[serde(rename = "rdapConformance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "RDAP conformance level."]
    pub rdap_conformance: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error title."]
    pub title: ::std::option::Option<::std::string::String>,
}
