use super::json_schema::JsonSchema;
use super::rest_method::RestMethod;
use super::rest_resource::RestResource;

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RestDescriptionAuthOAuth2Scope {
    pub description: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RestDescriptionAuthOAuth2 {
    pub scopes: Option<std::collections::BTreeMap<String, RestDescriptionAuthOAuth2Scope>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RestDescriptionAuth {
    pub oauth2: Option<RestDescriptionAuthOAuth2>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RestDescriptionIcons {
    pub x16: Option<String>,
    pub x32: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RestDescription {
    pub auth: Option<RestDescriptionAuth>,
    #[deprecated]
    pub base_path: Option<String>,
    #[deprecated]
    pub base_url: Option<String>,
    pub batch_path: Option<String>,
    pub canonical_name: Option<String>,
    pub description: Option<String>,
    #[serde(default = "rest_description_defaults::discovery_version")]
    pub discovery_version: String,
    pub documentation_link: Option<String>,
    pub etag: Option<String>,
    pub exponential_backoff_default: Option<bool>,
    pub features: Option<Vec<String>>,
    pub fully_encode_reserved_expansion: Option<bool>,
    pub icons: Option<RestDescriptionIcons>,
    pub id: Option<String>,
    #[serde(default = "rest_description_defaults::kind")]
    pub kind: String,
    pub labels: Option<Vec<String>>,
    pub methods: Option<std::collections::BTreeMap<String, RestMethod>>,
    pub mtls_root_url: Option<String>,
    pub name: String,
    pub owner_domain: Option<String>,
    pub owner_name: Option<String>,
    pub package_path: Option<String>,
    pub parameters: Option<std::collections::BTreeMap<String, JsonSchema>>,
    #[serde(default = "rest_description_defaults::protocol")]
    pub protocol: String,
    pub resources: Option<std::collections::BTreeMap<String, RestResource>>,
    pub revision: String,
    pub root_url: Option<String>,
    pub schemas: Option<std::collections::BTreeMap<String, JsonSchema>>,
    pub service_path: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "version_module")]
    pub version_module: Option<bool>,
    pub version: String,
}

mod rest_description_defaults {
    pub fn discovery_version() -> String {
        String::from("v1")
    }

    pub fn kind() -> String {
        String::from("discovery#restDescription")
    }

    pub fn protocol() -> String {
        String::from("rest")
    }
}
