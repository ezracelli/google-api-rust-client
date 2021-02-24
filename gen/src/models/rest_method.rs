use super::json_schema::JsonSchema;

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
    pub parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<JsonSchema>>>,
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
