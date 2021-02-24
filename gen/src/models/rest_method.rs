use super::json_schema::JsonSchema;

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RestMethodMediaUploadProtocolsResumable {
    pub path: Option<String>,
    #[serde(default = "rest_method_media_upload_protocols_resumable_defaults::multipart")]
    pub multipart: bool,
}

mod rest_method_media_upload_protocols_resumable_defaults {
    pub fn multipart() -> bool {
        true
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RestMethodMediaUploadProtocolsSimple {
    pub path: Option<String>,
    #[serde(default = "rest_method_media_upload_protocols_simple_defaults::multipart")]
    pub multipart: bool,
}

mod rest_method_media_upload_protocols_simple_defaults {
    pub fn multipart() -> bool {
        true
    }
}
#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RestMethodMediaUploadProtocols {
    pub resumable: Option<RestMethodMediaUploadProtocolsResumable>,
    pub simple: Option<RestMethodMediaUploadProtocolsSimple>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RestMethodMediaUpload {
    pub accept: Option<Vec<String>>,
    pub max_size: Option<String>,
    pub protocols: Option<RestMethodMediaUploadProtocols>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RestMethodRequest {
    #[serde(rename = "$ref")]
    pub r#ref: Option<String>,
    pub parameter_name: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RestMethodResponse {
    #[serde(rename = "$ref")]
    pub r#ref: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RestMethod {
    pub description: Option<String>,
    pub etag_required: Option<bool>,
    pub http_method: Option<String>,
    pub flat_path: Option<String>,
    pub id: Option<String>,
    pub media_upload: Option<RestMethodMediaUpload>,
    pub parameter_order: Option<Vec<String>>,
    pub parameters: Option<std::collections::BTreeMap<String, JsonSchema>>,
    pub path: Option<String>,
    pub request: Option<RestMethodRequest>,
    pub response: Option<RestMethodResponse>,
    pub scopes: Option<Vec<String>>,
    pub supports_media_download: Option<bool>,
    pub supports_media_upload: Option<bool>,
    pub supports_subscription: Option<bool>,
    pub streaming_type: Option<String>,
    pub use_media_download_service: Option<bool>,
}
