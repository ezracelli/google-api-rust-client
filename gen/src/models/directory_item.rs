#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryItemIcons {
    x16: Option<String>,
    x32: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryItem {
    pub description: Option<String>,
    pub discovery_link: Option<String>,
    pub discovery_rest_url: Option<String>,
    pub documentation_link: Option<String>,
    pub icons: Option<DirectoryItemIcons>,
    pub id: String,
    #[serde(default = "directory_item_defaults::kind")]
    pub kind: String,
    pub labels: Option<Vec<String>>,
    pub name: String,
    pub preferred: Option<bool>,
    pub title: String,
    pub version: String,
}

mod directory_item_defaults {
    pub fn kind() -> String {
        String::from("discovery#directoryItem")
    }
}
