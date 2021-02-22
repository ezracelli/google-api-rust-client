use super::rest_method::RestMethod;

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RestResource {
    pub methods: Option<std::collections::HashMap<String, RestMethod>>,
    pub resources: Option<std::collections::HashMap<String, RestResource>>,
}
