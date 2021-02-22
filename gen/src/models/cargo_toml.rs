#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub enum Dependency {
    Detailed {
        branch: Option<String>,
        default_features: Option<bool>,
        features: Option<Vec<String>>,
        git: Option<String>,
        optional: Option<bool>,
        path: Option<String>,
        registry: Option<String>,
        rev: Option<String>,
        tag: Option<String>,
        version: Option<String>,
    },
    Simple(String),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub enum Publish {
    Bool(bool),
    Vec(Vec<String>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Package {
    pub authors: Option<Vec<String>>,
    pub build: Option<String>,
    pub categories: Option<Vec<String>>,
    pub default_run: Option<String>,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub edition: Option<String>,
    pub exclude: Option<Vec<String>>,
    pub homepage: Option<String>,
    pub include: Option<Vec<String>>,
    pub keywords: Option<Vec<String>>,
    pub license_file: Option<String>,
    pub license: Option<String>,
    pub links: Option<String>,
    pub metadata: Option<std::collections::HashMap<String, toml::Value>>,
    pub name: String,
    pub publish: Option<Publish>,
    pub readme: Option<String>,
    pub repository: Option<String>,
    pub version: Option<String>,
    pub workspace: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Workspace {
    pub default_members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub members: Option<Vec<String>>,
    pub metadata: Option<std::collections::HashMap<String, toml::Value>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub build_dependencies: Option<std::collections::HashMap<String, Dependency>>,
    pub dependencies: Option<std::collections::HashMap<String, Dependency>>,
    pub dev_dependencies: Option<std::collections::HashMap<String, Dependency>>,
    pub features: Option<std::collections::HashMap<String, Vec<String>>>,
    pub package: Option<Package>,
    pub workspace: Option<Workspace>,
}
