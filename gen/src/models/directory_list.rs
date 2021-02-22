use super::directory_item::DirectoryItem;

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryList {
    #[serde(default = "directory_list_defaults::discovery_version")]
    pub discovery_version: String,
    pub items: Option<Vec<DirectoryItem>>,
    #[serde(default = "directory_list_defaults::kind")]
    pub kind: String,
}

mod directory_list_defaults {
    pub fn discovery_version() -> String {
        String::from("v1")
    }

    pub fn kind() -> String {
        String::from("discovery#directoryList")
    }
}
